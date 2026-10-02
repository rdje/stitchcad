"""D83: exact scalar-domain boundaries from independent Fraction expectations."""
from fractions import Fraction
from pathlib import Path
import runpy
ROOT = Path(__file__).resolve().parents[4]
import contextlib
import io
context_output = io.StringIO()
with contextlib.redirect_stdout(context_output):
    namespace, reference = runpy.run_path(str(ROOT / 'docs/tasks/artifacts/formula_structure/formula_input.py'))['load_arithmetic_context']()
assert context_output.getvalue() == '', ('numeric context ran unrelated output', context_output.getvalue())
checks = 1

def run(source, env=None):
    env = {} if env is None else env
    tree = reference.parse(source)
    reference.infer(tree, env)
    return reference.evaluate(tree, env)

def accept(source, kind, expected, env=None):
    global checks
    error = None
    result = None
    try: result = run(source, env)
    except namespace['FErr'] as refused: error = refused
    assert error is None, ('valid scalar refused', source, error)
    assert (result.kind, result.v) == (kind, Fraction(expected)), (source, result, expected)
    checks += 1

def refuse(source, kind, value, low, high, operation, env=None):
    global checks
    error = None
    try: run(source, env)
    except namespace['FErr'] as refused: error = refused
    assert error is not None, ('out-of-domain scalar accepted', source, value)
    assert error.token == 'formula_domain', ('scalar token', source, error)
    bound = '[%s, %s]' % (low, high if high is not None else 'unbounded')
    assert error.msg == '%s: %s domain=%s, measured=%s' % (operation, kind, bound, Fraction(value)), (source, error)
    checks += 1

# First fixture independently reproduces the originally accepted >1 km literal.
refuse('1000000001 um', 'length', 1000000001, -10**9, 10**9, 'literal length input')
for kind, bound in [('length', 10**9), ('area', 10**18)]:
    for sign in [1, -1]:
        for delta in [-1, 0, Fraction(1, 10), 1]:
            value = sign * (bound + delta)
            env = {'x': {'kind': kind, 'value': value, 'origin': 'parameter'}}
            if delta <= 0: accept('x', kind, value, env)
            else: refuse('x', kind, value, -bound, bound, 'name x %s result' % kind, env)
    accept('x', kind, 0, {'x': {'kind': kind, 'value': Fraction(0), 'origin': 'parameter'}})
# Canonical input rounds before scalar validation; expression values never round to hide excess.
for source in ['1000000000.4 um', '-1000000000.4 um']:
    accept(source, 'length', 10**9 if source[0] != '-' else -10**9)
for source in ['1000000000.5 um', '-1000000000.5 um']:
    refuse(source, 'length', 10**9+1, -10**9, 10**9, 'literal length input')
for source, value in [('1000000000 um + 1 um / 10', Fraction(10**9)+Fraction(1,10)),
                      ('-1000000000 um - 1 um / 10', -Fraction(10**9)-Fraction(1,10))]:
    refuse(source, 'length', value, -10**9, 10**9, 'bin %s length result' % ('+' if value>0 else '-'))
