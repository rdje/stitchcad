#!/usr/bin/env python3
"""Bounded, self-contained historical record reader (SPINE.19.2).

No third-party modules, extraction API, network or historical Git dependency.
Schema/TOC checks precede record reads; decoded bytes and resident bytes have
independent caps. The doctrine wrapper additionally guards committed windows.
"""
import argparse
import datetime
import gzip
import hashlib
import io
import json
import os
from pathlib import Path
import re
import runpy
import subprocess
import sys
import tarfile

MASTER = '.doctrine/history_archive/windows.json'
MAX_BYTES = 4_000_000
MAX_LINES = 60_000
TAR_BYTES = 8 * 1024 * 1024
SEGMENT = re.compile(r'docs/history/(?:bedrock-scaffold-changelog|stitchcad-(?:changelog|devnotes|defects|g0-contract-(?:changelog|decisions|evidence))-part[1-9][0-9]*)\.md\Z')
SHA = re.compile(r'[0-9a-f]{64}\Z')
REV = re.compile(r'[0-9a-f]{40}\Z')
PROJECT = Path(__file__).resolve().parent.parent
PROFILE_SOURCE = PROJECT / 'scripts/local_environment.py'
GENERATIONS_SOURCE = PROJECT / 'scripts/history_generations.py'


class Refusal(ValueError):
    """A malformed, unsafe, missing or over-budget retained record."""


def require(condition, message):
    if not condition:
        raise Refusal(message)


def digest(data):
    return hashlib.sha256(data).hexdigest()


def fields(obj, expected):
    require(type(obj) is dict and set(obj) == set(expected.split()), 'unknown/missing schema fields')


def text(value, maximum):
    require(type(value) is str and 0 < len(value.encode('utf-8')) <= maximum
            and not any(ord(c) < 32 for c in value), 'invalid bounded text')


def integer(value, maximum):
    require(type(value) is int and 0 <= value <= maximum, 'invalid integer/bound')


def local_path(root, relative):
    text(relative, 200)
    parts = relative.split('/')
    require(all(p not in ('', '.', '..') for p in parts) and not relative.startswith('/'), 'unsafe path')
    p = root
    for component in parts:
        p = p / component
        require(not p.is_symlink(), 'symlink path refused: ' + relative)
        if p.exists():
            require(not (p.is_dir() and (p / '.git').exists()), 'nested Git repository boundary: ' + relative)
            require(p.stat().st_dev == root.stat().st_dev, 'off-volume path: ' + relative)
    return p


def bounded_read(path, maximum):
    require(path.is_file(), 'missing file: ' + str(path.name))
    with path.open('rb') as stream:
        data = stream.read(maximum + 1)
    require(len(data) <= maximum, 'byte bound exceeded: ' + str(path.name))
    return data


def no_duplicate_keys(pairs):
    obj = {}
    for key, value in pairs:
        require(key not in obj, 'duplicate JSON key: ' + key)
        obj[key] = value
    return obj


def load_json(root, path, maximum):
    data = bounded_read(local_path(root, path), maximum)
    return json.loads(data, object_pairs_hook=no_duplicate_keys), len(data)


def _master_v1(root):
    obj, _ = load_json(root, MASTER, 8192)
    fields(obj, 'version windows')
    require(type(obj['version']) is int and obj['version'] == 1, 'unsupported schema version')
    windows = obj['windows']
    require(type(windows) is list and 1 <= len(windows) <= 16, 'window count bound')
    require(all(type(p) is str and re.fullmatch(r'\.doctrine/history_archive/window[1-9][0-9]*\.json', p)
                for p in windows), 'invalid manifest path')
    require(len(set(windows)) == len(windows), 'duplicate window')
    return windows


def record_stats(data):
    require(len(data) <= 160000 and data.count(b'\n') <= 2000, 'logical per-record bound exceeded')
    require(max((len(line) for line in data.split(b'\n')), default=0) <= 400, 'logical maxline bound exceeded')
    return data.count(b'\n'), len(data), digest(data)


