"""D156: planned Python entry paths, effective activation and actual standalone prefix captures."""
from pathlib import Path
import ast
import contextlib
import io
import os
import re
import runpy
import subprocess
import sys
import tempfile
import types

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'scripts/local_environment.py'
ENTRY = ROOT / 'docs/tasks/artifacts/size_membership/size_membership_mutations.py'
EXPECTED = {'CARGO_HOME': 'target/cargo-home', 'RUSTUP_HOME': 'target/cargo-home/rustup-ci',
            'CARGO_TARGET_DIR': 'target', 'TMPDIR': 'target/scratch', 'MAKE_TMPDIR': 'target/scratch'}
GOOD = runpy.run_path(str(SOURCE))


def controls(scope):
    work = GOOD['directory'](ROOT, 'target/scratch/python_producer_contract', create=True)
    saved = dict(os.environ)
    count = 0
    try:
        with tempfile.TemporaryDirectory(dir=work) as directory:
            root = Path(directory)
            source = root / 'input'
            source.write_text('owned source')
            output = root / 'target/output'
            wanted = {name: str(root / relative) for name, relative in EXPECTED.items()}
            os.environ['RUSTUP_TOOLCHAIN'] = '1.99.0'
            os.environ['D156_RETAINED'] = 'opaque value'
            original_mkdir, observations = Path.mkdir, []

            def observe(path, *args, **kwargs):
                if path == output:
                    observations.append({name: os.environ.get(name) for name in EXPECTED})
                return original_mkdir(path, *args, **kwargs)

            Path.mkdir = observe
            try:
                scope['prepare_producer'](root, directories=('target/output',), sources=('input',))
            finally:
                Path.mkdir = original_mkdir
            assert observations == [wanted], 'D156 Python output before activation'
            assert all(os.environ[name] == value for name, value in wanted.items()), 'D156 Python effective stores'
            assert os.environ['RUSTUP_TOOLCHAIN'] == '1.99.0' and os.environ['D156_RETAINED'] == 'opaque value', 'D156 Python retained environment'
            assert source.read_text() == 'owned source' and output.is_dir(), 'D156 Python owned paths'
            count += 4
        for kind in ('external', 'source-output', 'escape', 'bytes', 'type', 'missing-source',
                     'source-link', 'source-parent-link', 'source-git', 'late-source', 'source-directory', 'source-device'):
            with tempfile.TemporaryDirectory(dir=work) as directory:
                root = Path(directory)
                (root / 'input').write_text('owned source')
                outputs, inputs = ['target/output'], ['input']
                message = None
                if kind == 'external':
                    outputs = [str(ROOT.parent / 'never-created')]
                    message = 'producer path outside workspace'
                elif kind == 'source-output':
                    outputs = ['unowned-output']
                    message = 'producer output must be under target'
                elif kind == 'escape':
                    outputs = ['target/../escape']
                    message = 'invalid producer path components'
                elif kind == 'bytes':
                    inputs = ['input\n']
                    message = 'invalid producer path bytes'
                elif kind == 'type':
                    outputs = [None]
                    message = 'invalid producer path type'
                elif kind in ('missing-source', 'late-source'):
                    inputs = ['absent'] if kind == 'missing-source' else ['input', 'absent']
                    message = 'producer source is not a local regular file'
                elif kind == 'source-directory':
                    inputs = ['data']
                    (root / 'data').mkdir()
                    message = 'producer source is not a local regular file'
                elif kind == 'source-device':
                    message = 'producer source is not a local regular file'
                elif kind == 'source-link':
                    (root / 'alias').symlink_to(root / 'input')
                    inputs = ['alias']
                    message = 'symlink producer source'
                else:
                    (root / 'data').mkdir()
                    (root / 'data/input').write_text('owned source')
                    if kind == 'source-parent-link':
                        (root / 'alias').symlink_to(root / 'data', target_is_directory=True)
                        inputs = ['alias/input']
                        message = 'symlink store component'
                    else:
                        (root / 'data/.git').write_text('synthetic boundary marker, no Git database')
                        inputs = ['data/input']
                        message = 'store crosses another Git repository'
                before = dict(os.environ)
                original_stat = Path.stat

                def changed_device(path, *args, **kwargs):
                    metadata = original_stat(path, *args, **kwargs)
                    if kind == 'source-device' and path == root / 'input':
                        return types.SimpleNamespace(st_mode=metadata.st_mode, st_dev=metadata.st_dev + 1)
                    return metadata

                try:
                    Path.stat = changed_device
                    try:
                        scope['prepare_producer'](root, directories=outputs, sources=inputs)
                    except scope['Refusal'] as error:
                        assert str(error) == message, ('D156 Python refusal identity', kind, str(error))
                    else:
                        raise AssertionError('D156 Python accepted ' + kind)
                finally:
                    Path.stat = original_stat
                assert dict(os.environ) == before and not (root / 'target').exists(), 'D156 Python refusal created stores or activated environment'
                count += 1
                marker = root / 'data/.git'
                if marker.exists():
                    marker.unlink()
        with tempfile.TemporaryDirectory(dir=work) as directory:
            output = io.StringIO()
            with contextlib.redirect_stderr(output):
                try:
                    scope['enter_producer'](Path(directory), directories=('source',))
                except SystemExit as error:
                    assert error.code == 2, 'D156 Python CLI status'
                else:
                    raise AssertionError('D156 Python CLI accepted invalid output')
            assert output.getvalue() == 'local-environment: REFUSED — producer output must be under target\n', 'D156 Python CLI message'
            count += 1
    finally:
        os.environ.clear()
        os.environ.update(saved)
    return count


