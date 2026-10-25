"""Calibrated archive reader refusals; fixtures stay inside the project volume."""
import copy
import gzip
import hashlib
import importlib.util
import io
import json
import os
from pathlib import Path
import runpy
import shutil
import subprocess
import tarfile

ROOT = Path(__file__).resolve().parents[4]
READER = ROOT / 'scripts/history_archive.py'
SHELL = ROOT / 'scripts/history_archive.sh'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
WORK = ROOT / 'target/scratch/history_archive_probes'
if (PROFILE_SOURCE.parent.is_symlink() or not PROFILE_SOURCE.parent.is_dir() or
        PROFILE_SOURCE.parent.stat().st_dev != ROOT.stat().st_dev or
        (PROFILE_SOURCE.parent / '.git').exists() or (PROFILE_SOURCE.parent / '.git').is_symlink() or
        PROFILE_SOURCE.is_symlink() or not PROFILE_SOURCE.is_file() or
        PROFILE_SOURCE.stat().st_dev != ROOT.stat().st_dev):
    raise ValueError('unsafe archive fixture profile')
PROFILE = runpy.run_path(str(PROFILE_SOURCE))


def backing_inputs():
    files, folders = [], [ROOT / '.doctrine/history_archive', ROOT / 'docs/history']
    for folder in folders:
        PROFILE['directory'](ROOT, folder.relative_to(ROOT).as_posix())
        for path in sorted(folder.iterdir()):
            PROFILE['require'](not path.is_symlink(), 'symlink archive fixture input')
            (folders if path.is_dir() else files).append(path)
    PROFILE['plan_producer'](ROOT, sources=(Path(__file__), PROFILE_SOURCE, READER, SHELL, *files))
    return tuple(files), tuple(folders)


BACKING, INPUT_DIRECTORIES = backing_inputs()
SOURCES = (Path(__file__), PROFILE_SOURCE, READER, SHELL, *BACKING)
FROZEN = {path: path.read_bytes() for path in SOURCES}
SPEC = importlib.util.spec_from_file_location('history', READER)
history = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(history)
SOURCE, _ = history.inventory(ROOT)
MANIFEST = json.loads((ROOT / '.doctrine/history_archive/window1.json').read_text())
PASS = 0


def fixture():
    path = WORK / 'fixture'
    remove_tree(path, ignore_errors=True)
    make_directory(path)
    work_plan()
    shutil.copytree(ROOT / '.doctrine/history_archive', path / '.doctrine/history_archive')
    work_plan()
    shutil.copytree(ROOT / 'docs/history', path / 'docs/history')
    return path


def save(root, manifest):
    write_file(root / '.doctrine/history_archive/window1.json', json.dumps(manifest, indent=2) + '\n')


def replace_payload(root, raw):
    manifest = json.loads((root / '.doctrine/history_archive/window1.json').read_text())
    unlink_file(root / manifest['payload'])
    sha = hashlib.sha256(raw).hexdigest()
    manifest['payload_sha256'] = sha
    manifest['payload'] = 'docs/history/payloads/' + sha + '.tar.gz'
    write_file(root / manifest['payload'], raw)
    save(root, manifest)
    return manifest


def packed_payload(change=None, extra=None):
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
    return manifest, gzip.compress(data.getvalue(), mtime=0)


def pack(root, change=None, extra=None):
    manifest, raw = packed_payload(change, extra)
    replace_payload(root, raw)
    installed = json.loads((root / '.doctrine/history_archive/window1.json').read_text())
    manifest['payload'] = installed['payload']
    manifest['payload_sha256'] = installed['payload_sha256']
    save(root, manifest)


def resident_payload(padding):
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
    return manifest, raw, member


def fixture_population():
    fixture_root = WORK / 'fixture'
    files = {fixture_root / path.relative_to(ROOT) for path in BACKING}
    directories = {WORK.parent, WORK, fixture_root}
    directories.update(fixture_root / path.relative_to(ROOT) for path in INPUT_DIRECTORIES)
    files.update((fixture_root / '.doctrine/history_archive/orphan.json',
                  fixture_root / 'docs/history/payloads/orphan.bin',
                  fixture_root / MANIFEST['members'][0]['path']))
    files.update(fixture_root / ('docs/history/stitchcad-defects-part%d.md' % (1000 + index)) for index in range(22))
    files.update(WORK / 'materialized' / path for path in SOURCE)
    link = tarfile.TarInfo('docs/history/stitchcad-defects-part999.md')
    link.type, link.linkname = tarfile.SYMTYPE, '/outside'
    duplicate = tarfile.TarInfo(MANIFEST['members'][0]['path'])
    selected = {row['path'] for row in MANIFEST['members'][:26]}
    payloads = (
        packed_payload(extra=link)[1], packed_payload(extra=duplicate)[1],
        packed_payload(lambda row, content: b'a' * 401 + b'\n' if row == MANIFEST['members'][0] else content)[1],
        packed_payload(lambda row, content: (b'a' * 399 + b'\n') * 400 if row['path'] in selected else content)[1],
        gzip.compress(gzip.decompress((ROOT / MANIFEST['payload']).read_bytes()) + b'hidden nonzero tail', mtime=0),
        gzip.compress(b'\0' * (history.TAR_BYTES + 1), mtime=0),
        resident_payload(0)[1], resident_payload(700000)[1],
    )
    files.update(fixture_root / ('docs/history/payloads/' + history.digest(raw) + '.tar.gz') for raw in payloads)
    for path in files:
        directories.update(parent for parent in path.parents if parent == WORK or parent.is_relative_to(WORK))
    return frozenset(directories), frozenset(files)


