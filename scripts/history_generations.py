"""Strict flat generation decoder; loaded by history_archive.py after source validation.

No filesystem effects, extraction, recursive archive activation, network or Git reads.
"""
import datetime
import gzip
import hashlib
import io
from pathlib import Path
import re
import tarfile

MAX_GENERATIONS = 8
GENERATION_BYTES = 4_000_000
GENERATION_LINES = 60_000
INDEX_BYTES = 8192
MAX_FILES = 137 + 128 + 192


def tree_identity(files):
    """Domain-separated identity of every indexed physical file; raw drafts are not sealed."""
    digest = hashlib.sha256(b'StitchCAD-history-generation-tree-v1\0')
    for path, (size, checksum) in sorted(files.items()):
        digest.update(path.encode('utf-8') + b'\0' + str(size).encode('ascii') + b'\0'
                      + checksum.encode('ascii') + b'\n')
    return digest.hexdigest()


def descriptors(document, a):
    a['fields'](document, 'version generations')
    a['require'](type(document['version']) is int and document['version'] == 3, 'unsupported generation schema')
    rows = document['generations']
    a['require'](type(rows) is list and 2 <= len(rows) <= MAX_GENERATIONS, 'generation count bound')
    for index, row in enumerate(rows, 1):
        a['fields'](row, 'path lines bytes sha256 tree_sha256')
        a['_row_identity'](row, maximum=INDEX_BYTES, lines=2200)
        a['require'](type(row['tree_sha256']) is str and a['SHA'].fullmatch(row['tree_sha256']), 'invalid generation tree digest')
        a['require'](row['path'] == '.doctrine/history_archive/generation%d.json' % index,
                     'generation path/order mismatch')
    return rows


def group_document(document, index, last, a):
    a['fields'](document, 'version windows legacy_catalogs')
    a['require'](type(document['version']) is int and document['version'] == 2, 'unsupported group schema')
    windows = document['windows']
    a['require'](type(windows) is list and (0 if last else 1) <= len(windows) <= 16, 'group window count bound')
    a['require'](all(type(path) is str and re.fullmatch(r'\.doctrine/history_archive/window[1-9][0-9]*\.json', path)
                     for path in windows), 'invalid group manifest path')
    a['require'](len(set(windows)) == len(windows), 'duplicate group window')
    legacy = document['legacy_catalogs']
    a['require'](type(legacy) is list and len(legacy) == (16 if index == 1 else 0), 'group legacy catalog count')
    paths = set()
    for row in legacy:
        a['fields'](row, 'path lines bytes sha256')
        a['_row_identity'](row)
        a['require'](re.fullmatch(r'docs/history/window[1-9][0-9]*\.md', row['path']), 'invalid legacy catalog path')
        a['require'](row['path'] not in paths, 'duplicate legacy catalog')
        paths.add(row['path'])
    return document


def describe(root, a):
    document, data = a['_v2_json'](root, a['MASTER'], INDEX_BYTES)
    rows = descriptors(document, a)
    groups, windows = [], set()
    for index, row in enumerate(rows, 1):
        group, content = a['_v2_json'](root, row['path'], INDEX_BYTES)
        a['require']((content.count(b'\n'), len(content), a['digest'](content)) ==
                     (row['lines'], row['bytes'], row['sha256']), 'generation control identity mismatch')
        group_document(group, index, index == len(rows), a)
        a['require'](not windows.intersection(group['windows']), 'window belongs to multiple generations')
        windows.update(group['windows'])
        groups.append((row, group, content))
    return document, data, groups


def physical_files(root, a):
    files = set()
    for folder, allowed in (('.doctrine/history_archive', set()),
                            ('docs/history', {'payloads'}), ('docs/history/payloads', set())):
        base = a['local_path'](root, folder)
        a['require'](base.is_dir(), 'missing archive directory')
        for path in base.iterdir():
            relative = path.relative_to(root).as_posix()
            a['local_path'](root, relative)
            if path.is_dir():
                a['require'](path.name in allowed, 'unclassified archive directory')
                continue
            a['require'](path.is_file() and path.stat().st_nlink == 1, 'unsafe archive file')
            a['require'](path.stat().st_size <= 1048576, 'archive physical file bound')
            files.add(relative)
            a['require'](len(files) <= MAX_FILES, 'archive physical count bound')
    return files


