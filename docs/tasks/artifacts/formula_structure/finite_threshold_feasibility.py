"""Bounded forward threshold/Farey feasibility cohort; no full-population or runtime admission."""
from fractions import Fraction as F
from pathlib import Path
from math import isqrt
import hashlib
import json
import runpy
import time

ROOT = Path(__file__).resolve().parents[4]
PROFILE = ROOT / 'scripts/local_environment.py'
PI = ROOT / 'crates/sc-core/tests/fixtures/pi640_certificate.tsv'
OUTPUT = ROOT / 'target/trig_threshold_feasibility/0277/report.json'
M = 1_000_000
U = (1 << 128) - 1
BITS = 512
S = 1 << BITS
BAND = F(1, 1 << 384)
HALVES = 5
TERMS = (BITS + 9) // 10


def load_pi():
    rows = dict(line.split('\t') for line in PI.read_text().splitlines()
                if line and not line.startswith('#'))
    assert int(rows['fraction_bits']) == 640 and BITS <= 640
    low, high = int(rows['pi_lower_hex'], 16), int(rows['pi_upper_hex'], 16)
    assert high == low + 1
    return low, high


def ceildiv(a, b):
    assert a >= 0 and b > 0
    return (a + b - 1) // b


def interval(lo, hi):
    assert 0 <= lo <= hi
    return lo, hi


def rational(value):
    assert value >= 0
    return interval(value.numerator * S // value.denominator,
                    ceildiv(value.numerator * S, value.denominator))


def square(value):
    lo, hi = value
    return interval(lo * lo >> BITS, ceildiv(hi * hi, S))


