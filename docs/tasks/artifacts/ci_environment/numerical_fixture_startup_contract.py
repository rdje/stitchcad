"""Actual fixture bootstrap ordering, caller-store retention and late-input refusal controls."""
from pathlib import Path
import argparse
import os
import shutil
import runpy
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
ENTRY = Path(__file__).with_name('python_numerical_runtime_contract.py')
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
ENV = PROFILE['environment_for'](ROOT, os.environ, create=False)
GIT_ENV = {name:value for name,value in ENV.items() if not name.startswith('GIT_')}
GIT_ENV.update(GIT_CONFIG_NOSYSTEM='1', GIT_CONFIG_GLOBAL=os.devnull, GIT_OPTIONAL_LOCKS='0')
tracked = subprocess.check_output(['git', 'ls-files', 'crates'], cwd=ROOT, env=GIT_ENV, text=True).splitlines()
HERE = ROOT / 'docs/tasks/artifacts/formula_structure'
CORE = {ENTRY, PROFILE_SOURCE, *(HERE / (name + '_body_faults.py') for name in
    ('root', 'arc', 'forward', 'inverse', 'tangent', 'tangent_resource'))}
SNAPSHOT = {ROOT / name for name in tracked} | {ROOT / name for name in ('Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml')}
SOURCES = CORE | SNAPSHOT | {Path(__file__)}
PROFILE['plan_producer'](ROOT, sources=tuple(SOURCES))
FROZEN = {path:path.read_bytes() for path in SOURCES}
WORK = ROOT / 'target/numerical-fixture-startup-controls' / str(os.getpid())
PROFILE['prepare_producer'](ROOT, sources=tuple(SOURCES), directories=(WORK,))
SOURCE = ENTRY.read_text()


def capture(label, *, bad_source=False, script=SOURCE, diagnose=False):
    selected = WORK / label
    caller = {name:str(selected / 'stores' / name.lower()) for name in PROFILE['STORES']}
    saved = dict(os.environ)
    os.environ.update(caller, NUMERICAL_FIXTURE_CALLER='retained',
        GIT_DIR=str(WORK / 'unrelated.git'), GIT_INDEX_FILE=str(WORK / 'unrelated-index'),
        GIT_WORK_TREE=str(WORK / 'unrelated-tree'))
    configured = dict(os.environ)
    original = (runpy.run_path, Path.is_file, subprocess.Popen)
    planned, activated, commands = set(), [], []
    scope = dict(__file__=str(ENTRY), __name__='numerical_fixture_startup_control', CONTROL_WORK=selected / 'work')
    anchor = "WORK = ROOT / 'target/numerical-runtime-controls' / str(os.getpid())"
    assert script.count(anchor) == 1
    script = script.replace(anchor, 'WORK = CONTROL_WORK')
    error = None

    def load(path, *args, **kwargs):
        namespace = original[0](path, *args, **kwargs)
        if Path(path) != PROFILE_SOURCE:
            return namespace
        real_plan, real_prepare = namespace['plan_producer'], namespace['prepare_producer']

        def plan(root, **values):
            result = real_plan(root, **values)
            planned.update(values.get('sources', ()))
            return result

        def prepare(root, **values):
            # Delegate real directory/environment creation: an old early activation is observable.
            real_plan(root, **values)
            activated.append(set(planned))
            if not diagnose:
                assert set(scope.get('FROZEN', ())) == CORE | SNAPSHOT, 'fixture activation precedes input freeze'
            return real_prepare(root, **values)

        namespace.update(plan_producer=plan, prepare_producer=prepare)
        return namespace

    def is_file(path, *args, **kwargs):
        if bad_source and path == ROOT / 'Cargo.lock':
            return False
        return original[1](path, *args, **kwargs)

    def process(arguments, **kwargs):
        assert arguments == ['git', 'ls-files', 'crates'], 'fixture bootstrap reached a native child'
        assert kwargs['cwd'] == ROOT
        environment = kwargs['env']
        allowed = dict(GIT_CONFIG_NOSYSTEM='1', GIT_CONFIG_GLOBAL=os.devnull, GIT_OPTIONAL_LOCKS='0')
        assert {name:value for name,value in environment.items() if name.startswith('GIT_')} == allowed
        assert all(environment.get(name) == value for name,value in caller.items())
        assert environment['NUMERICAL_FIXTURE_CALLER'] == 'retained'
        assert environment['RUSTUP_AUTO_INSTALL'] == '0' and environment['MAKE_TMPDIR'] == caller['TMPDIR']
        commands.append(arguments)
        return original[2](arguments, **kwargs)

    try:
        runpy.run_path, Path.is_file, subprocess.Popen = load, is_file, process
        try:
            exec(compile(script, str(ENTRY), 'exec'), scope)
        except (AssertionError, ValueError) as caught:
            error = caught
        if diagnose:
            assert error is not None and activated and selected.exists(), (error, activated)
            assert all(path.read_bytes() == value for path,value in FROZEN.items())
            print('before:late Cargo.lock refusal follows real owned store/work creation;'
                  'caller environment changed=' + str(dict(os.environ) != configured) + ';no native child;rc=0')
            return
        if bad_source:
            assert error is not None and str(error) == 'producer source is not a local regular file', error
            assert not activated and not selected.exists(), 'fixture activated before complete input refusal'
            assert dict(os.environ) == configured, 'refused fixture changed caller environment'
        else:
            assert error is None, error
            assert len(activated) == 1 and CORE | SNAPSHOT <= activated[0], 'fixture activated before complete source plan'
            assert scope['SNAPSHOT'] == SNAPSHOT
            assert set(scope['FROZEN']) == CORE | SNAPSHOT
            assert all(scope['FROZEN'][path] == FROZEN[path] for path in CORE | SNAPSHOT)
            assert all(os.environ.get(name) == value for name,value in caller.items())
            assert os.environ['NUMERICAL_FIXTURE_CALLER'] == 'retained'
        assert len(commands) == 1
    finally:
        runpy.run_path, Path.is_file, subprocess.Popen = original
        os.environ.clear();os.environ.update(saved)
    assert all(path.read_bytes() == value for path,value in FROZEN.items())


