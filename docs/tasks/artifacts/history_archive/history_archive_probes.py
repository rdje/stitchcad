"""Calibrated archive reader refusals; fixtures stay inside the project volume."""
import copy
import gzip
import hashlib
import importlib.util
import io
import json
from pathlib import Path
import shutil
import subprocess
import tarfile

ROOT = Path(__file__).resolve().parents[4]
SPEC = importlib.util.spec_from_file_location('history', ROOT / 'scripts/history_archive.py')
history = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(history)
WORK = ROOT / 'target/scratch/history_archive_probes'
shutil.rmtree(WORK, ignore_errors=True)
WORK.mkdir(parents=True)
SOURCE, _ = history.inventory(ROOT)
MANIFEST = json.loads((ROOT / '.doctrine/history_archive/window1.json').read_text())
PASS = 0


def fixture():
    path = WORK / 'fixture'
    shutil.rmtree(path, ignore_errors=True)
    path.mkdir()
    shutil.copytree(ROOT / '.doctrine/history_archive', path / '.doctrine/history_archive')
    shutil.copytree(ROOT / 'docs/history', path / 'docs/history')
    return path


def save(root, manifest):
    (root / '.doctrine/history_archive/window1.json').write_text(json.dumps(manifest, indent=2) + '\n')


def replace_payload(root, raw):
    manifest = json.loads((root / '.doctrine/history_archive/window1.json').read_text())
    (root / manifest['payload']).unlink()
    sha = hashlib.sha256(raw).hexdigest()
    manifest['payload_sha256'] = sha
    manifest['payload'] = 'docs/history/payloads/' + sha + '.tar.gz'
    (root / manifest['payload']).write_bytes(raw)
    save(root, manifest)
    return manifest


def pack(root, change=None, extra=None):
    manifest = copy.deepcopy(MANIFEST)
    data = io.BytesIO()
    with tarfile.open(fileobj=data, mode='w', format=tarfile.PAX_FORMAT) as archive:
        for row in manifest['members']:
            content = SOURCE[row['path']]
            if change:
                content = change(row, content)
            row.update(lines=content.count(b'\n'), bytes=len(content), sha256=history.digest(content))
            member = tarfile.TarInfo(row['path'])
            member.size = len(content)
            archive.addfile(member, io.BytesIO(content))
        if extra:
            archive.addfile(extra)
    replace_payload(root, gzip.compress(data.getvalue(), mtime=0))
    installed = json.loads((root / '.doctrine/history_archive/window1.json').read_text())
    manifest['payload'] = installed['payload']
    manifest['payload_sha256'] = installed['payload_sha256']
    save(root, manifest)


def arm(name, operation, expected=None):
    global PASS
    root = fixture()
    try:
        operation(root)
        history.inventory(root)
    except history.Refusal as error:
        assert expected and expected in str(error), (name, str(error), expected)
    else:
        assert expected is None, (name, 'mutation passed')
    print('  ✓ ' + name)
    PASS += 1


