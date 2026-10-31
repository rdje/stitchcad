"""Read-only actual numerical snapshot plans with isolated Git selectors and no native effects."""
from pathlib import Path
import contextlib
import hashlib
import io
import os
import re
import runpy
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
HERE = ROOT / 'docs/tasks/artifacts/formula_structure'
DRIVER = HERE / 'root_body_faults.py'
ENTRIES = tuple(HERE / (name + '_body_faults.py') for name in ('root', 'arc', 'forward', 'inverse'))
PLATFORM = HERE / 'irrational_platform.py'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
CORE = (Path(__file__), PROFILE_SOURCE, *ENTRIES, PLATFORM,
        ROOT / 'docs/tasks/artifacts/artifact_cleanup/cleanup.py',
        ROOT / 'scripts/check_handoff.sh', ROOT / 'scripts/handoff_census.py')
PROFILE['plan_producer'](ROOT, sources=CORE)
# Independent Git oracle: neither the tested helper nor a native compiler is invoked here.
CLEAN = {name: value for name, value in os.environ.items() if not name.startswith('GIT_')}
CLEAN.update(GIT_CONFIG_NOSYSTEM='1', GIT_CONFIG_GLOBAL=os.devnull,
             GIT_OPTIONAL_LOCKS='0', GIT_LITERAL_PATHSPECS='1')
TRACKED = subprocess.check_output(['git', 'ls-files', 'crates'], cwd=ROOT, env=CLEAN, text=True).splitlines()
SNAPSHOT = {ROOT / name for name in TRACKED} | {ROOT / name for name in (
    'Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml')}
PROFILE['plan_producer'](ROOT, sources=(*CORE, *SNAPSHOT))
FROZEN = {path: hashlib.sha256(path.read_bytes()).digest() for path in {*CORE, *SNAPSHOT}}


def module(path, script=None):
    namespace = dict(__file__=str(path), __name__='numerical_inventory_control')
    exec(compile(path.read_text() if script is None else script, str(path), 'exec'), namespace)
    return namespace


def capture(entry, script=None):
    saved, argv = dict(os.environ), sys.argv
    stores = {key: str(ROOT / 'target/numerical-inventory-caller' / key.lower()) for key in PROFILE['STORES']}
    polluted = dict(stores, NUMERICAL_INVENTORY_CALLER='preserved', RUSTUP_TOOLCHAIN='1.99.0',
                    GIT_DIR=str(ROOT / 'target/nonexistent-inventory.git'),
                    GIT_INDEX_FILE=str(ROOT / 'target/unrelated-inventory-index'),
                    GIT_WORK_TREE=str(ROOT / 'target/unrelated-inventory-worktree'),
                    GIT_CONFIG_COUNT='1', GIT_CONFIG_KEY_0='core.worktree',
                    GIT_CONFIG_VALUE_0=str(ROOT / 'target/unrelated-inventory-worktree'))
    os.environ.update(polluted)
    configured = dict(os.environ)
    originals = subprocess.run, subprocess.Popen, Path.write_text, Path.write_bytes, os.mkdir
    commands = []

    def environment(value):
        assert value is not None, 'inventory child borrowed ambient Git authority'
        allowed = dict(GIT_CONFIG_NOSYSTEM='1', GIT_CONFIG_GLOBAL=os.devnull,
                       GIT_OPTIONAL_LOCKS='0', GIT_LITERAL_PATHSPECS='1')
        assert {key: item for key, item in value.items() if key.startswith('GIT_')} == allowed
        assert {key: item for key, item in value.items() if not key.startswith('GIT_')} == {
            key: item for key, item in configured.items() if not key.startswith('GIT_')}

    def child(command, **kwargs):
        assert command == ['git', 'ls-files', 'crates'] and kwargs['cwd'] == ROOT
        assert kwargs['check'] and kwargs['capture_output'] and kwargs['text']
        environment(kwargs.get('env'))
        commands.append(command)
        return originals[0](command, **kwargs)

    def process(command, *args, **kwargs):
        assert command == ['git', 'ls-files', 'crates'], 'plan-only reached a compiler or other child'
        environment(kwargs.get('env'))
        return originals[1](command, *args, **kwargs)

    def effect(*args, **kwargs):
        raise AssertionError('plan-only created a directory or wrote a file')

    try:
        subprocess.run, subprocess.Popen = child, process
        Path.write_text, Path.write_bytes, os.mkdir = effect, effect, effect
        ns = module(entry, script)
        sys.argv = [str(entry), '--plan-only']
        output = io.StringIO()
        with contextlib.redirect_stdout(output):
            if entry == DRIVER:
                ns['main']()
            else:
                ns['CONTROLLER']['main'](ns['CONFIG'])
        match = re.search(r'native (root|arc|forward|inverse) fault plan:(\d+) variants/(\d+) exact snapshot files/(\d+) complete outputs;', output.getvalue())
        assert match and match[1] == entry.name.split('_')[0]
        variants = dict(root=6, arc=6, forward=8, inverse=10)[match[1]]
        assert int(match[2]) == variants and int(match[3]) == len(SNAPSHOT)
        assert int(match[4]) == len(SNAPSHOT) + variants + 6
        assert len(commands) == 1 and dict(os.environ) == configured and sys.argv[1:] == ['--plan-only']
    finally:
        subprocess.run, subprocess.Popen, Path.write_text, Path.write_bytes, os.mkdir = originals
        sys.argv = argv
        os.environ.clear();os.environ.update(saved)
    assert all(hashlib.sha256(path.read_bytes()).digest() == value for path, value in FROZEN.items())
    return match[1], variants, len(SNAPSHOT)


