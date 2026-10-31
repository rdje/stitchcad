"""Controller effect controls with simulated compiler children; genuine native proof is separate."""
from pathlib import Path
import contextlib
import hashlib
import io
import json
import os
import re
import runpy
import signal
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
HERE = ROOT / 'docs/tasks/artifacts/formula_structure'
DRIVER = HERE / 'root_body_faults.py'
ENTRIES = tuple(HERE / (name + '_body_faults.py') for name in ('root', 'arc', 'forward', 'inverse'))
TANGENT_ENTRIES = (HERE / 'tangent_body_faults.py', HERE / 'tangent_resource_body_faults.py')
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
CORE = (Path(__file__), PROFILE_SOURCE, *ENTRIES, *TANGENT_ENTRIES)
WORK = ROOT / 'target/numerical-runtime-controls' / str(os.getpid())
PROFILE['prepare_producer'](ROOT, sources=CORE, directories=(WORK,))
GIT_ENV = {key: value for key, value in os.environ.items() if not key.startswith('GIT_')}
GIT_ENV.update(GIT_OPTIONAL_LOCKS='0', GIT_CONFIG_NOSYSTEM='1', GIT_CONFIG_GLOBAL=os.devnull)
TRACKED = subprocess.check_output(['git', 'ls-files', 'crates'], cwd=ROOT, env=GIT_ENV, text=True).splitlines()
SNAPSHOT = {ROOT / path for path in TRACKED} | {ROOT / path for path in ('Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml')}
PROFILE['plan_producer'](ROOT, sources=(*CORE, *SNAPSHOT))
FROZEN = {path: path.read_bytes() for path in {*CORE, *SNAPSHOT}}
WORKSPACES = set()


def module(path, script=None):
    ns = dict(__file__=str(path), __name__='numerical_runtime_control')
    exec(compile(path.read_text() if script is None else script, str(path), 'exec'), ns)
    return ns


def public_configurations(entry, script=None):
    """Observe actual wrapper dispatch rather than reconstructing its family configuration."""
    cases = []
    original_loader = runpy.run_path

    def load(path, *args, **kwargs):
        namespace = original_loader(path, *args, **kwargs)
        if Path(path) == DRIVER:
            namespace['main'] = lambda config: cases.append(dict(config))
        return namespace

    try:
        runpy.run_path = load
        scope = dict(__file__=str(entry), __name__='__main__')
        exec(compile(entry.read_text() if script is None else script, str(entry), 'exec'), scope)
    finally:
        runpy.run_path = original_loader
    return cases


def tangent_configurations(entry, script=None):
    cases = public_configurations(entry, script)
    expected = {
        TANGENT_ENTRIES[0]: (
            ('tangent-fold', 'prepare.rs', 139, 3),
            ('tangent-interval', 'interval.rs', 139, 3),
            ('tangent-execution', 'execute.rs', 139, 3)),
        TANGENT_ENTRIES[1]: (
            ('tangent-resource-prepare', 'prepare.rs', 33, 6),
            ('tangent-resource-execute', 'execute.rs', 33, 7)),
    }[entry]
    observed = [(case['family'], case['source'].name, case['rows'], len(case['faults'])) for case in cases]
    assert observed == list(expected), 'tangent public family dispatch drift'
    for case in cases:
        assert case['entry'] == entry and case['source'].parent == ROOT / 'crates/sc-core/src/recipe/tangent'
        resource = entry == TANGENT_ENTRIES[1]
        fixture, target, test = (
            ('formula_tangent_resources.tsv', 'formula_tangent_resource_contract',
             'independent_resource_rows_preserve_actual_stages_limits_demands_and_prior_progress') if resource else
            ('formula_tangent.tsv', 'formula_tangent_oracle_contract',
             'actual_upstream_sources_preserve_all_original_tangent_outcomes_through_signed_storage'))
        assert case['fixture'] == ROOT / 'crates/sc-core/tests/fixtures' / fixture
        assert (case['target'], case['test']) == (target, test), 'tangent original consumer binding drift'
    return cases