def _inventory_v1(root):
    """Return complete verified logical bytes and actual resident pressure."""
    windows = master(root)
    records = {}
    controls = {MASTER}
    payloads = set()
    catalogs = set()
    control_bytes = local_path(root, MASTER).stat().st_size
    decoded_bytes = 0
    decoded_lines = 0
    for manifest_path in windows:
        manifest, size = load_json(root, manifest_path, 65536)
        controls.add(manifest_path)
        control_bytes += size
        fields(manifest, 'version id source_revision created owner reason payload payload_sha256 catalog members')
        require(type(manifest['version']) is int and manifest['version'] == 1, 'unsupported manifest version')
        window_id = Path(manifest_path).stem
        require(manifest['id'] == window_id, 'window identity mismatch')
        require(type(manifest['source_revision']) is str and REV.fullmatch(manifest['source_revision']), 'invalid source revision')
        require(type(manifest['created']) is str and re.fullmatch(r'[0-9]{4}-[0-9]{2}-[0-9]{2}', manifest['created']), 'invalid date')
        datetime.date.fromisoformat(manifest['created'])
        text(manifest['owner'], 80)
        text(manifest['reason'], 400)
        sha = manifest['payload_sha256']
        require(type(sha) is str and SHA.fullmatch(sha), 'invalid payload digest')
        payload = manifest['payload']
        require(payload == 'docs/history/payloads/' + sha + '.tar.gz', 'content-address mismatch')
        require(payload not in payloads, 'duplicate payload')
        payloads.add(payload)
        catalog = manifest['catalog']
        require(catalog == 'docs/history/' + window_id + '.md', 'catalog identity mismatch')
        catalogs.add(catalog)
        catalog_data = bounded_read(local_path(root, catalog), 160000)
        members = manifest['members']
        require(type(members) is list and 1 <= len(members) <= 256, 'member count bound')
        expected = {}
        for row in members:
            fields(row, 'path lines bytes sha256')
            path = row['path']
            text(path, 200)
            require(type(path) is str and SEGMENT.fullmatch(path), 'invalid logical path')
            require(path not in records and path not in expected, 'duplicate logical identity')
            integer(row['lines'], 2000)
            integer(row['bytes'], 160000)
            require(type(row['sha256']) is str and SHA.fullmatch(row['sha256']), 'invalid member digest')
            expected[path] = row
        anchors = re.findall(rb'^### ([a-z0-9-]+\.md)$', catalog_data, re.M)
        require(anchors == [Path(p).name.encode() for p in expected], 'catalog membership/order mismatch')
        packed = bounded_read(local_path(root, payload), 1024 * 1024)
        require(digest(packed) == sha, 'payload digest mismatch')
        with gzip.GzipFile(fileobj=io.BytesIO(packed)) as zipped:
            tar_bytes = zipped.read(TAR_BYTES + 1)
        require(len(tar_bytes) <= TAR_BYTES, 'decompressed tar bound exceeded')
        seen = set()
        directories = set()
        with tarfile.open(fileobj=io.BytesIO(tar_bytes), mode='r:') as archive:
            for member in archive:
                require(member.name not in seen, 'duplicate tar member')
                seen.add(member.name)
                if member.isdir():
                    require(member.name in ('docs', 'docs/history'), 'unexpected tar directory')
                    directories.add(member.name)
                    continue
                require(member.type in (tarfile.REGTYPE, tarfile.AREGTYPE) and member.name in expected, 'unexpected/unsafe tar member')
                row = expected[member.name]
                require(member.size == row['bytes'], 'tar/member size mismatch')
                stream = archive.extractfile(member)
                require(stream is not None, 'unreadable member')
                with stream:
                    data = stream.read(160001)
                require(record_stats(data) == (row['lines'], row['bytes'], row['sha256']), 'member identity mismatch')
                decoded_bytes += len(data)
                decoded_lines += data.count(b'\n')
                require(decoded_bytes <= MAX_BYTES and decoded_lines <= MAX_LINES, 'decoded history aggregate bound exceeded')
                records[member.name] = data
            require(not tar_bytes[archive.offset:].strip(b'\0'), 'unparsed nonzero tar tail')
        require(seen == set(expected) | directories, 'tar membership mismatch')
        require(set(expected) <= set(records), 'missing tar member')
    require(control_bytes <= 262144, 'aggregate control bound exceeded')
    # All resident archive inputs must be classified; an orphan payload/manifest
    # cannot silently escape the reader or combined accounting.
    for path in (root / '.doctrine/history_archive').rglob('*'):
        local_path(root, path.relative_to(root).as_posix())
    actual_controls = {p.relative_to(root).as_posix() for p in (root / '.doctrine/history_archive').rglob('*') if p.is_file()}
    require(actual_controls == controls, 'orphan/missing archive control')
    actual_payloads = {p.relative_to(root).as_posix() for p in (root / 'docs/history/payloads').rglob('*') if p.is_file()}
    require(actual_payloads == payloads, 'orphan/missing payload')
    resident = control_bytes
    working = 0
    for path in (root / 'docs/history').rglob('*'):
        relative = path.relative_to(root).as_posix()
        require(not path.is_symlink(), 'symlink in history')
        if not path.is_file():
            continue
        local_path(root, relative)
        resident += path.stat().st_size
        if relative in payloads:
            continue
        require(relative in catalogs or SEGMENT.fullmatch(relative), 'unclassified resident history file')
        working += 1
        data = bounded_read(path, 160000)
        record_stats(data)
        if relative in catalogs:
            continue
        require(relative not in records, 'raw/packed duplicate logical identity')
        decoded_bytes += len(data)
        decoded_lines += data.count(b'\n')
        require(decoded_bytes <= MAX_BYTES and decoded_lines <= MAX_LINES, 'decoded history aggregate bound exceeded')
        records[relative] = data
    require(working <= 64, 'working Markdown count bound exceeded')
    logical_bytes = sum(len(data) for data in records.values())
    logical_lines = sum(data.count(b'\n') for data in records.values())
    # Catalogs consume the original history aggregate too, not just resident space.
    for catalog in catalogs:
        data = local_path(root, catalog).read_bytes()
        logical_bytes += len(data)
        logical_lines += data.count(b'\n')
    require(logical_bytes <= MAX_BYTES and logical_lines <= MAX_LINES, 'decoded history aggregate bound exceeded')
    require(resident <= MAX_BYTES, 'resident history aggregate bound exceeded')
    return records, (working, logical_lines, logical_bytes, resident)



def _v2_json(root, path, maximum):
    data = bounded_read(local_path(root, path), maximum)
    require(data.count(b'\n') <= 2200, 'control line bound exceeded')
    require(max((len(line) for line in data.split(b'\n')), default=0) <= 400, 'control maxline bound exceeded')
    return json.loads(data, object_pairs_hook=no_duplicate_keys), data


def _row_identity(row, *, maximum=160000, lines=2000):
    text(row['path'], 200)
    integer(row['lines'], lines)
    integer(row['bytes'], maximum)
    require(type(row['sha256']) is str and SHA.fullmatch(row['sha256']), 'invalid member digest')


def _master_v2(root):
    document, data = _v2_json(root, MASTER, 8192)
    fields(document, 'version windows legacy_catalogs')
    require(type(document['version']) is int and document['version'] == 2, 'unsupported schema version')
    windows = document['windows']
    require(type(windows) is list and 1 <= len(windows) <= 16, 'window count bound')
    require(all(type(path) is str and re.fullmatch(r'\.doctrine/history_archive/window[1-9][0-9]*\.json', path)
                for path in windows), 'invalid manifest path')
    require(len(set(windows)) == len(windows), 'duplicate window')
    legacy = document['legacy_catalogs']
    require(type(legacy) is list and len(legacy) == 16, 'legacy catalog count bound')
    paths = set()
    for row in legacy:
        fields(row, 'path lines bytes sha256')
        _row_identity(row)
        require(re.fullmatch(r'docs/history/window[1-9][0-9]*\.md', row['path']), 'invalid legacy catalog path')
        require(row['path'] not in paths, 'duplicate legacy catalog')
        paths.add(row['path'])
    return document, data


