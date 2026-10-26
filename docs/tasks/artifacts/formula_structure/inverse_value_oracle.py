"""Pure stdout inverse fixtures: independent addition reduction, no native roots or pi fixture.

768-bit enclosures and512 atan terms; pi comes from the exact rational atan(1/2)+atan(1/3)
identity. Separate directed sin/cos series construct rational neighbors of output midpoints.
"""
from fractions import Fraction as F
from math import factorial

M = 1_000_000
U = (1 << 128) - 1
S = 1 << 768


def ceildiv(a, b):
    return (a + b - 1) // b


def series(value):
    assert 0 <= value <= F(1, 2)
    lo = value.numerator * S // value.denominator
    hi = ceildiv(value.numerator * S, value.denominator)
    square = lo * lo // S, ceildiv(hi * hi, S)
    power = lo, hi
    low = high = 0
    for n in range(512):
        a, b = power[0] // (2 * n + 1), ceildiv(power[1], 2 * n + 1)
        low, high = (low - b, high - a) if n % 2 else (low + a, high + b)
        power = power[0] * square[0] // S, ceildiv(power[1] * square[1], S)
    assert S < 2 ** 1025 * 1025  # first omitted term <1/S
    return F(max(0, low - 1), S), F(high + 1, S)


def pi_bounds():
    bounds = []
    for base in (2, 3):
        total = sum((F((-1) ** n, (2 * n + 1) * base ** (2 * n + 1))
                     for n in range(512)), F(0))
        bounds.append((total, total + F(1, 1025 * base ** 1025)))
    return 4 * (bounds[0][0] + bounds[1][0]), 4 * (bounds[0][1] + bounds[1][1])


def atan(value, pi, anchor):
    if not value:
        return F(0), F(0)
    if value < 0:
        lo, hi = atan(-value, pi, anchor)
        return -hi, -lo
    if value > 1:
        lo, hi = atan(1 / value, pi, anchor)
        return F(1, 4) - hi, F(1, 4) - lo
    residual = (value - F(1, 2)) / (1 + value / 2)
    lo, hi = series(abs(residual))
    radians = ((anchor[0] + lo, anchor[1] + hi) if residual >= 0
               else (anchor[0] - hi, anchor[1] - lo))
    return radians[0] / (2 * pi[1]), radians[1] / (2 * pi[0])


def atan2(y, x, pi, anchor):
    if not x:
        if not y:
            raise ValueError('zero vector')
        axis = F(1, 4) if y > 0 else -F(1, 4)
        return axis, axis
    lo, hi = atan(y / x, pi, anchor)
    offset = (F(1, 2) if y >= 0 else -F(1, 2)) if x < 0 else 0
    return lo + offset, hi + offset


def sincos(angle, cosine, pi):
    xlo, xhi = angle * pi[0] / (180 * M), angle * pi[1] / (180 * M)
    lo, hi = xlo.numerator * S // xlo.denominator, ceildiv(xhi.numerator * S, xhi.denominator)
    assert 0 <= lo <= hi < S
    low = high = 0
    for n in range(128):
        power = 2 * n + (0 if cosine else 1)
        denominator = S ** power * factorial(power)
        a, b = lo ** power * S // denominator, ceildiv(hi ** power * S, denominator)
        low, high = (low - b, high - a) if n % 2 else (low + a, high + b)
    assert S < factorial(256)
    return F(low - 1, S), F(high + 1, S)


def neighbors(value, maximum):
    n, d = value.numerator, value.denominator
    p0, q0, p1, q1 = 0, 1, 1, 0
    while d:
        a = n // d
        if q0 + a * q1 > maximum:
            break
        p0, q0, p1, q1 = p1, q1, p0 + a * p1, q0 + a * q1
        n, d = d, n - a * d
    assert d
    k = (maximum - q0) // q1
    return sorted((F(p0 + k * p1, q0 + k * q1), F(p1, q1)))


def midpoint_neighbors(pi):
    for angle in (F(1, 2), F(60 * M + 1, 2), F(90 * M - 1, 2)):
        sine, cosine = sincos(angle, False, pi), sincos(angle, True, pi)
        lo, hi = M * sine[0] / cosine[1], M * sine[1] / cosine[0]
        left, right = neighbors(lo, U // M)
        assert left < lo <= hi < right
        assert right - left < F(1, 1 << 180)
        yield left
        yield right


def rounded(value):
    value *= 360 * M
    q, r = divmod(abs(value.numerator), value.denominator)
    magnitude = q + (2 * r >= value.denominator)
    return -magnitude if value < 0 else magnitude


def rows():
    pi, anchor = pi_bounds(), series(F(1, 2))
    hard = list(midpoint_neighbors(pi))
    values = [F(v) for v in (0, 1, -1, M, -M, M // 2, -M // 2, 2 * M, -2 * M, U, -U)]
    values += [F(1, U), F(-1, U), F(U, U - 1), F(-U, U - 1)]
    values += [sign * v for v in hard for sign in (1, -1)]
    cases = [('atan', 'Ratio', v, F(0)) for v in values]
    vectors = [(F(y), F(x)) for y, x in ((0, 1), (0, -1), (1, 0), (-1, 0),
               (1, 1), (1, -1), (-1, -1), (-1, 1), (1, 2), (2, 1), (-2, 1), (1, -2))]
    for kind in ('Length', 'Ratio'):
        cases += [('atan2', kind, y, x) for y, x in vectors]
        cases += [('atan2', kind, sign * v, F(M)) for v in hard for sign in (1, -1)]
        axis = F(1_000_000_000) if kind == 'Length' else F(U)
        cases += [('atan2', kind, F(sign, U), -axis) for sign in (-1, 1)]
        if kind == 'Ratio':
            cases += [('atan2', kind, sign * axis, F(1, U)) for sign in (-1, 1)]
    yield '# call\tkind\ty numerator\ty denominator\tx numerator\tx denominator\tangle microdegrees'
    for call, kind, y, x in cases:
        for value in (y, x):
            assert abs(value.numerator) <= U and value.denominator <= U
            assert kind != 'Length' or abs(value) <= 1_000_000_000
        interval = atan(y / M, pi, anchor) if call == 'atan' else atan2(y, x, pi, anchor)
        answer = rounded(interval[0])
        assert answer == rounded(interval[1]), ('unresolved rounding', call, y, x)
        yield f'{call}\t{kind}\t{y.numerator}\t{y.denominator}\t{x.numerator}\t{x.denominator}\t{answer}'


if __name__ == '__main__':
    print('\n'.join(rows()))
