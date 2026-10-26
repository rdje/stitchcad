"""Exact tangent pole/range/coverage profile; no runtime rounding plan is admitted here.

Pure stdout, independent finite-model pole enumeration, no certificate input or native imports.
The accompanying record supplies the universal integer/analytic derivation; enumeration supports
transcription, not an inference from the small domain to128-bit inputs.
"""
from fractions import Fraction as F

M = 1_000_000
A = 90 * M
U = (1 << 128) - 1


def pole_distance(n, d, quarter):
    n = abs(n)
    p = (2 * (n // (2 * quarter * d)) + 1) * quarter
    return F(abs(n - p * d), d)


def enumerate_distance(n, d, quarter, maximum):
    # Direct independent comparison with every possible nearest odd-quarter pole, both signs.
    angle = F(n, d)
    return min(abs(angle - odd * quarter)
               for odd in range(-2 * maximum - 1, 2 * maximum + 2, 2))


def main():
    rows = poles = 0
    for maximum in range(1, 17):
        for quarter in (1, 3, 7):
            for n in range(-maximum, maximum + 1):
                for d in range(1, maximum + 1):
                    distance = pole_distance(n, d, quarter)
                    assert distance == enumerate_distance(n, d, quarter, maximum)
                    if not distance:
                        poles += 1
                        continue
                    assert distance >= F(quarter, maximum + 1)
                    rows += 1
    # The +1 in the universal bound cannot simply be dropped.
    assert pole_distance(5, 2, 3) < F(3, 5)
    # A seemingly tiny1/U gap beside90 degrees would violate the numerator limit.
    assert A * U - 1 > U
    d = (U + 1) // A
    n = A * d - 1
    assert 0 < n <= U and 0 < d <= U
    delta = pole_distance(n, d, A)
    assert delta == F(1, d) and delta >= F(A, U + 1)
    # t=pi/(2*A*d)<2/(A*d)<1/2; sin(t)<=t and cos(t)>=1/2.
    # Hence M*cot(t)>A*M*d/4. This exceeds the public width boundary U+1/2.
    assert F(2, A * d) < F(1, 2)
    lower_ppm = F(A * M * d, 4)
    assert lower_ppm > U + F(1, 2)
    # tan(t)>=t and pi>3 give M*abs(tan(angle))<2*M*(U+1)/3.
    # Add one quantum conservatively to bound the rounded magnitude as an integer.
    upper_ppm = (2 * M * (U + 1) + 2) // 3 + 1
    assert upper_ppm.bit_length() == 148 and upper_ppm > lower_ppm
    positive_thresholds = U + 1  # k=0,...,U, including U+1/2 for the width decision.
    assert positive_thresholds == 1 << 128
    assert positive_thresholds > 45_000_000 > 1_000_000
    assert (F(1, 3) - F(1, 2)).denominator != 1  # admitted input off the inverse half-µdeg grid
    print(f'tangent profile: nonpole distance>=90M/(U+1); rounded magnitude<=148 bits; relevant positive thresholds={positive_thresholds}, rc=0')
    print(f'independent small-domain distances/bounds={rows}; exact poles={poles}; negative controls2; full-width overflow witness n={n},d={d}, rc=0')
    print('NO runtime admission: existing1M sine/45M inverse certificates do not cover tangent rounding; practical .d.2 remains P1')


if __name__ == '__main__':
    main()
