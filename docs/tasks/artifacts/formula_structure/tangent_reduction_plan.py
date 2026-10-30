"""Exact tangent folding/conditioning/enclosure prerequisites;no runtime method is admitted.

Pure standard-library arithmetic/stdout. Universal arguments live in the midpoint/resource fact;
independent Fraction folding and finite controls support transcription,not population completeness.
"""
from collections import namedtuple
from fractions import Fraction as F
from math import factorial

M = 1_000_000
A = 90 * M
U = (1 << 128) - 1
Folded = namedtuple('Folded', 'role numerator denominator negative reciprocal')
Enclosure = namedtuple('Enclosure', 'folded bits pi radians sine cosine ppm')


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


def ceildiv(numerator, denominator):
    return (numerator + denominator - 1) // denominator


def integer_limit(bits):
    return 16 * bits + (2 * bits + 1).bit_length() + 16


def fixed_atan(base, bits):
    assert base in (5, 239) and bits >= 128
    scale, power = 1 << bits, base
    low = high = 0
    for index in range(bits):
        denominator = (2 * index + 1) * power
        floor, remainder = divmod(scale, denominator)
        ceiling = floor + bool(remainder)
        if index % 2:
            low -= ceiling
            high -= floor
        else:
            low += floor
            high += ceiling
        power *= base * base
        assert max(abs(v).bit_length() for v in (denominator, power, low, high, scale)) <= integer_limit(bits)
    tail = ceildiv(scale, (2 * bits + 1) * power)
    assert tail <= 1
    if bits % 2:
        low -= tail
    else:
        high += tail
    assert high - low <= bits + 1
    return low, high


def pi_interval(bits):
    assert bits >= 128
    scale = 1 << bits
    a, b = fixed_atan(5, bits), fixed_atan(239, bits)
    low, high = 16 * a[0] - 4 * b[1], 16 * a[1] - 4 * b[0]
    assert high - low <= 20 * (bits + 1)
    return max(low, 3 * scale), min(high, 4 * scale)


def series_interval(x, cosine, bits):
    scale = 1 << bits
    assert 0 <= x[0] <= x[1] <= scale and x[1] - x[0] <= 5 * bits + 7
    z = x[0]**2 // scale, ceildiv(x[1]**2, scale)
    assert z[1] - z[0] <= 10 * bits + 16
    term = (scale, scale) if cosine else x
    low = high = 0
    for index in range(bits):
        assert 0 <= term[0] <= term[1] <= scale and term[1] - term[0] <= 10 * bits + 20
        if index % 2:
            low -= term[1]
            high -= term[0]
        else:
            low += term[0]
            high += term[1]
        divisor = ((2 * index + 1) * (2 * index + 2) if cosine else
                   (2 * index + 2) * (2 * index + 3)) * scale
        term = term[0] * z[0] // divisor, ceildiv(term[1] * z[1], divisor)
        assert max(abs(v).bit_length() for v in (low, high, divisor, *term, z[1]**2)) <= integer_limit(bits)
    result = max(0, low - 1), min(scale, high + 1)
    assert 0 <= result[0] <= result[1] <= scale
    assert result[1] - result[0] <= 10 * bits**2 + 20 * bits + 2
    return result


def enclose(numerator, denominator, bits):
    """One explicit diagnostic precision,never a language precision limit or native result."""
    assert bits >= 128
    folded = fold(numerator, denominator)
    if folded.role in ('pole', 'zero', 'unit'):
        ppm = None if folded.role == 'pole' else (F(0), F(0)) if folded.role == 'zero' else (F(M), F(M))
        return Enclosure(folded, bits, None, None, None, None, ppm)
    scale = 1 << bits
    pi = pi_interval(bits)
    divisor = 2 * A * denominator
    assert max(divisor.bit_length(), (folded.numerator * pi[1]).bit_length()) <= integer_limit(bits)
    x = folded.numerator * pi[0] // divisor, ceildiv(folded.numerator * pi[1], divisor)
    sine, cosine = series_interval(x, False, bits), series_interval(x, True, bits)
    n, d = (cosine, sine) if folded.reciprocal else (sine, cosine)
    floor = 1 << (bits - 128) if folded.reciprocal else scale // 2
    d = max(d[0], floor), d[1]
    assert 0 < d[0] <= d[1]
    ppm = F(M * n[0], d[1]), F(M * n[1], d[0])
    epsilon = F(10 * bits**2 + 20 * bits + 2, scale)
    limit = F(1, U + 1) if folded.reciprocal else F(1, 2)
    assert ppm[1] - ppm[0] <= M * epsilon * (1 / limit + 1 / limit**2)
    return Enclosure(folded, bits, pi, x, sine, cosine, ppm)