def configuration(entry, label, script=None, *, family=None):
    if family is not None:
        cases = tangent_configurations(entry)
        config = next(case for case in cases if case['family'] == family)
        ns = module(DRIVER, script)
    elif entry == DRIVER:
        ns = module(entry, script)
        config = dict(entry=entry, source=ns['SOURCE'], fixture=ns['FIXTURE'], run=ns['RUN'],
                      faults=ns['FAULTS'], test=ns['TEST'], target='formula_root_value_contract', rows=85, family='root')
    else:
        ns = module(entry)
        config = dict(ns['CONFIG'])
        ns = ns['CONTROLLER']['main'].__globals__
    # Only this borrowed provenance label changes; fixture/source/test/fault populations stay exact.
    config['run'] = WORK / label / config['family']
    return ns, config


def capture(entry, label, script=None, *, outcome='success', family=None):
    ns, config = configuration(entry, label, script, family=family)
    original_profile = runpy.run_path
    original_writer = ns['write_report']
    original = (Path.write_bytes, Path.write_text, Path.read_bytes, Path.mkdir,
                subprocess.Popen, os.killpg)
    saved, argv = dict(os.environ), sys.argv
    stores = {key: str(WORK / 'caller-stores' / key.lower()) for key in PROFILE['STORES']}
    PROFILE['prepare_producer'](ROOT, sources=CORE, directories=tuple(Path(value) for value in stores.values()))
    os.environ.update(stores, NUMERICAL_RUNTIME_CALLER='retained')
    configured = dict(os.environ)
    expected_sources = {*SNAPSHOT, DRIVER, entry, PROFILE_SOURCE, config['source'], config['fixture']}
    original_body = FROZEN[config['source']]
    rows = FROZEN[config['fixture']].decode().splitlines()[1:]
    labels = ('baseline', *(name for name, _, _ in config['faults']))
    declared_files, declared_sources, declared_dirs, mutable = set(), set(), set(), set()
    paths, events, processes, killed, links = {}, [], {}, [], []
    expected_failure = None
    native_calls = 0
    restoration_ok = False
    report = None

    def selected_paths():
        if not paths or 'copy' in paths:
            return
        run = paths['run']
        paths['copy'] = run / 'workspace'
        paths['body'] = paths['copy'] / config['source'].relative_to(ROOT)
        paths['report'] = run / 'report.json'
        paths['destinations'] = {paths['copy'] / file.relative_to(ROOT) for file in SNAPSHOT}
        paths['logs'] = {run / (name + '.log') for name in labels}
        paths['controls'] = {run / 'capture-controls' / name / 'report.json' for name in (
            'guarded', 'reuse-omitted', 'exclusive-omitted')}
        paths['files'] = paths['destinations'] | paths['logs'] | paths['controls'] | {
            paths['report'], run / 'timeout-parent-only.txt', run / 'timeout-group.txt'}

    def complete():
        selected_paths()
        assert paths and expected_sources <= declared_sources, 'numerical original input plan omitted'
        assert paths['files'] <= declared_files, 'numerical complete file plan omitted'
        assert paths['copy'] in declared_dirs and paths['run'] in declared_dirs

    def load(path, *args, **kwargs):
        result = original_profile(path, *args, **kwargs)
        if Path(path) != PROFILE_SOURCE:
            return result
        real_plan, real_enter = result['plan_producer'], result['enter_producer']

        def plan(root, **values):
            assert root == ROOT
            for folder in values.get('directories', ()):
                if folder.name == 'workspace':
                    assert not paths or paths['run'] == folder.parent
                    paths['run'] = folder.parent
            declared_sources.update(values.get('sources', ()))
            declared_files.update(values.get('output_files', ()))
            declared_dirs.update(values.get('directories', ()))
            mutable.update(values.get('mutable_sources', ()))
            if outcome == 'late-log-refusal' and native_calls > 1 and paths:
                outputs = set(values.get('output_files', ()))
                if len(outputs) == 1 and outputs <= paths['logs']:
                    raise result['Refusal']('controlled late log refusal')
            return real_plan(root, **values)

        def enter(root, **values):
            complete()
            events.append('activation')
            return real_enter(root, **values)

        result.update(plan_producer=plan, enter_producer=enter)
        return result

    def direct():
        return sys._getframe(2).f_globals is ns

    def mkdir(path, *args, **kwargs):
        if direct():
            complete()
            assert 'activation' in events, 'numerical capture creation before activation boundary'
            assert path in declared_dirs, 'numerical unplanned direct directory'
            events.append(('directory', path))
        return original[3](path, *args, **kwargs)

    def write(path, data, *args, **kwargs):
        selected = direct()
        if selected:
            complete()
            assert path in declared_files, 'numerical unplanned direct write'
            if path == paths['body'] and path.exists():
                assert path in mutable, 'numerical copied-body mutation declaration omitted'
            events.append(('write', path))
        writer = original[0] if isinstance(data, bytes) else original[1]
        result = writer(path, data, *args, **kwargs)
        if selected and outcome == 'body-hardlink' and path == paths['body'] and not links:
            alias = WORK / (label + '-body-alias')
            PROFILE['plan_producer'](ROOT, sources=(*CORE, path), output_files=(alias,))
            os.link(path, alias)
            links.append(alias)
        return result

    def read(path, *args, **kwargs):
        if direct():
            assert path in declared_sources or path in declared_files, \
                'numerical read lacks a source/output plan: ' + str(path.relative_to(ROOT))
        return original[2](path, *args, **kwargs)

    class Process:
        def __init__(self, command, **kwargs):
            nonlocal native_calls
            complete()
            assert kwargs == dict(cwd=ROOT, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                                 text=True, start_new_session=True), 'numerical native process contract changed'
            assert all(os.environ.get(key) == value for key, value in stores.items()), 'numerical caller stores reset'
            assert os.environ['NUMERICAL_RUNTIME_CALLER'] == 'retained'
            PROFILE['verify'](ROOT, os.environ)
            self.pid = 2_000_000_000 + len(processes)
            self.calls, self.returncode = 0, 0
            self.command = command
            self.stdout, self.stderr = '', ''
            processes[self.pid] = self
            if command == ['rustc', '-vV']:
                self.stdout = 'SIMULATED compiler identity;dispatch control only\n'
                return
            assert command == ['cargo', 'test', '--offline', '--locked', '--manifest-path',
                str(paths['copy'] / 'Cargo.toml'), '-p', 'sc-core', '--test', config['target'],
                '--message-format=json', config['test'], '--', '--nocapture']
            index = native_calls
            native_calls += 1
            name = labels[index]
            expected = original_body
            if index:
                _, before, after = config['faults'][index - 1]
                expected = original_body.decode().replace(before, after, 1).encode()
            assert original[2](paths['body']) == expected, 'numerical wrong actual copied body'
            assert {file for file in paths['copy'].rglob('*') if file.is_file()} == paths['destinations']
            artifact = Path(stores['CARGO_TARGET_DIR']) / 'SIMULATED-artifacts' / label / (config['family'] + '-' + name)
            PROFILE['prepare_producer'](ROOT, sources=CORE, directories=(artifact.parent,), output_files=(artifact,))
            original[0](artifact, b'SIMULATED child artifact\n' + hashlib.sha256(expected).digest())
            messages = [dict(reason='build-finished', success=outcome != 'compile-failure'),
                        dict(reason='compiler-artifact', target=dict(name=config['target']), executable=str(artifact))]
            if outcome == 'artifact-outside':
                messages[1]['executable'] = str(ROOT.parent / 'unowned-numerical-artifact')
            self.stdout = '\n'.join(json.dumps(item) for item in messages) + '\nrunning 1 test\n' + config['test'] + '\n'
            if index and outcome != 'surviving-fault':
                self.returncode = 101
                self.stdout += 'test result: FAILED. 0 passed; 1 failed\n'
                self.stderr = ("thread '" + config['test'] + "' panicked at SIMULATED.rs:1:1:\n"
                               'assertion failed: fixture-row=1;' + rows[0] + '\n')
            else:
                self.stdout += 'test result: ok. 1 passed; 0 failed\n'
            if outcome == 'compile-failure':
                self.returncode = 101

        def communicate(self, timeout=None):
            self.calls += 1
            if self.command[:1] == ['cargo'] and native_calls > 1 and self.calls == 1:
                if outcome == 'timeout':
                    raise subprocess.TimeoutExpired(self.command, timeout)
                if outcome == 'interruption':
                    raise KeyboardInterrupt('controlled native interruption')
            return self.stdout, self.stderr

    def process(command, **kwargs):
        if command[:1] == ['git']:
            assert command == ['git', 'ls-files', 'crates'] and kwargs['cwd'] == ROOT
            assert not set(kwargs['env']) & {'GIT_DIR', 'GIT_INDEX_FILE', 'GIT_WORK_TREE'}
            return original[4](command, **kwargs)
        if command[:1] == [sys.executable]:
            complete()
            assert command[1:4] == ['-I', '-B', '-c']
            assert paths['run'].relative_to(ROOT).as_posix() in command[4]
            assert kwargs['start_new_session'], 'numerical timeout child has no owned session'
            return original[4](command, **kwargs)
        return Process(command, **kwargs)

    def killpg(pid, selected_signal):
        if pid in processes:
            assert selected_signal == signal.SIGKILL
            killed.append(pid)
            return
        return original[5](pid, selected_signal)

    def publish(path, value, owner):
        nonlocal report
        complete()
        assert path in declared_files
        if path != paths['report']:
            return original_writer(path, value, owner)
        # Retained fixtures must never masquerade as compiled numerical evidence.
        value = dict(value, scope='SIMULATED compiler dispatch control;no native numerical evidence', simulated_children=True)
        original_writer(path, value, owner)
        report = value

    try:
        runpy.run_path = load
        ns['write_report'] = publish
        Path.write_bytes, Path.write_text, Path.read_bytes, Path.mkdir = write, write, read, mkdir
        subprocess.Popen, os.killpg = process, killpg
        sys.argv = [str(entry)]
        with contextlib.redirect_stdout(io.StringIO()):
            try:
                ns['main'](config)
            except (AssertionError, ValueError, subprocess.TimeoutExpired, KeyboardInterrupt) as error:
                if outcome == 'success':
                    raise
                expected_failure = error
        if outcome == 'success':
            assert report and report['simulated_children'] and len(report['outcomes']) == len(labels)
            assert [item['label'] for item in report['outcomes']] == list(labels)
            assert [item['cargo_rc'] for item in report['outcomes']] == [0, *([101] * (len(labels) - 1))]
            assert report['timeout_control'] == dict(parent_only_child_wrote=True, guarded_group_child_write_absent=True)
        else:
            assert expected_failure is not None and report is None, 'numerical failed run published acceptance'
            if outcome == 'timeout':
                assert isinstance(expected_failure, subprocess.TimeoutExpired) and killed
            if outcome == 'interruption':
                assert isinstance(expected_failure, KeyboardInterrupt) and killed
            if outcome == 'late-log-refusal':
                assert str(expected_failure) == 'controlled late log refusal'
            if outcome == 'compile-failure':
                expected = 'compiler failure is not an actual ' + config['family'] + ' fault rejection'
                assert expected in str(expected_failure), expected_failure
            if outcome == 'surviving-fault':
                assert 'actual compiled native fault survived' in str(expected_failure)
            if outcome == 'body-hardlink':
                assert 'hard-linked' in str(expected_failure)
            if outcome == 'artifact-outside':
                assert 'producer path outside workspace' in str(expected_failure)
        restoration_ok = bool(paths) and all(original[2](paths['copy'] / path.relative_to(ROOT)) == value
                                             for path, value in FROZEN.items() if path in SNAPSHOT)
        assert restoration_ok, 'numerical complete snapshot restoration omitted'
        assert {path for path in paths['copy'].rglob('*') if path.is_file()} == paths['destinations']
        assert dict(os.environ) == dict(configured, MAKE_TMPDIR=stores['TMPDIR'], RUSTUP_AUTO_INSTALL='0')
    finally:
        ns['write_report'] = original_writer
        runpy.run_path = original_profile
        Path.write_bytes, Path.write_text, Path.read_bytes, Path.mkdir, subprocess.Popen, os.killpg = original
        sys.argv = argv
        os.environ.clear();os.environ.update(saved)
        for alias in links:
            assert alias.samefile(paths['body'])
            alias.unlink()
        if paths and paths.get('body') and paths['body'].exists() and paths['body'].read_bytes() != original_body:
            PROFILE['plan_producer'](ROOT, sources=(paths['body'],), mutable_sources=(paths['body'],), output_files=(paths['body'],))
            paths['body'].write_bytes(original_body)
        if paths and paths.get('copy') and paths['copy'].exists():
            WORKSPACES.add(paths['copy'])
    assert all(path.read_bytes() == data for path, data in FROZEN.items()), 'numerical original sources changed'
    return len(labels), len(paths['destinations'])


