"""Pure stdout root fixtures: standard-library integer root plus exact Fraction midpoints.

No production arithmetic is imported. These expected rows precede the sqrt/hypot implementation.
Regenerate with python3 -I -B into crates/sc-core/tests/fixtures/formula_roots.tsv.
"""
from fractions import Fraction
from math import isqrt


def rounded_root(value):
    lower = isqrt(value.numerator // value.denominator)
    return lower + (value >= Fraction((2 * lower + 1) ** 2, 4))


def fixture_rows():
    maximum = (1 << 128) - 1
    cases = [("sqrt", "area", Fraction(n, d), Fraction(0))
             for n, d in [(0, 1), (1, 1), (2, 1), (9, 4), (25, 4), (16, 1), (-1, 1)]]
    for delta in [-1, 0, 1]:
        midpoint = Fraction(9, 4) + Fraction(delta, 4 * ((1 << 100) + 1))
        cases.extend((("sqrt", "area", midpoint, Fraction(0)),
                      ("sqrt", "ratio", midpoint / 1_000_000, Fraction(0))))
    cases.extend((("sqrt", "ratio", Fraction(maximum), Fraction(0)),
                  ("sqrt", "ratio", Fraction(-1), Fraction(0))))
    cases.extend(("hypot", "length", Fraction(left_n, left_d), Fraction(right_n, right_d))
                 for left_n, left_d, right_n, right_d in [(0, 1, 0, 1), (3, 1, 4, 1),
                     (-3, 1, 4, 1), (1, 2, 0, 1), (-1, 2, 0, 1), (1, 3, 0, 1),
                     (10**9, 1, 1, 1), (10**9, 1, 10**9, 1),
                     (1, maximum, 1, maximum - 1),
                     (maximum - 1, maximum, maximum - 2, maximum - 1)])
    yield "# operation\tkind\tleft numerator\tleft denominator\tright numerator\tright denominator\tresult kind\texpected"
    for operation, kind, left, right in cases:
        result_kind = "ratio" if kind == "ratio" else "length"
        if operation == "sqrt" and left < 0:
            expected = "negative"
        else:
            radicand = left**2 + right**2 if operation == "hypot" else left * (1_000_000 if kind == "ratio" else 1)
            integer = rounded_root(radicand)
            expected = ("domain:" if result_kind == "length" and integer > 10**9 else "integer:") + str(integer)
        yield "\t".join(map(str, (operation, kind, left.numerator, left.denominator,
                                 right.numerator, right.denominator, result_kind, expected)))


if __name__ == "__main__":
    print("\n".join(fixture_rows()))
