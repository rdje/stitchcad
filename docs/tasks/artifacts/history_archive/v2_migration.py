"""Prepare source-pinned archive transition/append/generation candidates; installation is separately guarded."""
import argparse
import datetime
import gzip
import hashlib
import io
import json
from pathlib import Path
import re
import runpy
import subprocess
import sys
import tarfile

ROOT = Path(__file__).resolve().parents[4]
READER = ROOT / 'scripts/history_archive.py'
PROFILE = ROOT / 'scripts/local_environment.py'
GENERATIONS = ROOT / 'scripts/history_generations.py'
INSTALLER = Path(__file__).with_name('v2_install.py')
MASTER = '.doctrine/history_archive/windows.json'
OWNER = 'G1-SLICE.RETENTION-CAPACITY.2'


def require(condition, message):
    if not condition:
        raise ValueError(message)


def digest(data):
    return hashlib.sha256(data).hexdigest()


def document(value):
    return (json.dumps(value, indent=2, ensure_ascii=True) + '\n').encode()


def identity(path, data, kind=None):
    result = {'path': path, 'lines': data.count(b'\n'), 'bytes': len(data), 'sha256': digest(data)}
    if kind is not None:
        result['kind'] = kind
    return result


def packet(items):
    output = io.BytesIO()
    with tarfile.open(fileobj=output, mode='w', format=tarfile.USTAR_FORMAT) as archive:
        for path, data, _kind in items:
            info = tarfile.TarInfo(path)
            info.size, info.mode, info.mtime = len(data), 0o444, 0
            archive.addfile(info, io.BytesIO(data))
    raw = output.getvalue()
    require(len(raw) <= 8 * 1024 * 1024, 'candidate tar bound exceeded')
    packed = gzip.compress(raw, mtime=0)
    require(len(packed) <= 1048576, 'candidate compressed bound exceeded')
    return packed


def candidate_files(records, backings, catalogs, revision, created):
    """Pure formatter; every source record and original backing remains a first-level member."""
    files = dict(catalogs)
    items = sorted([(path, data, 'record') for path, data in records.items()]
                   + [(path, data, 'backing') for path, data in backings.items()])
    windows = []
    original = json.loads(backings[MASTER])
    start = max(int(Path(name).stem.removeprefix('window')) for name in original['windows']) + 1
    for offset in range(0, len(items), 256):
        group = items[offset:offset + 256]
        number = start + offset // 256
        packed = packet(group)
        require(packed == packet(group), 'candidate formatter is not reproducible')
        payload = 'docs/history/payloads/' + digest(packed) + '.tar.gz'
        path = '.doctrine/history_archive/window%d.json' % number
        catalog = 'docs/history/window%d.md' % number
        manifest = {
            'version': 2, 'id': 'window%d' % number, 'source_revision': revision,
            'created': created, 'owner': OWNER,
            'reason': 'Flat immutable-byte migration retaining original v1 backing and logical identities within unchanged bounds.',
            'payload': payload, 'payload_sha256': digest(packed), 'catalog': catalog,
            'members': [identity(name, data, kind) for name, data, kind in group],
        }
        text = '# Retained history — window %d\n\nSource `%s`; owner `%s`.\n' % (number, revision, OWNER)
        text += 'Original logical records and backing bytes remain immutable; corrections use new records.\n\n'
        text += 'Use the published list/read/read-backing/materialize commands; ordinary retrieval is offline.\n\n'
        for name, data, kind in group:
            text += '### %s\n\n%s; %d full-file lines, %d bytes; `sha256:%s`.\n\n' % (
                Path(name).name, kind, data.count(b'\n'), len(data), digest(data))
        files.update({payload: packed, path: document(manifest), catalog: text.encode()})
        windows.append(path)
    require(1 <= len(windows) <= 16, 'candidate window count bound exceeded')
    files[MASTER] = document({'version': 2, 'windows': windows,
                              'legacy_catalogs': [identity(path, data) for path, data in sorted(catalogs.items())]})
    return files


