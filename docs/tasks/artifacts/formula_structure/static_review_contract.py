"""Independent book-static outcomes and approved D124 recognition; not runtime signoff."""
from contextlib import redirect_stdout
from fractions import Fraction
from pathlib import Path
import io
import math
import re
import runpy
import sys

ROOT = Path(__file__).resolve().parents[4]
HERE = ROOT / 'docs/tasks/artifacts/formula_structure'
BASE = runpy.run_path(str(HERE / 'static_namespace_contract.py'))
SOURCE, BOOK = BASE['SOURCE'], ROOT / 'docs/book/src'
Declaration = BASE['Declaration']
WORKED = (
    ('garment_waist', 'length'), ('garment_hip', 'length'), ('quarter_waist', 'length'),
    ('quarter_hip', 'length'), ('suppression', 'length'), ('hip_to_hem_drop', 'length'),
    ('front_dart_centre', 'length'), ('side_seam_length', 'length'), ('quarter_hem_width', 'length'),
    ('garment_hem', 'length'), ('waistband_cut_width', 'length'), ('waistband_pattern_length', 'length'),
    ('dart_leg_angle', 'angle'), ('dart_intake_share', 'ratio'), ('zip_notch_param', 'ratio'),
    ('dart_apex_depth', 'length'), ('dart_count', 'count'))
ASSERTIONS = ('allocation_balance', 'waist_closure', 'waistband_length_closure', 'waistband_width_closure')
# Exact source/outcome expectations authored independently from the book table reader.
# A None static token means the candidate is statically valid, with a later runtime obligation.
REFUSALS = (
    ('waist_girth + 2.5', 'formula_dimension', None),
    ('dart_intake / 0', None, 'length'),
    ('sqrt(hip_to_hem_drop)', 'formula_dimension', None),
    ('dart_leg_angle * dart_len_front', 'formula_dimension', None),
    ('- dart_count', 'formula_dimension', None),
    ('let dart_intake_share: ratio = suppression / dart_intake', 'formula_rebinding', None),
    ('let a: length = b + 1 cm', 'formula_unbound_name', None),
    ('if(dart_intake > 4 cm, 5 cm)', 'formula_parse', None),
    ('"wide"', 'formula_parse', None),
    ('dart_intake ^ 3', 'formula_unsupported', None),
    ('garment_hem * (100 pct + shrinkage)', None, 'length'),
    ('eps_phys + 1 cm', None, 'length'),
    ('spline(cb_seam, 3)', 'env_nurbs', None))
ENVELOPE = {'nurbs': 'env_nurbs', 'spline': 'env_nurbs', 'bspline': 'env_nurbs',
            'solve': 'env_sketch_constraints', 'constraint': 'env_sketch_constraints', 'fixpoint': 'env_sketch_constraints'}
# Director ruling2026-10-03: existing grammar recognition, with no excluded-form grammar added.
OBSERVED = (
    ('loop(width)', 'formula_unbound_name'), ('repeat(2,width)', 'formula_unbound_name'),
    ('while(width > 0 um)', 'formula_unbound_name'), ('fn helper(width) = width', 'formula_parse'),
    ('macro helper(width) = width', 'formula_parse'), ('width ^ 3', 'formula_unsupported'),
    ('translate(width, 1 mm)', 'formula_unbound_name'))
ORDINARY_NAMES = ('loop', 'repeat', 'while', 'fn', 'macro')


def recognition_documentation(ns, replacement=None):
    contract = '\n'.join(ns['CON']['6'])
    grammar = '\n'.join(ns['GRA']['1.1'])
    if replacement:
        before, after = replacement
        assert (contract + grammar).count(before) == 1, 'recognition documentation fault anchor changed'
        contract, grammar = contract.replace(before, after), grammar.replace(before, after)
    rows = {row[0]: tuple(re.findall(r'formula_[a-z_]+', row[2]))
            for row in ns['table_in'](contract.splitlines(), 'Excluded')[1]}
    for capability in ('loops, iteration, recursion', 'user-defined functions and macros'):
        assert rows[capability] == ('formula_unbound_name', 'formula_parse'), (
            'static review exclusion documentation', capability, rows[capability])
    keywords = tuple(ns['debacktick'](row[0]) for row in ns['table_in'](grammar.splitlines(), 'Keyword')[1])
    assert keywords == ('let', 'assert', 'if'), ('static review keyword documentation', keywords)


