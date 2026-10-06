"""D156: planned Python entry paths, effective activation and actual standalone prefix captures."""
from pathlib import Path
import ast
import contextlib
import io
import json
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
            before = dict(os.environ)
            assert scope['plan_producer'](root, directories=('target/output',), sources=('input',)) == \
                ('target/output',), 'D156 Python planned output identity'
            assert dict(os.environ) == before and not (root / 'target').exists(), \
                'D156 Python inspection created stores or activated environment'
            count += 2
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
                        return types.SimpleNamespace(st_mode=metadata.st_mode, st_dev=metadata.st_dev + 1,
                                                     st_nlink=metadata.st_nlink)
                    return metadata

                try:
                    Path.stat = changed_device
                    try:
                        scope['plan_producer'](root, directories=outputs, sources=inputs)
                    except scope['Refusal'] as error:
                        assert str(error) == message, ('D156 Python plan refusal identity', kind, str(error))
                    else:
                        raise AssertionError('D156 Python plan accepted ' + kind)
                    assert dict(os.environ) == before and not (root / 'target').exists(), \
                        'D156 Python plan refusal created stores or activated environment'
                    try:
                        scope['prepare_producer'](root, directories=outputs, sources=inputs)
                    except scope['Refusal'] as error:
                        assert str(error) == message, ('D156 Python refusal identity', kind, str(error))
                    else:
                        raise AssertionError('D156 Python accepted ' + kind)
                finally:
                    Path.stat = original_stat
                assert dict(os.environ) == before and not (root / 'target').exists(), 'D156 Python refusal created stores or activated environment'
                count += 2
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


def output_controls(scope):
    """Real isolated leaf aliases and regular-file cases; no destination is written."""
    work = GOOD['directory'](ROOT, 'target/scratch/python_producer_output_contract', create=True)
    passed = 0
    for case in ('missing', 'regular', 'alias', 'dangling', 'directory', 'device', 'parent-link',
                 'parent-git', 'outside', 'outside-target', 'escape', 'type', 'bytes'):
        with tempfile.TemporaryDirectory(dir=work) as name:
            root = Path(name)
            (root / 'input').write_text('owned input')
            parent = root / 'target/output'
            parent.mkdir(parents=True)
            victim = root / 'victim'
            victim.write_text('unchanged victim')
            destination = parent / 'result.log'
            wanted, marker = None, None
            if case == 'regular':
                destination.write_text('unchanged output')
            elif case in ('alias', 'dangling'):
                destination.symlink_to(victim if case == 'alias' else root / 'absent')
                wanted = 'symlink producer output'
            elif case == 'directory':
                destination.mkdir()
                wanted = 'producer output is not a local regular file'
            elif case == 'device':
                destination.write_text('unchanged output')
                wanted = 'producer output is not a local regular file'
            elif case == 'parent-link':
                alias = root / 'target/alias'
                alias.symlink_to(parent, target_is_directory=True)
                destination = alias / 'result.log'
                wanted = 'symlink store component'
            elif case == 'parent-git':
                nested = parent / 'nested'
                nested.mkdir()
                marker = nested / '.git'
                marker.write_text('boundary fixture; never a Git database')
                destination = nested / 'result.log'
                wanted = 'store crosses another Git repository'
            elif case == 'outside':
                destination = ROOT.parent / 'unowned-output.log'
                wanted = 'producer path outside workspace'
            elif case == 'outside-target':
                destination = root / 'unowned-output.log'
                wanted = 'producer output must be under target'
            elif case == 'escape':
                destination = 'target/../unowned-output.log'
                wanted = 'invalid producer path components'
            elif case == 'type':
                destination = None
                wanted = 'invalid producer path type'
            elif case == 'bytes':
                destination = 'target/output/bad\nname.log'
                wanted = 'invalid producer path bytes'
            before, stat = dict(os.environ), Path.stat

            def metadata(path, *args, **kwargs):
                value = stat(path, *args, **kwargs)
                if case == 'device' and path == destination:
                    return types.SimpleNamespace(st_mode=value.st_mode, st_dev=value.st_dev + 1,
                                                 st_nlink=value.st_nlink)
                return value

            # The directory-only plan's historical limitation is retained as calibration:
            # a leaf alias is refused only once that file destination is declared.
            if case in ('alias', 'dangling'):
                assert scope['plan_producer'](root, directories=('target/output',), sources=('input',)) == \
                    ('target/output',), 'D156 Python directory-only scope changed'
            try:
                Path.stat = metadata
                for action in ('plan_producer', 'prepare_producer') if wanted else ('plan_producer',):
                    try:
                        result = scope[action](root, directories=('target/output',),
                                               output_files=('target/output/first.log', destination),
                                               sources=('input',))
                    except scope['Refusal'] as error:
                        assert wanted and str(error) == wanted, ('D156 Python output refusal identity', case, str(error))
                    else:
                        assert wanted is None and result == ('target/output',), 'D156 Python output accepted ' + case
                    assert dict(os.environ) == before and not (root / 'target/cargo-home').exists(), \
                        'D156 Python output plan produced effects'
                    assert not (parent / 'first.log').exists() and victim.read_text() == 'unchanged victim', \
                        'D156 Python output plan wrote a destination'
                    assert (root / 'input').read_text() == 'owned input', 'D156 Python output plan changed source'
                    if case in ('regular', 'device'):
                        assert destination.read_text() == 'unchanged output', 'D156 Python output plan changed existing file'
                    passed += 1
            finally:
                Path.stat = stat
                if marker is not None and marker.exists():
                    marker.unlink()
    return passed


def mutable_controls(scope):
    """Physical inode aliases; no input/output alias is written by the producer API."""
    work = GOOD['directory'](ROOT, 'target/scratch/python_mutable_path_contract', create=True)
    saved, passed = dict(os.environ), 0
    cases = ('readonly-link', 'regular-mutable', 'output-link', 'mutable-link', 'late-mutable-link',
             'undeclared', 'outside', 'escape', 'bytes', 'type')
    try:
        for case in cases:
            with tempfile.TemporaryDirectory(dir=work) as name:
                root = Path(name)
                source, victim = root / 'input', root / 'victim'
                source.write_bytes(b'unchanged singleton source')
                victim.write_bytes(b'unchanged shared inode')
                output = root / 'target/output/result.log'
                output.parent.mkdir(parents=True)
                alias = root / 'alias'
                os.link(victim, alias)
                os.link(victim, output)
                assert victim.stat().st_nlink == 3 and \
                    {(p.stat().st_dev, p.stat().st_ino) for p in (victim, alias, output)} == \
                    {(victim.stat().st_dev, victim.stat().st_ino)}, 'D156 mutable fixture inode identity'
                sources, mutable, files, wanted = (source, alias), (), (), None
                if case == 'regular-mutable':
                    mutable = (source,)
                elif case == 'output-link':
                    files = (root / 'target/output/first.log', output)
                    wanted = 'hard-linked producer output'
                elif case in ('mutable-link', 'late-mutable-link'):
                    mutable = (alias,) if case == 'mutable-link' else (source, alias)
                    wanted = 'hard-linked mutable producer source'
                elif case == 'undeclared':
                    mutable = (victim,)
                    wanted = 'mutable producer source is not declared'
                elif case == 'outside':
                    mutable = (ROOT.parent / 'unowned-source',)
                    wanted = 'producer path outside workspace'
                elif case == 'escape':
                    mutable = ('target/../input',)
                    wanted = 'invalid producer path components'
                elif case == 'bytes':
                    mutable = ('bad\nsource',)
                    wanted = 'invalid producer path bytes'
                elif case == 'type':
                    mutable = (None,)
                    wanted = 'invalid producer path type'
                before = {p: p.read_bytes() for p in (source, victim, alias, output)}
                environment = dict(os.environ)
                for action in ('plan_producer', 'prepare_producer', 'enter_producer') if wanted else ('plan_producer',):
                    diagnostic = io.StringIO()
                    with contextlib.redirect_stderr(diagnostic):
                        try:
                            result = scope[action](root, directories=('target/planned',), sources=sources,
                                                   output_files=files, mutable_sources=mutable)
                        except scope['Refusal'] as error:
                            assert wanted and str(error) == wanted, ('D156 mutable refusal identity', case, action, str(error))
                        except SystemExit as error:
                            assert action == 'enter_producer' and wanted and error.code == 2, \
                                ('D156 mutable standalone refusal', case, error.code)
                            assert diagnostic.getvalue() == 'local-environment: REFUSED — ' + wanted + '\n', \
                                ('D156 mutable standalone diagnostic', case, diagnostic.getvalue())
                        else:
                            assert wanted is None and result == ('target/planned',), \
                                'D156 mutable plan accepted ' + case
                    assert dict(os.environ) == environment and not (root / 'target/cargo-home').exists() \
                        and not (root / 'target/planned').exists(), 'D156 mutable plan produced effects'
                    assert all(p.read_bytes() == data for p, data in before.items()) and \
                        not (output.parent / 'first.log').exists(), 'D156 mutable plan changed an alias byte'
                    passed += 1
    finally:
        os.environ.clear()
        os.environ.update(saved)
    return passed


def capture(text, entry_path=ENTRY, *, work_name=None, refusal=False):
    """Execute the actual prefix through its first mkdir, intercepting every dispatch/write."""
    tree = ast.parse(text)
    boundary = next(node.end_lineno for node in tree.body if isinstance(node, ast.Expr) and
                    isinstance(node.value, ast.Call) and isinstance(node.value.func, ast.Attribute) and
                    ast.unparse(node.value.func) == 'WORK.mkdir')
    prefix = '\n'.join(text.splitlines()[:boundary]) + '\n'
    saved, mkdir, run = dict(os.environ), Path.mkdir, subprocess.run
    read_text, read_bytes = Path.read_text, Path.read_bytes
    observations, reads = [], []
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

    def observe_read(method, path, *args, **kwargs):
        assert all(os.environ.get(name) == str(ROOT / value) for name, value in EXPECTED.items()), 'D156 Python standalone activation omitted'
        reads.append(path)
        return method(path, *args, **kwargs)

    try:
        for name in EXPECTED:
            os.environ.pop(name, None)
        os.environ['RUSTUP_TOOLCHAIN'] = '1.99.0'
        Path.mkdir, subprocess.run = observe, no_child
        Path.read_text = lambda path, *a, **k: observe_read(read_text, path, *a, **k)
        Path.read_bytes = lambda path, *a, **k: observe_read(read_bytes, path, *a, **k)
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
        Path.read_text, Path.read_bytes = read_text, read_bytes
        os.environ.clear()
        os.environ.update(saved)
    assert len(observations) == (0 if refusal else 1), 'D156 Python standalone capture absent'
    assert not refusal or not reads, 'D156 Python late-source refusal read an input'


def domain_fixture():
    """Authored five-source/two-consumer fixture, independent of native CASES."""
    mutable = tuple(ROOT / name for name in (
        'crates/sc-units/src/error.rs', 'crates/sc-units/src/length.rs',
        'crates/sc-units/src/ratio.rs', 'crates/sc-core/src/ontology/range.rs',
        'crates/sc-core/src/ontology/topology.rs'))
    unit = ROOT / 'crates/sc-units/tests/domain_context_contract.rs'
    work = ROOT / 'target/domain_context_mutations'
    bindings = (0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 2, 3, 3, 4)
    files = tuple(work / ('mutation-%d.log' % i) for i in range(1, 15)) + \
        (work / 'restored-units.log', work / 'restored-core.log')
    return mutable, unit, work, bindings, files


def mixed_fixture(entry_path, *, classifier_only=False, coupled=False):
    """Authored source/input/consumer order, including read-only classifier modes."""
    assert not classifier_only or entry_path.name in {
        'semantic_mutations.py', 'recipe_mutations.py', 'statement_mutations.py',
        'literal_normalization_mutations.py', 'normalized_expression_mutations.py',
        'normalized_recipe_mutations.py', 'canonical_recipe_mutations.py', 'canonical_expression_mutations.py',
        'checked_expression_mutations.py', 'checked_statement_mutations.py', 'checked_recipe_mutations.py', 'namespace_mutations.py', 'ordered_name_mutations.py', 'declaration_mutations.py', 'name_read_mutations.py', 'builtin_signature_mutations.py', 'operator_signature_mutations.py', 'wanted_signature_mutations.py', 'call_lookup_mutations.py', 'formula_expression_mutations.py'
    , 'reduction_boundary_mutations.py', 'recipe_input_review_mutations.py'}, 'D156 unexpected classifier mode'
    assert not coupled or entry_path.name in {'recipe_mutations.py', 'canonical_expression_mutations.py'}, 'D156 unexpected coupled mode'
    if entry_path == ROOT / 'docs/tasks/artifacts/formula_structure/domain_context_mutations.py':
        mutable, contract, work, bindings, files = domain_fixture()
        units = ['cargo', 'test', '-p', 'sc-units', '--test', 'domain_context_contract']
        core = ['cargo', 'test', '-p', 'sc-core', '--lib', 'domain_context_contracts']
        contexts = ('units',) * 11 + ('core',) * 3
        calls = tuple(units if context == 'units' else core for context in contexts) + (units, core)
        extras = ()
        macro_sources = (contract, mutable[3], mutable[4])
        initial_reads = tuple(mutable[i] for i in bindings) + macro_sources
    elif entry_path == ROOT / 'docs/tasks/artifacts/formula_lex/formula_lex_mutations.py':
        mutable = (ROOT / 'crates/sc-core/src/recipe/lexer.rs', ROOT / 'crates/sc-core/src/name.rs')
        contract = ROOT / 'crates/sc-core/tests/formula_lex_contract.rs'
        work = ROOT / 'target/formula_lex_mutations'
        bindings = (0, 0, 1, 0, 0, 0, 0, 0, 0)
        contexts = ('unicode_preflight_refuses_before_any_valid_prefix_token',
                    'only_three_keywords_are_reserved_and_spelling_matches_machine_tokens',
                    'only_three_keywords_are_reserved_and_spelling_matches_machine_tokens',
                    'paired_comparisons_use_longest_match', 'decimals_require_nonempty_fraction',
                    'binding_keeps_exact_roles_text_and_byte_positions',
                    'end_and_first_error_are_permanently_fused', 'debug_and_errors_do_not_dump_customer_source',
                    'whitespace_gaps_preserve_later_unit_separator_authority')
        base = ['cargo', 'test', '-p', 'sc-core', '--test', 'formula_lex_contract']
        calls = tuple(base + [test, '--', '--exact'] for test in contexts) + (base,)
        files = tuple(work / ('mutation-%d.log' % i) for i in range(1, 10)) + (work / 'restored.log',)
        extras = (ROOT / 'docs/book/src/spec/formula-language/examples.md',)
        macro_sources = (contract,)
        initial_reads = (*mutable, contract)
    elif entry_path == ROOT / 'docs/tasks/artifacts/formula_structure/recipe_mutations.py':
        mutable = (ROOT / 'crates/sc-core/src/recipe/ordered.rs', ROOT / 'crates/sc-core/src/recipe/statement.rs')
        contract = ROOT / 'crates/sc-core/tests/formula_recipe_contract.rs'
        work = ROOT / 'target/recipe_mutations'
        bindings = (1, 0, 0, 1, 0) if coupled else (0,) * 11 + (1,) * 4
        boundary = 'coupled_token_boundaries_preserve_zero_gap_and_all_whitespace'
        provenance = 'coupled_later_header_and_operand_rules_keep_exact_global_provenance'
        maximum = 'coupled_all_statement_and_expression_limits_reach_their_maximum_together'
        contexts = (boundary, provenance, provenance, provenance, maximum) if coupled else ('',) * 15
        base = ['cargo', 'test', '-p', 'sc-core', '--test', 'formula_recipe_contract']
        calls = tuple(base + ([test] if test else []) for test in contexts) + (base,)
        label = 'coupled' if coupled else 'recipe'
        files = tuple(work / ('%s-fault-%d.log' % (label, i)) for i in range(1, len(bindings) + 1)) + (work / 'restored.log',)
        extras = tuple(ROOT / ('docs/tasks/artifacts/formula_structure/' + name) for name in (
            'recipe_cases.tsv', 'recipe_coupled_cases.tsv', 'canonical_worked_cases.tsv')) + \
            (ROOT / 'docs/book/src/spec/formula-language/examples.md',)
        macro_sources = (contract,)
        initial_reads = (*mutable, contract)
    elif entry_path == ROOT / 'docs/tasks/artifacts/formula_structure/literal_normalization_mutations.py':
        mutable = (ROOT / 'crates/sc-core/src/recipe/literal.rs',)
        contract = ROOT / 'crates/sc-core/tests/formula_literal_contract.rs'
        work = ROOT / 'target/formula_literal_mutations'
        bindings = (0,) * 13
        fractions = 'independent_fraction_fixtures_cover_conversion_width_and_domain'
        kinds = 'kinds_quanta_and_angle_turns_are_preserved'
        widths = 'width_checks_precede_rounding_and_length_checks_follow_it'
        contexts = (fractions, kinds, kinds, kinds, kinds, fractions,
                    'huge_spelling_refuses_or_reduces_without_new_language_caps',
                    fractions, fractions, widths, widths, fractions,
                    'source_location_privacy_and_unary_identity_survive_conversion')
        base = ['cargo', 'test', '-p', 'sc-core', '--test', 'formula_literal_contract']
        calls = tuple(base + [test, '--', '--exact'] for test in contexts) + (base,)
        files = tuple(work / ('mutation-%d.log' % i) for i in range(1, 14)) + (work / 'restored.log',)
        extras = tuple(ROOT / ('docs/tasks/artifacts/formula_structure/' + name) for name in (
            'literal_normalization_cases.tsv', 'reduction_boundary_cases.tsv'))
        macro_sources = (contract,)
        initial_reads = (*mutable, contract)
    elif entry_path == ROOT / 'docs/tasks/artifacts/formula_structure/normalized_expression_mutations.py':
        mutable = (ROOT / 'crates/sc-core/src/recipe/normalized.rs',)
        contract = ROOT / 'crates/sc-core/tests/formula_normalized_contract.rs'
        work = ROOT / 'target/formula_normalized_mutations'
        bindings = (0,) * 17
        shape = 'authored_shapes_match_reference_kind_order_nodes_and_depth'
        arguments = 'argument_views_are_ordered_exact_size_fused_and_arena_bound'
        contexts = (shape,) * 11 + ('normalized_arena_retains_source_spans_and_borrows_after_syntax_drop',
                    shape, 'every_literal_position_refuses_atomically_with_original_span',
                    arguments, arguments, 'every_worked_expression_normalizes_and_debug_omits_customer_content')
        base = ['cargo', 'test', '-p', 'sc-core', '--test', 'formula_normalized_contract']
        calls = tuple(base + [test, '--', '--exact'] for test in contexts) + (base,)
        files = tuple(work / ('mutation-%d.log' % i) for i in range(1, 18)) + (work / 'restored.log',)
        extras = tuple(ROOT / ('docs/tasks/artifacts/formula_structure/' + name) for name in (
            'normalized_expression_shapes.tsv', 'literal_normalization_cases.tsv')) + \
            (ROOT / 'docs/book/src/spec/formula-language/examples.md',)
        macro_sources = (contract,)
        initial_reads = (*mutable, contract)
    elif entry_path == ROOT / 'docs/tasks/artifacts/formula_structure/normalized_recipe_mutations.py':
        mutable = tuple(ROOT / ('crates/sc-core/src/recipe/' + name + '.rs') for name in (
            'statement', 'ordered', 'normalized_recipe'))
        contract = ROOT / 'crates/sc-core/tests/formula_normalized_recipe_contract.rs'
        work = ROOT / 'target/normalized_recipe_mutations'
        bindings = (0,) * 9 + (1,) * 3 + (2,) * 5
        contexts = ('',) * 17
        base = ['cargo', 'test', '-p', 'sc-core', '--test', 'formula_normalized_recipe_contract']
        calls = (base,) * 18
        files = tuple(work / ('fault-%d.log' % i) for i in range(1, 18)) + (work / 'restored.log',)
        extras = tuple(ROOT / ('docs/tasks/artifacts/formula_structure/' + name) for name in (
            'canonical_statement_cases.tsv', 'canonical_recipe_cases.tsv', 'literal_normalization_cases.tsv'))
        macro_sources = (contract,)
        initial_reads = (*mutable, contract)
    elif entry_path == ROOT / 'docs/tasks/artifacts/formula_structure/canonical_recipe_mutations.py':
        mutable = (ROOT / 'crates/sc-core/src/recipe/canonical_recipe.rs',)
        contract = ROOT / 'crates/sc-core/tests/formula_canonical_recipe_contract.rs'
        work = ROOT / 'target/canonical_recipe_mutations'
        bindings = (0,) * 21
        contexts = ('',) * 21
        base = ['cargo', 'test', '-p', 'sc-core', '--test', 'formula_canonical_recipe_contract']
        calls = (base,) * 22
        files = tuple(work / ('fault-%d.log' % i) for i in range(1, 22)) + (work / 'restored.log',)
        extras = tuple(ROOT / ('docs/tasks/artifacts/formula_structure/' + name) for name in (
            'canonical_statement_cases.tsv', 'canonical_recipe_cases.tsv',
            'canonical_expression_cases.tsv', 'literal_normalization_cases.tsv')) + (
            ROOT / 'docs/book/src/spec/formula-language/grammar.md',
            ROOT / 'docs/decisions/decision_recipe-bytes.md')
        macro_sources = (contract,)
        initial_reads = (*mutable, contract)
    elif entry_path == ROOT / 'docs/tasks/artifacts/formula_structure/canonical_expression_mutations.py':
        mutable = (ROOT / 'crates/sc-core/src/recipe/canonical.rs',)
        contract = ROOT / 'crates/sc-core/tests/formula_canonical_contract.rs'
        work = ROOT / 'target/canonical_expression_mutations'
        bindings = (0,) * (3 if coupled else 19)
        contexts = ('coupled_',) * 3 if coupled else ('',) * 19
        base = ['cargo', 'test', '-p', 'sc-core', '--test', 'formula_canonical_contract']
        calls = tuple(base + ([test] if test else []) for test in contexts) + (base,)
        files = tuple(work / ('fault-%d.log' % i) for i in range(1, len(bindings) + 1)) + (work / 'restored.log',)
        extras = tuple(ROOT / ('docs/tasks/artifacts/formula_structure/' + name) for name in (
            'canonical_expression_cases.tsv', 'literal_normalization_cases.tsv', 'canonical_worked_cases.tsv')) + (
            ROOT / 'docs/book/src/annexes/formula-literals.md',
            ROOT / 'docs/book/src/spec/formula-language/examples.md')
        macro_sources = (contract,)
        initial_reads = (*mutable, contract)
    elif entry_path == ROOT / 'docs/tasks/artifacts/formula_structure/checked_expression_mutations.py':
        mutable = (ROOT / 'crates/sc-core/src/recipe/checked.rs',)
        contract = ROOT / 'crates/sc-core/tests/formula_checked_expression_contract.rs'
        work = ROOT / 'target/checked_expression_mutations'
        bindings, contexts = (0,) * 16, ('',) * 16
        base = ['cargo', 'test', '-p', 'sc-core', '--test', 'formula_checked_expression_contract']
        calls = tuple(base for _ in range(17))
        files = tuple(work / ('fault-%d.log' % i) for i in range(1, 17)) + (work / 'restored.log',)
        extras, macro_sources, initial_reads = (), (contract,), (*mutable, contract)
    elif entry_path == ROOT / 'docs/tasks/artifacts/formula_structure/ordered_name_mutations.py':
        mutable = (ROOT / 'crates/sc-core/src/recipe/namespace.rs',
                   ROOT / 'crates/sc-core/src/recipe/namespace/ordered.rs')
        contract = ROOT / 'crates/sc-core/tests/formula_ordered_names_contract.rs'
        work = ROOT / 'target/ordered_name_mutations'
        bindings, contexts = (1,) * 16 + (0,) * 2, ('',) * 18
        base = ['cargo', 'test', '-p', 'sc-core', '--test', 'formula_ordered_names_contract']
        calls = (base,) * 19
        files = tuple(work / ('fault-%d.log' % i) for i in range(1, 19)) + (work / 'restored.log',)
        extras, macro_sources, initial_reads = (), (contract,), (*mutable, contract)
    elif entry_path == ROOT / 'docs/tasks/artifacts/formula_structure/declaration_mutations.py':
        mutable = (ROOT / 'crates/sc-core/src/recipe/declaration.rs',)
        contract = ROOT / 'crates/sc-core/tests/formula_declaration_contract.rs'
        work = ROOT / 'target/declaration_mutations'
        bindings, contexts = (0,) * 20, ('',) * 20
        base = ['cargo', 'test', '-p', 'sc-core', '--test', 'formula_declaration_contract']
        docs = ['cargo', 'test', '-p', 'sc-core', '--doc', 'recipe::declaration::FormulaDeclaration']
        calls = (base,) * 19 + (docs, base, docs)
        files = tuple(work / ('fault-%d.log' % i) for i in range(1, 20)) + (
            work / 'scalar-domain-compile-fail.log', work / 'restored.log', work / 'restored-docs.log')
        extras, macro_sources, initial_reads = (), (contract,), (*mutable, contract)
    elif entry_path == ROOT / 'docs/tasks/artifacts/formula_structure/namespace_mutations.py':
        mutable = (ROOT / 'crates/sc-core/src/recipe/namespace.rs',)
        contract = ROOT / 'crates/sc-core/tests/formula_namespace_contract.rs'
        work = ROOT / 'target/namespace_mutations'
        bindings, contexts = (0,) * 17, ('',) * 17
        base = ['cargo', 'test', '-p', 'sc-core', '--test', 'formula_namespace_contract']
        calls = (base,) * 18
        files = tuple(work / ('fault-%d.log' % i) for i in range(1, 18)) + (work / 'restored.log',)
        extras, macro_sources, initial_reads = (), (contract,), (*mutable, contract)
    elif entry_path == ROOT / 'docs/tasks/artifacts/formula_structure/builtin_signature_mutations.py':
        mutable = (ROOT / 'crates/sc-core/src/recipe/builtins.rs',)
        contract = ROOT / 'crates/sc-core/tests/formula_builtin_signature_contract.rs'
        work = ROOT / 'target/builtin_signature_mutations'
        bindings, contexts = (0,) * 21, ('',) * 21
        base = ['cargo', 'test', '-p', 'sc-core', '--test', 'formula_builtin_signature_contract']
        calls = (base,) * 22
        files = tuple(work / ('fault-%d.log' % i) for i in range(1, 22)) + (work / 'restored.log',)
        extras = (ROOT / 'docs/book/src/spec/formula-language/grammar.md',)
        macro_sources, initial_reads = (contract,), (*mutable, contract)
    elif entry_path == ROOT / 'docs/tasks/artifacts/formula_structure/operator_signature_mutations.py':
        mutable = (ROOT / 'crates/sc-core/src/recipe/operators.rs',)
        contract = ROOT / 'crates/sc-core/tests/formula_operator_signature_contract.rs'
        work = ROOT / 'target/operator_signature_mutations'
        bindings, contexts = (0,) * 14, ('',) * 14
        base = ['cargo', 'test', '-p', 'sc-core', '--test', 'formula_operator_signature_contract']
        calls = (base,) * 15
        files = tuple(work / ('fault-%d.log' % i) for i in range(1, 15)) + (work / 'restored.log',)
        extras = (ROOT / 'docs/book/src/spec/formula-language/grammar.md',)
        macro_sources, initial_reads = (contract,), (*mutable, contract)
    elif entry_path == ROOT / 'docs/tasks/artifacts/formula_structure/call_lookup_mutations.py':
        mutable = (ROOT / 'crates/sc-core/src/recipe/calls.rs',)
        contract = ROOT / 'crates/sc-core/tests/formula_call_lookup_contract.rs'
        work = ROOT / 'target/call_lookup_mutations'
        bindings, contexts = (0,) * 18, ('',) * 18
        base = ['cargo', 'test', '-p', 'sc-core', '--test', 'formula_call_lookup_contract']
        calls = (base,) * 19
        files = tuple(work / ('fault-%d.log' % i) for i in range(1, 19)) + (work / 'restored.log',)
        extras, macro_sources, initial_reads = (), (contract,), (*mutable, contract)
    elif entry_path == ROOT / 'docs/tasks/artifacts/formula_structure/recipe_input_review_mutations.py':
        mutable = tuple(ROOT / 'crates/sc-core/src/recipe' / name for name in
                        ('statement.rs', 'ordered.rs', 'canonical_recipe.rs', 'normalized_recipe.rs'))
        contract = ROOT / 'crates/sc-core/tests/formula_recipe_input_review.rs'
        work = ROOT / 'target/recipe_input_review_mutations'
        bindings = (0, 1, 1, 2, 2, 1, 3)
        worked = 'actual_worked_population_retains_all_headers_global_locations_and_complete_identity'
        refusal = 'later_input_refusals_abort_whole_identity_even_in_untaken_branches_and_call_arguments'
        contexts = (worked, refusal, refusal, worked, worked,
                    'combined_maximum_and_each_first_excess_preserve_stage_and_later_context', worked)
        base = ['cargo', 'test', '-p', 'sc-core', '--test', 'formula_recipe_input_review']
        calls = tuple(base + [test] for test in contexts) + (base,)
        files = tuple(work / ('fault-%d.log' % i) for i in range(1, 8)) + (work / 'restored.log',)
        extras = (ROOT / 'docs/tasks/artifacts/formula_structure/canonical_worked_recipe_cases.tsv',
                  ROOT / 'docs/book/src/spec/formula-language/examples.md',
                  ROOT / 'docs/tasks/artifacts/formula_structure/recipe_refusal_stage_cases.tsv')
        macro_sources, initial_reads = (contract,), (*mutable, contract)
    elif entry_path == ROOT / 'docs/tasks/artifacts/formula_structure/reduction_boundary_mutations.py':
        mutable = (ROOT / 'crates/sc-core/src/recipe/literal.rs',)
        contract = ROOT / 'crates/sc-core/tests/formula_literal_contract.rs'
        work = ROOT / 'target/formula_reduction_mutations'
        bindings = (0,) * 4
        contexts = ('coupled_reduction_frontier_preserves_valid_inputs_and_located_width_refusals',) * 4
        base = ['cargo', 'test', '-p', 'sc-core', '--test', 'formula_literal_contract']
        calls = tuple(base + [test, '--', '--exact'] for test in contexts) + (base,)
        files = tuple(work / ('mutation-%d.log' % i) for i in range(1, 5)) + (work / 'restored.log',)
        extras = (ROOT / 'docs/tasks/artifacts/formula_structure/literal_normalization_cases.tsv',
                  ROOT / 'docs/tasks/artifacts/formula_structure/reduction_boundary_cases.tsv')
        macro_sources, initial_reads = (contract,), (*mutable, contract)
    elif entry_path == ROOT / 'docs/tasks/artifacts/formula_structure/formula_expression_mutations.py':
        mutable = (ROOT / 'crates/sc-core/src/recipe/parser.rs',)
        contract = ROOT / 'crates/sc-core/tests/formula_expression_contract.rs'
        work = ROOT / 'target/formula_expression_mutations'
        bindings = (0,) * 11
        contexts = ('semantic_node_bounds_count_calls_and_square_payload_correctly', 'conditional_depth_visits_calls_and_untaken_branches_but_not_siblings', 'number_text_and_closed_units_remain_unconverted_and_borrowed', 'all_binary_comparisons_have_separate_syntax_roles', 'square_payload_is_exact_and_other_exponents_are_unsupported', 'square_payload_is_exact_and_other_exponents_are_unsupported', 'ordinary_calls_and_conditionals_keep_every_ordered_child', 'precedence_and_left_associativity_are_normative', 'precedence_and_left_associativity_are_normative', 'precedence_and_left_associativity_are_normative', 'ordinary_calls_and_conditionals_keep_every_ordered_child')
        base = ['cargo', 'test', '-p', 'sc-core', '--test', 'formula_expression_contract']
        calls = tuple(base + [name, '--', '--exact'] for name in contexts) + (base,)
        files = tuple(work / ('mutation-%d.log' % i) for i in range(1, 12)) + (work / 'restored.log',)
        extras = (ROOT / 'docs/book/src/spec/formula-language/examples.md',
                  ROOT / 'docs/tasks/artifacts/formula_structure/expression_shapes.tsv')
        macro_sources, initial_reads = (contract,), (*mutable, contract)
    elif entry_path == ROOT / 'docs/tasks/artifacts/formula_structure/wanted_signature_mutations.py':
        mutable = (ROOT / 'crates/sc-core/src/recipe/signatures.rs',)
        contract = ROOT / 'crates/sc-core/tests/formula_operator_signature_contract.rs'
        other = ROOT / 'crates/sc-core/tests/formula_builtin_signature_contract.rs'
        work = ROOT / 'target/wanted_signature_mutations'
        bindings, contexts = (0,) * 19, ('',) * 19
        base = ['cargo', 'test', '-p', 'sc-core', '--test', 'formula_operator_signature_contract',
                '--test', 'formula_builtin_signature_contract']
        calls = (base,) * 20
        files = tuple(work / ('fault-%d.log' % i) for i in range(1, 20)) + (work / 'restored.log',)
        extras = (other, ROOT / 'docs/book/src/spec/formula-language/grammar.md')
        macro_sources, initial_reads = (contract, other), (*mutable, contract, other)
    elif entry_path == ROOT / 'docs/tasks/artifacts/formula_structure/name_read_mutations.py':
        mutable = (ROOT / 'crates/sc-core/src/recipe/namespace.rs',)
        contract = ROOT / 'crates/sc-core/tests/formula_name_read_contract.rs'
        work = ROOT / 'target/name_read_mutations'
        bindings, contexts = (0,) * 9, ('',) * 9
        base = ['cargo', 'test', '-p', 'sc-core', '--test', 'formula_name_read_contract']
        calls = (base,) * 10
        files = tuple(work / ('fault-%d.log' % i) for i in range(1, 10)) + (work / 'restored.log',)
        extras, macro_sources, initial_reads = (), (contract,), (*mutable, contract)
    elif entry_path == ROOT / 'docs/tasks/artifacts/formula_structure/checked_recipe_mutations.py':
        mutable = (ROOT / 'crates/sc-core/src/recipe/checked_recipe.rs',)
        contract = ROOT / 'crates/sc-core/tests/formula_checked_recipe_contract.rs'
        work = ROOT / 'target/checked_recipe_mutations'
        bindings, contexts = (0,) * 19, ('',) * 19
        base = ['cargo', 'test', '-p', 'sc-core', '--test', 'formula_checked_recipe_contract']
        calls = (base,) * 21
        files = (work / 'baseline.log',) + tuple(work / ('fault-%d.log' % i) for i in range(1, 20)) + (work / 'restored.log',)
        extras, macro_sources, initial_reads = (), (contract,), (*mutable, contract)
    elif entry_path == ROOT / 'docs/tasks/artifacts/formula_structure/checked_statement_mutations.py':
        mutable = tuple(ROOT / path for path in (
            'crates/sc-core/src/recipe/checked.rs',
            'crates/sc-core/src/recipe/checked_statement.rs',
            'crates/sc-core/src/recipe/namespace/ordered.rs'))
        contract = ROOT / 'crates/sc-core/tests/formula_checked_statement_contract.rs'
        work = ROOT / 'target/checked_statement_mutations'
        bindings, contexts = (1,) * 10 + (0,) * 4 + (1,) * 7 + (2,), ('',) * 22
        base = ['cargo', 'test', '-p', 'sc-core', '--test', 'formula_checked_statement_contract']
        calls = (base,) * 23
        files = tuple(work / ('fault-%d.log' % i) for i in range(1, 23)) + (work / 'restored.log',)
        extras, macro_sources, initial_reads = (), (contract,), (*mutable, contract)
    elif entry_path == ROOT / 'docs/tasks/artifacts/formula_structure/statement_mutations.py':
        mutable = (ROOT / 'crates/sc-core/src/recipe/statement.rs',)
        contract = ROOT / 'crates/sc-core/tests/formula_statement_contract.rs'
        work = ROOT / 'target/statement_mutations'
        bindings = (0,) * 15
        contexts = ('',) * 15
        base = ['cargo', 'test', '-p', 'sc-core', '--test', 'formula_statement_contract']
        calls = (base,) * 16
        files = tuple(work / ('fault-%d.log' % i) for i in range(1, 16)) + (work / 'restored.log',)
        extras = tuple(ROOT / ('docs/tasks/artifacts/formula_structure/' + name) for name in (
            'statement_cases.tsv', 'canonical_worked_cases.tsv')) + \
            (ROOT / 'docs/book/src/spec/formula-language/examples.md',)
        macro_sources = (contract,)
        initial_reads = (*mutable, contract)
    else:
        assert entry_path == ROOT / 'docs/tasks/artifacts/formula_structure/semantic_mutations.py', 'D156 unknown mixed entry'
        mutable = (ROOT / 'crates/sc-core/src/recipe/semantic.rs',)
        contract = ROOT / 'crates/sc-core/tests/formula_semantic_contract.rs'
        work = ROOT / 'target/semantic_mutations'
        bindings = (0,) * 10
        kinds = 'all_operand_kinds_and_binding_conversions_are_closed'
        reserved = 'reserved_metadata_preserves_kinds_origins_contexts_and_tolerance_roles'
        contexts = (kinds,) * 4 + ('all_source_origins_have_distinct_exact_tokens',) + (reserved,) * 4 + \
                   ('lookups_refuse_repairs_aliases_and_new_excluded_form_reservations',)
        base = ['cargo', 'test', '-p', 'sc-core', '--test', 'formula_semantic_contract']
        calls = (base,) * 11
        files = tuple(work / ('fault-%d.log' % i) for i in range(1, 11)) + (work / 'restored.log',)
        extras = (ROOT / 'docs/book/src/spec/formula-language.md',)
        macro_sources = (contract,)
        initial_reads = (*mutable, contract)
    if classifier_only:
        calls, files = (), ()
    return dict(mutable=mutable, contract=contract, work=work, bindings=bindings, files=files,
                contexts=contexts, calls=calls, extras=extras, macro_sources=macro_sources,
                initial_reads=initial_reads,
                baseline=entry_path.name == 'checked_recipe_mutations.py' and not classifier_only,
                restoration_order=[mutable[i] for i in
                                   (bindings[:-1] if entry_path.name == 'declaration_mutations.py' else bindings)] +
                                  list(mutable))