def master(root):
    """Validated active window paths for either schema; the original v1 validator is unchanged."""
    document, _ = load_json(root, MASTER, 8192)
    if _uses_v3_document(document):
        _document, _data, groups = generation_reader()['describe'](root, globals())
        return [path for _row, group, _bytes in groups for path in group['windows']]
    if type(document) is dict and type(document.get('version')) is int and document['version'] == 2:
        return _master_v2(root)[0]['windows']
    return _master_v1(root)


def _backing_limit(path):
    if path == MASTER:
        return 8192
    if re.fullmatch(r'\.doctrine/history_archive/window[1-9][0-9]*\.json', path):
        return 65536
    if re.fullmatch(r'docs/history/payloads/[0-9a-f]{64}\.tar\.gz', path):
        return 1048576
    raise Refusal('invalid backing path')


def _validate_legacy_evidence(records, backings, legacy_catalogs):
    """Validate original metadata and byte bindings without decompressing any old payload."""
    require(MASTER in backings, 'missing legacy master')
    original = json.loads(backings[MASTER], object_pairs_hook=no_duplicate_keys)
    fields(original, 'version windows')
    require(type(original['version']) is int and original['version'] == 1, 'invalid legacy schema')
    windows = original['windows']
    require(type(windows) is list and len(windows) == 16, 'legacy window count bound')
    require(all(type(path) is str and re.fullmatch(r'\.doctrine/history_archive/window[1-9][0-9]*\.json', path)
                for path in windows), 'invalid legacy manifest path')
    require(len(set(windows)) == len(windows), 'duplicate legacy window')
    expected = {MASTER, *windows}
    catalogs, logical = set(), set()
    controls = len(backings[MASTER])
    control_lines = backings[MASTER].count(b'\n')
    for control in (MASTER, *windows):
        require(control in backings, 'missing legacy control')
        data = backings[control]
        require(data.count(b'\n') <= 2200, 'legacy control line bound exceeded')
        require(max((len(line) for line in data.split(b'\n')), default=0) <= 400, 'legacy control maxline bound exceeded')
    for path in windows:
        require(path in backings, 'missing legacy manifest')
        controls += len(backings[path])
        control_lines += backings[path].count(b'\n')
        manifest = json.loads(backings[path], object_pairs_hook=no_duplicate_keys)
        fields(manifest, 'version id source_revision created owner reason payload payload_sha256 catalog members')
        require(type(manifest['version']) is int and manifest['version'] == 1, 'invalid legacy manifest version')
        require(manifest['id'] == Path(path).stem, 'legacy window identity mismatch')
        require(type(manifest['source_revision']) is str and REV.fullmatch(manifest['source_revision']), 'invalid source revision')
        require(type(manifest['created']) is str and re.fullmatch(r'[0-9]{4}-[0-9]{2}-[0-9]{2}', manifest['created']), 'invalid date')
        datetime.date.fromisoformat(manifest['created'])
        text(manifest['owner'], 80)
        text(manifest['reason'], 400)
        payload, checksum = manifest['payload'], manifest['payload_sha256']
        require(type(checksum) is str and SHA.fullmatch(checksum), 'invalid payload digest')
        require(payload == 'docs/history/payloads/' + checksum + '.tar.gz', 'content-address mismatch')
        require(payload not in expected, 'duplicate legacy payload')
        expected.add(payload)
        require(payload in backings and digest(backings[payload]) == checksum, 'legacy payload identity mismatch')
        catalog = manifest['catalog']
        require(catalog == 'docs/history/' + manifest['id'] + '.md', 'legacy catalog identity mismatch')
        catalogs.add(catalog)
        members = manifest['members']
        require(type(members) is list and 1 <= len(members) <= 256, 'member count bound')
        for row in members:
            fields(row, 'path lines bytes sha256')
            _row_identity(row)
            require(SEGMENT.fullmatch(row['path']), 'invalid logical path')
            require(row['path'] not in logical, 'duplicate legacy logical identity')
            logical.add(row['path'])
            require(row['path'] in records and record_stats(records[row['path']]) ==
                    (row['lines'], row['bytes'], row['sha256']), 'legacy logical identity mismatch')
    require(set(backings) == expected, 'legacy backing membership mismatch')
    require(catalogs == set(legacy_catalogs), 'legacy catalog membership mismatch')
    require(controls <= 262144 and control_lines <= 20000, 'legacy control bound exceeded')


