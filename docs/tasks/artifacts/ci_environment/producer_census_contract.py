"""Independent census assertions and actual-body counterfactuals; no producer is invoked."""
from pathlib import Path
import argparse
import ast
import os
import runpy
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[4]
HERE = Path(__file__).resolve().parent
SOURCE = HERE / 'producer_census.py'
GOOD = runpy.run_path(str(SOURCE))
PROFILE = runpy.run_path(str(ROOT / 'scripts/local_environment.py'))


def controls(scope):
    calls = scope['python_calls']("""# subprocess.run(['cargo'])
fault = "subprocess.run(['cargo'])"
from subprocess import run as invoke
invoke(['rustc', 'source.rs'])
registry['dynamic'](argument)
path.write_bytes(b'fixture')
""", 'independent.py')
    assert [(row['line'], row['callee']) for row in calls] == [
        (4, 'invoke'), (5, "registry['dynamic']"), (6, 'path.write_bytes')], 'D156 AST coverage'
    assert calls[1]['candidate'] and calls[2]['candidate'], 'D156 dynamic/write candidates'
    assert calls[0]['source'] == "invoke(['rustc', 'source.rs'])", 'D156 literal argv retained'
    shell = """# python3 <<'IGNORED'
cat <<'DATA'
subprocess.run(['cargo'])
DATA
python3 - <<'PY'
from subprocess import run as invoke
invoke(['rustc'])
PY
grep pattern <<<"$out"
cat <<-END
\tdata
\tEND
"""
    bodies = scope['shell_sections'](shell, 'independent.sh')
    assert [(row['opener_line'], row['first_line'], row['last_line'], row['delimiter'])
            for row in bodies] == [(2, 3, 3, 'DATA'), (5, 6, 7, 'PY'), (10, 11, 11, 'END')], 'D156 heredoc boundaries'
    assert bodies[0]['python_calls'] is None, 'D156 data is not a command'
    assert [(row['line'], row['callee']) for row in bodies[1]['python_calls']] == [
        (7, 'invoke')], 'D156 embedded AST location'
    assert bodies[2]['source'] == '\tdata\n', 'D156 literal body retained'
    refused = 0
    for action, wanted in (
        (lambda: scope['python_calls']('def broken(', 'broken.py'), 'unparsed Python: broken.py'),
        (lambda: scope['shell_sections']("cat <<'END'\nmissing\n", 'broken.sh'), 'unclosed literal heredoc: broken.sh'),
    ):
        try:
            action()
        except scope['Refusal'] as error:
            assert str(error) == wanted, 'D156 refusal identity'
            refused += 1
        else:
            raise AssertionError('D156 inventory accepted malformed source')
    return 7, refused


def source_controls():
    # Prepare before the first temporary fixture, including a direct standalone invocation.
    PROFILE['environment_for'](ROOT, dict(os.environ))
    work = PROFILE['directory'](ROOT, 'target/scratch/producer_census_contract', create=True)
    with tempfile.TemporaryDirectory(prefix='case-', dir=work) as directory:
        fixture = Path(directory)
        for name in GOOD['TREES']:
            (fixture / name).mkdir(parents=True)
        for name in GOOD['FILES']:
            path = fixture / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text('')
        script = fixture / 'scripts/one.py'
        script.write_text('print("read-only fixture")\n')
        expected = sorted(['scripts/one.py', 'Makefile', 'Cargo.toml', 'rust-toolchain.toml', 'docs/book/book.toml'])
        assert GOOD['inventory'](fixture) == expected, 'D156 independent source inventory'
        manifest = fixture / 'entries.tsv'
        header = 'path\tkind\towner\treview\n'
        rows = [path + '\treference\tfixture\tindependent\n' for path in expected]
        manifest.write_text(header + ''.join(rows))
        assert [row['path'] for row in GOOD['census'](fixture, manifest)] == expected, 'D156 manifest coverage'
        failures = 0

        def reject(action, message):
            nonlocal failures
            try:
                action()
            except GOOD['Refusal'] as error:
                assert message in str(error), 'D156 source refusal identity'
                failures += 1
            else:
                raise AssertionError('D156 source boundary accepted')

        manifest.write_text(header + ''.join(rows[:-1]))
        reject(lambda: GOOD['census'](fixture, manifest), 'source/entry inventory differs')
        manifest.write_text(header + ''.join(rows) + rows[0])
        reject(lambda: GOOD['entries'](manifest), 'duplicate inventory entry')
        manifest.write_text(header + ''.join(rows).replace('\treference\t', '\tunknown\t', 1))
        reject(lambda: GOOD['entries'](manifest), 'unclassified source entry')
        script.unlink()
        script.symlink_to(fixture / 'Cargo.toml')
        reject(lambda: GOOD['inventory'](fixture), 'source symlink')
        script.unlink()
        script.write_text('')
        marker = fixture / 'scripts/.git'
        for dangling in (False, True):
            if dangling:
                marker.symlink_to(fixture / 'absent')
            else:
                marker.write_text('fixture boundary')
            try:
                reject(lambda: GOOD['inventory'](fixture), 'nested Git source boundary')
            finally:
                marker.unlink()
        book = fixture / 'docs/book'
        (book / 'book.toml').unlink()
        book.rmdir()
        book.symlink_to(fixture / 'scripts')
        try:
            reject(lambda: GOOD['inventory'](fixture), 'source symlink ancestor')
        finally:
            book.unlink()
        return 2, failures