def readonly_git_control():
    """A macOS-only diagnostic denies persistent writes; /dev/null is the required OS sink."""
    launcher = Path('/usr/bin/sandbox-exec')
    assert launcher.is_file(), 'macOS read-only sandbox unavailable'
    marker, missing = WORK / 'denied-metadata-write', WORK / 'missing-metadata-temporary'
    PROFILE['plan_producer'](ROOT, sources=tuple(SOURCES), directories=(missing,), output_files=(marker,))
    assert not marker.exists() and not missing.exists()
    environment = dict(GIT_ENV, TMPDIR=str(missing), MAKE_TMPDIR=str(missing))
    policy = '(version 1) (allow default) (deny file-write*) (allow file-write* (literal "/dev/null"))'
    prefix = [str(launcher), '-p', policy]
    program = ('import os,sys\ntry:os.open(sys.argv[1],os.O_WRONLY|os.O_CREAT,0o600)\n'
               'except PermissionError:print("write denied")\n'
               'else:raise AssertionError("write admitted")')
    negative = subprocess.run(prefix + [sys.executable, '-I', '-B', '-c', program, str(marker)],
        cwd=ROOT, env=environment, capture_output=True, text=True)
    assert negative.returncode == 0 and negative.stdout == 'write denied\n' and not marker.exists()
    executable = shutil.which('git', path=environment.get('PATH'))
    assert executable, 'Git executable unavailable'
    positive = subprocess.run(prefix + [executable, 'ls-files', 'crates'], cwd=ROOT,
        env=environment, capture_output=True, text=True)
    assert positive.returncode == 0 and set(positive.stdout.splitlines()) == set(tracked)
    assert not missing.exists()
    print('metadata read-only control:' + executable + ';' + str(len(tracked)) +
          ' exact rows with all persistent file writes denied;negative creation denied;rc=0')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--diagnose', metavar='REVISION')
    parser.add_argument('--macos-readonly-git', action='store_true')
    args = parser.parse_args()
    if args.diagnose:
        import re
        revision = args.diagnose
        assert re.fullmatch('[0-9a-f]{40}', revision)
        prior = subprocess.check_output(['git', 'show', revision + ':' + ENTRY.relative_to(ROOT).as_posix()],
            cwd=ROOT, env=GIT_ENV, text=True)
        capture('before', bad_source=True, script=prior, diagnose=True)
        return
    if args.macos_readonly_git:
        readonly_git_control()
    capture('valid')
    capture('late-source', bad_source=True)
    faults = (
        ('source-plan', "PROFILE['plan_producer'](ROOT, sources=(*CORE, *SNAPSHOT))", 'pass'),
        ('early-activation',
         "FROZEN = {path: path.read_bytes() for path in {*CORE, *SNAPSHOT}}\n"
         "PROFILE['prepare_producer'](ROOT, sources=(*CORE, *SNAPSHOT), directories=(WORK,))",
         "PROFILE['prepare_producer'](ROOT, sources=(*CORE, *SNAPSHOT), directories=(WORK,))\n"
         "FROZEN = {path: path.read_bytes() for path in {*CORE, *SNAPSHOT}}"),
        ('git-selector', "if not key.startswith('GIT_')", 'if True'),
    )
    for label, old, new in faults:
        assert SOURCE.count(old) == 1, label
        try:
            capture('fault-' + label, script=SOURCE.replace(old, new))
        except AssertionError:
            pass
        else:
            raise AssertionError('actual fixture bootstrap omission survived: ' + label)
    print('numerical fixture startup:valid actual activation/late-source refusal/'
          '3 actual body omissions;caller stores/Git isolation/source bytes exact;no compiler/browser;rc=0')


if __name__ == '__main__':
    main()
