"""Prepare/prove window5 in a local fixture; never install it or remove source records."""
from pathlib import Path
import gzip
import hashlib
import json
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
SOURCE = 'c91cdf5ecc4c3246588f625a0bb84f1809483caf'
FIXTURE = ROOT / 'target/history_window5_capture'
READER = ROOT / 'scripts/history_archive.py'
MASTER = '.doctrine/history_archive/windows.json'
OWNER = 'G1-SLICE.5b.4c.h0'


def git(*args):
    return subprocess.check_output(['git', *args], cwd=ROOT)


def reader(operation, argument=None):
    args = [sys.executable, '-I', '-B', str(READER), '--root', str(FIXTURE), operation]
    if argument is not None:
        args.append(argument)
    result = subprocess.run(args, cwd=ROOT, capture_output=True, check=True)
    return result.stdout


assert git('rev-parse', 'HEAD').decode().strip() == SOURCE, 'capture requires its stable source HEAD'
# Reuse the published path guard for fixture device/links/Git boundaries; no deletion is needed.
import runpy
path_guard = runpy.run_path(str(READER))['local_path']
assert path_guard(ROOT, FIXTURE.relative_to(ROOT).as_posix()) == FIXTURE
assert not FIXTURE.exists(), 'capture requires a fresh fixture identity'
FIXTURE.mkdir(parents=True)
master = json.loads((ROOT / MASTER).read_text())
assert master == {'version': 1, 'windows': ['.doctrine/history_archive/window1.json',
                                        '.doctrine/history_archive/window2.json',
                                        '.doctrine/history_archive/window3.json',
                                        '.doctrine/history_archive/window4.json']}
# Only raw sealed records are captured; the existing retained catalog/payload remain immutable.
paths = git('ls-tree', '-r', '--name-only', SOURCE, 'docs/history').decode().splitlines()
paths = sorted(p for p in paths if p.endswith('.md') and Path(p).name not in ('window1.md', 'window2.md', 'window3.md', 'window4.md'))
assert len(paths) == 60, ('capture membership', paths)
original = {path: git('show', SOURCE + ':' + path) for path in paths}
assert all((ROOT / path).read_bytes() == data for path, data in original.items()), 'raw source drift'
tar = git('archive', '--format=tar', SOURCE, '--', *paths)
assert tar == git('archive', '--format=tar', SOURCE, '--', *paths), 'Git archive is not deterministic'
packed = gzip.compress(tar, mtime=0)
assert packed == gzip.compress(tar, mtime=0), 'gzip encoding is not deterministic'
sha = hashlib.sha256(packed).hexdigest()
payload_path = 'docs/history/payloads/' + sha + '.tar.gz'
manifest = {
    'version': 1, 'id': 'window5', 'source_revision': SOURCE, 'created': '2026-10-04',
    'owner': OWNER,
    'reason': 'Consolidate 60 immutable raw records before D154 repair records exceed the unchanged 64-file working limit.',
    'payload': payload_path, 'payload_sha256': sha, 'catalog': 'docs/history/window5.md',
    'members': [
        {'path': path, 'lines': data.count(b'\n'), 'bytes': len(data),
         'sha256': hashlib.sha256(data).hexdigest()}
        for path, data in original.items()
    ],
}
# Install only into the isolated fixture. No raw copies or Git repository are present there.
shutil.copytree(ROOT / '.doctrine/history_archive', FIXTURE / '.doctrine/history_archive')
previous = [json.loads((ROOT / path).read_text()) for path in master['windows']]
for manifest_row in previous:
    for path in [manifest_row['payload'], manifest_row['catalog']]:
        destination = FIXTURE / path
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(ROOT / path, destination)
(FIXTURE / payload_path).write_bytes(packed)
(FIXTURE / '.doctrine/history_archive/window5.json').write_text(json.dumps(manifest, indent=2) + '\n')
master['windows'].append('.doctrine/history_archive/window5.json')
(FIXTURE / MASTER).write_text(json.dumps(master, indent=2) + '\n')
catalog = '''# Retained history — window 5

Captured from `%s` on 2026-10-04; owner `%s`.
Full-file identities live in `.doctrine/history_archive/window5.json`; payload `sha256:%s`.
This window is immutable; corrections belong in superseding records.

Discover with `bash scripts/history_archive.sh list`; retrieve original descriptors/coverage/bytes
with `bash scripts/history_archive.sh read docs/history/<basename>`. Ordinary retrieval needs no
historical Git objects or network. Materialize into a fresh local directory with
`bash scripts/history_archive.sh materialize target/history-review`.
Independent capture proof: `bash scripts/history_archive.sh prove-source window5` when its source
Git object is available. See the book upkeep guide for the separate identity/semantic/pressure scopes.

''' % (SOURCE, OWNER, sha)
for row in manifest['members']:
    catalog += '### %s\n\n%d full-file lines, %d bytes; `sha256:%s`.\n\n' % (
        Path(row['path']).name, row['lines'], row['bytes'], row['sha256'])
(FIXTURE / manifest['catalog']).write_text(catalog.rstrip('\n') + '\n')
print(reader('verify').decode().strip())
print(reader('prove-source', 'window5').decode().strip())
print(reader('materialize', 'target/recovered').decode().strip())
recovered = FIXTURE / 'target/recovered'
assert not (FIXTURE / '.git').exists()
assert all(not (FIXTURE / path).exists() for path in paths), 'fixture retains raw capture copies'
for path, data in original.items():
    assert (recovered / path).read_bytes() == data, ('independent source reconstruction', path)
    assert reader('read', path) == data, ('published read source mismatch', path)
# All previous windows reconstruct exactly; original source snapshots are independent evidence.
for manifest_row in previous:
    for row in manifest_row['members']:
        data = git('show', manifest_row['source_revision'] + ':' + row['path'])
        assert (recovered / row['path']).read_bytes() == data, ('previous window changed', row['path'])
        assert reader('read', row['path']) == data, ('previous read changed', row['path'])
assert len(list(recovered.rglob('*.md'))) == 309
print('window5 capture: 60 full files / %d lines / %d bytes / %d compressed bytes / %d tar bytes' % (
    sum(row['lines'] for row in manifest['members']), sum(row['bytes'] for row in manifest['members']),
    len(packed), len(tar)))
print('capture/source/materialization/read: 309 exact logical files; 0 missing/extra; no raw copies in fixture')
print('payload sha256:' + sha)
