"""Independent census assertions and actual-body counterfactuals; no producer is invoked."""
from pathlib import Path
import argparse
import ast
import copy
import csv
import io
import os
import re
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
    effect_plan_controls()
    file_plan_controls()
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


def validate_effect_plan(entries, text, tasks):
    """Exact planned path/kind/owner coverage; no runtime locality authority."""
    reader = csv.DictReader(io.StringIO(text), delimiter='\t')
    assert reader.fieldnames == ['path', 'kind', 'leaf'], 'D156 effect plan schema'
    rows = list(reader)
    assert all(set(row) == {'path', 'kind', 'leaf'} and
               all(isinstance(value, str) and value for value in row.values())
               for row in rows), 'D156 effect row shape'
    expected = {p: row['kind'] for p, row in entries.items()
                if p.endswith('.py') and row['kind'] in {'output', 'delegate'}}
    assert len(rows) == len({row['path'] for row in rows}), 'D156 duplicate effect path'
    assert {row['path'] for row in rows} == set(expected), 'D156 effect path coverage'
    assert all(row['kind'] == expected[row['path']] for row in rows), 'D156 effect kind changed'
    owners = {line.removeprefix('- ID: `').removesuffix('`') for line in tasks.splitlines()
              if line.startswith('- ID: `G1-SLICE.5b.4c.h2.b.p.o.a')}
    owners = {owner for owner in owners if not any(
        other.startswith(owner + '.') for other in owners)}
    assert owners and {row['leaf'] for row in rows} == owners, 'D156 effect owner coverage'
    return len(rows), len(owners)


def effect_plan_cases(validate):
    # Independent authored data keeps guard faults orthogonal even as real groups split.
    parent = 'G1-SLICE.5b.4c.h2.b.p.o.a01'
    other = 'G1-SLICE.5b.4c.h2.b.p.o.a02'
    entries = {'fixture/a.py': {'kind': 'output'}, 'fixture/b.py': {'kind': 'output'},
               'fixture/c.py': {'kind': 'delegate'}, 'fixture/ignored.py': {'kind': 'profile'}}
    text = ('path\tkind\tleaf\nfixture/a.py\toutput\t' + parent +
            '\nfixture/b.py\toutput\t' + parent + '\nfixture/c.py\tdelegate\t' + other + '\n')
    tasks = '- ID: `' + parent + '`\n- ID: `' + other + '`\n'
    result = validate(entries, text, tasks)
    lines = text.splitlines(keepends=True)
    swapped = ''.join('\t'.join((parts[1], parts[0], parts[2]))
                      for parts in (line.split('\t') for line in lines))
    # A split replaces row owners, while the original group remains a tracked parent.
    child = parent + '.1'
    assert validate(entries, text.replace(parent + '\n', child + '\n'),
                    tasks + '\n- ID: `' + child + '`\n') == result, 'D156 split owner coverage'
    failures = 0
    for changed, task_text, message in (
        (swapped, tasks, 'D156 effect plan schema'),
        (''.join(lines[:-1]) + lines[-1].rstrip('\n') + '\textra\n', tasks,
         'D156 effect row shape'),
        (text.replace('\toutput\t', '\t\t', 1), tasks, 'D156 effect row shape'),
        (''.join(lines[:1] + lines[2:]), tasks, 'D156 effect path coverage'),
        (text + lines[-1], tasks, 'D156 duplicate effect path'),
        (text.replace('\toutput\t', '\treference\t', 1), tasks, 'D156 effect kind changed'),
        (text.replace('\tG1-SLICE.', '\tUNKNOWN.', 1), tasks, 'D156 effect owner coverage'),
        (text, tasks.replace('- ID: `' + parent + '`', '- ID: `REMOVED`', 1),
         'D156 effect owner coverage'),
    ):
        try:
            validate(entries, changed, task_text)
        except AssertionError as error:
            assert str(error) == message, ('D156 unrelated effect-plan refusal', str(error))
            failures += 1
        else:
            raise AssertionError(('D156 missed effect-plan refusal', message))
    return result, failures


