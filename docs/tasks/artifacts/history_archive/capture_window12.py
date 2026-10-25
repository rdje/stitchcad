"""Prepare and prove window12 in a fresh local fixture; never delete or install live records."""
from pathlib import Path
import gzip
import hashlib
import json
import runpy
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
SOURCE = 'f168eec4e43cc794dc3e989706215a61f44cdec4'
FIXTURE = ROOT / 'target/history_window12_capture'
READER = ROOT / 'scripts/history_archive.py'
MASTER = '.doctrine/history_archive/windows.json'
OWNER = 'G1-SLICE.5d.1.2.2.h'


def git(*args):
    return subprocess.check_output(['git', *args], cwd=ROOT)


def reader(operation, argument=None):
    command = [sys.executable, '-I', '-B', str(READER), '--root', str(FIXTURE), operation]
    if argument is not None:
        command.append(argument)
    return subprocess.check_output(command, cwd=ROOT)


assert git('rev-parse', 'HEAD').decode().strip() == SOURCE, 'requires stable source HEAD'
inputs = git('ls-tree', '-r', '--name-only', SOURCE,
             '.doctrine/history_archive', 'docs/history').decode().splitlines()
profile = runpy.run_path(str(ROOT / 'scripts/local_environment.py'))
profile['directory'](ROOT, FIXTURE.relative_to(ROOT).as_posix(), create=False, missing_ok=True)
assert not FIXTURE.exists(), 'requires a fresh fixture identity'
profile['enter_producer'](ROOT, directories=(FIXTURE,),
                          sources=(Path(__file__), READER, *(ROOT / p for p in inputs)))
master = json.loads((ROOT / MASTER).read_text())
assert master == {'version': 1, 'windows': [
    '.doctrine/history_archive/window%d.json' % i for i in range(1, 12)]}
previous = [json.loads((ROOT / p).read_text()) for p in master['windows']]
catalogs = {m['catalog'] for m in previous}
paths = sorted(p for p in inputs if p.startswith('docs/history/') and p.endswith('.md')
               and p not in catalogs)
assert len(paths) == 53, 'capture membership changed'
original = {p: git('show', SOURCE + ':' + p) for p in paths}
assert all((ROOT / p).read_bytes() == data for p, data in original.items()), 'raw source drift'
tar = git('archive', '--format=tar', SOURCE, '--', *paths)
assert tar == git('archive', '--format=tar', SOURCE, '--', *paths)
packed = gzip.compress(tar, mtime=0)
assert packed == gzip.compress(tar, mtime=0)
sha = hashlib.sha256(packed).hexdigest()
payload = 'docs/history/payloads/' + sha + '.tar.gz'
manifest = {
    'version': 1, 'id': 'window12', 'source_revision': SOURCE, 'created': '2026-10-25',
    'owner': OWNER,
    'reason': 'Consolidate all53 immutable raw records at the fixed64-file working limit before the next numerical feature.',
    'payload': payload, 'payload_sha256': sha, 'catalog': 'docs/history/window12.md',
    'members': [{'path': p, 'lines': data.count(b'\n'), 'bytes': len(data),
                 'sha256': hashlib.sha256(data).hexdigest()} for p, data in original.items()],
}
shutil.copytree(ROOT / '.doctrine/history_archive', FIXTURE / '.doctrine/history_archive')
for old in previous:
    for p in (old['payload'], old['catalog']):
        destination = FIXTURE / p
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(ROOT / p, destination)
(FIXTURE / payload).write_bytes(packed)
(FIXTURE / '.doctrine/history_archive/window12.json').write_text(json.dumps(manifest, indent=2) + '\n')
master['windows'].append('.doctrine/history_archive/window12.json')
(FIXTURE / MASTER).write_text(json.dumps(master, indent=2) + '\n')
catalog = '''# Retained history — window 12

Captured from `%s` on2026-10-25; owner `%s`.
Full-file identities: `.doctrine/history_archive/window12.json`.
Payload: `sha256:%s`. This window is immutable; corrections belong in later records.

Use `bash scripts/history_archive.sh list` and
`bash scripts/history_archive.sh read docs/history/<basename>` without historical Git or network.
Materialize into a fresh repository-local directory:
`bash scripts/history_archive.sh materialize target/history-review`.
Optional source comparison: `bash scripts/history_archive.sh prove-source window12`.
The book governance guide distinguishes identity, semantic truth and storage pressure.

''' % (SOURCE, OWNER, sha)
for row in manifest['members']:
    catalog += '### %s\n\n%d full-file lines, %d bytes; `sha256:%s`.\n\n' % (
        Path(row['path']).name, row['lines'], row['bytes'], row['sha256'])
(FIXTURE / manifest['catalog']).write_text(catalog.rstrip('\n') + '\n')
print(reader('verify').decode().strip())
print(reader('prove-source', 'window12').decode().strip())
print(reader('materialize', 'target/recovered').decode().strip())
assert not (FIXTURE / '.git').exists()
assert all(not (FIXTURE / p).exists() for p in paths), 'raw capture copy remains'
for p, data in original.items():
    assert (FIXTURE / 'target/recovered' / p).read_bytes() == data
    assert reader('read', p) == data
for old in previous:
    for p in (old['payload'], old['catalog']):
        assert (FIXTURE / p).read_bytes() == git('show', SOURCE + ':' + p)
print('window12:53 committed files; source/materialization/public read exact;11 prior windows unchanged')
print('capture bytes:%d; payload bytes:%d; sha256:%s' % (
    sum(len(data) for data in original.values()), len(packed), sha))
