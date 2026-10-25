"""Pure stdout fixtures from Python's independent arbitrary-precision Fraction arithmetic.

Regenerate from the repository root with python3 -I -B and redirect stdout to the tracked
crates/sc-core/tests/fixtures/formula_rationals.tsv. No project arithmetic is imported.
"""
from fractions import Fraction


def fixture_rows():
    maximum = (1 << 128) - 1
    cases = [
        ("add", 2, 3, 1, 2),
        ("sub", 1, 2, 2, 3),
        ("add", -2, 3, -1, 2),
        ("mul", -2, 3, -9, 4),
        ("div", -2, 3, 4, 9),
        ("add", maximum, maximum - 1, maximum, maximum - 1),
        ("add", maximum, maximum - 1, -maximum, maximum - 1),
        ("sub", maximum, maximum - 1, 1, maximum - 1),
        ("add", maximum - 1, maximum, 1, maximum),
        ("mul", maximum, 2, -2, maximum),
        ("div", maximum, maximum - 1, maximum, maximum - 1),
        ("add", maximum, 1, 1, 1),
        ("sub", -maximum, 1, 1, 1),
        ("add", maximum, maximum - 1, maximum - 1, maximum - 2),
        ("sub", 1, maximum, 1, maximum - 1),
        ("mul", maximum, 1, maximum, 1),
        ("mul", 1, maximum, 1, maximum),
        ("div", maximum, 1, 1, 2),
        ("div", 1, maximum, 2, 1),
        ("mul", (1 << 127) + 1, 1, 2, 1),
        ("mul", (1 << 64) - 1, 1, (1 << 64) + 1, 1),
        ("div", -maximum, 1, -1, 1),
    ]
    for operation in ("add", "sub", "mul", "div"):
        cases.extend(((operation, 0, maximum, -maximum, 1),
                      (operation, maximum, 1, 0, maximum),
                      (operation, 0, 1, 0, 1)))
    yield "# operation\tleft numerator\tleft denominator\tright numerator\tright denominator\texpected"
    for operation, left_n, left_d, right_n, right_d in cases:
        left, right = Fraction(left_n, left_d), Fraction(right_n, right_d)
        if operation == "div" and right == 0:
            expected = "ZeroDivisor"
        else:
            result = {"add": Fraction.__add__, "sub": Fraction.__sub__,
                      "mul": Fraction.__mul__, "div": Fraction.__truediv__}[operation](left, right)
            numerator_bits = abs(result.numerator).bit_length()
            denominator_bits = result.denominator.bit_length()
            if numerator_bits > 128:
                expected = "Numerator:" + str(numerator_bits)
            elif denominator_bits > 128:
                expected = "Denominator:" + str(denominator_bits)
            else:
                expected = "value:%d/%d" % (result.numerator, result.denominator)
        yield "\t".join(map(str, (operation, left_n, left_d, right_n, right_d, expected)))


if __name__ == "__main__":
    print("\n".join(fixture_rows()))