def divide(left, right):
    a, b = left
    c, d = right
    assert c > 0
    return interval(a * S // d, ceildiv(b * S, c))


def root(value):
    lo, hi = value
    low = isqrt(lo * S)
    high = isqrt(hi * S)
    return interval(low, high + (high * high < hi * S))


def atan_endpoint(z):
    assert 0 <= z <= S >> HALVES
    zz = z * z
    power_lo = power_hi = z
    low = high = 0
    for n in range(TERMS):
        divisor = 2 * n + 1
        a, b = power_lo // divisor, ceildiv(power_hi, divisor)
        if n % 2 == 0:
            low += a
            high += b
        else:
            low -= b
            high -= a
        power_lo = power_lo * zz >> (2 * BITS)
        power_hi = ceildiv(power_hi * zz, S * S)
    remainder = ceildiv(power_hi, 2 * TERMS + 1)
    return interval(max(0, low - remainder), high + remainder)


def asin_phase(value, pi_bounds):
    pi_low, pi_high = pi_bounds
    assert 0 <= value <= 1
    sine = rational(value)
    sq = square(sine)
    radical = root(interval(max(0, S - sq[1]), S - sq[0]))
    x = divide(sine, (S + radical[0], S + radical[1]))
    x = interval(x[0], min(x[1], S))  # True tan(asin(s)/2) is at most one.
    for step in range(HALVES):
        sq = square(x)
        radical = root((S + sq[0], S + sq[1]))
        x = divide(x, (S + radical[0], S + radical[1]))
        x = interval(x[0], min(x[1], S >> (step + 1)))
    low = atan_endpoint(x[0])[0]
    high = atan_endpoint(x[1])[1]
    pi = (pi_low >> (640 - BITS), ceildiv(pi_high, 1 << (640 - BITS)))
    return divide((low << HALVES, high << HALVES), pi)


def neighbors(value, maximum):
    """Continued-fraction candidates; independent determinant/band predicates certify them."""
    p0, q0, p1, q1 = 0, 1, 1, 0
    n, d = value.numerator, value.denominator
    while d:
        quotient = n // d
        q2 = q0 + quotient * q1
        if q2 > maximum:
            break
        p0, q0, p1, q1 = p1, q1, p0 + quotient * p1, q2
        n, d = d, n - quotient * d
    else:
        return None  # The selected point itself has an admitted denominator.
    k = (maximum - q0) // q1
    return sorted((F(p0 + k * p1, q0 + k * q1), F(p1, q1)))


def certify(low, high, maximum):
    assert low <= high
    bounds = neighbors((low + high) / 2, maximum)
    if bounds is None:
        return None
    left, right = bounds
    assert 0 < left.denominator <= maximum and 0 < right.denominator <= maximum
    assert right.numerator * left.denominator - left.numerator * right.denominator == 1
    assert left.denominator + right.denominator > maximum
    if not left < low <= high < right:
        return None
    return left, right


def band(center, pi_bounds):
    assert 0 < center - BAND < center + BAND < 1
    low = asin_phase(center - BAND, pi_bounds)[0]
    high = asin_phase(center + BAND, pi_bounds)[1]
    return F(low, S), F(high, S)


def exact_asin_series(value, terms):
    """Different positive series and exact Fraction arithmetic; tail uses coefficients <=1."""
    assert 0 <= value <= F(1, 4)
    term = value
    total = F(0)
    for n in range(terms):
        total += term
        term *= F((2 * n + 1) ** 2, 2 * (n + 1) * (2 * n + 3)) * value * value
    tail = value ** (2 * terms + 1) / (1 - value * value)
    return total, total + tail


def controls(pi_bounds):
    pi_low, pi_high = pi_bounds
    for sine, expected in [(F(0), F(0)), (F(1, 2), F(1, 12)), (F(1), F(1, 4))]:
        lo, hi = asin_phase(sine, pi_bounds)
        assert F(lo, S) <= expected <= F(hi, S), ('known inverse value', sine)
    pi_lo, pi_hi = F(pi_low, 1 << 640), F(pi_high, 1 << 640)
    for value in [F(1, 2 * M), F(1, 8), F(1, 4)]:
        low, high = exact_asin_series(value, 160)
        oracle = low / (2 * pi_hi), high / (2 * pi_lo)
        actual = asin_phase(value, pi_bounds)
        assert F(actual[0], S) <= oracle[0] <= oracle[1] <= F(actual[1], S), ('Fraction oracle', value)
    low, high = exact_asin_series(F(1, 2000), 64)
    oracle = F(1, 4) - high / pi_lo, F(1, 4) - low / pi_hi
    actual = asin_phase(1 - F(1, 2 * M), pi_bounds)
    assert F(actual[0], S) <= oracle[0] <= oracle[1] <= F(actual[1], S), 'near-extremum oracle'
    for maximum in range(1, 33):
        low, high = F(7, 13) - F(1, 1 << 20), F(7, 13) + F(1, 1 << 20)
        rational_present = any(low <= F(p, q) <= high for q in range(1, maximum + 1)
                               for p in range(q + 1))
        assert (certify(low, high, maximum) is None) == rational_present, ('small-Q oracle', maximum)
    assert certify(*band(F(1, 2), pi_bounds), 360 * M * U) is None, 'scale1 sin30 midpoint must refuse'
    wide_low = F(asin_phase(F(3, 8), pi_bounds)[0], S)
    wide_high = F(asin_phase(F(5, 8), pi_bounds)[1], S)
    assert certify(wide_low, wide_high, 360 * M * U) is None, 'wide band must refuse'


def main():
    profile = runpy.run_path(str(PROFILE))
    profile['plan_producer'](ROOT, sources=(Path(__file__), PROFILE, PI),
                              directories=(OUTPUT.parent,), output_files=(OUTPUT,))
    pi_bounds = load_pi()
    assert (F(1, U) / (360 * M)).denominator == 360 * M * U
    assert (F(1, U) / F(U)).denominator == U * U
    controls(pi_bounds)
    indices = sorted({0, 1, 2, M // 2 - 1, M // 2, M - 3, M - 2, M - 1,
                      *(i * (M - 1) // 31 for i in range(32))})
    start = time.perf_counter_ns()
    evidence = []
    for index in indices:
        center = F(2 * index + 1, 2 * M)
        low, high = band(center, pi_bounds)
        gap = certify(low, high, 360 * M * U)
        assert gap is not None, ('cohort band not certified', index)
        evidence.append({'index': index, 'phase_low': str(low), 'phase_high': str(high),
                         'gap_low': str(gap[0]), 'gap_high': str(gap[1])})
    elapsed = time.perf_counter_ns() - start
    report = {'scope': 'feasibility_cohort_only', 'fraction_bits': BITS, 'forward_value_band_bits': 384, 'forward_value_band': str(BAND),
              'half_angle_steps': HALVES, 'atan_terms': TERMS,
              'phase_denominator_bits': (360 * M * U).bit_length(), 'slope_denominator_bits': (U * U).bit_length(),
              'forward_threshold_population': M, 'inverse_octant_threshold_population': 45 * M,
              'cohort_count': len(indices), 'cohort_elapsed_ns': elapsed,
              'controls': {'known_values': 3, 'Fraction_oracles': 4, 'small_Q': 32,
                           'scale1_rejected': True, 'wide_band_rejected': True},
              'sources': {path.relative_to(ROOT).as_posix(): hashlib.sha256(path.read_bytes()).hexdigest()
                          for path in (Path(__file__), PI)}, 'evidence': evidence}
    profile['enter_producer'](ROOT, sources=(Path(__file__), PROFILE, PI),
                              directories=(OUTPUT.parent,), output_files=(OUTPUT,))
    profile['plan_producer'](ROOT, output_files=(OUTPUT,))
    OUTPUT.write_text(json.dumps(report, indent=2) + '\n')
    print('threshold feasibility: %d forward bands certified; known3/Fraction4/small-Q32/negative2 controls pass, rc=0' % len(indices))
    print('profile:%dbit/value-band2^-384; Q_phase157/Q_slope256; cohort_ns=%d; full populations NOT verified' % (BITS, elapsed))


if __name__ == '__main__':
    main()