def capture(text, entry_path=ENTRY, *, work_name=None, refusal=False):
    """Execute the actual prefix through its first mkdir, intercepting every dispatch/write."""
    tree = ast.parse(text)
    boundary = next(node.end_lineno for node in tree.body if isinstance(node, ast.Expr) and
                    isinstance(node.value, ast.Call) and isinstance(node.value.func, ast.Attribute) and
                    ast.unparse(node.value.func) == 'WORK.mkdir')
    prefix = '\n'.join(text.splitlines()[:boundary]) + '\n'
    saved, mkdir, run = dict(os.environ), Path.mkdir, subprocess.run
    observations = []
    diagnostic = io.StringIO()

    class Stopped(Exception):
        pass

    def observe(path, *args, **kwargs):
        expected_work = ROOT / 'target' / (work_name or entry_path.stem)
        assert path == expected_work, 'D156 Python unexpected prefix write'
        assert all(os.environ.get(name) == str(ROOT / value) for name, value in EXPECTED.items()), 'D156 Python standalone activation omitted'
        assert os.environ.get('RUSTUP_TOOLCHAIN') == '1.99.0', 'D156 Python standalone channel lost'
        observations.append(path)
        raise Stopped

    def no_child(*args, **kwargs):
        raise AssertionError('D156 Python unexpected prefix child')

    try:
        for name in EXPECTED:
            os.environ.pop(name, None)
        os.environ['RUSTUP_TOOLCHAIN'] = '1.99.0'
        Path.mkdir, subprocess.run = observe, no_child
        try:
            with contextlib.redirect_stderr(diagnostic):
                exec(compile(prefix, str(entry_path), 'exec'), {'__file__': str(entry_path), '__name__': 'entry_capture'})
        except Stopped:
            assert not refusal, 'D156 Python late-source refusal reached a write'
            pass
        except SystemExit as error:
            assert refusal and error.code == 2, 'D156 Python actual entry refusal status'
            assert diagnostic.getvalue() == 'local-environment: REFUSED — producer source is not a local regular file\n', 'D156 Python actual entry refusal message'
    finally:
        Path.mkdir, subprocess.run = mkdir, run
        os.environ.clear()
        os.environ.update(saved)
    assert len(observations) == (0 if refusal else 1), 'D156 Python standalone capture absent'