def preflight(label, script=None):
    """Calibrate real aliases/types and controlled device/marker observations before activation."""
    ns, config = configuration(DRIVER, 'preflight-' + label, script)
    folder = WORK / 'preflight-fixtures' / label
    kernel, fixture = folder / 'kernel.rs', folder / 'fixture.tsv'
    PROFILE['prepare_producer'](ROOT, sources=CORE, directories=(folder,), output_files=(kernel, fixture))
    kernel.write_bytes(FROZEN[config['source']])
    fixture.write_bytes(FROZEN[config['fixture']])
    if label == 'source-link':
        kernel.unlink();kernel.symlink_to(config['source']);config['source'] = kernel
    elif label == 'source-missing':
        config['source'] = folder / 'absent.rs'
    elif label == 'source-directory':
        config['source'] = folder
    elif label in ('source-device', 'untracked-kernel'):
        config['source'] = kernel
    elif label == 'source-marker':
        config['source'] = kernel
    elif label == 'source-outside':
        config['source'] = ROOT.parent / 'unowned-kernel.rs'
    elif label == 'untracked-fixture':
        config['fixture'] = fixture
    elif label == 'duplicate-baseline':
        _, before, after = config['faults'][0]
        config['faults'] = (('baseline', before, after),)
    elif label == 'duplicate-fault':
        config['faults'] = (config['faults'][0], config['faults'][0])
    elif label == 'run-outside':
        config['run'] = ROOT.parent / 'unowned-capture'
    elif label == 'capture-parent-link':
        relative = config['run'].relative_to(ROOT / 'target')
        parent = ROOT / 'target/irrational_platform_certificate/native-body-faults' / relative.parent
        PROFILE['prepare_producer'](ROOT, sources=CORE, directories=(parent.parent,), output_files=(parent,))
        parent.symlink_to(folder, target_is_directory=True)
    else:
        raise AssertionError('unknown preflight fixture')
    real_load, stat, exists = runpy.run_path, Path.stat, Path.exists
    originals = subprocess.Popen, Path.write_bytes, Path.write_text, os.mkdir
    saved, argv = dict(os.environ), sys.argv
    entered = []

    def load(path, *args, **kwargs):
        result = real_load(path, *args, **kwargs)
        if Path(path) == PROFILE_SOURCE:
            def activation(*args, **kwargs):
                entered.append(True)
                raise AssertionError('invalid configuration reached activation')
            result['enter_producer'] = activation
        return result

    def observed(path, *args, **kwargs):
        value = stat(path, *args, **kwargs)
        if label == 'source-device' and path == kernel:
            fields = list(value);fields[2] += 1
            return os.stat_result(fields)
        return value

    def observed_exists(path, *args, **kwargs):
        return True if label == 'source-marker' and path == folder / '.git' else exists(path, *args, **kwargs)

    def process(command, **kwargs):
        assert command == ['git', 'ls-files', 'crates'], 'preflight reached a native child'
        return originals[0](command, **kwargs)

    def effect(*args, **kwargs):
        raise AssertionError('preflight configuration produced an effect')

    try:
        runpy.run_path, Path.stat, Path.exists = load, observed, observed_exists
        subprocess.Popen, Path.write_bytes, Path.write_text, os.mkdir = process, effect, effect, effect
        sys.argv = [str(DRIVER)]
        with contextlib.redirect_stdout(io.StringIO()):
            try:
                ns['main'](config)
            except (AssertionError, ValueError) as error:
                assert not entered, error
                if label.startswith('duplicate'):
                    assert str(error) == 'distinct baseline and fault labels required'
                elif label.startswith('untracked'):
                    assert str(error) == 'selected kernel and fixture must belong to the copied workspace'
                else:
                    expected = {
                        'source-link': 'symlink producer source',
                        'source-missing': 'producer source is not a local regular file',
                        'source-directory': 'producer source is not a local regular file',
                        'source-device': 'producer source is not a local regular file',
                        'source-marker': 'store crosses another Git repository',
                        'source-outside': 'producer path outside workspace',
                        'run-outside': 'producer path outside workspace',
                        'capture-parent-link': 'symlink store component',
                    }
                    assert str(error) == expected[label], error
            else:
                raise AssertionError('invalid numerical configuration accepted: ' + label)
        assert dict(os.environ) == saved
    finally:
        runpy.run_path, Path.stat, Path.exists = real_load, stat, exists
        subprocess.Popen, Path.write_bytes, Path.write_text, os.mkdir = originals
        sys.argv = argv
        os.environ.clear();os.environ.update(saved)


