"""D82: actual reference operators preserve independently expected rational results."""
from fractions import Fraction
from pathlib import Path
import runpy
ROOT = Path(__file__).resolve().parents[4]
namespace, reference = runpy.run_path(str(ROOT / 'docs/tasks/artifacts/formula_structure/formula_input.py'))['load_context']()
grammar = namespace['sections'](str(ROOT / 'docs/book/src/spec/formula-language/grammar.md'))
# Load only published dimension signatures. This is test context, not another evaluator/parser.
for left, right, product, quotient in namespace['table_in'](grammar['5.1'], 'Left')[1]:
    left, right = namespace['debacktick'](left), namespace['debacktick'](right)
    product, quotient = namespace['debacktick'](product), namespace['debacktick'](quotient)
    product = None if product == '—' else product
    quotient = None if quotient == '—' else quotient
    reference.pairs[left, right] = {'*': product, '/': quotient}
    if product:
        reference.pairs.setdefault((right, left), {})['*'] = product
reference.sigs = {'param_at': [(['edge', 'length'], False, 'ratio')],
                  'round_to': [(['T', 'T'], False, 'T')],
                  'sqrt': [(['area'], False, 'length'), (['ratio'], False, 'ratio')],
                  'hypot': [(['length', 'length'], False, 'length')]}
checks = 0

def value(source, kind, expected, env=None):
    global checks
    env = {} if env is None else env
    tree = reference.parse(source)
    assert reference.infer(tree, env) == kind, ('static dimension', source)
    error = None
    try:
        actual = reference.evaluate(tree, env)
    except namespace['FErr'] as refused:
        error = refused
    assert error is None, ('unexpected runtime refusal', source, error)
    assert (actual.kind, actual.v) == (kind, Fraction(expected)), ('exact internal result', source, actual, expected)
    checks += 1
    return actual.v

rows = 0
for line in (ROOT / 'docs/tasks/artifacts/formula_structure/arithmetic_cases.tsv').read_text().splitlines():
    if not line or line.startswith('#'):
        continue
    source, kind, exact, binding = line.split('\t')
    actual = value(source, kind, Fraction(exact))
    assert namespace['rnd'](actual) == int(binding), ('one binding round', source, actual)
    checks += 1
    rows += 1
assert rows == 24
# Independent parameterized Fraction oracle, authored as arithmetic values, not AST traversal.
# Every division has deliberately tiny nonintegral outcomes which premature rounding cannot hide.
parameter_cases = 0
for numerator in [1, 2, 5, 7, 11]:
    for denominator in [2, 3, 6, 8, 13]:
        exact = Fraction(numerator, denominator)
        value('%d um / %d + %d um / %d' % (numerator, denominator, numerator, denominator), 'length', exact + exact)
        value('%d um / %d * %d' % (numerator, denominator, denominator), 'length', numerator)
        value('(%d um / %d) ^ 2' % (numerator, denominator), 'area', exact ** 2)
        value('%d / %d' % (numerator, denominator), 'ratio', exact * 1000000)
        parameter_cases += 4
# Compare two routes through the declared scale, retaining internal rational precision.
for source in ['(1 um / 3) + (2 um / 3)', '(1 um + 2 um) / 3',
               '(1 um / 3) * 3', '1 um * (1 / 3 + 2 / 3)']:
    value(source, 'length', 1)
# Rational edge selector model only: no real curve inversion/geometric accuracy claim.
edge_env = {'edge_a': {'kind': 'edge', 'value': Fraction(3), 'origin': 'geometry'}}
value('param_at(edge_a, 1 um)', 'ratio', Fraction(1000000, 3), edge_env)
value('param_at(edge_a, 1 um) * 3', 'ratio', 1000000, edge_env)
# Explicit quantization and irrational calls still round where their signatures require it.
value('round_to(1 um / 2, 1 um)', 'length', 1)
value('round_to(-1 um / 2, 1 um)', 'length', -1)
value('sqrt((3 um / 2) ^ 2)', 'length', 2)
value('hypot(1 um, 1 um)', 'length', 1)
# Taken-only evaluation remains lazy even when the other branch divides by zero.
value('if(1 == 1, 1 um / 2, 1 um / 0)', 'length', Fraction(1, 2))
value('if(1 != 1, 1 um / 0, 1 um / 2)', 'length', Fraction(1, 2))
for source in ['1 um / 0', '1.0 / 0.0']:
    error = None
    try:
        reference.evaluate(reference.parse(source), {})
    except Exception as refused:
        error = refused
    assert isinstance(error, namespace['FErr']) and error.token == 'formula_division', ('typed zero division', source, error)
    checks += 1
print('reference arithmetic contracts: %d rows / %d parameter cases / %d controls / 0 fail; independent Fraction oracle' % (rows, parameter_cases, checks))
