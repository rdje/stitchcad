"""D84: signed principal angles, raw stored sweeps/equality, actual book replay."""
from fractions import Fraction
from pathlib import Path
import math
import os
import runpy
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[4]
namespace, reference = runpy.run_path(str(ROOT / 'docs/tasks/artifacts/formula_structure/formula_input.py'))['load_arithmetic_context']()
reference.sigs.update({
    'atan': [(['ratio'], False, 'angle')],
    'atan2': [(['length', 'length'], False, 'angle'), (['ratio', 'ratio'], False, 'angle')],
    'arc_length': [(['angle', 'length'], False, 'length')],
    'dir': [(['point', 'point'], False, 'angle')],
})
checks = 0


def value(source, kind, expected, env=None, binding=False):
    global checks
    env = {} if env is None else env
    before = {key: dict(entry) for key, entry in env.items()}
    error = actual = None
    try:
        if binding:
            actual = reference.statement('let saved: %s = %s' % (kind, source), env)[3]
        else:
            tree = reference.parse(source)
            assert reference.infer(tree, env) == kind, (source, kind)
            actual = reference.evaluate(tree, env)
    except namespace['FErr'] as refused:
        error = refused
    assert error is None, ('valid signed angle refused', source, error)
    assert (actual.kind, actual.v) == (kind, Fraction(expected)), (source, actual, expected)
    assert env == before, ('caller environment changed', source, env)
    checks += 1
    return actual


def refusal(source, token, reason):
    global checks
    error = None
    try:
        tree = reference.parse(source)
        reference.infer(tree, {})
        reference.evaluate(tree, {})
    except namespace['FErr'] as refused:
        error = refused
    assert error is not None and error.token == token and error.msg == reason, (source, error)
    checks += 1


# This first independent assertion reproduces the former outer inverse modulo.
value('atan(-1.0)', 'angle', -45000000)
for number in [-2, -1, 0, 1, 2]:
    expected = math.degrees(math.atan(number)) * 1000000
    rounded = math.floor(abs(expected) + 0.5) * (-1 if expected < 0 else 1)
    value('atan(%d.0)' % number, 'angle', rounded)
for y, x, expected in [
    (1, 1, 45000000), (-1, 1, -45000000),
    (1, -1, 135000000), (-1, -1, -135000000),
    (0, 1, 0), (0, -1, 180000000), (1, 0, 90000000), (-1, 0, -90000000),
    (1, -1000000000, 180000000), (-1, -1000000000, -180000000),
]:
    value('atan2(%d um, %d um)' % (y, x), 'angle', expected)
    value('atan2(%d.0, %d.0)' % (y, x), 'angle', expected)
value('atan2(-0 um, -1 um)', 'angle', 180000000)
value('atan2(-0.0, -1.0)', 'angle', 180000000)
# Finite real atan is open at ±90, but the stored microdegree may round to either endpoint.
for number, expected in [(2**100, 90000000), (-2**100, -90000000), (Fraction(1, 10**12), 0)]:
    env = {'input': {'kind': 'ratio', 'value': number * 1000000, 'origin': 'computed_fixture'}}
    value('atan(input)', 'angle', expected, env)
for source in ['atan2(0 um, 0 um)', 'atan2(0.0, 0.0)']:
    refusal(source, 'formula_domain', 'atan2(0, 0) has no direction')
for source, kinds in [('atan2(1 um, 1.0)', 'length, ratio'),
                      ('atan2(1.0, 1 um)', 'ratio, length'),
                      ('atan2(1, 1)', 'count, count')]:
    refusal(source, 'formula_dimension', '`atan2` has no signature for (%s)' % kinds)

bound_env = {}
for index, (source, expected) in enumerate([
    ('0 deg', 0), ('360 deg', 360000000), ('720 deg', 720000000),
    ('-360 deg', -360000000), ('-720 deg', -720000000),
    ('450 deg', 450000000), ('-450 deg', -450000000),
    ('360 deg - 0.000001 deg / 2', 360000000),
    ('-360 deg + 0.000001 deg / 2', -360000000),
    ('0.000001 deg / 2', 1), ('-0.000001 deg / 2', -1),
    ('atan(-1.0)', -45000000), ('atan2(-1 um, -1 um)', -135000000),
]):
    result = value(source, 'angle', expected, binding=True)
    bound_env['turn_%d' % index] = {'kind': result.kind, 'value': result.v, 'origin': 'recipe'}