def faults():
    source = DRIVER.read_text()
    variants = (
        ('file-plan', source.replace('output_files=outputs)', 'output_files=())')),
        ('source-plan', source.replace('sources=sources,', 'sources=(),')),
        ('mutable-plan', source.replace('mutable_sources=(body,),', '')),
        ('activation', re.sub(r"(?m)^    profile\['enter_producer'\]\(ROOT, sources=sources, directories=.*\n", '', source)),
        ('session', source.replace('start_new_session=True', 'start_new_session=False')),
        ('restoration', source.replace('        body.write_bytes(original)\n', '        pass\n')),
    )
    for label, variant in variants:
        assert variant != source
        try:
            capture(DRIVER, 'fault-' + label, variant)
        except AssertionError:
            pass
        else:
            raise AssertionError('actual numerical runtime fault survived: ' + label)
    membership = "    assert kernel_source in snapshot and fixture_path in snapshot, \\\n        'selected kernel and fixture must belong to the copied workspace'\n"
    labels = "    assert len(labels) == len(set(labels)), 'distinct baseline and fault labels required'\n"
    for label, old, fixture in (('membership', membership, 'untracked-kernel'), ('labels', labels, 'duplicate-baseline')):
        assert source.count(old) == 1, label
        try:
            preflight(fixture, source.replace(old, ''))
        except AssertionError:
            pass
        else:
            raise AssertionError('actual numerical preflight fault survived: ' + label)
    return len(variants) + 2