def load(replacement=None):
    program = SOURCE.read_text().split("<<'PY'\n", 1)[1].rsplit('\nPY', 1)[0]
    boundary = '# ── L1:'
    assert program.count(boundary) == 1, 'actual fixture-loading boundary changed'
    prefix = program.split(boundary, 1)[0]
    if replacement:
        before, after = replacement
        assert prefix.count(before) == 1, 'actual static review fault anchor changed'
        prefix = prefix.replace(before, after)
    namespace, argv = {}, sys.argv
    try:
        sys.argv = ['reference', *[str(BOOK / p) for p in (
            'spec/formula-language.md', 'spec/formula-language/grammar.md',
            'spec/formula-language/examples.md', 'spec/reference-skirt.md', '.', 'spec/formula-language')]]
        with redirect_stdout(io.StringIO()):
            exec(compile(prefix, str(SOURCE), 'exec'), namespace)
    finally:
        sys.argv = argv
    assert namespace['fails'] == 0, 'actual table/fixture loading failed'
    reference = namespace['EV']
    env = {name: Declaration(entry['kind'], entry['origin']) for name, entry in namespace['env'].items()}
    reference.reserved = {name: BASE['ReservedKind'](kind) for name, kind in BASE['RESERVED'].items()}
    for method in ('statement', 'evaluate', '_evaluate', 'value_of_name', 'resolve_geometry', 'call', 'stored'):
        def trapped(*args, **kwargs):
            raise AssertionError('static review invoked execution')
        setattr(reference, method, trapped)
    return namespace, reference, env


def contracts(replacement=None, verbose=True):
    ns, reference, env = load(replacement)
    recognition_documentation(ns)
    table, span, plain = ns['table_in'], ns['first_span'], ns['debacktick']
    rows = table(ns['EXA']['2'], 'Token')[1]
    assert tuple((plain(row[0]), row[1]) for row in rows) == WORKED, 'worked population/order/kind drift'
    assertions = table(ns['EXA']['3'], 'Assertion')[1]
    chunks = ['let %s: %s = %s' % (plain(row[0]), row[1], span(row[2])) for row in rows]
    chunks += [span(row[0]) for row in assertions]
    plan = reference.preflight('\n'.join(chunks), env.items())
    assert tuple(record[2][:3] for record in plan) == (
        tuple(('let', name, kind) for name, kind in WORKED)
        + tuple(('assert', name, 'eps_num') for name in ASSERTIONS)), 'worked static headers drift'
    for _, _, checked in plan:
        if checked[0] == 'let':
            env[checked[1]] = Declaration(checked[2], 'recipe')
    rows = table(ns['EXA']['4'], 'Refusal')[1]
    assert tuple(span(row[0]) for row in rows) == tuple(row[0] for row in REFUSALS), 'refusal source/order drift'
    count = len(plan)

    def check(source, token, kind=None):
        nonlocal count
        try:
            result = reference.static_statement(source, env)
        except ns['FErr'] as error:
            assert error.token == token, ('static review token', source, token, error.token)
        else:
            assert token is None and result[2] == kind, ('static review accepted refusal', source, token, result[:3])
        count += 1

    for source, token, kind in REFUSALS:
        check(source, token, kind)
    assert reference.envelope == ENVELOPE, 'envelope population/token drift'
    # Envelope precedence over unknown operands and operand kind checking, in either branch.
    for construct, token in ENVELOPE.items():
        for expression in (construct + '(missing)', construct + '(width, 1)',
                           'if(is_base_size, 1 cm, ' + construct + '(missing))',
                           'if(is_base_size, ' + construct + '(missing), 1 cm)'):
            check(expression, token)
    env['width'] = Declaration('length', 'measurement')
    env['loop'] = Declaration('length', 'parameter')
    for source, token in OBSERVED:
        check(source, token)
    check('loop', None, 'length')
    for name in ORDINARY_NAMES:
        env[name] = Declaration('length', 'parameter')
        check(name, None, 'length')
        checked = reference.static_statement('let %s:length=1 cm' % name,
                                             {key: val for key, val in env.items() if key != name})
        assert checked[:3] == ('let', name, 'length'), ('static review name/header', name, checked[:3])
        count += 1
        # A scalar declaration does not authorize a callable; unknown call wins over its argument.
        for source in (name + '(missing)', 'if(is_base_size,1 cm,' + name + '(missing))',
                       'if(is_base_size,' + name + '(missing),1 cm)'):
            check(source, 'formula_unbound_name')
    for keyword in ('let', 'assert', 'if'):
        check('let %s:length=1 cm' % keyword, 'formula_parse')
    for source in ('fn helper(width)=missing', 'macro helper(width)=missing',
                   'loop(', 'repeat(2,width', 'while(width>0 um'):
        check(source, 'formula_parse')
    check('assert deliberately_false: eps_num = 1 cm == 2 cm', None, 'eps_num')
    if verbose:
        print('static review: %d actual cases;21 worked /13 refusal rows / envelope6; values and execution trapped' % count)
        print('D124 recognition: approved current grammar / five ordinary identifiers / three keywords / unknown-call and parse precedence; capability grammar unchanged')
    return count


