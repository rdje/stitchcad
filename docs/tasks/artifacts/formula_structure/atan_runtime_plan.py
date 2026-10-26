"""Source-gated bounded atan/atan2 plan; no native API or full-population rerun."""
from fractions import Fraction as F
from math import isqrt
from pathlib import Path
import hashlib
import runpy

ROOT = Path(__file__).resolve().parents[4]
PROFILE = ROOT / 'scripts/local_environment.py'
WITNESS = ROOT / 'crates/sc-core/tests/fixtures/inverse_rounding_certificate.tsv'
PI = ROOT / 'crates/sc-core/tests/fixtures/pi640_certificate.tsv'
BITS = 576
S = 1 << BITS
TERMS = 64
HALVES = 5
ORACLE_BITS = 768
OS = 1 << ORACLE_BITS
MAX_ROOT_STEPS = 0


def ceildiv(a, b):
    return (a + b - 1) // b


def root_floor(n):
    global MAX_ROOT_STEPS
    assert S * S <= n <= 2 * S * S
    t, steps = 2 * S, 0
    while True:
        steps += 1
        next_t = (t + n // t) // 2
        if next_t >= t:
            assert t * t <= n < (t + 1) ** 2 and steps <= 13
            MAX_ROOT_STEPS = max(MAX_ROOT_STEPS, steps)
            return t
        t = next_t


def atan_interval(x):
    lo, hi = x
    assert 0 <= lo <= hi <= S // 32 and hi - lo <= 11
    square = lo * lo // S, ceildiv(hi * hi, S)
    assert square[1] - square[0] <= 3 and square[1] <= S // 1024
    p = x
    low = high = 0
    for n in range(TERMS):
        assert p[1] - p[0] <= (11 if n == 0 else 3)
        a, b = p[0] // (2 * n + 1), ceildiv(p[1], 2 * n + 1)
        if n % 2:
            low -= b
            high -= a
        else:
            low += a
            high += b
        p = p[0] * square[0] // S, ceildiv(p[1] * square[1], S)
    result = max(0, low - 1), high + 1
    assert result[1] - result[0] <= 328
    return result


def base_atan(value, pi):
    assert 0 <= value <= 1
    if not value:
        return 0, 0
    x = value.numerator * S // value.denominator, ceildiv(value.numerator * S, value.denominator)
    for step in range(HALVES):
        square = x[0] * x[0] // S, ceildiv(x[1] * x[1], S)
        low_n, high_n = S * (S + square[0]), S * (S + square[1])
        rl = root_floor(low_n)
        rh = rl + ceildiv(high_n - rl * rl, 2 * rl)
        assert rl * rl <= low_n and rh * rh >= high_n
        assert rh - rl <= x[1] - x[0] + 2
        x = x[0] * S // (S + rh), ceildiv(x[1] * S, S + rl)
        assert x[1] - x[0] <= 11 and x[1] <= S >> (step + 1)
    low, high = atan_interval(x)
    # atan(original)/(2*pi)=16*atan(after five halvings)/pi.
    result = low * 16 * S // pi[1], ceildiv(high * 16 * S, pi[0])
    assert result[1] - result[0] < 4096
    return result


def model_atan(value, pi):
    if abs(value) > 1:
        low, high = base_atan(1 / abs(value), pi)
        low, high = S // 4 - high, S // 4 - low
    else:
        low, high = base_atan(abs(value), pi)
    return (low, high) if value >= 0 else (-high, -low)


def model_atan2(y, x, pi):
    if not x:
        if not y:
            raise ValueError('zero vector')
        return (S // 4, S // 4) if y > 0 else (-S // 4, -S // 4)
    if not y:
        return (S // 2, S // 2) if x < 0 else (0, 0)
    low, high = model_atan(abs(y / x), pi)
    if x < 0:
        low, high = S // 2 - high, S // 2 - low
    return (low, high) if y > 0 else (-high, -low)


def oracle_series(value):
    """Independent768-bit rational-input series, no square-root/half-angle reduction."""
    assert 0 <= value <= F(1, 2)
    x = value.numerator * OS // value.denominator, ceildiv(value.numerator * OS, value.denominator)
    square = x[0] * x[0] // OS, ceildiv(x[1] * x[1], OS)
    power = x
    low = high = 0
    for n in range(512):
        a, b = power[0] // (2 * n + 1), ceildiv(power[1], 2 * n + 1)
        if n % 2:
            low -= b
            high -= a
        else:
            low += a
            high += b
        power = power[0] * square[0] // OS, ceildiv(power[1] * square[1], OS)
    return F(max(0, low - 1), OS), F(high + 1, OS)


def oracle_pi():
    bounds = []
    for base in (2, 3):
        total = sum((F((-1) ** n, (2 * n + 1) * base ** (2 * n + 1)) for n in range(512)), F(0))
        bounds.append((total, total + F(1, 1025 * base ** 1025)))
    return 4 * (bounds[0][0] + bounds[1][0]), 4 * (bounds[0][1] + bounds[1][1])


def oracle_atan(value, pi, anchor):
    if not value:
        return F(0), F(0)
    if value < 0:
        low, high = oracle_atan(-value, pi, anchor)
        return -high, -low
    if value > 1:
        low, high = oracle_atan(1 / value, pi, anchor)
        return F(1, 4) - high, F(1, 4) - low
    residual = (value - F(1, 2)) / (1 + value / 2)
    low, high = oracle_series(abs(residual))
    radians = ((anchor[0] + low, anchor[1] + high) if residual >= 0
               else (anchor[0] - high, anchor[1] - low))
    return radians[0] / (2 * pi[1]), radians[1] / (2 * pi[0])


def oracle_atan2(y, x, pi, anchor):
    if not x:
        if not y:
            raise ValueError('zero vector')
        return (F(1, 4), F(1, 4)) if y > 0 else (-F(1, 4), -F(1, 4))
    low, high = oracle_atan(y / x, pi, anchor)
    offset = (F(1, 2) if y >= 0 else -F(1, 2)) if x < 0 else 0
    return low + offset, high + offset


def main():
    profile = runpy.run_path(str(PROFILE))
    profile['plan_producer'](ROOT, sources=(Path(__file__), PROFILE, WITNESS, PI))
    witness = dict(line.split('\t') for line in WITNESS.read_text().splitlines() if line and not line.startswith('#'))
    assert all(witness[key] == str(value) for key, value in {
        'schema': 1, 'population': 45_000_000, 'chunk_count': 450, 'row_bytes': 288,
        'fraction_bits': 640, 'turn_band_bits': 600, 'angular_margin_bits': 540,
        'slope_denominator_bound': ((1 << 128) - 1) ** 2,
    }.items()), 'certificate profile changed'
    sources = {name[7:]: value for name, value in witness.items() if name.startswith('source:')}
    assert set(sources) == {
        'crates/sc-core/examples/inverse_threshold_chunk.rs',
        'crates/sc-core/tests/fixtures/pi640_certificate.tsv',
        'docs/tasks/artifacts/formula_structure/inverse_threshold_certificate.py',
        'docs/tasks/artifacts/formula_structure/verify_inverse_certificate.py',
        'docs/tasks/artifacts/formula_structure/inverse_certificate_witness.py',
    }, 'certificate source inventory'
    profile['plan_producer'](ROOT, sources=tuple(ROOT / name for name in sources))
    assert all(hashlib.sha256((ROOT / name).read_bytes()).hexdigest() == value for name, value in sources.items()), 'certificate basis changed'
    assert int(witness['minimum_slope_margin_scaled']) > 1 << 104
    # Analytic width/workspace and decreasing integer Newton envelope.
    assert F(3, 4) * 11 + F(11, 4) == 11
    assert F(11, 1024) + F(3, 32) + 2 < 3
    assert F(16 * 328, 3) + F(1, 9) + 2 < 4096
    assert 16 * (S // 32 + 328) < S
    assert (1 << (5 * (2 * TERMS + 1))) > S
    assert 4096 * (1 << 540) < S and not 4096 * (1 << 540) < 1 << 512
    e = S
    for _ in range(11):
        e = e * e // (2 * S) + 1
    assert e == 1  # At most one exact floor-root step, then one stopping test: <=13.
    pi_rows = dict(line.split('\t') for line in PI.read_text().splitlines() if line and not line.startswith('#'))
    pi_low = int(pi_rows['pi_lower_hex'], 16) >> (640 - BITS)
    pi = pi_low, pi_low + 1
    assert 3 * S < pi[0] < pi[1] < 4 * S
    independent_pi = oracle_pi()
    anchor = oracle_series(F(1, 2))
    u = (1 << 128) - 1
    cases = [F(0), F(1), F(-1), F(1, 2), F(-1, 2), F(1, 3), F(2, 3), F(1, 7),
             F(u, 1_000_000), F(-u, 1_000_000), F(1, u * 1_000_000), F(1, u * u),
             F(u * u - 1, u * u), F(u, (u - 1) * 1_000_000)]
    for value in cases:
        actual, expected = model_atan(value, pi), oracle_atan(value, independent_pi, anchor)
        assert F(actual[0], S) <= expected[0] <= expected[1] <= F(actual[1], S), ('independent atan', value)
    vectors = [(F(y), F(x)) for y, x in [(0, 1), (0, -1), (1, 0), (-1, 0), (1, 1),
               (1, -1), (-1, -1), (-1, 1), (1, 2), (2, 1), (-2, 1), (1, -2)]]
    vectors += [(F(sign, u), F(-u)) for sign in (-1, 1)]
    vectors += [(F(sign * u), F(1, u)) for sign in (-1, 1)]
    for y, x in vectors:
        actual, expected = model_atan2(y, x, pi), oracle_atan2(y, x, independent_pi, anchor)
        assert F(actual[0], S) <= expected[0] <= expected[1] <= F(actual[1], S), ('independent atan2', y, x)
    for function in [lambda: model_atan2(F(0), F(0), pi),
                     lambda: oracle_atan2(F(0), F(0), independent_pi, anchor)]:
        try:
            function()
        except ValueError:
            pass
        else:
            raise AssertionError('zero vector accepted')
    for n in [S * S, S * S + 1, 2 * S * S - 1, 2 * S * S]:
        assert root_floor(n) == isqrt(n)
    assert MAX_ROOT_STEPS > 8
    assert max(S * S * 2 + 4 * S, ((1 << 256) - 1) * S, 720_000_000 * S) < 1 << 1154
    print('atan plan: source-gated margin540/P576/5half/64terms; turn width<2^-564; workspace<2^1154; Newton<=13, rc=0')
    print('independent addition-series atan%d/atan2%d/zero-vector2/root4/insufficient-budget2 pass; observed root_steps_max=%d, rc=0' %
          (len(cases), len(vectors), MAX_ROOT_STEPS))


if __name__ == '__main__':
    main()
