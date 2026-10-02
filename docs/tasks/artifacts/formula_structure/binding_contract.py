"""D83: once-rounded bounded let values, distinct from exact unbound expressions."""
from decimal import Decimal, localcontext, ROUND_HALF_UP
from fractions import Fraction
from pathlib import Path
import os
import runpy
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[4]
context = ROOT / 'docs/tasks/artifacts/formula_structure/formula_input.py'
namespace, reference = runpy.run_path(str(context))['load_arithmetic_context']()
LOW, HIGH = -(2**63), 2**63 - 1
checks = 0


def refusal(source, kind, expected, env=None):
    global checks
    error = None
    try:
        reference.statement(source, {} if env is None else env)
    except namespace['FErr'] as refused:
        error = refused
    assert error is not None, ('oversize bound integer accepted', source)
    low = 0 if kind == 'count' else LOW
    message = 'let bound binding: %s storage=i64 [%s, %s], measured=%s' % (
        kind, low, HIGH, expected)
    assert error.token == 'formula_domain' and error.msg == message, (source, error)
    checks += 1


# Two individually i64-safe literals reproduce the predecessor binding defect.
refusal('let bound: count = 9223372036854775807 + 1', 'count', HIGH + 1)


def binds(source, kind, expected, env=None):
    global checks
    env = {} if env is None else env
    original = {key: dict(value) for key, value in env.items()}
    error = result = None
    try:
        result = reference.statement(source, env)
    except namespace['FErr'] as refused:
        error = refused
    assert error is None, ('valid binding refused', source, error)
    assert result[:3] == ('let', 'bound', kind), (source, result)
    value = result[3]
    assert value.kind == kind and value.v == Fraction(expected), (source, value, expected)
    assert value.v.denominator == 1, ('noninteger binding', source, value)
    assert env == original, ('binding mutated caller environment', source, env)
    checks += 1
    return value


def computed_env(kind, value):
    # These are exact computed fixtures, not declarations of stored measurement inputs.
    return {'x': {'kind': kind, 'value': Fraction(value), 'origin': 'computed_fixture'}}


def decimal_integer(value):
    # Independent standard-library rounding, not the reference's quotient/remainder routine.
    with localcontext() as context:
        context.prec = 80
        exact = Decimal(value.numerator) / Decimal(value.denominator)
        return int(exact.to_integral_value(rounding=ROUND_HALF_UP))


binds('let bound: length = 1 um / 2', 'length', 1)
binds('let bound: length = -1 um / 2', 'length', -1)
binds('let bound: count = 1 / 2.0', 'count', 1)
for kind in ['angle', 'ratio', 'count']:
    endpoints = [0, 1, HIGH] if kind == 'count' else [LOW, LOW + 1, -1, 0, 1, HIGH - 1, HIGH]
    for integer in endpoints:
        binds('let bound: %s = x' % kind, kind, integer, computed_env(kind, integer))
    for endpoint in ([HIGH] if kind == 'count' else [HIGH, LOW]):
        for numerator in [-51, -50, -49, 0, 49, 50, 51]:
            value = Fraction(endpoint) + Fraction(numerator, 100)
            expected = decimal_integer(value)
            env = computed_env(kind, value)
            source = 'let bound: %s = x' % kind
            if LOW <= expected <= HIGH:
                binds(source, kind, expected, env)
            else:
                refusal(source, kind, expected, env)

# Smaller scalar domains still govern length/area, including signed inclusive endpoints.
for kind, bound in [('length', 10**9), ('area', 10**18)]:
    for integer in [-bound, 0, bound]:
        binds('let bound: %s = x' % kind, kind, integer, computed_env(kind, integer))