FAULTS = (
    ('envelope dispatch bypass', 'if name in self.envelope:\n            raise FErr(self.envelope[name], "the envelope owns this construct, not the language")\n        if name not in self.sigs:', 'if name not in self.sigs:'),
    ('unknown calls accepted', 'raise FErr("formula_unbound_name", "`%s` is no declared function, selector or name" % name)', 'return "length"'),
    ('book statement omitted', 'return tuple(plan)', 'return tuple(plan[:-1])'),
    ('static values observed', 'if name in env: return env[name]["kind"]', 'if name in env: return env[name]["value"]'),
    ('ordinary name reserved', 'if not allow_keyword and text in {"let", "assert", "if"}:',
     'if not allow_keyword and text in {"let", "assert", "if", "macro"}:'),
    ('keyword reservation removed', 'if not allow_keyword and text in {"let", "assert", "if"}:', 'if False:'),
    ('recognized power token changed', 'raise FErr("formula_unsupported", "only the square',
     'raise FErr("formula_parse", "only the square'),
)


def parameter_quantum_contract():
    """D126: independent exact inequalities; this does not execute product geometry."""
    ns, reference = BASE['LOADER']['load_reference']()
    assert ns['RATIO_SCALE'] == 1000000 and reference.reserved['eps_geo'][2] == 10
    units = (BOOK / 'spec/units-and-tolerances.md').read_text()
    assert '| a piece bounding box | ≤ 10⁷ µm per side | 10 m |' in units
    assert '**Circular arc**' in units and '**10 µm** for internal geometry' in units
    radius, half_quantum = 5000000, Fraction(1, 2000000)
    # The270-degree arc includes all four extrema, fitting a10m square exactly.
    extrema = ((radius, 0), (0, radius), (-radius, 0), (0, -radius))
    assert all(max(point[i] for point in extrema) - min(point[i] for point in extrema) == 10000000
               for i in (0, 1))
    # Independent rational bounds3<pi<22/7 and sin(x)>x-x^3/6 for0<x<1.
    low = Fraction(270, 180) * 3 * half_quantum
    high = Fraction(270, 180) * Fraction(22, 7) * half_quantum
    chord_lower = radius * low - radius * high ** 3 / 24
    assert chord_lower > 10, 'parameter chord exceeds internal T2 even under exact lower bound'
    actual = reference.statement('let curve_length: length = arc_length(270 deg, 5 m)', {})[3].v
    assert actual == 23561945 and Fraction(270, 180) * 3 * radius < actual < Fraction(270, 180) * Fraction(22, 7) * radius
    chord = 2 * radius * math.sin(270 * math.pi / 180 * float(half_quantum) / 2)
    assert 11.78 < chord < 11.79 and actual * half_quantum > 10
    print('D126 parameter quantum: reference length23561945um; independent chord %.9fum > T2=10um; exact rational lower bound >10um' % chord)
    print('parameter quantum proof: bounding-box size is not an arc-length bound; product selector/geometry budgets remain .5f.3/G2')


if __name__ == '__main__':
    original = SOURCE.read_bytes()
    contracts()
    parameter_quantum_contract()
    if '--mutations' in sys.argv:
        for name, before, after in FAULTS:
            try:
                contracts((before, after), False)
            except AssertionError as error:
                assert any(marker in str(error) for marker in (
                    'static review token', 'static review accepted refusal', 'worked static headers drift',
                    'static review name/header', 'static namespace read value/state/geometry')), (name, 'not a body assertion red', error)
                print('  actual compiled static review assertion red:', name)
            else:
                raise AssertionError(('static review fault escaped', name))
        assert SOURCE.read_bytes() == original
        print('static review faults: %d actual assertion reds; producer unchanged' % len(FAULTS))
        namespace, _, _ = load()
        for before, after in (
            ('| loops, iteration, recursion | a recipe is a bounded ordered list; repetition is an operation list | unknown call: `formula_unbound_name`; malformed syntax: `formula_parse` |',
             '| loops, iteration, recursion | a recipe is a bounded ordered list; repetition is an operation list | `formula_unsupported` |'),
            ('| user-defined functions and macros | v1 has no function-definition or macro scope | unknown call: `formula_unbound_name`; malformed syntax: `formula_parse` |',
             '| user-defined functions and macros | v1 has no function-definition or macro scope | `formula_unsupported` |'),
            ('| `if` | the one special form', '| `macro` | the one special form'),
        ):
            try:
                recognition_documentation(namespace, (before, after))
            except AssertionError as error:
                assert 'documentation' in str(error) and 'fault anchor' not in str(error), ('not a documentation body red', error)
                print('  actual loaded documentation assertion red:', before.split('|')[1].strip())
            else:
                raise AssertionError('static recognition documentation fault escaped')
        print('recognition documentation: three actual loaded-row assertion reds; tracked documents unchanged')
