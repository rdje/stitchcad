"""Independent Fraction/raw-integer notch quantity width controls;pure stdout,project-free math."""
from fractions import Fraction as F
from pathlib import Path
from random import Random
import sys


def progress(lower, upper, at, magnitude, component_bits):
    assert 0 <= lower < upper <= 1 and 0 <= at <= 1
    assert max(lower.numerator.bit_length(), lower.denominator.bit_length(),
               upper.numerator.bit_length(), upper.denominator.bit_length()) <= component_bits
    assert max(at.numerator.bit_length(), at.denominator.bit_length()) <= 63
    assert 0 <= magnitude <= 1_000_000_000
    ln, ld, un, ud = lower.numerator, lower.denominator, upper.numerator, upper.denominator
    xn, xd = at.numerator, at.denominator
    operations = [xn * ld, ln * xd, xn * ud, un * xd, un * ld, ln * ud]
    if at <= lower:
        numerator, denominator = 0, 1
    elif at >= upper:
        numerator, denominator = 1, 1
    else:
        offset, extent = xn * ld - ln * xd, un * ld - ln * ud
        numerator, denominator = offset * ud, extent * xd
        operations.extend((offset, extent, numerator, denominator))
    exact = max(F(0), min(F(1), (at - lower) / (upper - lower)))
    assert F(numerator, denominator) == exact
    for prefix in (numerator, denominator - numerator):
        scaled = prefix * magnitude
        quotient, remainder = divmod(scaled, denominator)
        operations.extend((prefix, scaled, quotient, remainder, denominator - remainder))
    bound = 2 * component_bits + 63 + 30
    assert max(value.bit_length() for value in operations) <= bound


def render():
    maximum = (1 << 64) - 1
    last = (maximum - 93) // 2
    rows = ['# original_component_bits\tquantity_integer_bits']
    for component in (0, 63, 190, 317, last, last + 1):
        bound = 2 * component + 93
        rows.append(f'{component}\t{bound if bound <= maximum else "Overflow"}')
    # Actual wide inverse-offset examples,calculated independently with Fraction.
    m = (1 << 63) - 1
    lower = F(1, m) / 2
    assert lower.denominator.bit_length() == 64
    progress(lower, F(1, 4), F(1, 8), 1, 64)
    low, high = F(1, m - 1), F(1, m - 3)
    a, b = (low + q * (high - low) for q in (F(1, m - 2), F(2, m - 2)))
    assert a.denominator.bit_length() == 188
    progress(a, b, low, 1_000_000_000, 188)
    rng = Random(0x4E4F544348)
    for _ in range(300):
        width = rng.randrange(63, 257)
        values = []
        for _ in range(2):
            denominator = rng.randrange(1 << (width - 1), 1 << width)
            values.append(F(rng.randrange(denominator + 1), denominator))
        lower, upper = sorted(values)
        if lower == upper:
            lower, upper = F(0), F(1)
        at = F(rng.randrange(m + 1), m)
        progress(lower, upper, at, rng.randrange(1_000_000_001), width)
    return '\n'.join(rows) + '\n'


def quantize(value):
    whole, remainder = divmod(abs(value.numerator), value.denominator)
    result = whole + (2 * remainder >= value.denominator)
    return -result if value < 0 else result


def render_queries():
    m = (1 << 63) - 1
    cases = [
        ('symmetric', False, F(0), F(1), F(1, 4), F(3, 4),
         (F(0), F(1, 4), F(3, 8), F(1, 2), F(5, 8), F(3, 4), F(1))),
        ('wide-half', False, F(0), F(1, 2), F(1, m), F(1, 2),
         (F(0), F(1, 8), F(1, 4), F(1, 2))),
        ('wide-offset', False, F(1, m - 1), F(1, m - 3), F(1, m - 2), F(m - 3, m - 2),
         (F(1, m - 1), F(1, m - 2), F(1, m - 3))),
        ('reversed', True, F(1, 4), F(3, 4), F(1, 4), F(3, 4),
         (F(1, 4), F(3, 8), F(1, 2), F(5, 8), F(3, 4))),
    ]
    rows = ['# case\treversed\tfrom_n\tfrom_d\tto_n\tto_d\tlocal_start_n\tlocal_start_d\tlocal_end_n\tlocal_end_d\tat_n\tat_d\tside\tdirection\tparent_um\tcumulative_um\tlower_interval_um\tupper_interval_um\toriginal_max_bits']
    for name, reversed_frame, low, high, start, end, queries in cases:
        originals = [low + (1 - q if reversed_frame else q) * (high - low) for q in (start, end)]
        lower, upper = sorted(originals)
        width = max(value.bit_length() for q in originals for value in (q.numerator, q.denominator))
        if name == 'wide-half':
            assert width == 64
            assert quantize((F(1, 8) - lower) / (upper - lower)) == 0
            assert (float(F(1, 8)) - float(lower)) / (float(upper) - float(lower)) == 0.5
        if name == 'wide-offset':
            assert width == 188
            assert quantize((F(1, m - 2) - lower) / (upper - lower)) == 0
        for at in queries:
            progress(lower, upper, at, 1_000_000_000, max(63, width))
            ascending = max(F(0), min(F(1), (at - lower) / (upper - lower)))
            for side, direction in [('A', 'Same'), ('A', 'Opposite'), ('B', 'Same'), ('B', 'Opposite')]:
                reverse = side == 'B' and direction == 'Opposite'
                fraction = 1 - ascending if reverse else ascending
                for parent in (-1_000_000_000, -1, 0, 1, 1_000_000_000):
                    cumulative = quantize(parent * fraction)
                    first = parent - cumulative if reverse else cumulative
                    second = parent - first
                    assert first + second == parent and cumulative == -quantize(-parent * fraction)
                    row = [name, str(int(reversed_frame))]
                    for value in (low, high, start, end, at):
                        row.extend((str(value.numerator), str(value.denominator)))
                    row.extend(map(str, (side, direction, parent, cumulative, first, second, width)))
                    rows.append('\t'.join(row))
    return '\n'.join(rows) + '\n'


if __name__ == '__main__':
    output = render()
    if sys.argv[1:] == ['--check']:
        root = Path(__file__).resolve().parents[4]
        assert (root / 'crates/sc-core/tests/fixtures/notch_quantity_workspace.tsv').read_bytes() == output.encode()
        queries = render_queries()
        assert (root / 'crates/sc-core/tests/fixtures/notch_quantities.tsv').read_bytes() == queries.encode()
        print(f'notch quantity workspace:6 checked bound rows;302 independent Fraction/raw-integer controls;{len(queries.splitlines()) - 1} exact query rows;64/188-bit originals;rc=0')
    elif sys.argv[1:] == ['--queries']:
        print(render_queries(), end='')
    elif not sys.argv[1:]:
        print(output, end='')
    else:
        raise SystemExit('usage:notch_quantity_oracle.py [--check|--queries]')
