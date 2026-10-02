"""D95: exact canonical literal width and operator identity, distinct from bound i64."""
from decimal import Decimal, localcontext, ROUND_HALF_UP
from fractions import Fraction
from pathlib import Path
import runpy

ROOT = Path(__file__).resolve().parents[4]
context = ROOT / 'docs/tasks/artifacts/formula_structure/formula_input.py'
namespace, reference = runpy.run_path(str(context))['load_arithmetic_context']()
MAX128, LOW64, HIGH64 = 2**128 - 1, -(2**63), 2**63 - 1
checks = 0


def machine_integer(integer, quantum, unit=''):
    """Authored fixture spelling: decimal division by a power-of-ten quantum, never floats."""
    whole, remainder = divmod(integer, quantum)
    digits = len(str(quantum)) - 1
    text = '%d.%0*d' % (whole, digits, remainder)
    return text + (' ' + unit if unit else '')


def parses(source, expected):
    global checks
    error = tree = None
    try:
        tree = reference.parse(source)
    except namespace['FErr'] as refused:
        error = refused
    assert error is None, ('valid canonical input refused', source, error)
    assert tree == expected, ('canonical kind/integer/operator identity', source, tree, expected)
    checks += 1
    return tree


def evaluates(tree, kind, expected):
    global checks
    reference.infer(tree, {})
    error = value = None
    try:
        value = reference.evaluate(tree, {})
    except namespace['FErr'] as refused:
        error = refused
    assert error is None, ('valid exact canonical value refused', tree, error)
    assert (value.kind, value.v) == (kind, Fraction(expected)), (tree, value, expected)
    checks += 1


def refuses_parse(source, operation, bits):
    global checks
    error = None
    try:
        reference.parse(source)
    except namespace['FErr'] as refused:
        error = refused
    assert error is not None and error.token == 'formula_domain', (source, error)
    message = '%s: rational max_rational_bits=128, measured=%d' % (operation, bits)
    assert error.msg == message, (source, error, message)
    checks += 1


# Independently authored powers establish the meaning of the 128-bit absolute magnitude limit.
for bits in [63, 64, 127, 128]:
    for integer in [2**(bits - 1), 2**bits - 1]:
        for kind, source in [
            ('count', str(integer)),
            ('angle', machine_integer(integer, 10**6, 'deg')),
            ('ratio', machine_integer(integer, 10**6)),
        ]:
            node = ('lit', kind, Fraction(integer))
            tree = parses(source, node)
            evaluates(tree, kind, integer)
            if kind != 'count':
                negative = parses('-' + source, ('neg', node))
                evaluates(negative, kind, -integer)
        # Percent spelling and a bare decimal are the same ratio node at every width.
        parses(machine_integer(integer, 10**4, 'pct'), ('lit', 'ratio', Fraction(integer)))

# Exact canonical zero/sign/operator structure is not simplified algebraically.
zero = ('lit', 'angle', Fraction(0))
parses('0 deg', zero)
parses('-0 deg', ('neg', zero))
parses('--0 deg', ('neg', ('neg', zero)))
parses('-1.0 ^ 2', ('neg', ('sq', ('lit', 'ratio', Fraction(10**6)))))
# Count has no unary negation rule even though syntax retains the operator node.
negative_count = parses('-' + str(2**63), ('neg', ('lit', 'count', Fraction(2**63))))
error = None
try:
    reference.infer(negative_count, {})
except namespace['FErr'] as refused:
    error = refused
assert error is not None and error.token == 'formula_dimension', error
checks += 1

# Unit aliases and longer reducible spellings preserve canonical identity, not lexical text.
for source in ['2.5 cm', '25 mm', '25000 um', '0.025 m']:
    parses(source, ('lit', 'length', Fraction(25000)))
parses('18 in', ('lit', 'length', Fraction(457200)))
parses('0' * 100 + str(MAX128), ('lit', 'count', Fraction(MAX128)))
parses('0.0000005' + '0' * 100, ('lit', 'ratio', Fraction(1)))
parses('0.' + '0' * 100, ('lit', 'ratio', Fraction(0)))
parses('1', ('lit', 'count', Fraction(1)))
parses('1.0', ('lit', 'ratio', Fraction(1000000)))

# The standard-library Decimal oracle supplies half-quantum expectations independently.
for unit, kind, factor in [('um', 'length', 1), ('deg', 'angle', 10**6), ('pct', 'ratio', 10**4)]:
    for source_number in ['0.49', '0.5', '0.51']:
        with localcontext() as context:
            context.prec = 80
            authored = Decimal(source_number) / factor
            # The fixture spelling has enough places to represent each exact authored value.
            text = format(authored, 'f')
            expected = int((authored * factor).to_integral_value(rounding=ROUND_HALF_UP))
        node = ('lit', kind, Fraction(expected))
        parses(text + ' ' + unit, node)
        negative = parses('-' + text + ' ' + unit, ('neg', node))
        evaluates(negative, kind, -expected)

# Converted exact width refuses before input rounding, including unit and bare-decimal paths.
refuses_parse(str(2**128), 'literal count', 129)
refuses_parse(machine_integer(2**128, 10**6, 'deg'), 'literal angle conversion', 129)
refuses_parse(machine_integer(2**128, 10**6), 'literal ratio conversion', 129)
refuses_parse(machine_integer(2**128, 10**4, 'pct'), 'literal ratio conversion', 129)
for source, factor, operation in [
    ('0.' + '0' * 100 + '5', 10**6, 'literal ratio conversion'),
    ('0.' + '0' * 100 + '5 deg', 10**6, 'literal angle conversion'),
]:
    exact = Fraction(Decimal(source.split(' ')[0])) * factor
    measured = max(abs(exact.numerator).bit_length(), exact.denominator.bit_length())
    assert measured > 128
    refuses_parse(source, operation, measured)

# Canonical length input still obeys its scalar domain after input rounding.
parses('1000000000.4 um', ('lit', 'length', Fraction(10**9)))
error = None
try:
    reference.parse('1000000000.5 um')
except namespace['FErr'] as refused:
    error = refused
assert error is not None and error.token == 'formula_domain', error
assert error.msg == 'literal length input: length domain=[-1000000000, 1000000000], measured=1000000001', error
checks += 1

# Direct signed endpoint keeps a positive child outside i64; binding supplies the signed range.
for kind, suffix in [('angle', ' deg'), ('ratio', '')]:
    source = machine_integer(2**63, 10**6) + suffix
    node = ('lit', kind, Fraction(2**63))
    parses('-' + source, ('neg', node))
    result = reference.statement('let bound: %s = -%s' % (kind, source), {})
    assert result[3].kind == kind and result[3].v == LOW64, result
    checks += 1
    error = None
    try:
        reference.statement('let bound: %s = %s' % (kind, source), {})
    except namespace['FErr'] as refused:
        error = refused
    message = 'let bound binding: %s storage=i64 [%s, %s], measured=%s' % (
        kind, LOW64, HIGH64, 2**63)
    assert error is not None and error.token == 'formula_domain' and error.msg == message, error
    checks += 1
# Count's widest canonical node can cancel into a valid bound integer without a sign fold.
source = 'let bound: count = (%d - %d) + %d' % (MAX128, MAX128, HIGH64)
assert reference.statement(source, {})[3].v == HIGH64
checks += 1
print('canonical literal contracts: %d independent node/Decimal controls / 0 fail; D95 width/identity' % checks)