def decision(enclosure):
    """Diagnostic certificate only. An unresolved cell is not a value or numerical refusal."""
    if enclosure.folded.role == 'pole':
        return 'pole', None
    low, high = enclosure.ppm
    if low > U + F(1, 2):
        return 'width', None
    a, b = round_positive(low), round_positive(high)
    if a != b:
        return 'undecided', None
    if a > U:
        return 'width', None
    return 'rounded', -a if enclosure.folded.negative and a else a


def fraction_atan(value, terms):
    partial = sum(((-1)**i * value**(2 * i + 1) / (2 * i + 1) for i in range(terms)), F(0))
    other = partial + (-1)**terms * value**(2 * terms + 1) / (2 * terms + 1)
    return min(partial, other), max(partial, other)


def fraction_pi(terms):
    # A different exact identity and Fraction arithmetic,not fixed-point Machin recurrence.
    a, b = fraction_atan(F(1, 2), terms), fraction_atan(F(1, 3), terms)
    return 4 * (a[0] + b[0]), 4 * (a[1] + b[1])


def fraction_series(value, cosine, terms):
    offset = 0 if cosine else 1
    partial = sum(((-1)**i * value**(2 * i + offset) / factorial(2 * i + offset)
                   for i in range(terms)), F(0))
    other = partial + (-1)**terms * value**(2 * terms + offset) / factorial(2 * terms + offset)
    return min(partial, other), max(partial, other)


def enclosure_controls():
    components = complete = 0
    assert (2 * 1732051 - 1)**2 < 12 * M**2 < (2 * 1732051 + 1)**2, 'independent tan60 nearest cell'
    for bits in (128, 256, 512):
        scale = 1 << bits
        pi = pi_interval(bits)
        oracle = fraction_pi(bits + 32)
        assert F(pi[0], scale) <= oracle[0] < oracle[1] <= F(pi[1], scale)
        terms = 1
        while factorial(2 * terms) <= (1 << (bits + 64)):
            terms += 1
        for value in map(F, (0, F(1, 17), F(1, 4), F(1, 3), F(1, 2), F(3, 4), 1)):
            x = value.numerator * scale // value.denominator, ceildiv(value.numerator * scale, value.denominator)
            for cosine in (False, True):
                interval = series_interval(x, cosine, bits)
                exact = fraction_series(value, cosine, terms)
                assert F(interval[0], scale) <= exact[0] <= exact[1] <= F(interval[1], scale)
                components += 1
        for degrees, expected in ((0, 0), (30, 577350), (45, M), (60, 1732051),
                                  (120, -1732051), (180, 0), (210, 577350),
                                  (-30, -577350), (-120, 1732051), (-180, 0)):
            assert decision(enclose(degrees * M, 1, bits)) == ('rounded', expected)
            complete += 1
        for numerator, denominator in ((1, U), (U, U), (-U, U)):
            assert decision(enclose(numerator, denominator, bits)) == ('rounded', 0)
            complete += 1
        for numerator in (A, -A, 3 * A, -3 * A):
            assert decision(enclose(numerator, 1, bits)) == ('pole', None)
            complete += 1
        d = (U + 1) // A
        assert decision(enclose(A * d - 1, d, bits)) == ('width', None)
        complete += 1
    # A genuine admitted angle near the first half-ppm threshold. Its expected side comes from
    # independent arctangent/alternate-pi intervals,not a result from the tested sin/cos model.
    pi = fraction_pi(288)
    tangent = fraction_atan(F(1, 2 * M), 16)
    lower, upper = 2 * A * tangent[0] / pi[1], 2 * A * tangent[1] / pi[0]
    assert 28 < lower < upper < 29
    candidate = ((lower + upper) / 2).limit_denominator(U // 29)
    assert 0 < candidate.numerator <= U and 0 < candidate.denominator <= U
    assert candidate < lower or candidate > upper, 'independent oracle must decide original threshold side'
    expected = int(candidate > upper)
    observed = []
    for bits in (128, 256, 512):
        verdict = decision(enclose(candidate.numerator, candidate.denominator, bits))
        assert verdict in (('undecided', None), ('rounded', expected))
        observed.append(verdict)
    assert observed[0] == ('undecided', None) and observed[-1] == ('rounded', expected)
    print(f'tangent enclosures:3 pi/{components} independent Fraction component/{complete} coupled endpoint,width,pole controls;rc=0')
    print(f'actual near-midpoint input={candidate.numerator}/{candidate.denominator} microdegrees;'
          f'128/256/512-bit certificates={observed};independent threshold side={expected};rc=0')
    print('per-precision width<=(10B^2+20B+2)*2^-B;convergence is not a practical uniform precision/resource admission')


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
    enclosure_controls()


if __name__ == '__main__':
    main()
