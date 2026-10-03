"""Whole ordered reference preflight and actual book-consumer ordering, without execution."""
from contextlib import redirect_stdout
from pathlib import Path
import io
import os
import runpy
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
HERE = ROOT / 'docs/tasks/artifacts/formula_structure'
HELPER = runpy.run_path(str(HERE / 'static_namespace_contract.py'))
LOADER = HELPER['LOADER']
SOURCE = HELPER['SOURCE']
BOOK = ROOT / 'docs/book/src'
WORK = ROOT / 'target/formula_structure/static_recipe'
Declaration = HELPER['Declaration']


def contracts(replacement=None, verbose=True):
    ns, reference = LOADER['load_reference'](replacement)
    reference.reserved = {name: HELPER['ReservedKind'](kind)
                          for name, kind in HELPER['RESERVED'].items()}
    for method in ('evaluate', '_evaluate', 'value_of_name', 'resolve_geometry', 'call', 'stored'):
        def trapped(*args, **kwargs):
            raise AssertionError('recipe preflight invoked execution')
        setattr(reference, method, trapped)
    checks = 0
    inputs = [('width', Declaration('length', 'measurement')),
              ('flag', Declaration('boolean', 'parameter')),
              ('edge_value', Declaration('edge', 'geometry'))]

    def accepted(source, chunks, headers, declarations=inputs):
        nonlocal checks
        before = list(declarations)
        try:
            plan = reference.preflight(source, declarations)
        except ns['FErr'] as error:
            raise AssertionError(('recipe positive refused', source[:90], error.token, error.msg)) from error
        assert isinstance(plan, tuple) and len(plan) == len(chunks), ('recipe complete plan', len(plan), len(chunks))
        cursor = 0
        for record, chunk, header in zip(plan, chunks, headers):
            start, end, checked = record
            # Authored slices, not the producer's boundary algorithm, determine every source byte.
            expected_start = source.index(chunk, cursor)
            assert not source[cursor:expected_start].strip(), ('recipe uncovered source', source)
            assert start == expected_start and source[start:end].strip() == chunk, ('recipe original slice', record, chunk)
            assert checked[:3] == header, ('recipe static header', checked, header)
            cursor = end
        assert not source[cursor:].strip() and list(declarations) == before, 'recipe changed caller declarations'
        checks += 1
        return plan

    def refused(source, token, detail=None, declarations=inputs):
        nonlocal checks
        before = list(declarations)
        result = sentinel = object()
        try:
            result = reference.preflight(source, declarations)
        except ns['FErr'] as error:
            assert error.token == token, ('recipe refusal token', error.token, token, error.msg)
            assert detail is None or detail in error.msg, ('recipe refusal detail', error.msg, detail)
        else:
            raise AssertionError(('recipe accepted refusal', source[:90], token))
        assert result is sentinel and list(declarations) == before, 'recipe published partial plan or changed caller'
        checks += 1

    for source in ('', ' \t\r\n\f\v'):
        accepted(source, [], [])
    let = 'let saved: length = width'
    assertion = 'assert closure: eps_fmt = saved == width'
    # Newlines do not terminate an expression; original literal gaps survive slicing.
    for separator in (' ', '\n', '\r\n\t', '\f\v'):
        chunks = [let, assertion, 'let after: length = saved + 1 cm']
        accepted(' \t' + separator.join(chunks) + ' \n', chunks,
                 [('let', 'saved', 'length'), ('assert', 'closure', 'eps_fmt'), ('let', 'after', 'length')])
    for source, header in [
        ('let saved: length = width\n+ 1 cm', ('let', 'saved', 'length')),
        ('let saved: length = if(flag, width, width)', ('let', 'saved', 'length')),
        ('let saved: ratio = param_at(edge_value, width)', ('let', 'saved', 'ratio')),
        ('let saved: length = width / 0', ('let', 'saved', 'length')),
        ('assert saved: eps_phys = width == width', ('assert', 'saved', 'eps_phys')),
    ]:
        accepted(source, [source], [header])
    # Every initial origin remains metadata-only; none grants a value/state default.
    for origin in HELPER['ORIGINS']:
        for kind in HELPER['KINDS'][:5]:
            source = 'let saved: %s = input_value' % kind
            accepted(source, [source], [('let', 'saved', kind)], [('input_value', Declaration(kind, origin))])
        source = 'let saved: boolean = input_value'
        accepted(source, [source], [('let', 'saved', 'boolean')], [('input_value', Declaration('boolean', origin))])
    for first in HELPER['ORIGINS']:
        for second in HELPER['ORIGINS']:
            duplicate = [('collision', Declaration('length', first)), ('collision', Declaration('length', second))]
            refused('', 'formula_ambiguous_name', 'both %s and %s' % (first, second), duplicate)
    for name in HELPER['RESERVED']:
        refused('', 'formula_rebinding', 'reserved', [(name, Declaration('length', 'parameter'))])
    for bad, token in [
        ('1', 'formula_parse'), ('width + 1 cm', 'formula_parse'), ('let', 'formula_parse'),
        ('let saved: length = (width let hidden: count = 1)', 'formula_parse'),
        ('let saved: length = width)', 'formula_parse'),
        ('let saved: length = width;', 'formula_parse'),
        ('let saved: length = 1cm', 'formula_parse'),
        ('let saved: length = 1\ncm', 'formula_parse'),
        ('let saved: length = 1  cm', 'formula_parse'),
        ('let saved: length = 1 mmlet hidden: count = 1', 'formula_parse'),
        ('let saved: Length = width', 'formula_parse'),
        ('let saved: point = point_at(edge_value, 0.5)', 'formula_dimension'),
        ('let saved: length = saved', 'formula_unbound_name'),
        ('let saved: length = later let later: length = width', 'formula_unbound_name'),
        ('let saved: length = if(flag, width, missing)', 'formula_unbound_name'),
        ('let saved: length = if(flag, missing, width)', 'formula_unbound_name'),
        ('assert closure: eps_num = width == missing', 'formula_unbound_name'),
        ('assert closure: eps_num = missing == width', 'formula_unbound_name'),
        ('assert closure: eps_num = flag == flag', 'formula_dimension'),
        ('assert closure: size_index = width == width', 'formula_parse'),
        ('assert closure: eps_num = width == width let saved: length = closure', 'formula_unbound_name'),
        ('let width: length = 1 cm', 'formula_ambiguous_name'),
        ('let saved: length = width let saved: length = width', 'formula_rebinding'),
        ('let eps_num: length = width', 'formula_rebinding'),
        ('let saved: length = width let Upper: count = 1', 'formula_parse'),
        ('let saved: length = width assert closure: eps_num = width == é', 'formula_parse'),
    ]:
        refused(bad, token)
    # A correct first statement must not publish a result when the LAST one is bad.
    for prefix in ('let first: length = width / 0',
                   'let first: length = width let second: length = first'):
        for tail, token in [('let last: count = width', 'formula_dimension'),
                            ('assert last: eps_geo = missing == width', 'formula_unbound_name'),
                            ('let last: length = if(flag, width, missing)', 'formula_unbound_name')]:
            refused(prefix + '\n' + tail, token)
    # Limits are the actual chapter's production contract, never reduced fixture ceilings.
    assert reference.limits == {'max_expression_nodes': 256, 'max_recipe_statements': 4096,
                                'max_if_depth': 16, 'max_rational_bits': 128}
    for count in (4095, 4096):
        chunks = ['let v%d: count = %s' % (i, '1' if i == 0 else 'v%d' % (i - 1)) for i in range(count)]
        accepted('\n'.join(chunks), chunks, [('let', 'v%d' % i, 'count') for i in range(count)])
    many = '\n'.join('assert c%d: eps_num = width == width' % i for i in range(4096))
    refused(many + '\nlet excess: count = 1', 'formula_domain', 'statement 4097')
    refused(many + '\nassert excess: eps_num = width == width', 'formula_domain', 'statement 4097')
    for nodes in (255, 256):
        expr = '-' * (nodes - 1) + '1 um'
        source = 'let saved: length = ' + expr
        accepted(source, [source], [('let', 'saved', 'length')])
    refused('let first: length = width let last: length = ' + '-' * 256 + '1 um',
            'formula_domain', '257 nodes')
    for depth in (15, 16, 17):
        expr = 'width'
        for _ in range(depth):
            expr = 'if(flag, width, ' + expr + ')'
        source = 'let saved: length = ' + expr
        if depth <= 16:
            accepted(source, [source], [('let', 'saved', 'length')])
        else:
            refused('let first: length = width / 0\n' + source, 'formula_domain', 'conditional depth is 17')
    if verbose:
        print('static recipe contract: %d cases; ordered whole source /4096 boundary; value and execution trapped' % checks)
    return checks