def faults():
    source = DRIVER.read_text()
    changes = (
        ('ambient', 'env=inventory_environment(), ', ''),
        ('selector', "if not name.startswith('GIT_')", 'if True'),
        ('locking', "GIT_OPTIONAL_LOCKS='0'", "GIT_OPTIONAL_LOCKS='1'"),
        ('global-config', 'GIT_CONFIG_GLOBAL=os.devnull', "GIT_CONFIG_GLOBAL='unrelated-config'"),
        ('caller-reset', "{name: value for name, value in os.environ.items() if not name.startswith('GIT_')}", '{}'),
        ('cwd', "['git', 'ls-files', 'crates'], cwd=ROOT,", "['git', 'ls-files', 'crates'], cwd=ROOT.parent,"),
    )
    for label, old, new in changes:
        assert source.count(old) == 1, label
        try:
            capture(DRIVER, source.replace(old, new))
        except AssertionError:
            pass
        else:
            raise AssertionError('actual inventory body fault survived: ' + label)
    return len(changes)


def platform_capture(script=None):
    saved, argv = dict(os.environ), sys.argv
    stores = {key: str(ROOT / 'target/platform-inventory-caller' / key.lower()) for key in PROFILE['STORES']}
    os.environ.update(stores, PLATFORM_INVENTORY_CALLER='retained',
                      GIT_DIR=str(ROOT / 'target/nonexistent-platform-inventory.git'),
                      GIT_INDEX_FILE=str(ROOT / 'target/unrelated-platform-index'),
                      GIT_WORK_TREE=str(ROOT / 'target/unrelated-platform-worktree'),
                      GIT_CONFIG_COUNT='1', GIT_CONFIG_KEY_0='core.worktree',
                      GIT_CONFIG_VALUE_0=str(ROOT / 'target/unrelated-platform-worktree'))
    configured = dict(os.environ)
    originals = subprocess.run, subprocess.Popen, Path.write_text, Path.write_bytes, os.mkdir
    commands = []

    def effect(*args, **kwargs):
        raise AssertionError('platform plan-only produced a filesystem effect')

    def child(command, **kwargs):
        assert command == ['git', 'ls-files', '-z', 'crates'] and kwargs['cwd'] == ROOT
        assert kwargs['check'] and kwargs['capture_output']
        environment = kwargs.get('env')
        assert environment is not None, 'platform inventory borrowed ambient Git authority'
        expected = dict(GIT_CONFIG_NOSYSTEM='1', GIT_CONFIG_GLOBAL=os.devnull,
                        GIT_OPTIONAL_LOCKS='0', GIT_LITERAL_PATHSPECS='1')
        assert {key: value for key, value in environment.items() if key.startswith('GIT_')} == expected
        wanted = {key: value for key, value in configured.items() if not key.startswith('GIT_')}
        wanted.update(RUSTUP_AUTO_INSTALL='0', MAKE_TMPDIR=stores['TMPDIR'])
        assert {key: value for key, value in environment.items() if not key.startswith('GIT_')} == wanted
        commands.append(command)
        return originals[0](command, **kwargs)

    def process(command, *args, **kwargs):
        assert command == ['git', 'ls-files', '-z', 'crates'], 'platform plan reached compiler/browser'
        return originals[1](command, *args, **kwargs)

    try:
        subprocess.run, subprocess.Popen = child, process
        Path.write_text, Path.write_bytes, os.mkdir = effect, effect, effect
        ns = module(PLATFORM, script)
        real_environment = ns['PROFILE']['environment_for']

        def environment(root, variables, *, create=True):
            assert not create, 'platform inventory activated stores'
            return real_environment(root, variables, create=False)

        ns['PROFILE']['environment_for'] = environment
        sys.argv = [str(PLATFORM), '--plan-only']
        output = io.StringIO()
        with contextlib.redirect_stdout(output):
            ns['main']()
        assert '9 original consumers/31 selected/6 host-only exclusions' in output.getvalue()
        assert '6 original fixtures/405 rows;no writes,compiler or browser dispatch;rc=0' in output.getvalue()
        assert len(commands) == 1 and dict(os.environ) == configured
    finally:
        subprocess.run, subprocess.Popen, Path.write_text, Path.write_bytes, os.mkdir = originals
        sys.argv = argv
        os.environ.clear();os.environ.update(saved)
    assert all(hashlib.sha256(path.read_bytes()).digest() == value for path, value in FROZEN.items())