def _inventory_v2(root):
    """Decode one flat level; old compressed payloads remain opaque retained evidence."""
    document, master_data = _master_v2(root)
    records, backings, manifests = {}, {}, {}
    controls, payloads, catalogs = {MASTER}, set(), {}
    control_bytes = len(master_data)
    control_lines = master_data.count(b'\n')
    decoded_bytes, decoded_lines = 0, 0
    indexed = set()
    for manifest_path in document['windows']:
        manifest, control = _v2_json(root, manifest_path, 65536)
        control_bytes += len(control)
        control_lines += control.count(b'\n')
        controls.add(manifest_path)
        fields(manifest, 'version id source_revision created owner reason payload payload_sha256 catalog members')
        require(type(manifest['version']) is int and manifest['version'] == 2, 'unsupported manifest version')
        identifier = Path(manifest_path).stem
        require(manifest['id'] == identifier, 'window identity mismatch')
        require(type(manifest['source_revision']) is str and REV.fullmatch(manifest['source_revision']), 'invalid source revision')
        require(type(manifest['created']) is str and re.fullmatch(r'[0-9]{4}-[0-9]{2}-[0-9]{2}', manifest['created']), 'invalid date')
        datetime.date.fromisoformat(manifest['created'])
        text(manifest['owner'], 80)
        text(manifest['reason'], 400)
        checksum, payload, catalog = manifest['payload_sha256'], manifest['payload'], manifest['catalog']
        require(type(checksum) is str and SHA.fullmatch(checksum), 'invalid payload digest')
        require(payload == 'docs/history/payloads/' + checksum + '.tar.gz', 'content-address mismatch')
        require(payload not in payloads, 'duplicate payload')
        payloads.add(payload)
        require(catalog == 'docs/history/' + identifier + '.md', 'catalog identity mismatch')
        catalog_data = bounded_read(local_path(root, catalog), 160000)
        record_stats(catalog_data)
        catalogs[catalog] = catalog_data
        members = manifest['members']
        require(type(members) is list and 1 <= len(members) <= 256, 'member count bound')
        expected = {}
        for row in members:
            fields(row, 'path lines bytes sha256 kind')
            require(type(row['kind']) is str and row['kind'] in ('record', 'backing'), 'invalid member kind')
            text(row['path'], 200)
            if row['kind'] == 'record':
                require(SEGMENT.fullmatch(row['path']), 'invalid logical path')
                _row_identity(row)
            else:
                _row_identity(row, maximum=_backing_limit(row['path']), lines=MAX_LINES)
            require(row['path'] not in indexed, 'duplicate logical identity')
            indexed.add(row['path'])
            expected[row['path']] = row
        headings = re.findall(rb'^### (.+)$', catalog_data, re.M)
        require(headings == [Path(path).name.encode() for path in expected], 'catalog membership/order mismatch')
        packed = bounded_read(local_path(root, payload), 1048576)
        require(digest(packed) == checksum, 'payload digest mismatch')
        with gzip.GzipFile(fileobj=io.BytesIO(packed)) as stream:
            raw = stream.read(TAR_BYTES + 1)
        require(len(raw) <= TAR_BYTES, 'decompressed tar bound exceeded')
        seen, directories = set(), set()
        with tarfile.open(fileobj=io.BytesIO(raw), mode='r:') as archive:
            for member in archive:
                require(member.name not in seen, 'duplicate tar member')
                seen.add(member.name)
                if member.isdir():
                    require(member.name in ('docs', 'docs/history', 'docs/history/payloads',
                                             '.doctrine', '.doctrine/history_archive'), 'unexpected tar directory')
                    directories.add(member.name)
                    continue
                require(member.type in (tarfile.REGTYPE, tarfile.AREGTYPE) and member.name in expected, 'unexpected/unsafe tar member')
                row = expected[member.name]
                require(member.size == row['bytes'], 'tar/member size mismatch')
                stream = archive.extractfile(member)
                require(stream is not None, 'unreadable member')
                with stream:
                    data = stream.read(row['bytes'] + 1)
                require((data.count(b'\n'), len(data), digest(data)) ==
                        (row['lines'], row['bytes'], row['sha256']), 'member identity mismatch')
                if row['kind'] == 'record':
                    record_stats(data)
                    records[member.name] = data
                else:
                    backings[member.name] = data
                decoded_bytes += len(data)
                decoded_lines += data.count(b'\n')
                require(decoded_bytes <= MAX_BYTES and decoded_lines <= MAX_LINES, 'decoded history aggregate bound exceeded')
            require(not raw[archive.offset:].strip(b'\0'), 'unparsed nonzero tar tail')
        require(seen == set(expected) | directories, 'tar membership mismatch')
        manifests[manifest_path] = manifest
    require(control_bytes <= 262144 and control_lines <= 20000, 'aggregate control bound exceeded')
    legacy_catalogs = {}
    for row in document['legacy_catalogs']:
        require(row['path'] not in catalogs, 'legacy/active catalog collision')
        data = bounded_read(local_path(root, row['path']), 160000)
        require(record_stats(data) == (row['lines'], row['bytes'], row['sha256']), 'legacy catalog identity mismatch')
        legacy_catalogs[row['path']] = data
    catalogs.update(legacy_catalogs)
    for path in (root / '.doctrine/history_archive').rglob('*'):
        local_path(root, path.relative_to(root).as_posix())
    actual_controls = {path.relative_to(root).as_posix() for path in (root / '.doctrine/history_archive').rglob('*') if path.is_file()}
    require(actual_controls == controls, 'orphan/missing archive control')
    actual_payloads = {path.relative_to(root).as_posix() for path in (root / 'docs/history/payloads').rglob('*') if path.is_file()}
    require(actual_payloads == payloads, 'orphan/missing payload')
    resident, working = control_bytes, 0
    for path in (root / 'docs/history').rglob('*'):
        relative = path.relative_to(root).as_posix()
        require(not path.is_symlink(), 'symlink in history')
        if not path.is_file():
            continue
        local_path(root, relative)
        resident += path.stat().st_size
        if relative in payloads:
            continue
        require(relative in catalogs or SEGMENT.fullmatch(relative), 'unclassified resident history file')
        working += 1
        data = bounded_read(path, 160000)
        record_stats(data)
        if relative in catalogs:
            continue
        require(relative not in records, 'raw/packed duplicate logical identity')
        records[relative] = data
        decoded_bytes += len(data)
        decoded_lines += data.count(b'\n')
    decoded_bytes += sum(map(len, catalogs.values()))
    decoded_lines += sum(data.count(b'\n') for data in catalogs.values())
    require(working <= 64, 'working Markdown count bound exceeded')
    require(decoded_bytes <= MAX_BYTES and decoded_lines <= MAX_LINES, 'decoded history aggregate bound exceeded')
    require(resident <= MAX_BYTES, 'resident history aggregate bound exceeded')
    _validate_legacy_evidence(records, backings, legacy_catalogs)
    return records, (working, decoded_lines, decoded_bytes, resident), backings, manifests


