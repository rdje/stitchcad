"""Independent exact Fraction tangent fixtures;no Rust,native output or frozen fixture inputs.

Signed rational centering,atan(1/2)+atan(1/3) pi and direct power/factorial alternating bounds
are separate from the native absolute fold,Machin pi and fixed-point interval recurrences.
The finite fixture budget must actually separate every selected row or this producer refuses.
"""
import argparse
from fractions import Fraction as F
from functools import lru_cache
import hashlib
from math import factorial
from pathlib import Path
import runpy

ROOT = Path(__file__).resolve().parents[4]
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
PROFILE['plan_producer'](ROOT, sources=(Path(__file__), PROFILE_SOURCE))
M, A, U = 1_000_000, 90_000_000, (1 << 128) - 1


def alternating(value, terms, cosine=None):
    if cosine is None:
        power = lambda i: 2 * i + 1
        divisor = lambda i: 2 * i + 1
    else:
        power = lambda i: 2 * i + (0 if cosine else 1)
        divisor = lambda i: factorial(power(i))
    partial = sum(((-1)**i * value**power(i) / divisor(i) for i in range(terms)), F(0))
    other = partial + (-1)**terms * value**power(terms) / divisor(terms)
    return min(partial, other), max(partial, other)


@lru_cache(maxsize=1)
def pi_bounds():
    # atan(1/2)+atan(1/3)=pi/4 by addition and both angles in the first quadrant.
    a, b = alternating(F(1, 2), 192), alternating(F(1, 3), 192)
    return 4 * (a[0] + b[0]), 4 * (a[1] + b[1])


def nearest(value):
    assert value >= 0
    return (2 * value.numerator + value.denominator) // (2 * value.denominator)


def outcome(value):
    centered = (value + A) % (2 * A) - A
    if abs(centered) == A:
        return 'pole', '-'
    if not centered:
        return 'value', '0'
    negative, distance = centered < 0, abs(centered)
    if distance == A // 2:
        return 'value', str(-M if negative else M)
    reciprocal = distance > A // 2
    if reciprocal:
        distance = A - distance
    pi_low, pi_high = pi_bounds()
    low, high = distance * pi_low / (2 * A), distance * pi_high / (2 * A)
    assert 0 < low <= high < 1
    # Monotonicity on[0,pi/4] lets independent exact endpoint powers bound both components.
    sine = alternating(low, 32, False)[0], alternating(high, 32, False)[1]
    cosine = alternating(high, 32, True)[0], alternating(low, 32, True)[1]
    numerator, denominator = (cosine, sine) if reciprocal else (sine, cosine)
    assert 0 < denominator[0] <= denominator[1]
    a = nearest(M * numerator[0] / denominator[1])
    b = nearest(M * numerator[1] / denominator[0])
    if a.bit_length() > 128 and a.bit_length() == b.bit_length():
        return 'width', str(a.bit_length())
    assert a == b, ('finite fixture not separated by independent bounds', value, a, b)
    return 'value', str(-a if negative and a else a)


def cases():
    result = []
    for degree in range(-720, 721, 15):
        result.append((f'degree_{degree}', F(degree * M)))
    for denominator in (1, 3, 7, 10**20, U // A):
        for side in (-1, 1):
            for sign in (-1, 1):
                result.append((f'pole_neighbor_{denominator}_{side}_{sign}',
                               F(sign * (A * denominator + side), denominator)))
    for denominator in (1, 2, 3, U, U - 1):
        for sign in (-1, 1):
            result.append((f'full_{denominator}_{sign}', F(sign * U, denominator)))
    for sign in (-1, 1):
        result.append((f'tiny_{sign}', F(sign, U)))
        turns = U // (2 * A)
        result.append((f'full_turn_{sign}', F(sign * turns * 2 * A)))
        odd = (U // A - 1) | 1
        result.append((f'full_pole_{sign}', F(sign * odd * A)))
        for delta in (-1, 0, 1):
            result.append((f'near_midpoint_{delta}_{sign}',
                F(sign * (247385123034593398372138492992328993860 + delta),
                  8635369834810561370044731507908276509)))
    for label, value in result:
        assert abs(value.numerator) <= U and 0 < value.denominator <= U, label
    assert len({label for label, _ in result}) == len(result)
    return result


def fixture():
    rows = ['# label\tnumerator\tdenominator\toutcome\tinteger_ppm_or_exact_width_bits']
    counts = dict(value=0, pole=0, width=0)
    for label, value in cases():
        kind, payload = outcome(value)
        counts[kind] += 1
        rows.append(f'{label}\t{value.numerator}\t{value.denominator}\t{kind}\t{payload}')
    # Different exact sqrt(3) midpoint inequalities also establish the tan30 rounded integer.
    assert 3 * (2 * 577350 - 1)**2 < 4 * M**2 < 3 * (2 * 577350 + 1)**2
    assert outcome(F(30 * M)) == ('value', '577350')
    assert nearest(F(500000 * M, 866025)) == 577351, 'rounded-component wrong-method control'
    return '\n'.join(rows) + '\n', counts


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    text, counts = fixture()
    if args.check:
        data = text.encode()
        print(f'independent tangent Fraction oracle:{counts};{len(data)} bytes;'
              f'sha256:{hashlib.sha256(data).hexdigest()};rc=0')
    else:
        print(text, end='')


if __name__ == '__main__':
    main()
