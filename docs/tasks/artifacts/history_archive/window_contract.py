"""Published archive CLI contracts for every window and the newest window's actual refusals."""
from pathlib import Path
import hashlib
import json
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
READER = ROOT / 'scripts/history_archive.py'
WORK = ROOT / 'target/scratch/history_window_contract'
MASTER = '.doctrine/history_archive/windows.json'
checks = 0


def command(root, operation, argument=None):
    args = [sys.executable, '-I', '-B', str(READER), '--root', str(root), operation]
    if argument is not None:
        args.append(argument)
    return subprocess.run(args, cwd=ROOT, capture_output=True)


def success(result):
    assert result.returncode == 0, (result.returncode, result.stdout, result.stderr)
    return result.stdout


def catalog_title(root, manifest):
    title = (root / manifest['catalog']).read_text().splitlines()[0]
    # Window3's historical typo is immutable; the live guide supersedes its label.
    number = manifest['id'].removeprefix('window')
    if manifest['id'] == 'window3':
        assert "Window3's retained catalog has a historical heading" in (
            ROOT / 'docs/book/src/governance.md').read_text(), 'legacy catalog correction missing'
        number = '2'
    assert title == '# Retained history — window ' + number, 'catalog title identity mismatch'


def fixture():
    root = WORK / 'fixture'
    if root.exists():
        assert not list(root.rglob('.git')), 'fixture crosses another Git repository'
        shutil.rmtree(root)
    shutil.copytree(ROOT / '.doctrine/history_archive', root / '.doctrine/history_archive')
    shutil.copytree(ROOT / 'docs/history', root / 'docs/history')
    return root


def refusal(label, mutate, expected):
    global checks
    root = fixture()
    mutate(root)
    result = command(root, 'verify')
    assert result.returncode == 1 and expected.encode() in result.stderr, (
        label, result.returncode, result.stdout, result.stderr)
    checks += 1
    print('  window refusal:', label, expected, 'rc=1')


if WORK.exists():
    assert not list(WORK.rglob('.git')), 'scratch crosses another Git repository'
    shutil.rmtree(WORK)
WORK.mkdir(parents=True)
try:
    manifest_paths = json.loads((ROOT / MASTER).read_text())['windows']
    manifests = [json.loads((ROOT / path).read_text()) for path in manifest_paths]
    for manifest in manifests:
        catalog_title(ROOT, manifest)
        checks += 1
    declared = {row['path']: row for manifest in manifests for row in manifest['members']}
    catalog_paths = {manifest['catalog'] for manifest in manifests}
    raw = {path.relative_to(ROOT).as_posix(): path.read_bytes()
           for path in (ROOT / 'docs/history').glob('*.md')
           if path.relative_to(ROOT).as_posix() not in catalog_paths}
    expected_paths = set(declared) | set(raw)
    listed = success(command(ROOT, 'list')).decode().splitlines()
    assert listed == sorted(expected_paths), ('logical list drift', listed, expected_paths)
    checks += 1
    # Every published read agrees with independently measured manifest identity or raw source bytes.
    for path in listed:
        data = success(command(ROOT, 'read', path))
        if path in declared:
            row = declared[path]
            assert len(data) == row['bytes'] and data.count(b'\n') == row['lines'], path
            assert hashlib.sha256(data).hexdigest() == row['sha256'], path
        else:
            assert data == raw[path], path
        checks += 1
    root = fixture()
    recovered = root / 'target/all-records'
    success(command(root, 'materialize', 'target/all-records'))
    materialized = {p.relative_to(recovered).as_posix(): p.read_bytes()
                    for p in recovered.rglob('*.md')}
    assert set(materialized) == expected_paths, 'materialization membership drift'
    for path, data in materialized.items():
        assert data == success(command(ROOT, 'read', path)), path
    checks += 1
    # A copied fixture with no raw records of packed members retrieves from payloads, without Git.
    newest_path, newest = manifest_paths[-1], manifests[-1]
    assert not any((root / row['path']).exists() for row in newest['members'])
    success(command(root, 'verify'))
    checks += 1

    # Independently compare the actual human label; inventory intentionally ignores prose.
    root = fixture()
    catalog = root / newest['catalog']
    lines = catalog.read_text().splitlines(keepends=True)
    lines[0] = '# Retained history — window 0\n'
    catalog.write_text(''.join(lines))
    success(command(root, 'verify'))
    try:
        catalog_title(root, newest)
    except AssertionError as error:
        assert str(error) == 'catalog title identity mismatch', error
    else:
        raise AssertionError('misleading newest title passed the independent label control')
    checks += 1
    print('  window label control: misleading newest title refused; inventory unchanged')

    def corrupt_payload(root):
        payload = root / newest['payload']
        payload.write_bytes(payload.read_bytes() + b'fault')

    def member_digest(root):
        path = root / newest_path
        manifest = json.loads(path.read_text())
        manifest['members'][0]['sha256'] = '0' * 64
        path.write_text(json.dumps(manifest, indent=2) + '\n')

    def catalog_member(root):
        path = root / newest['catalog']
        heading = '### ' + Path(newest['members'][0]['path']).name
        text = path.read_text()
        assert text.count(heading) == 1, 'catalog mutation anchor changed'
        path.write_text(text.replace(heading, '### absent.md', 1))

    refusal('newest payload digest', corrupt_payload, 'payload digest mismatch')
    refusal('newest member identity', member_digest, 'member identity mismatch')
    refusal('newest catalog membership', catalog_member, 'catalog membership/order mismatch')
    if len(manifests) > 1:
        def cross_window_collision(root):
            path = root / newest_path
            manifest = json.loads(path.read_text())
            manifest['members'][0]['path'] = manifests[0]['members'][0]['path']
            path.write_text(json.dumps(manifest, indent=2) + '\n')
        refusal('cross-window logical collision', cross_window_collision, 'duplicate logical identity')
    # An unrelated copied prose edit is green for inventory, not an immutability exemption.
    root = fixture()
    catalog = root / newest['catalog']
    catalog.write_text(catalog.read_text() + '\nFixture control prose.\n')
    success(command(root, 'verify'))
    checks += 1
    prior = subprocess.run(['git', 'show', 'HEAD:' + MASTER], cwd=ROOT, capture_output=True)
    if prior.returncode == 0 and newest_path in json.loads(prior.stdout)['windows']:
        result = command(root, 'verify-retention')
        assert result.returncode == 1 and b'committed retained file changed' in result.stderr, (
            result.returncode, result.stdout, result.stderr)
        checks += 1
        print('  window refusal: newest committed catalog edit rc=1')
    else:
        print('  newest-window immutability control awaits its recording commit')
    print('window CLI contracts: %d controls / 0 fail; %d windows / %d logical reads; newest %s' % (
        checks, len(manifests), len(listed), newest['id']))
finally:
    shutil.rmtree(WORK)
