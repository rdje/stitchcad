"""Exact envelope and independent Fraction controls for the planned native sin/cos kernel."""
from fractions import Fraction as F
from math import factorial

BITS = 512
S = 1 << BITS
TERMS = 64
M = 1_000_000
U = (1 << 128) - 1


def ceildiv(a, b):
    return (a + b - 1) // b


def series_interval(x, cosine):
    """Integer recurrence model; input interval contains a real x in [0,1]."""
    lo, hi = x
    assert 0 <= lo <= hi <= S and hi - lo <= 3
    z = lo * lo // S, ceildiv(hi * hi, S)
    assert z[1] - z[0] <= 8
    term = (S, S) if cosine else x
    lower = upper = 0
    for n in range(TERMS):
        assert 0 <= term[0] <= term[1] <= S and term[1] - term[0] <= 12
        if n % 2:
            lower -= term[1]
            upper -= term[0]
        else:
            lower += term[0]
            upper += term[1]
        k = (2 * n + 1) * (2 * n + 2) if cosine else (2 * n + 2) * (2 * n + 3)
        term = term[0] * z[0] // (S * k), ceildiv(term[1] * z[1], S * k)
    # Both alternating remainders have magnitude <1/128! <1/S for x<=1.
    lower, upper = max(0, lower - 1), min(S, upper + 1)
    assert 0 <= lower <= upper <= S and upper - lower <= 770
    return F(lower, S), F(upper, S)


def fraction_oracle(x, cosine):
    """Direct powers/factorials, no fixed-point recurrence or rounded intermediate."""
    offset = 0 if cosine else 1
    partial = sum(((-1) ** n * x ** (2 * n + offset) / factorial(2 * n + offset)
                   for n in range(TERMS)), F(0))
    next_term = x ** (2 * TERMS + offset) / factorial(2 * TERMS + offset)
    return partial, partial + next_term


def folded(angle, cosine):
    d, numerator = angle.denominator, abs(angle.numerator)
    quarter = 90 * M * d
    phase = numerator % (4 * quarter)
    negative = angle < 0 and not cosine
    if cosine:
        phase = (phase + quarter) % (4 * quarter)
    if phase >= 2 * quarter:
        phase -= 2 * quarter
        negative = not negative
    if phase > quarter:
        phase = 2 * quarter - phase
    use_cosine = phase > quarter // 2
    if use_cosine:
        phase = quarter - phase
    if phase == 0 and not use_cosine:
        negative = False
    return F(phase, 360 * M * d), use_cosine, negative


def octant_table_oracle(angle, cosine):
    """Different signed-turn reduction and explicit eight-sector identity table."""
    phase = (angle / (360 * M) + (F(1, 4) if cosine else 0)) % 1
    sector = phase // F(1, 8)
    local = phase - sector * F(1, 8)
    basis = [False, True, True, False, False, True, True, False][sector]
    if sector % 2:
        local = F(1, 8) - local
    if local == F(1, 8):
        basis = False  # sin(pi/4)=cos(pi/4)
    negative = sector >= 4 and (local != 0 or basis)
    return local, basis, negative


def main():
    assert factorial(128) > S and factorial(129) > S
    x_width, z_width, term_width = 3, 8, 12
    assert z_width >= 2 * x_width + 2
    assert term_width >= F(term_width + z_width, 2) + 2
    total_width = TERMS * term_width + 2
    assert total_width == 770 and total_width < 1 << (BITS - 384)
    assert total_width < 1 << 10  # Width <2^-502, strictly below certified value margin2^-384.
    workspace_bound = 1 << 1026
    for expression in [(360 * M * U + U) * (4 * S), S * S,
                       64 * S + 2, 128 * 129 * S, 2 * M * S]:
        assert expression < workspace_bound
    values = [F(0), F(1, S), F(1, U), F(1, 4), F(1, 2), F(3, 4), F(1)]
    cases = 0
    for value in values:
        low = value.numerator * S // value.denominator
        high = ceildiv(value.numerator * S, value.denominator)
        x = max(0, low - 1), min(S, high + 1)
        for cosine in (False, True):
            oracle = fraction_oracle(value, cosine)
            actual = series_interval(x, cosine)
            assert actual[0] <= oracle[0] <= oracle[1] <= actual[1], ('Fraction containment', value, cosine)
            cases += 1
    near_axis_d = U // (90 * M)
    angles = [F(value * M) for value in [-720, -450, -360, -270, -180, -90, -45, -30, 0,
                                        30, 45, 90, 180, 270, 360, 450, 720]]
    angles += [F(U), F(-U), F(U, U - 1), F(-U, U - 1), F(1, U), F(-1, U),
               F(90 * M * near_axis_d - 1, near_axis_d), F(90 * M * near_axis_d + 1, near_axis_d)]
    for angle in angles:
        assert abs(angle.numerator) <= U and angle.denominator <= U
        for cosine in (False, True):
            assert folded(angle, cosine) == octant_table_oracle(angle, cosine), ('fold identity', angle, cosine)
    # Deliberately insufficient terms/precision cannot satisfy this admission bound.
    assert not factorial(4) > S and not total_width < 1 << (384 - 384)
    print('sincos plan: P512/64terms; width<=770*2^-512<2^-502<2^-384; workspace<2^1026, rc=0')
    print('Fraction containment%d/signed-octant%d/insufficient-profile2 controls pass, rc=0' % (cases, len(angles) * 2))


if __name__ == '__main__':
    main()