def measurement_contract(replacement=None):
    """An independent per-recipe ceiling lies between21 worked statements and34 total cases."""
    book = WORK / 'per_recipe_measurement' / 'src'
    if book.exists():
        shutil.rmtree(book)
    shutil.copytree(BOOK, book)
    contract = book / 'spec/formula-language.md'
    text = contract.read_text()
    before = '| `max_recipe_statements` | 4096 |'
    assert text.count(before) == 1
    contract.write_text(text.replace(before, '| `max_recipe_statements` | 25 |'))
    program = SOURCE.read_text().split("<<'PY'\n", 1)[1].rsplit('\nPY', 1)[0]
    if replacement:
        before, after = replacement
        assert program.count(before) == 1, 'measurement fault anchor changed'
        program = program.replace(before, after)
    argv, out = sys.argv, io.StringIO()
    try:
        sys.argv = ['reference', *[str(book / p) for p in [
            'spec/formula-language.md', 'spec/formula-language/grammar.md',
            'spec/formula-language/examples.md', 'spec/reference-skirt.md', '.', 'spec/formula-language']]]
        with redirect_stdout(out):
            try:
                exec(compile(program, str(SOURCE), 'exec'), {})
            except SystemExit as error:
                (WORK / 'per_recipe_measurement.log').write_text(out.getvalue())
                assert error.code == 0, ('recipe measurement false refusal', error.code, out.getvalue())
            else:
                raise AssertionError('recipe measurement consumer did not terminate')
    finally:
        sys.argv = argv
    assert '21 statements' in out.getvalue(), ('recipe measurement total', out.getvalue())
    print('static recipe measurement: actual21-statement recipe passes25 ceiling; unrelated13 refusal cases excluded')


