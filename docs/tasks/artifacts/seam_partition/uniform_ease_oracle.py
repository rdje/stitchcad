"""Independent exact Fraction oracle for original-source uniform cumulative quantization.

Pure stdout; no project arithmetic imports. Native sewing tests consume the frozen TSV.
"""
from fractions import Fraction


def quantize(value):
    whole, remainder = divmod(abs(value.numerator), value.denominator)
    result = whole + (2 * remainder >= value.denominator)
    return -result if value < 0 else result


print('# parent_um\tprogress_n\tprogress_d\tfirst_um\tsecond_um')
maximum = (1 << 63) - 1
for parent in (-1_000_000_000, -5, -1, 0, 1, 5, 1_000_000_000):
    for progress in (Fraction(1, 3), Fraction(1, 2), Fraction(2, 3),
                     Fraction(1, maximum), Fraction(maximum - 1, maximum)):
        first = quantize(parent * progress)
        second = parent - first
        assert first + second == parent
        assert first == -quantize(-parent * progress)
        print(f'{parent}\t{progress.numerator}\t{progress.denominator}\t{first}\t{second}')
