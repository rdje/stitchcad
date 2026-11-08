"""Full actual platform entry with modeled compiler/browser children; no numerical verdict."""
from pathlib import Path
import contextlib
import hashlib
import io
import json
import os
import runpy
import signal
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
ENTRY = ROOT / 'docs/tasks/artifacts/formula_structure/irrational_platform.py'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
WORK = ROOT / 'target/platform-runtime-controls' / str(os.getpid())
PROFILE['prepare_producer'](ROOT, sources=(Path(__file__), ENTRY, PROFILE_SOURCE), directories=(WORK,))
ENV = {name:value for name,value in os.environ.items() if not name.startswith('GIT_')}
ENV.update(GIT_CONFIG_NOSYSTEM='1', GIT_CONFIG_GLOBAL=os.devnull, GIT_OPTIONAL_LOCKS='0')
SNAPSHOT = {ROOT / name for name in subprocess.check_output(
    ['git', 'ls-files', 'crates'], cwd=ROOT, env=ENV, text=True).splitlines()}
SOURCES = SNAPSHOT | {ENTRY, PROFILE_SOURCE, *(ROOT / name for name in (
    'Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml', 'scripts/check_handoff.sh',
    'scripts/handoff_census.py', 'docs/tasks/artifacts/artifact_cleanup/cleanup.py'))}
PROFILE['plan_producer'](ROOT, sources=tuple(SOURCES))
FROZEN = {path:path.read_bytes() for path in SOURCES}
SCRIPT = ENTRY.read_text()


