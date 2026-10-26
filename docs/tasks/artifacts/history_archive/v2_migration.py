"""Prepare a source-pinned flat-v2 candidate; live installation is a separate guarded phase."""
import argparse
import datetime
import gzip
import hashlib
import io
import json
from pathlib import Path
import runpy
import subprocess
import sys
import tarfile

ROOT = Path(__file__).resolve().parents[4]
READER = ROOT / 'scripts/history_archive.py'
PROFILE = ROOT / 'scripts/local_environment.py'
MASTER = '.doctrine/history_archive/windows.json'
OWNER = 'G1-SLICE.RETENTION-CAPACITY.2'


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
    assert len(raw) <= 8 * 1024 * 1024, 'candidate tar bound exceeded'
    packed = gzip.compress(raw, mtime=0)
    assert len(packed) <= 1048576, 'candidate compressed bound exceeded'
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
        assert packed == packet(group), 'candidate formatter is not reproducible'
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
    assert 1 <= len(windows) <= 16
    files[MASTER] = document({'version': 2, 'windows': windows,
                              'legacy_catalogs': [identity(path, data) for path, data in sorted(catalogs.items())]})
    return files


def prepare(work):
    profile = runpy.run_path(str(PROFILE))
    profile['plan_producer'](ROOT, sources=(Path(__file__), READER, PROFILE), directories=(work,))
    reader = runpy.run_path(str(READER))
    sources = (Path(__file__), *reader['backing_sources'](ROOT, profile))
    profile['plan_producer'](ROOT, sources=sources, directories=(work,))
    assert not work.exists(), 'fresh migration workspace required'
    revision = reader['_head_revision'](ROOT)
    master = json.loads((ROOT / MASTER).read_bytes())
    assert master['version'] == 1 and len(master['windows']) == 16
    records, pressure = reader['inventory'](ROOT)
    original = {path.relative_to(ROOT).as_posix(): path.read_bytes() for path in sources
                if path.is_relative_to(ROOT / 'docs/history') or path.is_relative_to(ROOT / '.doctrine/history_archive')}
    for path, data in original.items():
        bound = 160000 if path.endswith('.md') else reader['_backing_limit'](path)
        assert reader['_git_blob'](ROOT, revision, path, bound) == data, 'source drift: ' + path
    backings = reader['backing_inventory'](ROOT)
    catalogs = {}
    for name in master['windows']:
        manifest = json.loads(original[name])
        catalogs[manifest['catalog']] = original[manifest['catalog']]
    created = subprocess.check_output(['git', '-C', str(ROOT), 'show', '-s', '--format=%cs', revision],
                                      env=reader['metadata_environment'](ROOT)).decode().strip()
    datetime.date.fromisoformat(created)
    files = candidate_files(records, backings, catalogs, revision, created)
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
        'source_revision': revision,
        'producer_sha256': digest(Path(__file__).read_bytes()), 'reader_sha256': digest(READER.read_bytes()),
        'profile_sha256': digest(PROFILE.read_bytes()), 'source_pressure': pressure,
        'source_files': [identity(path, data) for path, data in sorted(original.items())],
        'candidate_files': [identity(path, data) for path, data in sorted(files.items())],
    }
    profile['plan_producer'](ROOT, **plan)
    with source_file.open('xb') as stream:
        stream.write(document(audit))
    assert reader['inventory'](candidate)[0] == records
    assert reader['backing_inventory'](candidate) == backings
    commands = [
        ['verify-retention'], ['prove-source', Path(next(iter(json.loads(files[MASTER])['windows']))).stem],
        ['prove-source', 'window16'],
    ]
    for operation in commands:
        profile['plan_producer'](ROOT, sources=(*sources, *(candidate / name for name in files)))
        subprocess.run([sys.executable, '-I', '-B', str(READER), '--root', str(candidate), *operation],
                       cwd=ROOT, check=True)
    assert reader['_head_revision'](ROOT) == revision, 'source HEAD changed during preparation'
    assert all((ROOT / path).read_bytes() == data for path, data in original.items())
    report = {'source_revision': revision, 'candidate_pressure': reader['inventory'](candidate)[1],
              'logical_records': len(records), 'original_backings': len(backings),
              'legacy_catalogs': len(catalogs), 'live_installed': False}
    profile['plan_producer'](ROOT, **plan)
    with report_file.open('xb') as stream:
        stream.write(document(report))
    print(json.dumps(report, sort_keys=True))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('operation', choices=['prepare'])
    parser.add_argument('workspace', help='fresh repository-relative target/ directory')
    args = parser.parse_args()
    assert args.workspace.startswith('target/') and '..' not in Path(args.workspace).parts
    prepare(ROOT / args.workspace)


if __name__ == '__main__':
    main()