def consumer_contracts(replacement=None, verbose=True):
    WORK.mkdir(parents=True, exist_ok=True)
    program = SOURCE.read_text().split("<<'PY'\n", 1)[1].rsplit('\nPY', 1)[0]
    if replacement:
        before, after = replacement
        assert program.count(before) == 1, 'consumer fault anchor changed'
        program = program.replace(before, after)
    marker = '# Check the ENTIRE worked recipe before its first statement can execute.'
    assert program.count(marker) == 1
    program = program.replace(marker, '''def forbidden_execution(*args, **kwargs):
    raise AssertionError("recipe consumer invoked execution before late static refusal")
for method in ("statement", "evaluate", "_evaluate", "value_of_name", "resolve_geometry", "call", "stored"):
    setattr(EV, method, forbidden_execution)
''' + marker)
    checks = 0
    # Actual published last assertion is made ill-typed or unresolved. The first binding's
    # zero divisor would execute if the consumer skipped preflight or preflight returned early.
    for label, suffix, token in [('late_kind', '== size_index`', 'formula_dimension'),
                                 ('late_name', '== missing_width`', 'formula_unbound_name')]:
        book = WORK / label / 'src'
        if book.exists():
            shutil.rmtree(book)
        shutil.copytree(BOOK, book)
        examples = book / 'spec/formula-language/examples.md'
        text = examples.read_text()
        for before, after in [('`waist_girth + ease_waist`', '`waist_girth / 0`'),
                              ('== 2 * wb_width`', suffix)]:
            assert text.count(before) == 1, ('consumer copied-book anchor', before)
            text = text.replace(before, after)
        examples.write_text(text)
        argv = sys.argv
        out = io.StringIO()
        try:
            sys.argv = ['reference', *[str(book / p) for p in [
                'spec/formula-language.md', 'spec/formula-language/grammar.md',
                'spec/formula-language/examples.md', 'spec/reference-skirt.md', '.', 'spec/formula-language']]]
            with redirect_stdout(out):
                try:
                    exec(compile(program, str(SOURCE), 'exec'), {})
                except SystemExit as error:
                    assert error.code == 1, ('recipe consumer exit', error.code)
                else:
                    raise AssertionError('recipe consumer accepted late static error')
        finally:
            sys.argv = argv
        output = out.getvalue()
        (WORK / (label + '.log')).write_text(output)
        assert 'worked recipe preflight: ' + token in output, ('recipe consumer refusal', output)
        assert '-- L2 bindings' not in output and 'no statement executed' in output, ('recipe consumer order', output)
        checks += 1
    if verbose:
        # This arm runs the actual shell entry point, retaining the real numerical replay.
        result = subprocess.run(['bash', str(SOURCE)], cwd=ROOT, env=dict(os.environ, FORMULA_BOOK=str(BOOK)),
                                capture_output=True, text=True)
        output = result.stdout + result.stderr
        (WORK / 'real.log').write_text(output)
        assert result.returncode == 0 and 'statically accepted statements: 21' in output, output
        assert '17 bindings / 4 assertions / 13 refusals / 0 mismatch(es)' in output, output
        print('static recipe consumer: 3 controls; actual21-statement replay green / last-error-before-first-execution')
    return checks


