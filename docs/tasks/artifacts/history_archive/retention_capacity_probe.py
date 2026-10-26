"""Historical ad3f8fc v2 capacity prototype; no installation or live retirement.

Requires the fixed source HEAD and a fresh repository-local fixture. Native OS tar is a
read-only oracle. All original bytes, including opaque v1 backing evidence, count toward
finite decoded/resident budgets. This producer does not adopt or implement the v2 reader.
"""
import argparse
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
SOURCE = 'ad3f8fc294b9ec5a6314db2649a1d0e9c1cb2ca3'
READER = ROOT / 'scripts/history_archive.py'
PROFILE = ROOT / 'scripts/local_environment.py'
MASTER = '.doctrine/history_archive/windows.json'
OWNER = 'G1-SLICE.RETENTION-CAPACITY.1'


def sha(data):
    return hashlib.sha256(data).hexdigest()


def encoded(value):
    return (json.dumps(value, indent=2, ensure_ascii=True) + '\n').encode()


def row(path, data, kind=None):
    result = {'path': path, 'lines': data.count(b'\n'), 'bytes': len(data), 'sha256': sha(data)}
    if kind is not None:
        result['kind'] = kind
    return result


def packet(items):
    stream = io.BytesIO()
    with tarfile.open(fileobj=stream, mode='w', format=tarfile.USTAR_FORMAT) as archive:
        for path, data, _kind in items:
            info = tarfile.TarInfo(path)
            info.size = len(data)
            info.mode = 0o444
            info.mtime = 0
            archive.addfile(info, io.BytesIO(data))
    raw = stream.getvalue()
    return raw, gzip.compress(raw, mtime=0)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('fixture', help='fresh repository-relative target/ directory')
    args = parser.parse_args()
    assert args.fixture.startswith('target/') and '..' not in Path(args.fixture).parts
    fixture = ROOT / args.fixture
    profile = runpy.run_path(str(PROFILE))
    profile['plan_producer'](ROOT, sources=(Path(__file__), READER, PROFILE), directories=(fixture,))
    reader = runpy.run_path(str(READER))
    sources = (Path(__file__), *reader['backing_sources'](ROOT, profile))
    profile['plan_producer'](ROOT, sources=sources, directories=(fixture,))
    assert not fixture.exists(), 'fresh fixture required'
    environment = reader['metadata_environment'](ROOT)
    head = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, env=environment).decode().strip()
    assert head == SOURCE, 'historical probe requires fixed source HEAD; never rerun at current HEAD'
    master = json.loads((ROOT / MASTER).read_bytes())
    assert master['version'] == 1 and len(master['windows']) == 16
    records, original_pressure = reader['inventory'](ROOT)
    backing_files = {path.relative_to(ROOT).as_posix(): path.read_bytes()
                     for path in sources if path.is_relative_to(ROOT / 'docs/history')
                     or path.is_relative_to(ROOT / '.doctrine/history_archive')}
    for path, data in backing_files.items():
        assert subprocess.check_output(['git', 'show', SOURCE + ':' + path], cwd=ROOT, env=environment) == data
    catalogs = {}
    blobs = {MASTER: backing_files[MASTER]}
    for path in master['windows']:
        manifest = json.loads(backing_files[path])
        for backing in [path, manifest['payload']]:
            blobs[backing] = backing_files[backing]
        catalogs[manifest['catalog']] = backing_files[manifest['catalog']]
    assert len(blobs) == 33 and len(catalogs) == 16
    items = sorted([(path, data, 'record') for path, data in records.items()]
                   + [(path, data, 'backing') for path, data in blobs.items()])
    groups = [items[index:index + 256] for index in range(0, len(items), 256)]
    files = dict(catalogs)
    manifests = []
    tar_sizes = []
    payload_sizes = []
    for index, group in enumerate(groups, 17):
        raw, packed = packet(group)
        assert (raw, packed) == packet(group), 'producer reproducibility failed'
        tar_sizes.append(len(raw))
        payload_sizes.append(len(packed))
        payload = 'docs/history/payloads/' + sha(packed) + '.tar.gz'
        catalog = 'docs/history/window%d.md' % index
        manifest_path = '.doctrine/history_archive/window%d.json' % index
        manifest = {
            'version': 2, 'id': 'window%d' % index, 'source_revision': SOURCE,
            'created': '2026-10-26', 'owner': OWNER,
            'reason': 'Unadopted capacity prototype: exact logical records and opaque original v1 backing bytes.',
            'payload': payload, 'payload_sha256': sha(packed), 'catalog': catalog,
            'members': [row(path, data, kind) for path, data, kind in group],
        }
        catalog_data = ('# Retained history — window %d\n\nUnadopted capacity prototype from `%s`.\n\n'
                        % (index, SOURCE))
        for path, data, kind in group:
            catalog_data += '### %s\n\n%s; %d bytes; `sha256:%s`.\n\n' % (Path(path).name, kind, len(data), sha(data))
        files.update({payload: packed, catalog: catalog_data.encode(), manifest_path: encoded(manifest)})
        manifests.append((manifest_path, manifest))
    candidate = {'version': 2, 'windows': [path for path, _ in manifests],
                 'legacy_catalogs': [row(path, data) for path, data in sorted(catalogs.items())]}
    files[MASTER] = encoded(candidate)
    report_path = fixture / 'report.json'
    outputs = [fixture / path for path in files] + [report_path]
    directories = {fixture}
    for path in outputs:
        directories.update(parent for parent in path.parents if parent.is_relative_to(fixture))
    profile['enter_producer'](ROOT, sources=sources, output_files=tuple(outputs), directories=tuple(sorted(directories)))
    for path, data in files.items():
        output = fixture / path
        profile['plan_producer'](ROOT, output_files=(output,))
        with output.open('xb') as destination:
            destination.write(data)
    native_members = 0
    for (_path, manifest), group in zip(manifests, groups):
        archive = fixture / manifest['payload']
        listing = subprocess.check_output(['tar', '-tf', str(archive)], cwd=ROOT)
        assert listing.decode().splitlines() == [path for path, _data, _kind in group]
        combined = subprocess.check_output(['tar', '-xOf', str(archive)], cwd=ROOT)
        assert combined == b''.join(data for _path, data, _kind in group), 'native tar content differs'
        offset = 0
        for path, data, _kind in group:
            recovered = combined[offset:offset + len(data)]
            assert recovered == data and sha(recovered) == sha(data), path
            offset += len(data)
            native_members += 1
        assert offset == len(combined)
    controls = {path: data for path, data in files.items() if path.startswith('.doctrine/')}
    new_catalogs = [files[manifest['catalog']] for _path, manifest in manifests]
    decoded = [*records.values(), *blobs.values(), *catalogs.values(), *new_catalogs]
    metrics = {
        'logical_records': len(records), 'opaque_v1_backings': len(blobs), 'legacy_catalogs': len(catalogs),
        'active_windows': len(manifests), 'members_per_window': [len(group) for group in groups],
        'working_markdown': len(catalogs) + len(manifests),
        'decoded_bytes_including_evidence': sum(map(len, decoded)),
        'decoded_lines_including_binary_newlines': sum(data.count(b'\n') for data in decoded),
        'resident_bytes': sum(map(len, files.values())),
        'control_files': len(controls), 'control_bytes': sum(map(len, controls.values())),
        'control_lines': sum(data.count(b'\n') for data in controls.values()),
        'control_maxline': max(len(line) for data in controls.values() for line in data.splitlines()),
        'master_bytes': len(files[MASTER]),
        'manifest_bytes': [len(files[path]) for path, _manifest in manifests],
        'payload_bytes': payload_sizes, 'tar_bytes': tar_sizes,
        'catalog_max_bytes': max(map(len, [*catalogs.values(), *new_catalogs])),
        'catalog_max_lines': max(data.count(b'\n') for data in [*catalogs.values(), *new_catalogs]),
        'catalog_maxline': max(len(line) for data in [*catalogs.values(), *new_catalogs] for line in data.splitlines()),
    }
    limits = {
        'active_windows': (metrics['active_windows'], 16), 'members': (max(metrics['members_per_window']), 256),
        'working_markdown': (metrics['working_markdown'], 64),
        'decoded_bytes': (metrics['decoded_bytes_including_evidence'], 4_000_000),
        'decoded_lines': (metrics['decoded_lines_including_binary_newlines'], 60_000),
        'resident_bytes': (metrics['resident_bytes'], 4_000_000),
        'control_files': (metrics['control_files'], 17), 'control_bytes': (metrics['control_bytes'], 262144),
        'control_lines': (metrics['control_lines'], 20000), 'control_maxline': (metrics['control_maxline'], 400),
        'master_bytes': (metrics['master_bytes'], 8192), 'manifest_bytes': (max(metrics['manifest_bytes']), 65536),
        'payload_bytes': (max(payload_sizes), 1048576), 'tar_bytes': (max(tar_sizes), 8 * 1024 * 1024),
        'catalog_bytes': (metrics['catalog_max_bytes'], 160000), 'catalog_lines': (metrics['catalog_max_lines'], 2000),
        'catalog_maxline': (metrics['catalog_maxline'], 400),
    }
    failures = {name: [actual, bound] for name, (actual, bound) in limits.items() if actual > bound}
    refused = subprocess.run([sys.executable, '-I', '-B', str(READER), '--root', str(fixture), 'verify'],
                             cwd=ROOT, capture_output=True)
    assert refused.returncode == 1 and not refused.stdout and b'unknown/missing schema fields' in refused.stderr
    assert not (fixture / '.git').exists()
    for path, data in backing_files.items():
        assert (ROOT / path).read_bytes() == data, 'source changed during prototype: ' + path
    report = {
        'source_revision': SOURCE, 'producer_sha256': sha(Path(__file__).read_bytes()),
        'original_pressure': original_pressure, 'metrics': metrics, 'limits': limits, 'failures': failures,
        'native_tar_members_exact': native_members, 'native_v1_refusal_rc': refused.returncode,
        'all_source_bytes_unchanged': True, 'live_protocol_adopted': False, 'migration_implemented': False,
    }
    profile['plan_producer'](ROOT, output_files=(report_path,))
    with report_path.open('xb') as destination:
        destination.write(encoded(report))
    print(json.dumps(report, sort_keys=True))
    assert not failures, 'candidate exceeds unchanged independent limits'


if __name__ == '__main__':
    main()
