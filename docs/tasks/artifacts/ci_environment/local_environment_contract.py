"""D156: capture real Make producer environments without invoking a compiler or shared store."""
from pathlib import Path
import json
import contextlib
import io
import os
import shutil
import subprocess
import sys
import types

ROOT = Path(__file__).resolve().parents[4]
WORK = ROOT / 'target/scratch/local_environment_contract'
EXPECTED = {'CARGO_HOME': 'target/cargo-home', 'RUSTUP_HOME': 'target/cargo-home/rustup-ci',
            'CARGO_TARGET_DIR': 'target', 'TMPDIR': 'target/scratch'}
SOURCE = ROOT / 'scripts/local_environment.py'


def load(text):
    scope = {'__file__': str(SOURCE), '__name__': 'local_environment_probe'}
    exec(compile(text, str(SOURCE), 'exec'), scope)
    return scope


def controls(scope):
    cleanup()
    WORK.mkdir(parents=True)
    fixture = WORK / 'workspace'
    fixture.mkdir()
    prepare, verify, refusal = scope['environment_for'], scope['verify'], scope['Refusal']
    wanted = {name: str(fixture / path) for name, path in EXPECTED.items()}
    result = prepare(fixture, {'RETAINED': 'opaque value', 'RUSTUP_TOOLCHAIN': '1.99.0'})
    assert all(result[name] == value for name, value in wanted.items()), 'D156 store identity'
    assert result['RETAINED'] == 'opaque value' and result['RUSTUP_TOOLCHAIN'] == '1.99.0', 'D156 unrelated environment'
    assert result['RUSTUP_AUTO_INSTALL'] == '0', 'D156 implicit install'
    verify(fixture, result)
    assert result['MAKE_TMPDIR'] == wanted['TMPDIR'], 'D156 Make temp identity'
    count = 5

    def reject(action, message):
        nonlocal count
        try:
            action()
        except refusal as error:
            assert str(error) == message, ('D156 refusal identity', message, str(error))
        else:
            raise AssertionError('D156 guard accepted: ' + message)
        count += 1

    for relative in ('target/../escape', 'target/.git/store'):
        reject(lambda: scope['directory'](fixture, relative, missing_ok=True), 'invalid store components')
        reject(lambda: scope['select'](fixture, 'TMPDIR', relative), 'invalid store components')
    for name in EXPECTED:
        external = prepare(fixture, {name: str(ROOT.parent)}, create=False)
        assert external[name] == wanted[name], 'D156 ambient external store'
        empty = prepare(fixture, {name: ''}, create=False)
        assert empty[name] == wanted[name], 'D156 empty store'
        custom = 'target/custom store/' + name
        for value in (custom, str(fixture / custom)):
            configured = prepare(fixture, {name: value})
            assert configured[name] == str(fixture / custom), 'D156 local override'
            verify(fixture, configured)
            count += 1
        for value in ('target/../escape', 'target/.git/store'):
            reject(lambda: prepare(fixture, {name: value}), 'invalid store components')
        reject(lambda: prepare(fixture, {name: 'crates/store'}), 'local store must be under target')
        reject(lambda: prepare(fixture, {name: 'target/store\n'}), 'invalid ' + name + ' value')
        bad = dict(result)
        bad.pop(name)
        reject(lambda: verify(fixture, bad), 'effective ' + name + ' is not local')
        count += 2
    reject(lambda: prepare(fixture, {'RUSTUP_TOOLCHAIN': '/external/toolchain'}), 'toolchain override must be a name')
    reject(lambda: prepare(ROOT.parent, {}), 'workspace outside project target')
    scratch = fixture / 'target/scratch'
    scratch.rmdir()
    reject(lambda: verify(fixture, result), 'missing store directory')
    scratch.write_text('fixture-owned ordinary file')
    reject(lambda: prepare(fixture, result, create=False), 'store component is not a directory')
    scratch.unlink()
    scratch.symlink_to(WORK)
    try:
        reject(lambda: prepare(fixture, result, create=False), 'symlink store component')
    finally:
        scratch.unlink()
    scratch.mkdir()
    marker = scratch / '.git'
    for dangling in (False, True):
        if dangling:
            marker.symlink_to(scratch / 'absent-boundary')
        else:
            marker.write_text('fixture boundary marker; no Git repository')
        try:
            reject(lambda: prepare(fixture, result, create=False), 'store crosses another Git repository')
        finally:
            marker.unlink()
    original_stat = Path.stat

    def changed_device(path, *args, **kwargs):
        metadata = original_stat(path, *args, **kwargs)
        if path == scratch:
            return types.SimpleNamespace(st_mode=metadata.st_mode, st_dev=metadata.st_dev + 1)
        return metadata

    Path.stat = changed_device
    try:
        reject(lambda: prepare(fixture, result, create=False), 'store is on another volume')
    finally:
        Path.stat = original_stat
    bad = dict(result, RUSTUP_AUTO_INSTALL='1', LOCAL_ENVIRONMENT_READY='1')
    reject(lambda: verify(fixture, bad), 'effective RUSTUP_AUTO_INSTALL is not local')
    bad = dict(result, MAKE_TMPDIR=str(ROOT.parent))
    reject(lambda: verify(fixture, bad), 'effective MAKE_TMPDIR is not local')
    # A late invalid store must refuse before any earlier missing store is created.
    late = WORK / 'late'
    (late / 'target/boundary').mkdir(parents=True)
    marker = late / 'target/boundary/.git'
    marker.write_text('fixture boundary marker; no Git repository')
    try:
        reject(lambda: prepare(late, {'TMPDIR': 'target/boundary/new'}), 'store crosses another Git repository')
        assert not (late / 'target/cargo-home').exists(), 'D156 refusal created earlier stores'
    finally:
        marker.unlink()
    return count