for source, expected in [
    ('0 deg == 360 deg', 0), ('360 deg == 720 deg', 0), ('-360 deg == 0 deg', 0),
    ('-360 deg < 0 deg', 1), ('360 deg == -(-360 deg)', 1), ('-0 deg == 0 deg', 1),
    ('turn_0 == turn_1', 0), ('turn_1 != turn_2', 1), ('turn_3 < turn_0', 1),
    ('turn_2 > turn_1', 1), ('turn_7 == turn_1', 1), ('turn_8 == turn_3', 1),
]:
    value(source, 'boolean', expected, bound_env)
for index, source, expected in [
    (0, '0 deg', 0), (1, '360 deg', 6), (2, '720 deg', 13),
    (3, '-360 deg', -6), (4, '-720 deg', -13), (5, '450 deg', 8), (6, '-450 deg', -8),
]:
    value('arc_length(%s, 1 um)' % source, 'length', expected)
    value('arc_length(turn_%d, 1 um)' % index, 'length', expected, bound_env)
value('arc_length(turn_1, 1 mm)', 'length', 6283, bound_env)
# Before binding, half a microdegree remains exact. Binding rounds once; its later arc differs.
for source, raw, stored in [('0.000001 deg / 2', 9, 17), ('-0.000001 deg / 2', -9, -17)]:
    result = value(source, 'angle', 1 if stored > 0 else -1, binding=True)
    env = {'angle_input': {'kind': 'angle', 'value': result.v, 'origin': 'recipe'}}
    value('arc_length(%s, 1000 m)' % source, 'length', raw)
    value('arc_length(angle_input, 1000 m)', 'length', stored, env)
for x, y, principal, direction in [(6, -1, -9462322, 350537678),
                                    (-6, -1, -170537678, 189462322),
                                    (-1, 0, 180000000, 180000000)]:
    env = {'p': {'kind': 'point', 'value': (0, 0), 'origin': 'geometry'},
           'q': {'kind': 'point', 'value': (x, y), 'origin': 'geometry'}}
    value('atan2(%d um, %d um)' % (y, x), 'angle', principal)
    value('dir(p, q)', 'angle', direction, env)

work = ROOT / 'target/formula_signed_angle_replay'
if work.exists():
    assert not (work / '.git').exists(), 'scratch cannot be a Git repository'
    shutil.rmtree(work)
shutil.copytree(ROOT / 'docs/book/src', work)
p = work / 'spec/formula-language/examples.md'
text = p.read_text()
anchor = [line for line in text.splitlines() if line.startswith('| `dart_count` |')]
assert len(anchor) == 1
rows = [
    ('full_turn', 'angle', '360 deg', '360.0 deg'),
    ('turn_twice', 'angle', 'full_turn + full_turn', '720.0 deg'),
    ('negative_turn', 'angle', '-full_turn', '-360.0 deg'),
    ('full_sweep', 'length', 'arc_length(full_turn, 1 um)', '0.0006 cm'),
    ('negative_principal', 'angle', 'atan(-1.0)', '-45.0 deg'),
    ('negative_quadrant', 'angle', 'atan2(-1 um, -1 um)', '-135.0 deg'),
    ('turn_equal_zero', 'boolean', 'full_turn == 0 deg', 'false'),
    ('turn_test_read', 'count', 'if(turn_equal_zero, 1, 0)', '0'),
]
added = ''.join('\n| `%s` | %s | `%s` | %s | signed-angle replay |' % row for row in rows)
p.write_text(text.replace(anchor[0], anchor[0] + added, 1))


def replay(refusal=None):
    global checks
    result = subprocess.run(
        ['bash', 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'],
        cwd=ROOT, env=dict(os.environ, FORMULA_BOOK=str(work)), capture_output=True, text=True)
    output = result.stdout + result.stderr
    (work / ('replay.log' if refusal is None else 'refusal.log')).write_text(output)
    if refusal is None:
        assert result.returncode == 0 and 'bindings: 25 · mismatches: 0' in output, (result.returncode, output)
    else:
        assert result.returncode == 1 and refusal in output, (result.returncode, output)
    checks += 1


replay()
p.write_text(p.read_text().replace('| -45.0 deg | signed-angle replay |', '| 315.0 deg | signed-angle replay |', 1))
replay('computes to -45.0, the chapter publishes 315.0')
print('reference signed-angle contracts: %d independent principal/binding/equality/sweep/replay controls / 0 fail' % checks)
