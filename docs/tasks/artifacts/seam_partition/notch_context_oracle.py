"""Independent original-coordinate recovery examples; pure Fraction arithmetic and stdout.

Native tests reproduce the accepted landmark/graph states and forward witnesses. Original-source
projection is not implemented by those recovery tests. --check verifies the frozen evidence.
"""
from fractions import Fraction as F
from pathlib import Path
import sys


def quantize(value):
    whole, remainder = divmod(abs(value.numerator), value.denominator)
    rounded = whole + (2 * remainder >= value.denominator)
    return -rounded if value < 0 else rounded


def render():
    maximum = (1 << 63) - 1
    split = F(1, 2)
    current = F(1, maximum)
    original = current * split
    assert original.numerator == 1 and original.denominator == 2 * maximum
    assert original.denominator.bit_length() == 64
    # A linear allocation between the actual original endpoints is below half at1/8.
    # Replacing the tiny endpoint by zero or using binary64 subtraction changes the quantum.
    end, at = F(1, 4), F(1, 8)
    progress = (at - original) / (end - original)
    assert quantize(progress) == 0 and quantize(at / end) == 1
    assert (float(at) - float(original)) / (float(end) - float(original)) == 0.5
    # Split at half,reverse second fragment,merge first then reversed second with equal positive
    # declared lengths. The current join has original witnesses at the split and original end.
    candidates = (split, F(1))
    current_join = F(1, 2)
    first_path = (candidates[0] / split) / 2
    second_path = F(1, 2) + (1 - (candidates[1] - split) / (1 - split)) / 2
    assert first_path == second_path == current_join and candidates[0] != candidates[1]
    lower, cut = F(1, 4), F(1, 2)
    possible = [quantize((cut - lower) / (upper - lower)) for upper in candidates]
    assert possible == [1, 0]
    # A different case genuinely fits the existing current-reference fields. Only its temporary
    # offset exceeds that bound;it must not be confused with the legitimate wide-original case.
    divisor = maximum - 3
    split_at, local = F(1, 3), F(1, divisor)
    held = split_at + (1 - split_at) * local
    assert held == F(1537228672809129301, 4611686018427387902)
    offset = held - split_at
    assert offset.denominator.bit_length() == 64 and local.denominator.bit_length() == 63
    assert offset / (1 - split_at) == local
    return ('# case\tcurrent_n\tcurrent_d\toriginal1_n\toriginal1_d\toriginal2_n\toriginal2_d\n'
            f'wide-split\t1\t{maximum}\t1\t{2 * maximum}\t-\t-\n'
            'reversed-merge\t1\t2\t1\t2\t1\t1\n'
            f'split-final-fits\t1\t{divisor}\t{held.numerator}\t{held.denominator}\t1\t3\n')


if __name__ == '__main__':
    output = render()
    if sys.argv[1:] == ['--check']:
        root = Path(__file__).resolve().parents[4]
        assert (root / 'crates/sc-core/tests/fixtures/notch_context_recovery.tsv').read_bytes() == output.encode()
        print('notch context oracle:3 cases;64-bit original denominator;two inverse witnesses;'
              'linear quantum controls0/1 and1/0;64-bit temporary/63-bit final split;rc=0')
    elif not sys.argv[1:]:
        print(output, end='')
    else:
        raise SystemExit('usage: notch_context_oracle.py [--check]')