def capture_child(text, entry_path, work_name, sources, test_target, *, arguments=(), first_suffix=None):
    """Observe actual native calls with custom stores; every source write is intercepted."""
    original_sources = {path: path.read_bytes() for path in sources}
    work = ROOT / 'target' / work_name
    GOOD['directory'](ROOT, work.relative_to(ROOT).as_posix(), create=True)
    base = 'target/scratch/python_producer_contract/child-stores'
    custom = dict(zip(('CARGO_HOME', 'RUSTUP_HOME', 'CARGO_TARGET_DIR', 'TMPDIR'),
                      (base + '/cargo', base + '/rustup', base + '/build', base + '/scratch')))
    for value in custom.values():
        GOOD['directory'](ROOT, value, create=True)
    custom['MAKE_TMPDIR'] = custom['TMPDIR']
    saved, argv = dict(os.environ), sys.argv
    mkdir, write_text, write_bytes, run = Path.mkdir, Path.write_text, Path.write_bytes, subprocess.run
    observed = []

    class Stopped(Exception):
        pass

    def no_mkdir(path, *args, **kwargs):
        assert path == work, 'D156 Python unexpected child-capture directory'

    def no_write(path, *args, **kwargs):
        assert path in original_sources, 'D156 Python unexpected child-capture write'
        return 0

    def observe(args, *positional, **kwargs):
        environment = kwargs.get('env', os.environ)
        assert all(environment.get(name) == str(ROOT / value) for name, value in custom.items()), 'D156 Python child stores lost'
        assert environment.get('RUSTUP_TOOLCHAIN') == '1.99.0', 'D156 Python child channel lost'
        assert environment.get('RUSTUP_AUTO_INSTALL') == '0', 'D156 Python child install policy lost'
        assert environment.get('D156_PYTHON_UNRELATED') == 'preserved', 'D156 Python child unrelated environment lost'
        assert args[:6] == ['cargo', 'test', '-p', 'sc-core', '--test', test_target], 'D156 Python child native argv changed'
        if first_suffix is not None:
            wanted = list(first_suffix) if not observed else []
            assert args[6:] == wanted, 'D156 Python child test selection changed'
        assert kwargs.get('cwd') == ROOT and not positional, 'D156 Python child cwd changed'
        observed.append(args)
        raise Stopped

    try:
        os.environ.update({name: str(ROOT / value) for name, value in custom.items()})
        os.environ.update(RUSTUP_TOOLCHAIN='1.99.0', RUSTUP_AUTO_INSTALL='0',
                          D156_PYTHON_UNRELATED='preserved')
        sys.argv = [str(entry_path), *arguments]
        Path.mkdir, Path.write_text, Path.write_bytes, subprocess.run = no_mkdir, no_write, no_write, observe
        try:
            with contextlib.redirect_stdout(io.StringIO()):
                exec(compile(text, str(entry_path), 'exec'), {'__file__': str(entry_path), '__name__': 'child_capture'})
        except Stopped:
            pass
    finally:
        Path.mkdir, Path.write_text, Path.write_bytes, subprocess.run = mkdir, write_text, write_bytes, run
        sys.argv = argv
        os.environ.clear()
        os.environ.update(saved)
        assert all(path.read_bytes() == data for path, data in original_sources.items()), 'D156 Python child capture changed source'
    assert len(observed) == 2, 'D156 Python child capture/restoration absent'