def exercise(label, *, script=SCRIPT, failure=None, diagnose=False):
    scope = dict(__file__=str(ENTRY), __name__='platform_runtime_control')
    exec(compile(script, str(ENTRY), 'exec'), scope)
    base = WORK / label
    store, run, temporary = base / 'store', base / 'store/run', base / 'temporary'
    workspace = run / 'workspace'
    scope.update(STORE=store, RUN=run, WORK=workspace, TEMP=temporary,
                 LATEST=store / 'latest.json', PENDING=store / 'pending.tmp')
    # A repository-local diagnostic executable stands in for the external installed launcher.
    # Only its identity bytes are read; all compiler/browser dispatch below is simulated.
    scope['browser'] = lambda environment: ENTRY
    profile = dict(scope['PROFILE'])
    scope['PROFILE'] = profile
    original_plan, original_prepare = profile['plan_producer'], profile['prepare_producer']
    original_save = scope['save_new']
    files, directories, sources, mutable = set(), set(), set(), set()
    entered, commands, processes, killed, writes = [], [], {}, [], []
    original = (subprocess.Popen, os.killpg, Path.write_bytes, Path.write_text)
    saved, argv = dict(os.environ), sys.argv
    stores = {name:str(WORK / 'caller-stores' / name.lower()) for name in PROFILE['STORES']}
    PROFILE['prepare_producer'](ROOT, directories=tuple(Path(value) for value in stores.values()))
    os.environ.update(stores, PLATFORM_RUNTIME_CALLER='retained')
    wrong = workspace / 'consumers/fixtures/formula_tangent.tsv'
    baseline = b'degree_-720\t-720000000\t1\tvalue\t0\n'
    manifest = workspace / 'Cargo.toml'
    outcomes = []
    published = None
    error = None
    sentinel = None
    if failure == 'log-symlink':
        PROFILE['prepare_producer'](ROOT, directories=(run,), output_files=(base / 'sentinel',))
        sentinel = base / 'sentinel'
        sentinel.write_bytes(b'foreign sentinel\n')
        (run / 'wrong-fixture-chrome.log').symlink_to(sentinel)

    def complete():
        assert SOURCES <= sources, 'platform source plan omitted'
        names = ('rustc', 'chrome-version', 'rustc-final', 'chrome-version-final', 'lock', 'handoff',
                 *(f'{variant}-{kind}' for variant in ('baseline', 'wrong-fixture')
                   for kind in ('native-build', 'wasm-build', 'native-run')))
        required = {manifest, workspace / 'Cargo.lock', workspace / 'main.rs',
                    *(run / (name + '.log') for name in names),
                    *(run / name for name in ('report.json', 'failure.json')),
                    store / 'latest.json', store / 'pending.tmp',
                    *(run / (variant + suffix) for variant in ('baseline', 'wrong-fixture')
                      for suffix in ('.native', '.wasm', '.html', '-dom.html', '-chrome.log')),
                    *(workspace / 'consumers' / name for name in scope['CONSUMERS']),
                    *(workspace / 'consumers/fixtures' / name for name in (
                        'formula_roots.tsv', 'formula_arcs.tsv', 'formula_sincos.tsv',
                        'formula_inverse.tsv', 'formula_tangent.tsv', 'formula_tangent_resources.tsv'))}
        assert required <= files, 'platform complete output plan omitted'
        required_directories = {store, run, workspace, temporary, run / 'browser-temporary',
            *(run / (variant + suffix) for variant in ('baseline', 'wrong-fixture')
              for suffix in ('-profile', '-cache', '-crashes'))}
        assert required_directories <= directories, 'platform complete directory plan omitted'

    def plan(root, **values):
        assert root == ROOT
        files.update(values.get('output_files', ()))
        directories.update(values.get('directories', ()))
        sources.update(values.get('sources', ()))
        mutable.update(values.get('mutable_sources', ()))
        return original_plan(root, **values)

    def prepare(root, **values):
        complete()
        entered.append(True)
        return original_prepare(root, **values)

    profile.update(plan_producer=plan, prepare_producer=prepare)

    def direct():
        return sys._getframe(2).f_globals is scope

    def write(path, data, *args, **kwargs):
        if direct():
            complete()
            assert entered and path in files, 'platform write outside admitted plan'
            if path == wrong and path.exists():
                assert path in sources and path in mutable, 'platform copied fixture mutable plan omitted'
            writes.append(path)
        method = original[2] if isinstance(data, bytes) else original[3]
        return method(path, data, *args, **kwargs)

    def save_new(path, value):
        nonlocal published
        assert path in files, 'platform receipt outside admitted plan'
        if path == run / 'report.json':
            value = json.loads(value)
            value.update(scope='SIMULATED full platform effects;no native/browser numerical evidence',
                         simulated_children=True)
            published = value
            value = json.dumps(value, indent=2) + '\n'
        return original_save(path, value)

    class Process:
        def __init__(self, arguments, **kwargs):
            complete()
            assert entered
            environment = kwargs['env']
            PROFILE['verify'](ROOT, environment)
            assert environment['PLATFORM_RUNTIME_CALLER'] == 'retained'
            for name in ('CARGO_HOME', 'RUSTUP_HOME', 'CARGO_TARGET_DIR'):
                assert environment[name] == stores[name], 'platform caller store reset'
            assert all(environment[name] == str(temporary) for name in ('TMPDIR', 'MAKE_TMPDIR', 'TMP', 'TEMP'))
            assert kwargs == dict(cwd=ROOT, env=environment, stdout=subprocess.PIPE,
                                 stderr=subprocess.PIPE, text=True, start_new_session=True)
            self.pid = 2_000_000_000 + len(processes)
            processes[self.pid] = self
            self.returncode, self.calls = 0, 0
            self.stdout, self.stderr = '', ''
            self.arguments = arguments
            commands.append(arguments)
            if arguments == ['rustc', '-vV']:
                self.stdout = 'SIMULATED rustc identity\n'
            elif arguments == [str(ENTRY), '--version']:
                self.stdout = ('SIMULATED drifted Chrome identity\n' if failure == 'version-drift'
                               and len(commands) > 3 else 'SIMULATED Chrome identity\n')
            elif arguments[:2] == ['cargo', 'generate-lockfile']:
                assert arguments == ['cargo', 'generate-lockfile', '--offline', '--manifest-path', str(manifest)]
                original[3](workspace / 'Cargo.lock', FROZEN[ROOT / 'Cargo.lock'].decode() +
                            '\n[[package]]\nname = "sc-irrational-platform"\nversion = "0.0.0"\n')
            elif arguments[:2] == ['cargo', 'build']:
                expected = ['cargo', 'build', '--offline', '--locked', '--release', '--manifest-path',
                            str(manifest), '--target-dir', str(manifest.parent.parent / 'cargo-target'),
                            '--message-format=json']
                kind = 'wasm' if '--target' in arguments else 'native'
                assert arguments == expected + (['--target', 'wasm32-unknown-unknown'] if kind == 'wasm' else [])
                variant = 'baseline' if baseline in wrong.read_bytes() else 'wrong-fixture'
                artifact = base / 'artifacts' / (variant + '.' + kind)
                PROFILE['prepare_producer'](ROOT, directories=(artifact.parent,), output_files=(artifact,))
                original[2](artifact, b'SIMULATED artifact\n' + variant.encode() + kind.encode())
                self.stdout = '\n'.join(json.dumps(item) for item in (
                    dict(reason='build-finished', success=failure != 'compile-failure'),
                    dict(reason='compiler-artifact', target=dict(name='sc-irrational-platform'),
                         executable=str(artifact)))) + '\n'
                if failure == 'compile-failure':
                    self.returncode = 101
            elif arguments[:1] == ['bash']:
                assert arguments == ['bash', 'scripts/check_handoff.sh', '--idle-cua']
                self.stdout = 'handoff: OK\nSIMULATED process census;not a real handoff verdict\n'
                if failure == 'temp-replaced':
                    displaced = base / 'displaced-temporary'
                    PROFILE['plan_producer'](ROOT, directories=(displaced,))
                    temporary.rename(displaced)
                    temporary.mkdir()
            elif len(arguments) == 1:
                variant = Path(arguments[0]).stem
                assert variant in ('baseline', 'wrong-fixture') and Path(arguments[0]) == run / (variant + '.native')
                self.returncode = 104 if variant == 'baseline' else 101
                self.stderr = '' if variant == 'baseline' else 'SIMULATED panic degree_-720\n'
                outcomes.append(('native', variant))
            else:
                assert arguments[0] == str(ENTRY) and '--headless' in arguments
                assert '--user-data-dir=' + str(run / ('baseline-profile' if baseline in wrong.read_bytes()
                                                                      else 'wrong-fixture-profile')) in arguments
                variant = 'baseline' if baseline in wrong.read_bytes() else 'wrong-fixture'
                value = (dict(status='pass', phase='completed', marker=104, imports=[], userAgent='SIMULATED')
                         if variant == 'baseline' or failure == 'surviving-fixture' else
                         dict(status='trap', phase='executing', name='RuntimeError', message='SIMULATED trap'))
                self.stdout = '<pre id="result">' + json.dumps(value) + '</pre>'
                outcomes.append(('browser', variant))

        def communicate(self, timeout=None):
            self.calls += 1
            if self.arguments[:2] == ['cargo', 'build'] and self.calls == 1 and failure == 'interrupt':
                raise KeyboardInterrupt('controlled full-entry interruption')
            return self.stdout, self.stderr

    def launch(arguments, **kwargs):
        if arguments[:1] == ['git']:
            return original[0](arguments, **kwargs)
        return Process(arguments, **kwargs)

    def kill(pid, selected):
        assert pid in processes and selected == signal.SIGKILL
        killed.append(pid)

    try:
        subprocess.Popen, os.killpg = launch, kill
        Path.write_bytes, Path.write_text = write, write
        scope['save_new'] = save_new
        sys.argv = [str(ENTRY)]
        with contextlib.redirect_stdout(io.StringIO()):
            try:
                scope['main']()
            except (AssertionError, ValueError, OSError, KeyboardInterrupt) as caught:
                error = caught
                scope['write_failure'](caught)
        if diagnose:
            print('full-entry diagnostic:', type(error).__name__, str(error))
            return error
        if failure is None:
            assert error is None, error
            assert published and published['simulated_children'] and published['workspace_restored']
            assert outcomes == [('native', 'baseline'), ('browser', 'baseline'),
                                ('native', 'wrong-fixture'), ('browser', 'wrong-fixture')]
            assert not temporary.exists() and (run / 'browser-temporary').is_dir()
            pointer = json.loads((store / 'latest.json').read_text())
            assert pointer['sha256'] == hashlib.sha256((run / 'report.json').read_bytes()).hexdigest()
            assert not (store / 'pending.tmp').exists()
        elif failure == 'log-symlink':
            assert isinstance(error, ValueError) and str(error) == 'symlink producer output', error
            assert not entered and not commands and published is None
            assert sentinel.read_bytes() == b'foreign sentinel\n'
            assert not (run / 'failure.json').exists()
        else:
            assert error is not None and published is None and not (store / 'latest.json').exists()
            assert json.loads((run / 'failure.json').read_text())['status'] == 'incomplete'
            if failure == 'compile-failure':
                assert str(error) == 'compiler failure cannot count as runtime rejection'
            elif failure == 'surviving-fixture':
                assert isinstance(error, AssertionError)
            elif failure == 'interrupt':
                assert isinstance(error, KeyboardInterrupt)
            elif failure == 'version-drift':
                assert str(error) == 'tool version changed during capture'
            elif failure == 'temp-replaced':
                assert str(error) == 'owned temporary root changed before sealing'
        assert len(killed) == len(processes), 'platform owned child group cleanup omitted'
        if wrong.exists():
            assert wrong.read_bytes() == FROZEN[ROOT / 'crates/sc-core/tests/fixtures/formula_tangent.tsv']
        assert all(path.read_bytes() == data for path,data in FROZEN.items()), 'platform original source drift'
    finally:
        subprocess.Popen, os.killpg, Path.write_bytes, Path.write_text = original
        os.environ.clear();os.environ.update(saved)
        sys.argv = argv
        if wrong.is_file() and wrong.read_bytes() != FROZEN[ROOT / 'crates/sc-core/tests/fixtures/formula_tangent.tsv']:
            PROFILE['plan_producer'](ROOT, sources=(wrong,), mutable_sources=(wrong,), output_files=(wrong,))
            wrong.write_bytes(FROZEN[ROOT / 'crates/sc-core/tests/fixtures/formula_tangent.tsv'])
    return len(commands)


