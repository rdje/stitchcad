"""Bounded inverse threshold/Farey cohort, never full-population or native admission."""
from fractions import Fraction as F
from math import factorial, isqrt
from pathlib import Path
import hashlib
import json
import runpy
import time

ROOT = Path(__file__).resolve().parents[4]
PROFILE = ROOT / 'scripts/local_environment.py'
HELPER = Path(__file__).with_name('finite_threshold_feasibility.py')
PI = ROOT / 'crates/sc-core/tests/fixtures/pi640_certificate.tsv'
OUTPUT = ROOT / 'target/trig_inverse_feasibility/0282/report.json'
BITS = 640
S = 1 << BITS
TERMS = 64
M = 1_000_000
POPULATION = 45 * M
Q = ((1 << 128) - 1) ** 2
BAND = F(1, 1 << 600)  # Turn distance on each side of a half-microdegree target.


def ceildiv(a, b):
    return (a + b - 1) // b


def trig_interval(x, cosine):
    lo, hi = x
    assert 0 <= lo <= hi <= S and hi - lo <= 3
    square = lo * lo // S, ceildiv(hi * hi, S)
    assert square[1] - square[0] <= 8
    term = (S, S) if cosine else x
    lower = upper = 0
    for n in range(TERMS):
        assert term[1] - term[0] <= 12
        if n % 2:
            lower -= term[1]
            upper -= term[0]
        else:
            lower += term[0]
            upper += term[1]
        k = (2 * n + 1) * (2 * n + 2) if cosine else (2 * n + 2) * (2 * n + 3)
        term = term[0] * square[0] // (S * k), ceildiv(term[1] * square[1], S * k)
    # For x<=1, both64-term alternating tails are <1/128!<2^-640.
    return max(0, lower - 1), min(S, upper + 1)


def tangent_turn(turn, pi_bounds):
    assert 0 <= turn <= F(1, 7)  # Includes the first octant and the rational45-degree refusal control.
    pi_low, pi_high = pi_bounds
    theta = (2 * turn.numerator * pi_low // turn.denominator,
             ceildiv(2 * turn.numerator * pi_high, turn.denominator))
    sine, cosine = trig_interval(theta, False), trig_interval(theta, True)
    assert cosine[0] > 0
    return F(sine[0] * S // cosine[1], S), F(ceildiv(sine[1] * S, cosine[0]), S)


def slope_band(center, pi_bounds, delta=BAND):
    assert 0 <= center - delta < center + delta <= F(1, 7)
    return tangent_turn(center - delta, pi_bounds)[0], tangent_turn(center + delta, pi_bounds)[1]


def atan_fraction_bounds(value):
    partial = sum((F((-1) ** n, 2 * n + 1) * value ** (2 * n + 1)
                   for n in range(512)), F(0))
    return partial, partial + value ** 1025 / 1025


def controls(pi_bounds, certify):
    oracle_scale = 1 << 768
    r2, r3 = isqrt(2 * oracle_scale ** 2), isqrt(3 * oracle_scale ** 2)
    oracles = [(F(0), (F(0), F(0))), (F(1, 8), (F(1), F(1))),
               (F(1, 16), (F(r2, oracle_scale) - 1, F(r2 + 1, oracle_scale) - 1)),
               (F(1, 24), (2 - F(r3 + 1, oracle_scale), 2 - F(r3, oracle_scale)))]
    for phase, oracle in oracles:
        low, high = tangent_turn(phase, pi_bounds)
        assert low <= oracle[0] <= oracle[1] <= high, ('algebraic tangent oracle', phase)
    pi_low, pi_high = (F(p, S) for p in pi_bounds)
    for slope in [F(1, 4), F(1, 2)]:
        low, high = atan_fraction_bounds(slope)
        low_phase, high_phase = low / (2 * pi_high), high / (2 * pi_low)
        assert tangent_turn(low_phase, pi_bounds)[0] <= slope <= tangent_turn(high_phase, pi_bounds)[1], 'independent atan enclosure'
    assert certify(*slope_band(F(1, 8), pi_bounds), Q) is None, 'rational tan45 must refuse'
    assert certify(*slope_band(F(1, 16), pi_bounds, F(1, 32)), Q) is None, 'wide inverse band must refuse'


def main():
    profile = runpy.run_path(str(PROFILE))
    sources = (Path(__file__), PROFILE, HELPER, PI)
    profile['plan_producer'](ROOT, sources=sources, directories=(OUTPUT.parent,), output_files=(OUTPUT,))
    helper = runpy.run_path(str(HELPER))
    pi_bounds = helper['load_pi']()
    assert (BITS, TERMS, Q.bit_length()) == (640, 64, 256)
    assert factorial(2 * TERMS) > S, 'alternating remainder exceeds the prototype quantum'
    controls(pi_bounds, helper['certify'])
    indices = sorted({0, 1, 2, POPULATION // 2 - 1, POPULATION // 2,
                      POPULATION - 3, POPULATION - 2, POPULATION - 1,
                      *(i * (POPULATION - 1) // 31 for i in range(32))})
    evidence = []
    start = time.perf_counter_ns()
    for index in indices:
        center = F(2 * index + 1, 720 * M)
        low, high = slope_band(center, pi_bounds)
        gap = helper['certify'](low, high, Q)
        assert gap is not None, ('uncertified inverse threshold', index)
        evidence.append({'index': index, 'slope_low': str(low), 'slope_high': str(high),
                         'gap_low': str(gap[0]), 'gap_high': str(gap[1])})
    elapsed = time.perf_counter_ns() - start
    report = {'scope': 'inverse_feasibility_cohort_only', 'population': POPULATION,
              'fraction_bits': BITS, 'terms': TERMS, 'turn_band': str(BAND),
              'slope_denominator_bound': str(Q), 'cohort_count': len(indices),
              'cohort_elapsed_ns': elapsed, 'algebraic_oracles': 4, 'atan_oracles': 2,
              'negative_controls': 2, 'evidence': evidence,
              'sources': {p.relative_to(ROOT).as_posix(): hashlib.sha256(p.read_bytes()).hexdigest() for p in sources}}
    profile['enter_producer'](ROOT, sources=sources, directories=(OUTPUT.parent,), output_files=(OUTPUT,))
    profile['plan_producer'](ROOT, output_files=(OUTPUT,))
    OUTPUT.write_text(json.dumps(report, indent=2) + '\n')
    print('inverse feasibility: %d bands/algebraic4/Fraction-atan2/negative2 controls pass, rc=0' % len(indices))
    print('profile:640bit/64terms/turn-band2^-600/Q_slope256; cohort_ns=%d; full45M/runtime NOT verified' % elapsed)


if __name__ == '__main__':
    main()
