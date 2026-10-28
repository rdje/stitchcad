"""Independent integer reservations and exact inverse-map width falsification;pure stdout."""
from fractions import Fraction as F
from pathlib import Path
from random import Random
import sys


def render():
    maximum = (1 << 64) - 1
    last = maximum // 254 - 1
    rows = ['# journal_entries\tcomponent_bits\tworkspace_bits']
    for count in (0, 1, 2, 17, last, last + 1):
        component = 63 + 127 * count
        workspace = 2 * component + 128
        value = str(workspace) if workspace <= maximum else 'Overflow'
        rows.append(f'{count}\t{component}\t{value}')
    # Each offset is a new actual unit-interval numeric frame. Cross-products use independent
    # Fraction reduction;the bound is deliberately conservative,not a measured maximum claim.
    rng = Random(0x535449544348434144)
    m = (1 << 63) - 1
    for _ in range(80):
        q = F(rng.randrange(m + 1), m)
        for count in range(1, 13):
            bounds = sorted((F(rng.randrange(m), m - rng.randrange(1000)),
                             F(rng.randrange(m), m - rng.randrange(1000))))
            low, high = bounds
            if not 0 <= low < high <= 1:
                low, high = F(1, m), F(m - 1, m)
            extent = high - low
            scaled = q * extent
            raw_sum = scaled.numerator * low.denominator + low.numerator * scaled.denominator
            raw_denominator = scaled.denominator * low.denominator
            workspace = 2 * (63 + 127 * count) + 128
            assert max(raw_sum.bit_length(), raw_denominator.bit_length()) <= workspace
            q = low + scaled
            assert 0 <= q <= 1
            assert max(q.numerator.bit_length(), q.denominator.bit_length()) <= 63 + 127 * count
    assert (F(1, m) / 2).denominator.bit_length() == 64
    return '\n'.join(rows) + '\n'


if __name__ == '__main__':
    output = render()
    if sys.argv[1:] == ['--check']:
        root = Path(__file__).resolve().parents[4]
        assert (root / 'crates/sc-core/tests/fixtures/projection_workspace.tsv').read_bytes() == output.encode()
        print('projection workspace:6 exact integer rows;960 Fraction inverse-offset component/scratch checks;64-bit control;rc=0')
    elif not sys.argv[1:]:
        print(output, end='')
    else:
        raise SystemExit('usage: projection_workspace_oracle.py [--check]')