def prepare(work):
    profile = runpy.run_path(str(PROFILE))
    profile['plan_producer'](ROOT, sources=(Path(__file__), INSTALLER, READER, PROFILE), directories=(work,))
    reader = runpy.run_path(str(READER))
    sources = (Path(__file__), INSTALLER, *reader['backing_sources'](ROOT, profile))
    profile['plan_producer'](ROOT, sources=sources, directories=(work,))
    require(not work.exists(), 'fresh migration workspace required')
    revision = reader['_head_revision'](ROOT)
    master = json.loads((ROOT / MASTER).read_bytes())
    require(master['version'] == 1 and len(master['windows']) == 16, 'v1 full-window source required')
    records, pressure = reader['inventory'](ROOT)
    original = {path.relative_to(ROOT).as_posix(): path.read_bytes() for path in sources
                if path.is_relative_to(ROOT / 'docs/history') or path.is_relative_to(ROOT / '.doctrine/history_archive')}
    for path, data in original.items():
        bound = 160000 if path.endswith('.md') else reader['_backing_limit'](path)
        require(reader['_git_blob'](ROOT, revision, path, bound) == data, 'source drift: ' + path)
    backings = reader['backing_inventory'](ROOT)
    catalogs = {}
    for name in master['windows']:
        manifest = json.loads(original[name])
        catalogs[manifest['catalog']] = original[manifest['catalog']]
    created = subprocess.check_output(['git', '-C', str(ROOT), 'show', '-s', '--format=%cs', revision],
                                      env=reader['metadata_environment'](ROOT)).decode().strip()
    datetime.date.fromisoformat(created)
    files = candidate_files(records, backings, catalogs, revision, created)
    commands = [
        ['verify-retention'], ['prove-source', Path(next(iter(json.loads(files[MASTER])['windows']))).stem],
        ['prove-source', 'window16'],
    ]
    write_prepared(work, profile, reader, sources, revision, pressure, original, files, records,
                   backings, commands, len(catalogs))


def write_prepared(work, profile, reader, sources, revision, pressure, original, files, records,
                   backings, commands, legacy_catalogs, extra_report=None):
    """Share the original fully planned candidate writes and exact-byte proofs between modes."""
    candidate = work / 'candidate'
    source_file = work / 'source.json'
    report_file = work / 'prepared.json'
    outputs = [candidate / name for name in files] + [source_file, report_file]
    directories = {work, candidate}
    for path in outputs:
        directories.update(parent for parent in path.parents if parent.is_relative_to(work))
    plan = dict(sources=sources, directories=tuple(sorted(directories)), output_files=tuple(outputs))
    profile['enter_producer'](ROOT, **plan)
    for name, data in files.items():
        profile['plan_producer'](ROOT, **plan)
        with (candidate / name).open('xb') as stream:
            stream.write(data)
    audit = {
        'schema_version': 2,
        'source_revision': revision,
        'producer_sha256': digest(Path(__file__).read_bytes()), 'reader_sha256': digest(READER.read_bytes()),
        'installer_sha256': digest(INSTALLER.read_bytes()),
        'generation_reader_sha256': digest(GENERATIONS.read_bytes()),
        'profile_sha256': digest(PROFILE.read_bytes()), 'source_pressure': pressure,
        'source_files': [identity(path, data) for path, data in sorted(original.items())],
        'candidate_files': [identity(path, data) for path, data in sorted(files.items())],
    }
    profile['plan_producer'](ROOT, **plan)
    with source_file.open('xb') as stream:
        stream.write(document(audit))
    require(reader['inventory'](candidate)[0] == records, 'candidate logical bytes differ')
    require(reader['backing_inventory'](candidate) == backings, 'candidate original backing bytes differ')
    for operation in commands:
        profile['plan_producer'](ROOT, sources=(*sources, *(candidate / name for name in files)))
        subprocess.run([sys.executable, '-I', '-B', str(READER), '--root', str(candidate), *operation],
                       cwd=ROOT, check=True)
    require(reader['_head_revision'](ROOT) == revision, 'source HEAD changed during preparation')
    require(all(((ROOT / path).read_bytes() == data for path, data in original.items())), 'source archive changed during preparation')
    report = {'source_revision': revision, 'candidate_pressure': reader['inventory'](candidate)[1],
              'logical_records': len(records), 'original_backings': len(backings),
              'legacy_catalogs': legacy_catalogs, 'live_installed': False}
    report.update(extra_report or {})
    profile['plan_producer'](ROOT, **plan)
    with report_file.open('xb') as stream:
        stream.write(document(report))
    print(json.dumps(report, sort_keys=True))