def decode_group(root, a, group, raw_paths, used):
    row, document, master_data = group
    records, backings, manifests, catalogs = {}, {}, {}, {}
    owned = {row['path']: (len(master_data), a['digest'](master_data))}
    control_bytes, control_lines = len(master_data), master_data.count(b'\n')
    decoded_bytes, decoded_lines = 0, 0

    def retain(path, data, *, control=False):
        nonlocal control_bytes, control_lines
        a['require'](path not in owned, 'physical backing belongs to multiple windows')
        owned[path] = (len(data), a['digest'](data))
        if control:
            control_bytes += len(data)
            control_lines += data.count(b'\n')
            a['require'](control_bytes <= 262144 and control_lines <= 20000, 'group control bound exceeded')

    def decoded(data):
        nonlocal decoded_bytes, decoded_lines
        decoded_bytes += len(data)
        decoded_lines += data.count(b'\n')
        a['require'](decoded_bytes <= GENERATION_BYTES and decoded_lines <= GENERATION_LINES,
                     'generation decoded bound exceeded')

    for path in document['windows']:
        manifest, control = a['_v2_json'](root, path, 65536)
        retain(path, control, control=True)
        a['fields'](manifest, 'version id source_revision created owner reason payload payload_sha256 catalog members')
        a['require'](type(manifest['version']) is int and manifest['version'] == 2, 'unsupported manifest version')
        identifier = Path(path).stem
        a['require'](manifest['id'] == identifier, 'window identity mismatch')
        a['require'](type(manifest['source_revision']) is str and a['REV'].fullmatch(manifest['source_revision']), 'invalid source revision')
        a['require'](type(manifest['created']) is str and re.fullmatch(r'[0-9]{4}-[0-9]{2}-[0-9]{2}', manifest['created']), 'invalid date')
        datetime.date.fromisoformat(manifest['created'])
        a['text'](manifest['owner'], 80)
        a['text'](manifest['reason'], 400)
        checksum, payload, catalog = manifest['payload_sha256'], manifest['payload'], manifest['catalog']
        a['require'](type(checksum) is str and a['SHA'].fullmatch(checksum), 'invalid payload digest')
        a['require'](payload == 'docs/history/payloads/' + checksum + '.tar.gz', 'content-address mismatch')
        a['require'](catalog == 'docs/history/' + identifier + '.md', 'catalog identity mismatch')
        catalog_data = a['bounded_read'](a['local_path'](root, catalog), 160000)
        a['record_stats'](catalog_data)
        retain(catalog, catalog_data)
        catalogs[catalog] = catalog_data
        decoded(catalog_data)
        members = manifest['members']
        a['require'](type(members) is list and 1 <= len(members) <= 256, 'member count bound')
        expected = {}
        for item in members:
            a['fields'](item, 'path lines bytes sha256 kind')
            a['require'](type(item['kind']) is str and item['kind'] in ('record', 'backing'), 'invalid member kind')
            a['text'](item['path'], 200)
            if item['kind'] == 'record':
                a['require'](a['SEGMENT'].fullmatch(item['path']), 'invalid logical path')
                a['_row_identity'](item)
            else:
                a['require'](row['path'].endswith('/generation1.json'), 'backing outside original generation')
                a['_row_identity'](item, maximum=a['_backing_limit'](item['path']), lines=GENERATION_LINES)
            a['require'](item['path'] not in used, 'duplicate logical identity')
            used.add(item['path'])
            expected[item['path']] = item
        a['require'](re.findall(rb'^### (.+)$', catalog_data, re.M) ==
                     [Path(name).name.encode() for name in expected], 'catalog membership/order mismatch')
        packed = a['bounded_read'](a['local_path'](root, payload), 1048576)
        retain(payload, packed)
        a['require'](a['digest'](packed) == checksum, 'payload digest mismatch')
        with gzip.GzipFile(fileobj=io.BytesIO(packed)) as stream:
            raw = stream.read(a['TAR_BYTES'] + 1)
        a['require'](len(raw) <= a['TAR_BYTES'], 'decompressed tar bound exceeded')
        seen, directories = set(), set()
        with tarfile.open(fileobj=io.BytesIO(raw), mode='r:') as archive:
            for member in archive:
                a['require'](member.name not in seen, 'duplicate tar member')
                seen.add(member.name)
                if member.isdir():
                    a['require'](member.name in ('docs', 'docs/history', 'docs/history/payloads',
                                                 '.doctrine', '.doctrine/history_archive'), 'unexpected tar directory')
                    directories.add(member.name)
                    continue
                a['require'](member.type in (tarfile.REGTYPE, tarfile.AREGTYPE) and member.name in expected,
                             'unexpected/unsafe tar member')
                item = expected[member.name]
                a['require'](member.size == item['bytes'], 'tar/member size mismatch')
                stream = archive.extractfile(member)
                a['require'](stream is not None, 'unreadable member')
                with stream:
                    content = stream.read(item['bytes'] + 1)
                a['require']((content.count(b'\n'), len(content), a['digest'](content)) ==
                             (item['lines'], item['bytes'], item['sha256']), 'member identity mismatch')
                if item['kind'] == 'record':
                    a['record_stats'](content)
                    records[member.name] = content
                else:
                    backings[member.name] = content
                decoded(content)
            a['require'](not raw[archive.offset:].strip(b'\0'), 'unparsed nonzero tar tail')
        a['require'](seen == set(expected) | directories, 'tar membership mismatch')
        manifests[path] = manifest
    legacy = {}
    for item in document['legacy_catalogs']:
        data = a['bounded_read'](a['local_path'](root, item['path']), 160000)
        a['require'](a['record_stats'](data) == (item['lines'], item['bytes'], item['sha256']), 'legacy catalog identity mismatch')
        retain(item['path'], data)
        decoded(data)
        legacy[item['path']] = data
    catalogs.update(legacy)
    for path in sorted(raw_paths):
        a['require'](path not in used, 'raw/packed duplicate logical identity')
        used.add(path)
        data = a['bounded_read'](a['local_path'](root, path), 160000)
        a['record_stats'](data)
        retain(path, data)
        decoded(data)
        records[path] = data
    a['require'](len(catalogs) + len(raw_paths) <= 64, 'group working Markdown bound exceeded')
    resident = sum(size for size, _checksum in owned.values())
    a['require'](resident <= GENERATION_BYTES, 'generation resident bound exceeded')
    sealed = {path: value for path, value in owned.items() if path not in raw_paths}
    a['require'](tree_identity(sealed) == row['tree_sha256'], 'generation tree identity mismatch')
    if legacy:
        a['_validate_legacy_evidence'](records, backings, legacy)
    return records, (len(catalogs) + len(raw_paths), decoded_lines, decoded_bytes, resident), backings, manifests, owned