def main():
    original = SOURCE.read_bytes()
    passed, refused = controls(GOOD)
    extra, boundaries = source_controls()
    faults = (
        ('AST call dropped', 'if isinstance(node, ast.Call):', 'if isinstance(node, ast.Call) and node.lineno != 5:'),
        ('dynamic call hidden', 'name in EFFECTS or name is None', 'name in EFFECTS'),
        ('embedded ordinal shifted', 'python_calls(source, filename, first)', 'python_calls(source, filename, first + 1)'),
        ('data treated as commands', "delimiter in {'PY', 'PYTHON'}", "delimiter in {'PY', 'PYTHON', 'DATA'}"),
    )
    red = 0
    for name, before, after in faults:
        assert original.decode().count(before) == 1, (name, 'actual anchor')
        scope = {'__file__': str(SOURCE), '__name__': 'census_fault'}
        exec(compile(original.decode().replace(before, after, 1), str(SOURCE), 'exec'), scope)
        try:
            controls(scope)
        except AssertionError as error:
            assert str(error).startswith('D156 '), (name, 'unrelated failure', str(error))
            red += 1
        else:
            raise AssertionError((name, 'counterfactual survived'))
    assert SOURCE.read_bytes() == original, 'D156 census source changed'
    coverage()
    print('producer-census controls: ' + str(passed + extra) + ' assertions, ' +
          str(refused + boundaries) + ' actual refusals, ' + str(red) + ' compiled-body reds; source unchanged')


def coverage():
    """Git's independent maintained-path census watches sources outside the traversal roots too."""
    environment = PROFILE['environment_for'](ROOT, dict(os.environ))
    raw = subprocess.check_output(['git', 'ls-files', '--cached', '--others', '--exclude-standard', '-z'],
                                  cwd=ROOT, env=environment).decode().split('\0')
    expected = {path for path in raw if path and (
        path.endswith(('.py', '.sh')) or path.startswith('.githooks/') or
        (path.startswith('.github/workflows/') and path.endswith(('.yml', '.yaml'))) or
        Path(path).name in {'Cargo.toml', 'build.rs'} or
        path in {'Makefile', 'rust-toolchain.toml', 'docs/book/book.toml'})}
    rows = GOOD['census'](ROOT, HERE / 'producer_entries.tsv')
    assert expected == {row['path'] for row in rows}, 'D156 independent Git source coverage'

    class Calls(ast.NodeVisitor):
        def __init__(self):
            self.positions = []

        def visit_Call(self, node):
            self.positions.append((node.lineno, node.col_offset))
            self.generic_visit(node)

    checked = 0
    for row in rows:
        if row['path'].endswith('.py'):
            visitor = Calls()
            visitor.visit(ast.parse(row['source']))
            assert sorted(visitor.positions) == [(call['line'], call['column']) for call in row['calls']], \
                ('D156 independent full AST coverage', row['path'])
            checked += 1
        if row['path'].endswith('.sh') or row['path'].startswith('.githooks/'):
            subprocess.run(['bash', '-n', str(ROOT / row['path'])], cwd=ROOT,
                           env=environment, check=True, capture_output=True)
            checked += 1
        for body in row.get('heredocs', []):
            lines = row['source'].splitlines(keepends=True)
            assert ''.join(lines[body['first_line'] - 1:body['last_line']]) == body['source'], \
                'D156 embedded source bytes'
            assert lines[body['last_line']].strip() == body['delimiter'], 'D156 actual closing delimiter'
            if body['python_calls'] is not None:
                visitor = Calls()
                visitor.visit(ast.parse(body['source']))
                assert sorted((line + body['first_line'] - 1, column) for line, column in visitor.positions) == [
                    (call['line'], call['column']) for call in body['python_calls']], 'D156 full embedded AST coverage'
            checked += 1
    print('producer-census watched coverage: ' + str(len(expected)) + ' independent Git paths / ' +
          str(checked) + ' AST/source/bash boundary checks; pass')


def baseline():
    """Stop the actual standalone body at its first mkdir, before mutation or compiler dispatch."""
    source = ROOT / 'docs/tasks/artifacts/size_membership/size_membership_mutations.py'
    original = source.read_bytes()
    wanted = ROOT / 'target/size_membership_mutations'
    # This source requests WORK.mkdir before reading its Rust source or entering the mutation
    # loop. Every mkdir and child dispatch is intercepted fail-closed.
    saved = dict(os.environ)
    mkdir, run, execute = Path.mkdir, subprocess.run, os.execvpe
    stores = ('CARGO_HOME', 'RUSTUP_HOME', 'CARGO_TARGET_DIR', 'TMPDIR', 'MAKE_TMPDIR')
    captured = []

    class Stopped(Exception):
        """No producer write or child invocation is permitted in this baseline."""

    def stop_mkdir(path, *args, **kwargs):
        assert path == wanted, ('D156 unexpected first directory', str(path))
        captured.append({name: os.environ.get(name) for name in stores})
        raise Stopped

    def stop_child(*args, **kwargs):
        raise AssertionError('D156 unexpected child before captured first write')

    try:
        for name in stores:
            os.environ.pop(name, None)
        Path.mkdir, subprocess.run, os.execvpe = stop_mkdir, stop_child, stop_child
        try:
            exec(compile(original, str(source), 'exec'),
                 {'__file__': str(source), '__name__': 'captured_standalone'})
        except Stopped:
            pass
        else:
            raise AssertionError('D156 standalone body never reached first write')
    finally:
        Path.mkdir, subprocess.run, os.execvpe = mkdir, run, execute
        os.environ.clear()
        os.environ.update(saved)
    assert captured == [{name: None for name in stores}], 'D156 standalone profile baseline changed'
    assert source.read_bytes() == original, 'D156 standalone baseline changed source'
    print('producer baseline: actual size-membership body reaches first mkdir with five exports absent; '
          'write refused by capture, no mutation/compiler/child, source unchanged')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--baseline', action='store_true', help='capture the pre-repair standalone prefix')
    args = parser.parse_args()
    main()
    if args.baseline:
        baseline()