def append_files(original, master, names, revision, created, owner):
    """Pure v2 formatter: pack the complete committed raw snapshot; retain old windows byte-exact."""
    require(master['version'] == 2 and len(master['windows']) < 16, 'v2 append needs an active window slot')
    require(2 <= len(names) <= 256, 'append needs two or more bounded committed raw records')
    files = dict(original)
    items = [(name, files.pop(name), 'record') for name in sorted(names)]
    packed = packet(items)
    require(packed == packet(items), 'append formatter is not reproducible')
    number = max(int(Path(name).stem.removeprefix('window')) for name in master['windows']) + 1
    window = 'window%d' % number
    manifest_path = '.doctrine/history_archive/' + window + '.json'
    catalog = 'docs/history/' + window + '.md'
    payload = 'docs/history/payloads/' + digest(packed) + '.tar.gz'
    require(not {manifest_path, catalog, payload} & set(original), 'append destination already exists')
    manifest = {
        'version': 2, 'id': window, 'source_revision': revision,
        'created': created, 'owner': owner,
        'reason': 'Committed raw-record append; old windows and all independent bounds remain unchanged.',
        'payload': payload, 'payload_sha256': digest(packed), 'catalog': catalog,
        'members': [identity(name, data, kind) for name, data, kind in items],
    }
    text = '# Retained history — window %d\n\nSource `%s`; owner `%s`.\n\n' % (number, revision, owner)
    text += 'Full-file identities are in the manifest; original bytes and logical addresses stay immutable.\n'
    text += 'Use the published list/read/materialize commands; ordinary retrieval is offline.\n\n'
    for name, _data, _kind in items:
        text += '### %s\n\n' % Path(name).name
    updated = dict(master, windows=[*master['windows'], manifest_path])
    files.update({MASTER: document(updated), manifest_path: document(manifest),
                  catalog: text.encode(), payload: packed})
    require(all((files[name] == data for name, data in original.items() if name != MASTER and name not in names)), 'append changed original physical bytes')
    return files, window


def append(work, owner):
    """Prepare current committed v2 raw records for the existing journaled installer, without live writes."""
    require(isinstance(owner, str) and re.fullmatch('[A-Za-z0-9][A-Za-z0-9_.-]{0,79}', owner), 'explicit task owner required')
    profile = runpy.run_path(str(PROFILE))
    profile['plan_producer'](ROOT, sources=(Path(__file__), INSTALLER, READER, PROFILE), directories=(work,))
    reader = runpy.run_path(str(READER))
    sources = (Path(__file__), INSTALLER, *reader['backing_sources'](ROOT, profile))
    profile['plan_producer'](ROOT, sources=sources, directories=(work,))
    require(not work.exists(), 'fresh append workspace required')
    revision = reader['_head_revision'](ROOT)
    master = json.loads((ROOT / MASTER).read_bytes())
    require(master['version'] == 2, 'append requires a v2 source')
    records, pressure = reader['inventory'](ROOT)
    original = {path.relative_to(ROOT).as_posix(): path.read_bytes() for path in sources
                if path.is_relative_to(ROOT / 'docs/history') or path.is_relative_to(ROOT / '.doctrine/history_archive')}
    for path, data in original.items():
        bound = 160000 if path.endswith('.md') else reader['_backing_limit'](path)
        require(reader['_git_blob'](ROOT, revision, path, bound) == data, 'source drift: ' + path)
    names = reader['_git_paths'](ROOT, revision)
    created = subprocess.check_output(['git', '-C', str(ROOT), 'show', '-s', '--format=%cs', revision],
                                      env=reader['metadata_environment'](ROOT)).decode().strip()
    datetime.date.fromisoformat(created)
    files, window = append_files(original, master, names, revision, created, owner)
    backings = reader['backing_inventory'](ROOT)
    write_prepared(work, profile, reader, sources, revision, pressure, original, files, records,
                   backings, [['verify-retention'], ['prove-source', window]], len(master['legacy_catalogs']),
                   {'new_window': window, 'packed_records': len(names)})