FAULTS = [
    ('duplicate pairs overwritten', 'env = self.namespace(declarations)\n        toks, starts', 'env = dict(declarations)\n        toks, starts'),
    ('bare expression admitted', 'if toks[0][1] not in {"let", "assert"}:\n            raise FErr("formula_parse", "a recipe contains only let/assert statements")',
     'if toks[0][1] not in {"let", "assert"}:\n            return ()'),
    ('original offsets corrupted', 'boundaries.append(start)', 'boundaries.append(start + 1)'),
    ('last statement omitted', 'zip(boundaries, ends), 1', 'zip(boundaries[:-1], ends), 1'),
    ('partial plan returned', '            plan.append((start, end, checked))', '            plan.append((start, end, checked))\n            return tuple(plan)'),
    ('declaration order unpublished', 'env[checked[1]] = {"kind": checked[2], "origin": "recipe"}', 'pass'),
    ('statement ceiling bypassed', 'if ordinal > self.limits["max_recipe_statements"]:', 'if False:'),
    ('expression node ceiling bypassed', 'if n > self.limits["max_expression_nodes"]:', 'if False:'),
    ('conditional ceiling bypassed', 'if depth > self.limits["max_if_depth"]:', 'if False:'),
    ('numeric execution added', '            checked = self._static_statement(src[start:end], env, ordinal, start, prior_sources)',
     '            self.evaluate(self.parse("1"), env)\n            checked = self._static_statement(src[start:end], env, ordinal, start, prior_sources)'),
    ('input availability observed', '        boundaries, depth = [], 0', '        [entry.get("state") for entry in env.values()]\n        boundaries, depth = [], 0'),
]
if __name__ == '__main__':
    original = SOURCE.read_bytes()
    contracts()
    consumer_contracts()
    measurement_contract()
    if '--mutations' in sys.argv:
        for name, before, after in FAULTS:
            try:
                contracts((before, after), False)
            except AssertionError as error:
                assert any(marker in str(error) for marker in (
                    'recipe positive refused', 'recipe complete plan', 'recipe original slice',
                    'recipe accepted refusal', 'recipe preflight invoked execution',
                    'static namespace read value/state/geometry')), (name, 'not a body assertion red', error)
                print('  actual compiled recipe assertion red:', name)
            else:
                raise AssertionError(('actual recipe fault escaped', name))
        for name, before, after in [
            ('consumer preflight bypass', 'worked_plan = EV.preflight(worked_source, env.items())', 'worked_plan = ()'),
            ('consumer early evaluation', 'worked_plan = EV.preflight(worked_source, env.items())',
             'EV.evaluate(EV.parse("1"), env)\n    worked_plan = EV.preflight(worked_source, env.items())'),
        ]:
            try:
                consumer_contracts((before, after), False)
            except AssertionError as error:
                assert 'recipe consumer invoked execution' in str(error), ('not a consumer body assertion red', error)
                print('  actual compiled recipe assertion red:', name)
            else:
                raise AssertionError(('consumer preflight fault escaped', name))
        try:
            measurement_contract(('statements = len(worked_plan)',
                                  'statements = len(bind_rows) + len(asserts) + len(refusals)'))
        except AssertionError as error:
            assert 'recipe measurement false refusal' in str(error), ('not a measurement body assertion red', error)
            print('  actual compiled recipe assertion red: unrelated refusal cases counted as recipe')
        else:
            raise AssertionError('measurement fault escaped')
        assert SOURCE.read_bytes() == original
        print('static recipe faults: %d actual assertion reds; producer unchanged' % (len(FAULTS) + 3))