DIRECTORIES, FILES = fixture_population()
OWNED_LINK = WORK / 'fixture' / MANIFEST['payload']


def work_plan(*, activate=False, owned_link=False):
    current_files, current_directories = backing_inputs()
    PROFILE['require'](set(current_files) == set(BACKING) and set(current_directories) == set(INPUT_DIRECTORIES),
                       'archive fixture input population changed')
    PROFILE['require'](all(path.read_bytes() == content for path, content in FROZEN.items()),
                       'archive fixture source bytes changed')
    PROFILE['directory'](ROOT, WORK.relative_to(ROOT).as_posix(), missing_ok=True)
    links = set()
    if WORK.exists():
        for folder, children, leaves in os.walk(WORK, followlinks=False):
            for name in (*children, *leaves):
                path = Path(folder) / name
                PROFILE['require'](path in DIRECTORIES or path in FILES, 'unknown archive fixture entry')
                if path.is_symlink():
                    PROFILE['require'](owned_link and path == OWNED_LINK and
                                       os.readlink(path) == str(ROOT / MANIFEST['payload']),
                                       'unexpected archive fixture link')
                    links.add(path)
    PROFILE['plan_producer'](ROOT, sources=SOURCES, directories=tuple(sorted(DIRECTORIES)),
                             output_files=tuple(sorted(FILES - links)))
    environment = PROFILE['environment_for'](ROOT, os.environ, create=False)
    PROFILE['require'](not any(Path(environment[name]).is_relative_to(WORK) for name in PROFILE['STORES']),
                       'archive fixture workspace contains a store')
    if activate:
        os.environ.update(PROFILE['environment_for'](ROOT, os.environ, create=True))
    return environment


def child_environment():
    environment = {name: value for name, value in work_plan().items() if not name.startswith('GIT_')}
    environment.update(GIT_CONFIG_NOSYSTEM='1', GIT_CONFIG_GLOBAL=os.devnull, GIT_OPTIONAL_LOCKS='0',
                       GIT_LITERAL_PATHSPECS='1', GIT_TERMINAL_PROMPT='0')
    return environment


def remove_tree(path, **kwargs):
    work_plan(owned_link=True)
    PROFILE['require'](path in DIRECTORIES and path.is_relative_to(WORK), 'unowned archive fixture removal')
    PROFILE['require'](shutil.rmtree.avoids_symlink_attacks, 'runtime lacks safe archive fixture removal')
    shutil.rmtree(path, **kwargs)


def make_directory(path, **kwargs):
    work_plan()
    PROFILE['require'](path in DIRECTORIES, 'unplanned archive fixture directory')
    path.mkdir(**kwargs)


def write_file(path, content):
    work_plan()
    PROFILE['require'](path in FILES, 'unplanned archive fixture file')
    return path.write_bytes(content) if isinstance(content, bytes) else path.write_text(content)


def unlink_file(path):
    work_plan()
    PROFILE['require'](path in FILES, 'unplanned archive fixture unlink')
    path.unlink()


def make_link(path, target):
    work_plan()
    PROFILE['require'](path == OWNED_LINK and target == ROOT / MANIFEST['payload'], 'unowned archive fixture link')
    path.symlink_to(target)


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


