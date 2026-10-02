"""D85/D86/D87: explicit angular identities and independently authored rounded controls."""
from pathlib import Path
from fractions import Fraction
import runpy
ROOT = Path(__file__).resolve().parents[4]
# Reuse table-only reference context; this executes its restored arithmetic contracts as a control.
context = runpy.run_path(str(ROOT / 'docs/tasks/artifacts/formula_structure/arithmetic_contract.py'))
namespace, reference = context['namespace'], context['reference']
reference.sigs.update({name: [(['angle'], False, 'ratio')] for name in ['sin', 'cos', 'tan']})
reference.sigs.update({'arc_length': [(['angle', 'length'], False, 'length')],
                       'atan': [(['ratio'], False, 'angle')],
                       'atan2': [(['length', 'length'], False, 'angle')],
                       'dir': [(['point', 'point'], False, 'angle')]})
checks = 0
rows = 0

def value(source, kind, expected, env=None):
    global checks
    env = {} if env is None else env
    tree = reference.parse(source)
    assert reference.infer(tree, env) == kind, ('angular dimension', source)
    error = None
    try:
        actual = reference.evaluate(tree, env)
    except namespace['FErr'] as refused:
        error = refused
    assert error is None, ('unexpected angular refusal', source, error)
    assert (actual.kind, actual.v) == (kind, Fraction(expected)), ('angular result', source, actual, expected)
    checks += 1
    return actual.v

for line in (ROOT / 'docs/tasks/artifacts/formula_structure/angle_cases.tsv').read_text().splitlines():
    if not line or line.startswith('#'):
        continue
    source, kind, expected = line.split('\t')
    value(source, kind, int(expected))
    rows += 1
assert rows == 42
for x, y, expected in [(1, 6, 80537678), (6, 1, 9462322),
                       (6, -1, 350537678), (-6, 1, 170537678),
                       (-6, -1, 189462322), (0, 1, 90000000), (1, 0, 0)]:
    env = {'p': {'kind': 'point', 'value': (0, 0), 'origin': 'geometry'},
           'q': {'kind': 'point', 'value': (x, y), 'origin': 'geometry'}}
    direction = value('dir(p, q)', 'angle', expected, env)
    same_vector = value('atan2(%d um, %d um)' % (y, x), 'angle', expected)
    assert direction == same_vector, ('dir/atan2 agreement', x, y)
    checks += 1
for source in ['tan(90 deg)', 'tan(-90 deg)', 'tan(270 deg)', 'tan(450 deg)',
               'tan(-450 deg)', 'tan(180 deg / 2)', 'atan2(0 um, 0 um)']:
    error = None
    try:
        reference.evaluate(reference.parse(source), {})
    except Exception as refused:
        error = refused
    assert isinstance(error, namespace['FErr']) and error.token == 'formula_domain', ('angular domain', source, error)
    reason = 'atan2(0, 0) has no direction' if source.startswith('atan2') else 'tan: angle is an exact odd-quarter-turn pole'
    assert error.msg == reason, ('angular domain reason', source, error)
    checks += 1
# Poles are exact microdegree residues; a representable neighbor is a finite ratio, not a pole.
for source in ['tan(89.999999 deg)', 'tan(90.000001 deg)']:
    tree = reference.parse(source)
    reference.infer(tree, {})
    actual = reference.evaluate(tree, {})
    assert actual.kind == 'ratio' and abs(actual.v) > 1000000, ('finite pole neighbor', source, actual)
    checks += 1
print('reference angle contracts: %d explicit rows / %d controls / 0 fail; curated angular identities, no arbitrary precision certificate' % (rows, checks))
