"""Independent exact Fraction oracle for product literal input, not decimal long division."""
from fractions import Fraction
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[4]
FIXTURE = ROOT / 'docs/tasks/artifacts/formula_structure/literal_normalization_cases.tsv'
MAX = 2**128 - 1
FACTORS = {'um': ('length', 1), 'mm': ('length', 1000), 'cm': ('length', 10000),
           'm': ('length', 1000000), 'in': ('length', 25400), 'deg': ('angle', 1000000),
           'pct': ('ratio', 10000)}
SOURCES = ['0', '004', '1.0', '2.0000', str(2**63), str(MAX), str(MAX+1),
           '0.0000004', '0.0000005', '0.0000015', '0.000000000000000000000000000000000000001',
           '0.0000000000000000000000000000000000000001']
for unit in FACTORS:
    SOURCES += [n + ' ' + unit for n in ['0', '1', '0.5', '2.5', '0.0000005',
                                         '0.00004', '0.00005', '0.00006',
                                         '999.999999999', '1000.0000005']]
SOURCES += ['1000000000.4 um', '1000000000.5 um', '100000.00004 cm', '100000.00005 cm',
            '9223372036854.775808 deg', '360 deg', '720 deg',
            '340282366920938463463374607431768.211455 deg',
            '340282366920938463463374607431768.211456 deg']
# Wide raw mantissas that reduce to valid internal rationals, including pure powers2/5.
for base, power, scale, unit in [(2, 128, 1, 'pct'), (2, 130, 40, 'deg'),
                                 (2, 140, 44, 'um'), (5, 128, 128, 'deg')]:
    digits = str(base**power).rjust(scale+1, '0')
    SOURCES.append(digits[:-scale]+'.'+digits[-scale:]+' '+unit)
SOURCES += ['0.'+'0'*134+'1 um', '0.'+'0'*135+'2 deg',
            '9'*180+' um', '0.'+'0'*180+'0 deg', '0'*180+'1.0'+ '0'*180+' pct']
assert len(SOURCES)==len(set(SOURCES))


def expected(source):
    parts=source.split(' ')
    number=parts[0]
    kind, factor=FACTORS[parts[1]] if len(parts)==2 else (('ratio',1000000) if '.' in number else ('count',1))
    value=Fraction(number)*factor
    if max(value.numerator.bit_length(), value.denominator.bit_length())>128:
        return kind, 'width'
    # Fraction's exact comparison independently encodes nearest ties up; no shared remainder code.
    low=value.numerator//value.denominator
    result=low+int(value-Fraction(low)>=Fraction(1,2))
    if kind=='length' and result>1000000000:
        return kind, 'length'
    return kind, str(result)


if sys.argv[1:]==['--emit']:
    print('# machine literal\tkind\tmagnitude or width/length refusal')
    for source in SOURCES:
        print(source+'\t'+'\t'.join(expected(source)))
else:
    assert not sys.argv[1:]
    rows=[line.split('\t') for line in FIXTURE.read_text().splitlines() if line and not line.startswith('#')]
    assert [row[0] for row in rows]==SOURCES
    for source,kind,want in rows:
        assert expected(source)==(kind,want), (source,kind,want,expected(source))
    print('literal normalization Fraction oracle: %d conversion/width/quantum/domain rows pass' % len(rows))