work_plan(activate=True, owned_link=True)
try:
    remove_tree(WORK, ignore_errors=True)
    make_directory(WORK, parents=True)
    arm('GREEN exact logical inventory', lambda p: None)
    recovered = WORK / 'materialized'
    subprocess.run(['bash', str(ROOT / 'scripts/history_archive.sh'), 'materialize', 'target/scratch/history_archive_probes/materialized'], cwd=ROOT, check=True, capture_output=True, env=child_environment())
    assert {p.relative_to(recovered).as_posix(): p.read_bytes() for p in recovered.rglob('*.md')} == SOURCE
    print('  ✓ GREEN materialization matches every logical byte')
    PASS += 1
    # A refusal must be the intended predicate, not an exception or another arm.
    arm('RED payload digest', lambda p: write_file(p / MANIFEST['payload'], b'corrupt'), 'payload digest mismatch')
    arm('RED missing payload', lambda p: unlink_file(p / MANIFEST['payload']), 'missing file')
    arm('RED catalog membership', lambda p: write_file(p / MANIFEST['catalog'], '# empty\n'), 'catalog membership/order mismatch')
    arm('RED orphan payload', lambda p: write_file(p / 'docs/history/payloads/orphan.bin', b'x'), 'orphan/missing payload')
    arm('RED orphan control', lambda p: write_file(p / '.doctrine/history_archive/orphan.json', '{}'), 'orphan/missing archive control')
    arm('RED raw/packed collision', lambda p: write_file(p / MANIFEST['members'][0]['path'], SOURCE[MANIFEST['members'][0]['path']]), 'raw/packed duplicate')
    arm('RED duplicate JSON key', lambda p: write_file(p / history.MASTER, '{"version":1,"version":1,"windows":[]}'), 'duplicate JSON key')
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
        c = (p / m['catalog']); write_file(c, c.read_text().replace(Path(MANIFEST['members'][0]['path']).name, 'stitchcad-defects-part999.md'))
    arm('RED unexpected tar member', missing_member, 'unexpected/unsafe tar member')
    def link(p):
        m = tarfile.TarInfo('docs/history/stitchcad-defects-part999.md'); m.type = tarfile.SYMTYPE; m.linkname = '/outside'; pack(p, extra=m)
    arm('RED tar symlink', link, 'unexpected/unsafe tar member')
    def duplicated_tar(p):
        m = tarfile.TarInfo(MANIFEST['members'][0]['path']); pack(p, extra=m)
    arm('RED duplicate tar member', duplicated_tar, 'duplicate tar member')
    def symlink(p):
        payload = p / MANIFEST['payload']; unlink_file(payload); make_link(payload, ROOT / MANIFEST['payload'])
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
        remove_tree(p / '.doctrine/history_archive')
        remove_tree(p / 'docs/history')
        make_directory(p / '.doctrine/history_archive', parents=True)
        make_directory(p / 'docs/history/payloads', parents=True)
        write_file(p / history.MASTER, json.dumps({
            'version': 1, 'windows': ['.doctrine/history_archive/window1.json']}) + '\n')
        manifest, raw, member = resident_payload(padding)
        write_file(p / manifest['payload'], raw)
        write_file(p / manifest['catalog'], '### ' + Path(member['path']).name + '\n')
        save(p, manifest)
        for i in range(22):
            write_file(p / ('docs/history/stitchcad-defects-part%d.md' % (1000 + i)), (b'a' * 399 + b'\n') * 400)
    arm('GREEN resident fixture below both aggregate ceilings', lambda p: resident_fixture(p, 0))
    arm('RED resident aggregate below decoded ceiling', lambda p: resident_fixture(p, 700000),
        'resident history aggregate bound exceeded')
    for command, argument, expected in [('read', 'docs/history/missing.md', 'unknown logical identity'), ('materialize', '../escape', 'destination must be under target/'), ('materialize', 'target/scratch/history_archive_probes/materialized', 'destination exists')]:
        result = subprocess.run(['bash', str(ROOT / 'scripts/history_archive.sh'), command, argument], cwd=ROOT, capture_output=True, env=child_environment())
        assert result.returncode == 1 and expected.encode() in result.stderr, result.stderr
        print('  ✓ RED ' + command + ' ' + expected); PASS += 1
    # HEAD-backed refusal is available after the initial transition commit. It
    # reads this project only; it creates no nested Git repository.
    prior = subprocess.run(['git', 'show', 'HEAD:' + history.MASTER], cwd=ROOT, capture_output=True, env=child_environment())
    if prior.returncode == 0:
        p = fixture()
        write_file(p / MANIFEST['catalog'], (p / MANIFEST['catalog']).read_text() + '\nSuperseding content must be a new record.\n')
        history.inventory(p)
        try:
            work_plan()
            history.guard_committed(p)
        except history.Refusal as error:
            assert 'committed retained file changed' in str(error)
        else:
            raise AssertionError('committed window edit passed')
        print('  ✓ RED committed immutable window'); PASS += 1
    else:
        absence = subprocess.run(['git', 'ls-tree', '--name-only', 'HEAD', '--', history.MASTER],
                                 cwd=ROOT, capture_output=True, check=True, env=child_environment())
        PROFILE['require'](absence.stdout == b'', 'archive fixture HEAD metadata unavailable')
        print('  - HEAD immutability mutation awaits initial transition commit')
    print('archive probes: %d pass / 0 fail' % PASS)
finally:
    remove_tree(WORK)
