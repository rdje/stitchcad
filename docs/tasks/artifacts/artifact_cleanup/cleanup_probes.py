"""Actual filesystem refusal controls and compiled producer guard faults; no source mutation."""
from pathlib import Path
import os
import runpy
import shutil
import sys
import types
from unittest.mock import patch

SOURCE = Path(__file__).with_name('cleanup.py')
ROOT = Path(__file__).resolve().parents[4]
BASE = ROOT / 'target/scratch/artifact_cleanup_probes'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'


def load_profile():
    """Check the first helper with local metadata before executing its planning API."""
    parent = PROFILE_SOURCE.parent
    if (parent.is_symlink() or not parent.is_dir() or parent.stat().st_dev != ROOT.stat().st_dev or
            (parent / '.git').exists() or (parent / '.git').is_symlink() or
            PROFILE_SOURCE.is_symlink() or not PROFILE_SOURCE.is_file() or
            PROFILE_SOURCE.stat().st_dev != ROOT.stat().st_dev):
        raise ValueError('unsafe cleanup fixture profile source')
    return runpy.run_path(str(PROFILE_SOURCE))


try:
    PROFILE = load_profile()
except (ValueError, OSError) as error:
    print('cleanup fixtures refused:', error, file=sys.stderr)
    raise SystemExit(2) from None
SOURCES = (Path(__file__), SOURCE, PROFILE_SOURCE, ROOT / 'scripts/check_handoff.sh',
           ROOT / 'scripts/handoff_census.py', ROOT / 'README.md')
NAMES = ('tracked', 'symlink-entry', 'symlink-parent', 'git-boundary', 'git-descendant',
         'nonignored', 'protected-store', 'escaping', 'absolute', 'special-file',
         'plan-drift', 'record-locality', 'device-parent', 'device-entry', 'valid')
FILE_NAMES = ('keep', 'link', 'pipe', 'output.log')
OWNED = set()


def producer_plan(*, directories=(), output_files=(), activate=False):
    """Validate the whole selection before activating stores or touching owned fixtures."""
    PROFILE['plan_producer'](ROOT, sources=SOURCES, directories=directories, output_files=output_files)
    environment = PROFILE['environment_for'](ROOT, os.environ, create=activate)
    if activate:
        os.environ.update(environment)
    return environment


def fixture_path(fixture):
    relative = PROFILE['producer_path'](ROOT, fixture)
    path = ROOT / relative
    PROFILE['require'](path.parent == BASE and path.name not in ('', '.', '.git'),
                       'fixture must be a named child of the cleanup probe base')
    return path


def construction_plan(fixture=None, *, activate=False):
    fixtures = tuple(BASE / name for name in NAMES) if fixture is None else (fixture_path(fixture),)
    directories = (BASE, *(path for fixture in fixtures for path in (fixture, fixture / 'nested')))
    files = tuple(fixture / name for fixture in fixtures for name in FILE_NAMES)
    return producer_plan(directories=directories, output_files=files, activate=activate)


def removal_plan(fixture):
    """Own only the freshly created tree; deliberate unsafe children are test data."""
    fixture = fixture_path(fixture)
    producer_plan(directories=(BASE,))
    PROFILE['require'](fixture in OWNED, 'fixture removal is not owned')
    PROFILE['require'](not fixture.is_symlink() and fixture.is_dir() and
                       fixture.stat().st_dev == ROOT.stat().st_dev, 'unsafe owned fixture root')
    PROFILE['require'](shutil.rmtree.avoids_symlink_attacks, 'runtime lacks safe fixture removal')


def remove_fixture(fixture):
    removal_plan(fixture)
    # These are empty controlled markers, never initialized repositories. rmdir
    # refuses a nonempty marker; teardown stops before removing its containing tree.
    for marker in fixture.rglob('.git'):
        removal_plan(fixture)
        PROFILE['require'](not marker.is_symlink() and marker.is_dir() and
                           marker.stat().st_dev == ROOT.stat().st_dev, 'unsafe controlled Git marker')
        marker.rmdir()
    producer_plan(directories=(fixture,))
    shutil.rmtree(fixture)
    OWNED.remove(fixture)