def tangent_controls():
    results = []
    for entry in TANGENT_ENTRIES:
        for case in tangent_configurations(entry):
            family = case['family']
            results.append(capture(entry, 'complete-' + family, family=family))
    for entry, family, outcome in (
        (TANGENT_ENTRIES[0], 'tangent-execution', 'compile-failure'),
        (TANGENT_ENTRIES[1], 'tangent-resource-execute', 'late-log-refusal')):
        capture(entry, family + '-' + outcome, family=family, outcome=outcome)
    rejected = 0
    for entry, old, new in (
        (TANGENT_ENTRIES[0], "for family, source, faults in FAMILIES:",
         "for family, source, faults in FAMILIES[:1]:"),
        (TANGENT_ENTRIES[1], "rows=33,", "rows=32,"),
        (TANGENT_ENTRIES[1], "target='formula_tangent_resource_contract'",
         "target='unrelated_contract'")):
        source = entry.read_text()
        assert source.count(old) == 1
        try:
            tangent_configurations(entry, source.replace(old, new))
        except AssertionError as error:
            assert str(error) in ('tangent public family dispatch drift', 'tangent original consumer binding drift')
            rejected += 1
        else:
            raise AssertionError('actual tangent dispatch omission survived')
    assert all(path.read_bytes() == data for path, data in FROZEN.items())
    print('tangent runtime:5 actual public configurations/27 simulated variants/'
          '2 failure paths/3 actual wrapper body omissions;complete source+snapshot restoration;'
          'native numerical evidence separate;rc=0')
    return results


