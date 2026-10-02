"""Actual filesystem refusal controls and compiled producer guard faults; no source mutation."""
from pathlib import Path
import os
import runpy
import shutil
import types
from unittest.mock import patch

SOURCE = Path(__file__).with_name('cleanup.py')
ROOT = SOURCE.resolve().parents[4]
BASE = ROOT / 'target/scratch/artifact_cleanup_probes'
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
    require_absent = not fixture.exists()
    assert require_absent, ('stale controlled fixture', fixture)
    fixture.mkdir()
    setup(fixture)
    try:
        refusal(GOOD, lambda module: operation(module, fixture), reason)
        if mutation:
            before, after = mutation
            source = SOURCE.read_text()
            assert source.count(before) == 1, ('actual guard anchor', name)
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
        # .git is just an empty controlled marker, never an initialized repository.
        for marker in fixture.rglob('.git'):
            marker.rmdir()
        shutil.rmtree(fixture)


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
     ("require(subprocess.run(['git', '-C', str(ROOT), 'check-ignore', '-q', '--', name]).returncode == 0,\n            'not ignored: ' + name)", 'pass'))
case('protected-store', lambda p: None,
     lambda m, p: m['inspect']('target/cargo-home', set()), 'protected output/store',
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
            return types.SimpleNamespace(st_dev=answer.st_dev + 1, st_mode=answer.st_mode)
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
fixture.mkdir()
(fixture / 'output.log').write_text('regenerable output\n')
try:
    row = GOOD['inspect'](relative(fixture), set())
    assert row['files'] == 1 and row['bytes'] == 19 and row['kind'] == 'tree'
    (fixture / 'output.log').write_text('changed output\n')
    assert GOOD['inspect'](relative(fixture), set())['sha256'] != row['sha256']
    GOOD['validate_plan']({'same': [1, 2]}, {'same': [1, 2]})
    PASS += 1
finally:
    shutil.rmtree(fixture)
assert not any(BASE.iterdir())
print('cleanup probes:', PASS, 'pass / 0 fail; 11 actual compiled assertion reds; source unchanged')