def inventory(root):
    """Return verified logical bytes/pressure; v1 and v2 retain the same public return shape."""
    document, _ = load_json(root, MASTER, 8192)
    if _uses_v3_document(document):
        records, pressure, _backings, _manifests = generation_reader()['inventory'](root, globals())
        return records, pressure
    if type(document) is dict and type(document.get('version')) is int and document['version'] == 2:
        records, pressure, _backings, _manifests = _inventory_v2(root)
        return records, pressure
    return _inventory_v1(root)


def archive_root(value):
    root = Path(value).absolute()
    require(root == PROJECT or root.is_relative_to(PROJECT / 'target'), 'fixture root must be under project target/')
    if root != PROJECT:
        local_path(PROJECT, root.relative_to(PROJECT).as_posix())
    require(root.is_dir(), 'archive root is not a directory')
    return root


def load_profile():
    parent = PROFILE_SOURCE.parent
    require(not parent.is_symlink() and parent.is_dir() and parent.stat().st_dev == PROJECT.stat().st_dev,
            'unsafe archive profile parent')
    marker = parent / '.git'
    require(not (marker.exists() or marker.is_symlink()), 'archive profile crosses Git boundary')
    require(not PROFILE_SOURCE.is_symlink() and PROFILE_SOURCE.is_file() and
            PROFILE_SOURCE.stat().st_dev == PROJECT.stat().st_dev, 'unsafe archive profile source')
    return runpy.run_path(str(PROFILE_SOURCE))


def backing_sources(root, profile):
    """Complete nonproducing input validation, independent of unused store settings."""
    sources = [Path(__file__), PROFILE_SOURCE, GENERATIONS_SOURCE, root / MASTER]
    folders = [root / '.doctrine/history_archive', root / 'docs/history']
    for folder in folders:
        profile['directory'](PROJECT, profile['producer_path'](PROJECT, folder))
        for path in sorted(folder.iterdir()):
            require(not path.is_symlink(), 'symlink archive backing input')
            if path.is_dir():
                folders.append(path)
            elif path not in sources:
                sources.append(path)
    for path in sources:
        relative = profile['producer_path'](PROJECT, path)
        profile['directory'](PROJECT, Path(relative).parent.as_posix())
        require(not path.is_symlink() and path.is_file() and path.stat().st_dev == PROJECT.stat().st_dev,
                'archive source is not a local regular file')
    return tuple(sources)


def metadata_environment(root):
    root = archive_root(root)
    backing_sources(root, load_profile())
    environment = {name: value for name, value in os.environ.items() if not name.startswith('GIT_')}
    environment.update(GIT_CONFIG_NOSYSTEM='1', GIT_CONFIG_GLOBAL=os.devnull,
                       GIT_OPTIONAL_LOCKS='0', GIT_LITERAL_PATHSPECS='1', GIT_TERMINAL_PROMPT='0')
    return environment


def materialization_plan(profile, root, destination, records, *, activate=False):
    files = tuple(destination / name for name in sorted(records))
    folders = {destination, *(parent for parent in destination.parents if parent.is_relative_to(PROJECT / 'target'))}
    for path in files:
        folders.update(parent for parent in path.parents
                       if parent == destination or parent.is_relative_to(destination))
    sources = backing_sources(root, profile)
    profile['plan_producer'](PROJECT, sources=sources, directories=tuple(sorted(folders)), output_files=files)
    environment = profile['environment_for'](PROJECT, os.environ, create=False)
    for name in profile['STORES']:
        store = Path(environment[name])
        require(not store.is_relative_to(destination), 'archive destination contains a store')
        require(not any(store.is_relative_to(root / base) for base in ('.doctrine/history_archive', 'docs/history')),
                'archive backing contains a store')
    if activate:
        os.environ.update(profile['environment_for'](PROJECT, os.environ, create=True))


def materialization_file_plan(profile, destination, output):
    profile['plan_producer'](PROJECT, sources=(Path(__file__), PROFILE_SOURCE),
                             directories=(destination, output.parent), output_files=(output,))



def _uses_v2(root):
    document, _ = load_json(root, MASTER, 8192)
    return type(document) is dict and type(document.get('version')) is int and document['version'] in (2, 3)


def _uses_v3_document(document):
    return type(document) is dict and type(document.get('version')) is int and document['version'] == 3


def generation_reader():
    """Load the bounded local sibling without activating or loading the producer profile."""
    source = local_path(PROJECT, 'scripts/history_generations.py')
    namespace = dict(__file__=str(source), __name__='history_generations')
    exec(compile(bounded_read(source, 65536), str(source), 'exec'), namespace)
    return namespace


def flat_snapshot(root):
    document, _ = load_json(root, MASTER, 8192)
    if _uses_v3_document(document):
        return generation_reader()['inventory'](root, globals())
    return _inventory_v2(root)


def flat_legacy_catalogs(root):
    document, _ = load_json(root, MASTER, 8192)
    if _uses_v3_document(document):
        _document, _data, groups = generation_reader()['describe'](root, globals())
        return groups[0][1]['legacy_catalogs']
    return _master_v2(root)[0]['legacy_catalogs']


