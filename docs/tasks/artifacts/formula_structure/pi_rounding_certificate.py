"""Pure stdout certificate for bounded arc_length rounding; no runtime/filesystem effects.

Two exact pi identities use different arithmetic to certify the same binary enclosure.
Adjacent bounded-denominator fractions certify that no admitted output midpoint is inside it.
"""
from fractions import Fraction


def atan_bounds(base, terms):
    partial = sum((Fraction((-1)**i, (2*i + 1) * base**(2*i + 1))
                   for i in range(terms)), Fraction())
    other = partial + Fraction((-1)**terms, (2*terms + 1) * base**(2*terms + 1))
    return min(partial, other), max(partial, other)


def fixed_atan_bounds(base, terms, bits):
    """Independent directed integer sums, including the signed alternating remainder."""
    scale = 1 << bits
    lower = upper = 0
    for index in range(terms):
        denominator = (2*index + 1) * pow(base, 2*index + 1)
        floor, remainder = divmod(scale, denominator)
        ceiling = floor + bool(remainder)
        if index % 2:
            lower -= ceiling
            upper -= floor
        else:
            lower += floor
            upper += ceiling
    tail_denominator = (2*terms + 1) * pow(base, 2*terms + 1)
    tail_ceiling = (scale + tail_denominator - 1) // tail_denominator
    if terms % 2:
        lower -= tail_ceiling
    else:
        upper += tail_ceiling
    return lower, upper


def bounded_cell(value, bound):
    """Return the two adjacent fractions around this input with denominators at most bound."""
    numerator, denominator = value.numerator, value.denominator
    p0, q0, p1, q1 = 0, 1, 1, 0
    for steps in range(400):
        quotient = numerator // denominator
        q2 = q0 + quotient*q1
        if q2 > bound:
            break
        p0, q0, p1, q1 = p1, q1, p0 + quotient*p1, q2
        numerator, denominator = denominator, numerator - quotient*denominator
        assert denominator, "input unexpectedly has an admitted exact denominator"
    else:
        raise AssertionError("continued-fraction certificate did not terminate within its envelope")
    multiplier = (bound - q0) // q1
    lower, upper = sorted((Fraction(p0 + multiplier*p1, q0 + multiplier*q1), Fraction(p1, q1)))
    return lower, upper, steps


def certificate():
    bits, terms, independent_terms, guard = 640, 160, 512, 32
    scale = 1 << bits
    # tan(4 atan(1/5) - atan(1/239)) = 1; both branches are inside (0, pi/2).
    tangent_twice = 2*Fraction(1, 5) / (1 - Fraction(1, 5)**2)
    tangent_four = 2*tangent_twice / (1 - tangent_twice**2)
    assert (tangent_four - Fraction(1, 239)) / (1 + tangent_four*Fraction(1, 239)) == 1
    assert (Fraction(1, 2) + Fraction(1, 3)) / (1 - Fraction(1, 6)) == 1
    a_low, a_high = atan_bounds(5, terms)
    b_low, b_high = atan_bounds(239, terms)
    assert 0 < 4*a_low - b_high < 4*a_high - b_low < 1
    pi_low, pi_high = 16*a_low - 4*b_high, 16*a_high - 4*b_low
    floor_low = (pi_low*scale).__floor__()
    assert floor_low == (pi_high*scale).__floor__(), "Machin enclosure crosses a binary boundary"
    # pi = 4*(atan(1/2) + atan(1/3)), evaluated with directed integer sums instead of Fractions.
    c_low, c_high = fixed_atan_bounds(2, independent_terms, bits + guard)
    d_low, d_high = fixed_atan_bounds(3, independent_terms, bits + guard)
    assert 4*(c_low + d_low) >> guard == floor_low
    assert 4*(c_high + d_high) >> guard == floor_low
    lower, upper = Fraction(floor_low, scale), Fraction(floor_low + 1, scale)
    bound = 1 << 257  # A is a product of two u128 magnitudes, so every midpoint denominator <=2A<2^257.
    cell_low, cell_high, steps = bounded_cell(lower, bound)
    assert cell_low < lower < upper < cell_high
    assert cell_high.numerator*cell_low.denominator - cell_low.numerator*cell_high.denominator == 1
    assert max(cell_low.denominator, cell_high.denominator) <= bound
    assert cell_low.denominator + cell_high.denominator > bound
    # Public standard-library closest-fraction checks independently identify an endpoint of this cell.
    assert lower.limit_denominator(bound) in (cell_low, cell_high)
    assert upper.limit_denominator(bound) in (cell_low, cell_high)
    return (
        ("fraction_bits", bits), ("coefficient_numerator_bits", 256),
        ("half_target_denominator_bits", 257), ("machin_terms", terms),
        ("independent_terms", independent_terms), ("independent_guard_bits", guard),
        ("pi_lower_hex", format(floor_low, "x")), ("pi_upper_hex", format(floor_low + 1, "x")),
        ("cell_lower_numerator", cell_low.numerator), ("cell_lower_denominator", cell_low.denominator),
        ("cell_upper_numerator", cell_high.numerator), ("cell_upper_denominator", cell_high.denominator),
        ("continued_fraction_steps", steps),
    )


if __name__ == "__main__":
    print("# Exact pi enclosure and bounded midpoint exclusion certificate")
    for name, value in certificate():
        print(name + "\t" + str(value))