def effect_plan_controls():
    entries = GOOD['entries'](HERE / 'producer_entries.tsv')
    text = (HERE / 'python_effect_entries.tsv').read_text()
    tasks = (ROOT / 'docs/tasks/G1-SLICE-producers.md').read_text()
    result = validate_effect_plan(entries, text, tasks)
    _, failures = effect_plan_cases(validate_effect_plan)
    original = Path(__file__).read_bytes()
    function = next(node for node in ast.parse(original).body
                    if isinstance(node, ast.FunctionDef) and node.name == 'validate_effect_plan')
    reds = 0
    for diagnostic in ('D156 effect plan schema', 'D156 effect row shape',
                       'D156 duplicate effect path', 'D156 effect path coverage',
                       'D156 effect kind changed', 'D156 effect owner coverage'):
        module = ast.Module(body=[copy.deepcopy(function)], type_ignores=[])
        matches = [node for node in ast.walk(module) if isinstance(node, ast.Assert) and
                   isinstance(node.msg, ast.Constant) and node.msg.value == diagnostic]
        assert len(matches) == 1, ('D156 actual effect guard anchor', diagnostic)
        # Remove the actual guard in memory; tracked source and all producers stay untouched.
        matches[0].test = ast.copy_location(ast.Constant(value=True), matches[0].test)
        scope = {'csv': csv, 'io': io}
        exec(compile(ast.fix_missing_locations(module), __file__, 'exec'), scope)
        try:
            effect_plan_cases(scope['validate_effect_plan'])
        except AssertionError as error:
            assert error.args == (('D156 missed effect-plan refusal', diagnostic),), \
                ('D156 unrelated effect guard fault', diagnostic, error.args)
            reds += 1
        else:
            raise AssertionError(('D156 effect guard omission survived', diagnostic))
    module = ast.Module(body=[copy.deepcopy(function)], type_ignores=[])
    filters = [node for node in ast.walk(module) if isinstance(node, ast.Assign) and
               isinstance(node.value, ast.SetComp) and
               isinstance(node.value.elt, ast.Name) and node.value.elt.id == 'owner']
    assert len(filters) == 1, 'D156 actual terminal-owner anchor'
    filters[0].value = ast.copy_location(ast.Name(id='owners', ctx=ast.Load()), filters[0].value)
    scope = {'csv': csv, 'io': io}
    exec(compile(ast.fix_missing_locations(module), __file__, 'exec'), scope)
    try:
        effect_plan_cases(scope['validate_effect_plan'])
    except AssertionError as error:
        assert error.args == ('D156 effect owner coverage',), \
            ('D156 unrelated split-owner fault', error.args)
        reds += 1
    else:
        raise AssertionError('D156 terminal-owner omission survived')
    assert Path(__file__).read_bytes() == original, 'D156 effect guard source changed'
    print('effect plan coverage:', result[0], 'current Python paths /', result[1],
          'owned groups /', failures, 'actual data/owner refusals /', reds,
          'actual guard-omission reds; split owner positive; inspection only, runtime pending')


def native_file_owners(tasks, entries):
    """Canonical native groups retain exact current membership, including split leaves."""
    matches = re.findall(
        r'^- ID: `(G1-SLICE\.5b\.4c\.h2\.b\.p\.o\.f\.n\d+(?:\.\d+)*)`\n(.*?)(?=^- ID: |\Z)',
        tasks, re.M | re.S)
    assert len(matches) == len({owner for owner, _ in matches}), 'D156 duplicate native file owner'
    groups = dict(matches)
    groups = {owner: body for owner, body in groups.items()
              if not any(other.startswith(owner + '.') for other in groups)}
    fields = {owner: re.findall(r'^  Paths: (.+)$', body, re.M) for owner, body in groups.items()}
    assert fields and all(len(value) == 1 for value in fields.values()), 'D156 native file group shape'
    paths = [(path, owner) for owner, value in fields.items() for path in value[0].split(';')]
    assert all(1 <= len(value[0].split(';')) <= 4 for value in fields.values()), 'D156 native file group bound'
    assert len(paths) == len({path for path, _ in paths}), 'D156 duplicate native file path'
    assert {path for path, _ in paths} == {path for path, row in entries.items()
                                          if path.endswith('.py') and row['kind'] == 'native'}, \
        'D156 native file path coverage'
    return dict(paths)


def validate_file_plan(entries, text, tasks, effects):
    """Every maintained Python path is owned; destination resolution remains inspection work."""
    reader = csv.DictReader(io.StringIO(text), delimiter='\t')
    assert reader.fieldnames == ['path', 'kind', 'leaf'], 'D156 file plan schema'
    rows = list(reader)
    assert all(set(row) == {'path', 'kind', 'leaf'} and
               all(isinstance(value, str) and value for value in row.values()) for row in rows), \
        'D156 file row shape'
    expected = {path: row['kind'] for path, row in entries.items() if path.endswith('.py')}
    assert len(rows) == len({row['path'] for row in rows}), 'D156 duplicate file path'
    assert {row['path'] for row in rows} == set(expected), 'D156 file path coverage'
    assert all(row['kind'] == expected[row['path']] for row in rows), 'D156 file kind changed'
    native = native_file_owners(tasks, entries)
    assert all(row['leaf'] == native[row['path']] for row in rows if row['kind'] == 'native'), \
        'D156 native file owner changed'
    assert all(row['leaf'] == effects.get(row['path']) for row in rows
               if row['kind'] in {'output', 'delegate'}), 'D156 effect file owner changed'
    assert all(row['leaf'] == 'G1-SLICE.5b.4c.h2.b.p.v' for row in rows
               if row['kind'] in {'profile', 'reference'}), 'D156 remaining file owner changed'
    owners = set(re.findall(r'^- ID: `(G1-SLICE(?:\.[^`]+)?)`$', tasks, re.M))
    assert all(row['leaf'] in owners for row in rows), 'D156 file owner missing'
    return len(rows), len({row['leaf'] for row in rows}), len(set(native.values()))