def _git_blob(root, revision, path, maximum):
    require(revision == 'HEAD' or (type(revision) is str and REV.fullmatch(revision)), 'invalid source revision')
    text(path, 200)
    require(path == MASTER or SEGMENT.fullmatch(path) or
            re.fullmatch(r'\.doctrine/history_archive/window[1-9][0-9]*\.json', path) or
            re.fullmatch(r'docs/history/window[1-9][0-9]*\.md', path) or
            re.fullmatch(r'docs/history/payloads/[0-9a-f]{64}\.tar\.gz', path), 'invalid source path')
    environment = metadata_environment(root)
    object_name = revision + ':' + path
    size = subprocess.run(['git', '-C', str(root), 'cat-file', '-s', object_name],
                          capture_output=True, check=True, env=environment).stdout
    require(re.fullmatch(rb'[0-9]{1,20}\n', size) and int(size) <= maximum, 'Git source byte bound exceeded')
    data = subprocess.run(['git', '-C', str(root), 'show', object_name],
                          capture_output=True, check=True, env=environment).stdout
    require(len(data) == int(size), 'Git source size mismatch')
    return data


def _git_paths(root, revision):
    require(type(revision) is str and REV.fullmatch(revision), 'invalid source revision')
    result = subprocess.run(['git', '-C', str(root), 'ls-tree', '--full-tree', '-r', '--name-only', revision, '--', 'docs/history'],
                            capture_output=True, check=True, env=metadata_environment(root)).stdout
    require(len(result) <= 1048576, 'Git source path inventory bound exceeded')
    return {path for path in result.decode().splitlines() if SEGMENT.fullmatch(path)}


def _head_revision(root):
    data = subprocess.run(['git', '-C', str(root), 'rev-parse', '--verify', 'HEAD'],
                          capture_output=True, check=True, env=metadata_environment(root)).stdout.decode().strip()
    require(REV.fullmatch(data), 'invalid HEAD revision')
    return data


def _legacy_records(backings):
    """Explicit source/transition proof only; ordinary flat-v2 reads never expand old payloads."""
    original = json.loads(backings[MASTER], object_pairs_hook=no_duplicate_keys)
    records = {}
    total, lines = 0, 0
    for name in original['windows']:
        manifest = json.loads(backings[name], object_pairs_hook=no_duplicate_keys)
        expected = {row['path']: row for row in manifest['members']}
        with gzip.GzipFile(fileobj=io.BytesIO(backings[manifest['payload']])) as stream:
            raw = stream.read(TAR_BYTES + 1)
        require(len(raw) <= TAR_BYTES, 'legacy source tar bound exceeded')
        seen, directories = set(), set()
        with tarfile.open(fileobj=io.BytesIO(raw), mode='r:') as archive:
            for member in archive:
                require(member.name not in seen, 'duplicate legacy source tar member')
                seen.add(member.name)
                if member.isdir():
                    require(member.name in ('docs', 'docs/history'), 'unexpected legacy source directory')
                    directories.add(member.name)
                    continue
                require(member.type in (tarfile.REGTYPE, tarfile.AREGTYPE) and member.name in expected,
                        'unexpected/unsafe legacy source member')
                row = expected[member.name]
                require(member.size == row['bytes'], 'legacy source member size mismatch')
                stream = archive.extractfile(member)
                require(stream is not None, 'unreadable legacy source member')
                with stream:
                    data = stream.read(160001)
                require(record_stats(data) == (row['lines'], row['bytes'], row['sha256']), 'legacy source member identity mismatch')
                require(member.name not in records, 'duplicate legacy source identity')
                records[member.name] = data
                total += len(data)
                lines += data.count(b'\n')
                require(total <= MAX_BYTES and lines <= MAX_LINES, 'legacy source aggregate bound exceeded')
            require(not raw[archive.offset:].strip(b'\0'), 'unparsed legacy source tar tail')
        require(seen == set(expected) | directories, 'legacy source membership mismatch')
    return records


def _v1_source_snapshot(root, revision, backings, catalogs):
    require(_git_blob(root, revision, MASTER, 8192) == backings[MASTER], 'source legacy master mismatch')
    for path, data in backings.items():
        require(_git_blob(root, revision, path, _backing_limit(path)) == data, 'source backing reconstruction mismatch: ' + path)
    records = _legacy_records(backings)
    for path in _git_paths(root, revision):
        require(path not in records, 'source raw/packed duplicate')
        records[path] = _git_blob(root, revision, path, 160000)
        record_stats(records[path])
    original_catalogs = {}
    for path in catalogs:
        original_catalogs[path] = _git_blob(root, revision, path, 160000)
        require(original_catalogs[path] == bounded_read(local_path(root, path), 160000), 'source legacy catalog mismatch')
    content = [*records.values(), *original_catalogs.values()]
    require(sum(map(len, content)) <= MAX_BYTES and sum(data.count(b'\n') for data in content) <= MAX_LINES,
            'source history aggregate bound exceeded')
    return records


def backing_inventory(root):
    """Original retained control/payload bytes; plain byte retrieval does not need historical Git."""
    if _uses_v2(root):
        return flat_snapshot(root)[2]
    inventory(root)
    paths = {MASTER}
    for name in master(root):
        manifest, _ = load_json(root, name, 65536)
        paths.update((name, manifest['payload']))
    return {path: bounded_read(local_path(root, path), _backing_limit(path)) for path in paths}


