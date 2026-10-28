"""Independent exact affine inverse coordinates,including components beyond128 bits."""
from fractions import Fraction as F
from pathlib import Path
import sys


def render():
    m = (1 << 63) - 1
    cases = [
        ('wide-half', F(1, m), F(0), F(1, 2)),
        ('wide-offset', F(1, m - 2), F(1, m - 1), F(1, m - 3)),
        ('near-one-wide', F(m - 1, m), F(m - 4, m - 3), F(m - 2, m - 1)),
        ('interior', F(1, 3), F(1, 5), F(4, 5)),
        ('start', F(0), F(1, m - 1), F(1, m - 3)),
        ('end', F(1), F(1, m - 1), F(1, m - 3)),
    ]
    rows = ['# case\tlocal_n\tlocal_d\tfrom_n\tfrom_d\tto_n\tto_d\texpected_n\texpected_d']
    widest = 0
    for name, local, low, high in cases:
        assert 0 <= local <= 1 and 0 <= low < high <= 1
        original = low + local * (high - low)
        widest = max(widest, original.numerator.bit_length(), original.denominator.bit_length())
        rows.append('\t'.join(map(str, (name, local.numerator, local.denominator,
                                       low.numerator, low.denominator, high.numerator, high.denominator,
                                       original.numerator, original.denominator))))
    assert widest == 188
    return '\n'.join(rows) + '\n'


if __name__ == '__main__':
    output = render()
    if sys.argv[1:] == ['--check']:
        root = Path(__file__).resolve().parents[4]
        assert (root / 'crates/sc-core/tests/fixtures/original_projection.tsv').read_bytes() == output.encode()
        print('original projection oracle:6 affine Fraction rows;188-bit component control;rc=0')
    elif not sys.argv[1:]:
        print(output, end='')
    else:
        raise SystemExit('usage: original_projection_oracle.py [--check]')
