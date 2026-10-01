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


def master(root):
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


def inventory(root):
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


def guard_committed(root):
    """Windows already retained at HEAD cannot disappear or be silently changed."""
    result = subprocess.run(['git', '-C', str(root), 'show', 'HEAD:' + MASTER], capture_output=True, check=False)
    if result.returncode:
        return  # first transition; source reconstruction is separately proved
    old = json.loads(result.stdout, object_pairs_hook=no_duplicate_keys)
    current = master(root)
    require(set(old['windows']) <= set(current), 'committed window removed')
    for path in old['windows']:
        saved = subprocess.run(['git', '-C', str(root), 'show', 'HEAD:' + path], capture_output=True, check=True).stdout
        require(local_path(root, path).read_bytes() == saved, 'committed manifest changed')
        manifest = json.loads(saved)
        for retained in (manifest['payload'], manifest['catalog']):
            saved_bytes = subprocess.run(['git', '-C', str(root), 'show', 'HEAD:' + retained], capture_output=True, check=True).stdout
            require(local_path(root, retained).read_bytes() == saved_bytes, 'committed retained file changed')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', required=True, type=Path)
    parser.add_argument('command', choices=['verify', 'verify-retention', 'list', 'read', 'materialize', 'prove-source'])
    parser.add_argument('argument', nargs='?')
    args = parser.parse_args()
    project = Path(__file__).resolve().parent.parent
    root = args.root.absolute()
    require(root == project or root.is_relative_to(project / 'target'), 'fixture root must be under project target/')
    require(root == local_path(project, root.relative_to(project).as_posix()) if root != project else True, 'invalid fixture root')
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
        paths = subprocess.run(['git', '-C', str(project), 'ls-tree', '-r', '--name-only', revision, 'docs/history'], capture_output=True, check=True).stdout.decode().splitlines()
        source_paths = {p for p in paths if SEGMENT.fullmatch(p)}
        require(source_paths == {row['path'] for row in manifest['members']}, 'source snapshot membership mismatch')
        for path in sorted(source_paths):
            original = subprocess.run(['git', '-C', str(project), 'show', revision + ':' + path], capture_output=True, check=True).stdout
            require(records[path] == original, 'source reconstruction mismatch: ' + path)
        print('archive source proof: %d byte-identical full files; 0 missing/extra' % len(source_paths))
    elif args.command == 'materialize':
        require(args.argument is not None and args.argument.startswith('target/'), 'destination must be under target/')
        destination = local_path(root, args.argument)
        require(not destination.exists(), 'destination exists; overwrite refused')
        destination.mkdir(parents=True)
        for path, data in sorted(records.items()):
            output = local_path(root, args.argument + '/' + path)
            output.parent.mkdir(parents=True, exist_ok=True)
            with output.open('xb') as stream:
                stream.write(data)
        print('archive: materialized %d records at %s' % (len(records), args.argument))


if __name__ == '__main__':
    try:
        main()
    except (Refusal, OSError, ValueError, EOFError, tarfile.TarError, subprocess.CalledProcessError, RecursionError) as error:
        print('ARCHIVE-RETENTION: REFUSED — ' + str(error), file=sys.stderr)
        sys.exit(1)