try:
    construction_plan(activate=True)
except (PROFILE['Refusal'], OSError) as error:
    print('cleanup fixtures refused:', error, file=sys.stderr)
    raise SystemExit(2) from None
BASE.mkdir(parents=True, exist_ok=True)
GOOD = runpy.run_path(str(SOURCE))
PASS = 0


def refusal(module, operation, reason):
    try:
        operation(module)
    except module['Refusal'] as error:
        assert reason in str(error), (reason, error)
    else:
        raise AssertionError('guard failed to refuse ' + reason)


def case(name, setup, operation, reason, mutation=None):
    global PASS
    fixture = BASE / name
    construction_plan(fixture)
    require_absent = not fixture.exists()
    assert require_absent, ('stale controlled fixture', fixture)
    construction_plan(fixture, activate=True)
    fixture.mkdir()
    OWNED.add(fixture)
    try:
        setup(fixture)
        refusal(GOOD, lambda module: operation(module, fixture), reason)
        if mutation:
            before, after = mutation
            producer_plan()
            source = SOURCE.read_text()
            assert source.count(before) == 1, ('actual guard anchor', name)
            # These original faults can expose a linked read-only source. Isolate
            # the newer deletion-link guard in their in-memory mutants; its real
            # policy and separate D211 controls remain enabled.
            if name in ('symlink-entry', 'nonignored'):
                link_guard = "require(entry.stat().st_nlink == 1, 'hard-linked entry: ' + rel)"
                assert source.count(link_guard) == 1, ('link isolation anchor', name)
                source = source.replace(link_guard, 'pass', 1)
            module = {'__file__': str(SOURCE), '__name__': 'cleanup_fault'}
            exec(compile(source.replace(before, after, 1), str(SOURCE), 'exec'), module)
            try:
                refusal(module, lambda m: operation(m, fixture), reason)
            except AssertionError as error:
                assert str(error).startswith('guard failed to refuse '), (name, error)
            else:
                raise AssertionError(('actual removed guard did not fail assertion', name))
            print('  actual compiled guard assertion red:', name)
        print('  refusal:', name)
        PASS += 1
    finally:
        remove_fixture(fixture)


def relative(path):
    return path.relative_to(ROOT).as_posix()


case('tracked', lambda p: (p / 'keep').write_text('input'),
     lambda m, p: m['inspect'](relative(p), {relative(p / 'keep')}), 'tracked descendant',
     ("require(not any(under(p, name) for p in tracked_paths), 'tracked descendant: ' + name)", 'pass'))
case('symlink-entry', lambda p: (p / 'link').symlink_to(ROOT / 'README.md'),
     lambda m, p: m['inspect'](relative(p), set()), 'symlink entry',
     ("require(not entry.is_symlink(), 'symlink entry: ' + rel)", 'pass'))
case('symlink-parent', lambda p: (p / 'link').symlink_to(ROOT / 'target', target_is_directory=True),
     lambda m, p: m['local'](Path(relative(p / 'link/absent'))), 'symlink ancestor',
     ("require(not parent.is_symlink(), 'symlink ancestor: ' + str(path))", 'pass'))
case('git-boundary', lambda p: (p / '.git').mkdir(),
     lambda m, p: m['local'](Path(relative(p))), 'nested repository',
     ("require(not (parent / '.git').exists(), 'nested repository: ' + str(path))", 'pass'))
case('git-descendant', lambda p: (p / 'nested/.git').mkdir(parents=True),
     lambda m, p: m['inspect'](relative(p), set()), 'nested repository',
     ("require(entry.name != '.git', 'nested repository: ' + rel)", 'pass'))
case('nonignored', lambda p: None,
     lambda m, p: m['inspect']('README.md', set()), 'not ignored',
     ("require(subprocess.run(['git', '-C', str(ROOT), 'check-ignore', '-q', '--', name], env=environment).returncode == 0,\n            'not ignored: ' + name)", 'pass'))
