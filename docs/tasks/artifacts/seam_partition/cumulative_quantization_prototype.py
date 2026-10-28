"""Independent planning prototype; native cumulative-allocation integration remains pending.

Run from the repository root with python3 -I -B. This imports no project arithmetic and writes
only stdout. It demonstrates why retained original source/domains precede nested split effects.
"""
from fractions import Fraction


def quantize(value):
    magnitude, remainder = divmod(abs(value.numerator), value.denominator)
    rounded = magnitude + (2 * remainder >= value.denominator)
    return -rounded if value < 0 else rounded


for original in (1, -1):
    first = quantize(Fraction(original, 3))
    middle = quantize(Fraction(original - first, 2))
    left_first = (first, middle, original - first - middle)
    prefix = quantize(Fraction(2 * original, 3))
    first = quantize(Fraction(prefix, 2))
    right_first = (first, prefix - first, original - prefix)
    boundaries = [quantize(original * point) for point in
                  (Fraction(0), Fraction(1, 3), Fraction(2, 3), Fraction(1))]
    cumulative = tuple(right - left for left, right in zip(boundaries, boundaries[1:]))
    assert left_first != right_first
    assert sum(left_first) == sum(right_first) == sum(cumulative) == original
    print(f'original={original} um; child-left-first={left_first}; '
          f'child-right-first={right_first}; original-cumulative={cumulative}')
print('prototype only; native cumulative allocation remains G1-SLICE.6.5.3.2.3.2.2')
