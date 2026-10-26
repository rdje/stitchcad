"""Pure stdout independent full signed-turn sin/cos fixtures.

Different pi identity,768-bit direct powers/factorials over [-pi,pi],128 terms.
No native octant folding, term recurrence, pi fixture or threshold-certificate import.
"""
from fractions import Fraction as F
from math import factorial

M = 1_000_000
U = (1 << 128) - 1
BITS = 768
S = 1 << BITS
TERMS = 128


def ceildiv(a, b):
    return (a + b - 1) // b


def pi_bounds():
    values = []
    for base in (2, 3):
        partial = sum((F((-1) ** n, (2 * n + 1) * base ** (2 * n + 1))
                       for n in range(512)), F(0))
        values.append((partial, partial + F(1, 1025 * base ** 1025)))
    # tan(atan(1/2)+atan(1/3))=1 in the principal interval: their sum is pi/4.
    return 4 * (values[0][0] + values[1][0]), 4 * (values[0][1] + values[1][1])


def rounded(numerator, denominator):
    q, r = divmod(abs(numerator), denominator)
    result = q + (2 * r >= denominator)
    return -result if numerator < 0 else result


def output_interval(angle, cosine, pi):
    # Signed modulo and a full principal interval expose quadrant/sign bugs in the native folding.
    angle %= 360 * M
    if angle > 180 * M:
        angle -= 360 * M
    sign = -1 if angle < 0 and not cosine else 1
    magnitude = abs(angle) / (180 * M)
    xlow, xhigh = magnitude * pi[0], magnitude * pi[1]
    low = xlow.numerator * S // xlow.denominator
    high = ceildiv(xhigh.numerator * S, xhigh.denominator)
    assert 0 <= low <= high < 4 * S
    lower = upper = 0
    for n in range(TERMS):
        power = 2 * n + (0 if cosine else 1)
        denominator = S ** power * factorial(power)
        a = low ** power * S // denominator
        b = ceildiv(high ** power * S, denominator)
        if n % 2:
            lower -= b
            upper -= a
        else:
            lower += a
            upper += b
    # Even at x=4 the first omitted term is below one768-bit unit, for either series.
    assert S * 4 ** 256 < factorial(256)
    lower, upper = max(-S, lower - 1), min(S, upper + 1)
    return (lower, upper) if sign > 0 else (-upper, -lower)


def neighbors(value, maximum):
    n, d = value.numerator, value.denominator
    p0, q0, p1, q1 = 0, 1, 1, 0
    while d:
        a = n // d
        if q0 + a * q1 > maximum:
            break
        p0, q0, p1, q1 = p1, q1, p0 + a * p1, q0 + a * q1
        n, d = d, n - a * d
    assert d, 'threshold unexpectedly rational at admitted width'
    k = (maximum - q0) // q1
    return sorted((F(p0 + k * p1, q0 + k * q1), F(p1, q1)))


def hard_angles(pi):
    for value in (F(1, 2 * M), F(M + 1, 2 * M)):
        term, total = value, F(0)
        for n in range(256):
            total += term
            term *= F((2 * n + 1) ** 2, 2 * (n + 1) * (2 * n + 3)) * value * value
        tail = value ** 513 / (1 - value * value)
        low, high = total * 180 * M / pi[1], (total + tail) * 180 * M / pi[0]
        left, right = neighbors(low, U // (90 * M))
        assert left < low <= high < right
        for angle in (left, right, 90 * M - left, 90 * M - right):
            yield angle
            yield -angle


def fixture_rows():
    pi = pi_bounds()
    angles = [F(v * M) for v in [-720, -450, -360, -270, -180, -90, -60, -45, -30,
                                 0, 30, 45, 60, 90, 180, 270, 360, 450, 720]]
    d = U // (90 * M)
    angles += [F(U), F(-U), F(U, U - 1), F(-U, U - 1), F(1, U), F(-1, U),
               F(90 * M * d - 1, d), F(90 * M * d + 1, d), F(1, 3)]
    angles += list(hard_angles(pi))
    yield '# angle numerator\tangle denominator\tsine ppm\tcosine ppm'
    for angle in angles:
        assert abs(angle.numerator) <= U and angle.denominator <= U
        answers = []
        for cosine in (False, True):
            lower, upper = output_interval(angle, cosine, pi)
            answer = rounded(lower * M, S)
            assert answer == rounded(upper * M, S), ('oracle rounding unresolved', angle, cosine)
            answers.append(answer)
        if angle == 0:
            assert answers == [0, M]
        if angle == 30 * M:
            assert answers[0] == M // 2
        if abs(angle) == 90 * M:
            assert answers == [M if angle > 0 else -M, 0]
        yield '\t'.join(map(str, (angle.numerator, angle.denominator, *answers)))


if __name__ == '__main__':
    print('\n'.join(fixture_rows()))