# Both a safe-child expression and the D95 direct spelling reach the full signed endpoint.
binds('let bound: angle = -9223372036854.775807 deg - 0.000001 deg', 'angle', LOW)
binds('let bound: angle = -9223372036854.775808 deg', 'angle', LOW)
binds('let bound: angle = 360 deg', 'angle', 360000000)
binds('let bound: angle = -720 deg', 'angle', -720000000)
# Oversized temporaries may cancel into a valid binding within the existing rational width.
binds('let bound: count = (9223372036854775807 + 1) - 1', 'count', HIGH)
value = reference.evaluate(reference.parse('9223372036854775807 + 1'), {})
assert value.kind == 'count' and value.v == HIGH + 1
checks += 1
# Replay publishes returned values explicitly; subsequent reads see the rounded binding.
value = binds('let bound: length = 1 um / 2', 'length', 1)
env = {'saved': {'kind': value.kind, 'value': value.v, 'origin': 'recipe'}}
binds('let bound: length = saved + saved', 'length', 2, env)
# Refusal does not publish into the caller-owned environment.
env = {'safe': {'kind': 'count', 'value': Fraction(1), 'origin': 'recipe'}}
original = {key: dict(value) for key, value in env.items()}
refusal('let bound: count = 9223372036854775807 + 1', 'count', HIGH + 1, env)
assert env == original
checks += 1
# Boolean stays Boolean; an unbound rational expression does not bind implicitly.
binds('let bound: boolean = 1 == 1', 'boolean', 1)
assert reference.statement('1 um / 2', {})[3].v == Fraction(1, 2)
checks += 1
# Negative fractions cannot round to zero to hide Count's exact domain refusal.
error = None
try:
    reference.statement('let bound: count = 1 / -10.0', {})
except namespace['FErr'] as refused:
    error = refused
assert error is not None and error.token == 'formula_domain', error
assert error.msg == 'bin / count result: count domain=[0, unbounded], measured=-1/10', error
checks += 1
binds('let bound: boolean = 1 != 1', 'boolean', 0)

# Consume an independently altered declared width rather than duplicating 64 at runtime.
work = ROOT / 'target/formula_binding_source'
if work.exists():
    shutil.rmtree(work)
shutil.copytree(ROOT / 'docs/book/src', work)
p = work / 'spec/formula-language.md'
p.write_text(p.read_text().replace('signed i64 microdegrees', 'signed i32 microdegrees', 1))
original = reference.storage
try:
    reference.storage = namespace['storage_domains'](str(work))
    assert reference.storage['angle'] == (32, -2**31, 2**31 - 1)
    assert reference.statement('let bound: angle = 2147.483647 deg', {})[3].v == 2**31 - 1
    error = None
    try:
        reference.statement('let bound: angle = 2147.483648 deg', {})
    except namespace['FErr'] as refused:
        error = refused
    assert error is not None and error.token == 'formula_domain', error
    message = 'let bound binding: angle storage=i32 [-2147483648, 2147483647], measured=2147483648'
    assert error.msg == message, error
    checks += 2
finally:
    reference.storage = original
p.write_text(p.read_text().replace('signed i32 microdegrees', 'signed microdegrees', 1))
error = None
try:
    namespace['storage_domains'](str(work))
except ValueError as refused:
    error = refused
assert error is not None and 'angle binding storage' in str(error), error
checks += 1

# Copied-book replay tests the real census consumer and subsequent recipe name read.
copied = work / 'spec/formula-language/examples.md'
text = copied.read_text()
rows = [line for line in text.splitlines() if line.startswith('| `dart_count` |')]
assert len(rows) == 1, 'copied-book binding anchor changed'
added = ('\n| `binding_half` | count | `1 / 2.0` | 1 | binding quantum |'
         '\n| `binding_read` | count | `binding_half + binding_half` | 2 | replay |')
copied.write_text(text.replace(rows[0], rows[0] + added, 1))
# Restore the normative storage table after the isolated source-reader refusal.
p.write_text((ROOT / 'docs/book/src/spec/formula-language.md').read_text())
result = subprocess.run(
    ['bash', 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'],
    cwd=ROOT, env=dict(os.environ, FORMULA_BOOK=str(work)), capture_output=True, text=True)
(work / 'replay.log').write_text(result.stdout + result.stderr)
assert result.returncode == 0 and 'bindings: 19 · mismatches: 0' in result.stdout, (
    result.returncode, result.stdout, result.stderr)
checks += 1
print('reference binding contracts: %d independent Fraction/Decimal controls / 0 fail' % checks)
