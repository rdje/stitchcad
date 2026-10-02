"""D79: actual reference literal input agrees with explicit rows and Decimal quantum oracle."""
from decimal import Decimal, localcontext, ROUND_HALF_UP
from fractions import Fraction
from pathlib import Path
import runpy
ROOT = Path(__file__).resolve().parents[4]
namespace, reference = runpy.run_path(str(ROOT / 'docs/tasks/artifacts/formula_structure/formula_input.py'))['load_context']()
# Authored independently from the evaluator and its chapter-table reader. Decimal rounding is
# implemented by Python's standard library, not the reference's quotient/remainder routine.
FACTORS = {'um': ('length', 1), 'mm': ('length', 1000), 'cm': ('length', 10000),
           'm': ('length', 1000000), 'in': ('length', 25400),
           'deg': ('angle', 1000000), 'pct': ('ratio', 10000)}

def oracle(source):
    parts = source.split(' ')
    if len(parts) == 2:
        kind, factor = FACTORS[parts[1]]
    else:
        kind, factor = ('ratio', 1000000) if '.' in source else ('count', 1)
    with localcontext() as context:
        context.prec = len(parts[0]) + 20
        value = Decimal(parts[0]) * factor
        return kind, int(value.to_integral_value(rounding=ROUND_HALF_UP))

def respelling(kind, value):
    if kind == 'count':
        return str(value)
    if kind == 'length':
        return str(value) + ' um'
    whole, fraction = divmod(value, 1000000)
    decimal = '%d.%06d' % (whole, fraction)
    return decimal + (' deg' if kind == 'angle' else '')

rows = 0
checks = 0
for row in (ROOT / 'docs/tasks/artifacts/formula_structure/literal_cases.tsv').read_text().splitlines():
    if not row or row.startswith('#'):
        continue
    source, kind, integer = row.split('\t')
    expected = int(integer)
    assert oracle(source) == (kind, expected), ('explicit row/Decimal disagreement', source)
    tree = reference.parse(source)
    assert tree == ('lit', kind, Fraction(expected)), ('literal canonical integer', source, tree)
    assert tree[2].denominator == 1, ('fraction retained in canonical literal', source, tree)
    canonical_source = respelling(kind, expected)
    assert reference.parse(canonical_source) == tree, ('identity respelling', source, canonical_source)
    value = reference.evaluate(tree, {})
    assert (value.kind, value.v) == (kind, Fraction(expected)), ('evaluated literal', source, value)
    assert reference.evaluate(reference.parse(source + ' + ' + source), {}).v == expected * 2, ('sum of input quanta', source)
    checks += 5
    if kind != 'count':
        negative = reference.parse('-' + source)
        reference.infer(negative, {})
        assert reference.evaluate(negative, {}).v == -expected, ('sign symmetry', source)
        checks += 1
    rows += 1
assert rows == 60, ('fixture population changed', rows)
for source in ['0.00004 cm + 0.00004 cm', '0.0000004 + 0.0000004']:
    assert reference.evaluate(reference.parse(source), {}).v == 0, ('D79 regression', source)
    checks += 1
# Literal canonical identity preserves kind; decimal 1.0 never becomes count 1.
assert reference.parse('1') != reference.parse('1.0')
checks += 1
# Context precision must not truncate a lexical decimal before its exact conversion.
long_source = '0.' + '0' * 100 + '5 cm'
assert reference.parse(long_source) == ('lit', 'length', Fraction(0))
checks += 1
print('reference literal contracts: %d rows / %d controls / 0 fail; independent Decimal oracle' % (rows, checks))
