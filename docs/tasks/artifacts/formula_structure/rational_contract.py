"""D83: actual reduced-value refusals, independently authored Fraction boundaries."""
from fractions import Fraction
from pathlib import Path
import runpy
ROOT = Path(__file__).resolve().parents[4]
namespace, reference = runpy.run_path(str(ROOT / 'docs/tasks/artifacts/formula_structure/formula_input.py'))['load_arithmetic_context']()
checks = 0

def run(source, env=None):
    env = {} if env is None else env
    tree = reference.parse(source)
    reference.infer(tree, env)
    return reference.evaluate(tree, env)

def accept(source, expected, env=None):
    global checks
    error = None
    try:
        actual = run(source, env)
    except namespace['FErr'] as refusal:
        error = refusal
    assert error is None, ('valid reduced value refused', source, error)
    assert actual.v == Fraction(expected), ('exact reduced result', source, actual, expected)
    checks += 1

def refuse(source, bits, operation, env=None):
    global checks
    error = None
    try:
        run(source, env)
    except namespace['FErr'] as refusal:
        error = refusal
    assert error is not None, ('oversized rational accepted', source, bits)
    assert error.token == 'formula_domain', ('rational diagnostic token', source, error)
    assert 'max_rational_bits=128' in error.msg and 'measured=%d' % bits in error.msg, ('rational bound/measurement', source, error)
    assert operation in error.msg, ('rational operation', source, error)
    checks += 1

# Boundaries derived from powers of two, not from the evaluator's bit-width counter.
for bits in [127, 128, 129]:
    integer = 2 ** bits - 1
    if bits <= 128:
        accept(str(integer), integer)
        # Names carry exact internal values; this tests denominators without literal quantization.
        for kind in ['length', 'angle', 'area', 'ratio']:
            for sign in [1, -1]:
                fraction = Fraction(sign, integer)
                env = {'x': {'kind': kind, 'value': fraction, 'origin': 'parameter'}}
                accept('x', fraction, env)
    else:
        refuse(str(integer), bits, 'literal count')
        for kind in ['length', 'angle', 'area', 'ratio']:
            for sign in [1, -1]:
                env = {'x': {'kind': kind, 'value': Fraction(sign, integer), 'origin': 'parameter'}}
                refuse('x', bits, 'name', env)

# Literal conversion checks exact internal units BEFORE the input quantum can hide width.
for source in ['0.' + '0' * 100 + '5 cm', '0.' + '0' * 100 + '5',
               '0.' + '0' * 100 + '5 pct']:
    parts = source.split()
    factor = 10000 if len(parts) == 2 else 1000000
    exact = Fraction(parts[0]) * factor
    # Independent Fraction oracle supplies the measurement; powers-of-two cases above fix edges.
    bits = max(abs(exact.numerator).bit_length(), exact.denominator.bit_length())
    refuse(source, bits, 'literal')
for source, expected in [('0.' + '0' * 100 + ' cm', 0),
                         ('1.' + '0' * 100 + ' cm', 10000),
                         ('0.5000000000000000000000000000000000000000 um', 1)]:
    accept(source, expected)
# A scaled unit input may exceed the limit although its surface integer fits it.
refuse(str(2 ** 127) + ' m', 147, 'literal length')

big = 2 ** 127
# Every completed node is bounded: a later cancellation must not rescue an oversized sum/product.
refuse('%d + %d - %d' % (big, big, big), 129, 'bin')
wide_ratio = {'x': {'kind': 'ratio', 'value': Fraction(big), 'origin': 'parameter'}}
refuse('x - -x', 129, 'bin', wide_ratio)
refuse('(1 um / %d) ^ 2' % (2 ** 64), 129, 'sq')
refuse('%d * %d' % (2 ** 64, 2 ** 64), 129, 'bin')
refuse('(1 um / %d) / %d' % (2 ** 64, 2 ** 65), 130, 'bin')
sum_env = {name: {'kind': 'length', 'value': Fraction(1, denominator), 'origin': 'parameter'}
           for name, denominator in [('x', 2 ** 65 - 1), ('y', 2 ** 65 + 1)]}