def capture_mixed_entry(text, entry_path, *, classifier_only=False, coupled=False):
    """Complete actual mixed-source body; intercepted children certify dispatch only."""
    fixture = mixed_fixture(entry_path, classifier_only=classifier_only, coupled=coupled)
    mutable, contract, work, bindings, files = (fixture[key] for key in ('mutable', 'contract', 'work', 'bindings', 'files'))
    sources = (*mutable, contract, *fixture['extras'])
    originals = {p: p.read_bytes() for p in (*sources, entry_path, SOURCE)}
    GOOD['directory'](ROOT, work.relative_to(ROOT).as_posix(), create=True)
    base = 'target/scratch/python_producer_contract/domain-stores'
    stores = {name: str(ROOT / base / suffix) for name, suffix in
              (('CARGO_HOME', 'cargo'), ('RUSTUP_HOME', 'rustup'),
               ('CARGO_TARGET_DIR', 'build'), ('TMPDIR', 'scratch'))}
    for value in stores.values():
        GOOD['directory'](ROOT, Path(value).relative_to(ROOT).as_posix(), create=True)
    scope = {'__file__': str(entry_path), '__name__': 'domain_capture'}
    saved, argv = dict(os.environ), sys.argv
    mkdir, read_text, read_bytes, write_text, write_bytes, run, load = (
        Path.mkdir, Path.read_text, Path.read_bytes, Path.write_text, Path.write_bytes,
        subprocess.run, runpy.run_path)
    planned, reads, calls, written, pending, errors, mutations, restorations = [], [], [], {}, {}, [], [], []

    def environment(values):
        assert all(values.get(k) == v for k, v in stores.items()), 'D156 measure child stores lost'
        assert values.get('MAKE_TMPDIR') == stores['TMPDIR'], 'D156 measure scratch alias lost'
        assert values.get('RUSTUP_TOOLCHAIN') == '1.99.0' and values.get('RUSTUP_AUTO_INSTALL') == '0', \
            'D156 measure toolchain changed'
        assert values.get('D156_DOMAIN_UNRELATED') == 'preserved', 'D156 measure unrelated value lost'

    def profile(path, *args, **kwargs):
        assert path == str(SOURCE) and not args and not kwargs, 'D156 measure profile entry changed'

        def enter(root, *, directories=(), output_files=(), sources=(), mutable_sources=()):
            assert root == ROOT and directories == (work,), 'D156 measure directory plan changed'
            assert output_files == files, 'D156 measure file plan changed'
            assert sources == (*mutable, contract, *fixture['extras']), 'D156 measure source plan changed'
            assert mutable_sources == (() if classifier_only else mutable), 'D156 measure mutable plan changed'
            assert not planned and not reads and not calls and not written, 'D156 measure plan was late'
            planned.append(True)
            return GOOD['enter_producer'](root, directories=directories, output_files=output_files,
                                          sources=sources, mutable_sources=mutable_sources)
        return dict(GOOD, enter_producer=enter)

    def read(path, binary, *args, **kwargs):
        assert planned == [True] and path in sources, 'D156 measure unplanned input read'
        environment(os.environ)
        reads.append(path)
        return (read_bytes if binary else read_text)(path, *args, **kwargs)

    def directory(path, *args, **kwargs):
        assert path == work and planned == [True], 'D156 measure unplanned directory write'
        environment(os.environ)

    def write(path, content, *args, **kwargs):
        assert not classifier_only, 'D156 classifier mode performed native effects'
        assert planned == [True], 'D156 measure write before plan'
        environment(os.environ)
        if path in mutable:
            if isinstance(content, bytes):
                assert content == originals[path], 'D156 measure wrong source restoration'
                restorations.append(path)
                pending.pop(path, None)
            else:
                index = len(mutations)
                assert index < len(bindings) and not pending, 'D156 measure source mutation order'
                assert path == mutable[bindings[index]], 'D156 domain mutable source order changed'
                if entry_path.name == 'recipe_mutations.py':
                    selected = scope['coupled_cases'] if coupled else [(*case, '') for case in scope['cases']]
                    _, relative, before, after, family = selected[index]
                elif entry_path.name == 'recipe_input_review_mutations.py':
                    _, relative, before, after, family = scope['cases'][index]
                elif entry_path.name in {'statement_mutations.py', 'canonical_recipe_mutations.py',
                                         'canonical_expression_mutations.py'}:
                    selected = scope['CASES'] if entry_path.name == 'canonical_expression_mutations.py' else scope['cases']
                    _, before, after = selected[index]
                    relative, family = mutable[0], fixture['contexts'][index]
                elif entry_path.name in {'literal_normalization_mutations.py', 'normalized_expression_mutations.py', 'formula_expression_mutations.py'}:
                    _, before, after, family = scope['CASES'][index]
                    relative = mutable[0]
                elif entry_path.name == 'normalized_recipe_mutations.py':
                    _, relative, before, after = scope['cases'][index]
                    family = fixture['contexts'][index]
                elif entry_path.name in {'semantic_mutations.py', 'namespace_mutations.py', 'name_read_mutations.py', 'builtin_signature_mutations.py', 'operator_signature_mutations.py', 'wanted_signature_mutations.py', 'call_lookup_mutations.py', 'reduction_boundary_mutations.py'}:
                    _, before, after = scope['CASES'][index]
                    relative, family = mutable[0], fixture['contexts'][index]
                elif entry_path.name == 'declaration_mutations.py':
                    if index < 19:
                        _, before, after = scope['CASES'][index]
                    else:
                        before, after = 'origin: FormulaScalarInputOrigin,', 'origin: FormulaInputOrigin,'
                    relative, family = mutable[0], fixture['contexts'][index]
                elif entry_path.name == 'ordered_name_mutations.py':
                    _, relative, before, after = scope['CASES'][index]
                    family = fixture['contexts'][index]
                elif entry_path.name == 'checked_recipe_mutations.py':
                    _, before, after, occurrences = scope['CASES'][index]
                    relative, family = mutable[0], fixture['contexts'][index]
                elif entry_path.name == 'checked_statement_mutations.py':
                    _, relative, before, after, occurrences = scope['CASES'][index]
                    family = fixture['contexts'][index]
                elif entry_path.name == 'checked_expression_mutations.py':
                    _, steps = scope['CASES'][index]
                    relative, family = mutable[0], fixture['contexts'][index]
                else:
                    _, relative, before, after, family = scope['CASES'][index]
                assert ROOT / relative == path and family == fixture['contexts'][index], \
                    'D156 domain authored family/source changed'
                expected = originals[path].decode()
                if entry_path.name in {'checked_statement_mutations.py', 'checked_recipe_mutations.py'}:
                    try:
                        assert expected.count(before) == occurrences, 'D156 measure nonunique source anchor'
                        expected = expected.replace(before, after, occurrences)
                        assert content == expected, 'D156 measure source mutation bytes changed'
                    except AssertionError as error:
                        errors.append(error)
                        raise
                elif entry_path.name in {'namespace_mutations.py', 'ordered_name_mutations.py', 'declaration_mutations.py', 'name_read_mutations.py', 'builtin_signature_mutations.py', 'operator_signature_mutations.py', 'wanted_signature_mutations.py', 'call_lookup_mutations.py', 'formula_expression_mutations.py', 'reduction_boundary_mutations.py', 'recipe_input_review_mutations.py'}:
                    try:
                        occurrences = 2 if entry_path.name == 'declaration_mutations.py' and index == 19 else 1
                        assert expected.count(before) == occurrences, 'D156 measure nonunique source anchor'
                        expected = expected.replace(before, after, occurrences)
                        assert content == expected, 'D156 measure source mutation bytes changed'
                    except AssertionError as error:
                        errors.append(error)
                        raise
                elif entry_path.name == 'checked_expression_mutations.py':
                    try:
                        for before, after in steps:
                            assert expected.count(before) == 1, 'D156 measure nonunique source anchor'
                            expected = expected.replace(before, after, 1)
                        assert content == expected, 'D156 measure source mutation bytes changed'
                    except AssertionError as error:
                        errors.append(error)
                        raise
                else:
                    occurrences = (5 if entry_path.name == 'canonical_expression_mutations.py'
                                   and not coupled and index == 18 else 1)
                    assert expected.count(before) == occurrences, 'D156 measure nonunique source anchor'
                    expected = expected.replace(before, after, occurrences)
                assert content == expected, 'D156 measure source mutation bytes changed'
                mutations.append(index)
                pending[path] = content
        else:
            assert path in files and path not in written, 'D156 measure unplanned output write'
            assert calls and path == files[len(calls) - 1], 'D156 measure diagnostic order changed'
            assert content == (b'baseline dispatch\n' if fixture['baseline'] and len(calls) == 1 else
                               b'restored dispatch\n' if len(calls) > len(bindings) + int(fixture['baseline']) else scope['LAST_OUTPUT']), \
                'D156 measure diagnostic bytes changed'
            written[path] = content
        return len(content)

    def child(args, *positional, **kwargs):
        try:
            assert not classifier_only, 'D156 classifier mode performed native effects'
            environment(kwargs.get('env', os.environ))
            assert planned == [True] and not positional and kwargs.get('cwd') == ROOT, \
                'D156 measure native cwd changed'
            assert kwargs.get('capture_output') is True, 'D156 measure native capture changed'
            index = len(calls)
            assert index < len(fixture['calls']), 'D156 measure extra native child'
            wanted = fixture['calls'][index]
            assert args == wanted, 'D156 measure native argv changed'
            fault_index = index - int(fixture['baseline'])
            expected_pending = {mutable[bindings[fault_index]]} if 0 <= fault_index < len(bindings) else set()
            assert set(pending) == expected_pending, 'D156 measure native source state changed'
            assert mutations == list(range(min(max(fault_index + 1, 0), len(bindings)))), 'D156 measure native mutation order changed'
            if 0 <= fault_index < len(bindings) and not (
                    entry_path.name == 'declaration_mutations.py' and fault_index == 19):
                site_source = (fixture['macro_sources'][fault_index % 2]
                               if entry_path.name == 'wanted_signature_mutations.py' else
                               contract if len(fixture['macro_sources']) == 1 or fixture['contexts'][fault_index] == 'units'
                               else mutable[bindings[fault_index]])
                current = read_text(site_source)
                if site_source == contract or (
                        entry_path.name == 'wanted_signature_mutations.py' and site_source in fixture['macro_sources']):
                    start = (current.index('fn coupled_worked_book_population_matches_authored_canonical_bytes(')
                             if entry_path.name == 'canonical_expression_mutations.py' and coupled else
                             current.index('fn ' + fixture['contexts'][fault_index] + '(')
                             if fixture['contexts'][fault_index] and len(fixture['macro_sources']) == 1 else 0)
                else:
                    start = current.index('mod domain_context_contracts {')
                body = current[start:]
                offsets = [body.index(token) for token in ('assert!(', 'assert_eq!(', 'assert_ne!(') if token in body]
                position = start + min(offsets)
                number = current[:position].count('\n') + 1
                column = len(current[current.rfind('\n', 0, position) + 1:position].encode()) + 1
                path = site_source.relative_to(ROOT).as_posix()
                assert (path, number, column) in scope['ASSERTION_SITES'], 'D177 domain contract macro absent'
        except AssertionError as error:
            errors.append(error)
            raise
        calls.append(args)
        if 0 <= fault_index < len(bindings):
            if entry_path.name == 'declaration_mutations.py' and fault_index == 19:
                output = b'Test compiled successfully\ncompile_fail\ntest result: FAILED.\n'
            else:
                output = (f"\nfailures:\nthread 'fixture' panicked at {path}:{number}:{column}:\n"
                          'authored dispatch fixture\n\nfailures:\nfixture\ntest result: FAILED.\n').encode()
            scope['LAST_OUTPUT'] = output
            return subprocess.CompletedProcess(args, 101, stdout=output, stderr=b'')
        return subprocess.CompletedProcess(args, 0,
                                           stdout=b'baseline dispatch\n' if fixture['baseline'] and index == 0 else b'restored dispatch\n',
                                           stderr=b'')

    try:
        os.environ.update(stores, RUSTUP_TOOLCHAIN='1.99.0', D156_DOMAIN_UNRELATED='preserved')
        if entry_path.name in {
            'semantic_mutations.py', 'recipe_mutations.py', 'statement_mutations.py',
            'literal_normalization_mutations.py', 'normalized_expression_mutations.py',
            'normalized_recipe_mutations.py', 'canonical_recipe_mutations.py', 'canonical_expression_mutations.py', 'checked_expression_mutations.py', 'checked_statement_mutations.py', 'checked_recipe_mutations.py', 'namespace_mutations.py', 'ordered_name_mutations.py', 'declaration_mutations.py', 'name_read_mutations.py', 'builtin_signature_mutations.py', 'operator_signature_mutations.py', 'wanted_signature_mutations.py', 'call_lookup_mutations.py', 'formula_expression_mutations.py'
        , 'reduction_boundary_mutations.py', 'recipe_input_review_mutations.py'}:
            sys.argv = [str(entry_path), *(['--classifier-only'] if classifier_only else ['--coupled'] if coupled else [])]
        Path.mkdir, Path.write_text, Path.write_bytes = directory, write, write
        Path.read_text = lambda p, *a, **k: read(p, False, *a, **k)
        Path.read_bytes = lambda p, *a, **k: read(p, True, *a, **k)
        subprocess.run, runpy.run_path = child, profile
        with contextlib.redirect_stdout(io.StringIO()):
            try:
                exec(compile(text, str(entry_path), 'exec'), scope)
            except AssertionError:
                if errors:
                    raise errors[0]
                raise
            except SystemExit as error:
                assert classifier_only and error.code == 0, 'D156 semantic classifier mode exit changed'
    finally:
        Path.mkdir, Path.read_text, Path.read_bytes = mkdir, read_text, read_bytes
        Path.write_text, Path.write_bytes, subprocess.run, runpy.run_path = write_text, write_bytes, run, load
        os.environ.clear()
        os.environ.update(saved)
        sys.argv = argv
        assert all(p.read_bytes() == data for p, data in originals.items()), 'D156 measure capture changed source'
    if classifier_only:
        assert planned == [True] and reads == list(fixture['initial_reads']) and \
            not calls and not written and not mutations and not restorations and not pending, \
            'D156 classifier mode performed native effects'
        return
    assert planned == [True] and len(calls) == len(fixture['calls']) and mutations == list(range(len(bindings))) and \
        set(written) == set(files) and not pending, 'D156 measure full capture incomplete'
    assert restorations == fixture['restoration_order'], 'D156 domain restoration sequence changed'
    assert reads[:len(fixture['initial_reads'])] == list(fixture['initial_reads']), \
        'D156 domain initial input order changed'


def capture_domain_entry(text, entry_path):
    return capture_mixed_entry(text, entry_path)


def domain_refusals(text, entry_path, *, classifier_only=False, coupled=False):
    """Actual copied mixed entry: every mutable source alias and endpoint log refuses."""
    work = GOOD['directory'](ROOT, 'target/scratch/domain_entry_refusals', create=True)
    fixture = mixed_fixture(entry_path, classifier_only=classifier_only, coupled=coupled)
    mutable, contract, output, files = (fixture[key] for key in ('mutable', 'contract', 'work', 'files'))
    sources = (*mutable, contract, *fixture['extras'])
    cases = ([('late-source', path) for path in sources] if classifier_only else
             [('first-file', files[0]), ('last-file', files[-1])] +
             [('mutable-source', path) for path in mutable] +
             [('late-source', path) for path in (contract, *fixture['extras'])])
    saved, argv, count = dict(os.environ), sys.argv, 0
    for case, path in cases:
        with tempfile.TemporaryDirectory(dir=work) as name:
            root = Path(name)
            entry = root / entry_path.relative_to(ROOT)
            for source, data in tuple((p, p.read_bytes()) for p in sources) + \
                    ((SOURCE, SOURCE.read_bytes()), (entry_path, text.encode())):
                destination = root / source.relative_to(ROOT)
                destination.parent.mkdir(parents=True, exist_ok=True)
                destination.write_bytes(data)
            (root / output.relative_to(ROOT)).mkdir(parents=True)
            victim = root / 'victim'; victim.write_bytes(b'unchanged isolated alias victim')
            if case in ('first-file', 'last-file'):
                (root / path.relative_to(ROOT)).symlink_to(victim)
                wanted = 'symlink producer output'
            elif case == 'mutable-source':
                victim.unlink(); os.link(root / path.relative_to(ROOT), victim)
                wanted = 'hard-linked mutable producer source'
            else:
                (root / path.relative_to(ROOT)).unlink()
                wanted = 'producer source is not a local regular file'
            before = {p: p.read_bytes() for p in root.rglob('*') if p.is_file() and not p.is_symlink()}
            environment = dict(os.environ)
            mkdir, read_text, read_bytes, write_text, write_bytes, run = (
                Path.mkdir, Path.read_text, Path.read_bytes, Path.write_text, Path.write_bytes, subprocess.run)
            def no_effect(*args, **kwargs):
                raise AssertionError('D156 measure physical refusal reached a read/write/child')
            diagnostic = io.StringIO()
            try:
                Path.mkdir, Path.read_text, Path.read_bytes = no_effect, no_effect, no_effect
                Path.write_text, Path.write_bytes, subprocess.run = no_effect, no_effect, no_effect
                with contextlib.redirect_stderr(diagnostic):
                    try:
                        if entry_path.name in {
                            'semantic_mutations.py', 'recipe_mutations.py', 'statement_mutations.py',
                            'literal_normalization_mutations.py', 'normalized_expression_mutations.py',
                            'normalized_recipe_mutations.py', 'canonical_recipe_mutations.py', 'canonical_expression_mutations.py', 'checked_expression_mutations.py', 'checked_statement_mutations.py', 'checked_recipe_mutations.py', 'namespace_mutations.py', 'ordered_name_mutations.py', 'declaration_mutations.py', 'name_read_mutations.py', 'builtin_signature_mutations.py', 'operator_signature_mutations.py', 'wanted_signature_mutations.py', 'call_lookup_mutations.py', 'formula_expression_mutations.py'
                        , 'reduction_boundary_mutations.py', 'recipe_input_review_mutations.py'}:
                            sys.argv = [str(entry), *(['--classifier-only'] if classifier_only else ['--coupled'] if coupled else [])]
                        exec(compile(text, str(entry), 'exec'), {'__file__': str(entry), '__name__': 'physical_entry'})
                    except SystemExit as error:
                        assert error.code == 2, 'D156 measure physical refusal status'
                    else:
                        raise AssertionError('D156 measure physical invalid entry accepted')
            finally:
                after_environment = dict(os.environ)
                Path.mkdir, Path.read_text, Path.read_bytes = mkdir, read_text, read_bytes
                Path.write_text, Path.write_bytes, subprocess.run = write_text, write_bytes, run
                os.environ.clear(); os.environ.update(saved)
                sys.argv = argv
            assert diagnostic.getvalue() == 'local-environment: REFUSED — ' + wanted + '\n', \
                ('D156 measure physical refusal diagnostic', case, diagnostic.getvalue())
            assert after_environment == environment and not (root / 'target/cargo-home').exists(), \
                'D156 measure physical refusal activated stores'
            assert all(p.read_bytes() == data for p, data in before.items()), 'D156 measure physical refusal changed bytes'
            count += 1
    return count


def measure_fixture(entry_path):
    """Independent authored dispatch populations; never derive expectations from CASES."""
    if entry_path == ENTRY:
        stem = 'size_membership'
        selected = ('blank_labels_refuse_including_unicode_whitespace',
                    'exactly_one_base_requires_nonempty_members_and_existing_identity',
                    'member_ids_and_exact_labels_must_be_unique_and_distinct_from_set',
                    'member_ids_and_exact_labels_must_be_unique_and_distinct_from_set',
                    'exactly_one_base_requires_nonempty_members_and_existing_identity',
                    'revision_successors_retain_identity_and_refuse_overflow_without_wrapping',
                    'authored_order_and_base_do_not_follow_lexical_or_numeric_sort')
    elif entry_path == ROOT / 'docs/tasks/artifacts/size_chart/size_chart_mutations.py':
        stem = 'size_chart'
        selected = ('context_refuses_duplicate_tables_and_cross_kind_set_member_identities',
                    'context_refuses_duplicate_tables_and_cross_kind_set_member_identities',
                    'both_authored_roles_require_garment_domain',
                    'observation_identity_cannot_alias_any_supplied_semantic_record',
                    'exact_set_identity_and_revision_are_required',
                    'values_members_and_both_roles_are_borrowed_without_recomputing',
                    'same_content_table_peer_does_not_replace_named_table',
                    'explicit_table_rebinding_does_not_silently_rebind_an_observation')
    elif entry_path == ROOT / 'docs/tasks/artifacts/measurement_table/table_mutations.py':
        stem = 'table'
        selected = ('duplicate_entry_id_and_token_refuse_without_coalescing_or_first_peer_selection',
                    'duplicate_entry_id_and_token_refuse_without_coalescing_or_first_peer_selection',
                    'current_inventory_id_collisions_refuse_before_any_table_lookup',
                    'current_token_reassignment_is_not_an_implicit_table_rename',
                    'current_domain_reassignment_is_refused_even_when_new_metadata_is_valid_in_its_new_domain',
                    'current_scalar_reassignment_requires_explicit_binding_replacement',
                    'required_current_target_failures_keep_table_measurement_and_underlying_identity_evidence',
                    'current_context_collision_of_table_identity_is_not_cached_birth_approval')
    elif entry_path == ROOT / 'docs/tasks/artifacts/mtm_chart/mtm_chart_mutations.py':
        stem = 'mtm_chart'
        selected = ('chart_and_context_identities_cannot_alias_members_sets_or_canonical_records',
                    'chart_and_context_identities_cannot_alias_members_sets_or_canonical_records',
                    'chart_and_context_identities_cannot_alias_members_sets_or_canonical_records',
                    'exact_set_revision_and_member_identity_are_required_not_matching_labels',
                    'mtm_requires_custom_system_and_exactly_one_member',
                    'mtm_requires_custom_system_and_exactly_one_member',
                    'every_set_target_and_authored_order_requires_explicit_chart_replacement',
                    'new_current_design_pom_invalidates_prior_complete_coverage',
                    'unknown_and_derived_body_inputs_remain_inspectable_and_refuse_numeric_defaults',
                    'unknown_and_derived_ease_amounts_remain_distinct_unresolved_inputs',
                    'incomplete_and_empty_drafts_cannot_certify_design_pom_coverage',
                    'grade_rule_input_always_refuses_for_complete_and_empty_authored_mtm_charts')
    elif entry_path == ROOT / 'docs/tasks/artifacts/size_chart_collection/size_chart_collection_mutations.py':
        stem = 'size_chart_collection'
        selected = ('collection_and_context_identities_cannot_alias_supplied_records',
                    'collection_and_context_identities_cannot_alias_supplied_records',
                    'exact_membership_reference_and_stable_member_are_required',
                    'authored_target_body_domain_is_refused_even_when_current_table_is_valid',
                    'duplicate_pom_token_observation_and_valid_same_cell_are_refused',
                    'duplicate_pom_token_observation_and_valid_same_cell_are_refused',
                    'duplicate_pom_token_observation_and_valid_same_cell_are_refused',
                    'changed_pom_binding_needs_explicit_chart_and_observation_replacement',
                    'reduced_target_list_cannot_certify_omitted_design_pom',
                    'incomplete_draft_refuses_exact_missing_cell_without_interpolation',
                    'all_saved_observation_targets_require_explicit_collection_replacement',
                    'undeclared_pom_and_observation_from_another_design_table_are_refused',
                    'unknown_and_derived_cells_can_be_structurally_complete_but_never_numeric_defaults',
                    'empty_inventory_is_inspectable_but_not_complete_and_keeps_table_revision_checks')
    elif entry_path == ROOT / 'docs/tasks/artifacts/ease/ease_mutations.py':
        stem = 'ease'
        selected = ('authored_sides_must_be_body_then_garment',
                    'amount_must_not_alias_either_measurement_scalar',
                    'current_mapping_identity_collisions_refuse',
                    'same_id_binding_reassignments_require_explicit_replacement',
                    'all_authored_value_states_enforce_negative_permission_and_preserve_sign',
                    'current_amount_edits_are_visible_and_missing_amount_refuses',
                    'missing_current_metadata_preserves_side_and_underlying_error')
    elif entry_path == ROOT / 'docs/tasks/artifacts/ease/ease_set_mutations.py':
        stem = 'ease_set'
        selected = ('contexts_refuse_duplicate_and_cross_kind_identities_before_lookup',
                    'contexts_refuse_duplicate_and_cross_kind_identities_before_lookup',
                    'duplicate_mapping_tokens_and_poms_refuse_without_coalescing',
                    'duplicate_mapping_tokens_and_poms_refuse_without_coalescing',
                    'duplicate_mapping_tokens_and_poms_refuse_without_coalescing',
                    'set_identity_collisions_refuse_at_creation_and_in_current_context',
                    'current_retargeted_mapping_needs_explicit_new_set_binding',
                    'empty_drafts_require_existing_tables_and_queries_never_default',
                    'selected_current_table_membership_is_required_even_when_metadata_exists',
                    'current_invalid_amount_and_table_targets_preserve_underlying_errors')
    elif entry_path == ROOT / 'docs/tasks/artifacts/formula_structure/round_mutations.py':
        stem = 'round'
        selected = ('both_i64_endpoints_are_representable_with_either_denominator_sign',
                    'extreme_i128_magnitudes_refuse_with_typed_overflow_without_panicking',
                    'both_i64_endpoints_are_representable_with_either_denominator_sign',
                    'extreme_denominators_zero_and_signs_preserve_half_away_rounding',
                    'extreme_denominators_zero_and_signs_preserve_half_away_rounding')
    elif entry_path == ROOT / 'docs/tasks/artifacts/formula_structure/unsigned_round_mutations.py':
        stem = 'unsigned_round'
        selected = ('full_width_inputs_and_high_remainders_round_without_narrowing',) * 4 + \
                   ('even_and_odd_denominator_half_neighborhoods_are_distinct',) * 2 + \
                   ('zero_denominator_retains_public_operation_without_panicking',) * 3
    else:
        assert entry_path == ROOT / 'docs/tasks/artifacts/formula_structure/length_operator_mutations.py', 'D156 unknown native entry'
        stem = 'length_operator'
        selected = ('signed_addition_crossings_refuse_instead_of_constructing_invalid_lengths',
                    'signed_subtraction_crossings_refuse_instead_of_constructing_invalid_lengths') * 3
    work_name = {'table': 'measurement_table_mutations', 'round': 'formula_round_mutations',
                 'unsigned_round': 'formula_unsigned_round_mutations'}.get(stem, stem + '_mutations')
    package = 'sc-units' if stem in {'round', 'unsigned_round', 'length_operator'} else 'sc-measure'
    source_stem = {'unsigned_round': 'round', 'length_operator': 'length'}.get(stem, stem)
    return (ROOT / ('crates/' + package + '/src/' + source_stem + '.rs'),
            ROOT / ('crates/' + package + '/tests/' + stem + '_contract.rs'),
            ROOT / ('target/' + work_name), selected)


