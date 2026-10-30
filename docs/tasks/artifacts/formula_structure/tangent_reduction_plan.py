"""Exact tangent folding/conditioning prerequisites;no runtime rounding method is admitted.

Pure standard-library arithmetic/stdout. Universal arguments live in the midpoint/resource fact;
independent Fraction folding and finite controls support transcription,not population completeness.
"""
from collections import namedtuple
from fractions import Fraction as F

M = 1_000_000
A = 90 * M
U = (1 << 128) - 1
Folded = namedtuple('Folded', 'role numerator denominator negative reciprocal')


def fold(numerator, denominator, quarter=A, maximum=U):
    assert abs(numerator) <= maximum and 0 < denominator <= maximum and quarter > 0
    magnitude = abs(numerator)
    pole = quarter * denominator
    phase = magnitude % (2 * pole)
    negative = numerator < 0
    if phase > pole:
        phase = 2 * pole - phase
        negative = not negative
    if phase == pole:
        return Folded('pole', 0, denominator, False, False)
    if phase == 0:
        return Folded('zero', 0, denominator, False, False)
    reciprocal = 2 * phase > pole
    if reciprocal:
        phase = pole - phase
    assert 0 < phase <= magnitude <= maximum and 2 * phase <= pole
    role = 'unit' if 2 * phase == pole else 'quotient'
    return Folded(role, phase, denominator, negative, reciprocal)


def fraction_oracle(numerator, denominator, quarter):
    # Signed Fraction period first,then principal half-period and independent symmetry selection.
    phase = F(numerator, denominator) % (2 * quarter)
    if phase > quarter:
        phase -= 2 * quarter
    if abs(phase) == quarter:
        return 'pole', F(0), False, False
    if phase == 0:
        return 'zero', F(0), False, False
    negative, distance = phase < 0, abs(phase)
    reciprocal = distance > F(quarter, 2)
    if reciprocal:
        distance = quarter - distance
    return ('unit' if distance == F(quarter, 2) else 'quotient', distance, negative, reciprocal)


def checked(numerator, denominator, quarter=A, maximum=U):
    result = fold(numerator, denominator, quarter, maximum)
    actual = result.role, F(result.numerator, result.denominator), result.negative, result.reciprocal
    assert actual == fraction_oracle(numerator, denominator, quarter)
    if result.reciprocal:
        # The folded angle is the original distance to an actual odd-quarter pole. Its integer
        # lower bound implies sin(x)>=2*x/pi>=1/(maximum+1),without approximating pi or sin.
        assert F(result.numerator, result.denominator) >= F(quarter, maximum + 1)
        assert F(result.numerator, quarter * result.denominator) >= F(1, maximum + 1)
    return result


def quotient_bounds(n_low, n_high, d_low, d_high, floor):
    assert 0 <= n_low <= n_high <= 1 and 0 <= d_low <= d_high <= 1 and 0 < floor <= d_high
    # A proved analytic floor may tighten an enclosure;it is not a guessed replacement value.
    d_low = max(d_low, floor)
    return n_low / d_high, n_high / d_low


def round_positive(value):
    return (2 * value.numerator + value.denominator) // (2 * value.denominator)


def main():
    finite = poles = reciprocal = 0
    for maximum in range(1, 25):
        for quarter in (3, 7, 10):
            for numerator in range(-maximum, maximum + 1):
                for denominator in range(1, maximum + 1):
                    result = checked(numerator, denominator, quarter, maximum)
                    finite += result.role != 'pole'
                    poles += result.role == 'pole'
                    reciprocal += result.reciprocal
    # Actual128-bit component edges;exact zeros/units/poles and nearest admitted pole neighbors.
    cases = {(n, d) for n in (0, 1, A // 2, A, 2 * A, U - 1, U)
             for d in (1, 2, U - 1, U)}
    for odd in (1, 3, 17, U // A):
        if odd % 2 == 0:
            odd -= 1
        for d in (1, U // (odd * A)):
            if d:
                for offset in (-1, 0, 1):
                    n = odd * A * d + offset
                    if 0 <= n <= U:
                        cases.add((n, d))
    cases |= {(-n, d) for n, d in cases}
    for numerator, denominator in cases:
        checked(numerator, denominator)
    assert (2 * A * U).bit_length() == 156
    overflow_d = (U + 1) // A
    overflow_n = A * overflow_d - 1
    overflow = checked(overflow_n, overflow_d)
    assert overflow.role == 'quotient' and overflow.reciprocal and overflow.numerator == 1
    assert F(A * M * overflow_d, 4) > U + F(1, 2), 'actual nonpole width witness retained'

    conditioning = 0
    for floor in (F(1, 2), F(1, 4), F(1, U + 1)):
        for n_low in (F(0), F(1, 4), F(1, 2)):
            for d_low in (floor, 2 * floor):
                for dn in (F(0), floor / 16, floor / 3):
                    for dd in (F(0), floor / 16, floor / 3):
                        if max(n_low + dn, d_low + dd) > 1:
                            continue
                        low, high = quotient_bounds(n_low, n_low + dn, d_low, d_low + dd, floor)
                        assert high - low <= dn / floor + dd / floor**2
                        # Independently sample the real rational rectangle's interior/endpoints.
                        for i in range(3):
                            for j in range(3):
                                value = (n_low + F(i, 2) * dn) / (d_low + F(j, 2) * dd)
                                assert low <= value <= high
                        conditioning += 1
    # Exact square comparisons prove nearest ppm at30 degrees without a trig implementation:
    # sin30=1/2,cos30=sqrt(3)/2,tan30=1/sqrt(3). Dividing rounded sin/cos is one ppm wrong.
    cosine, tangent = 866025, 577350
    assert (2 * cosine - 1)**2 < 3 * M**2 < (2 * cosine + 1)**2
    assert 3 * (2 * tangent - 1)**2 < 4 * M**2 < 3 * (2 * tangent + 1)**2
    rounded_quotient = round_positive(F((M // 2) * M, cosine))
    assert rounded_quotient == tangent + 1
    # Keeping only the original input sign also fails:tan120 is negative although120 is positive.
    assert checked(120 * M, 1).negative is True
    assert checked(-120 * M, 1).negative is False
    print(f'tangent reduction: {finite} finite/{poles} poles/{reciprocal} reciprocal finite-model cases;'
          f'{len(cases)} actual full-width/signed witnesses;independent Fraction controls;rc=0')
    print(f'folded components<=128 bits;period<=156 bits;reciprocal sine>=2^-128;'
          f'{conditioning} quotient rectangles;two wrong-composition/sign controls;rc=0')
    print(f'tan30 nearest ppm={tangent};rounded sin/cos quotient={rounded_quotient};'
          'no kernel,rounding-separation,precision,latency or full runtime resource admission')


if __name__ == '__main__':
    main()