def cleanup():
    if WORK.exists():
        assert not WORK.is_symlink() and WORK.stat().st_dev == ROOT.stat().st_dev
        assert not list(WORK.rglob('.git')), 'fixture cleanup crosses a Git boundary'
        shutil.rmtree(WORK)


def make_controls(make_text=None):
    cleanup()
    WORK.mkdir(parents=True)
    # Test inner recipes with Make's own startup temp already local. The independent outer-launcher
    # controls below test absent exports before Make starts (including macOS platform dispatch).
    make = ['make', '--no-print-directory']
    if make_text is not None:
        makefile = WORK / 'Makefile'
        makefile.write_text(make_text)
        make += ['-f', str(makefile)]
    binary = WORK / 'bin'
    binary.mkdir()
    output = WORK / 'captured.jsonl'
    capture = '''#!/usr/bin/env python3
import json, os, pathlib, sys
p = pathlib.Path(os.environ['D156_CAPTURE'])
with p.open('a') as f:
    f.write(json.dumps({'command': pathlib.Path(sys.argv[0]).name, 'args': sys.argv[1:],
                       'stores': {k: os.environ.get(k) for k in ('CARGO_HOME', 'RUSTUP_HOME', 'CARGO_TARGET_DIR', 'TMPDIR')},
                       'toolchain': os.environ.get('RUSTUP_TOOLCHAIN'),
                       'auto_install': os.environ.get('RUSTUP_AUTO_INSTALL')}) + '\\n')
'''
    for name in ('cargo', 'rustup', 'mdbook'):
        path = binary / name
        path.write_text(capture)
        path.chmod(0o755)
    environment = dict(os.environ)
    for name in (*EXPECTED, 'RUSTUP_TOOLCHAIN', 'RUSTUP_AUTO_INSTALL'):
        environment.pop(name, None)
    environment.update(PATH=str(binary) + os.pathsep + environment['PATH'],
                       D156_CAPTURE=str(output), TMPDIR=str(ROOT / 'target/scratch'),
                       MAKE_TMPDIR=str(ROOT / 'target/scratch'))
    result = subprocess.run([*make, 'check'], cwd=ROOT,
                            env=environment, capture_output=True, text=True)
    assert result.returncode == 0, ('D156 Make capture failed', result.returncode, result.stderr)
    rows = [json.loads(line) for line in output.read_text().splitlines()]
    assert [row['args'] for row in rows] == [
        ['fmt', '--all', '--', '--check'],
        ['clippy', '--all-targets', '--all-features', '--', '-D', 'warnings'],
        ['test', '--all']], ('D156 actual native command identity/order', rows)
    wanted = {key: str(ROOT / relative) for key, relative in EXPECTED.items()}
    for row in rows:
        assert row['stores'] == wanted, ('D156 actual Make effective stores',
                                        {key: value == wanted[key] for key, value in row['stores'].items()})
        assert row['toolchain'] is None, 'D156 changed stable-file toolchain selection'
        assert row['auto_install'] == '0', 'D156 implicit toolchain install not disabled'
    print('local environment: actual Make fmt/Clippy/test arguments and all four default stores; no compiler/shared-store write')
    output.unlink()
    result = subprocess.run([*make, 'toolchain'], cwd=ROOT,
                            env=environment, capture_output=True, text=True)
    assert result.returncode == 0, ('D156 toolchain capture failed', result.returncode, result.stderr)
    rows = [json.loads(line) for line in output.read_text().splitlines()]
    assert len(rows) == 1 and rows[0]['command'] == 'rustup', 'D156 toolchain action identity'
    assert rows[0]['args'] == ['toolchain', 'install', '--profile', 'minimal', '--component', 'rustfmt',
                              '--component', 'clippy', '--target', 'wasm32-unknown-unknown',
                              '--no-self-update'], ('D156 toolchain flags or channel changed', rows)
    assert rows[0]['stores'] == wanted and rows[0]['auto_install'] == '0'
    print('local environment: actual active-channel setup/no-self-update/component/target/profile arguments')

    output.unlink()
    environment['RUSTUP_TOOLCHAIN'] = '1.99.0'
    result = subprocess.run([*make, 'wasm', 'book'], cwd=ROOT,
                            env=environment, capture_output=True, text=True)
    assert result.returncode == 0, ('D156 WASM/book capture failed', result.stderr)
    rows = [json.loads(line) for line in output.read_text().splitlines()]
    assert [row['args'] for row in rows] == [
        ['build', '--target', 'wasm32-unknown-unknown', '-p', 'sc-units', '-p', 'sc-core', '-p', 'sc-measure'],
        ['build', 'docs/book']], 'D156 WASM/book argument identity'
    assert all(row['stores'] == wanted and row['toolchain'] == '1.99.0' for row in rows), 'D156 named toolchain preserved'
    print('local environment: actual Make WASM/book commands and named toolchain preserved')


