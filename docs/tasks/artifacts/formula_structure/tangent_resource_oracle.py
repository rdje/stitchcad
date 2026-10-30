"""Independent requested-policy/stage/progress fixture,never native or frozen output input.

The diagnostic dyadic enclosure supplies iteration separation,with the exact rounded endpoint
bit agreement required by the public width contract. The alternate-pi Fraction oracle checks
every mathematical conclusion separately. This does not reuse the older model's width shortcut.
"""
from fractions import Fraction as F
from functools import lru_cache
from pathlib import Path
import runpy

ROOT = Path(__file__).resolve().parents[4]
MODEL_SOURCE = Path(__file__).with_name('tangent_reduction_plan.py')
EXACT_SOURCE = Path(__file__).with_name('tangent_value_oracle.py')
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
PROFILE['plan_producer'](ROOT, sources=(Path(__file__), MODEL_SOURCE, EXACT_SOURCE, PROFILE_SOURCE))
MODEL = runpy.run_path(str(MODEL_SOURCE))
EXACT = runpy.run_path(str(EXACT_SOURCE))
enclose = lru_cache(maxsize=None)(MODEL['enclose'])
exact = lru_cache(maxsize=None)(EXACT['outcome'])
U, A = (1 << 128) - 1, 90_000_000
NEAR = F(247385123034593398372138492992328993860, 8635369834810561370044731507908276509)
WIDE = F(A * (U // A) - 1, U // A)
CASES = (
    ('empty_zero', 'angle', F(0), 0, 0),
    ('fold_short_zero', 'angle', F(0), 155, 0),
    ('fold_short_pole', 'angle', F(A), 155, 0),
    ('kind_before_empty', 'length', F(0), 0, 0),
    ('zero', 'angle', F(0), 156, 0),
    ('unit', 'angle', F(A, 2), 156, 0),
    ('negative_unit', 'angle', F(-A, 2), 156, 0),
    ('turn', 'angle', F(-4 * A), 156, 0),
    ('pole', 'angle', F(A), 156, 0),
    ('negative_pole', 'angle', F(-A), 156, 0),
    ('first', 'angle', F(30_000_000), 2073, 512),
    ('first_more', 'angle', F(30_000_000), 16412, 7680),
    ('first_bits', 'angle', F(30_000_000), 2072, 512),
    ('first_terms', 'angle', F(30_000_000), 2073, 511),
    ('both_first', 'angle', F(30_000_000), 2072, 511),
    ('first_zero_terms', 'angle', F(30_000_000), 2073, 0),
    ('second_bits', 'angle', NEAR, 2073, 3584),
    ('second_terms', 'angle', NEAR, 8219, 512),
    ('both_second', 'angle', NEAR, 2073, 512),
    ('third_bits', 'angle', NEAR, 4122, 3584),
    ('third_terms', 'angle', NEAR, 8219, 1536),
    ('both_third', 'angle', NEAR, 4122, 1536),
    ('refined', 'angle', NEAR, 8219, 3584),
    ('refined_more', 'angle', NEAR, 16412, 7680),
    ('negative_refined', 'angle', -NEAR, 8219, 3584),
    ('near_above', 'angle', NEAR + F(1, NEAR.denominator), 8219, 3584),
    ('wide_first', 'angle', WIDE, 2073, 512),
    ('wide_exact', 'angle', WIDE, 4122, 1536),
    ('wide_bits', 'angle', WIDE, 2073, 1536),
    ('wide_terms', 'angle', WIDE, 4122, 512),
    ('wide_storage', 'angle', F(A * 10**20 - 1, 10**20), 16412, 7680),
    ('tiny', 'angle', F(1, U), 2073, 512),
    ('both_first_zero', 'angle', F(30_000_000), 2072, 0),
)


def outcome(kind, angle, maximum_bits, maximum_terms):
    if kind != 'angle':
        return 'prepare:kind', '-', '-', '-', '-', 'formula_dimension'
    reserved = charged = 0

    def resource(stage, unit, precision, requested):
        maximum = maximum_bits if unit == 'integer_bits' else maximum_terms
        return f'{stage}:{unit}:{maximum}', precision, reserved, charged, requested, '-'

    if maximum_bits < 156:
        return resource('prepare', 'integer_bits', '-', 156)
    reserved = 156
    n, d = angle.numerator, angle.denominator
    role = MODEL['fold'](n, d).role
    if role == 'pole':
        assert exact(angle)[0] == 'pole'
        return 'prepare:pole', '-', '-', '-', '-', 'formula_domain'
    if role in ('zero', 'unit'):
        tag, answer = exact(angle)
        assert tag == 'value'
        return 'value:' + answer, '-', reserved, charged, '-', '-'
    precision = 128
    while True:
        bits, terms = MODEL['integer_limit'](precision), 4 * precision
        stage = 'prepare' if precision == 128 else 'execute'
        if bits > maximum_bits:
            return resource(stage, 'integer_bits', precision, bits)
        if terms > maximum_terms - charged:
            return resource(stage, 'series_terms', precision, terms)
        reserved, charged = bits, charged + terms
        interval = enclose(n, d, precision)
        low, high = map(MODEL['round_positive'], interval.ppm)
        if low.bit_length() > 128 and low.bit_length() == high.bit_length():
            assert exact(angle) == ('width', str(low.bit_length()))
            return f'execute:width:{low.bit_length()}', precision, reserved, charged, '-', 'formula_domain'
        if low == high:
            answer = str(-low if interval.folded.negative and low else low)
            assert exact(angle) == ('value', answer)
            return 'value:' + answer, precision, reserved, charged, '-', '-'
        precision *= 2


def counter_bounds():
    # For every admitted B,16B < I(B)<=usize::MAX implies2B and4B fit.
    # A charged<=MAX invariant plus terms<=MAX-charged implies charged+terms<=MAX.
    for width in (16, 32, 64):
        maximum = (1 << width) - 1
        b = maximum // 16
        assert 2 * b <= maximum and 4 * b <= maximum
        assert MODEL['integer_limit'](maximum) > maximum
        assert 0 <= maximum - (maximum - 1) < 2
    return 3


def fixture():
    assert counter_bounds() == 3 and len({case[0] for case in CASES}) == len(CASES) == 33
    rows = ['# label\tkind\tnumerator\tdenominator\tmaximum_bits\tmaximum_terms\toutcome\tprecision\treserved\tcharged\trequested\tdiagnostic']
    for label, kind, angle, bits, terms in CASES:
        result = outcome(kind, angle, bits, terms)
        rows.append('\t'.join(map(str, (label, kind, angle.numerator, angle.denominator, bits, terms, *result))))
    return '\n'.join(rows) + '\n'


if __name__ == '__main__':
    print(fixture(), end='')