def classifier_controls(text, entry_path, *, strict=False):
    """Compile only the actual classifier; no profile, native dispatch or source write."""
    tree = ast.parse(text)
    functions = [node for node in tree.body if isinstance(node, ast.FunctionDef) and node.name == 'assertion_failure']
    assert len(functions) == 1, 'D159 normalization classifier missing'
    scope = {'re': re}
    exec(compile(ast.Module(body=functions, type_ignores=[]), str(entry_path), 'exec'), scope)
    classify = scope['assertion_failure']
    accepted = b'\nfailures:\nthread panicked:\nassertion failed: operands differ\n\nfailures:\nreal_test\n'
    noises = (
        b'test assertion_name ... ok\n\nfailures:\nthread panicked:\nexpect-only\n\nfailures:\nreal_test\n',
        b'\nfailures:\nthread panicked:\nexpect-only\n\nfailures:\nassertion_name\n',
        b'assertion failed: compiler diagnostic\n',
        b'\nfailures:\nthread panicked:\nexpect-only\n',
    )
    if strict:
        helpers = [node for node in tree.body if isinstance(node, ast.FunctionDef) and node.name == 'assertion_sites']
        assert len(helpers) == 1, 'D160 assertion-site producer absent'
        exec(compile(ast.Module(body=helpers, type_ignores=[]), str(entry_path), 'exec'), scope)
        contract = 'fn fixture() {\n    assert!(false, "custom");\n    let _ = None::<()>.expect("assertion: expect-only");\n}\n'
        path = 'crates/sc-core/tests/authored_assertion_fixture.rs'
        sites = scope['assertion_sites'](contract, path)
        assert sites == {(path, 2, 5)}, 'D160 assertion sites changed'
        scope['ASSERTION_SITES'] = sites
        location = b"\nfailures:\nthread 'fixture' panicked at " + path.encode()
        accepted = location + b':2:5:\nassertion: custom message\n\nfailures:\nfixture\n'
        noises += (location + b':3:13:\nassertion: expect-only\n\nfailures:\nfixture\n',
                   location + b':2:6:\nassertion: wrong column\n\nfailures:\nfixture\n',
                   location.replace(path.encode(), b'crates/sc-core/tests/other.rs') + b':2:5:\nassertion failed: wrong source\n\nfailures:\nfixture\n',
                   location.replace(path.encode(), b'\xff') + b':2:5:\nassertion failed: invalid path bytes\n\nfailures:\nfixture\n')
    assert classify(accepted), 'D159 actual failed-body assertion refused'
    for noise in noises:
        assert not classify(noise), 'D159 assertion evidence accepted noise'