try:
    arm('GREEN exact logical inventory', lambda p: None)
    recovered = WORK / 'materialized'
    subprocess.run(['bash', str(ROOT / 'scripts/history_archive.sh'), 'materialize', 'target/scratch/history_archive_probes/materialized'], cwd=ROOT, check=True, capture_output=True)
    assert {p.relative_to(recovered).as_posix(): p.read_bytes() for p in recovered.rglob('*.md')} == SOURCE
    print('  ✓ GREEN materialization matches every logical byte')
    PASS += 1
    # A refusal must be the intended predicate, not an exception or another arm.
    arm('RED payload digest', lambda p: (p / MANIFEST['payload']).write_bytes(b'corrupt'), 'payload digest mismatch')
    arm('RED missing payload', lambda p: (p / MANIFEST['payload']).unlink(), 'missing file')
    arm('RED catalog membership', lambda p: (p / MANIFEST['catalog']).write_text('# empty\n'), 'catalog membership/order mismatch')
    arm('RED orphan payload', lambda p: (p / 'docs/history/payloads/orphan.bin').write_bytes(b'x'), 'orphan/missing payload')
    arm('RED orphan control', lambda p: (p / '.doctrine/history_archive/orphan.json').write_text('{}'), 'orphan/missing archive control')
    arm('RED raw/packed collision', lambda p: (p / MANIFEST['members'][0]['path']).write_bytes(SOURCE[MANIFEST['members'][0]['path']]), 'raw/packed duplicate')
    arm('RED duplicate JSON key', lambda p: (p / history.MASTER).write_text('{"version":1,"version":1,"windows":[]}'), 'duplicate JSON key')
    def unknown_field(p):
        m = copy.deepcopy(MANIFEST); m['unknown'] = True; save(p, m)
    arm('RED unknown schema field', unknown_field, 'unknown/missing schema fields')
    def duplicate_member(p):
        m = copy.deepcopy(MANIFEST); m['members'].append(m['members'][0]); save(p, m)
    arm('RED duplicate logical identity', duplicate_member, 'duplicate logical identity')
    def invalid_path(p):
        m = copy.deepcopy(MANIFEST); m['members'][0]['path'] = '../escape.md'; save(p, m)
    arm('RED escaped logical path', invalid_path, 'invalid logical path')
    def bool_integer(p):
        m = copy.deepcopy(MANIFEST); m['members'][0]['lines'] = True; save(p, m)
    arm('RED bool masquerading as integer', bool_integer, 'invalid integer/bound')
    def bad_identity(p):
        m = copy.deepcopy(MANIFEST); m['members'][0]['sha256'] = '0' * 64; save(p, m)
    arm('RED member digest', bad_identity, 'member identity mismatch')
    def missing_member(p):
        m = copy.deepcopy(MANIFEST); m['members'][0]['path'] = 'docs/history/stitchcad-defects-part999.md'; save(p, m)
        c = (p / m['catalog']); c.write_text(c.read_text().replace(Path(MANIFEST['members'][0]['path']).name, 'stitchcad-defects-part999.md'))
    arm('RED unexpected tar member', missing_member, 'unexpected/unsafe tar member')
    def link(p):
        m = tarfile.TarInfo('docs/history/stitchcad-defects-part999.md'); m.type = tarfile.SYMTYPE; m.linkname = '/outside'; pack(p, extra=m)
    arm('RED tar symlink', link, 'unexpected/unsafe tar member')
    def duplicated_tar(p):
        m = tarfile.TarInfo(MANIFEST['members'][0]['path']); pack(p, extra=m)
    arm('RED duplicate tar member', duplicated_tar, 'duplicate tar member')
    def symlink(p):
        payload = p / MANIFEST['payload']; payload.unlink(); payload.symlink_to(ROOT / MANIFEST['payload'])
    arm('RED filesystem symlink', symlink, 'symlink path refused')
    def tar_tail(p):
        raw = gzip.decompress((p / MANIFEST['payload']).read_bytes())
        replace_payload(p, gzip.compress(raw + b'hidden nonzero tail', mtime=0))
    arm('RED hidden tar tail', tar_tail, 'unparsed nonzero tar tail')
    arm('RED decompression bound', lambda p: replace_payload(p, gzip.compress(b'\0' * (history.TAR_BYTES + 1), mtime=0)), 'decompressed tar bound exceeded')
    arm('RED record maxline', lambda p: pack(p, lambda row, content: b'a' * 401 + b'\n' if row == MANIFEST['members'][0] else content), 'logical maxline bound exceeded')
    def decoded_overflow(p):
        selected = {r['path'] for r in MANIFEST['members'][:26]}
        pack(p, lambda row, content: (b'a' * 399 + b'\n') * 400 if row['path'] in selected else content)
    arm('RED decoded aggregate independent of compression', decoded_overflow, 'decoded history aggregate bound exceeded')
    def resident_fixture(p, padding):
        # This predicate must not depend on accumulated production history. Use a minimal valid
        # one-member window; the catalog, controls and 22 raw records are fixed in both arms.
        shutil.rmtree(p / '.doctrine/history_archive')
        shutil.rmtree(p / 'docs/history')
        (p / '.doctrine/history_archive').mkdir(parents=True)
        (p / 'docs/history/payloads').mkdir(parents=True)
        (p / history.MASTER).write_text(json.dumps({
            'version': 1, 'windows': ['.doctrine/history_archive/window1.json']}) + '\n')
        content = b'x\n'
        manifest = copy.deepcopy(MANIFEST)
        member = copy.deepcopy(manifest['members'][0])
        member.update(lines=1, bytes=len(content), sha256=history.digest(content))
        manifest['members'] = [member]
        data = io.BytesIO()
        with tarfile.open(fileobj=data, mode='w', format=tarfile.PAX_FORMAT) as archive:
            entry = tarfile.TarInfo(member['path'])
            entry.size = len(content)
            archive.addfile(entry, io.BytesIO(content))
        raw = gzip.compress(data.getvalue(), mtime=0) + b'\0' * padding
        manifest['payload_sha256'] = history.digest(raw)
        manifest['payload'] = 'docs/history/payloads/' + manifest['payload_sha256'] + '.tar.gz'
        (p / manifest['payload']).write_bytes(raw)
        (p / manifest['catalog']).write_text('### ' + Path(member['path']).name + '\n')
        save(p, manifest)
        for i in range(22):
            (p / ('docs/history/stitchcad-defects-part%d.md' % (1000 + i))).write_bytes((b'a' * 399 + b'\n') * 400)
    arm('GREEN resident fixture below both aggregate ceilings', lambda p: resident_fixture(p, 0))
    arm('RED resident aggregate below decoded ceiling', lambda p: resident_fixture(p, 700000),
        'resident history aggregate bound exceeded')
    for command, argument, expected in [('read', 'docs/history/missing.md', 'unknown logical identity'), ('materialize', '../escape', 'destination must be under target/'), ('materialize', 'target/scratch/history_archive_probes/materialized', 'destination exists')]:
        result = subprocess.run(['bash', str(ROOT / 'scripts/history_archive.sh'), command, argument], cwd=ROOT, capture_output=True)
        assert result.returncode == 1 and expected.encode() in result.stderr, result.stderr
        print('  ✓ RED ' + command + ' ' + expected); PASS += 1
    # HEAD-backed refusal is available after the initial transition commit. It
    # reads this project only; it creates no nested Git repository.
    prior = subprocess.run(['git', 'show', 'HEAD:' + history.MASTER], cwd=ROOT, capture_output=True)
    if prior.returncode == 0:
        p = fixture()
        (p / MANIFEST['catalog']).write_text((p / MANIFEST['catalog']).read_text() + '\nSuperseding content must be a new record.\n')
        history.inventory(p)
        try:
            history.guard_committed(p)
        except history.Refusal as error:
            assert 'committed retained file changed' in str(error)
        else:
            raise AssertionError('committed window edit passed')
        print('  ✓ RED committed immutable window'); PASS += 1
    else:
        print('  - HEAD immutability mutation awaits initial transition commit')
    print('archive probes: %d pass / 0 fail' % PASS)
finally:
    shutil.rmtree(WORK)