def measure_extra_sources(entry_path):
    if entry_path == ROOT / 'docs/tasks/artifacts/formula_structure/round_mutations.py':
        return (ROOT / 'docs/tasks/artifacts/formula_structure/round_cases.tsv',)
    if entry_path == ROOT / 'docs/tasks/artifacts/formula_structure/unsigned_round_mutations.py':
        return (ROOT / 'docs/tasks/artifacts/formula_structure/unsigned_round_cases.tsv',)
    return ()


def measure_dispatch_fixture(entry_path, work, selected):
    """Authored consumer/profile/case/log order, independent of the entry's CASES."""
    if entry_path == ROOT / 'docs/tasks/artifacts/formula_structure/unsigned_round_mutations.py':
        indices = ((0, False), (1, False), (2, False), (3, False), (3, True),
                   (4, False), (5, False), (6, False), (7, False), (8, False))
        faults = tuple((index, ('--release',) if release else (), selected[index],
                        work / ('release-remainder.log' if release else 'mutation-%d.log' % (index + 1)))
                       for index, release in indices)
        return faults + ((None, (), None, work / 'restored-debug.log'),
                         (None, ('--release',), None, work / 'restored-release.log'))
    return tuple((index, (), test, work / ('mutation-%d.log' % (index + 1)))
                 for index, test in enumerate(selected)) + ((None, (), None, work / 'restored.log'),)


def measure_files_fixture(entry_path, work, selected):
    files = tuple(work / ('mutation-%d.log' % i) for i in range(1, len(selected) + 1))
    if entry_path == ROOT / 'docs/tasks/artifacts/formula_structure/unsigned_round_mutations.py':
        return files + (work / 'release-remainder.log', work / 'restored-debug.log', work / 'restored-release.log')
    return files + (work / 'restored.log',)


def capture_measure_entry(text, entry_path=ENTRY):
    """Complete actual body; intercepted native calls prove dispatch, not outcomes."""
    if entry_path in {ROOT / 'docs/tasks/artifacts/formula_structure/domain_context_mutations.py',
                      ROOT / 'docs/tasks/artifacts/formula_lex/formula_lex_mutations.py',
                      ROOT / 'docs/tasks/artifacts/formula_structure/semantic_mutations.py'}:
        return capture_mixed_entry(text, entry_path)
    source, contract, work, selected = measure_fixture(entry_path)
    extras = measure_extra_sources(entry_path)
    sources = (source, contract, *extras)
    originals = {p: p.read_bytes() for p in (*sources, entry_path, SOURCE)}
    files = measure_files_fixture(entry_path, work, selected)
    dispatch = measure_dispatch_fixture(entry_path, work, selected)
    restored_files = {file for index, _, _, file in dispatch if index is None}
    GOOD['directory'](ROOT, work.relative_to(ROOT).as_posix(), create=True)
    base = 'target/scratch/python_producer_contract/membership-stores'
    stores = {name: str(ROOT / base / suffix) for name, suffix in
              (('CARGO_HOME', 'cargo'), ('RUSTUP_HOME', 'rustup'),
               ('CARGO_TARGET_DIR', 'build'), ('TMPDIR', 'scratch'))}
    for value in stores.values():
        GOOD['directory'](ROOT, Path(value).relative_to(ROOT).as_posix(), create=True)
    scope = {'__file__': str(entry_path), '__name__': 'membership_capture'}
    saved = dict(os.environ)
    mkdir, read_text, read_bytes, write_text, write_bytes, run, load = (
        Path.mkdir, Path.read_text, Path.read_bytes, Path.write_text, Path.write_bytes,
        subprocess.run, runpy.run_path)
    planned, reads, calls, written, pending, errors, mutations = [], [], [], {}, {}, [], []

    def environment(values):
        assert all(values.get(k) == v for k, v in stores.items()), 'D156 measure child stores lost'
        assert values.get('MAKE_TMPDIR') == stores['TMPDIR'], 'D156 measure scratch alias lost'
        assert values.get('RUSTUP_TOOLCHAIN') == '1.99.0' and values.get('RUSTUP_AUTO_INSTALL') == '0', \
            'D156 measure toolchain changed'
        assert values.get('D156_MEMBERSHIP_UNRELATED') == 'preserved', 'D156 measure unrelated value lost'

    def profile(path, *args, **kwargs):
        assert path == str(SOURCE) and not args and not kwargs, 'D156 measure profile entry changed'

        def enter(root, *, directories=(), output_files=(), sources=(), mutable_sources=()):
            assert root == ROOT and directories == (work,), 'D156 measure directory plan changed'
            assert output_files == files, 'D156 measure file plan changed'
            assert sources == (source, contract, *extras), 'D156 measure source plan changed'
            assert mutable_sources == (source,), 'D156 measure mutable plan changed'
            assert not planned and not reads and not calls and not written, 'D156 measure plan was late'
            planned.append(True)
            return GOOD['enter_producer'](root, directories=directories, output_files=output_files,
                                          sources=sources, mutable_sources=mutable_sources)
        return dict(GOOD, enter_producer=enter)

    def read(path, binary, *args, **kwargs):
        assert planned == [True] and path in sources, 'D156 measure unplanned input read'
        environment(os.environ)
        reads.append(path)
        return (read_bytes if binary else read_text)(path, *args, **kwargs)

    def directory(path, *args, **kwargs):
        assert path == work and planned == [True], 'D156 measure unplanned directory write'
        environment(os.environ)

    def write(path, content, *args, **kwargs):
        assert planned == [True], 'D156 measure write before plan'
        environment(os.environ)
        if path == source:
            if isinstance(content, bytes):
                assert content == originals[source], 'D156 measure wrong source restoration'
                pending.clear()
            else:
                index = len(mutations)
                assert index < len(selected) and not pending, 'D156 measure source mutation order'
                if entry_path == ROOT / 'docs/tasks/artifacts/size_chart_collection/size_chart_collection_mutations.py':
                    _, replacements, _ = scope['CASES'][index]
                else:
                    _, before, after, _ = scope['CASES'][index]
                    replacements = ((before, after),)
                expected = originals[source].decode()
                for before, after in replacements:
                    assert expected.count(before) == 1, 'D156 measure nonunique source anchor'
                    expected = expected.replace(before, after, 1)
                if content != expected:
                    error = AssertionError('D156 measure source mutation bytes changed')
                    errors.append(error)
                    raise error
                mutations.append(index)
                pending[path] = content
        else:
            assert path in files and path not in written, 'D156 measure unplanned output write'
            assert len(calls) and path == dispatch[len(calls) - 1][3], 'D156 measure diagnostic order changed'
            assert content == (b'restored dispatch\n' if path in restored_files else scope['LAST_OUTPUT']), \
                'D156 measure diagnostic bytes changed'
            written[path] = content
        return len(content)

    def child(args, *positional, **kwargs):
        try:
            environment(kwargs.get('env', os.environ))
            assert planned == [True] and not positional and kwargs.get('cwd') == ROOT, \
                'D156 measure native cwd changed'
            assert kwargs.get('capture_output') is True, 'D156 measure native capture changed'
            index = len(calls)
            assert index < len(dispatch), 'D156 measure extra native child'
            case_index, extra, test, _ = dispatch[index]
            package = 'sc-units' if source.is_relative_to(ROOT / 'crates/sc-units') else 'sc-measure'
            wanted = ['cargo', 'test', '-p', package, *extra, '--test', contract.stem]
            if case_index is not None:
                wanted += [test, '--', '--exact']
            assert args == wanted, 'D156 measure native argv changed'
            assert bool(pending) == (case_index is not None), 'D156 measure native source state changed'
            if case_index is not None:
                assert mutations == list(range(case_index + 1)), 'D156 measure native mutation order changed'
        except AssertionError as error:
            errors.append(error)
            raise
        calls.append(args)
        if case_index is not None:
            current = contract.read_text()
            function_start = current.index('fn ' + test + '()')
            body = current[function_start:].split('\n#[test]', 1)[0]
            # Independent literal-token lookup; no production site helper is called.
            offsets = [body.index(token) for token in ('assert!(', 'assert_eq!(', 'assert_ne!(') if token in body]
            assert offsets, 'D156 measure selected test has no macro'
            position = function_start + min(offsets)
            number = current[:position].count('\n') + 1
            prefix = current[current.rfind('\n', 0, position) + 1:position]
            column = len(prefix.encode('utf-8')) + 1
            output = (f"\nfailures:\nthread 'fixture' panicked at {contract.relative_to(ROOT).as_posix()}:{number}:{column}:\n"
                      'authored dispatch fixture\n\nfailures:\nfixture\ntest result: FAILED.\n').encode()
            scope['LAST_OUTPUT'] = output
            return subprocess.CompletedProcess(args, 101, stdout=output, stderr=b'')
        return subprocess.CompletedProcess(args, 0, stdout=b'restored dispatch\n', stderr=b'')

    try:
        os.environ.update(stores, RUSTUP_TOOLCHAIN='1.99.0', D156_MEMBERSHIP_UNRELATED='preserved')
        Path.mkdir, Path.write_text, Path.write_bytes = directory, write, write
        Path.read_text = lambda p, *a, **k: read(p, False, *a, **k)
        Path.read_bytes = lambda p, *a, **k: read(p, True, *a, **k)
        subprocess.run, runpy.run_path = child, profile
        with contextlib.redirect_stdout(io.StringIO()):
            try:
                exec(compile(text, str(entry_path), 'exec'), scope)
            except AssertionError:
                if errors:
                    raise errors[0]
                raise
    finally:
        Path.mkdir, Path.read_text, Path.read_bytes = mkdir, read_text, read_bytes
        Path.write_text, Path.write_bytes, subprocess.run, runpy.run_path = write_text, write_bytes, run, load
        os.environ.clear()
        os.environ.update(saved)
        assert all(p.read_bytes() == data for p, data in originals.items()), 'D156 measure capture changed source'
    assert planned == [True] and len(calls) == len(dispatch) and mutations == list(range(len(selected))) and \
        set(written) == set(files) and not pending, \
        'D156 measure full capture incomplete'
    assert reads[:2] == [source, contract], 'D156 measure initial input order changed'


def measure_refusals(text, entry_path=ENTRY):
    """Actual copied entry and profile refuse physical aliases before reads/writes/children."""
    if entry_path in {ROOT / 'docs/tasks/artifacts/formula_structure/domain_context_mutations.py',
                      ROOT / 'docs/tasks/artifacts/formula_lex/formula_lex_mutations.py',
                      ROOT / 'docs/tasks/artifacts/formula_structure/semantic_mutations.py'}:
        return domain_refusals(text, entry_path)
    work = GOOD['directory'](ROOT, 'target/scratch/membership_entry_refusals', create=True)
    source, contract, output, selected = measure_fixture(entry_path)
    output_files = measure_files_fixture(entry_path, output, selected)
    extras = measure_extra_sources(entry_path)
    source, contract = (p.relative_to(ROOT).as_posix() for p in (source, contract))
    output_relative = output.relative_to(ROOT)
    saved, count = dict(os.environ), 0
    cases = ('first-file', 'last-file', 'mutable-source', 'late-source') + (('fixture-source',) if extras else ())
    for case in cases:
        with tempfile.TemporaryDirectory(dir=work) as name:
            root = Path(name)
            entry = root / entry_path.relative_to(ROOT)
            files = ((source, (ROOT / source).read_bytes()), (contract, (ROOT / contract).read_bytes()),
                     ('scripts/local_environment.py', SOURCE.read_bytes()),
                     (entry_path.relative_to(ROOT).as_posix(), text.encode())) + \
                    tuple((path.relative_to(ROOT).as_posix(), path.read_bytes()) for path in extras)
            for relative, data in files:
                destination = root / relative
                destination.parent.mkdir(parents=True, exist_ok=True)
                destination.write_bytes(data)
            output = root / output_relative
            output.mkdir(parents=True)
            victim = root / 'victim'
            victim.write_bytes(b'unchanged isolated alias victim')
            if case in ('first-file', 'last-file'):
                (root / output_files[0 if case == 'first-file' else -1].relative_to(ROOT)).symlink_to(victim)
                wanted = 'symlink producer output'
            elif case == 'mutable-source':
                victim.unlink()
                os.link(root / source, victim)
                wanted = 'hard-linked mutable producer source'
            else:
                missing = contract if case == 'late-source' else extras[-1].relative_to(ROOT).as_posix()
                (root / missing).unlink()
                wanted = 'producer source is not a local regular file'
            before = {p: p.read_bytes() for p in root.rglob('*') if p.is_file() and not p.is_symlink()}
            environment = dict(os.environ)
            mkdir, read_text, read_bytes, write_text, write_bytes, run = (
                Path.mkdir, Path.read_text, Path.read_bytes, Path.write_text, Path.write_bytes, subprocess.run)

            def no_effect(*args, **kwargs):
                raise AssertionError('D156 measure physical refusal reached a read/write/child')

            diagnostic = io.StringIO()
            try:
                Path.mkdir, Path.read_text, Path.read_bytes = no_effect, no_effect, no_effect
                Path.write_text, Path.write_bytes, subprocess.run = no_effect, no_effect, no_effect
                with contextlib.redirect_stderr(diagnostic):
                    try:
                        exec(compile(text, str(entry), 'exec'), {'__file__': str(entry), '__name__': 'physical_entry'})
                    except SystemExit as error:
                        assert error.code == 2, 'D156 measure physical refusal status'
                    else:
                        raise AssertionError('D156 measure physical invalid entry accepted')
            finally:
                after_environment = dict(os.environ)
                Path.mkdir, Path.read_text, Path.read_bytes = mkdir, read_text, read_bytes
                Path.write_text, Path.write_bytes, subprocess.run = write_text, write_bytes, run
                os.environ.clear()
                os.environ.update(saved)
            assert diagnostic.getvalue() == 'local-environment: REFUSED — ' + wanted + '\n', \
                ('D156 measure physical refusal diagnostic', case, diagnostic.getvalue())
            assert after_environment == environment and not (root / 'target/cargo-home').exists(), \
                'D156 measure physical refusal activated stores'
            assert all(p.read_bytes() == data for p, data in before.items()), \
                'D156 measure physical refusal changed bytes'
            count += 1
    return count


def fenced_blocks(text, language):
    """Independent line census for the public documentation fixture bytes."""
    blocks, current = [], None
    for line in text.splitlines():
        if line == '```' and current is not None:
            blocks.append('\n'.join(current) + '\n')
            current = None
        elif current is not None:
            current.append(line)
        elif line == '```' + language:
            current = []
    assert current is None, 'D156 compiler fixture fence unclosed'
    return blocks


def capture_compiler_entry(text, entry_path, owner):
    """Execute every actual compiler/example call; simulated output proves dispatch only."""
    assert owner in ('recipe', 'statement'), 'D156 compiler owner changed'
    source = ROOT / f'crates/sc-core/src/recipe/checked_{owner}.rs'
    sources = [source]
    book = ROOT / 'docs/book/src/annexes/formula-checked-recipes.md'
    if owner == 'recipe':
        sources.append(book)
    original_sources = {path: path.read_bytes() for path in sources}
    docs = '\n'.join(line[4:] if line.startswith('/// ') else line[3:]
                     for line in source.read_text().splitlines() if line.startswith('///'))
    guards = fenced_blocks(docs, 'compile_fail')
    examples = fenced_blocks(book.read_text(), 'rust') if owner == 'recipe' else []
    assert len(guards) == 5 and len(examples) == (4 if owner == 'recipe' else 0), 'D156 compiler fixture population'
    work = ROOT / f'target/{owner}_owner_contract'
    GOOD['directory'](ROOT, work.relative_to(ROOT).as_posix(), create=True)
    base = 'target/scratch/python_producer_contract/compiler-stores'
    custom = dict(zip(('CARGO_HOME', 'RUSTUP_HOME', 'CARGO_TARGET_DIR', 'TMPDIR'),
                      (base + '/cargo', base + '/rustup', base + '/build', base + '/scratch')))
    for value in custom.values():
        GOOD['directory'](ROOT, value, create=True)
    custom['MAKE_TMPDIR'] = custom['TMPDIR']
    directories = [ROOT / custom['CARGO_TARGET_DIR'] / name for name in ('a-deps', 'z-deps')]
    for path in directories:
        GOOD['directory'](ROOT, path.relative_to(ROOT).as_posix(), create=True)
    artifact = directories[1] / 'libsc_core_dispatch_fixture.rlib'
    dependency = directories[0] / 'libfixture.rmeta'
    GOOD['plan_producer'](ROOT, output_files=(artifact, dependency))
    artifact.write_bytes(b'authored public-metadata dispatch fixture; never compiled')
    dependency.write_bytes(b'authored dependency input; never compiled')
    files = (work / 'artifacts.json', work / 'build.log',
             *(work / f'guard_{index}{suffix}' for index in range(1, 6)
               for suffix in ('.rs', '.log', '')),
             *(work / f'example_{index}{suffix}' for index in range(1, len(examples) + 1)
               for suffix in ('.rs', '.log', '')))
    metadata = '\n'.join(json.dumps(message) for message in (
        {'reason': 'compiler-artifact', 'target': {'name': 'fixture', 'kind': ['lib']},
         'filenames': [str(directories[0] / 'libfixture.rmeta')]},
        {'reason': 'compiler-artifact', 'target': {'name': 'sc_core', 'kind': ['lib']},
         'filenames': [str(artifact)]},
        {'reason': 'build-finished', 'success': True})) + '\n'
    search_paths = [arg for path in directories for arg in ('-L', 'dependency=' + str(path))]
    expected_calls = [(['cargo', 'build', '-p', 'sc-core', '--message-format=json'], 0, metadata, '')]
    expected_writes = {work / 'artifacts.json': metadata, work / 'build.log': ''}
    for index, (code, diagnostic) in enumerate(zip(guards, ('E0451', 'E0515', 'E0451', 'E0515', 'E0515')), 1):
        fixture = work / f'guard_{index}.rs'
        stderr = json.dumps({'level': 'error', 'code': {'code': diagnostic}}) + '\n'
        expected_calls.append((['rustc', '--edition=2021', '--error-format=json', '--crate-name',
                               f'{owner}_guard_{index}', '--extern', 'sc_core=' + str(artifact),
                               *search_paths, str(fixture), '-o', str(work / f'guard_{index}')], 1, '', stderr))
        expected_writes[fixture] = code + 'fn main() {}\n'
        expected_writes[work / f'guard_{index}.log'] = stderr
    for index, code in enumerate(examples, 1):
        fixture, binary = work / f'example_{index}.rs', work / f'example_{index}'
        expected_calls.append((['rustc', '--edition=2021', '--crate-name', f'recipe_example_{index}',
                               '--extern', 'sc_core=' + str(artifact), *search_paths,
                               str(fixture), '-o', str(binary)], 0, '', ''))
        expected_calls.append(([str(binary)], 0, '', ''))
        # The extraction regex leaves the terminal source LF inside the fenced block.
        expected_writes[fixture] = 'fn main() {\n' + code + '\n}\n'
        expected_writes[work / f'example_{index}.log'] = ''
    saved, argv = dict(os.environ), sys.argv
    mkdir, read_text, write_text, write_bytes, run = Path.mkdir, Path.read_text, Path.write_text, Path.write_bytes, subprocess.run
    run_path = runpy.run_path
    observed, written, read, planned = [], {}, [], []
    library_plans, directory_plans = [], []

    def observe_profile(path, *args, **kwargs):
        assert path == str(ROOT / 'scripts/local_environment.py') and not args and not kwargs, 'D156 compiler profile entry changed'

        def enter(root, *, directories=(), output_files=(), sources=(), mutable_sources=()):
            assert root == ROOT and directories == (work,), 'D156 compiler planned output changed'
            assert sources == tuple(original_sources), 'D156 compiler planned inputs changed'
            assert output_files == files, 'D156 compiler file plan changed'
            assert mutable_sources == (), 'D156 compiler read-only inputs changed'
            assert not planned and not read and not written and not observed, 'D156 compiler profile was late'
            planned.append(True)
            return GOOD['enter_producer'](root, directories=directories, output_files=output_files,
                                          sources=sources, mutable_sources=mutable_sources)

        def plan(root, *, directories=(), output_files=(), sources=(), mutable_sources=()):
            assert root == ROOT, 'D156 compiler unexpected library plan'
            assert not directories and not output_files and not mutable_sources, 'D156 compiler library plan has effects'
            assert sources == (artifact, dependency), 'D156 compiler library inputs changed'
            assert observed == [expected_calls[0][0]] and len(written) == 2 and not library_plans, \
                'D156 compiler library plan was late'
            library_plans.append(True)
            return GOOD['plan_producer'](root, sources=sources)

        def directory(root, relative, *, create=False, missing_ok=False):
            assert root == ROOT and not create and not missing_ok, \
                'D156 compiler library directory plan has effects'
            assert library_plans == [True], 'D156 compiler library plan omitted'
            assert observed == [expected_calls[0][0]], 'D156 compiler library directory plan was late'
            path = ROOT / relative
            assert path in directories and path not in directory_plans, 'D156 compiler library directory changed'
            directory_plans.append(path)
            return GOOD['directory'](root, relative)

        return dict(GOOD, enter_producer=enter, plan_producer=plan, directory=directory)

    def validate_environment(environment):
        assert all(environment.get(name) == str(ROOT / value) for name, value in custom.items()), 'D156 compiler child stores lost'
        assert environment.get('RUSTUP_TOOLCHAIN') == '1.99.0', 'D156 compiler child channel lost'
        assert environment.get('RUSTUP_AUTO_INSTALL') == '0', 'D156 compiler child install policy lost'
        assert environment.get('D156_PYTHON_UNRELATED') == 'preserved', 'D156 compiler child unrelated environment lost'

    def observe_read(path, *args, **kwargs):
        assert path in original_sources, 'D156 compiler unexpected input read'
        assert planned, 'D156 compiler input read before profile'
        validate_environment(os.environ)
        read.append(path)
        return read_text(path, *args, **kwargs)

    def observe_mkdir(path, *args, **kwargs):
        assert path == work, 'D156 compiler unexpected directory write'
        validate_environment(os.environ)

    def observe_write(path, content, *args, **kwargs):
        assert path in expected_writes and path not in written, 'D156 compiler unexpected output write'
        assert content == expected_writes[path], 'D156 compiler fixture/output bytes changed'
        written[path] = content
        return len(content)

    def observe_child(args, *positional, **kwargs):
        validate_environment(kwargs.get('env', os.environ))
        assert kwargs.get('cwd') == ROOT and not positional, 'D156 compiler child cwd changed'
        assert kwargs.get('capture_output') is True and kwargs.get('text') is True, 'D156 compiler diagnostic capture changed'
        assert len(observed) < len(expected_calls), 'D156 compiler unexpected child'
        if observed:
            assert library_plans == [True], 'D156 compiler library plan omitted'
            assert directory_plans == directories, 'D156 compiler library directory plan omitted'
        wanted, status, stdout, stderr = expected_calls[len(observed)]
        assert args == wanted, 'D156 compiler child argv changed'
        observed.append(args)
        return subprocess.CompletedProcess(args, status, stdout=stdout, stderr=stderr)

    try:
        os.environ.update({name: str(ROOT / value) for name, value in custom.items()})
        os.environ.update(RUSTUP_TOOLCHAIN='1.99.0', RUSTUP_AUTO_INSTALL='0', D156_PYTHON_UNRELATED='preserved')
        sys.argv = [str(entry_path)]
        Path.mkdir, Path.read_text = observe_mkdir, observe_read
        Path.write_text, Path.write_bytes, subprocess.run = observe_write, observe_write, observe_child
        runpy.run_path = observe_profile
        with contextlib.redirect_stdout(io.StringIO()):
            exec(compile(text, str(entry_path), 'exec'), {'__file__': str(entry_path), '__name__': 'compiler_capture'})
    finally:
        Path.mkdir, Path.read_text, Path.write_text, Path.write_bytes, subprocess.run = mkdir, read_text, write_text, write_bytes, run
        runpy.run_path = run_path
        sys.argv = argv
        os.environ.clear()
        os.environ.update(saved)
        assert all(path.read_bytes() == data for path, data in original_sources.items()), 'D156 compiler capture changed input'
    assert len(observed) == len(expected_calls), 'D156 compiler child omitted'
    assert written == expected_writes, 'D156 compiler output omitted'
    assert read == sources, 'D156 compiler source/book reads changed'
    assert planned == [True], 'D156 compiler complete plan omitted'
    assert library_plans == [True], 'D156 compiler library plan omitted'
    assert directory_plans == directories, 'D156 compiler library directory plan omitted'



def compiler_file_refusals(text, entry_path, owner):
    """Actual copied compiler entry refuses aliases/types before input reads or Cargo."""
    assert owner in ('recipe', 'statement'), 'D156 compiler refusal owner changed'
    work = GOOD['directory'](ROOT, 'target/scratch/compiler_file_refusals', create=True)
    relative = f'target/{owner}_owner_contract'
    sources = [f'crates/sc-core/src/recipe/checked_{owner}.rs']
    if owner == 'recipe':
        sources.append('docs/book/src/annexes/formula-checked-recipes.md')
    saved = dict(os.environ)
    cases = ('first-link', 'last-link', 'guard-binary-link',
             'first-hardlink', 'last-hardlink', 'file-directory', 'late-source')
    for case in cases:
        with tempfile.TemporaryDirectory(dir=work) as name:
            root = Path(name)
            entry = root / entry_path.relative_to(ROOT)
            fixtures = [(source, (ROOT / source).read_bytes()) for source in sources]
            fixtures.extend((('scripts/local_environment.py', SOURCE.read_bytes()),
                             (entry_path.relative_to(ROOT).as_posix(), text.encode())))
            for path, data in fixtures:
                destination = root / path
                destination.parent.mkdir(parents=True, exist_ok=True)
                destination.write_bytes(data)
            output = root / relative
            output.mkdir(parents=True)
            victim = root / 'victim'
            victim.write_bytes(b'unchanged isolated compiler alias victim')
            first = output / 'artifacts.json'
            last = output / ('example_4' if owner == 'recipe' else 'guard_5')
            if case in ('first-link', 'last-link', 'guard-binary-link'):
                destination = first if case == 'first-link' else last if case == 'last-link' else output / 'guard_1'
                destination.symlink_to(victim)
                wanted = 'symlink producer output'
            elif case in ('first-hardlink', 'last-hardlink'):
                os.link(victim, first if case == 'first-hardlink' else last)
                wanted = 'hard-linked producer output'
            elif case == 'file-directory':
                first.mkdir()
                wanted = 'producer output is not a local regular file'
            else:
                (root / sources[-1]).unlink()
                wanted = 'producer source is not a local regular file'
            before = {path: path.read_bytes() for path in root.rglob('*')
                      if path.is_file() and not path.is_symlink()}
            environment = dict(os.environ)
            mkdir, read_text, read_bytes, write_text, write_bytes, run = (
                Path.mkdir, Path.read_text, Path.read_bytes, Path.write_text, Path.write_bytes, subprocess.run)

            def no_effect(*args, **kwargs):
                raise AssertionError('D156 compiler physical refusal reached a read/write/child')

            diagnostic = io.StringIO()
            try:
                Path.mkdir, Path.read_text, Path.read_bytes = no_effect, no_effect, no_effect
                Path.write_text, Path.write_bytes, subprocess.run = no_effect, no_effect, no_effect
                with contextlib.redirect_stderr(diagnostic):
                    try:
                        exec(compile(text, str(entry), 'exec'), {'__file__': str(entry), '__name__': 'compiler_physical'})
                    except SystemExit as error:
                        assert error.code == 2, 'D156 compiler physical refusal status'
                    else:
                        raise AssertionError('D156 compiler physical invalid entry accepted')
            finally:
                after_environment = dict(os.environ)
                Path.mkdir, Path.read_text, Path.read_bytes = mkdir, read_text, read_bytes
                Path.write_text, Path.write_bytes, subprocess.run = write_text, write_bytes, run
                os.environ.clear()
                os.environ.update(saved)
            assert diagnostic.getvalue() == 'local-environment: REFUSED — ' + wanted + '\n', \
                ('D156 compiler physical refusal diagnostic', case, diagnostic.getvalue())
            assert after_environment == environment and not (root / 'target/cargo-home').exists(), \
                'D156 compiler physical refusal activated stores'
            assert all(path.read_bytes() == data for path, data in before.items()), \
                'D156 compiler physical refusal changed bytes'
    return len(cases)