def file_plan_cases(validate):
    """Authored orthogonal fixtures do not depend on current group sizes or ordering."""
    native = 'G1-SLICE.5b.4c.h2.b.p.o.f.n01'
    effect = 'G1-SLICE.5b.4c.h2.b.p.o.a02'
    other = 'G1-SLICE.5b.4c.h2.b.p.o.a03'
    remaining = 'G1-SLICE.5b.4c.h2.b.p.v'
    entries = {'fixture/native.py': {'kind': 'native'}, 'fixture/a.py': {'kind': 'output'},
               'fixture/b.py': {'kind': 'delegate'}, 'fixture/c.py': {'kind': 'profile'},
               'fixture/d.py': {'kind': 'reference'}, 'fixture/ignored.sh': {'kind': 'output'}}
    effects = {'fixture/a.py': effect, 'fixture/b.py': other}
    text = ('path\tkind\tleaf\nfixture/native.py\tnative\t' + native +
            '\nfixture/a.py\toutput\t' + effect + '\nfixture/b.py\tdelegate\t' + other +
            '\nfixture/c.py\tprofile\t' + remaining + '\nfixture/d.py\treference\t' + remaining + '\n')
    tasks = ('- ID: `' + native + '`\n  Paths: fixture/native.py\n- ID: `' + effect +
             '`\n- ID: `' + other + '`\n- ID: `' + remaining + '`\n')
    assert validate(entries, text, tasks, effects) == (5, 4, 1), 'D156 file fixture population'
    child = native + '.1'
    split = tasks + '- ID: `' + child + '`\n  Paths: fixture/native.py\n'
    assert validate(entries, text.replace(native + '\n', child + '\n'), split, effects) == (5, 4, 1), \
        'D156 native file split owner'
    lines = text.splitlines(keepends=True)
    swapped = ''.join('\t'.join((parts[1], parts[0], parts[2]))
                      for parts in (line.split('\t') for line in lines))
    cases = (
        (swapped, tasks, 'D156 file plan schema'),
        (''.join(lines[:-1]) + lines[-1].rstrip('\n') + '\textra\n', tasks, 'D156 file row shape'),
        (text.replace('\tprofile\t', '\t\t', 1), tasks, 'D156 file row shape'),
        (text + lines[-1], tasks, 'D156 duplicate file path'),
        (''.join(lines[:-1]), tasks, 'D156 file path coverage'),
        (text.replace('\tprofile\t', '\treference\t', 1), tasks, 'D156 file kind changed'),
        (text.replace('native\t' + native, 'native\t' + effect, 1), tasks, 'D156 native file owner changed'),
        (text.replace('output\t' + effect, 'output\t' + other, 1), tasks, 'D156 effect file owner changed'),
        (text.replace('profile\t' + remaining, 'profile\t' + effect, 1), tasks, 'D156 remaining file owner changed'),
        (text, tasks.replace('- ID: `' + remaining + '`', '- ID: `REMOVED`'), 'D156 file owner missing'),
    )
    for changed, task_text, message in cases:
        try:
            validate(entries, changed, task_text, effects)
        except AssertionError as error:
            assert str(error) == message, ('D156 unrelated file-plan refusal', str(error))
        else:
            raise AssertionError(('D156 missed file-plan refusal', message))
    return len(cases)


def native_file_cases(parse):
    first, second = ('G1-SLICE.5b.4c.h2.b.p.o.f.n01', 'G1-SLICE.5b.4c.h2.b.p.o.f.n02')
    paths = ['fixture/n%d.py' % i for i in range(5)]
    entries = {path: {'kind': 'native'} for path in paths}
    head = '- ID: `' + first + '`\n  Paths: ' + ';'.join(paths[:4]) + '\n'
    tail = '- ID: `' + second + '`\n  Paths: ' + paths[4] + '\n'
    assert parse(head + tail, entries) == {path: first if i < 4 else second
                                          for i, path in enumerate(paths)}, 'D156 native membership fixture'
    cases = (
        (head + head + tail, 'D156 duplicate native file owner'),
        (head + tail.replace('  Paths:', '  Lost:'), 'D156 native file group shape'),
        (head + tail.replace(paths[4], paths[0]), 'D156 duplicate native file path'),
        (head, 'D156 native file path coverage'),
        ('- ID: `' + first + '`\n  Paths: ' + ';'.join(paths) + '\n', 'D156 native file group bound'),
    )
    for tasks, message in cases:
        try:
            parse(tasks, entries)
        except AssertionError as error:
            assert str(error) == message, ('D156 unrelated native-group refusal', str(error))
        else:
            raise AssertionError(('D156 missed native-group refusal', message))
    return len(cases)


