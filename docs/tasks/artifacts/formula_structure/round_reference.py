"""D78: independent arbitrary-precision Fraction oracle for public i128 -> i64 boundary rows."""
from fractions import Fraction
from pathlib import Path
ROOT = Path(__file__).resolve().parents[4]
count = 0
for row in (ROOT / 'docs/tasks/artifacts/formula_structure/round_cases.tsv').read_text().splitlines():
    if row.startswith('#') or not row: continue
    numerator, denominator, expected = row.split('\t')
    n,d = int(numerator),int(denominator)
    assert -(1 << 127) <= n < (1 << 127) and -(1 << 127) <= d < (1 << 127)
    if d == 0: actual = 'zero'
    else:
        ratio = Fraction(n,d)
        quotient,remainder = divmod(abs(ratio.numerator),ratio.denominator)
        # Compare exact fraction to its integer midpoint, with no bounded casts or floating point.
        midpoint = Fraction(2 * quotient + 1, 2)
        rounded = quotient + (abs(ratio) >= midpoint)
        rounded = -rounded if ratio < 0 else rounded
        actual = str(rounded) if -(1 << 63) <= rounded < (1 << 63) else 'overflow'
    assert actual == expected, (row,actual)
    count += 1
assert count == 36
print('round Fraction oracle: 36 exact width/sign/tie/zero rows pass')
