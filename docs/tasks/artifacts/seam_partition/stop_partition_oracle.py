"""Independent exact closed interval membership in canonical A pair order;no project imports."""
from fractions import Fraction as F
from pathlib import Path
import sys


def render():
    points = [('start', F(0)), ('quarter', F(1, 4)), ('third', F(1, 3)),
              ('half', F(1, 2)), ('two-thirds', F(2, 3)), ('three-quarters', F(3, 4)),
              ('end', F(1)), ('wide-later', F(1, 3 * ((1 << 63) - 1)))]
    rows = ['# case\tposition_n\tposition_d\tside\tdirection\tcut_n\tcut_d\tfirst\tsecond']
    shared = 0
    for name, point in points:
        for side, cut in (('A', F(1, 2)), ('B', F(2, 3))):
            for direction in ('Same', 'Opposite'):
                halves = ((F(0), cut), (cut, F(1)))
                if side == 'B' and direction == 'Opposite':
                    halves = halves[::-1]
                first, second = (low <= point <= high for low, high in halves)
                assert first or second
                assert (first and second) == (point == cut)
                shared += first and second
                rows.append('\t'.join(map(str, (f'{name}-{side}-{direction}',
                    point.numerator, point.denominator, side, direction,
                    cut.numerator, cut.denominator, int(first), int(second)))))
    assert shared == 4
    assert points[-1][1].denominator.bit_length() == 65
    # A half-open assignment loses one exact cut reference;ignoring B orientation swaps endpoints.
    assert F(1, 2) <= F(1, 2) and not F(1, 2) < F(1, 2)
    assert not (F(2, 3) <= F(0) <= F(1)) and F(0) <= F(0) <= F(2, 3)
    return '\n'.join(rows) + '\n'


if __name__ == '__main__':
    output = render()
    if sys.argv[1:] == ['--check']:
        root = Path(__file__).resolve().parents[4]
        assert (root / 'crates/sc-core/tests/fixtures/seam_stop_partition.tsv').read_bytes() == output.encode()
        print('stop partition oracle:32 exact closed membership rows;4 shared-cut controls;'
              'B orientation control;65-bit later-original control;rc=0')
    elif not sys.argv[1:]:
        print(output, end='')
    else:
        raise SystemExit('usage:stop_partition_oracle.py [--check]')