def cli_controls():
    environment = dict(os.environ)
    for name in (*EXPECTED, 'RUSTUP_TOOLCHAIN', 'RUSTUP_AUTO_INSTALL'):
        environment.pop(name, None)
    child = "import json, os, sys; print(json.dumps([sys.argv[1:], {k: os.environ.get(k) for k in ('CARGO_HOME','RUSTUP_HOME','CARGO_TARGET_DIR','TMPDIR')}, os.environ['D156_VALUE']])); sys.exit(7)"
    argv = ['one argument', '$literal;$(never execute)', '', '--help']
    environment['D156_VALUE'] = 'retained child value'
    result = subprocess.run([sys.executable, '-I', '-B', str(SOURCE), '--', sys.executable,
                             '-I', '-B', '-c', child, *argv], cwd=ROOT, env=environment,
                            capture_output=True, text=True)
    assert result.returncode == 7, ('D156 exec status', result.returncode, result.stderr)
    actual = json.loads(result.stdout)
    assert actual == [argv, {k: str(ROOT / v) for k, v in EXPECTED.items()}, 'retained child value'], 'D156 exec argv/environment'
    print('local environment: actual exec argv/empty argument/unrelated environment/exit7')
    inspection = dict(environment, **{name: str(WORK / 'inspection' / name) for name in EXPECTED})
    result = subprocess.run([sys.executable, '-I', '-B', str(SOURCE), '--inspect'], cwd=ROOT,
                            env=inspection, capture_output=True, text=True)
    assert result.returncode == 0, ('D156 readonly inspect', result.stderr)
    assert json.loads(result.stdout) == {name: 'target/scratch/local_environment_contract/inspection/' + name for name in EXPECTED}, 'D156 inspection identity'
    assert not (WORK / 'inspection').exists(), 'D156 inspection created stores'
    # A real review child captures effective variables; no real G0 check is substituted as evidence.
    output = WORK / 'review-environment.jsonl'
    git_called = WORK / 'premature-git'
    binary = WORK / 'review-bin'
    binary.mkdir()
    git = binary / 'git'
    git.write_text('#!/usr/bin/env python3\nimport os, pathlib\npathlib.Path(os.environ["D156_GIT_CAPTURE"]).write_text("called")\nprint(' + repr(str(ROOT)) + ')\n')
    git.chmod(0o755)
    child_path = WORK / 'review_capture.py'
    child_path.write_text("import json, os, pathlib\nwith pathlib.Path(os.environ['D156_CAPTURE']).open('a') as f:\n    f.write(json.dumps({k: os.environ.get(k) for k in " + repr(tuple(EXPECTED)) + "}) + '\\n')\n")
    plane = ROOT / 'docs/tasks/artifacts/g0_exit/g0_exit_clauses.tsv'
    lines = []
    count = 0
    for line in plane.read_text().splitlines():
        fields = line.split('\t')
        if len(fields) == 8 and fields[5] == 'instrument':
            fields[4] = 'python3 -I -B ' + child_path.relative_to(ROOT).as_posix()
            count += 1
            line = '\t'.join(fields)
        lines.append(line)
    assert count == 18, 'D156 review instrument population changed'
    clauses = WORK / 'clauses.tsv'
    clauses.write_text('\n'.join(lines) + '\n')
    environment.update(D156_CAPTURE=str(output), G0_EXIT_CLAUSES=str(clauses))
    environment.update(PATH=str(binary) + os.pathsep + environment['PATH'], D156_GIT_CAPTURE=str(git_called))
    environment.pop('G0_EXIT_SKIP_CHECKS', None)
    result = subprocess.run(['bash', '../docs/tasks/artifacts/g0_exit/run_g0_exit_review.sh'],
                            cwd=ROOT / 'scripts', env=environment, capture_output=True, text=True)
    assert result.returncode == 0, ('D156 review subdirectory entry', result.stderr)
    rows = [json.loads(line) for line in output.read_text().splitlines()]
    assert len(rows) == count and all(row == {k: str(ROOT / v) for k, v in EXPECTED.items()} for row in rows), 'D156 review child environment'
    assert not git_called.exists(), 'D156 root discovery invoked Git before preparation'
    print('local environment: actual G0 review subdirectory re-entry; 18 captured child environments; no premature Git')


