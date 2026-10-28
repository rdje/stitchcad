"""Independent Fraction fixtures for complete source-frame progress.

Pure stdout; regenerate crates/sc-core/tests/fixtures/range_progress.tsv from the repository
root with python3 -I -B. No project arithmetic is imported.
"""
from fractions import Fraction


def fixture_rows():
    maximum = (1 << 63) - 1
    d = maximum - 1
    cases = [
        ('start', 0, 1, 0, 1, 1, 1),
        ('end', 0, 1, 1, 1, 1, 1),
        ('half', 0, 1, 1, 2, 1, 1),
        ('third', 0, 1, 1, 3, 1, 1),
        ('partial-a', 1, 7, 2, 3, 6, 7),
        ('partial-b', 1, 5, 3, 7, 4, 5),
        ('partial-start', 1, 7, 1, 7, 6, 7),
        ('partial-end', 1, 7, 6, 7, 6, 7),
        ('outside-low', 1, 7, 0, 1, 6, 7),
        ('outside-high', 1, 7, 1, 1, 6, 7),
        ('tiny-global', 0, 1, 1, maximum, 1, 1),
        ('upper-global', 0, 1, maximum - 1, maximum, 1, 1),
        ('wide-shared-frame', maximum - 4, maximum, maximum - 2, maximum, maximum - 1, maximum),
        ('cancelled-small-frame', 1, d, 1, d - 2, 1, d - 4),
        ('cancelled-near-one', d - 5, d - 4, d - 3, d - 2, d - 1, d),
        ('final-denominator-overflow', 1, maximum, 1, maximum - 1, 2, maximum - 1),
        ('final-two-component-overflow', 1, maximum, 1, 3, 1, 2),
    ]
    yield '# case\tfrom numerator\tfrom denominator\tat numerator\tat denominator\tto numerator\tto denominator\texpected'
    for name, lower_n, lower_d, at_n, at_d, upper_n, upper_d in cases:
        lower = Fraction(lower_n, lower_d)
        at = Fraction(at_n, at_d)
        upper = Fraction(upper_n, upper_d)
        assert 0 <= lower < upper <= 1 and 0 <= at <= 1
        if not lower <= at <= upper:
            expected = 'Outside'
        else:
            progress = (at - lower) / (upper - lower)
            if progress.numerator > maximum or progress.denominator > maximum:
                expected = 'Overflow'
            else:
                expected = f'value:{progress.numerator}/{progress.denominator}'
        yield '\t'.join(map(str, (name, lower_n, lower_d, at_n, at_d, upper_n, upper_d, expected)))


if __name__ == '__main__':
    print('\n'.join(fixture_rows()))