def protected_store(module, fixture):
    # runpy may return a copy of the module mapping. Change the actual function
    # namespace, retaining every real prefix and restoring it on every outcome.
    namespace = module['inspect'].__globals__
    original = namespace['PROTECTED']
    namespace['PROTECTED'] = (*original, relative(fixture))
    try:
        module['inspect'](relative(fixture), set())
    finally:
        namespace['PROTECTED'] = original


# The genuine store still refuses with all guards intact. Never run the
# removed-guard mutant on its uncontrolled contents (which may include Git).
refusal(GOOD, lambda m: m['inspect']('target/cargo-home', set()), 'protected output/store')
case('protected-store', lambda p: None,
     protected_store, 'protected output/store',
     ("require(not any(under(name, p) for p in PROTECTED), 'protected output/store: ' + name)", 'pass'))
case('escaping', lambda p: None,
     lambda m, p: m['local'](Path('../outside')), 'path locality',
     ("require(not path.is_absolute() and '..' not in path.parts and bool(path.parts), 'path locality')", 'pass'))
case('absolute', lambda p: None,
     lambda m, p: m['local'](p), 'path locality')
case('special-file', lambda p: os.mkfifo(p / 'pipe'),
     lambda m, p: m['inspect'](relative(p), set()), 'special entry')
def plan_drift(module, fixture):
    for key in ['head', 'producer_sha256', 'tracked_sha256', 'candidates']:
        refusal(module, lambda m: m['validate_plan']({key: 'a'}, {key: 'b'}), 'plan drift')
    # The outer refusal operation checks the same actual guard, so its removed-fault arm fails.
    module['validate_plan']({'head': 'a'}, {'head': 'b'})


case('plan-drift', lambda p: None,
     plan_drift, 'plan drift',
     ("require(current == plan, 'plan drift: regenerate with a fresh run identity')", 'pass'))
case('record-locality', lambda p: None,
     lambda m, p: m['record_directory']('docs/overwrite'), 'record must be')

# Simulated metadata exercises both device guards without mounting/writing another volume.
def foreign_device(module, fixture, entry=False):
    selected = fixture / 'output.log' if entry else fixture
    original_stat = Path.stat

    def foreign_stat(path, *args, **kwargs):
        answer = original_stat(path, *args, **kwargs)
        if path == selected:
            return types.SimpleNamespace(st_dev=answer.st_dev + 1, st_mode=answer.st_mode, st_nlink=answer.st_nlink)
        return answer

    with patch.object(Path, 'stat', foreign_stat):
        if entry:
            module['inspect'](relative(selected), set())
        else:
            module['local'](Path(relative(selected)))


case('device-parent', lambda p: None,
     foreign_device, 'foreign device',
     ("require(parent.stat().st_dev == ROOT.stat().st_dev, 'foreign device: ' + str(path))", 'pass'))
case('device-entry', lambda p: (p / 'output.log').write_text('generated'),
     lambda m, p: foreign_device(m, p, entry=True), 'foreign device',
     ("require(entry.stat().st_dev == ROOT.stat().st_dev, 'foreign device: ' + rel)", 'pass'))

fixture = BASE / 'valid'
construction_plan(fixture, activate=True)
fixture.mkdir()
OWNED.add(fixture)
try:
    (fixture / 'output.log').write_text('regenerable output\n')
    row = GOOD['inspect'](relative(fixture), set())
    assert row['files'] == 1 and row['bytes'] == 19 and row['kind'] == 'tree'
    construction_plan(fixture)
    (fixture / 'output.log').write_text('changed output\n')
    assert GOOD['inspect'](relative(fixture), set())['sha256'] != row['sha256']
    GOOD['validate_plan']({'same': [1, 2]}, {'same': [1, 2]})
    PASS += 1
finally:
    remove_fixture(fixture)
assert not any(BASE.iterdir())
print('cleanup probes:', PASS, 'pass / 0 fail; 11 actual compiled assertion reds; source unchanged')