def shape_control(scope):
    fixture = WORK / 'invalid-command'
    fixture.mkdir()
    original_root, original_environment, original_argv = scope['ROOT'], os.environ, sys.argv
    try:
        scope['ROOT'] = fixture
        os.environ = {}
        for argv in ([], ['--inspect', 'unexpected']):
            sys.argv = [str(SOURCE), *argv]
            try:
                scope['main']()
            except scope['Refusal']:
                pass
            else:
                raise AssertionError('D156 invalid command accepted')
            assert not (fixture / 'target').exists(), 'D156 invalid command created stores'
        sys.argv = [str(SOURCE), '--inspect']
        with contextlib.redirect_stdout(io.StringIO()):
            scope['main']()
        assert not (fixture / 'target').exists(), 'D156 inspection created stores'
    finally:
        scope['ROOT'], os.environ, sys.argv = original_root, original_environment, original_argv


def launcher_controls():
    launcher = ROOT / 'scripts/run_make.sh'
    original = launcher.read_bytes()
    fixture = WORK / 'launcher'
    (fixture / 'scripts').mkdir(parents=True)
    (fixture / 'scripts/local_environment.py').write_bytes(SOURCE.read_bytes())
    binary = fixture / 'bin'
    binary.mkdir()
    capture = binary / 'make'
    capture.write_text('''#!/usr/bin/env python3
import json, os, pathlib, sys
pathlib.Path(os.environ['D156_CAPTURE']).write_text(json.dumps([sys.argv[1:],
    {k: os.environ.get(k) for k in ('CARGO_HOME','RUSTUP_HOME','CARGO_TARGET_DIR','TMPDIR')},
    os.environ.get('MAKE_TMPDIR'), os.environ.get('RUSTUP_AUTO_INSTALL')]))
''')
    capture.chmod(0o755)
    environment = dict(os.environ)
    for name in (*EXPECTED, 'MAKE_TMPDIR', 'RUSTUP_AUTO_INSTALL', 'RUSTUP_TOOLCHAIN'):
        environment.pop(name, None)
    output = fixture / 'actual.json'
    environment.update(PATH=str(binary) + os.pathsep + environment['PATH'], D156_CAPTURE=str(output))
    arguments = ['check', 'value with spaces', '$literal;$(never execute)', '']

    def check(script, expected_root):
        output.unlink(missing_ok=True)
        result = subprocess.run(['bash', str(script), *arguments], cwd=ROOT / 'docs',
                                env=environment, capture_output=True, text=True)
        assert result.returncode == 0, ('D156 launcher execution', result.stderr)
        actual = json.loads(output.read_text())
        assert actual[0] == ['-C', str(expected_root), *arguments], 'D156 launcher argv/root'
        assert actual[1] == {k: str(expected_root / v) for k, v in EXPECTED.items()}, 'D156 pre-Make profile'
        assert actual[2:] == [str(expected_root / 'target/scratch'), '0'], 'D156 pre-Make temp/installation'

    check(launcher, ROOT)
    copied = fixture / 'scripts/run_make.sh'
    copied.write_bytes(original)
    check(copied, fixture)
    needle = 'exec python3 -I -B "$ROOT/scripts/local_environment.py" -- make'
    text = original.decode()
    assert text.count(needle) == 1, 'D156 changed launcher fault anchor'
    copied.write_text(text.replace(needle, 'exec make'))
    try:
        check(copied, fixture)
    except AssertionError as error:
        assert str(error) == 'D156 pre-Make profile', error
    else:
        raise AssertionError('D156 launcher fault passed')
    copied.write_bytes(original)
    check(copied, fixture)
    assert launcher.read_bytes() == original, 'D156 launcher source changed'
    print('local environment: real pre-Make launcher/relocated copied fixture/one actual shell body red/restored')


