"""Wide magnitude fixtures/oracle: Decimal rounding independent of production remainder arithmetic."""
from decimal import Decimal, ROUND_HALF_UP, localcontext
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[4]
FIXTURE = ROOT / 'docs/tasks/artifacts/formula_structure/unsigned_round_cases.tsv'
MAX = 2**128 - 1
numerators = [0, 1, 2, 3, 2**63 - 1, 2**63, 2**63 + 1,
              2**127 - 1, 2**127, 2**127 + 1, MAX - 2, MAX - 1, MAX]
denominators = [0, 1, 2, 3, 5, 7, 2**127 - 1, 2**127, MAX - 1, MAX]
pairs = [(n, d) for n in numerators for d in denominators]
# Even/odd high-denominator half neighborhoods cannot overflow a decimal oracle's integers.
for d in [MAX, MAX - 1, 2**127, 2**127 - 1]:
    for n in [d // 2 - 1, d // 2, d // 2 + 1]:
        if (n, d) not in pairs:
            pairs.append((n, d))


def expected(numerator, denominator):
    if denominator == 0:
        return 'zero'
    # 120 significant digits exceed 39 integer digits plus the smallest possible distance
    # from a half-integer (at least 1/(2*d), d <= 2^128-1). Exact ties are terminating halves.
    with localcontext() as context:
        context.prec = 120
        return str(int((Decimal(numerator) / Decimal(denominator)).to_integral_value(rounding=ROUND_HALF_UP)))


if sys.argv[1:] == ['--emit']:
    print('# numerator\tdenominator\tnearest unsigned magnitude (zero = refusal)')
    for n, d in pairs:
        print('%s\t%s\t%s' % (n, d, expected(n, d)))
else:
    assert not sys.argv[1:], 'only --emit or default verification supported'
    rows = [line.split('\t') for line in FIXTURE.read_text().splitlines()
            if line and not line.startswith('#')]
    assert [(int(n), int(d)) for n, d, _ in rows] == pairs, 'authored wide fixture population changed'
    for n, d, want in rows:
        n, d = int(n), int(d)
        assert 0 <= n <= MAX and 0 <= d <= MAX
        assert expected(n, d) == want, (n, d, want, expected(n, d))
        assert want == 'zero' or 0 <= int(want) <= MAX
    print('unsigned round Decimal oracle: %d full-u128 boundary/tie/zero rows pass' % len(rows))