accept('1000000000 um', 'length', 10**9)
accept('-1000000000 um', 'length', -10**9)
accept('(1000000000 um) ^ 2', 'area', 10**18)
accept('1000000000 um * -1000000000 um', 'area', -10**18)
# Squared rational widths can stay valid while an area magnitude leaves its own domain.
area_env = {'x': {'kind': 'area', 'value': Fraction(10**18), 'origin': 'parameter'}}
refuse('x + 1 um * 1 um', 'area', 10**18+1, -10**18, 10**18, 'bin + area result', area_env)
refuse('1000000000 um * 2', 'length', 2*10**9, -10**9, 10**9, 'bin * length result')
refuse('1000000000 um / 0.5', 'length', 2*10**9, -10**9, 10**9, 'bin / length result')
refuse('1000000000 um + 1 um - 1 um', 'length', 10**9+1, -10**9, 10**9, 'bin + length result')
# Count is nonnegative at each completed node, including exact nonintegral results.
accept('0', 'count', 0)
accept('1 / 2.0', 'count', Fraction(1,2))
accept('2 - 1', 'count', 1)
refuse('0 - 1', 'count', -1, 0, None, 'bin - count result')
refuse('1 / -2.0', 'count', Fraction(-1,2), 0, None, 'bin / count result')
refuse('0 - 1 + 1', 'count', -1, 0, None, 'bin - count result')
refuse('x', 'count', Fraction(-1,10), 0, None, 'name x count result', {'x': {'kind': 'count', 'value': Fraction(-1,10), 'origin': 'parameter'}})
# Selector/call results enter the scalar boundary; opaque geometry payloads do not.
reference.sigs.update({'x': [(['point'], False, 'length')], 'len': [(['edge'], False, 'length')]})
refuse('x(p)', 'length', 10**9+1, -10**9, 10**9, 'call x length result', {'p': {'kind': 'point', 'value': (10**9+1,0), 'origin': 'geometry'}})
refuse('len(e)', 'length', 10**9+1, -10**9, 10**9, 'call len length result', {'e': {'kind': 'edge', 'value': Fraction(10**9+1), 'origin': 'geometry'}})
refuse('hypot(1000000000 um, 1000000000 um)', 'length', 1414213562, -10**9, 10**9, 'call hypot length result')
opaque = {'p': {'kind': 'point', 'value': (10**30,0), 'origin': 'geometry'}}
assert run('p', opaque).v == (10**30,0)
checks += 1
# Untaken arithmetic does not execute, but both branches' literals canonicalize at parse time.
accept('if(1 == 1, 1 um, 1000000000 um + 1 um)', 'length', 1)
accept('if(1 != 1, 1000000000 um + 1 um, 1 um)', 'length', 1)
refuse('if(1 == 1, 1000000000 um + 1 um, 1 um)', 'length', 10**9+1, -10**9, 10**9, 'bin + length result')
refuse('if(1 == 1, 1 um, 1000000001 um)', 'length', 10**9+1, -10**9, 10**9, 'literal length input')
# Implicit tolerance names cannot bypass scalar checks.
reference.reserved['eps_num'] = ('length', True, Fraction(10**9+1))
reference.sigs['within'] = [(['T','T','tolerance'], False, 'boolean')]
refuse('within(1 um, 1 um, eps_num)', 'length', 10**9+1, -10**9, 10**9, 'name eps_num length result')
# This scalar slice adds no magnitude restriction to angle/ratio (storage i64 is separately owned).
for kind in ['angle','ratio']:
    for value in [-(2**100), 2**100]:
        accept('x', kind, value, {'x': {'kind': kind, 'value': Fraction(value), 'origin': 'parameter'}})
# Assert's implicit tolerance follows the same named-value guard.
error = None
try: reference.statement('assert same: eps_num = 1 um == 1 um', {})
except namespace['FErr'] as refused: error = refused
assert error is not None and error.token == 'formula_domain' and error.msg == 'name eps_num length result: length domain=[-1000000000, 1000000000], measured=1000000001', error
checks += 1
# Read an independently altered normative bound; runtime may not duplicate its original constant.
import shutil
work = ROOT / 'target/formula_scalar_source'
if work.exists(): shutil.rmtree(work)
shutil.copytree(ROOT / 'docs/book/src', work)
p = work / 'spec/units-and-tolerances.md'
text = p.read_text()
assert text.count('10⁹ µm') >= 1
p.write_text(text.replace('10⁹ µm', '10⁸ µm', 1))
original = reference.domains
try:
    reference.domains = namespace['scalar_domains'](str(work))
    assert reference.domains['length'] == (-10**8, 10**8)
    refuse('100000001 um', 'length', 10**8+1, -10**8, 10**8, 'literal length input')
finally: reference.domains = original
# Removing the declared nonnegative rule cannot leave a fabricated default.
p = work / 'spec/formula-language.md'
p.write_text(p.read_text().replace('i64, never negative', 'i64', 1))
error = None
try: namespace['scalar_domains'](str(work))
except ValueError as refused: error = refused
assert error is not None and 'nonnegative Count contract' in str(error), error
checks += 1
print('reference scalar contracts: %d independent Fraction/domain controls / 0 fail' % checks)