def capture_child(text, entry_path, work_name, sources, test_target, *, arguments=(), first_suffix=None,
                  doc_suffix=None, capture_doc_fault=False):
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
    child_errors = []
    test_targets = (test_target,) if isinstance(test_target, str) else test_target
    native_argv = ['cargo', 'test', '-p', 'sc-core']
    for target in test_targets:
        native_argv.extend(['--test', target])
    doc_argv = ['cargo', 'test', '-p', 'sc-core', '--doc', doc_suffix]
    expected = [native_argv, native_argv]
    fixture_output = b''
    if doc_suffix is not None:
        expected.append(doc_argv)
    if capture_doc_fault:
        assert doc_suffix is not None, 'D156 Python missing doc profile'
        expected = [native_argv] * 19 + [doc_argv, native_argv, doc_argv]
        # Only dispatch is simulated here; genuine assertion evidence is verified natively.
        contract = ROOT / 'crates/sc-core/tests' / (test_target + '.rs')
        sites = [(index, len(match.group(1).encode()) + 1)
                 for index, line in enumerate(contract.read_text().splitlines(), 1)
                 if (match := re.match(r'^(\s*)assert(?:_eq|_ne)?!\s*\(', line))]
        assert sites, 'D156 Python doc capture has no assertion fixture site'
        line, column = sites[0]
        fixture_output = (f'\nfailures:\nthread panicked at {contract.relative_to(ROOT).as_posix()}:{line}:{column}:\n'
                          'custom assertion fixture\n\nfailures:\nfixture\ntest result: FAILED.\n').encode()

    class Stopped(Exception):
        pass

    def no_mkdir(path, *args, **kwargs):
        assert path == work, 'D156 Python unexpected child-capture directory'

    def no_write(path, *args, **kwargs):
        assert path in original_sources or path.parent == work, 'D156 Python unexpected child-capture write'
        return 0

    def validate_child(args, positional, kwargs):
        environment = kwargs.get('env', os.environ)
        assert all(environment.get(name) == str(ROOT / value) for name, value in custom.items()), 'D156 Python child stores lost'
        assert environment.get('RUSTUP_TOOLCHAIN') == '1.99.0', 'D156 Python child channel lost'
        assert environment.get('RUSTUP_AUTO_INSTALL') == '0', 'D156 Python child install policy lost'
        assert environment.get('D156_PYTHON_UNRELATED') == 'preserved', 'D156 Python child unrelated environment lost'
        assert len(observed) < len(expected), 'D156 Python child native argv changed'
        current_argv = expected[len(observed)]
        assert args[:len(current_argv)] == current_argv, 'D156 Python child native argv changed'
        if doc_suffix is not None:
            assert len(args) == 6, 'D156 Python child profile selection changed'
        if first_suffix is not None:
            wanted = list(first_suffix) if not observed else []
            assert args[len(current_argv):] == wanted, 'D156 Python child test selection changed'
        assert kwargs.get('cwd') == ROOT and not positional, 'D156 Python child cwd changed'

    def observe(args, *positional, **kwargs):
        try:
            validate_child(args, positional, kwargs)
        except AssertionError as error:
            child_errors.append(error)
            raise
        observed.append(args)
        if capture_doc_fault and len(observed) <= 19:
            return subprocess.CompletedProcess(args, 101, stdout=fixture_output, stderr=b'')
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
    if child_errors:
        raise child_errors[0]
    assert len(observed) == len(expected), 'D156 Python child capture/restoration absent'


def classifier_controls(text, entry_path, *, strict=False, thread_only=False):
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
        if thread_only:
            noises += (location.replace(b"thread 'fixture'", b'compiler note:') +
                       b':2:5:\nnonthread location mention\n\nfailures:\nfixture\n',)
    assert classify(accepted), 'D159 actual failed-body assertion refused'
    for noise in noises:
        assert not classify(noise), 'D159 assertion evidence accepted noise'



def measure_inline_controls(text, entry_path):
    tree = ast.parse(text)
    functions = [node for node in tree.body if isinstance(node, ast.FunctionDef)
                 and node.name in {'assertion_sites', 'assertion_failure'}]
    scope = {'re': re}
    exec(compile(ast.Module(body=functions, type_ignores=[]), str(entry_path), 'exec'), scope)
    path = 'crates/sc-measure/tests/inline_fixture.rs'
    fixture = 'fn fixture() {\n    apply(|c| assert_eq!(1, 2));\n    let é = (); apply(|| assert_ne!(1, 2));\n}\n'
    sites = scope['assertion_sites'](fixture, path)
    assert sites == {(path, 2, 15), (path, 3, 27)}, 'D177 measure inline macro sites changed'
    scope['ASSERTION_SITES'] = sites
    for line, column in ((2, 15), (3, 27)):
        output = (f"\nfailures:\nthread 'fixture' panicked at {path}:{line}:{column}:\n"
                  'custom message\n\nfailures:\nfixture\n').encode()
        assert scope['assertion_failure'](output), 'D177 measure inline macro evidence refused'
        assert not scope['assertion_failure'](output.replace(f':{column}:'.encode(), f':{column - 1}:'.encode())), \
            'D177 measure inline wrong byte column accepted'


def lexer_scan_controls(text, entry_path):
    """The current scan helper contributes a real macro; labelled extraction supplies no evidence."""
    functions = [node for node in ast.parse(text).body if isinstance(node, ast.FunctionDef)
                 and node.name in {'assertion_sites', 'assertion_failure'}]
    scope = {'re': re}
    exec(compile(ast.Module(body=functions, type_ignores=[]), str(entry_path), 'exec'), scope)
    contract = mixed_fixture(entry_path)['contract']
    GOOD['plan_producer'](ROOT, sources=(contract,))
    current = contract.read_text()
    authored = '            assert!(item.is_ok(), "assertion: machine lexical form: {item:?}");'
    lines = current.splitlines()
    assert lines.count(authored) == 1, 'D177 lexer scan assertion changed'
    number = lines.index(authored) + 1
    path = contract.relative_to(ROOT).as_posix()
    sites = scope['assertion_sites'](current, path)
    assert (path, number, 13) in sites, 'D177 lexer scan macro absent'
    scope['ASSERTION_SITES'] = sites
    output = (f"\nfailures:\nthread 'fixture' panicked at {path}:{number}:13:\n"
              'scan helper without message-prefix authority\n\nfailures:\nfixture\n').encode()
    assert scope['assertion_failure'](output), 'D177 lexer scan macro refused'
    expect_line = next(n for n, line in enumerate(lines, 1) if 'result.expect_err("machine ASCII required")' in line)
    for noise in (output.replace(f':{number}:13:'.encode(), f':{expect_line}:28:'.encode()),
                  output.replace(f':{number}:13:'.encode(), f':{number}:14:'.encode()),
                  output.replace(path.encode(), b'crates/sc-core/src/recipe/lexer.rs')):
        assert not scope['assertion_failure'](noise), 'D177 lexer scan accepted noise'


def round_totality_controls(text, entry_path):
    functions = [node for node in ast.parse(text).body if isinstance(node, ast.FunctionDef)
                 and node.name in {'assertion_sites', 'assertion_failure'}]
    scope = {'re': re}
    exec(compile(ast.Module(body=functions, type_ignores=[]), str(entry_path), 'exec'), scope)
    _, contract, _, _ = measure_fixture(entry_path)
    GOOD['plan_producer'](ROOT, sources=(contract,))
    current = contract.read_text()
    message = 'unsigned rounding must never unwind' if contract.stem == 'unsigned_round_contract' else \
              'public rounding must never unwind'
    authored = '    assert!(result.is_ok(), "' + message + '");'
    lines = current.splitlines()
    assert lines.count(authored) == 1, 'D177 round totality assertion changed'
    number = lines.index(authored) + 1
    path = contract.relative_to(ROOT).as_posix()
    site = (path, number, 5)
    sites = scope['assertion_sites'](current, path)
    assert site in sites, 'D177 round totality macro site absent'
    scope['ASSERTION_SITES'] = sites
    prefix = "\nfailures:\nthread 'fixture' panicked at "
    actual = (prefix + f'{path}:{number}:5:\ncustom totality message\n\nfailures:\nfixture\n').encode()
    assert scope['assertion_failure'](actual), 'D177 round totality assertion refused'
    expect_line = next(n for n, line in enumerate(lines, 1) if 'result.expect(' in line)
    noises = (actual.replace(f':{number}:5:'.encode(), f':{expect_line}:12:'.encode()),
              actual.replace(f':{number}:5:'.encode(), f':{number}:6:'.encode()),
              actual.replace(path.encode(), b'crates/sc-units/src/round.rs'))
    for noise in noises:
        assert not scope['assertion_failure'](noise), 'D177 round totality evidence accepted noise'
    panic = b"thread 'fixture' panicked at crates/sc-units/src/round.rs:80:17:\nattempt to divide by zero\n"
    assert scope['assertion_failure'](actual.replace(b'\nfailures:\n', b'\nfailures:\n' + panic, 1)), \
        'D177 round source panic masked valid totality assertion'


def driver_classifier_controls(text, path):
    names = {'driver_assertion_sites', 'driver_assertion_failure'}
    functions = [n for n in ast.parse(text).body if isinstance(n, ast.FunctionDef) and n.name in names]
    assert {n.name for n in functions} == names, 'D172 driver classifier helpers missing'
    scope = {'re': re, 'Path': Path, 'ROOT': ROOT}
    exec(compile(ast.Module(body=functions, type_ignores=[]), str(path), 'exec'), scope)
    fixture = 'docs/tasks/artifacts/formula_structure/authored_driver_fixture.rs'
    sites = scope['driver_assertion_sites']('fn main() {\n    assert!(false, "custom");\n    None::<()>.expect("assertion: noise");\n}\n', fixture)
    assert sites == {(fixture, 2, 5)}, 'D172 driver assertion sites changed'
    scope['DRIVER_ASSERTION_SITES'] = sites
    classify = scope['driver_assertion_failure']
    prefix = b"thread 'main' panicked at " + fixture.encode()
    assert classify(prefix + b':2:5:\nassertion failed: actual driver fixture\n'), 'D172 valid driver macro refused'
    for noise in (
        prefix + b':3:5:\nassertion: expect-only noise\n',
        prefix + b':2:6:\nassertion: wrong column\n',
        prefix.replace(fixture.encode(), b'other.rs') + b':2:5:\nassertion: wrong source\n',
        prefix.replace(fixture.encode(), b'\xff') + b':2:5:\nassertion: invalid path bytes\n',
        b'error: assertion failed at compiler/link stage\n',
        b'test assertion_name ... ok\nthread panicked\nassertion: no location\n',
        b'',
    ):
        assert not classify(noise), 'D172 driver classifier accepted noise'
    assert classify(prefix + b':2:5:\ncustom message without a prefix\n'), 'D172 valid driver macro refused'
    absolute = b"thread 'main' panicked at " + str(ROOT / fixture).encode()
    assert classify(absolute + b':2:5:\ncustom\n'), 'D172 current absolute driver macro refused'


def capture_static_entry(text, path, *, mutations=False, early_refusal=False, borrowed_label=None):
    """Actual standalone modes; all writes/children intercepted, native proof stays separate."""
    assert borrowed_label is None or not mutations and not early_refusal, 'D156 static borrowed mode changed'
    here = ROOT / 'docs/tasks/artifacts/formula_structure'
    driver = here / 'static_coupled_driver.rs'
    mutable = tuple(ROOT / 'crates/sc-core/src/recipe' / name for name in (
        'checked.rs', 'checked_statement.rs', 'namespace/ordered.rs', 'checked_recipe.rs'))
    helpers = tuple(here / name for name in (
        'dimension_payload_contract.py', 'static_review_contract.py', 'static_namespace_contract.py',
        'static_signature_contract.py', 'checked_recipe_mutations.py'))
    books = tuple(ROOT / 'docs/book/src' / name for name in (
        'spec/formula-language.md', 'spec/formula-language/grammar.md',
        'spec/formula-language/examples.md', 'spec/reference-skirt.md', 'spec/units-and-tolerances.md',
        'annexes/formula-static-validation.md', 'annexes/formula-wanted-signatures.md',
        'annexes/implementation-status.md'))
    sources = (ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh',
               driver, here / 'canonical_worked_recipe_cases.tsv', *helpers, *mutable, *books)
    originals = {source: source.read_bytes() for source in sources}
    work = ROOT / 'target/static_coupled_contract'
    GOOD['directory'](ROOT, work.relative_to(ROOT).as_posix(), create=True)
    base = 'target/scratch/python_producer_contract/static-stores'
    custom = dict(zip(('CARGO_HOME', 'RUSTUP_HOME', 'CARGO_TARGET_DIR', 'TMPDIR'),
                      (base + '/cargo', base + '/rustup', base + '/build', base + '/scratch')))
    for value in custom.values():
        GOOD['directory'](ROOT, value, create=True)
    custom['MAKE_TMPDIR'] = custom['TMPDIR']
    deps = ROOT / custom['CARGO_TARGET_DIR'] / 'deps'
    GOOD['directory'](ROOT, deps.relative_to(ROOT).as_posix(), create=True)
    library = deps / 'libsc_core_dispatch_fixture.rlib'
    dependency = deps / 'libdependency_dispatch_fixture.rmeta'
    GOOD['plan_producer'](ROOT, output_files=(library, dependency))
    library.write_bytes(b'authored metadata fixture; never compiled')
    dependency.write_bytes(b'authored dependency metadata fixture; never compiled')
    metadata = '\n'.join(json.dumps(message) for message in (
        {'reason': 'compiler-artifact', 'target': {'name': 'dependency', 'kind': ['lib']},
         'filenames': [str(dependency)]},
        {'reason': 'compiler-artifact', 'target': {'name': 'sc_core', 'kind': ['lib']},
         'filenames': [str(library)]})) + '\n'
    filenames = ('cargo-artifacts.json', 'cargo.log', 'rustc.log', 'cases.hex.tsv',
                 'driver.tsv', 'driver.log', 'driver')
    planned_labels = ['baseline']
    if mutations:
        planned_labels += [f'fault-{index}' for index in range(1, 29)] + ['restored']
    files = tuple(work / label / name for label in planned_labels for name in filenames)
    reference_files = tuple(work / ('reference-' + name + '.log') for name in (
        'operation-arguments', 'conditional-condition', 'variadic-consistency',
        'complete-operand-kinds', 'metadata-value-queried', 'whole-phase-interleaved'))
    if mutations:
        files += tuple(work / f'fault-{index}/oracle.log' for index in range(1, 29))
        files += reference_files
    assert len(files) == (244 if mutations else 7), 'D156 static authored file population'
    allowed_files = set(files)
    labels = [borrowed_label] if borrowed_label is not None else ['baseline']
    if mutations:
        labels += ['fault-1', 'restored'] if early_refusal else [f'fault-{i}' for i in range(1, 29)] + ['restored']
    scope = {'__file__': str(path), '__name__': 'static_borrowed' if borrowed_label is not None else '__main__'}
    saved, argv = dict(os.environ), sys.argv
    mkdir, read_text, read_bytes = Path.mkdir, Path.read_text, Path.read_bytes
    write_text, write_bytes, run, run_path = Path.write_text, Path.write_bytes, subprocess.run, runpy.run_path
    observed, written, pending, plans, loads = [], {}, {}, [], []
    build_plans, library_plans, library_directories = [], [], []
    batch, stage, outcome = 0, 0, None

    def environment(values):
        assert all(values.get(name) == str(ROOT / value) for name, value in custom.items()), 'D156 static child stores lost'
        assert values.get('RUSTUP_TOOLCHAIN') == '1.99.0' and values.get('RUSTUP_AUTO_INSTALL') == '0', 'D156 static child toolchain changed'
        assert values.get('D156_PYTHON_UNRELATED') == 'preserved', 'D156 static unrelated environment lost'

    def profile(file, *args, **kwargs):
        if file != str(ROOT / 'scripts/local_environment.py'):
            assert plans and Path(file) in helpers, 'D156 static unplanned loader'
            loads.append(Path(file))
            return run_path(file, *args, **kwargs)

        def enter(root, *, directories=(), output_files=(), sources=(), mutable_sources=()):
            assert root == ROOT and directories == (work,), 'D156 static output plan changed'
            assert sources == tuple(originals), 'D156 static input plan changed'
            assert output_files == files, 'D156 static file plan changed'
            assert mutable_sources == (mutable if mutations else ()), 'D156 static mutable plan changed'
            assert not plans and not loads and not observed and not written, 'D156 static late profile'
            plans.append(True)
            return GOOD['enter_producer'](root, directories=directories, output_files=output_files,
                                          sources=sources, mutable_sources=mutable_sources)

        def plan(root, *, directories=(), output_files=(), sources=(), mutable_sources=()):
            assert root == ROOT and not mutable_sources, 'D156 static pure plan changed'
            label = labels[batch]
            if directories:
                output = work / label
                assert stage == 0 and len(build_plans) == batch, 'D156 static build plan was late'
                assert directories == (output,), 'D156 static build directory plan changed'
                assert output_files == tuple(output / name for name in filenames), 'D156 static build file plan changed'
                assert sources == tuple(originals), 'D156 static build source plan changed'
                allowed_files.update(output_files)
                build_plans.append(label)
            else:
                assert stage == 1 and len(library_plans) == batch, 'D156 static library plan was late'
                assert not output_files, 'D156 static library plan has effects'
                assert sources == (library, dependency), 'D156 static library inputs changed'
                library_plans.append(label)
            return GOOD['plan_producer'](root, directories=directories, output_files=output_files,
                                         sources=sources)

        def library_directory(root, relative, *, create=False, missing_ok=False):
            assert root == ROOT and not create and not missing_ok, 'D156 static library directory plan has effects'
            assert library_plans == labels[:batch + 1], 'D156 static library plan omitted'
            assert stage == 1 and len(library_directories) == batch and ROOT / relative == deps, 'D156 static library directory changed'
            library_directories.append(labels[batch])
            return GOOD['directory'](root, relative)

        return dict(GOOD, enter_producer=enter, plan_producer=plan, directory=library_directory)

    def read(path_, binary, *args, **kwargs):
        assert plans, 'D156 static read before profile'
        environment(os.environ)
        if path_ in written:
            return written[path_].encode() if binary else written[path_]
        assert path_ in originals, 'D156 static unplanned input read'
        return (read_bytes if binary else read_text)(path_, *args, **kwargs)

    def directory(path_, *args, **kwargs):
        assert path_ == work or path_ in {work / label for label in labels}, 'D156 static unplanned directory'
        if path_ != work:
            assert build_plans == labels[:batch + 1], 'D156 static build plan omitted'
            assert stage == 0, 'D156 static build plan was late'
        environment(os.environ)

    def write(path_, data, *args, **kwargs):
        if path_ in mutable:
            if isinstance(data, bytes):
                assert data == originals[path_], 'D172 static wrong source restoration bytes'
                pending.pop(path_, None)
            else:
                assert batch < len(labels) and labels[batch].startswith('fault-'), 'D172 static source write outside fault'
                index = int(labels[batch].split('-')[1]) - 1
                _, relative, before, after, count = scope['RUST_FAULTS'][index]
                assert path_ == ROOT / relative and data == originals[path_].decode().replace(before, after, count), 'D172 static actual source fault changed'
                pending[path_] = data
        else:
            assert path_ in allowed_files and path_.parent in {work, *(work / label for label in labels)}, 'D156 static unplanned output write'
            assert isinstance(data, str), 'D156 static output encoding changed'
            written[path_] = data
        return len(data)

    def child(args, *positional, **kwargs):
        nonlocal batch, stage, outcome
        environment(kwargs.get('env', os.environ))
        assert kwargs.get('cwd') == ROOT and not positional, 'D156 static child cwd changed'
        assert kwargs.get('capture_output') is True and kwargs.get('text') is True, 'D156 static child capture changed'
        assert batch < len(labels), 'D156 static extra child'
        label = labels[batch];output = work / label
        assert build_plans == labels[:batch + 1], 'D156 static build plan omitted'
        if stage:
            assert library_plans == labels[:batch + 1], 'D156 static library plan omitted'
            assert library_directories == labels[:batch + 1], 'D156 static library directory plan omitted'
        wanted = (
            ['cargo', 'build', '-p', 'sc-core', '--message-format=json'],
            ['rustc', '--edition=2021', '-Dwarnings', '--extern', 'sc_core=' + str(library),
             '-L', 'dependency=' + str(deps), str(driver), '-o', str(output / 'driver')],
            [str(output / 'driver')],
        )[stage]
        assert args == wanted, 'D156 static child argv changed'
        assert bool(pending) == label.startswith('fault-'), 'D172 static child source state changed'
        observed.append((label, stage))
        stdout, stderr, status = '', '', 0
        if stage == 0:
            stdout = metadata
        elif stage == 1 and early_refusal and label == 'fault-1':
            stderr, status = 'authored compiler refusal; never body proof', 1
        elif stage == 2:
            lines = kwargs['input'].splitlines()
            assert lines[0] == ';'.join(':'.join(row) for row in scope['CATALOG']), 'D156 static catalog input changed'
            assert len(lines) == len(scope['CASES']) + 1, 'D156 static shared input population changed'
            for i, (line, (mode, source, _)) in enumerate(zip(lines[1:], scope['CASES'])):
                fields = line.split('\t')
                assert fields == [str(i), mode, source.encode().hex()], 'D156 static actual driver input changed'
            if outcome is None:
                outcome = ''.join(str(i) + '\t' + '\t'.join(expected) + '\n'
                                  for i, (_, _, expected) in enumerate(scope['CASES']))
            stdout = '0\tERR\tformula_dimension\t-\n' + outcome.split('\n', 1)[1] if pending else outcome
        if stage == 2 or status:
            batch += 1
            stage = 0
        else:
            stage += 1
        return subprocess.CompletedProcess(args, status, stdout=stdout, stderr=stderr)

    try:
        os.environ.update({name: str(ROOT / value) for name, value in custom.items()})
        os.environ.update(RUSTUP_TOOLCHAIN='1.99.0', RUSTUP_AUTO_INSTALL='0', D156_PYTHON_UNRELATED='preserved')
        sys.argv = [str(path), *(['--mutations'] if mutations else [])]
        Path.mkdir = directory
        Path.read_text = lambda p, *a, **k: read(p, False, *a, **k)
        Path.read_bytes = lambda p, *a, **k: read(p, True, *a, **k)
        Path.write_text, Path.write_bytes, subprocess.run, runpy.run_path = write, write, child, profile
        with contextlib.redirect_stdout(io.StringIO()):
            try:
                exec(compile(text, str(path), 'exec'), scope)
                if borrowed_label is not None:
                    scope['build_and_check'](borrowed_label)
            except AssertionError as error:
                assert early_refusal and 'compiler/link/expect/noise refused' in str(error), error
    finally:
        Path.mkdir, Path.read_text, Path.read_bytes = mkdir, read_text, read_bytes
        Path.write_text, Path.write_bytes, subprocess.run, runpy.run_path = write_text, write_bytes, run, run_path
        sys.argv = argv
        os.environ.clear()
        os.environ.update(saved)
        assert all(source.read_bytes() == data for source, data in originals.items()), 'D156 static capture changed source'
    assert plans == [True] and set(loads) == set(helpers[:-1]), 'D156 static loader/profile coverage changed'
    assert not pending, 'D172 static source restoration absent'
    assert batch == len(labels) and stage == 0, 'D172 static restored compiler dispatch absent'
    assert len(observed) == (8 if early_refusal else 90 if mutations else 3), 'D156 static native mode dispatch changed'
    assert build_plans == library_plans == library_directories == labels, 'D156 static build/library plan omitted'



def static_file_refusals(text, entry_path):
    """Copied actual CLI plans refuse aliases before reads/effects; ordinary inputs stay read-only."""
    here = ROOT / 'docs/tasks/artifacts/formula_structure'
    mutable = tuple(ROOT / 'crates/sc-core/src/recipe' / name for name in (
        'checked.rs', 'checked_statement.rs', 'namespace/ordered.rs', 'checked_recipe.rs'))
    helpers = tuple(here / name for name in (
        'dimension_payload_contract.py', 'static_review_contract.py', 'static_namespace_contract.py',
        'static_signature_contract.py', 'checked_recipe_mutations.py'))
    books = tuple(ROOT / 'docs/book/src' / name for name in (
        'spec/formula-language.md', 'spec/formula-language/grammar.md',
        'spec/formula-language/examples.md', 'spec/reference-skirt.md', 'spec/units-and-tolerances.md',
        'annexes/formula-static-validation.md', 'annexes/formula-wanted-signatures.md',
        'annexes/implementation-status.md'))
    sources = (ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh',
               here / 'static_coupled_driver.rs', here / 'canonical_worked_recipe_cases.tsv',
               *helpers, *mutable, *books)
    assert len(sources) == 20, 'D156 static physical source population'
    fixtures = [(path.relative_to(ROOT), path.read_bytes()) for path in sources]
    fixtures += [(Path('scripts/local_environment.py'), SOURCE.read_bytes()),
                 (entry_path.relative_to(ROOT), text.encode())]
    scratch = GOOD['directory'](ROOT, 'target/scratch/static_file_refusals', create=True)
    saved, argv, count = dict(os.environ), sys.argv, 0
    for mode in (False, True):
        cases = ['first-link', 'last-link', 'first-hardlink', 'last-hardlink',
                 'file-directory', 'late-source']
        cases += ['driver-link', 'first-mutable-link', 'last-mutable-link'] if mode else ['readonly-links']
        for case in cases:
            with tempfile.TemporaryDirectory(dir=scratch) as name:
                root = Path(name)
                destinations = [root / path for path, _ in fixtures]
                GOOD['plan_producer'](ROOT, output_files=destinations)
                for (relative, data), destination in zip(fixtures, destinations):
                    destination.parent.mkdir(parents=True, exist_ok=True)
                    destination.write_bytes(data)
                work = root / 'target/static_coupled_contract'
                first = work / 'baseline/cargo-artifacts.json'
                driver = work / 'baseline/driver'
                last = work / 'reference-whole-phase-interleaved.log' if mode else driver
                for destination in (first, last, driver):
                    GOOD['plan_producer'](ROOT, output_files=(destination,))
                    destination.parent.mkdir(parents=True, exist_ok=True)
                victim = root / 'target/victim'
                aliases = (root / 'target/source-alias-1', root / 'target/source-alias-2')
                GOOD['plan_producer'](ROOT, output_files=(victim, *aliases))
                victim.write_bytes(b'unchanged isolated static alias victim')
                if case in ('first-link', 'last-link', 'driver-link'):
                    destination = first if case == 'first-link' else last if case == 'last-link' else driver
                    destination.symlink_to(victim)
                    wanted = 'symlink producer output'
                elif case in ('first-hardlink', 'last-hardlink'):
                    os.link(victim, first if case == 'first-hardlink' else last)
                    wanted = 'hard-linked producer output'
                elif case == 'file-directory':
                    first.mkdir()
                    wanted = 'producer output is not a local regular file'
                elif case == 'late-source':
                    (root / books[-1].relative_to(ROOT)).unlink()
                    wanted = 'producer source is not a local regular file'
                elif case in ('first-mutable-link', 'last-mutable-link'):
                    source = root / mutable[0 if case == 'first-mutable-link' else -1].relative_to(ROOT)
                    for alias in aliases:
                        os.link(source, alias)
                    assert source.stat().st_nlink == 3
                    wanted = 'hard-linked mutable producer source'
                else:
                    assert not mode
                    for index, original in enumerate(mutable):
                        source = root / original.relative_to(ROOT)
                        for number in (1, 2):
                            alias = root / ('target/readonly-%d-%d' % (index, number))
                            GOOD['plan_producer'](ROOT, output_files=(alias,))
                            os.link(source, alias)
                        assert source.stat().st_nlink == 3
                    unused = work / 'reference-whole-phase-interleaved.log'
                    GOOD['plan_producer'](ROOT, output_files=(unused,))
                    unused.symlink_to(victim)
                before = {path: path.read_bytes() for path in root.rglob('*')
                          if path.is_file() and not path.is_symlink()}
                environment = dict(os.environ)
                mkdir, read_text, read_bytes, write_text, write_bytes, run = (
                    Path.mkdir, Path.read_text, Path.read_bytes, Path.write_text, Path.write_bytes, subprocess.run)
                diagnostic = io.StringIO()
                def no_effect(*args, **kwargs):
                    raise AssertionError('D156 static physical plan reached a read/write/child')
                entry = root / entry_path.relative_to(ROOT)
                try:
                    sys.argv = [str(entry), *(['--mutations'] if mode else [])]
                    Path.read_text, Path.read_bytes = no_effect, no_effect
                    Path.write_text, Path.write_bytes, subprocess.run = no_effect, no_effect, no_effect
                    if case == 'readonly-links':
                        tree = ast.parse(text)
                        boundary = next(node.end_lineno for node in tree.body if isinstance(node, ast.Expr)
                                        and isinstance(node.value, ast.Call)
                                        and ast.unparse(node.value.func) == "PROFILE['enter_producer']")
                        prefix = '\n'.join(text.splitlines()[:boundary]) + '\n'
                        scope = {'__file__': str(entry), '__name__': '__main__'}
                        exec(compile(prefix, str(entry), 'exec'), scope)
                        assert scope['MUTABLE_SOURCES'] == () and len(scope['OUTPUT_FILES']) == 7
                        assert (root / 'target/cargo-home').is_dir() and unused.is_symlink()
                    else:
                        Path.mkdir = no_effect
                        with contextlib.redirect_stderr(diagnostic):
                            try:
                                exec(compile(text, str(entry), 'exec'), {'__file__': str(entry), '__name__': '__main__'})
                            except SystemExit as error:
                                assert error.code == 2, 'D156 static physical refusal status'
                            else:
                                raise AssertionError('D156 static physical invalid entry accepted')
                finally:
                    after_environment = dict(os.environ)
                    Path.mkdir, Path.read_text, Path.read_bytes = mkdir, read_text, read_bytes
                    Path.write_text, Path.write_bytes, subprocess.run = write_text, write_bytes, run
                    os.environ.clear(); os.environ.update(saved); sys.argv = argv
                if case != 'readonly-links':
                    assert diagnostic.getvalue() == 'local-environment: REFUSED — ' + wanted + '\n', \
                        ('D156 static physical refusal diagnostic', mode, case, diagnostic.getvalue())
                    assert after_environment == environment and not (root / 'target/cargo-home').exists(), \
                        'D156 static physical refusal activated stores'
                    count += 1
                assert all(path.read_bytes() == data for path, data in before.items()), \
                    'D156 static physical plan changed bytes'
    assert count == 15
    return count