def run():
    original = SOURCE.read_bytes()
    text = original.decode()
    count = controls(load(text))
    print('local environment: %d independent runtime controls' % count)
    shape_control(load(text))
    faults = {
        'default store': ("'CARGO_HOME': 'target/cargo-home'", "'CARGO_HOME': 'target/wrong-home'"),
        'implicit install': ("result['RUSTUP_AUTO_INSTALL'] = '0'", "result['RUSTUP_AUTO_INSTALL'] = '1'"),
        'Make temp identity': ("result['MAKE_TMPDIR'] = result['TMPDIR']", "result['MAKE_TMPDIR'] = str(root / 'target/wrong-temp')"),
        'inspection creation': ('create=not args.inspect', 'create=True'),
        'effective identity': ('environment.get(name) == wanted[name]', 'True'),
        'selection escape': ("'..' not in path.parts and '.git' not in path.parts, 'invalid store components'", "True, 'invalid store components'"),
        'component escape': ("not path.is_absolute() and '..' not in path.parts and '.git' not in path.parts", 'True'),
        'symlink': ('not current.is_symlink()', 'True'),
        'Git boundary': ('not (marker.exists() or marker.is_symlink())', 'True'),
        'device': ('current.stat().st_dev == device', 'True'),
        'component type': ('current.is_dir()', 'True'),
        'whole-plan validation': ('directory(root, path.relative_to(root).as_posix(), missing_ok=True)', 'pass'),
    }
    for label, (needle, replacement) in faults.items():
        assert text.count(needle) == 1, ('D156 changed fault anchor', label)
        try:
            scope = load(text.replace(needle, replacement))
            controls(scope)
            shape_control(scope)
        except AssertionError as error:
            assert str(error).startswith('D156 '), (label, 'non-body refusal', error)
        else:
            raise AssertionError('D156 compiled fault passed: ' + label)
        print('  actual compiled local assertion red:', label)
    cleanup()
    WORK.mkdir(parents=True)
    early = "    else:\n        require(bool(command), 'a producer command is required')\n"
    assert text.count(early) == 1
    delayed = text.replace(early, '').replace('    if args.inspect:\n', "    require(bool(command) or args.inspect, 'a producer command is required')\n    if args.inspect:\n")
    try:
        shape_control(load(delayed))
    except AssertionError as error:
        assert str(error) == 'D156 invalid command created stores', error
    else:
        raise AssertionError('D156 delayed shape guard passed')
    print('  actual compiled local assertion red: command validation order')
    make_controls()
    make_text = (ROOT / 'Makefile').read_text()
    make_faults = {
        'Make profile forwarding': ('LOCAL_RUN := python3 -I -B "$(CURDIR)/scripts/local_environment.py" --', 'LOCAL_RUN :='),
        'explicit setup self-update': (' --no-self-update\n', '\n'),
    }
    for label, (needle, replacement) in make_faults.items():
        assert make_text.count(needle) == 1, ('D156 changed Make fault anchor', label)
        try:
            make_controls(make_text.replace(needle, replacement))
        except AssertionError as error:
            assert isinstance(error.args[0], tuple) and error.args[0][0] in (
                'D156 actual Make effective stores', 'D156 toolchain flags or channel changed'), (label, error)
        else:
            raise AssertionError('D156 actual Make fault passed: ' + label)
        print('  actual Make assertion red:', label)
    make_controls()
    cli_controls()
    launcher_controls()
    assert SOURCE.read_bytes() == original, 'D156 source bytes changed'
    print('local-environment probes: %d runtime controls / %d actual compiled reds / 2 actual Make reds / 1 actual launcher red / actual exec+G0 children / 0 fail' % (count, len(faults) + 1))


if __name__ == '__main__':
    try:
        run()
    finally:
        cleanup()
