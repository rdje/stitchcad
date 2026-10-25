"""Exact checks accompanying the trig midpoint proof; no production trig accuracy claim."""
from fractions import Fraction
from math import comb

RATIO_SCALE = 1_000_000
DEGREE_SCALE = 1_000_000
MAX_COMPONENT = (1 << 128) - 1


def doubled_cosine_polynomial(n):
    """C_n(2*cos(t))=2*cos(n*t), integer coefficients in ascending order."""
    previous, current = [2], [0, 1]
    for _ in range(1, n):
        shifted = [0] + current
        for i, coefficient in enumerate(previous):
            shifted[i] -= coefficient
        previous, current = current, shifted
    return current


def evaluate(coefficients, value):
    total = Fraction()
    for coefficient in reversed(coefficients):
        total = total * value + coefficient
    return total


def binomial_cosine_multiple(n, doubled_cosine):
    # Independent expansion of (cos(t)+i*sin(t))^n; only even sine powers survive.
    cosine = doubled_cosine / 2
    return 2 * sum(((-1)**j * comb(n, 2*j) * cosine**(n-2*j)
                    * (1-cosine*cosine)**j for j in range(n//2+1)), Fraction())


def main():
    # Completeness of these rational-value sets is the independently cited theorem,
    # not established by these finite arithmetic checks.
    rational_sines = tuple(map(Fraction, (-1, 0, 1))) + (Fraction(-1, 2), Fraction(1, 2))
    rational_tangents = tuple(map(Fraction, (-1, 0, 1)))
    assert RATIO_SCALE % 2 == 0
    for value in (*rational_sines, *rational_tangents):
        assert (RATIO_SCALE * value).denominator == 1
    for degrees in (-180, -135, -90, -45, 0, 45, 90, 135, 180):
        assert (Fraction(degrees) * DEGREE_SCALE).denominator == 1
    # Falsify a scale-independent claim: at scale1, sin(30 degrees)=1/2 is a half tie.
    assert (Fraction(1, 2) * 1).denominator == 2
    assert (Fraction(1, 2) * RATIO_SCALE).denominator == 1

    identities = 0
    for n in range(2, 25):
        polynomial = doubled_cosine_polynomial(n)
        assert len(polynomial) == n+1 and polynomial[-1] == 1
        for target in (Fraction(-2), Fraction(-1), Fraction(0), Fraction(1),
                       Fraction(2), Fraction(1, 3), Fraction(-5, 7)):
            assert evaluate(polynomial, target) == binomial_cosine_multiple(n, target)
            identities += 1
    # Known rational-angle roots verify the convention used in the written proof.
    for denominator, doubled_cosine in ((2, Fraction(0)), (3, Fraction(1)),
                                        (1, Fraction(-2))):
        assert evaluate(doubled_cosine_polynomial(2*denominator), doubled_cosine) == 2

    q_max = 180_000_000 * MAX_COMPONENT
    n_max = 4 * q_max  # Covers the pi/2 shift for sine as well as cosine.
    assert q_max < 1 << 156 and n_max < 1 << 158
    assert RATIO_SCALE < 1 << 20
    sufficient_bits_ceiling = 20*n_max + 318
    assert sufficient_bits_ceiling < 1 << 163
    print('midpoint arithmetic:8 theorem-value entries and9 branch/axis angles map to integers; scale1 counterexample verified')
    print('Chebyshev recurrence/binomial identities:%d exact;3 known roots; no transcendental sampling' % identities)
    print('forward sin/cos conservative sufficient-precision ceiling:%d bits (<2^163)' % sufficient_bits_ceiling)
    print('scope:qualitative midpoint/termination proof only; usable resources/interval implementation remain owned')


if __name__ == '__main__':
    main()