def _v2_source_proof(root, window):
    records, _pressure, backings, manifests = flat_snapshot(root)
    path = '.doctrine/history_archive/' + window + '.json'
    if path not in manifests:
        require(path in backings and path != MASTER and path.endswith('.json'), 'unknown window')
        manifest = json.loads(backings[path], object_pairs_hook=no_duplicate_keys)
        revision = manifest['source_revision']
        expected = _git_paths(root, revision)
        require(expected == {row['path'] for row in manifest['members']}, 'source snapshot membership mismatch')
        for name in expected:
            require(records[name] == _git_blob(root, revision, name, 160000), 'source reconstruction mismatch: ' + name)
        return len(expected), 0, 0
    manifest = manifests[path]
    revision = manifest['source_revision']
    original = json.loads(_git_blob(root, revision, MASTER, 8192), object_pairs_hook=no_duplicate_keys)
    if type(original.get('version')) is int and original['version'] == 1:
        legacy_catalogs = {row['path'] for row in flat_legacy_catalogs(root)}
        expected = _v1_source_snapshot(root, revision, backings, legacy_catalogs)
        cohort = {row['path']: row for value in manifests.values() if value['source_revision'] == revision
                  for row in value['members']}
        require(set(cohort) == set(expected) | set(backings), 'source cohort membership mismatch')
        for name, data in expected.items():
            require(cohort[name]['kind'] == 'record' and records[name] == data, 'source reconstruction mismatch: ' + name)
        for name in backings:
            require(cohort[name]['kind'] == 'backing', 'source cohort backing kind mismatch')
        return len(expected), len(backings), len(legacy_catalogs)
    require(type(original.get('version')) is int and original['version'] in (2, 3), 'unsupported source schema')
    expected = _git_paths(root, revision)
    require(expected == {row['path'] for row in manifest['members']} and
            all(row['kind'] == 'record' for row in manifest['members']), 'source snapshot membership mismatch')
    for name in expected:
        require(records[name] == _git_blob(root, revision, name, 160000), 'source reconstruction mismatch: ' + name)
    return len(expected), 0, 0


def _guard_v2(root, old, saved_master):
    records, _pressure, backings, manifests = _inventory_v2(root)
    document = _master_v2(root)[0]
    if type(old.get('version')) is int and old['version'] == 1:
        require(backings[MASTER] == saved_master, 'committed legacy master changed')
        revision = _head_revision(root)
        catalogs = {row['path'] for row in document['legacy_catalogs']}
        original = _v1_source_snapshot(root, revision, backings, catalogs)
        require(set(original) <= set(records), 'committed logical record missing')
        require(all(records[name] == data for name, data in original.items()), 'committed logical record changed')
        initial = [value for value in manifests.values() if any(row['kind'] == 'backing' for row in value['members'])]
        require(initial and len({value['source_revision'] for value in initial}) == 1, 'ambiguous transition source')
        _v2_source_proof(root, initial[0]['id'])
        return
    fields(old, 'version windows legacy_catalogs')
    require(type(old['version']) is int and old['version'] == 2, 'unsupported committed archive schema')
    require(old['legacy_catalogs'] == document['legacy_catalogs'], 'committed legacy catalog descriptors changed')
    require(set(old['windows']) <= set(document['windows']), 'committed window removed')
    for path in old['windows']:
        original = _git_blob(root, 'HEAD', path, 65536)
        require(bounded_read(local_path(root, path), 65536) == original, 'committed manifest changed')
        manifest = json.loads(original, object_pairs_hook=no_duplicate_keys)
        for retained in (manifest['payload'], manifest['catalog']):
            bound = 1048576 if retained == manifest['payload'] else 160000
            require(bounded_read(local_path(root, retained), bound) == _git_blob(root, 'HEAD', retained, bound),
                    'committed retained file changed')


def _guard_raw_records(root):
    records, _pressure = inventory(root)
    revision = _head_revision(root)
    for path in _git_paths(root, revision):
        require(path in records, 'committed raw record missing')
        require(records[path] == _git_blob(root, revision, path, 160000), 'committed raw record changed')


def _guard_v3(root, old, saved_master):
    """Keep sealed controls byte-exact and permit only ordered active-window/generation growth."""
    engine = generation_reader()
    document, _data, groups = engine['describe'](root, globals())
    engine['inventory'](root, globals())
    if type(old.get('version')) is int and old['version'] == 2:
        require(len(groups) == 2 and groups[0][2] == saved_master, 'initial generation seal differs from v2 source')
        prior_windows = old['windows']
    else:
        prior = engine['descriptors'](old, globals())
        require(len(prior) <= len(document['generations']), 'committed generation removed')
        require(prior[:-1] == document['generations'][:len(prior) - 1], 'committed generation seal changed')
        prior_windows = []
        for index, row in enumerate(prior):
            original = _git_blob(root, 'HEAD', row['path'], 8192)
            require((original.count(b'\n'), len(original), digest(original)) ==
                    (row['lines'], row['bytes'], row['sha256']), 'committed generation control identity mismatch')
            previous = json.loads(original, object_pairs_hook=no_duplicate_keys)
            engine['group_document'](previous, index + 1, index == len(prior) - 1, globals())
            current = groups[index][1]
            require(previous['legacy_catalogs'] == current['legacy_catalogs'], 'committed legacy descriptors changed')
            require(previous['windows'] == current['windows'][:len(previous['windows'])], 'committed window order changed')
            if index < len(prior) - 1:
                require(groups[index][2] == original, 'committed sealed control changed')
            prior_windows.extend(previous['windows'])
    for path in prior_windows:
        original = _git_blob(root, 'HEAD', path, 65536)
        require(bounded_read(local_path(root, path), 65536) == original, 'committed manifest changed')
        manifest = json.loads(original, object_pairs_hook=no_duplicate_keys)
        for retained in (manifest['payload'], manifest['catalog']):
            bound = 1048576 if retained == manifest['payload'] else 160000
            require(bounded_read(local_path(root, retained), bound) == _git_blob(root, 'HEAD', retained, bound),
                    'committed retained file changed')


def _extended_cli():
    """Explicit added operations; original ordinary CLI stays unchanged below this adapter."""
    if not any(value in ('read-backing', 'prove-source', '-h', '--help') for value in sys.argv[1:]):
        return False
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', required=True, type=Path)
    parser.add_argument('command', choices=['verify', 'verify-retention', 'list', 'read', 'materialize', 'prove-source', 'read-backing'])
    parser.add_argument('argument', nargs='?')
    args = parser.parse_args()
    if args.command not in ('read-backing', 'prove-source'):
        return False
    root = archive_root(args.root)
    if args.command == 'read-backing':
        backings = backing_inventory(root)
        require(args.argument in backings, 'unknown backing identity')
        sys.stdout.buffer.write(backings[args.argument])
        return True
    if not _uses_v2(root):
        return False
    require(args.argument is not None and re.fullmatch(r'window[1-9][0-9]*', args.argument), 'window id required')
    records, backings, catalogs = _v2_source_proof(root, args.argument)
    print('archive source proof: %d byte-identical full files; %d original backings; %d legacy catalogs; 0 missing/extra' %
          (records, backings, catalogs))
    return True