def inventory(root, a):
    _document, index_data, groups = describe(root, a)
    actual = physical_files(root, a)
    raw = {path for path in actual if a['SEGMENT'].fullmatch(path)}
    records, backings, manifests, used, owned = {}, {}, {}, set(), {a['MASTER']}
    pressure = [0, 0, 0, len(index_data)]
    for index, group in enumerate(groups):
        values = decode_group(root, a, group, raw if index == len(groups) - 1 else set(), used)
        found, measured, originals, windows, files = values
        a['require'](not owned.intersection(files), 'physical file belongs to multiple generations')
        owned.update(files)
        records.update(found)
        backings.update(originals)
        manifests.update(windows)
        pressure = [left + right for left, right in zip(pressure, measured)]
    a['require'](owned == actual, 'orphan/missing generation backing')
    a['require'](pressure[0] <= 192 and pressure[1] <= MAX_GENERATIONS * GENERATION_LINES
                 and pressure[2] <= MAX_GENERATIONS * GENERATION_BYTES
                 and pressure[3] <= MAX_GENERATIONS * GENERATION_BYTES + INDEX_BYTES,
                 'generation index aggregate bound exceeded')
    a['require'](a['bounded_read'](a['local_path'](root, a['MASTER']), INDEX_BYTES) == index_data, 'generation index changed during read')
    return records, tuple(pressure), backings, manifests