def platform_faults():
    source = PLATFORM.read_text()
    changes = (
        ('ambient', 'env=inventory_environment(),', 'env=None,'),
        ('selector', "if not name.startswith('GIT_')", 'if True'),
        ('locking', "GIT_OPTIONAL_LOCKS='0'", "GIT_OPTIONAL_LOCKS='1'"),
        ('global-config', 'GIT_CONFIG_GLOBAL=os.devnull', "GIT_CONFIG_GLOBAL='unrelated-config'"),
        ('caller-reset', 'prepared.items()', '{}.items()'),
        ('activation', "PROFILE['environment_for'](ROOT, os.environ, create=False)",
         "PROFILE['environment_for'](ROOT, os.environ, create=True)"),
        ('cwd', "['git', 'ls-files', '-z', 'crates'], cwd=ROOT,",
         "['git', 'ls-files', '-z', 'crates'], cwd=ROOT.parent,"),
    )
    for label, old, new in changes:
        assert source.count(old) == 1, label
        try:
            platform_capture(source.replace(old, new))
        except AssertionError:
            pass
        else:
            raise AssertionError('actual platform inventory fault survived: ' + label)
    return len(changes)


def main():
    plans = [capture(entry) for entry in ENTRIES]
    rejected = faults()
    platform_capture()
    platform_rejected = platform_faults()
    assert all(hashlib.sha256(path.read_bytes()).digest() == value for path, value in FROZEN.items())
    print('numerical inventory:4 actual plan-only entries;variants6/6/8/10;'
          f'{plans[0][2]} exact snapshot files;{rejected} actual body faults;'
          f'platform31 checks/405 rows/{platform_rejected} platform body faults;'
          'Git selectors isolated;caller stores/argv/sources exact;no directory/write/compiler/browser effects;rc=0')


if __name__ == '__main__':
    main()
