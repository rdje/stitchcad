"""Independent Fraction cases at the converted reduced denominator-width frontier."""
from fractions import Fraction
from pathlib import Path
import runpy
import sys
ROOT=Path(__file__).resolve().parents[4]
FIXTURE=ROOT/'docs/tasks/artifacts/formula_structure/reduction_boundary_cases.tsv'
FACTORS={'um':('length',1),'mm':('length',1000),'cm':('length',10000),
         'm':('length',1000000),'in':('length',25400),'deg':('angle',1000000),
         'pct':('ratio',10000),'':('ratio',1000000)}
_,reference=runpy.run_path(str(ROOT/'docs/tasks/artifacts/formula_structure/formula_input.py'))['load_context']()
assert reference.units=={unit:(kind,Fraction(factor)) for unit,(kind,factor) in FACTORS.items() if unit}

def valuation(value,prime):
    count=0
    while value%prime==0:
        value//=prime;count+=1
    return count

maximum=max(valuation(factor,prime) for _,factor in FACTORS.values() for prime in [2,5])
assert maximum==6
assert len(str(2**128-1))==39 and 10**39>2**128-1
assert 134-maximum>=128 and 134+39==173
# Trimmed nonzero mantissas lack at least one factor2/5. For scales >134, the absent
# factor remains in the denominator at exponent >=129 even after the multiplier's <=6.
# These are independent arithmetic witnesses, not a second decimal-conversion implementation.
ROWS=[]
for prime,scales in [(2,range(54,64)),(5,range(125,137))]:
    for scale in scales:
        digits=str(prime**scale).rjust(scale+1,'0')
        number=digits[:-scale]+'.'+digits[-scale:]
        for unit,(kind,factor) in FACTORS.items():
            source=number+(' '+unit if unit else '')
            value=Fraction(number)*factor
            bits=max(value.numerator.bit_length(),value.denominator.bit_length())
            if bits>128:
                verdict='width'
            else:
                low=value.numerator//value.denominator
                magnitude=low+int(value-low>=Fraction(1,2))
                assert magnitude==0 # These controls isolate valid sub-quantum input versus width refusal.
                verdict=str(magnitude)
            ROWS.append((source,kind,verdict))
assert len(ROWS)==176
assert any(kind=='angle' and verdict=='0' and len(source.split('.')[1].split(' ')[0])==133 for source,kind,verdict in ROWS)
assert any(kind=='angle' and verdict=='width' and len(source.split('.')[1].split(' ')[0])==134 for source,kind,verdict in ROWS)
if sys.argv[1:]==['--emit']:
    print('# source\tkind\tmagnitude or width refusal')
    for row in ROWS:print('\t'.join(row))
else:
    assert not sys.argv[1:]
    actual=[tuple(line.split('\t')) for line in FIXTURE.read_text().splitlines() if line and not line.startswith('#')]
    assert actual==ROWS,'reduction frontier fixture drift'
    accepted=sum(row[2]=='0' for row in ROWS)
    print('reduction boundary oracle: 176 Fraction rows / %d accepted / %d width refusals; multiplier valuations6/digits39/scale134/workspace173 witnesses pass'%(accepted,len(ROWS)-accepted))
