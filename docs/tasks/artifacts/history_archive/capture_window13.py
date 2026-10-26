"""Prepare/prove window13 in a fresh project-local fixture; never install or retire live records."""
from pathlib import Path
import gzip
import hashlib
import json
import os
import runpy
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
SOURCE = '87d0987a71d38c587a1da68e1810620d3e5e9b72'
FIXTURE = ROOT / 'target/history_window13_capture'
READER = ROOT / 'scripts/history_archive.py'
PROFILE = ROOT / 'scripts/local_environment.py'
MASTER = '.doctrine/history_archive/windows.json'
OWNER = 'G1-SLICE.5e.2.h13'


def git(*args):
    # Same OS config suppression as the public reader; no foreign Git context or optional locks.
    environment = {name: value for name, value in os.environ.items() if not name.startswith('GIT_')}
    environment.update(GIT_CONFIG_NOSYSTEM='1', GIT_CONFIG_GLOBAL=os.devnull,
                       GIT_OPTIONAL_LOCKS='0', GIT_LITERAL_PATHSPECS='1', GIT_TERMINAL_PROMPT='0')
    return subprocess.check_output(['git', *args], cwd=ROOT, env=environment)


def reader(operation, argument=None):
    command = [sys.executable, '-I', '-B', str(READER), '--root', str(FIXTURE), operation]
    if argument is not None:
        command.append(argument)
    return subprocess.check_output(command, cwd=ROOT)


assert Path(git('rev-parse', '--show-toplevel').decode().strip()).resolve() == ROOT
assert git('rev-parse', 'HEAD').decode().strip() == SOURCE, 'requires stable source HEAD'
inputs = git('ls-tree', '-r', '--name-only', SOURCE,
             '.doctrine/history_archive', 'docs/history').decode().splitlines()
profile = runpy.run_path(str(PROFILE))
sources = (Path(__file__), READER, PROFILE, *(ROOT / path for path in inputs))
profile['plan_producer'](ROOT, sources=sources, directories=(FIXTURE,))
assert not FIXTURE.exists(), 'requires a fresh fixture identity'
environment = profile['environment_for'](ROOT, os.environ, create=False)
assert all(not Path(environment[name]).is_relative_to(FIXTURE) for name in profile['STORES']), \
    'fixture cannot contain a prepared store'

master = json.loads((ROOT / MASTER).read_text())
assert master == {'version': 1, 'windows': [
    '.doctrine/history_archive/window%d.json' % i for i in range(1, 13)]}
assert (ROOT / MASTER).read_bytes() == git('show', SOURCE + ':' + MASTER)
previous = [json.loads((ROOT / path).read_text()) for path in master['windows']]
catalogs = {manifest['catalog'] for manifest in previous}
paths = sorted(path for path in inputs if path.startswith('docs/history/') and path.endswith('.md')
               and path not in catalogs)
assert len(paths) == 52 and len(catalogs) == 12, 'capture membership changed'
assert len(list((ROOT / 'docs/history').glob('*.md'))) == 64, 'working population changed'
original = {path: git('show', SOURCE + ':' + path) for path in paths}
assert all((ROOT / path).read_bytes() == data for path, data in original.items()), 'raw source drift'
tar = git('archive', '--format=tar', SOURCE, '--', *paths)
assert tar == git('archive', '--format=tar', SOURCE, '--', *paths)
packed = gzip.compress(tar, mtime=0)
assert packed == gzip.compress(tar, mtime=0)
sha = hashlib.sha256(packed).hexdigest()
payload = 'docs/history/payloads/' + sha + '.tar.gz'
manifest = {
    'version': 1, 'id': 'window13', 'source_revision': SOURCE, 'created': '2026-10-26',
    'owner': OWNER,
    'reason': 'Consolidate all52 immutable raw records at the fixed64-file working limit before more expression-execution history.',
    'payload': payload, 'payload_sha256': sha, 'catalog': 'docs/history/window13.md',
    'members': [{'path': path, 'lines': data.count(b'\n'), 'bytes': len(data),
                 'sha256': hashlib.sha256(data).hexdigest()} for path, data in original.items()],
}
catalog = '''# Retained history — window 13

Captured from `%s` on2026-10-26; owner `%s`.
Full-file identities: `.doctrine/history_archive/window13.json`.
Payload: `sha256:%s`. This window is immutable; corrections belong in later records.

Use `bash scripts/history_archive.sh list` and
`bash scripts/history_archive.sh read docs/history/<basename>` without historical Git or network.
Materialize into a fresh repository-local directory:
`bash scripts/history_archive.sh materialize target/history-review`.
Optional source comparison: `bash scripts/history_archive.sh prove-source window13`.
The book governance guide distinguishes identity, semantic truth and storage pressure.

''' % (SOURCE, OWNER, sha)
for row in manifest['members']:
    catalog += '### %s\n\n%d full-file lines, %d bytes; `sha256:%s`.\n\n' % (
        Path(row['path']).name, row['lines'], row['bytes'], row['sha256'])

# Copy only declared committed controls/backing files, never an ambient directory population.
retained = [*master['windows'], *(path for old in previous for path in (old['payload'], old['catalog']))]
files = {}
for path in retained:
    data = git('show', SOURCE + ':' + path)
    assert (ROOT / path).read_bytes() == data, 'prior backing drift: ' + path
    files[path] = data
master['windows'].append('.doctrine/history_archive/window13.json')
files.update({MASTER: (json.dumps(master, indent=2) + '\n').encode(),
              '.doctrine/history_archive/window13.json': (json.dumps(manifest, indent=2) + '\n').encode(),
              payload: packed, manifest['catalog']: (catalog.rstrip('\n') + '\n').encode()})
outputs = tuple(FIXTURE / path for path in files)
directories = {FIXTURE}
for output in outputs:
    directories.update(parent for parent in output.parents if parent.is_relative_to(FIXTURE))
profile['enter_producer'](ROOT, sources=sources, output_files=outputs,
                          directories=tuple(sorted(directories)))
for path, data in files.items():
    output = FIXTURE / path
    profile['plan_producer'](ROOT, output_files=(output,))
    with output.open('xb') as stream:
        stream.write(data)

assert not (FIXTURE / '.git').exists(), 'fixture must never be another Git repository'
assert all(not (FIXTURE / path).exists() for path in paths), 'raw capture copy remains'
print(reader('verify').decode().strip(), flush=True)
print(reader('prove-source', 'window13').decode().strip(), flush=True)
print(reader('materialize', 'target/recovered').decode().strip(), flush=True)
for path, data in original.items():
    assert (FIXTURE / 'target/recovered' / path).read_bytes() == data
    assert reader('read', path) == data
for path in retained:
    assert (FIXTURE / path).read_bytes() == git('show', SOURCE + ':' + path)
assert not (FIXTURE / '.git').exists()
print('window13:52 committed files; source/materialization/public read exact;12 prior windows unchanged')
print('capture bytes:%d; payload bytes:%d; sha256:%s' % (
    sum(len(data) for data in original.values()), len(packed), sha))