def generations(work, owner):
    """Seal exact v2 metadata and assign unsealed raw records to a fresh bounded active group."""
    require(isinstance(owner, str) and re.fullmatch('[A-Za-z0-9][A-Za-z0-9_.-]{0,79}', owner), 'explicit task owner required')
    profile = runpy.run_path(str(PROFILE))
    profile['plan_producer'](ROOT, sources=(Path(__file__), INSTALLER, READER, GENERATIONS, PROFILE), directories=(work,))
    reader = runpy.run_path(str(READER))
    sources = (Path(__file__), INSTALLER, *reader['backing_sources'](ROOT, profile))
    profile['plan_producer'](ROOT, sources=sources, directories=(work,))
    require(not work.exists(), 'fresh generation workspace required')
    revision = reader['_head_revision'](ROOT)
    original = {path.relative_to(ROOT).as_posix(): path.read_bytes() for path in sources
                if path.is_relative_to(ROOT / 'docs/history') or path.is_relative_to(ROOT / '.doctrine/history_archive')}
    require(json.loads(original[MASTER])['version'] == 2, 'generation transition requires v2 source')
    records, pressure = reader['inventory'](ROOT)
    for path, data in original.items():
        bound = 160000 if path.endswith('.md') else reader['_backing_limit'](path)
        require(reader['_git_blob'](ROOT, revision, path, bound) == data, 'source drift: ' + path)
    files = dict(original)
    first, active = ('.doctrine/history_archive/generation%d.json' % n for n in (1, 2))
    require(first not in files and active not in files, 'generation controls already exist')
    files[first] = original[MASTER]
    files[active] = document({'version': 2, 'windows': [], 'legacy_catalogs': []})
    engine = reader['generation_reader']()
    descriptors = []
    for path in (first, active):
        group = json.loads(files[path])
        names = {path, *(row['path'] for row in group['legacy_catalogs'])}
        for name in group['windows']:
            manifest = json.loads(files[name])
            names.update((name, manifest['payload'], manifest['catalog']))
        row = identity(path, files[path])
        row['tree_sha256'] = engine['tree_identity']({name: (len(files[name]), digest(files[name])) for name in names})
        descriptors.append(row)
    files[MASTER] = document({'version': 3, 'generations': descriptors})
    write_prepared(work, profile, reader, sources, revision, pressure, original, files, records,
                   reader['backing_inventory'](ROOT), [['verify-retention'], ['prove-source', 'window17'],
                                                       ['prove-source', 'window21']], 16,
                   {'owner': owner, 'schema_version': 3, 'generations': 2})


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('operation', choices=['prepare', 'append', 'generations'])
    parser.add_argument('workspace', help='fresh repository-relative target/ directory')
    parser.add_argument('--owner', help='explicit owning task leaf for append/generation maintenance')
    args = parser.parse_args()
    require(args.workspace.startswith('target/') and '..' not in Path(args.workspace).parts, 'workspace must be repository-relative target/')
    if args.operation == 'generations':
        generations(ROOT / args.workspace, args.owner)
    elif args.operation == 'append':
        append(ROOT / args.workspace, args.owner)
    else:
        require(args.owner is None, 'owner applies only to append or generations')
        prepare(ROOT / args.workspace)


if __name__ == '__main__':
    main()
