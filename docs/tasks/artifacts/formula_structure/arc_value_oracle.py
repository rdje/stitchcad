"""Pure stdout arc fixtures from an independent pi identity and exact Fraction arithmetic.

The production certificate uses Machin's 1/5,1/239 identity. This oracle uses 1/2,1/3
alternating fractions and checks rounding at both enclosure endpoints, including hard midpoints.
"""
from fractions import Fraction


def pi_bounds():
    bounds = []
    for base in (2, 3):
        total = sum((Fraction((-1)**i, (2*i+1)*base**(2*i+1)) for i in range(512)), Fraction())
        next_sum = total + Fraction(1, 1025*base**1025)
        bounds.append((total, next_sum))  # 512 terms: next signed term is positive.
    return 4*(bounds[0][0]+bounds[1][0]), 4*(bounds[0][1]+bounds[1][1])


def neighbors(value, bound):
    n, d = value.numerator, value.denominator
    p0, q0, p1, q1 = 0, 1, 1, 0
    while True:
        a = n//d
        q2 = q0+a*q1
        if q2 > bound:
            break
        p0, q0, p1, q1 = p1, q1, p0+a*p1, q2
        n, d = d, n-a*d
        assert d
    k = (bound-q0)//q1
    return sorted((Fraction(p0+k*p1, q0+k*q1), Fraction(p1, q1)))


def round_half_away(value):
    quotient, remainder = divmod(abs(value.numerator), value.denominator)
    magnitude = quotient + (2*remainder >= value.denominator)
    return -magnitude if value < 0 else magnitude


def fixture_rows():
    maximum = (1 << 128)-1
    low_pi, high_pi = pi_bounds()
    cases = [(Fraction(angle), Fraction(radius)) for angle, radius in [
        (0, 0), (0, 10**9), (maximum, 0), (90_000_000, 1), (180_000_000, 1),
        (360_000_000, 1), (720_000_000, 1), (-720_000_000, 1),
        (720_000_000, -1), (-720_000_000, -1), (360_000_000, 10**9),
        (-360_000_000, 10**9), (maximum, 10**9), (-maximum, 10**9)]]
    cases.extend(((Fraction(1, 3), Fraction(10**9)), (Fraction(1, 3), Fraction(1, 7)),
                  (Fraction(maximum, maximum-1), Fraction(maximum-2, maximum-1))))
    # Neighbor fractions straddle exact half-output thresholds but each component fits u128.
    for midpoint, denominator_bound in [(Fraction(1, 2), maximum), (Fraction(2*10**9+1, 2), 1 << 95)]:
        threshold_low = midpoint*180_000_000/(high_pi*10**9)
        threshold_high = midpoint*180_000_000/(low_pi*10**9)
        lower, upper = neighbors(threshold_low, denominator_bound)
        assert lower < threshold_low < threshold_high < upper
        cases.extend(((lower, Fraction(10**9)), (upper, Fraction(10**9))))
        if midpoint == Fraction(1, 2):
            cases.extend(((-lower, Fraction(10**9)), (-upper, Fraction(10**9))))
    yield "# angle numerator\tangle denominator\tradius numerator\tradius denominator\texpected"
    for angle, radius in cases:
        assert max(abs(angle.numerator).bit_length(), angle.denominator.bit_length(),
                   abs(radius.numerator).bit_length(), radius.denominator.bit_length()) <= 128
        assert abs(radius) <= 10**9
        coefficient = angle*radius/180_000_000
        low, high = sorted((coefficient*low_pi, coefficient*high_pi))
        integer = round_half_away(low)
        assert integer == round_half_away(high), "independent enclosure cannot decide this fixture"
        bits = abs(integer).bit_length()
        expected = ("width:"+str(bits) if bits > 128 else
                    ("domain:" if abs(integer) > 10**9 else "integer:")+str(integer))
        yield "\t".join(map(str, (angle.numerator, angle.denominator, radius.numerator, radius.denominator, expected)))


if __name__ == "__main__":
    print("\n".join(fixture_rows()))