def _committed_master(root):
    """Observe HEAD metadata with the original fail-closed absence/Git authority checks."""
    root = archive_root(root)
    result = subprocess.run(['git', '-C', str(root), 'show', 'HEAD:' + MASTER], capture_output=True,
                            check=False, env=metadata_environment(root))
    if result.returncode:
        absence = subprocess.run(['git', '-C', str(root), 'ls-tree', '--full-tree', '--name-only', 'HEAD', '--', MASTER],
                                 capture_output=True, check=False, env=metadata_environment(root))
        require(absence.returncode == 0 and absence.stdout == b'', 'committed HEAD metadata unavailable')
        return  # independently observed first transition; source reconstruction is separately proved
    return result


def guard_committed(root):
    """Preserve every committed byte, including an explicit validated v1-to-v2 relocation."""
    root = archive_root(root)
    result = _committed_master(root)
    if result is None:
        return
    old = json.loads(result.stdout, object_pairs_hook=no_duplicate_keys)
    require(type(old) is dict and type(old.get('version')) is int and old['version'] in (1, 2, 3),
            'unsupported committed archive schema')
    current = master(root)
    document, _ = load_json(root, MASTER, 8192)
    if _uses_v3_document(document):
        _guard_v3(root, old, result.stdout)
        _guard_raw_records(root)
        return
    if _uses_v2(root):
        _guard_v2(root, old, result.stdout)
        _guard_raw_records(root)
        return
    require(old['version'] == 1, 'unsupported committed archive schema')
    fields(old, 'version windows')
    require(set(old['windows']) <= set(current), 'committed window removed')
    for path in old['windows']:
        saved = subprocess.run(['git', '-C', str(root), 'show', 'HEAD:' + path], capture_output=True,
                               check=True, env=metadata_environment(root)).stdout
        require(local_path(root, path).read_bytes() == saved, 'committed manifest changed')
        manifest = json.loads(saved)
        for retained in (manifest['payload'], manifest['catalog']):
            saved_bytes = subprocess.run(['git', '-C', str(root), 'show', 'HEAD:' + retained], capture_output=True,
                                         check=True, env=metadata_environment(root)).stdout
            require(local_path(root, retained).read_bytes() == saved_bytes, 'committed retained file changed')
    _guard_raw_records(root)


def main():
    if _extended_cli():
        return
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', required=True, type=Path)
    parser.add_argument('command', choices=['verify', 'verify-retention', 'list', 'read', 'materialize', 'prove-source'])
    parser.add_argument('argument', nargs='?')
    args = parser.parse_args()
    project = Path(__file__).resolve().parent.parent
    root = archive_root(args.root)
    records, pressure = inventory(root)
    if args.command in ('verify', 'verify-retention', 'list'):
        require(args.argument is None, 'unexpected argument')
    if args.command == 'verify-retention':
        guard_committed(root)
    if args.command.startswith('verify'):
        print('archive: OK — %d logical records; %d working Markdown; %d decoded lines; %d decoded bytes; %d resident bytes' % ((len(records),) + pressure))
    elif args.command == 'list':
        print('\n'.join(sorted(records)))
    elif args.command == 'read':
        require(args.argument in records, 'unknown logical identity')
        sys.stdout.buffer.write(records[args.argument])
    elif args.command == 'prove-source':
        require(args.argument is not None, 'window id required')
        manifest_path = '.doctrine/history_archive/' + args.argument + '.json'
        require(manifest_path in master(root), 'unknown window')
        manifest, _ = load_json(root, manifest_path, 65536)
        revision = manifest['source_revision']
        paths = subprocess.run(['git', '-C', str(project), 'ls-tree', '-r', '--name-only', revision, 'docs/history'],
                               capture_output=True, check=True, env=metadata_environment(root)).stdout.decode().splitlines()
        source_paths = {p for p in paths if SEGMENT.fullmatch(p)}
        require(source_paths == {row['path'] for row in manifest['members']}, 'source snapshot membership mismatch')
        for path in sorted(source_paths):
            original = subprocess.run(['git', '-C', str(project), 'show', revision + ':' + path],
                                      capture_output=True, check=True, env=metadata_environment(root)).stdout
            require(records[path] == original, 'source reconstruction mismatch: ' + path)
        print('archive source proof: %d byte-identical full files; 0 missing/extra' % len(source_paths))
    elif args.command == 'materialize':
        require(args.argument is not None and args.argument.startswith('target/'), 'destination must be under target/')
        destination = local_path(root, args.argument)
        require(not destination.exists(), 'destination exists; overwrite refused')
        profile = load_profile()
        materialization_plan(profile, root, destination, records, activate=True)
        materialization_plan(profile, root, destination, records)
        destination.mkdir(parents=True)
        for path, data in sorted(records.items()):
            output = local_path(root, args.argument + '/' + path)
            materialization_file_plan(profile, destination, output)
            output.parent.mkdir(parents=True, exist_ok=True)
            materialization_file_plan(profile, destination, output)
            with output.open('xb') as stream:
                stream.write(data)
        print('archive: materialized %d records at %s' % (len(records), args.argument))


if __name__ == '__main__':
    try:
        main()
    except (Refusal, OSError, ValueError, EOFError, tarfile.TarError, subprocess.CalledProcessError, RecursionError) as error:
        print('ARCHIVE-RETENTION: REFUSED — ' + str(error), file=sys.stderr)
        sys.exit(1)
