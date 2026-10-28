"""Independent Fraction evidence for completed reference maps; stdout or frozen-file check."""
from fractions import Fraction as F
from pathlib import Path
import sys


def render():
    m = (1 << 63) - 1
    p = F(1537228672809129301, 4611686018427387902)
    d = m - 1
    cases = [
        ('split-second-cancel', 'split-second', p, F(1, 3), F(1)),
        ('offset-shift-cancel', 'offset', p, F(1, 3), F(1)),
        ('offset-width-cancel', 'offset', F(1, d - 2), F(1, d), F(1, d - 4)),
        ('offset-near-end-cancel', 'offset', F(d - 3, d - 2), F(d - 5, d - 4), F(d - 1, d)),
        ('merge-first-product', 'merge-first', F(m - 1, m), F(1000), F(1000)),
        ('merge-second-shift', 'merge-second', F(1, m), F(1000), F(1000)),
        ('merge-second-product', 'merge-second', F(m - 2, m), F(1000), F(1000)),
        ('merge-first-unequal', 'merge-first', F(m - 3, m), F(1000), F(3000)),
        ('merge-second-unequal', 'merge-second', F(3, m), F(1000), F(3000)),
        ('split-first-final-wide', 'split-first', F(1, m), F(2, 3), F(1)),
        ('split-second-final-wide', 'split-second', F(m - 1, m), F(1, 3), F(1)),
        ('offset-final-wide', 'offset', F(1, m - 1), F(1, m), F(2, m - 1)),
        ('merge-first-final-wide', 'merge-first', F(1, m), F(1000), F(1000)),
        ('merge-second-final-wide', 'merge-second', F(m - 1, m), F(1000), F(1000)),
    ]
    rows = ['# case\tkind\theld_n\theld_d\ta_n\ta_d\tb_n\tb_d\texpected']
    cancellation = refusal = 0
    for name, kind, held, a, b in cases:
        if kind == 'split-first':
            assert 0 < held < a < 1
            result = held / a
            temporaries = []
        elif kind in ('split-second', 'offset'):
            assert 0 <= a < held < b <= 1
            result = (held - a) / (b - a)
            temporaries = [held - a, b - a]
        else:
            assert 0 < held < 1 and a.denominator == b.denominator == 1 and a > 0 and b > 0
            scale, shift = (a, F(0)) if kind == 'merge-first' else (b, a)
            result = (held * scale + shift) / (a + b)
            temporaries = [held * scale, held * scale + shift]
        fits = lambda x: abs(x.numerator) <= m and x.denominator <= m
        if fits(result):
            assert any(not fits(x) for x in temporaries), 'each success defeats premature narrowing'
            expected = f'value:{result.numerator}/{result.denominator}'
            cancellation += 1
        else:
            expected = 'Overflow'
            refusal += 1
        rows.append('\t'.join(map(str, (name, kind, held.numerator, held.denominator,
                                        a.numerator, a.denominator, b.numerator, b.denominator, expected))))
    assert cancellation == 9 and refusal == 5
    # A reversed symmetric held interval can fail in BOTH split branches. The original
    # forward arithmetic order must still report first-branch division,then reverse output
    # only after success. This independently establishes the native precedence control.
    low, high, cut = F(1, m), F(m - 1, m), F(2, 5)
    assert 1 - high == low and 1 - low == high
    assert (low / cut).denominator > m
    assert ((high - cut) / (1 - cut)).denominator > m
    # Retained internal half after split/merge maps wide through a valid shared-boundary offset;
    # original endpoints still map exactly to 0/1. New current semantic anchors do not erase this
    # complete held-range numeric repair. The native source validator retains its actual cause.
    assert ((F(1, 2) - F(1, m)) / (1 - F(1, m))).denominator > m
    return '\n'.join(rows) + '\n'


if __name__ == '__main__':
    output = render()
    if sys.argv[1:] == ['--check']:
        root = Path(__file__).resolve().parents[4]
        assert (root / 'crates/sc-core/tests/fixtures/reference_recomposition.tsv').read_bytes() == output.encode()
        print('reference recomposition:14 Fraction cases;9 cancelled intermediate refusals;5 genuine final-width refusals;reversed dual-failure control1;source internal-boundary control1;rc=0')
    elif not sys.argv[1:]:
        print(output, end='')
    else:
        raise SystemExit('usage: reference_recomposition_oracle.py [--check]')