def main():
    if '--diagnose' in sys.argv:
        import re
        revision = sys.argv[sys.argv.index('--diagnose') + 1]
        assert re.fullmatch('[0-9a-f]{40}', revision), 'full diagnostic Git revision required'
        prior = subprocess.check_output(['git', 'show', revision + ':' + ENTRY.relative_to(ROOT).as_posix()],
            cwd=ROOT, env=ENV, text=True)
        error = exercise('before', script=prior, diagnose=True)
        assert str(error) == 'effective MAKE_TMPDIR is not local'
        return
    exercise('complete')
    for failure in ('compile-failure', 'surviving-fixture', 'interrupt', 'version-drift',
                    'temp-replaced', 'log-symlink'):
        exercise(failure, failure=failure)
    faults = (
        ('temp-environment', "    environment['MAKE_TMPDIR'] = str(TEMP)\n", ''),
        ('fixture-mutable', 'mutable_sources=(wrong,), ', ''),
        ('file-plan', 'output_files=outputs)', 'output_files=())'),
        ('source-plan', 'sources=tuple(sources)', 'sources=()'),
        ('restore', '        wrong.write_bytes(original)\n', '        pass\n'),
        ('session', 'start_new_session=True', 'start_new_session=False'),
    )
    for label, old, new in faults:
        assert old in SCRIPT, label
        try:
            exercise('omission-' + label, script=SCRIPT.replace(old, new))
        except AssertionError as error:
            expected = {
                'temp-environment': 'effective MAKE_TMPDIR is not local',
                'fixture-mutable': 'platform copied fixture mutable plan omitted',
                'file-plan': 'platform complete output plan omitted',
                'source-plan': 'platform source plan omitted',
            }
            if label in expected:
                assert expected[label] in str(error), (label, error)
        else:
            raise AssertionError('actual full-platform body omission survived: ' + label)
    print('platform runtime:actual full-entry success/6 failure paths/6 actual body omissions;'
          'complete plans/caller stores/argv/source+fixture restoration/temp seal/latest;'
          'SIMULATED compiler/browser;no numerical verdict;rc=0')


if __name__ == '__main__':
    main()