refuse('x + y', 130, 'bin', sum_env)
accept('0', 0)
accept('-x', -big, {'x': {'kind': 'angle', 'value': Fraction(big), 'origin': 'parameter'}})
# The published bound is consumed, never duplicated as an evaluator constant.
reference.limits['max_rational_bits'] = 127
error = None
try:
    run(str(big))
except namespace['FErr'] as refused:
    error = refused
finally:
    reference.limits['max_rational_bits'] = 128
assert error is not None and 'max_rational_bits=127' in error.msg and 'measured=128' in error.msg
checks += 1
# Reduced results are allowed even when an implementation's raw cross-products exceed 128 bits.
ratio_env = {'x': {'kind': 'ratio', 'value': Fraction(1, 2 ** 120), 'origin': 'parameter'}}
accept('x * 1.0', Fraction(1, 2 ** 120), ratio_env)
accept('x / 1.0', Fraction(1, 2 ** 120), ratio_env)
accept('%d / %d' % (big, big), 1000000)
accept('0.0 * x', 0, ratio_env)
accept('x - x', 0, ratio_env)
# Width of the RESULT's internal units differs from the temporary dimensionless rational.
denom_env = {'x': {'kind': 'ratio', 'value': Fraction(1, 2 ** 64), 'origin': 'parameter'}}
refuse('x ^ 2', 148, 'sq', denom_env)
edge_env = {'edge_a': {'kind': 'edge', 'value': Fraction(2 ** 127 + 1), 'origin': 'geometry'}}
refuse('param_at(edge_a, 1 um / %d)' % (2 ** 20), 142, 'call', edge_env)
# Only completed values of the chosen branch are evaluated, but both branches still parse/type-check.
accept('if(1 == 1, 1, %d + %d)' % (big, big), 1)
accept('if(1 != 1, %d + %d, 1)' % (big, big), 1)
refuse('if(1 == 1, %d + %d, 1)' % (big, big), 129, 'bin')
refuse('if(1 != 1, 1, %d + %d)' % (big, big), 129, 'bin')
# Literal canonicalization precedes evaluation, including the untaken branch.
refuse('if(1 == 1, 1, %d)' % (2 ** 128), 129, 'literal count')
# Non-numeric semantic refs and boolean results do not acquire a fictitious rational domain.
accept('1 == 1', 1)
point_env = {'p': {'kind': 'point', 'value': (big, big), 'origin': 'geometry'}}
assert run('p', point_env).v == (big, big)
checks += 1
# Statement bookkeeping must not reclassify opaque geometry refs as rational values.
opaque_env = {'p': {'kind': 'point', 'value': (2 ** 128, 0), 'origin': 'geometry'},
              'e': {'kind': 'edge', 'value': Fraction(2 ** 128), 'origin': 'geometry'}}
assert reference.statement('p', opaque_env)[3].v == (2 ** 128, 0)
assert reference.statement('e', opaque_env)[3].v == 2 ** 128
checks += 2
# Implicit tolerance reads are also numeric nodes; neither comparison route may bypass width.
reference.reserved['eps_num'] = ('length', True, Fraction(2 ** 128))
reference.sigs['within'] = [(['T', 'T', 'tolerance'], False, 'boolean')]
refuse('within(1 um, 1 um, eps_num)', 129, 'name')
error = None
try:
    reference.statement('assert same: eps_num = 1 um == 1 um', {})
except namespace['FErr'] as refused:
    error = refused
assert error is not None and error.token == 'formula_domain' and 'name length result' in error.msg
checks += 1
print('reference rational contracts: %d controls / 0 fail; independent Fraction boundaries, limit128' % checks)