def main():
    if '--tangent-only' in sys.argv:
        tangent_controls()
        return
    tangent_controls()
    results = [capture(entry, 'complete-' + entry.stem) for entry in ENTRIES]
    outcomes = ('compile-failure', 'surviving-fault', 'timeout', 'interruption', 'late-log-refusal',
                'body-hardlink', 'artifact-outside')
    for outcome in outcomes:
        capture(DRIVER, outcome, outcome=outcome)
    capture(ENTRIES[1], 'arc-compile-failure', outcome='compile-failure')
    refusals = ('source-link', 'source-missing', 'source-directory', 'source-device', 'source-marker',
                'source-outside', 'untracked-kernel', 'untracked-fixture', 'duplicate-baseline',
                'duplicate-fault', 'run-outside', 'capture-parent-link')
    for label in refusals:
        preflight(label)
    rejected = faults()
    assert all(path.read_bytes() == data for path, data in FROZEN.items())
    for workspace in WORKSPACES:
        for path in workspace.rglob('*'):
            if path.is_file():
                source = ROOT / path.relative_to(workspace)
                assert source in SNAPSHOT and path.read_bytes() == FROZEN[source], \
                    'owned fault fixture was not restored independently'
    print('numerical runtime captures:4 borrowed output labels/all original source/test/fault contracts;'
          f'{sum(count for count, _ in results)} simulated variants;{len(outcomes)} failure paths;'
          f'{len(refusals)} preflight refusals/{rejected} actual body faults;'
          'actual timeout-parent/group controls;full snapshot/source restoration;native evidence separate;rc=0')


if __name__ == '__main__':
    main()