def main():
    original, entry = SOURCE.read_bytes(), ENTRY.read_bytes()
    count = controls(GOOD)
    faults = (
        ('output area', "require(Path(relative).parts[:1] == ('target',), 'producer output must be under target')", 'pass'),
        ('source link', "require(not path.is_symlink(), 'symlink producer source')", 'pass'),
        ('source parent', 'directory(root, Path(relative).parent.as_posix())', 'pass'),
        ('source device', 'path.is_file() and path.stat().st_dev == root.stat().st_dev', 'path.is_file()'),
        ('early stores', 'environment_for(root, os.environ, create=False)', 'environment_for(root, os.environ)'),
        ('activation', 'os.environ.update(environment)', 'pass'),
    )
    red = 0
    for name, before, after in faults:
        assert original.decode().count(before) == 1, (name, 'actual anchor')
        scope = {'__file__': str(SOURCE), '__name__': 'producer_fault'}
        exec(compile(original.decode().replace(before, after, 1), str(SOURCE), 'exec'), scope)
        try:
            controls(scope)
        except AssertionError as error:
            assert str(error).startswith('D156 Python'), (name, 'unrelated failure', str(error))
            red += 1
        else:
            raise AssertionError((name, 'body fault survived'))
    adopters = (
        (ENTRY, 'size_membership_mutations'),
        (ROOT / 'docs/tasks/artifacts/ease/ease_mutations.py', 'ease_mutations'),
        (ROOT / 'docs/tasks/artifacts/ease/ease_set_mutations.py', 'ease_set_mutations'),
        (ROOT / 'docs/tasks/artifacts/measurement_table/table_mutations.py', 'measurement_table_mutations'),
        (ROOT / 'docs/tasks/artifacts/size_chart/size_chart_mutations.py', 'size_chart_mutations'),
        (ROOT / 'docs/tasks/artifacts/size_chart_collection/size_chart_collection_mutations.py', 'size_chart_collection_mutations'),
        (ROOT / 'docs/tasks/artifacts/mtm_chart/mtm_chart_mutations.py', 'mtm_chart_mutations'),
        (ROOT / 'docs/tasks/artifacts/formula_structure/round_mutations.py', 'formula_round_mutations'),
        (ROOT / 'docs/tasks/artifacts/formula_structure/unsigned_round_mutations.py', 'formula_unsigned_round_mutations'),
        (ROOT / 'docs/tasks/artifacts/formula_structure/length_operator_mutations.py', 'length_operator_mutations'),
        (ROOT / 'docs/tasks/artifacts/formula_structure/domain_context_mutations.py', 'domain_context_mutations'),
        (ROOT / 'docs/tasks/artifacts/formula_lex/formula_lex_mutations.py', 'formula_lex_mutations'),
        (ROOT / 'docs/tasks/artifacts/formula_structure/semantic_mutations.py', 'semantic_mutations'),
        (ROOT / 'docs/tasks/artifacts/formula_structure/recipe_mutations.py', 'recipe_mutations'),
        (ROOT / 'docs/tasks/artifacts/formula_structure/statement_mutations.py', 'statement_mutations'),
        (ROOT / 'docs/tasks/artifacts/formula_structure/literal_normalization_mutations.py', 'formula_literal_mutations'),
        (ROOT / 'docs/tasks/artifacts/formula_structure/normalized_expression_mutations.py', 'formula_normalized_mutations'),
        (ROOT / 'docs/tasks/artifacts/formula_structure/normalized_recipe_mutations.py', 'normalized_recipe_mutations'),
        (ROOT / 'docs/tasks/artifacts/formula_structure/canonical_recipe_mutations.py', 'canonical_recipe_mutations'),
        (ROOT / 'docs/tasks/artifacts/formula_structure/canonical_expression_mutations.py', 'canonical_expression_mutations'),
        (ROOT / 'docs/tasks/artifacts/formula_structure/checked_expression_mutations.py', 'checked_expression_mutations'),
        (ROOT / 'docs/tasks/artifacts/formula_structure/checked_statement_mutations.py', 'checked_statement_mutations'),
        (ROOT / 'docs/tasks/artifacts/formula_structure/checked_recipe_mutations.py', 'checked_recipe_mutations'),
        (ROOT / 'docs/tasks/artifacts/formula_structure/namespace_mutations.py', 'namespace_mutations'),
        (ROOT / 'docs/tasks/artifacts/formula_structure/ordered_name_mutations.py', 'ordered_name_mutations'),
    )
    originals = {path: path.read_bytes() for path, _ in adopters}
    for path, work_name in adopters:
        text = path.read_text()
        multiple_sources = {
            'domain_context_mutations.py', 'formula_lex_mutations.py', 'recipe_mutations.py',
            'normalized_recipe_mutations.py', 'canonical_expression_mutations.py',
            'checked_expression_mutations.py', 'checked_statement_mutations.py',
            'checked_recipe_mutations.py', 'namespace_mutations.py', 'ordered_name_mutations.py',
        }
        source_argument = 'SOURCES' if path.name in multiple_sources else '(SOURCE,)'
        source_anchor = 'sources=' + source_argument
        call = ("runpy.run_path(str(ROOT / 'scripts/local_environment.py'))['enter_producer'](\n"
                "    ROOT, directories=(WORK,), " + source_anchor + ")\n")
        capture(text, path, work_name=work_name)
        assert not (ROOT / 'target/scratch/python_producer_contract/absent-source').exists()
        assert text.count(source_anchor) == 1
        capture(text.replace(source_anchor,
                             'sources=(*' + source_argument + ", ROOT / 'target/scratch/python_producer_contract/absent-source')", 1),
                path, work_name=work_name, refusal=True)
        assert text.count(call) == 1
        try:
            capture(text.replace(call, '', 1), path, work_name=work_name)
        except AssertionError as error:
            assert str(error) == 'D156 Python standalone activation omitted'
            red += 1
        else:
            raise AssertionError('D156 Python standalone body fault survived')
    lexer = ROOT / 'docs/tasks/artifacts/formula_lex/formula_lex_mutations.py'
    semantic = ROOT / 'docs/tasks/artifacts/formula_structure/semantic_mutations.py'
    lexer_sources = (ROOT / 'crates/sc-core/src/recipe/lexer.rs', ROOT / 'crates/sc-core/src/name.rs')
    semantic_sources = (ROOT / 'crates/sc-core/src/recipe/semantic.rs',)
    capture_child(lexer.read_text(), lexer, 'formula_lex_mutations', lexer_sources, 'formula_lex_contract')
    capture_child(semantic.read_text(), semantic, 'semantic_mutations', semantic_sources, 'formula_semantic_contract')
    text = semantic.read_text()
    before = 'environment = dict(os.environ)'
    assert text.count(before) == 1
    try:
        capture_child(text.replace(before, "environment = dict(os.environ, CARGO_HOME=str(ROOT / 'target/cargo-home'))", 1),
                      semantic, 'semantic_mutations', semantic_sources, 'formula_semantic_contract')
    except AssertionError as error:
        assert str(error) == 'D156 Python child stores lost'
        red += 1
    else:
        raise AssertionError('D156 Python actual child-store reset survived')
    for name, files, target in (
            ('recipe', ('ordered.rs', 'statement.rs'), 'formula_recipe_contract'),
            ('statement', ('statement.rs',), 'formula_statement_contract'),
            ('normalized_recipe', ('statement.rs', 'ordered.rs', 'normalized_recipe.rs'), 'formula_normalized_recipe_contract'),
            ('canonical_recipe', ('canonical_recipe.rs',), 'formula_canonical_recipe_contract')):
        path = ROOT / f'docs/tasks/artifacts/formula_structure/{name}_mutations.py'
        sources = tuple(ROOT / 'crates/sc-core/src/recipe' / file for file in files)
        text = path.read_text()
        capture_child(text, path, name + '_mutations', sources, target, first_suffix=())
        assert text.count(before) == 1
        try:
            capture_child(text.replace(before, "environment = dict(os.environ, CARGO_HOME=str(ROOT / 'target/cargo-home'))", 1),
                          path, name + '_mutations', sources, target)
        except AssertionError as error:
            assert str(error) == 'D156 Python child stores lost'
            red += 1
        else:
            raise AssertionError('D156 Python actual child-store reset survived')
    for filename, work_name, rust, target, test in (
            ('literal_normalization_mutations.py', 'formula_literal_mutations', 'literal.rs',
             'formula_literal_contract', 'independent_fraction_fixtures_cover_conversion_width_and_domain'),
            ('normalized_expression_mutations.py', 'formula_normalized_mutations', 'normalized.rs',
             'formula_normalized_contract', 'authored_shapes_match_reference_kind_order_nodes_and_depth'),
            ('canonical_expression_mutations.py', 'canonical_expression_mutations', 'canonical.rs',
             'formula_canonical_contract', None),
            ('checked_expression_mutations.py', 'checked_expression_mutations', 'checked.rs',
             'formula_checked_expression_contract', None),
            ('checked_statement_mutations.py', 'checked_statement_mutations',
             ('checked_statement.rs', 'checked.rs', 'namespace/ordered.rs'),
             'formula_checked_statement_contract', None),
            ('checked_recipe_mutations.py', 'checked_recipe_mutations', 'checked_recipe.rs',
             'formula_checked_recipe_contract', None),
            ('namespace_mutations.py', 'namespace_mutations', 'namespace.rs',
             'formula_namespace_contract', None),
            ('ordered_name_mutations.py', 'ordered_name_mutations', ('namespace.rs', 'namespace/ordered.rs'),
             'formula_ordered_names_contract', None)):
        path = ROOT / 'docs/tasks/artifacts/formula_structure' / filename
        rust_files = rust if isinstance(rust, tuple) else (rust,)
        sources = tuple(ROOT / 'crates/sc-core/src/recipe' / file for file in rust_files)
        text = path.read_text()
        selected = (test, '--', '--exact') if test is not None else ()
        capture_child(text, path, work_name, sources, target, first_suffix=selected)
        classifier_controls(text, path, strict=test is None)
        tree = ast.parse(text)
        function = next(node for node in tree.body if isinstance(node, ast.FunctionDef) and node.name == 'assertion_failure')
        actual = ast.get_source_segment(text, function)
        broad = "def assertion_failure(output):\n    return b'assertion' in output"
        assert text.count(actual) == 1
        try:
            classifier_controls(text.replace(actual, broad, 1), path, strict=test is None)
        except AssertionError as error:
            assert str(error) == 'D159 assertion evidence accepted noise'
            red += 1
        else:
            raise AssertionError('D159 actual broad classifier survived')
        if test is None:
            permissive = "def assertion_failure(output):\n    parts = output.split(b'\\nfailures:\\n', 2)\n    return len(parts) == 3 and re.search(rb'(?m)^assertion(?:[ :`]|$)', parts[1]) is not None"
            try:
                classifier_controls(text.replace(actual, permissive, 1), path, strict=True)
            except AssertionError as error:
                assert str(error) == 'D159 assertion evidence accepted noise'
                red += 1
            else:
                raise AssertionError('D160 actual expect-label classifier fault survived')
            helper = next(node for node in tree.body if isinstance(node, ast.FunctionDef) and node.name == 'assertion_sites')
            helper_text = ast.get_source_segment(text, helper)
            assert helper_text.count('if match is not None:') == 1
            omitted = helper_text.replace('if match is not None:', 'if False:', 1)
            try:
                classifier_controls(text.replace(helper_text, omitted, 1), path, strict=True)
            except AssertionError as error:
                assert str(error) == 'D160 assertion sites changed'
                red += 1
            else:
                raise AssertionError('D160 actual assertion-site omission survived')
        assert text.count(before) == 1
        try:
            capture_child(text.replace(before, "environment = dict(os.environ, CARGO_HOME=str(ROOT / 'target/cargo-home'))", 1),
                          path, work_name, sources, target)
        except AssertionError as error:
            assert str(error) == 'D156 Python child stores lost'
            red += 1
        else:
            raise AssertionError('D156 Python normalization child-store reset survived')
    recipe = ROOT / 'docs/tasks/artifacts/formula_structure/recipe_mutations.py'
    capture_child(recipe.read_text(), recipe, 'recipe_mutations',
                  (ROOT / 'crates/sc-core/src/recipe/ordered.rs', ROOT / 'crates/sc-core/src/recipe/statement.rs'),
                  'formula_recipe_contract', arguments=('--coupled',),
                  first_suffix=('coupled_token_boundaries_preserve_zero_gap_and_all_whitespace',))
    text = recipe.read_text()
    selector = "selected = coupled_cases if sys.argv[1:] == ['--coupled'] else [(*case, '') for case in cases]"
    assert text.count(selector) == 1
    try:
        capture_child(text.replace(selector, "selected = [(*case, '') for case in cases]", 1),
                      recipe, 'recipe_mutations',
                      (ROOT / 'crates/sc-core/src/recipe/ordered.rs', ROOT / 'crates/sc-core/src/recipe/statement.rs'),
                      'formula_recipe_contract', arguments=('--coupled',),
                      first_suffix=('coupled_token_boundaries_preserve_zero_gap_and_all_whitespace',))
    except AssertionError as error:
        assert str(error) == 'D156 Python child test selection changed'
        red += 1
    else:
        raise AssertionError('D156 Python actual coupled-selection fault survived')
    canonical = ROOT / 'docs/tasks/artifacts/formula_structure/canonical_expression_mutations.py'
    text = canonical.read_text()
    canonical_sources = (ROOT / 'crates/sc-core/src/recipe/canonical.rs',)
    capture_child(text, canonical, 'canonical_expression_mutations', canonical_sources,
                  'formula_canonical_contract', arguments=('--coupled',), first_suffix=('coupled_',))
    selector = "command.append('coupled_')"
    assert text.count(selector) == 1
    try:
        capture_child(text.replace(selector, "command.append('')", 1), canonical,
                      'canonical_expression_mutations', canonical_sources, 'formula_canonical_contract',
                      arguments=('--coupled',), first_suffix=('coupled_',))
    except AssertionError as error:
        assert str(error) == 'D156 Python child test selection changed'
        red += 1
    else:
        raise AssertionError('D156 Python actual canonical coupled-selection fault survived')
    assert SOURCE.read_bytes() == original and ENTRY.read_bytes() == entry, 'D156 Python source changed'
    assert all(path.read_bytes() == data for path, data in originals.items()), 'D156 Python adopter source changed'
    print('Python producer controls: ' + str(count) + ' runtime cases / ' + str(red) +
          ' actual body reds / ' + str(len(adopters)) + ' actual standalone pre-write captures / ' +
          str(len(adopters)) + ' actual late-source refusals / 16 actual native-child capture cases / 8 calibrated failed-body classifiers / source unchanged')


if __name__ == '__main__':
    main()