def file_plan_controls():
    entries = GOOD['entries'](HERE / 'producer_entries.tsv')
    tasks = '\n'.join(path.read_text() for path in sorted((ROOT / 'docs/tasks').glob('G1-SLICE*.md')))
    effects = {row['path']: row['leaf'] for row in csv.DictReader(
        io.StringIO((HERE / 'python_effect_entries.tsv').read_text()), delimiter='\t')}
    result = validate_file_plan(entries, (HERE / 'python_file_entries.tsv').read_text(), tasks, effects)
    failures = file_plan_cases(validate_file_plan) + native_file_cases(native_file_owners)
    original = Path(__file__).read_bytes()
    function = next(node for node in ast.parse(original).body
                    if isinstance(node, ast.FunctionDef) and node.name == 'validate_file_plan')
    reds = 0
    for guard in [node for node in ast.walk(function) if isinstance(node, ast.Assert)]:
        diagnostic = guard.msg.value
        module = ast.Module(body=[copy.deepcopy(function)], type_ignores=[])
        match = next(node for node in ast.walk(module) if isinstance(node, ast.Assert) and
                     node.msg.value == diagnostic)
        match.test = ast.copy_location(ast.Constant(value=True), match.test)
        scope = {'csv': csv, 'io': io, 're': re, 'native_file_owners': native_file_owners}
        exec(compile(ast.fix_missing_locations(module), __file__, 'exec'), scope)
        try:
            file_plan_cases(scope['validate_file_plan'])
        except AssertionError as error:
            assert error.args == (('D156 missed file-plan refusal', diagnostic),), \
                ('D156 unrelated file guard fault', diagnostic, error.args)
            reds += 1
        else:
            raise AssertionError(('D156 file guard omission survived', diagnostic))
    parser = next(node for node in ast.parse(original).body
                  if isinstance(node, ast.FunctionDef) and node.name == 'native_file_owners')
    for diagnostic in ('D156 native file group bound', 'D156 duplicate native file owner'):
        module = ast.Module(body=[copy.deepcopy(parser)], type_ignores=[])
        guard = next(node for node in ast.walk(module) if isinstance(node, ast.Assert) and
                     node.msg.value == diagnostic)
        guard.test = ast.copy_location(ast.Constant(value=True), guard.test)
        scope = {'re': re}
        exec(compile(ast.fix_missing_locations(module), __file__, 'exec'), scope)
        try:
            native_file_cases(scope['native_file_owners'])
        except AssertionError as error:
            assert error.args == (('D156 missed native-group refusal', diagnostic),), \
                ('D156 unrelated native-group fault', diagnostic, error.args)
            reds += 1
        else:
            raise AssertionError(('D156 native group guard omission survived', diagnostic))
    module = ast.Module(body=[copy.deepcopy(parser), copy.deepcopy(function)], type_ignores=[])
    filters = [node for node in ast.walk(module) if isinstance(node, ast.Assign) and
               any(isinstance(target, ast.Name) and target.id == 'groups' for target in node.targets) and
               isinstance(node.value, ast.DictComp)]
    assert len(filters) == 1, 'D156 actual native terminal-owner anchor'
    filters[0].value = ast.copy_location(ast.Name(id='groups', ctx=ast.Load()), filters[0].value)
    scope = {'csv': csv, 'io': io, 're': re}
    exec(compile(ast.fix_missing_locations(module), __file__, 'exec'), scope)
    try:
        file_plan_cases(scope['validate_file_plan'])
    except AssertionError as error:
        assert error.args == ('D156 duplicate native file path',), \
            ('D156 unrelated native split-owner fault', error.args)
        reds += 1
    else:
        raise AssertionError('D156 native terminal-owner omission survived')
    assert Path(__file__).read_bytes() == original, 'D156 file plan source changed'
    print('file plan coverage:', result[0], 'current Python paths /', result[1],
          'owners /', result[2], 'bounded native groups /', failures, 'actual data refusals /', reds,
          'actual guard-body reds; native split positive; destination resolution/runtime pending')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--baseline', action='store_true', help='retired pre-repair snapshot; see the committed census record')
    args = parser.parse_args()
    if args.baseline:
        parser.error('pre-repair baseline is retained in docs/history/stitchcad-devnotes-part131.md; current entry capture is in python_producer_contract.py')
    main()