def main():
    original, entry = SOURCE.read_bytes(), ENTRY.read_bytes()
    count = controls(GOOD)
    output_count = output_controls(GOOD)
    mutable_count = mutable_controls(GOOD)
    faults = (
        ('output area', "require(Path(relative).parts[:1] == ('target',), 'producer output must be under target')", 'pass'),
        ('source link', "require(not path.is_symlink(), 'symlink producer source')", 'pass'),
        ('source parent', 'directory(root, Path(relative).parent.as_posix())', 'pass'),
        ('source device', "require(path.is_file() and path.stat().st_dev == root.stat().st_dev,\n                'producer source is not a local regular file')", "require(path.is_file(), 'producer source is not a local regular file')"),
        ('early stores', 'environment_for(root, os.environ, create=False)', 'environment_for(root, os.environ)'),
        ('activation', 'os.environ.update(environment)', 'pass'),
    )
    red = 0
    measure_paths = (ENTRY, ROOT / 'docs/tasks/artifacts/size_chart/size_chart_mutations.py',
                     ROOT / 'docs/tasks/artifacts/measurement_table/table_mutations.py',
                     ROOT / 'docs/tasks/artifacts/mtm_chart/mtm_chart_mutations.py',
                     ROOT / 'docs/tasks/artifacts/size_chart_collection/size_chart_collection_mutations.py',
                     ROOT / 'docs/tasks/artifacts/ease/ease_mutations.py',
                     ROOT / 'docs/tasks/artifacts/ease/ease_set_mutations.py',
                     ROOT / 'docs/tasks/artifacts/formula_structure/round_mutations.py',
                     ROOT / 'docs/tasks/artifacts/formula_structure/unsigned_round_mutations.py',
                     ROOT / 'docs/tasks/artifacts/formula_structure/length_operator_mutations.py',
                     ROOT / 'docs/tasks/artifacts/formula_structure/domain_context_mutations.py',
                     ROOT / 'docs/tasks/artifacts/formula_lex/formula_lex_mutations.py',
                     ROOT / 'docs/tasks/artifacts/formula_structure/semantic_mutations.py')
    measure_physical, measure_classifiers = 0, 0
    for measure_path in measure_paths:
        measure_text = measure_path.read_text()
        capture_measure_entry(measure_text, measure_path)
        measure_physical += measure_refusals(measure_text, measure_path)
        classifier_controls(measure_text, measure_path, strict=True)
        measure_classifiers += 1
        appended = {ROOT / 'docs/tasks/artifacts/ease/ease_mutations.py': 1,
                    ROOT / 'docs/tasks/artifacts/ease/ease_set_mutations.py': 2}.get(measure_path, 0)
        numbered = 'range(1, len(CASES) + %d)' % (appended + 1)
        omitted = 'range(1, len(CASES) + %d)' % appended if appended else 'range(1, len(CASES))'
        for name, before, after, message in (
            ('files omitted', 'output_files=OUTPUT_FILES', 'output_files=()', 'D156 measure file plan changed'),
            ('last numbered file omitted', numbered, omitted, 'D156 measure file plan changed'),
            ('mutable source omitted', 'mutable_sources=MUTABLE_SOURCES' if measure_path.name in {'domain_context_mutations.py', 'formula_lex_mutations.py', 'semantic_mutations.py'} else 'mutable_sources=(SOURCE,)', 'mutable_sources=()', 'D156 measure mutable plan changed'),
            ('test source omitted', 'sources=SOURCES', 'sources=SOURCES[:1]', 'D156 measure source plan changed'),
            ('child stores reset', 'environment = dict(os.environ)' if measure_path.name == 'semantic_mutations.py' else 'capture_output=True',
             "environment = dict(os.environ, CARGO_HOME='broken')" if measure_path.name == 'semantic_mutations.py' else
             "capture_output=True, env={'CARGO_HOME':'broken'}", 'D156 measure child stores lost'),
        ):
            child_count = {'semantic_mutations.py': 1, 'unsigned_round_mutations.py': 3}.get(measure_path.name, 2)
            assert measure_text.count(before) == (child_count if name == 'child stores reset' else 1), (name, 'actual measure anchor')
            try:
                capture_measure_entry(measure_text.replace(before, after, 1), measure_path)
            except AssertionError as error:
                assert str(error) == message, (name, 'unrelated measurement capture refusal', str(error))
                red += 1
            else:
                raise AssertionError((name, 'measurement body fault survived'))
        functions = {n.name: ast.get_source_segment(measure_text, n)
                     for n in ast.parse(measure_text).body if isinstance(n, ast.FunctionDef)}
        for name, replacement, message in (
            ('assertion_failure', "def assertion_failure(output):\n    return b'assertion' in output", 'D159 assertion evidence accepted noise'),
            ('assertion_failure', "def assertion_failure(output):\n    parts = output.split(b'\\nfailures:\\n', 2)\n    return len(parts) == 3 and re.search(rb'(?m)^assertion(?:[ :`]|$)', parts[1]) is not None", 'D159 assertion evidence accepted noise'),
            ('assertion_sites', 'def assertion_sites(source, path):\n    return set()', 'D160 assertion sites changed'),
        ):
            actual = functions[name]
            assert measure_text.count(actual) == 1
            try:
                classifier_controls(measure_text.replace(actual, replacement, 1), measure_path, strict=True)
            except AssertionError as error:
                assert str(error) == message, (name, 'unrelated measurement classifier refusal', str(error))
                red += 1
            else:
                raise AssertionError((name, 'measurement classifier fault survived'))
    recipe_path = ROOT / 'docs/tasks/artifacts/formula_structure/recipe_mutations.py'
    recipe_text = recipe_path.read_text()
    classifier_controls(recipe_text, recipe_path, strict=True)
    measure_classifiers += 1
    recipe_captures = 0
    for classifier_only, coupled in ((False, False), (False, True), (True, False)):
        capture_mixed_entry(recipe_text, recipe_path, classifier_only=classifier_only, coupled=coupled)
        recipe_captures += 1
        measure_physical += domain_refusals(recipe_text, recipe_path, classifier_only=classifier_only, coupled=coupled)
        recipe_faults = [('current input omitted %d' % index, 'sources=SOURCES',
                   'sources=SOURCES[:%d] + SOURCES[%d:]' % (index, index + 1),
                   'D156 measure source plan changed') for index in range(7)]
        if classifier_only:
            recipe_faults += [
                ('classifier files', 'OUTPUT_FILES = (() if CLASSIFIER_ONLY else',
                 'OUTPUT_FILES = (() if False else', 'D156 measure file plan changed'),
                ('classifier mutable', 'MUTABLE_SOURCES = () if CLASSIFIER_ONLY else MUTABLE_INPUTS',
                 'MUTABLE_SOURCES = () if False else MUTABLE_INPUTS', 'D156 measure mutable plan changed'),
                ('classifier effects', 'if CLASSIFIER_ONLY:', 'if False:',
                 'D156 classifier mode performed native effects')]
        else:
            recipe_faults += [
                ('files absent', 'output_files=OUTPUT_FILES', 'output_files=()', 'D156 measure file plan changed'),
                ('last file absent', 'len(coupled_cases if COUPLED else cases) + 1',
                 'len(coupled_cases if COUPLED else cases)', 'D156 measure file plan changed'),
                ('mutable first absent', 'mutable_sources=MUTABLE_SOURCES',
                 'mutable_sources=MUTABLE_SOURCES[1:]', 'D156 measure mutable plan changed'),
                ('mutable last absent', 'mutable_sources=MUTABLE_SOURCES',
                 'mutable_sources=MUTABLE_SOURCES[:-1]', 'D156 measure mutable plan changed'),
                ('stores reset', 'environment = dict(os.environ)',
                 "environment = dict(os.environ, CARGO_HOME='broken')", 'D156 measure child stores lost'),
                ('final restoration absent', '    for path, content in originals.items():\n        path.write_bytes(content)\n',
                 '', 'D156 domain restoration sequence changed')]
        for name, before, after, message in recipe_faults:
            assert recipe_text.count(before) == 1, (name, 'actual recipe anchor')
            try:
                capture_mixed_entry(recipe_text.replace(before, after, 1), recipe_path,
                                    classifier_only=classifier_only, coupled=coupled)
            except AssertionError as error:
                assert str(error) == message, (name, classifier_only, coupled, 'unrelated recipe refusal', str(error))
                red += 1
            else:
                raise AssertionError((name, classifier_only, coupled, 'recipe omission survived'))
    recipe_functions = {n.name: ast.get_source_segment(recipe_text, n) for n in ast.parse(recipe_text).body
                        if isinstance(n, ast.FunctionDef)}
    for name, replacement, message in (
        ('assertion_failure', "def assertion_failure(output):\n    return b'assertion' in output", 'D159 assertion evidence accepted noise'),
        ('assertion_failure', "def assertion_failure(output):\n    parts = output.split(b'\\nfailures:\\n', 2)\n    return len(parts) == 3 and re.search(rb'(?m)^assertion(?:[ :`]|$)', parts[1]) is not None", 'D159 assertion evidence accepted noise'),
        ('assertion_sites', 'def assertion_sites(source, path):\n    return set()', 'D160 assertion sites changed')):
        try:
            classifier_controls(recipe_text.replace(recipe_functions[name], replacement, 1), recipe_path, strict=True)
        except AssertionError as error:
            assert str(error) == message, (name, 'unrelated recipe classifier refusal', str(error))
            red += 1
        else:
            raise AssertionError((name, 'recipe classifier fault survived'))
    statement_path = ROOT / 'docs/tasks/artifacts/formula_structure/statement_mutations.py'
    statement_text = statement_path.read_text()
    classifier_controls(statement_text, statement_path, strict=True)
    measure_classifiers += 1
    statement_captures = 0
    for classifier_only in (False, True):
        capture_mixed_entry(statement_text, statement_path, classifier_only=classifier_only)
        statement_captures += 1
        measure_physical += domain_refusals(statement_text, statement_path, classifier_only=classifier_only)
        statement_faults = [('current input omitted %d' % index, 'sources=SOURCES',
                            'sources=SOURCES[:%d] + SOURCES[%d:]' % (index, index + 1),
                            'D156 measure source plan changed') for index in range(5)]
        if classifier_only:
            statement_faults += [
                ('classifier files', 'OUTPUT_FILES = (() if CLASSIFIER_ONLY else',
                 'OUTPUT_FILES = (() if False else', 'D156 measure file plan changed'),
                ('classifier mutable', 'MUTABLE_SOURCES = () if CLASSIFIER_ONLY else (SOURCE,)',
                 'MUTABLE_SOURCES = () if False else (SOURCE,)', 'D156 measure mutable plan changed'),
                ('classifier effects', 'if CLASSIFIER_ONLY:', 'if False:',
                 'D156 classifier mode performed native effects')]
        else:
            statement_faults += [
                ('files absent', 'output_files=OUTPUT_FILES', 'output_files=()', 'D156 measure file plan changed'),
                ('last file absent', 'len(cases) + 1', 'len(cases)', 'D156 measure file plan changed'),
                ('mutable absent', 'mutable_sources=MUTABLE_SOURCES', 'mutable_sources=()',
                 'D156 measure mutable plan changed'),
                ('stores reset', 'environment = dict(os.environ)',
                 "environment = dict(os.environ, CARGO_HOME='broken')", 'D156 measure child stores lost'),
                ('final restoration absent', 'finally:\n    SOURCE.write_bytes(ORIGINAL)\n',
                 'finally:\n', 'D156 domain restoration sequence changed')]
        for name, before, after, message in statement_faults:
            assert statement_text.count(before) == 1, (name, 'actual statement anchor')
            try:
                capture_mixed_entry(statement_text.replace(before, after, 1), statement_path,
                                    classifier_only=classifier_only)
            except AssertionError as error:
                assert str(error) == message, (name, classifier_only, 'unrelated statement refusal', str(error))
                red += 1
            else:
                raise AssertionError((name, classifier_only, 'statement omission survived'))
    statement_functions = {n.name: ast.get_source_segment(statement_text, n) for n in ast.parse(statement_text).body
                           if isinstance(n, ast.FunctionDef)}
    for name, replacement, message in (
        ('assertion_failure', "def assertion_failure(output):\n    return b'assertion' in output", 'D159 assertion evidence accepted noise'),
        ('assertion_failure', "def assertion_failure(output):\n    parts = output.split(b'\\nfailures:\\n', 2)\n    return len(parts) == 3 and re.search(rb'(?m)^assertion(?:[ :`]|$)', parts[1]) is not None", 'D159 assertion evidence accepted noise'),
        ('assertion_sites', 'def assertion_sites(source, path):\n    return set()', 'D160 assertion sites changed')):
        try:
            classifier_controls(statement_text.replace(statement_functions[name], replacement, 1), statement_path, strict=True)
        except AssertionError as error:
            assert str(error) == message, (name, 'unrelated statement classifier refusal', str(error))
            red += 1
        else:
            raise AssertionError((name, 'statement classifier fault survived'))
    literal_path = ROOT / 'docs/tasks/artifacts/formula_structure/literal_normalization_mutations.py'
    literal_text = literal_path.read_text()
    literal_captures = 0
    for classifier_only in (False, True):
        capture_mixed_entry(literal_text, literal_path, classifier_only=classifier_only)
        literal_captures += 1
        measure_physical += domain_refusals(literal_text, literal_path, classifier_only=classifier_only)
        literal_faults = [('current input omitted %d' % index, 'sources=SOURCES',
                          'sources=SOURCES[:%d] + SOURCES[%d:]' % (index, index + 1),
                          'D156 measure source plan changed') for index in range(4)]
        if classifier_only:
            literal_faults += [
                ('classifier files', 'OUTPUT_FILES = (() if CLASSIFIER_ONLY else',
                 'OUTPUT_FILES = (() if False else', 'D156 measure file plan changed'),
                ('classifier mutable', 'MUTABLE_SOURCES = () if CLASSIFIER_ONLY else (SOURCE,)',
                 'MUTABLE_SOURCES = () if False else (SOURCE,)', 'D156 measure mutable plan changed'),
                ('classifier effects', 'if CLASSIFIER_ONLY:', 'if False:',
                 'D156 classifier mode performed native effects')]
        else:
            literal_faults += [
                ('files absent', 'output_files=OUTPUT_FILES', 'output_files=()', 'D156 measure file plan changed'),
                ('last file absent', 'len(CASES) + 1', 'len(CASES)', 'D156 measure file plan changed'),
                ('mutable absent', 'mutable_sources=MUTABLE_SOURCES', 'mutable_sources=()',
                 'D156 measure mutable plan changed'),
                ('stores reset', 'environment = dict(os.environ)',
                 "environment = dict(os.environ, CARGO_HOME='broken')", 'D156 measure child stores lost'),
                ('final restoration absent', 'finally:\n    SOURCE.write_bytes(ORIGINAL)\n',
                 'finally:\n', 'D156 domain restoration sequence changed')]
        for name, before, after, message in literal_faults:
            assert literal_text.count(before) == 1, (name, 'actual literal anchor')
            try:
                capture_mixed_entry(literal_text.replace(before, after, 1), literal_path,
                                    classifier_only=classifier_only)
            except AssertionError as error:
                assert str(error) == message, (name, classifier_only, 'unrelated literal refusal', str(error))
                red += 1
            else:
                raise AssertionError((name, classifier_only, 'literal omission survived'))
    literal_function = next(ast.get_source_segment(literal_text, n) for n in ast.parse(literal_text).body
                            if isinstance(n, ast.FunctionDef) and n.name == 'assertion_failure')
    unanchored = "def assertion_failure(output):\n    parts = output.split(b'\\nfailures:\\n', 2)\n    if len(parts) != 3:\n        return False\n    locations = re.finditer(rb'panicked at ([^:\\n]+):([0-9]+):([0-9]+):\\n', parts[1])\n    return any((m.group(1).decode('utf-8', 'replace'), int(m.group(2)), int(m.group(3))) in ASSERTION_SITES for m in locations)"
    try:
        classifier_controls(literal_text.replace(literal_function, unanchored, 1), literal_path,
                            strict=True, thread_only=True)
    except AssertionError as error:
        assert str(error) == 'D159 assertion evidence accepted noise', ('D178 unrelated refusal', str(error))
        red += 1
    else:
        raise AssertionError('D178 actual nonthread-location classifier fault survived')
    normalized_path = ROOT / 'docs/tasks/artifacts/formula_structure/normalized_expression_mutations.py'
    normalized_text = normalized_path.read_text()
    normalized_captures = 0
    for classifier_only in (False, True):
        capture_mixed_entry(normalized_text, normalized_path, classifier_only=classifier_only)
        normalized_captures += 1
        measure_physical += domain_refusals(normalized_text, normalized_path, classifier_only=classifier_only)
        normalized_faults = [('current input omitted %d' % index, 'sources=SOURCES',
                             'sources=SOURCES[:%d] + SOURCES[%d:]' % (index, index + 1),
                             'D156 measure source plan changed') for index in range(5)]
        if classifier_only:
            normalized_faults += [
                ('classifier files', 'OUTPUT_FILES = (() if CLASSIFIER_ONLY else',
                 'OUTPUT_FILES = (() if False else', 'D156 measure file plan changed'),
                ('classifier mutable', 'MUTABLE_SOURCES = () if CLASSIFIER_ONLY else (SOURCE,)',
                 'MUTABLE_SOURCES = () if False else (SOURCE,)', 'D156 measure mutable plan changed'),
                ('classifier effects', 'if CLASSIFIER_ONLY:', 'if False:',
                 'D156 classifier mode performed native effects')]
        else:
            normalized_faults += [
                ('files absent', 'output_files=OUTPUT_FILES', 'output_files=()', 'D156 measure file plan changed'),
                ('last file absent', 'len(CASES) + 1', 'len(CASES)', 'D156 measure file plan changed'),
                ('mutable absent', 'mutable_sources=MUTABLE_SOURCES', 'mutable_sources=()',
                 'D156 measure mutable plan changed'),
                ('stores reset', 'environment = dict(os.environ)',
                 "environment = dict(os.environ, CARGO_HOME='broken')", 'D156 measure child stores lost'),
                ('final restoration absent', 'finally:\n    SOURCE.write_bytes(ORIGINAL)\n',
                 'finally:\n', 'D156 domain restoration sequence changed')]
        for name, before, after, message in normalized_faults:
            assert normalized_text.count(before) == 1, (name, 'actual normalized anchor')
            try:
                capture_mixed_entry(normalized_text.replace(before, after, 1), normalized_path,
                                    classifier_only=classifier_only)
            except AssertionError as error:
                assert str(error) == message, (name, classifier_only, 'unrelated normalized refusal', str(error))
                red += 1
            else:
                raise AssertionError((name, classifier_only, 'normalized omission survived'))
    normalized_function = next(ast.get_source_segment(normalized_text, n) for n in ast.parse(normalized_text).body
                               if isinstance(n, ast.FunctionDef) and n.name == 'assertion_failure')
    unanchored = "def assertion_failure(output):\n    parts = output.split(b'\\nfailures:\\n', 2)\n    if len(parts) != 3:\n        return False\n    locations = re.finditer(rb'panicked at ([^:\\n]+):([0-9]+):([0-9]+):\\n', parts[1])\n    return any((m.group(1).decode('utf-8', 'replace'), int(m.group(2)), int(m.group(3))) in ASSERTION_SITES for m in locations)"
    try:
        classifier_controls(normalized_text.replace(normalized_function, unanchored, 1), normalized_path,
                            strict=True, thread_only=True)
    except AssertionError as error:
        assert str(error) == 'D159 assertion evidence accepted noise', ('D178 unrelated refusal', str(error))
        red += 1
    else:
        raise AssertionError('D178 actual nonthread-location classifier fault survived')
    recipe_input_path = ROOT / 'docs/tasks/artifacts/formula_structure/normalized_recipe_mutations.py'
    recipe_input_text = recipe_input_path.read_text()
    recipe_input_captures = 0
    for classifier_only in (False, True):
        capture_mixed_entry(recipe_input_text, recipe_input_path, classifier_only=classifier_only)
        recipe_input_captures += 1
        measure_physical += domain_refusals(recipe_input_text, recipe_input_path, classifier_only=classifier_only)
        recipe_input_faults = [('current input omitted %d' % index, 'sources=SOURCES',
                             'sources=SOURCES[:%d] + SOURCES[%d:]' % (index, index + 1),
                             'D156 measure source plan changed') for index in range(7)]
        if classifier_only:
            recipe_input_faults += [
                ('classifier files', 'OUTPUT_FILES = (() if CLASSIFIER_ONLY else',
                 'OUTPUT_FILES = (() if False else', 'D156 measure file plan changed'),
                ('classifier mutable', 'MUTABLE_SOURCES = () if CLASSIFIER_ONLY else MUTATED',
                 'MUTABLE_SOURCES = () if False else MUTATED', 'D156 measure mutable plan changed'),
                ('classifier effects', 'if CLASSIFIER_ONLY:', 'if False:',
                 'D156 classifier mode performed native effects')]
        else:
            recipe_input_faults += [
                ('files absent', 'output_files=OUTPUT_FILES', 'output_files=()', 'D156 measure file plan changed'),
                ('last file absent', 'len(cases) + 1', 'len(cases)', 'D156 measure file plan changed'),
                ('stores reset', 'environment = dict(os.environ)',
                 "environment = dict(os.environ, CARGO_HOME='broken')", 'D156 measure child stores lost'),
                ('final restoration absent', '    for path, content in originals.items():\n        path.write_bytes(content)\n',
                 '', 'D156 domain restoration sequence changed')]
        if not classifier_only:
            recipe_input_faults += [('mutable source omitted %d' % index, 'mutable_sources=MUTABLE_SOURCES',
                                     'mutable_sources=MUTABLE_SOURCES[:%d] + MUTABLE_SOURCES[%d:]' % (index, index + 1),
                                     'D156 measure mutable plan changed') for index in range(3)]
        for name, before, after, message in recipe_input_faults:
            assert recipe_input_text.count(before) == 1, (name, 'actual normalized recipe anchor')
            try:
                capture_mixed_entry(recipe_input_text.replace(before, after, 1), recipe_input_path,
                                    classifier_only=classifier_only)
            except AssertionError as error:
                assert str(error) == message, (name, classifier_only, 'unrelated normalized recipe refusal', str(error))
                red += 1
            else:
                raise AssertionError((name, classifier_only, 'normalized recipe omission survived'))
    recipe_input_function = next(ast.get_source_segment(recipe_input_text, n) for n in ast.parse(recipe_input_text).body
                               if isinstance(n, ast.FunctionDef) and n.name == 'assertion_failure')
    unanchored = "def assertion_failure(output):\n    parts = output.split(b'\\nfailures:\\n', 2)\n    if len(parts) != 3:\n        return False\n    locations = re.finditer(rb'panicked at ([^:\\n]+):([0-9]+):([0-9]+):\\n', parts[1])\n    return any((m.group(1).decode('utf-8', 'replace'), int(m.group(2)), int(m.group(3))) in ASSERTION_SITES for m in locations)"
    try:
        classifier_controls(recipe_input_text.replace(recipe_input_function, unanchored, 1), recipe_input_path,
                            strict=True, thread_only=True)
    except AssertionError as error:
        assert str(error) == 'D159 assertion evidence accepted noise', ('D178 unrelated refusal', str(error))
        red += 1
    else:
        raise AssertionError('D178 actual nonthread-location classifier fault survived')
    identity_path = ROOT / 'docs/tasks/artifacts/formula_structure/canonical_recipe_mutations.py'
    identity_text = identity_path.read_text()
    identity_captures = 0
    for classifier_only in (False, True):
        capture_mixed_entry(identity_text, identity_path, classifier_only=classifier_only)
        identity_captures += 1
        measure_physical += domain_refusals(identity_text, identity_path, classifier_only=classifier_only)
        identity_faults = [('current input omitted %d' % index, 'sources=SOURCES',
                             'sources=SOURCES[:%d] + SOURCES[%d:]' % (index, index + 1),
                             'D156 measure source plan changed') for index in range(8)]
        if classifier_only:
            identity_faults += [
                ('classifier files', 'OUTPUT_FILES = (() if CLASSIFIER_ONLY else',
                 'OUTPUT_FILES = (() if False else', 'D156 measure file plan changed'),
                ('classifier mutable', 'MUTABLE_SOURCES = () if CLASSIFIER_ONLY else (SOURCE,)',
                 'MUTABLE_SOURCES = () if False else (SOURCE,)', 'D156 measure mutable plan changed'),
                ('classifier effects', 'if CLASSIFIER_ONLY:', 'if False:',
                 'D156 classifier mode performed native effects')]
        else:
            identity_faults += [
                ('files absent', 'output_files=OUTPUT_FILES', 'output_files=()', 'D156 measure file plan changed'),
                ('last file absent', 'len(cases) + 1', 'len(cases)', 'D156 measure file plan changed'),
                ('mutable absent', 'mutable_sources=MUTABLE_SOURCES', 'mutable_sources=()',
                 'D156 measure mutable plan changed'),
                ('stores reset', 'environment = dict(os.environ)',
                 "environment = dict(os.environ, CARGO_HOME='broken')", 'D156 measure child stores lost'),
                ('final restoration absent', 'finally:\n    SOURCE.write_bytes(original)\n',
                 'finally:\n', 'D156 domain restoration sequence changed')]
        for name, before, after, message in identity_faults:
            assert identity_text.count(before) == 1, (name, 'actual canonical recipe anchor')
            try:
                capture_mixed_entry(identity_text.replace(before, after, 1), identity_path,
                                    classifier_only=classifier_only)
            except AssertionError as error:
                assert str(error) == message, (name, classifier_only, 'unrelated canonical recipe refusal', str(error))
                red += 1
            else:
                raise AssertionError((name, classifier_only, 'canonical recipe omission survived'))
    identity_function = next(ast.get_source_segment(identity_text, n) for n in ast.parse(identity_text).body
                               if isinstance(n, ast.FunctionDef) and n.name == 'assertion_failure')
    unanchored = "def assertion_failure(output):\n    parts = output.split(b'\\nfailures:\\n', 2)\n    if len(parts) != 3:\n        return False\n    locations = re.finditer(rb'panicked at ([^:\\n]+):([0-9]+):([0-9]+):\\n', parts[1])\n    return any((m.group(1).decode('utf-8', 'replace'), int(m.group(2)), int(m.group(3))) in ASSERTION_SITES for m in locations)"
    try:
        classifier_controls(identity_text.replace(identity_function, unanchored, 1), identity_path,
                            strict=True, thread_only=True)
    except AssertionError as error:
        assert str(error) == 'D159 assertion evidence accepted noise', ('D178 unrelated refusal', str(error))
        red += 1
    else:
        raise AssertionError('D178 actual nonthread-location classifier fault survived')
    canonical_mode_path = ROOT / 'docs/tasks/artifacts/formula_structure/canonical_expression_mutations.py'
    canonical_mode_text = canonical_mode_path.read_text()
    canonical_mode_captures = 0
    for classifier_only, coupled in ((False, False), (False, True), (True, False)):
        capture_mixed_entry(canonical_mode_text, canonical_mode_path, classifier_only=classifier_only, coupled=coupled)
        canonical_mode_captures += 1
        measure_physical += domain_refusals(canonical_mode_text, canonical_mode_path, classifier_only=classifier_only, coupled=coupled)
        canonical_mode_faults = [('current input omitted %d' % index, 'sources=SOURCES',
                             'sources=SOURCES[:%d] + SOURCES[%d:]' % (index, index + 1),
                             'D156 measure source plan changed') for index in range(7)]
        if classifier_only:
            canonical_mode_faults += [
                ('classifier files', 'OUTPUT_FILES = (() if CLASSIFIER_ONLY else',
                 'OUTPUT_FILES = (() if False else', 'D156 measure file plan changed'),
                ('classifier mutable', 'MUTABLE_SOURCES = () if CLASSIFIER_ONLY else (SOURCE,)',
                 'MUTABLE_SOURCES = () if False else (SOURCE,)', 'D156 measure mutable plan changed'),
                ('classifier effects', 'if CLASSIFIER_ONLY:', 'if False:',
                 'D156 classifier mode performed native effects')]
        else:
            canonical_mode_faults += [
                ('files absent', 'output_files=OUTPUT_FILES', 'output_files=()', 'D156 measure file plan changed'),
                ('last file absent', 'len(CASES) + 1', 'len(CASES)', 'D156 measure file plan changed'),
                ('mutable absent', 'mutable_sources=MUTABLE_SOURCES', 'mutable_sources=()',
                 'D156 measure mutable plan changed'),
                ('stores reset', 'environment = dict(os.environ)',
                 "environment = dict(os.environ, CARGO_HOME='broken')", 'D156 measure child stores lost'),
                ('final restoration absent', 'finally:\n    SOURCE.write_bytes(ORIGINAL)\n',
                 'finally:\n', 'D156 domain restoration sequence changed')]
        if coupled:
            canonical_mode_faults.append(('coupled selector omitted',
                "        if coupled:\n            command.append('coupled_')",
                "        if False:\n            command.append('coupled_')",
                'D156 measure native argv changed'))
        for name, before, after, message in canonical_mode_faults:
            assert canonical_mode_text.count(before) == 1, (name, 'actual canonical expression anchor')
            try:
                capture_mixed_entry(canonical_mode_text.replace(before, after, 1), canonical_mode_path,
                                    classifier_only=classifier_only, coupled=coupled)
            except AssertionError as error:
                assert str(error) == message, (name, classifier_only, 'unrelated canonical expression refusal', str(error))
                red += 1
            else:
                raise AssertionError((name, classifier_only, 'canonical expression omission survived'))
    canonical_mode_function = next(ast.get_source_segment(canonical_mode_text, n) for n in ast.parse(canonical_mode_text).body
                               if isinstance(n, ast.FunctionDef) and n.name == 'assertion_failure')
    unanchored = "def assertion_failure(output):\n    parts = output.split(b'\\nfailures:\\n', 2)\n    if len(parts) != 3:\n        return False\n    locations = re.finditer(rb'panicked at ([^:\\n]+):([0-9]+):([0-9]+):\\n', parts[1])\n    return any((m.group(1).decode('utf-8', 'replace'), int(m.group(2)), int(m.group(3))) in ASSERTION_SITES for m in locations)"
    try:
        classifier_controls(canonical_mode_text.replace(canonical_mode_function, unanchored, 1), canonical_mode_path,
                            strict=True, thread_only=True)
    except AssertionError as error:
        assert str(error) == 'D159 assertion evidence accepted noise', ('D179 unrelated refusal', str(error))
        red += 1
    else:
        raise AssertionError('D179 actual nonthread-location classifier fault survived')
    checked_mode_path = ROOT / 'docs/tasks/artifacts/formula_structure/checked_expression_mutations.py'
    checked_mode_text = checked_mode_path.read_text()
    checked_mode_captures = 0
    for classifier_only in (False, True):
        capture_mixed_entry(checked_mode_text, checked_mode_path, classifier_only=classifier_only)
        checked_mode_captures += 1
        measure_physical += domain_refusals(checked_mode_text, checked_mode_path, classifier_only=classifier_only)
        checked_mode_faults = [('current input omitted %d' % index, 'sources=SOURCES',
                   'sources=SOURCES[:%d] + SOURCES[%d:]' % (index, index + 1),
                   'D156 measure source plan changed') for index in range(2)]
        if classifier_only:
            checked_mode_faults += [
                ('classifier files', 'OUTPUT_FILES = (() if CLASSIFIER_ONLY else',
                 'OUTPUT_FILES = (() if False else', 'D156 measure file plan changed'),
                ('classifier mutable', 'MUTABLE_SOURCES = () if CLASSIFIER_ONLY else (SOURCE,)',
                 'MUTABLE_SOURCES = () if False else (SOURCE,)', 'D156 measure mutable plan changed'),
                ('classifier effects', 'if CLASSIFIER_ONLY:', 'if False:',
                 'D156 classifier mode performed native effects')]
        else:
            checked_mode_faults += [
                ('files absent', 'output_files=OUTPUT_FILES', 'output_files=()', 'D156 measure file plan changed'),
                ('last file absent', 'len(CASES) + 1', 'len(CASES)', 'D156 measure file plan changed'),
                ('mutable absent', 'mutable_sources=MUTABLE_SOURCES', 'mutable_sources=()',
                 'D156 measure mutable plan changed'),
                ('stores reset', 'environment = dict(os.environ)',
                 "environment = dict(os.environ, CARGO_HOME='broken')", 'D156 measure child stores lost'),
                ('final restoration absent', 'finally:\n    SOURCE.write_bytes(original)\n',
                 'finally:\n', 'D156 domain restoration sequence changed'),
                ('second replacement omitted', '        for before, after in steps:\n',
                 '        for before, after in steps[:1]:\n', 'D156 measure source mutation bytes changed')]
        for name, before, after, message in checked_mode_faults:
            assert checked_mode_text.count(before) == 1, (name, 'actual checked expression anchor')
            try:
                capture_mixed_entry(checked_mode_text.replace(before, after, 1), checked_mode_path,
                                    classifier_only=classifier_only)
            except AssertionError as error:
                assert str(error) == message, (name, classifier_only, 'unrelated checked expression refusal', str(error))
                red += 1
            else:
                raise AssertionError((name, classifier_only, 'checked expression omission survived'))
    checked_mode_function = next(ast.get_source_segment(checked_mode_text, node)
                                 for node in ast.parse(checked_mode_text).body
                                 if isinstance(node, ast.FunctionDef) and node.name == 'assertion_failure')
    try:
        classifier_controls(checked_mode_text.replace(checked_mode_function, unanchored, 1), checked_mode_path,
                            strict=True, thread_only=True)
    except AssertionError as error:
        assert str(error) == 'D159 assertion evidence accepted noise', ('D179 unrelated refusal', str(error))
        red += 1
    else:
        raise AssertionError('D179 actual checked nonthread-location classifier fault survived')
    checked_stmt_mode_path = ROOT / 'docs/tasks/artifacts/formula_structure/checked_statement_mutations.py'
    checked_stmt_mode_text = checked_stmt_mode_path.read_text()
    checked_stmt_mode_captures = 0
    for classifier_only in (False, True):
        capture_mixed_entry(checked_stmt_mode_text, checked_stmt_mode_path, classifier_only=classifier_only)
        checked_stmt_mode_captures += 1
        measure_physical += domain_refusals(checked_stmt_mode_text, checked_stmt_mode_path, classifier_only=classifier_only)
        checked_stmt_mode_faults = [('current input omitted %d' % index, 'sources=SOURCES',
                   'sources=SOURCES[:%d] + SOURCES[%d:]' % (index, index + 1),
                   'D156 measure source plan changed') for index in range(4)]
        if classifier_only:
            checked_stmt_mode_faults += [
                ('classifier files', 'OUTPUT_FILES = (() if CLASSIFIER_ONLY else',
                 'OUTPUT_FILES = (() if False else', 'D156 measure file plan changed'),
                ('classifier mutable', 'MUTABLE_SOURCES = () if CLASSIFIER_ONLY else tuple(ROOT / path for path in PATHS)',
                 'MUTABLE_SOURCES = () if False else tuple(ROOT / path for path in PATHS)', 'D156 measure mutable plan changed'),
                ('classifier effects', 'if CLASSIFIER_ONLY:', 'if False:',
                 'D156 classifier mode performed native effects')]
        else:
            checked_stmt_mode_faults += [
                ('files absent', 'output_files=OUTPUT_FILES', 'output_files=()', 'D156 measure file plan changed'),
                ('last file absent', 'len(CASES) + 1', 'len(CASES)', 'D156 measure file plan changed'),
                ('mutable absent', 'mutable_sources=MUTABLE_SOURCES', 'mutable_sources=()',
                 'D156 measure mutable plan changed'),
                ('stores reset', 'environment = dict(os.environ)',
                 "environment = dict(os.environ, CARGO_HOME='broken')", 'D156 measure child stores lost'),
                ('final restoration absent', 'finally:\n    for path, data in ORIGINAL.items():\n        (ROOT / path).write_bytes(data)\n',
                 'finally:\n', 'D156 domain restoration sequence changed'),
                ('count-limited replacement lost', '.replace(before, after, count)',
                 '.replace(before, after, 1)', 'D156 measure source mutation bytes changed')]
        if not classifier_only:
            checked_stmt_mode_faults += [('mutable source omitted %d' % index, 'mutable_sources=MUTABLE_SOURCES',
                'mutable_sources=MUTABLE_SOURCES[:%d] + MUTABLE_SOURCES[%d:]' % (index, index + 1),
                'D156 measure mutable plan changed') for index in range(3)]
        for name, before, after, message in checked_stmt_mode_faults:
            assert checked_stmt_mode_text.count(before) == 1, (name, 'actual checked statement anchor')
            try:
                capture_mixed_entry(checked_stmt_mode_text.replace(before, after, 1), checked_stmt_mode_path,
                                    classifier_only=classifier_only)
            except AssertionError as error:
                assert str(error) == message, (name, classifier_only, 'unrelated checked statement refusal', str(error))
                red += 1
            else:
                raise AssertionError((name, classifier_only, 'checked statement omission survived'))
    checked_stmt_mode_function = next(ast.get_source_segment(checked_stmt_mode_text, node)
                                 for node in ast.parse(checked_stmt_mode_text).body
                                 if isinstance(node, ast.FunctionDef) and node.name == 'assertion_failure')
    try:
        classifier_controls(checked_stmt_mode_text.replace(checked_stmt_mode_function, unanchored, 1), checked_stmt_mode_path,
                            strict=True, thread_only=True)
    except AssertionError as error:
        assert str(error) == 'D159 assertion evidence accepted noise', ('D179 unrelated refusal', str(error))
        red += 1
    else:
        raise AssertionError('D179 actual checked nonthread-location classifier fault survived')
    checked_recipe_mode_path = ROOT / 'docs/tasks/artifacts/formula_structure/checked_recipe_mutations.py'
    checked_recipe_mode_text = checked_recipe_mode_path.read_text()
    checked_recipe_mode_captures = 0
    for classifier_only in (False, True):
        capture_mixed_entry(checked_recipe_mode_text, checked_recipe_mode_path, classifier_only=classifier_only)
        checked_recipe_mode_captures += 1
        measure_physical += domain_refusals(checked_recipe_mode_text, checked_recipe_mode_path, classifier_only=classifier_only)
        checked_recipe_mode_faults = [('current input omitted %d' % index, 'sources=SOURCES',
                   'sources=SOURCES[:%d] + SOURCES[%d:]' % (index, index + 1),
                   'D156 measure source plan changed') for index in range(2)]
        if classifier_only:
            checked_recipe_mode_faults += [
                ('classifier files', 'OUTPUT_FILES = (() if CLASSIFIER_ONLY else',
                 'OUTPUT_FILES = (() if False else', 'D156 measure file plan changed'),
                ('classifier mutable', 'MUTABLE_SOURCES = () if CLASSIFIER_ONLY else (SOURCE,)',
                 'MUTABLE_SOURCES = () if False else (SOURCE,)', 'D156 measure mutable plan changed'),
                ('classifier effects', 'if CLASSIFIER_ONLY:', 'if False:',
                 'D156 classifier mode performed native effects')]
        else:
            checked_recipe_mode_faults += [
                ('files absent', 'output_files=OUTPUT_FILES', 'output_files=()', 'D156 measure file plan changed'),
                ('last file absent', 'len(CASES) + 1', 'len(CASES)', 'D156 measure file plan changed'),
                ('mutable absent', 'mutable_sources=MUTABLE_SOURCES', 'mutable_sources=()',
                 'D156 measure mutable plan changed'),
                ('stores reset', 'environment = dict(os.environ)',
                 "environment = dict(os.environ, CARGO_HOME='broken')", 'D156 measure child stores lost'),
                ('final restoration absent', 'finally:\n    SOURCE.write_bytes(ORIGINAL)\n',
                 'finally:\n', 'D156 domain restoration sequence changed'),
                ('count-limited replacement lost', '.replace(before, after, count)',
                 '.replace(before, after, 1)', 'D156 measure source mutation bytes changed')]
        if not classifier_only:
            checked_recipe_mode_faults += [
                ('last count-limited replacement lost', '.replace(before, after, count)',
                 '.replace(before, after, (1 if index == 19 else count))', 'D156 measure source mutation bytes changed'),
                ('baseline file omitted', "else (WORK / 'baseline.log',) +", "else () +", 'D156 measure file plan changed'),
                ('baseline call omitted', "    baseline = subprocess.run(command, cwd=ROOT, env=environment, capture_output=True)\n    (WORK / 'baseline.log').write_bytes(baseline.stdout + baseline.stderr)\n    assert baseline.returncode == 0, 'whole-proof baseline failed'\n", '', 'D156 measure native source state changed')]
        for name, before, after, message in checked_recipe_mode_faults:
            assert checked_recipe_mode_text.count(before) == 1, (name, 'actual checked recipe anchor')
            try:
                capture_mixed_entry(checked_recipe_mode_text.replace(before, after, 1), checked_recipe_mode_path,
                                    classifier_only=classifier_only)
            except AssertionError as error:
                assert str(error) == message, (name, classifier_only, 'unrelated checked recipe refusal', str(error))
                red += 1
            else:
                raise AssertionError((name, classifier_only, 'checked recipe omission survived'))
    checked_recipe_mode_function = next(ast.get_source_segment(checked_recipe_mode_text, node)
                                 for node in ast.parse(checked_recipe_mode_text).body
                                 if isinstance(node, ast.FunctionDef) and node.name == 'assertion_failure')
    try:
        classifier_controls(checked_recipe_mode_text.replace(checked_recipe_mode_function, unanchored, 1), checked_recipe_mode_path,
                            strict=True, thread_only=True)
    except AssertionError as error:
        assert str(error) == 'D159 assertion evidence accepted noise', ('D179 unrelated refusal', str(error))
        red += 1
    else:
        raise AssertionError('D179 actual checked nonthread-location classifier fault survived')
    namespace_mode_path = ROOT / 'docs/tasks/artifacts/formula_structure/namespace_mutations.py'
    namespace_mode_text = namespace_mode_path.read_text()
    namespace_mode_captures = 0
    for classifier_only in (False, True):
        capture_mixed_entry(namespace_mode_text, namespace_mode_path, classifier_only=classifier_only)
        namespace_mode_captures += 1
        measure_physical += domain_refusals(namespace_mode_text, namespace_mode_path, classifier_only=classifier_only)
        namespace_mode_faults = [('current input omitted %d' % index, 'sources=SOURCES',
                   'sources=SOURCES[:%d] + SOURCES[%d:]' % (index, index + 1),
                   'D156 measure source plan changed') for index in range(2)]
        if classifier_only:
            namespace_mode_faults += [
                ('classifier files', 'OUTPUT_FILES = (() if CLASSIFIER_ONLY else',
                 'OUTPUT_FILES = (() if False else', 'D156 measure file plan changed'),
                ('classifier mutable', 'MUTABLE_SOURCES = () if CLASSIFIER_ONLY else (SOURCE,)',
                 'MUTABLE_SOURCES = () if False else (SOURCE,)', 'D156 measure mutable plan changed'),
                ('classifier effects', 'if CLASSIFIER_ONLY:', 'if False:',
                 'D156 classifier mode performed native effects')]
        else:
            namespace_mode_faults += [
                ('files absent', 'output_files=OUTPUT_FILES', 'output_files=()', 'D156 measure file plan changed'),
                ('last file absent', 'len(CASES) + 1', 'len(CASES)', 'D156 measure file plan changed'),
                ('mutable absent', 'mutable_sources=MUTABLE_SOURCES', 'mutable_sources=()',
                 'D156 measure mutable plan changed'),
                ('stores reset', 'environment = dict(os.environ)',
                 "environment = dict(os.environ, CARGO_HOME='broken')", 'D156 measure child stores lost'),
                ('final restoration absent', 'finally:\n    SOURCE.write_bytes(original)\n',
                 'finally:\n', 'D156 domain restoration sequence changed'),
                ('mutation bytes changed', '.replace(before, after, 1)',
                 '.replace(before, before, 1)', 'D156 measure source mutation bytes changed')]
        for name, before, after, message in namespace_mode_faults:
            assert namespace_mode_text.count(before) == 1, (name, 'actual namespace anchor')
            try:
                capture_mixed_entry(namespace_mode_text.replace(before, after, 1), namespace_mode_path,
                                    classifier_only=classifier_only)
            except AssertionError as error:
                assert str(error) == message, (name, classifier_only, 'unrelated namespace refusal', str(error))
                red += 1
            else:
                raise AssertionError((name, classifier_only, 'namespace omission survived'))
    namespace_mode_function = next(ast.get_source_segment(namespace_mode_text, node)
                                  for node in ast.parse(namespace_mode_text).body
                                  if isinstance(node, ast.FunctionDef) and node.name == 'assertion_failure')
    try:
        classifier_controls(namespace_mode_text.replace(namespace_mode_function, unanchored, 1), namespace_mode_path,
                            strict=True, thread_only=True)
    except AssertionError as error:
        assert str(error) == 'D159 assertion evidence accepted noise', ('D185 unrelated refusal', str(error))
        red += 1
    else:
        raise AssertionError('D185 actual namespace nonthread-location classifier fault survived')
    ordered_mode_path = ROOT / 'docs/tasks/artifacts/formula_structure/ordered_name_mutations.py'
    ordered_mode_text = ordered_mode_path.read_text()
    ordered_mode_captures = 0
    for classifier_only in (False, True):
        capture_mixed_entry(ordered_mode_text, ordered_mode_path, classifier_only=classifier_only)
        ordered_mode_captures += 1
        measure_physical += domain_refusals(ordered_mode_text, ordered_mode_path, classifier_only=classifier_only)
        ordered_mode_faults = [('current input omitted %d' % index, 'sources=SOURCES',
                   'sources=SOURCES[:%d] + SOURCES[%d:]' % (index, index + 1),
                   'D156 measure source plan changed') for index in range(3)]
        if classifier_only:
            ordered_mode_faults += [
                ('classifier files', 'OUTPUT_FILES = (() if CLASSIFIER_ONLY else',
                 'OUTPUT_FILES = (() if False else', 'D156 measure file plan changed'),
                ('classifier mutable', 'MUTABLE_SOURCES = () if CLASSIFIER_ONLY else tuple(ROOT / path for path in PATHS)',
                 'MUTABLE_SOURCES = () if False else tuple(ROOT / path for path in PATHS)', 'D156 measure mutable plan changed'),
                ('classifier effects', 'if CLASSIFIER_ONLY:', 'if False:',
                 'D156 classifier mode performed native effects')]
        else:
            ordered_mode_faults += [
                ('files absent', 'output_files=OUTPUT_FILES', 'output_files=()', 'D156 measure file plan changed'),
                ('last file absent', 'len(CASES) + 1', 'len(CASES)', 'D156 measure file plan changed'),
                ('mutable absent', 'mutable_sources=MUTABLE_SOURCES', 'mutable_sources=()',
                 'D156 measure mutable plan changed'),
                ('stores reset', 'environment = dict(os.environ)',
                 "environment = dict(os.environ, CARGO_HOME='broken')", 'D156 measure child stores lost'),
                ('final restoration absent', 'finally:\n    for path, data in ORIGINAL.items():\n        (ROOT / path).write_bytes(data)\n',
                 'finally:\n', 'D156 domain restoration sequence changed'),
                ('mutation bytes changed', '.replace(before, after, 1)',
                 '.replace(before, before, 1)', 'D156 measure source mutation bytes changed')]
        if not classifier_only:
            ordered_mode_faults += [('mutable source omitted %d' % index, 'mutable_sources=MUTABLE_SOURCES',
                'mutable_sources=MUTABLE_SOURCES[:%d] + MUTABLE_SOURCES[%d:]' % (index, index + 1),
                'D156 measure mutable plan changed') for index in range(2)]
        for name, before, after, message in ordered_mode_faults:
            assert ordered_mode_text.count(before) == 1, (name, 'actual ordered-name anchor')
            try:
                capture_mixed_entry(ordered_mode_text.replace(before, after, 1), ordered_mode_path,
                                    classifier_only=classifier_only)
            except AssertionError as error:
                assert str(error) == message, (name, classifier_only, 'unrelated ordered-name refusal', str(error))
                red += 1
            else:
                raise AssertionError((name, classifier_only, 'ordered-name omission survived'))
    ordered_mode_function = next(ast.get_source_segment(ordered_mode_text, node)
                                  for node in ast.parse(ordered_mode_text).body
                                  if isinstance(node, ast.FunctionDef) and node.name == 'assertion_failure')
    try:
        classifier_controls(ordered_mode_text.replace(ordered_mode_function, unanchored, 1), ordered_mode_path,
                            strict=True, thread_only=True)
    except AssertionError as error:
        assert str(error) == 'D159 assertion evidence accepted noise', ('D186 unrelated refusal', str(error))
        red += 1
    else:
        raise AssertionError('D186 actual ordered-name nonthread-location classifier fault survived')
    declaration_mode_path = ROOT / 'docs/tasks/artifacts/formula_structure/declaration_mutations.py'
    declaration_mode_text = declaration_mode_path.read_text()
    declaration_mode_captures = 0
    for classifier_only in (False, True):
        capture_mixed_entry(declaration_mode_text, declaration_mode_path, classifier_only=classifier_only)
        declaration_mode_captures += 1
        measure_physical += domain_refusals(declaration_mode_text, declaration_mode_path, classifier_only=classifier_only)
        declaration_mode_faults = [('current input omitted %d' % index, 'sources=SOURCES',
                   'sources=SOURCES[:%d] + SOURCES[%d:]' % (index, index + 1),
                   'D156 measure source plan changed') for index in range(2)]
        if classifier_only:
            declaration_mode_faults += [
                ('classifier files', 'OUTPUT_FILES = (() if CLASSIFIER_ONLY else',
                 'OUTPUT_FILES = (() if False else', 'D156 measure file plan changed'),
                ('classifier mutable', 'MUTABLE_SOURCES = () if CLASSIFIER_ONLY else (SOURCE,)',
                 'MUTABLE_SOURCES = () if False else (SOURCE,)', 'D156 measure mutable plan changed'),
                ('classifier effects', 'if CLASSIFIER_ONLY:', 'if False:',
                 'D156 classifier mode performed native effects')]
        else:
            declaration_mode_faults += [
                ('files absent', 'output_files=OUTPUT_FILES', 'output_files=()', 'D156 measure file plan changed'),
                ('last file absent', 'len(CASES) + 1', 'len(CASES)', 'D156 measure file plan changed'),
                ('mutable absent', 'mutable_sources=MUTABLE_SOURCES', 'mutable_sources=()',
                 'D156 measure mutable plan changed'),
                ('stores reset', 'environment = dict(os.environ)',
                 "environment = dict(os.environ, CARGO_HOME='broken')", 'D156 measure child stores lost'),
                ('final restoration absent', 'finally:\n    SOURCE.write_bytes(original)\n',
                 'finally:\n', 'D156 measure native source state changed'),
                ('mutation bytes changed', '.replace(before, after, 1)',
                 '.replace(before, before, 1)', 'D156 measure source mutation bytes changed'),
                ('domain count2 lost',
                 ".replace('origin: FormulaScalarInputOrigin,', 'origin: FormulaInputOrigin,')",
                 ".replace('origin: FormulaScalarInputOrigin,', 'origin: FormulaInputOrigin,', 1)",
                 'D156 measure source mutation bytes changed')]
            tree = ast.parse(declaration_mode_text)
            doc_calls = [node for node in ast.walk(tree) if isinstance(node, ast.Assign)
                         and isinstance(node.value, ast.Call)
                         and ast.unparse(node.value.func) == 'subprocess.run'
                         and any(isinstance(value, ast.Constant) and value.value == '--doc'
                                 for value in ast.walk(node.value))]
            assert len(doc_calls) == 2, 'D188 declaration negative/restored doc calls changed'
            negative = next(ast.get_source_segment(declaration_mode_text, node.value)
                            for node in doc_calls if any(isinstance(t, ast.Name) and t.id == 'result' for t in node.targets))
            restored_docs = next(ast.get_source_segment(declaration_mode_text, node.value)
                                 for node in doc_calls if any(isinstance(t, ast.Name) and t.id == 'restored_docs' for t in node.targets))
            declaration_mode_faults += [
                ('negative construction selection changed', negative,
                 negative.replace('recipe::declaration::FormulaDeclaration', 'recipe::declaration::Wrong'),
                 'D156 measure native argv changed'),
                ('restored doc child omitted', restored_docs,
                 "subprocess.CompletedProcess([], 0, stdout=b'restored dispatch\\n', stderr=b'')",
                 'D156 measure diagnostic order changed')]
        for name, before, after, message in declaration_mode_faults:
            assert declaration_mode_text.count(before) == 1, (name, 'actual declaration anchor')
            try:
                capture_mixed_entry(declaration_mode_text.replace(before, after, 1), declaration_mode_path,
                                    classifier_only=classifier_only)
            except AssertionError as error:
                assert str(error) == message, (name, classifier_only, 'unrelated declaration refusal', str(error))
                red += 1
            else:
                raise AssertionError((name, classifier_only, 'declaration omission survived'))
    declaration_mode_function = next(ast.get_source_segment(declaration_mode_text, node)
                                  for node in ast.parse(declaration_mode_text).body
                                  if isinstance(node, ast.FunctionDef) and node.name == 'assertion_failure')
    try:
        classifier_controls(declaration_mode_text.replace(declaration_mode_function, unanchored, 1), declaration_mode_path,
                            strict=True, thread_only=True)
    except AssertionError as error:
        assert str(error) == 'D159 assertion evidence accepted noise', ('D188 unrelated refusal', str(error))
        red += 1
    else:
        raise AssertionError('D188 actual declaration nonthread-location classifier fault survived')
    name_read_mode_path = ROOT / 'docs/tasks/artifacts/formula_structure/name_read_mutations.py'
    name_read_mode_text = name_read_mode_path.read_text()
    name_read_mode_captures = 0
    for classifier_only in (False, True):
        capture_mixed_entry(name_read_mode_text, name_read_mode_path, classifier_only=classifier_only)
        name_read_mode_captures += 1
        measure_physical += domain_refusals(name_read_mode_text, name_read_mode_path, classifier_only=classifier_only)
        name_read_mode_faults = [('current input omitted %d' % index, 'sources=SOURCES',
                   'sources=SOURCES[:%d] + SOURCES[%d:]' % (index, index + 1),
                   'D156 measure source plan changed') for index in range(2)]
        if classifier_only:
            name_read_mode_faults += [
                ('classifier files', 'OUTPUT_FILES = (() if CLASSIFIER_ONLY else',
                 'OUTPUT_FILES = (() if False else', 'D156 measure file plan changed'),
                ('classifier mutable', 'MUTABLE_SOURCES = () if CLASSIFIER_ONLY else (SOURCE,)',
                 'MUTABLE_SOURCES = () if False else (SOURCE,)', 'D156 measure mutable plan changed'),
                ('classifier effects', 'if CLASSIFIER_ONLY:', 'if False:',
                 'D156 classifier mode performed native effects')]
        else:
            name_read_mode_faults += [
                ('files absent', 'output_files=OUTPUT_FILES', 'output_files=()', 'D156 measure file plan changed'),
                ('last file absent', 'len(CASES) + 1', 'len(CASES)', 'D156 measure file plan changed'),
                ('mutable absent', 'mutable_sources=MUTABLE_SOURCES', 'mutable_sources=()',
                 'D156 measure mutable plan changed'),
                ('stores reset', 'environment = dict(os.environ)',
                 "environment = dict(os.environ, CARGO_HOME='broken')", 'D156 measure child stores lost'),
                ('final restoration absent', 'finally:\n    SOURCE.write_bytes(original)\n',
                 'finally:\n', 'D156 domain restoration sequence changed'),
                ('mutation bytes changed', '.replace(before, after, 1)',
                 '.replace(before, before, 1)', 'D156 measure source mutation bytes changed')]
        for name, before, after, message in name_read_mode_faults:
            assert name_read_mode_text.count(before) == 1, (name, 'actual exact-name anchor')
            try:
                capture_mixed_entry(name_read_mode_text.replace(before, after, 1), name_read_mode_path,
                                    classifier_only=classifier_only)
            except AssertionError as error:
                assert str(error) == message, (name, classifier_only, 'unrelated exact-name refusal', str(error))
                red += 1
            else:
                raise AssertionError((name, classifier_only, 'exact-name omission survived'))
    name_read_mode_function = next(ast.get_source_segment(name_read_mode_text, node)
                                  for node in ast.parse(name_read_mode_text).body
                                  if isinstance(node, ast.FunctionDef) and node.name == 'assertion_failure')
    try:
        classifier_controls(name_read_mode_text.replace(name_read_mode_function, unanchored, 1), name_read_mode_path,
                            strict=True, thread_only=True)
    except AssertionError as error:
        assert str(error) == 'D159 assertion evidence accepted noise', ('D189 unrelated refusal', str(error))
        red += 1
    else:
        raise AssertionError('D189 actual exact-name nonthread-location classifier fault survived')
    builtin_mode_path = ROOT / 'docs/tasks/artifacts/formula_structure/builtin_signature_mutations.py'
    builtin_mode_text = builtin_mode_path.read_text()
    builtin_mode_captures = 0
    for classifier_only in (False, True):
        capture_mixed_entry(builtin_mode_text, builtin_mode_path, classifier_only=classifier_only)
        builtin_mode_captures += 1
        measure_physical += domain_refusals(builtin_mode_text, builtin_mode_path, classifier_only=classifier_only)
        builtin_mode_faults = [('current input omitted %d' % index, 'sources=SOURCES',
                   'sources=SOURCES[:%d] + SOURCES[%d:]' % (index, index + 1),
                   'D156 measure source plan changed') for index in range(3)]
        if classifier_only:
            builtin_mode_faults += [
                ('classifier files', 'OUTPUT_FILES = (() if CLASSIFIER_ONLY else',
                 'OUTPUT_FILES = (() if False else', 'D156 measure file plan changed'),
                ('classifier mutable', 'MUTABLE_SOURCES = () if CLASSIFIER_ONLY else (SOURCE,)',
                 'MUTABLE_SOURCES = () if False else (SOURCE,)', 'D156 measure mutable plan changed'),
                ('classifier effects', 'if CLASSIFIER_ONLY:', 'if False:',
                 'D156 classifier mode performed native effects')]
        else:
            builtin_mode_faults += [
                ('files absent', 'output_files=OUTPUT_FILES', 'output_files=()', 'D156 measure file plan changed'),
                ('last file absent', 'len(CASES) + 1', 'len(CASES)', 'D156 measure file plan changed'),
                ('mutable absent', 'mutable_sources=MUTABLE_SOURCES', 'mutable_sources=()',
                 'D156 measure mutable plan changed'),
                ('stores reset', 'environment = dict(os.environ)',
                 "environment = dict(os.environ, CARGO_HOME='broken')", 'D156 measure child stores lost'),
                ('final restoration absent', 'finally:\n    SOURCE.write_bytes(original)\n',
                 'finally:\n', 'D156 domain restoration sequence changed'),
                ('mutation bytes changed', '.replace(before, after, 1)',
                 '.replace(before, before, 1)', 'D156 measure source mutation bytes changed')]
        for name, before, after, message in builtin_mode_faults:
            assert builtin_mode_text.count(before) == 1, (name, 'actual builtin anchor')
            try:
                capture_mixed_entry(builtin_mode_text.replace(before, after, 1), builtin_mode_path,
                                    classifier_only=classifier_only)
            except AssertionError as error:
                assert str(error) == message, (name, classifier_only, 'unrelated builtin refusal', str(error))
                red += 1
            else:
                raise AssertionError((name, classifier_only, 'builtin omission survived'))
    builtin_mode_function = next(ast.get_source_segment(builtin_mode_text, node)
                                  for node in ast.parse(builtin_mode_text).body
                                  if isinstance(node, ast.FunctionDef) and node.name == 'assertion_failure')
    try:
        classifier_controls(builtin_mode_text.replace(builtin_mode_function, unanchored, 1), builtin_mode_path,
                            strict=True, thread_only=True)
    except AssertionError as error:
        assert str(error) == 'D159 assertion evidence accepted noise', ('D192 unrelated refusal', str(error))
        red += 1
    else:
        raise AssertionError('D192 actual builtin nonthread-location classifier fault survived')
    operator_mode_path = ROOT / 'docs/tasks/artifacts/formula_structure/operator_signature_mutations.py'
    operator_mode_text = operator_mode_path.read_text()
    operator_mode_captures = 0
    for classifier_only in (False, True):
        capture_mixed_entry(operator_mode_text, operator_mode_path, classifier_only=classifier_only)
        operator_mode_captures += 1
        measure_physical += domain_refusals(operator_mode_text, operator_mode_path, classifier_only=classifier_only)
        operator_mode_faults = [('current input omitted %d' % index, 'sources=SOURCES',
                   'sources=SOURCES[:%d] + SOURCES[%d:]' % (index, index + 1),
                   'D156 measure source plan changed') for index in range(3)]
        if classifier_only:
            operator_mode_faults += [
                ('classifier files', 'OUTPUT_FILES = (() if CLASSIFIER_ONLY else',
                 'OUTPUT_FILES = (() if False else', 'D156 measure file plan changed'),
                ('classifier mutable', 'MUTABLE_SOURCES = () if CLASSIFIER_ONLY else (SOURCE,)',
                 'MUTABLE_SOURCES = () if False else (SOURCE,)', 'D156 measure mutable plan changed'),
                ('classifier effects', 'if CLASSIFIER_ONLY:', 'if False:',
                 'D156 classifier mode performed native effects')]
        else:
            operator_mode_faults += [
                ('files absent', 'output_files=OUTPUT_FILES', 'output_files=()', 'D156 measure file plan changed'),
                ('last file absent', 'len(CASES) + 1', 'len(CASES)', 'D156 measure file plan changed'),
                ('mutable absent', 'mutable_sources=MUTABLE_SOURCES', 'mutable_sources=()',
                 'D156 measure mutable plan changed'),
                ('stores reset', 'environment = dict(os.environ)',
                 "environment = dict(os.environ, CARGO_HOME='broken')", 'D156 measure child stores lost'),
                ('final restoration absent', 'finally:\n    SOURCE.write_bytes(original)\n',
                 'finally:\n', 'D156 domain restoration sequence changed'),
                ('mutation bytes changed', '.replace(before, after, 1)',
                 '.replace(before, before, 1)', 'D156 measure source mutation bytes changed')]
        for name, before, after, message in operator_mode_faults:
            assert operator_mode_text.count(before) == 1, (name, 'actual operator anchor')
            try:
                capture_mixed_entry(operator_mode_text.replace(before, after, 1), operator_mode_path,
                                    classifier_only=classifier_only)
            except AssertionError as error:
                assert str(error) == message, (name, classifier_only, 'unrelated operator refusal', str(error))
                red += 1
            else:
                raise AssertionError((name, classifier_only, 'operator omission survived'))
    operator_mode_function = next(ast.get_source_segment(operator_mode_text, node)
                                  for node in ast.parse(operator_mode_text).body
                                  if isinstance(node, ast.FunctionDef) and node.name == 'assertion_failure')
    try:
        classifier_controls(operator_mode_text.replace(operator_mode_function, unanchored, 1), operator_mode_path,
                            strict=True, thread_only=True)
    except AssertionError as error:
        assert str(error) == 'D159 assertion evidence accepted noise', ('D192 unrelated refusal', str(error))
        red += 1
    else:
        raise AssertionError('D192 actual operator nonthread-location classifier fault survived')
    call_mode_path = ROOT / 'docs/tasks/artifacts/formula_structure/call_lookup_mutations.py'
    call_mode_text = call_mode_path.read_text()
    call_mode_captures = 0
    for classifier_only in (False, True):
        capture_mixed_entry(call_mode_text, call_mode_path, classifier_only=classifier_only)
        call_mode_captures += 1
        measure_physical += domain_refusals(call_mode_text, call_mode_path, classifier_only=classifier_only)
        call_mode_faults = [('current input omitted %d' % index, 'sources=SOURCES',
                   'sources=SOURCES[:%d] + SOURCES[%d:]' % (index, index + 1),
                   'D156 measure source plan changed') for index in range(2)]
        if classifier_only:
            call_mode_faults += [
                ('classifier files', 'OUTPUT_FILES = (() if CLASSIFIER_ONLY else',
                 'OUTPUT_FILES = (() if False else', 'D156 measure file plan changed'),
                ('classifier mutable', 'MUTABLE_SOURCES = () if CLASSIFIER_ONLY else (SOURCE,)',
                 'MUTABLE_SOURCES = () if False else (SOURCE,)', 'D156 measure mutable plan changed'),
                ('classifier effects', 'if CLASSIFIER_ONLY:', 'if False:',
                 'D156 classifier mode performed native effects')]
        else:
            call_mode_faults += [
                ('files absent', 'output_files=OUTPUT_FILES', 'output_files=()', 'D156 measure file plan changed'),
                ('last file absent', 'len(CASES) + 1', 'len(CASES)', 'D156 measure file plan changed'),
                ('mutable absent', 'mutable_sources=MUTABLE_SOURCES', 'mutable_sources=()',
                 'D156 measure mutable plan changed'),
                ('stores reset', 'environment = dict(os.environ)',
                 "environment = dict(os.environ, CARGO_HOME='broken')", 'D156 measure child stores lost'),
                ('final restoration absent', 'finally:\n    SOURCE.write_bytes(original)\n',
                 'finally:\n', 'D156 domain restoration sequence changed'),
                ('mutation bytes changed', '.replace(before, after, 1)',
                 '.replace(before, before, 1)', 'D156 measure source mutation bytes changed')]
        for name, before, after, message in call_mode_faults:
            assert call_mode_text.count(before) == 1, (name, 'actual call anchor')
            try:
                capture_mixed_entry(call_mode_text.replace(before, after, 1), call_mode_path,
                                    classifier_only=classifier_only)
            except AssertionError as error:
                assert str(error) == message, (name, classifier_only, 'unrelated call refusal', str(error))
                red += 1
            else:
                raise AssertionError((name, classifier_only, 'call omission survived'))
    call_mode_function = next(ast.get_source_segment(call_mode_text, node)
                                  for node in ast.parse(call_mode_text).body
                                  if isinstance(node, ast.FunctionDef) and node.name == 'assertion_failure')
    try:
        classifier_controls(call_mode_text.replace(call_mode_function, unanchored, 1), call_mode_path,
                            strict=True, thread_only=True)
    except AssertionError as error:
        assert str(error) == 'D159 assertion evidence accepted noise', ('D192 unrelated refusal', str(error))
        red += 1
    else:
        raise AssertionError('D192 actual call nonthread-location classifier fault survived')
    expression_mode_path = ROOT / 'docs/tasks/artifacts/formula_structure/formula_expression_mutations.py'
    expression_mode_text = expression_mode_path.read_text()
    expression_mode_captures = 0
    for classifier_only in (False, True):
        capture_mixed_entry(expression_mode_text, expression_mode_path, classifier_only=classifier_only)
        expression_mode_captures += 1
        measure_physical += domain_refusals(expression_mode_text, expression_mode_path, classifier_only=classifier_only)
        expression_mode_faults = [('current input omitted %d' % index, 'sources=SOURCES',
                   'sources=SOURCES[:%d] + SOURCES[%d:]' % (index, index + 1),
                   'D156 measure source plan changed') for index in range(4)]
        if classifier_only:
            expression_mode_faults += [
                ('classifier files', 'OUTPUT_FILES = (() if CLASSIFIER_ONLY else',
                 'OUTPUT_FILES = (() if False else', 'D156 measure file plan changed'),
                ('classifier mutable', 'MUTABLE_SOURCES = () if CLASSIFIER_ONLY else (SOURCE,)',
                 'MUTABLE_SOURCES = () if False else (SOURCE,)', 'D156 measure mutable plan changed'),
                ('classifier effects', 'if CLASSIFIER_ONLY:', 'if False:',
                 'D156 classifier mode performed native effects')]
        else:
            expression_mode_faults += [
                ('files absent', 'output_files=OUTPUT_FILES', 'output_files=()', 'D156 measure file plan changed'),
                ('last file absent', 'len(CASES) + 1', 'len(CASES)', 'D156 measure file plan changed'),
                ('mutable absent', 'mutable_sources=MUTABLE_SOURCES', 'mutable_sources=()',
                 'D156 measure mutable plan changed'),
                ('stores reset', 'environment = dict(os.environ)',
                 "environment = dict(os.environ, CARGO_HOME='broken')", 'D156 measure child stores lost'),
                ('final restoration absent', 'finally:\n    SOURCE.write_bytes(original)\n',
                 'finally:\n', 'D156 domain restoration sequence changed'),
                ('mutation bytes changed', '.replace(before, after, 1)',
                 '.replace(before, before, 1)', 'D156 measure source mutation bytes changed')]
        for name, before, after, message in expression_mode_faults:
            assert expression_mode_text.count(before) == 1, (name, 'actual expression anchor')
            try:
                capture_mixed_entry(expression_mode_text.replace(before, after, 1), expression_mode_path,
                                    classifier_only=classifier_only)
            except AssertionError as error:
                assert str(error) == message, (name, classifier_only, 'unrelated expression refusal', str(error))
                red += 1
            else:
                raise AssertionError((name, classifier_only, 'expression omission survived'))
    expression_mode_function = next(ast.get_source_segment(expression_mode_text, node)
                                  for node in ast.parse(expression_mode_text).body
                                  if isinstance(node, ast.FunctionDef) and node.name == 'assertion_failure')
    try:
        classifier_controls(expression_mode_text.replace(expression_mode_function, unanchored, 1), expression_mode_path,
                            strict=True, thread_only=True)
    except AssertionError as error:
        assert str(error) == 'D159 assertion evidence accepted noise', ('D197 unrelated refusal', str(error))
        red += 1
    else:
        raise AssertionError('D197 actual expression nonthread-location classifier fault survived')
    reduction_mode_path = ROOT / 'docs/tasks/artifacts/formula_structure/reduction_boundary_mutations.py'
    reduction_mode_text = reduction_mode_path.read_text()
    reduction_mode_captures = 0
    for classifier_only in (False, True):
        capture_mixed_entry(reduction_mode_text, reduction_mode_path, classifier_only=classifier_only)
        reduction_mode_captures += 1
        measure_physical += domain_refusals(reduction_mode_text, reduction_mode_path, classifier_only=classifier_only)
        reduction_mode_faults = [('current input omitted %d' % index, 'sources=SOURCES',
                   'sources=SOURCES[:%d] + SOURCES[%d:]' % (index, index + 1),
                   'D156 measure source plan changed') for index in range(4)]
        if classifier_only:
            reduction_mode_faults += [
                ('classifier files', 'OUTPUT_FILES = (() if CLASSIFIER_ONLY else',
                 'OUTPUT_FILES = (() if False else', 'D156 measure file plan changed'),
                ('classifier mutable', 'MUTABLE_SOURCES = () if CLASSIFIER_ONLY else (SOURCE,)',
                 'MUTABLE_SOURCES = () if False else (SOURCE,)', 'D156 measure mutable plan changed'),
                ('classifier effects', 'if CLASSIFIER_ONLY:', 'if False:',
                 'D156 classifier mode performed native effects')]
        else:
            reduction_mode_faults += [
                ('files absent', 'output_files=OUTPUT_FILES', 'output_files=()', 'D156 measure file plan changed'),
                ('last file absent', 'len(CASES) + 1', 'len(CASES)', 'D156 measure file plan changed'),
                ('mutable absent', 'mutable_sources=MUTABLE_SOURCES', 'mutable_sources=()',
                 'D156 measure mutable plan changed'),
                ('stores reset', 'environment = dict(os.environ)',
                 "environment = dict(os.environ, CARGO_HOME='broken')", 'D156 measure child stores lost'),
                ('final restoration absent', 'finally:\n    SOURCE.write_bytes(original)\n',
                 'finally:\n', 'D156 domain restoration sequence changed'),
                ('mutation bytes changed', '.replace(before, after, 1)',
                 '.replace(before, before, 1)', 'D156 measure source mutation bytes changed')]
        for name, before, after, message in reduction_mode_faults:
            assert reduction_mode_text.count(before) == 1, (name, 'actual reduction anchor')
            try:
                capture_mixed_entry(reduction_mode_text.replace(before, after, 1), reduction_mode_path,
                                    classifier_only=classifier_only)
            except AssertionError as error:
                assert str(error) == message, (name, classifier_only, 'unrelated reduction refusal', str(error))
                red += 1
            else:
                raise AssertionError((name, classifier_only, 'reduction omission survived'))
    reduction_mode_function = next(ast.get_source_segment(reduction_mode_text, node)
                                  for node in ast.parse(reduction_mode_text).body
                                  if isinstance(node, ast.FunctionDef) and node.name == 'assertion_failure')
    try:
        classifier_controls(reduction_mode_text.replace(reduction_mode_function, unanchored, 1), reduction_mode_path,
                            strict=True, thread_only=True)
    except AssertionError as error:
        assert str(error) == 'D159 assertion evidence accepted noise', ('D197 unrelated refusal', str(error))
        red += 1
    else:
        raise AssertionError('D197 actual reduction nonthread-location classifier fault survived')
    input_mode_path = ROOT / 'docs/tasks/artifacts/formula_structure/recipe_input_review_mutations.py'
    input_mode_text = input_mode_path.read_text()
    input_mode_captures = 0
    for classifier_only in (False, True):
        capture_mixed_entry(input_mode_text, input_mode_path, classifier_only=classifier_only)
        input_mode_captures += 1
        measure_physical += domain_refusals(input_mode_text, input_mode_path, classifier_only=classifier_only)
        input_mode_faults = [('current input omitted %d' % index, 'sources=SOURCES',
                   'sources=SOURCES[:%d] + SOURCES[%d:]' % (index, index + 1),
                   'D156 measure source plan changed') for index in range(8)]
        if classifier_only:
            input_mode_faults += [
                ('classifier files', 'OUTPUT_FILES = (() if CLASSIFIER_ONLY else',
                 'OUTPUT_FILES = (() if False else', 'D156 measure file plan changed'),
                ('classifier mutable', 'MUTABLE_SOURCES = () if CLASSIFIER_ONLY else MUTATED',
                 'MUTABLE_SOURCES = () if False else MUTATED', 'D156 measure mutable plan changed'),
                ('classifier effects', 'if CLASSIFIER_ONLY:', 'if False:',
                 'D156 classifier mode performed native effects')]
        else:
            input_mode_faults += [
                ('files absent', 'output_files=OUTPUT_FILES', 'output_files=()', 'D156 measure file plan changed'),
                ('last file absent', 'len(cases) + 1', 'len(cases)', 'D156 measure file plan changed'),
                ('stores reset', 'environment = dict(os.environ)',
                 "environment = dict(os.environ, CARGO_HOME='broken')", 'D156 measure child stores lost'),
                ('final restoration absent', 'finally:\n    for path, content in originals.items():\n        path.write_bytes(content)\n',
                 'finally:\n', 'D156 domain restoration sequence changed'),
                ('mutation bytes changed', '.replace(before, after, 1)',
                 '.replace(before, before, 1)', 'D156 measure source mutation bytes changed')]
        if not classifier_only:
            input_mode_faults += [('mutable omitted %d' % index, 'mutable_sources=MUTABLE_SOURCES',
                                   'mutable_sources=MUTABLE_SOURCES[:%d] + MUTABLE_SOURCES[%d:]' % (index, index + 1),
                                   'D156 measure mutable plan changed') for index in range(4)]
        for name, before, after, message in input_mode_faults:
            assert input_mode_text.count(before) == 1, (name, 'actual input anchor')
            try:
                capture_mixed_entry(input_mode_text.replace(before, after, 1), input_mode_path,
                                    classifier_only=classifier_only)
            except AssertionError as error:
                assert str(error) == message, (name, classifier_only, 'unrelated input refusal', str(error))
                red += 1
            else:
                raise AssertionError((name, classifier_only, 'input omission survived'))
    input_mode_function = next(ast.get_source_segment(input_mode_text, node)
                                  for node in ast.parse(input_mode_text).body
                                  if isinstance(node, ast.FunctionDef) and node.name == 'assertion_failure')
    try:
        classifier_controls(input_mode_text.replace(input_mode_function, unanchored, 1), input_mode_path,
                            strict=True, thread_only=True)
    except AssertionError as error:
        assert str(error) == 'D159 assertion evidence accepted noise', ('D197 unrelated refusal', str(error))
        red += 1
    else:
        raise AssertionError('D197 actual input nonthread-location classifier fault survived')
    wanted_mode_path = ROOT / 'docs/tasks/artifacts/formula_structure/wanted_signature_mutations.py'
    wanted_mode_text = wanted_mode_path.read_text()
    wanted_mode_captures = 0
    for classifier_only in (False, True):
        capture_mixed_entry(wanted_mode_text, wanted_mode_path, classifier_only=classifier_only)
        wanted_mode_captures += 1
        measure_physical += domain_refusals(wanted_mode_text, wanted_mode_path, classifier_only=classifier_only)
        wanted_mode_faults = [('current input omitted %d' % index, 'sources=SOURCES',
                   'sources=SOURCES[:%d] + SOURCES[%d:]' % (index, index + 1),
                   'D156 measure source plan changed') for index in range(4)]
        if classifier_only:
            wanted_mode_faults += [
                ('classifier files', 'OUTPUT_FILES = (() if CLASSIFIER_ONLY else',
                 'OUTPUT_FILES = (() if False else', 'D156 measure file plan changed'),
                ('classifier mutable', 'MUTABLE_SOURCES = () if CLASSIFIER_ONLY else (SOURCE,)',
                 'MUTABLE_SOURCES = () if False else (SOURCE,)', 'D156 measure mutable plan changed'),
                ('classifier effects', 'if CLASSIFIER_ONLY:', 'if False:',
                 'D156 classifier mode performed native effects')]
        else:
            wanted_mode_faults += [
                ('files absent', 'output_files=OUTPUT_FILES', 'output_files=()', 'D156 measure file plan changed'),
                ('last file absent', 'len(CASES) + 1', 'len(CASES)', 'D156 measure file plan changed'),
                ('mutable absent', 'mutable_sources=MUTABLE_SOURCES', 'mutable_sources=()',
                 'D156 measure mutable plan changed'),
                ('stores reset', 'environment = dict(os.environ)',
                 "environment = dict(os.environ, CARGO_HOME='broken')", 'D156 measure child stores lost'),
                ('final restoration absent', 'finally:\n    SOURCE.write_bytes(original)\n',
                 'finally:\n', 'D156 domain restoration sequence changed'),
                ('mutation bytes changed', '.replace(before, after, 1)',
                 '.replace(before, before, 1)', 'D156 measure source mutation bytes changed')]
        for name, before, after, message in wanted_mode_faults:
            assert wanted_mode_text.count(before) == 1, (name, 'actual wanted anchor')
            try:
                capture_mixed_entry(wanted_mode_text.replace(before, after, 1), wanted_mode_path,
                                    classifier_only=classifier_only)
            except AssertionError as error:
                assert str(error) == message, (name, classifier_only, 'unrelated wanted refusal', str(error))
                red += 1
            else:
                raise AssertionError((name, classifier_only, 'wanted omission survived'))
    wanted_mode_function = next(ast.get_source_segment(wanted_mode_text, node)
                                  for node in ast.parse(wanted_mode_text).body
                                  if isinstance(node, ast.FunctionDef) and node.name == 'assertion_failure')
    try:
        classifier_controls(wanted_mode_text.replace(wanted_mode_function, unanchored, 1), wanted_mode_path,
                            strict=True, thread_only=True)
    except AssertionError as error:
        assert str(error) == 'D159 assertion evidence accepted noise', ('D192 unrelated refusal', str(error))
        red += 1
    else:
        raise AssertionError('D192 actual wanted nonthread-location classifier fault survived')
    semantic_path = ROOT / 'docs/tasks/artifacts/formula_structure/semantic_mutations.py'
    semantic_text = semantic_path.read_text()
    capture_mixed_entry(semantic_text, semantic_path, classifier_only=True)
    measure_physical += domain_refusals(semantic_text, semantic_path, classifier_only=True)
    for mode in (False, True):
        for name, before, after, message in (
            ('current test omitted', 'sources=SOURCES', 'sources=SOURCES[:1] + SOURCES[2:]', 'D156 measure source plan changed'),
            ('canonical book omitted', 'sources=SOURCES', 'sources=SOURCES[:-1]', 'D156 measure source plan changed'),
        ):
            assert semantic_text.count(before) == 1, (name, 'actual semantic anchor')
            try:
                capture_mixed_entry(semantic_text.replace(before, after, 1), semantic_path, classifier_only=mode)
            except AssertionError as error:
                assert str(error) == message, (name, mode, 'unrelated semantic refusal', str(error))
                red += 1
            else:
                raise AssertionError((name, mode, 'semantic omission survived'))
    for name, before, after, mode, message in (
        ('classifier output plan', 'OUTPUT_FILES = (() if CLASSIFIER_ONLY else', 'OUTPUT_FILES = (() if False else', True, 'D156 measure file plan changed'),
        ('classifier mutable plan', 'MUTABLE_SOURCES = () if CLASSIFIER_ONLY else (SOURCE,)', 'MUTABLE_SOURCES = () if False else (SOURCE,)', True, 'D156 measure mutable plan changed'),
        ('classifier native effects', 'if CLASSIFIER_ONLY:', 'if False:', True, 'D156 classifier mode performed native effects'),
        ('final semantic restoration', 'finally:\n    SOURCE.write_bytes(original)\n', 'finally:\n', False, 'D156 domain restoration sequence changed'),
    ):
        assert semantic_text.count(before) == 1, (name, 'actual semantic anchor')
        try:
            capture_mixed_entry(semantic_text.replace(before, after, 1), semantic_path, classifier_only=mode)
        except AssertionError as error:
            assert str(error) == message, (name, 'unrelated semantic refusal', str(error))
            red += 1
        else:
            raise AssertionError((name, 'semantic mode/restoration fault survived'))
    lexer_path = ROOT / 'docs/tasks/artifacts/formula_lex/formula_lex_mutations.py'
    lexer_text = lexer_path.read_text()
    lexer_scan_controls(lexer_text, lexer_path)
    for index in range(2):
        before = 'mutable_sources=MUTABLE_SOURCES'
        after = f'mutable_sources=MUTABLE_SOURCES[:{index}] + MUTABLE_SOURCES[{index + 1}:]'
        try:
            capture_mixed_entry(lexer_text.replace(before, after, 1), lexer_path)
        except AssertionError as error:
            assert str(error) == 'D156 measure mutable plan changed', ('lexer mutable omission', index, str(error))
            red += 1
        else:
            raise AssertionError(('lexer mutable omission survived', index))
    for name, before, after, message in (
        ('imported examples omitted', 'sources=SOURCES', 'sources=SOURCES[:-1]', 'D156 measure source plan changed'),
        ('final lexer restorations omitted', '    for path, original in ORIGINALS.items():\n        path.write_bytes(original)\n', '', 'D156 domain restoration sequence changed'),
    ):
        assert lexer_text.count(before) == 1, (name, 'actual lexer anchor')
        try:
            capture_mixed_entry(lexer_text.replace(before, after, 1), lexer_path)
        except AssertionError as error:
            assert str(error) == message, (name, 'unrelated lexer capture refusal', str(error))
            red += 1
        else:
            raise AssertionError((name, 'lexer omission survived'))
    actual = next(ast.get_source_segment(lexer_text, node) for node in ast.parse(lexer_text).body
                  if isinstance(node, ast.FunctionDef) and node.name == 'assertion_sites')
    changed = actual.replace('if match is not None:', "if match is not None and 'item.is_ok()' not in line:", 1)
    assert changed != actual
    try:
        lexer_scan_controls(lexer_text.replace(actual, changed, 1), lexer_path)
    except AssertionError as error:
        assert str(error) == 'D177 lexer scan macro absent', ('lexer scan-site omission', str(error))
        red += 1
    else:
        raise AssertionError('D177 lexer scan-site omission survived')
    domain_path = ROOT / 'docs/tasks/artifacts/formula_structure/domain_context_mutations.py'
    domain_text = domain_path.read_text()
    for index in range(5):
        before = 'mutable_sources=MUTABLE_SOURCES'
        after = f'mutable_sources=MUTABLE_SOURCES[:{index}] + MUTABLE_SOURCES[{index + 1}:]'
        try:
            capture_domain_entry(domain_text.replace(before, after, 1), domain_path)
        except AssertionError as error:
            assert str(error) == 'D156 measure mutable plan changed', ('domain mutable omission', index, str(error))
            red += 1
        else:
            raise AssertionError(('domain mutable omission survived', index))
    for index in range(3):
        before = 'for source in TEST_SOURCES))'
        after = f'for source in TEST_SOURCES[:{index}] + TEST_SOURCES[{index + 1}:]))'
        assert domain_text.count(before) == 1
        try:
            capture_domain_entry(domain_text.replace(before, after, 1), domain_path)
        except AssertionError as error:
            assert str(error) == 'D177 domain contract macro absent', ('domain macro-source omission', index, str(error))
            red += 1
        else:
            raise AssertionError(('domain macro-source omission survived', index))
    for name, before, after, message in (
        ('final source restorations omitted', '    for path, data in ORIGINAL.items():\n        (ROOT / path).write_bytes(data)\n', '', 'D156 domain restoration sequence changed'),
        ('core restore profile changed', "('core', ['cargo', 'test', '-p', 'sc-core', '--lib', 'domain_context_contracts'])", "('core', ['cargo', 'test', '-p', 'sc-units', '--lib', 'domain_context_contracts'])", 'D156 measure native argv changed'),
        ('core restoration child omitted', "        ('core', ['cargo', 'test', '-p', 'sc-core', '--lib', 'domain_context_contracts']),\n", '', 'D156 measure full capture incomplete'),
    ):
        assert domain_text.count(before) == 1, (name, 'actual domain anchor')
        try:
            capture_domain_entry(domain_text.replace(before, after, 1), domain_path)
        except AssertionError as error:
            assert str(error) == message, (name, 'unrelated domain capture refusal', str(error))
            red += 1
        else:
            raise AssertionError((name, 'domain restoration fault survived'))
    collection_path = ROOT / 'docs/tasks/artifacts/size_chart_collection/size_chart_collection_mutations.py'
    assert collection_path in measure_paths
    collection_text = collection_path.read_text()
    measure_inline_controls(collection_text, collection_path)
    assert collection_text.count('for before, after in mutations:') == 1
    try:
        capture_measure_entry(collection_text.replace('for before, after in mutations:',
                                                     'for before, after in mutations[:1]:', 1), collection_path)
    except AssertionError as error:
        assert str(error) == 'D156 measure source mutation bytes changed', ('collection multi-anchor fault', str(error))
        red += 1
    else:
        raise AssertionError('D177 collection second-anchor omission survived')
    actual = next(ast.get_source_segment(collection_text, node) for node in ast.parse(collection_text).body
                  if isinstance(node, ast.FunctionDef) and node.name == 'assertion_sites')
    leading = next(ast.get_source_segment(measure_paths[0].read_text(), node)
                   for node in ast.parse(measure_paths[0].read_text()).body
                   if isinstance(node, ast.FunctionDef) and node.name == 'assertion_sites')
    try:
        measure_inline_controls(collection_text.replace(actual, leading, 1), collection_path)
    except AssertionError as error:
        assert str(error) == 'D177 measure inline macro sites changed', ('collection inline fault', str(error))
        red += 1
    else:
        raise AssertionError('D177 measure inline site omission survived')
    set_path = ROOT / 'docs/tasks/artifacts/ease/ease_set_mutations.py'
    assert set_path in measure_paths
    set_text = set_path.read_text()
    measure_inline_controls(set_text, set_path)
    actual = next(ast.get_source_segment(set_text, node) for node in ast.parse(set_text).body
                  if isinstance(node, ast.FunctionDef) and node.name == 'assertion_sites')
    try:
        measure_inline_controls(set_text.replace(actual, leading, 1), set_path)
    except AssertionError as error:
        assert str(error) == 'D177 measure inline macro sites changed', ('Ease-set inline fault', str(error))
        red += 1
    else:
        raise AssertionError('D177 Ease-set inline site omission survived')
    for name in ('round_mutations.py', 'unsigned_round_mutations.py'):
        round_path = ROOT / 'docs/tasks/artifacts/formula_structure' / name
        assert round_path in measure_paths
        round_text = round_path.read_text()
        round_totality_controls(round_text, round_path)
        actual = next(ast.get_source_segment(round_text, node) for node in ast.parse(round_text).body
                      if isinstance(node, ast.FunctionDef) and node.name == 'assertion_sites')
        changed = actual.replace('if match is not None:', "if match is not None and 'result.is_ok()' not in line:", 1)
        assert changed != actual
        try:
            round_totality_controls(round_text.replace(actual, changed, 1), round_path)
        except AssertionError as error:
            assert str(error) == 'D177 round totality macro site absent', ('round totality site fault', str(error))
            red += 1
        else:
            raise AssertionError('D177 round totality site omission survived')
        assert round_text.count('sources=SOURCES') == 1
        try:
            capture_measure_entry(round_text.replace('sources=SOURCES', 'sources=SOURCES[:2]', 1), round_path)
        except AssertionError as error:
            assert str(error) == 'D156 measure source plan changed', ('round imported fixture omission', str(error))
            red += 1
        else:
            raise AssertionError('D177 round imported fixture omission survived')
    unsigned_path = ROOT / 'docs/tasks/artifacts/formula_structure/unsigned_round_mutations.py'
    unsigned_text = unsigned_path.read_text()
    for name, before, after, message in (
        ('release log plan', "WORK / 'release-remainder.log', ", '', 'D156 measure file plan changed'),
        ('debug restore plan', "WORK / 'restored-debug.log', ", '', 'D156 measure file plan changed'),
        ('release fault profile', "'sc-units', '--release', '--test'", "'sc-units', '--test'", 'D156 measure native argv changed'),
        ('premature fault restoration', "        if name == 'doubled remainder overflow':\n",
         "        if name == 'doubled remainder overflow':\n            SOURCE.write_bytes(ORIGINAL)\n", 'D156 measure native source state changed'),
        ('release restore profile', "('release', ['--release'])", "('release', [])", 'D156 measure native argv changed'),
        ('release restore omitted', "(('debug', []), ('release', ['--release']))", "(('debug', []),)", 'D156 measure full capture incomplete'),
    ):
        assert unsigned_text.count(before) == 1, (name, 'actual unsigned anchor')
        try:
            capture_measure_entry(unsigned_text.replace(before, after, 1), unsigned_path)
        except AssertionError as error:
            assert str(error) == message, (name, 'unrelated unsigned capture refusal', str(error))
            red += 1
        else:
            raise AssertionError((name, 'unsigned profile/file omission survived'))
    profile_functions = {node.name: ast.get_source_segment(original.decode(), node)
                         for node in ast.parse(original).body if isinstance(node, ast.FunctionDef)}
    mutable_faults = [
        ('output hard link', "require(path.stat().st_nlink == 1, 'hard-linked producer output')", 'pass'),
        ('mutable hard link', "require(path.stat().st_nlink == 1, 'hard-linked mutable producer source')", 'pass'),
        ('declared mutable subset', "require(mutable <= set(inputs), 'mutable producer source is not declared')", 'pass'),
        ('readonly input preserved', 'if relative in mutable:', 'if True:'),
    ]
    for function in ('prepare_producer', 'enter_producer'):
        actual = profile_functions[function]
        assert actual.count('mutable_sources=mutable_sources') == 1
        mutable_faults.append((function + ' mutable forwarding', actual,
                               actual.replace('mutable_sources=mutable_sources', 'mutable_sources=()')))
    for name, before, after in mutable_faults:
        assert original.decode().count(before) == 1, (name, 'actual mutable guard anchor')
        scope = {'__file__': str(SOURCE), '__name__': 'mutable_path_fault'}
        exec(compile(original.decode().replace(before, after, 1), str(SOURCE), 'exec'), scope)
        try:
            mutable_controls(scope)
        except AssertionError as error:
            assert 'D156 mutable' in str(error), (name, 'unrelated mutable guard fault', str(error))
            red += 1
        else:
            raise AssertionError((name, 'mutable guard fault survived'))
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
    for name, before, after in (
        ('file alias', "require(not path.is_symlink(), 'symlink producer output')", 'pass'),
        ('file device', "path.is_file() and path.stat().st_dev == root.stat().st_dev,\n                    'producer output is not a local regular file'", "path.is_file(),\n                    'producer output is not a local regular file'"),
        ('file parent', 'directory(root, Path(relative).parent.as_posix(), missing_ok=True)', 'pass'),
        ('file output area', 'files = [producer_output_path(root, path) for path in output_files]',
         'files = [producer_path(root, path) for path in output_files]'),
    ):
        assert original.decode().count(before) == 1, (name, 'actual file guard anchor')
        scope = {'__file__': str(SOURCE), '__name__': 'producer_output_fault'}
        exec(compile(original.decode().replace(before, after, 1), str(SOURCE), 'exec'), scope)
        try:
            output_controls(scope)
        except AssertionError as error:
            assert str(error).startswith('D156 Python output'), (name, 'unrelated file guard fault', str(error))
            red += 1
        else:
            raise AssertionError((name, 'file guard fault survived'))
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
        (ROOT / 'docs/tasks/artifacts/formula_structure/declaration_mutations.py', 'declaration_mutations'),
        (ROOT / 'docs/tasks/artifacts/formula_structure/name_read_mutations.py', 'name_read_mutations'),
        (ROOT / 'docs/tasks/artifacts/formula_structure/builtin_signature_mutations.py', 'builtin_signature_mutations'),
        (ROOT / 'docs/tasks/artifacts/formula_structure/operator_signature_mutations.py', 'operator_signature_mutations'),
        (ROOT / 'docs/tasks/artifacts/formula_structure/wanted_signature_mutations.py', 'wanted_signature_mutations'),
        (ROOT / 'docs/tasks/artifacts/formula_structure/call_lookup_mutations.py', 'call_lookup_mutations'),
        (ROOT / 'docs/tasks/artifacts/formula_structure/formula_expression_mutations.py', 'formula_expression_mutations'),
        (ROOT / 'docs/tasks/artifacts/formula_structure/reduction_boundary_mutations.py', 'formula_reduction_mutations'),
        (ROOT / 'docs/tasks/artifacts/formula_structure/recipe_input_review_mutations.py', 'recipe_input_review_mutations'),
        (ROOT / 'docs/tasks/artifacts/formula_structure/recipe_owner_contract.py', 'recipe_owner_contract'),
        (ROOT / 'docs/tasks/artifacts/formula_structure/statement_owner_contract.py', 'statement_owner_contract'),
        (ROOT / 'docs/tasks/artifacts/formula_structure/static_coupled_contract.py', 'static_coupled_contract'),
    )
    originals = {path: path.read_bytes() for path, _ in adopters}
    for path, work_name in adopters:
        text = path.read_text()
        multiple_sources = {
            'size_membership_mutations.py', 'size_chart_mutations.py', 'table_mutations.py', 'mtm_chart_mutations.py', 'size_chart_collection_mutations.py', 'ease_mutations.py', 'ease_set_mutations.py', 'round_mutations.py', 'unsigned_round_mutations.py', 'length_operator_mutations.py',
            'domain_context_mutations.py', 'formula_lex_mutations.py', 'semantic_mutations.py', 'recipe_mutations.py', 'statement_mutations.py',
            'normalized_recipe_mutations.py', 'canonical_expression_mutations.py',
            'checked_expression_mutations.py', 'checked_statement_mutations.py',
            'checked_recipe_mutations.py', 'namespace_mutations.py', 'ordered_name_mutations.py',
            'declaration_mutations.py', 'name_read_mutations.py',
            'literal_normalization_mutations.py', 'normalized_expression_mutations.py',
            'canonical_recipe_mutations.py',
            'builtin_signature_mutations.py', 'operator_signature_mutations.py',
            'wanted_signature_mutations.py', 'call_lookup_mutations.py',
            'formula_expression_mutations.py', 'reduction_boundary_mutations.py',
            'recipe_input_review_mutations.py',
            'recipe_owner_contract.py', 'statement_owner_contract.py',
            'static_coupled_contract.py',
        }
        source_argument = 'SOURCES' if path.name in multiple_sources else '(SOURCE,)'
        profile_name = "runpy.run_path(str(ROOT / 'scripts/local_environment.py'))['enter_producer']"
        tree = ast.parse(text)
        if path.name in ('recipe_owner_contract.py', 'statement_owner_contract.py', 'static_coupled_contract.py'):
            profile_name = "PROFILE['enter_producer']"
            bindings = [node for node in ast.walk(tree) if isinstance(node, ast.Assign)
                        and any(isinstance(target, ast.Name) and target.id == 'PROFILE'
                                for target in node.targets)]
            assert len(bindings) == 1 and bindings[0] in tree.body and \
                ast.unparse(bindings[0].value) == "runpy.run_path(str(ROOT / 'scripts/local_environment.py'))", \
                'D156 Python entry profile binding changed'
        calls = [node.value for node in tree.body if isinstance(node, ast.Expr)
                 and isinstance(node.value, ast.Call) and ast.unparse(node.value.func) == profile_name]
        assert len(calls) == 1, 'D156 Python entry profile-call identity'
        node = calls[0]
        if path.name in ('recipe_owner_contract.py', 'statement_owner_contract.py', 'static_coupled_contract.py'):
            assert bindings[0].lineno < node.lineno, 'D156 Python entry profile binding was late'
        source_keyword = next(keyword for keyword in node.keywords if keyword.arg == 'sources')
        assert ast.unparse(source_keyword.value) == source_argument, 'D156 Python authored source population'
        call = ast.get_source_segment(text, node)
        capture(text, path, work_name=work_name)
        assert not (ROOT / 'target/scratch/python_producer_contract/absent-source').exists()
        # Change only the actual keyword; mutable_sources may repeat its value.
        source_keyword.value = ast.parse(
            '(*' + source_argument + ", ROOT / 'target/scratch/python_producer_contract/absent-source')",
            mode='eval').body
        capture(text.replace(call, ast.unparse(node), 1),
                path, work_name=work_name, refusal=True)
        assert text.count(call) == 1
        try:
            capture(text.replace(call, '', 1), path, work_name=work_name)
        except AssertionError as error:
            assert str(error) == 'D156 Python standalone activation omitted'
            red += 1
        else:
            raise AssertionError('D156 Python standalone body fault survived')
    static = ROOT / 'docs/tasks/artifacts/formula_structure/static_coupled_contract.py'
    text = static.read_text()
    static_count = 0
    for mode, early in ((False, False), (True, False), (True, True)):
        capture_static_entry(text, static, mutations=mode, early_refusal=early)
        static_count += 1
    capture_static_entry(text, static, borrowed_label='first-phase')
    static_count += 1
    measure_physical += static_file_refusals(text, static)
    for name, before, after, mode, early, message in (
        ('complete book plan', '*BOOK_INPUTS)', '*BOOK_INPUTS[:-1])', False, False, 'D156 static input plan changed'),
        ('prepared stores', 'environment = dict(os.environ)', "environment = dict(os.environ, CARGO_HOME=str(ROOT / 'target/cargo-home'))", False, False, 'D156 static child stores lost'),
        ('public Cargo metadata', ", '--message-format=json'", '', False, False, 'D156 static child argv changed'),
        ('public driver source', "str(DRIVER), '-o'", "str(SOURCE), '-o'", False, False, 'D156 static child argv changed'),
        ('mutation mode', 'if sys.argv[1:]:\n        mutations()', 'if False:\n        mutations()', True, False, 'D172 static restored compiler dispatch absent'),
        ('early-refusal artifact restoration', "build_and_check('restored')", 'None', True, True, 'D172 static restored compiler dispatch absent'),
    ):
        assert text.count(before) == 1, (name, 'actual static anchor')
        try:
            capture_static_entry(text.replace(before, after, 1), static, mutations=mode, early_refusal=early)
        except AssertionError as error:
            assert str(error) == message, (name, 'unrelated static capture refusal', str(error))
            red += 1
        else:
            raise AssertionError((name, 'actual static body fault survived'))
    build_function = next(node for node in ast.parse(text).body if isinstance(node, ast.FunctionDef) and node.name == 'build_and_check')
    build_plan = next(node for node in ast.walk(build_function) if isinstance(node, ast.Call)
                      and ast.unparse(node.func) == "PROFILE['plan_producer']"
                      and any(keyword.arg == 'directories' for keyword in node.keywords))
    actual_build_plan = ast.get_source_segment(text, build_plan)
    for mode, count in ((False, 7), (True, 244)):
        faults = [
            ('per-build plan', actual_build_plan, 'None', 'D156 static build plan omitted'),
            ('complete initial files', 'output_files=OUTPUT_FILES', 'output_files=()', 'D156 static file plan changed'),
            ('complete library inputs', 'sources=(library, *sorted(library_files - {library}))', 'sources=(library,)', 'D156 static library inputs changed'),
            ('library plan', "PROFILE['plan_producer'](ROOT, sources=(library, *sorted(library_files - {library})))", 'None', 'D156 static library plan omitted'),
            ('library directories', "PROFILE['directory'](ROOT, path.relative_to(ROOT).as_posix())", 'None', 'D156 static library directory plan omitted'),
            ('per-build source plan', 'sources=SOURCES)', 'sources=())', 'D156 static build source plan changed'),
        ]
        faults.extend(('initial file omitted %d' % index, 'output_files=OUTPUT_FILES',
                       'output_files=OUTPUT_FILES[:%d] + OUTPUT_FILES[%d:]' % (index, index + 1),
                       'D156 static file plan changed') for index in range(count))
        faults.extend(('initial source omitted %d' % index, 'sources=SOURCES,',
                       'sources=SOURCES[:%d] + SOURCES[%d:],' % (index, index + 1),
                       'D156 static input plan changed') for index in range(20))
        faults.extend(('per-build file omitted %d' % index,
                       'output_files=tuple(output_dir / name for name in BUILD_FILENAMES)',
                       'output_files=tuple(output_dir / name for name in BUILD_FILENAMES[:%d] + BUILD_FILENAMES[%d:])' % (index, index + 1),
                       'D156 static build file plan changed') for index in range(7))
        if mode:
            faults.extend(('mutable source omitted %d' % index, 'mutable_sources=MUTABLE_SOURCES',
                           'mutable_sources=MUTABLE_SOURCES[:%d] + MUTABLE_SOURCES[%d:]' % (index, index + 1),
                           'D156 static mutable plan changed') for index in range(4))
        else:
            faults.append(('ordinary mutable authority', 'mutable_sources=MUTABLE_SOURCES',
                           'mutable_sources=MUTATED', 'D156 static mutable plan changed'))
        for name, before, after, message in faults:
            assert text.count(before) == 1, (name, 'actual static plan anchor')
            try:
                capture_static_entry(text.replace(before, after, 1), static, mutations=mode)
            except AssertionError as error:
                assert str(error) == message, (name, 'unrelated static plan refusal', str(error))
                red += 1
            else:
                raise AssertionError((name, 'actual static plan fault survived'))
    for name, before, after, message in (
        ('borrowed complete build plan', actual_build_plan, 'None', 'D156 static build plan omitted'),
        ('borrowed build sources', 'sources=SOURCES)', 'sources=())', 'D156 static build source plan changed'),
        ('borrowed build files', 'output_files=tuple(output_dir / name for name in BUILD_FILENAMES)',
         'output_files=()', 'D156 static build file plan changed'),
        ('borrowed library inputs', 'sources=(library, *sorted(library_files - {library}))',
         'sources=(library,)', 'D156 static library inputs changed'),
    ):
        assert text.count(before) == 1, (name, 'actual borrowed plan anchor')
        try:
            capture_static_entry(text.replace(before, after, 1), static, borrowed_label='first-phase')
        except AssertionError as error:
            assert str(error) == message, (name, 'unrelated borrowed plan refusal', str(error))
            red += 1
        else:
            raise AssertionError((name, 'actual borrowed plan fault survived'))
    driver_count = 0
    driver_classifier_controls(text, static)
    driver_count += 1
    functions = {n.name: ast.get_source_segment(text, n) for n in ast.parse(text).body if isinstance(n, ast.FunctionDef)}
    old = "def driver_assertion_failure(output):\n    log = output.decode('utf-8', errors='replace')\n    return 'panicked at' in log and re.search(r'(?m)^assertion(?:[ :`]|$)', log) is not None"
    for function, changed, message in (
        ('driver_assertion_failure', old, 'D172 driver classifier accepted noise'),
        ('driver_assertion_sites', 'def driver_assertion_sites(source, path):\n    return set()', 'D172 driver assertion sites changed'),
    ):
        actual = functions[function]
        assert text.count(actual) == 1
        try:
            driver_classifier_controls(text.replace(actual, changed, 1), static)
        except AssertionError as error:
            assert str(error) == message, (function, 'unrelated driver classifier refusal', str(error))
            red += 1
        else:
            raise AssertionError((function, 'actual driver classifier fault survived'))
    compiler_count = 0
    for owner in ('recipe', 'statement'):
        path = ROOT / f'docs/tasks/artifacts/formula_structure/{owner}_owner_contract.py'
        text = path.read_text()
        capture_compiler_entry(text, path, owner)
        compiler_count += 1
        faults = [
            ('complete inputs', 'sources=SOURCES', 'sources=()', 'D156 compiler planned inputs changed'),
            ('complete output', 'directories=(WORK,)', 'directories=()', 'D156 compiler planned output changed'),
            ('prepared stores', 'environment = dict(os.environ)',
             "environment = dict(os.environ, CARGO_HOME=str(ROOT / 'target/cargo-home'))", 'D156 compiler child stores lost'),
            ('Cargo public metadata', ", '--message-format=json'", '', 'D156 compiler child argv changed'),
            ('Rustc public diagnostics', ", '--error-format=json'", '', 'D156 compiler child argv changed'),
            ('dependency search', "('-L', 'dependency='", "('-I', 'dependency='", 'D156 compiler child argv changed'),
            ('actual documentation fixture', "str(source), '-o'", "str(SOURCE), '-o'", 'D156 compiler child argv changed'),
            ('fixture bytes', "+ '\\nfn main() {}\\n'", "+ '\\nfn main() {panic!();}\\n'", 'D156 compiler fixture/output bytes changed'),
        ]
        measure_physical += compiler_file_refusals(text, path, owner)
        faults.extend([
            ('complete files', 'output_files=OUTPUT_FILES', 'output_files=()', 'D156 compiler file plan changed'),
            ('guard binary files', "for suffix in ('.rs', '.log', '')", "for suffix in ('.rs', '.log')", 'D156 compiler file plan changed'),
            ('library plan', "PROFILE['plan_producer'](ROOT, sources=(artifact, *sorted(library_files - {artifact})))", 'None', 'D156 compiler library plan omitted'),
            ('library inputs', 'sources=(artifact, *sorted(library_files - {artifact}))', 'sources=(artifact,)', 'D156 compiler library inputs changed'),
            ('library directory plan', "PROFILE['directory'](ROOT, path.relative_to(ROOT).as_posix())", 'None', 'D156 compiler library directory plan omitted'),
            ('read-only sources', 'output_files=OUTPUT_FILES, sources=SOURCES)', 'output_files=OUTPUT_FILES, sources=SOURCES, mutable_sources=(SOURCE,))', 'D156 compiler read-only inputs changed'),
        ])
        faults.extend(('planned file omitted %d' % index, 'output_files=OUTPUT_FILES',
                       'output_files=OUTPUT_FILES[:%d] + OUTPUT_FILES[%d:]' % (index, index + 1),
                       'D156 compiler file plan changed') for index in range(29 if owner == 'recipe' else 17))
        faults.extend(('planned source omitted %d' % index, 'sources=SOURCES',
                       'sources=SOURCES[:%d] + SOURCES[%d:]' % (index, index + 1),
                       'D156 compiler planned inputs changed') for index in range(2 if owner == 'recipe' else 1))
        if owner == 'recipe':
            faults.append(('chapter input plan', 'SOURCES = (SOURCE, BOOK)', 'SOURCES = (SOURCE,)', 'D156 compiler planned inputs changed'))
            faults.append(('book compilation', "'recipe_example_%d'", "'wrong_example_%d'", 'D156 compiler child argv changed'))
            execute = next(node for node in ast.walk(ast.parse(text)) if isinstance(node, ast.Call)
                           and ast.unparse(node.func) == 'subprocess.run'
                           and ast.unparse(node.args[0]) == '[str(binary)]')
            actual = ast.get_source_segment(text, execute)
            faults.append(('book execution', actual, "subprocess.CompletedProcess([], 0, stdout='', stderr='')",
                           'D156 compiler child argv changed'))
        for name, before, after, message in faults:
            assert text.count(before) >= 1, (name, 'actual compiler anchor')
            try:
                capture_compiler_entry(text.replace(before, after, 1), path, owner)
            except AssertionError as error:
                assert str(error) == message, (name, 'unrelated compiler-control failure', str(error))
                red += 1
            else:
                raise AssertionError((name, 'actual compiler body fault survived'))
    lexer = ROOT / 'docs/tasks/artifacts/formula_lex/formula_lex_mutations.py'
    semantic = ROOT / 'docs/tasks/artifacts/formula_structure/semantic_mutations.py'
    lexer_sources = (ROOT / 'crates/sc-core/src/recipe/lexer.rs', ROOT / 'crates/sc-core/src/name.rs')
    semantic_sources = (ROOT / 'crates/sc-core/src/recipe/semantic.rs',)
    child_count = 0
    capture_child(lexer.read_text(), lexer, 'formula_lex_mutations', lexer_sources, 'formula_lex_contract')
    child_count += 1
    capture_child(semantic.read_text(), semantic, 'semantic_mutations', semantic_sources, 'formula_semantic_contract')
    child_count += 1
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
            ('statement', ('statement.rs',), 'formula_statement_contract')):
        path = ROOT / f'docs/tasks/artifacts/formula_structure/{name}_mutations.py'
        sources = tuple(ROOT / 'crates/sc-core/src/recipe' / file for file in files)
        text = path.read_text()
        capture_child(text, path, name + '_mutations', sources, target, first_suffix=())
        child_count += 1
        assert text.count(before) == 1
        try:
            capture_child(text.replace(before, "environment = dict(os.environ, CARGO_HOME=str(ROOT / 'target/cargo-home'))", 1),
                          path, name + '_mutations', sources, target)
        except AssertionError as error:
            assert str(error) == 'D156 Python child stores lost'
            red += 1
        else:
            raise AssertionError('D156 Python actual child-store reset survived')
    classifier_count = driver_count + measure_classifiers
    for filename, work_name, rust, target, test in (
            ('recipe_input_review_mutations.py', 'recipe_input_review_mutations',
             ('statement.rs', 'ordered.rs', 'canonical_recipe.rs', 'normalized_recipe.rs'),
             'formula_recipe_input_review',
             ('actual_worked_population_retains_all_headers_global_locations_and_complete_identity',)),
            ('formula_expression_mutations.py', 'formula_expression_mutations', 'parser.rs',
             'formula_expression_contract', 'semantic_node_bounds_count_calls_and_square_payload_correctly'),
            ('reduction_boundary_mutations.py', 'formula_reduction_mutations', 'literal.rs',
             'formula_literal_contract', 'coupled_reduction_frontier_preserves_valid_inputs_and_located_width_refusals'),
            ('wanted_signature_mutations.py', 'wanted_signature_mutations', 'signatures.rs',
             ('formula_operator_signature_contract', 'formula_builtin_signature_contract'), None),
            ('call_lookup_mutations.py', 'call_lookup_mutations', 'calls.rs', 'formula_call_lookup_contract', None),
            ('builtin_signature_mutations.py', 'builtin_signature_mutations', 'builtins.rs', 'formula_builtin_signature_contract', None),
            ('operator_signature_mutations.py', 'operator_signature_mutations', 'operators.rs', 'formula_operator_signature_contract', None),
            ('normalized_recipe_mutations.py', 'normalized_recipe_mutations', ('statement.rs', 'ordered.rs', 'normalized_recipe.rs'), 'formula_normalized_recipe_contract', None),
            ('canonical_recipe_mutations.py', 'canonical_recipe_mutations', 'canonical_recipe.rs', 'formula_canonical_recipe_contract', None),
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
             'formula_ordered_names_contract', None),
            ('declaration_mutations.py', 'declaration_mutations', 'declaration.rs',
             'formula_declaration_contract', None),
            ('name_read_mutations.py', 'name_read_mutations', 'namespace.rs',
             'formula_name_read_contract', None)):
        path = ROOT / 'docs/tasks/artifacts/formula_structure' / filename
        rust_files = rust if isinstance(rust, tuple) else (rust,)
        sources = tuple(ROOT / 'crates/sc-core/src/recipe' / file for file in rust_files)
        text = path.read_text()
        selected = (test, '--', '--exact') if isinstance(test, str) else test or ()
        doc_suffix = 'recipe::declaration::FormulaDeclaration' if filename == 'declaration_mutations.py' else None
        capture_child(text, path, work_name, sources, target, first_suffix=selected, doc_suffix=doc_suffix)
        child_count += 1
        if filename == 'recipe_input_review_mutations.py':
            node = next(n for n in ast.walk(ast.parse(text)) if isinstance(n, ast.Assign)
                        and any(isinstance(t, ast.Name) and t.id == 'result' for t in n.targets))
            actual_call = ast.get_source_segment(text, node.value)
            assert actual_call.count("'formula_recipe_input_review', test]") == 1
            omitted = actual_call.replace("'formula_recipe_input_review', test]", "'formula_recipe_input_review', '']", 1)
            try:
                capture_child(text.replace(actual_call, omitted, 1), path, work_name, sources,
                              target, first_suffix=selected)
            except AssertionError as error:
                assert str(error) == 'D156 Python child test selection changed'
                red += 1
            else:
                raise AssertionError('D171 actual coupled recipe-input selection fault survived')
        if filename in ('formula_expression_mutations.py', 'reduction_boundary_mutations.py'):
            node = next(n for n in ast.walk(ast.parse(text)) if isinstance(n, ast.Assign)
                        and any(isinstance(t, ast.Name) and t.id == 'result' for t in n.targets))
            actual_call = ast.get_source_segment(text, node.value)
            assert actual_call.count("test, '--', '--exact'") == 1
            for changed in ("'', '--', '--exact'", "test, '--'"):
                altered = actual_call.replace("test, '--', '--exact'", changed, 1)
                try:
                    capture_child(text.replace(actual_call, altered, 1), path, work_name, sources,
                                  target, first_suffix=selected)
                except AssertionError as error:
                    assert str(error) == 'D156 Python child test selection changed'
                    red += 1
                else:
                    raise AssertionError('D170 actual selected-test/exact-flag fault survived')
        if filename == 'wanted_signature_mutations.py':
            for variable in ('result', 'restored'):
                node = next(n for n in ast.walk(ast.parse(text)) if isinstance(n, ast.Assign)
                            and any(isinstance(t, ast.Name) and t.id == variable for t in n.targets))
                actual_call = ast.get_source_segment(text, node.value)
                assert actual_call.count("'--test', 'formula_builtin_signature_contract'") == 1
                omitted_call = actual_call.replace(", '--test', 'formula_builtin_signature_contract'", '', 1)
                try:
                    capture_child(text.replace(actual_call, omitted_call, 1), path, work_name, sources,
                                  target, first_suffix=selected)
                except AssertionError as error:
                    assert str(error) == 'D156 Python child native argv changed'
                    red += 1
                else:
                    raise AssertionError('D169 actual joint-target omission survived')
        if doc_suffix is not None:
            capture_child(text, path, work_name, sources, target, doc_suffix=doc_suffix, capture_doc_fault=True)
            child_count += 1
            wrong_doc = text.replace("result = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--doc',",
                                     "result = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test',", 1)
            assert wrong_doc != text
            try:
                capture_child(wrong_doc, path, work_name, sources, target,
                              doc_suffix=doc_suffix, capture_doc_fault=True)
            except AssertionError as error:
                assert str(error) == 'D156 Python child native argv changed'
                red += 1
            else:
                raise AssertionError('D156 Python negative-construction dispatch fault survived')
            filtered_doc = text.replace("'recipe::declaration::FormulaDeclaration'],",
                                        "'recipe::declaration::FormulaDeclaration', '--ignored'],", 1)
            assert filtered_doc != text
            try:
                capture_child(filtered_doc, path, work_name, sources, target,
                              doc_suffix=doc_suffix, capture_doc_fault=True)
            except AssertionError as error:
                assert str(error) == 'D156 Python child profile selection changed'
                red += 1
            else:
                raise AssertionError('D156 Python doc selection fault survived')
            restore = next(node for node in ast.walk(ast.parse(text))
                           if isinstance(node, ast.Assign) and any(isinstance(t, ast.Name) and t.id == 'restored_docs' for t in node.targets))
            actual_restore = ast.get_source_segment(text, restore)
            omitted = text.replace(actual_restore,
                                   "restored_docs = subprocess.CompletedProcess([], 0, stdout=b'', stderr=b'')", 1)
            try:
                capture_child(omitted, path, work_name, sources, target, doc_suffix=doc_suffix)
            except AssertionError as error:
                assert str(error) == 'D156 Python child capture/restoration absent'
                red += 1
            else:
                raise AssertionError('D156 Python doc restoration omission survived')
        classifier_controls(text, path, strict=True, thread_only=path.name in {
                                    'literal_normalization_mutations.py', 'normalized_expression_mutations.py',
                                    'normalized_recipe_mutations.py', 'canonical_recipe_mutations.py', 'canonical_expression_mutations.py', 'checked_expression_mutations.py', 'checked_statement_mutations.py', 'checked_recipe_mutations.py', 'namespace_mutations.py', 'ordered_name_mutations.py', 'declaration_mutations.py', 'name_read_mutations.py', 'builtin_signature_mutations.py', 'operator_signature_mutations.py', 'wanted_signature_mutations.py', 'call_lookup_mutations.py', 'formula_expression_mutations.py', 'reduction_boundary_mutations.py', 'recipe_input_review_mutations.py'})
        classifier_count += 1
        tree = ast.parse(text)
        function = next(node for node in tree.body if isinstance(node, ast.FunctionDef) and node.name == 'assertion_failure')
        actual = ast.get_source_segment(text, function)
        broad = "def assertion_failure(output):\n    return b'assertion' in output"
        assert text.count(actual) == 1
        try:
            classifier_controls(text.replace(actual, broad, 1), path, strict=True,
                                thread_only=path.name in {
                                    'literal_normalization_mutations.py', 'normalized_expression_mutations.py',
                                    'normalized_recipe_mutations.py', 'canonical_recipe_mutations.py', 'canonical_expression_mutations.py', 'checked_expression_mutations.py', 'checked_statement_mutations.py', 'checked_recipe_mutations.py', 'namespace_mutations.py', 'ordered_name_mutations.py', 'declaration_mutations.py', 'name_read_mutations.py', 'builtin_signature_mutations.py', 'operator_signature_mutations.py', 'wanted_signature_mutations.py', 'call_lookup_mutations.py', 'formula_expression_mutations.py', 'reduction_boundary_mutations.py', 'recipe_input_review_mutations.py'})
        except AssertionError as error:
            assert str(error) == 'D159 assertion evidence accepted noise'
            red += 1
        else:
            raise AssertionError('D159 actual broad classifier survived')
        permissive = "def assertion_failure(output):\n    parts = output.split(b'\\nfailures:\\n', 2)\n    return len(parts) == 3 and re.search(rb'(?m)^assertion(?:[ :`]|$)', parts[1]) is not None"
        try:
            classifier_controls(text.replace(actual, permissive, 1), path, strict=True,
                                thread_only=path.name in {
                                    'literal_normalization_mutations.py', 'normalized_expression_mutations.py',
                                    'normalized_recipe_mutations.py', 'canonical_recipe_mutations.py', 'canonical_expression_mutations.py', 'checked_expression_mutations.py', 'checked_statement_mutations.py', 'checked_recipe_mutations.py', 'namespace_mutations.py', 'ordered_name_mutations.py', 'declaration_mutations.py', 'name_read_mutations.py', 'builtin_signature_mutations.py', 'operator_signature_mutations.py', 'wanted_signature_mutations.py', 'call_lookup_mutations.py', 'formula_expression_mutations.py', 'reduction_boundary_mutations.py', 'recipe_input_review_mutations.py'})
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
            classifier_controls(text.replace(helper_text, omitted, 1), path, strict=True,
                                thread_only=path.name in {
                                    'literal_normalization_mutations.py', 'normalized_expression_mutations.py',
                                    'normalized_recipe_mutations.py', 'canonical_recipe_mutations.py', 'canonical_expression_mutations.py', 'checked_expression_mutations.py', 'checked_statement_mutations.py', 'checked_recipe_mutations.py', 'namespace_mutations.py', 'ordered_name_mutations.py', 'declaration_mutations.py', 'name_read_mutations.py', 'builtin_signature_mutations.py', 'operator_signature_mutations.py', 'wanted_signature_mutations.py', 'call_lookup_mutations.py', 'formula_expression_mutations.py', 'reduction_boundary_mutations.py', 'recipe_input_review_mutations.py'})
        except AssertionError as error:
            assert str(error) == 'D160 assertion sites changed'
            red += 1
        else:
            raise AssertionError('D160 actual assertion-site omission survived')
        assert text.count(before) == 1
        try:
            capture_child(text.replace(before, "environment = dict(os.environ, CARGO_HOME=str(ROOT / 'target/cargo-home'))", 1),
                          path, work_name, sources, target, doc_suffix=doc_suffix)
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
    child_count += 1
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
    child_count += 1
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
          ' actual body reds / ' + str(output_count) + ' actual output-file controls / ' +
          str(mutable_count) + ' physical mutable-path controls / ' +
          str(len(adopters)) + ' actual standalone pre-write captures / ' +
          str(len(adopters)) + ' actual late-source refusals / ' + str(child_count) + ' actual native-child capture cases / ' +
          str(compiler_count) + ' complete compiler/example captures / ' +
          str(static_count) + ' complete static-entry captures / ' +
          str(classifier_count) + ' calibrated failed-body classifiers / source unchanged')
    print('guarded native entries: ' + str(len(measure_paths) + 1 + recipe_captures + statement_captures + literal_captures + normalized_captures + recipe_input_captures + identity_captures + canonical_mode_captures + checked_mode_captures + checked_stmt_mode_captures + checked_recipe_mode_captures + namespace_mode_captures + ordered_mode_captures + declaration_mode_captures + name_read_mode_captures + builtin_mode_captures + operator_mode_captures + wanted_mode_captures + call_mode_captures + expression_mode_captures + reduction_mode_captures + input_mode_captures) + ' complete child/file captures / ' + str(measure_physical) +
          ' actual copied-entry physical refusals / five entry and three classifier body reds per entry; '
          'dispatch simulation separate from real native proof')


if __name__ == '__main__':
    main()
