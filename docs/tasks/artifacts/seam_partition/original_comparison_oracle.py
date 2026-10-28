"""Independent Fraction orderings for actual offset-projected originals;frozen native inputs."""
from fractions import Fraction as F
from pathlib import Path
import sys

def render():
    m=(1<<63)-1
    cases=[
        ('wide-half',False,F(0),F(1,2),F(1,m),F(1,2),F(1,8)),
        ('wide-offset',False,F(1,m-1),F(1,m-3),F(1,m-2),F(2,m-2),F(1,m-1)),
        ('exact-bound',False,F(1,4),F(3,4),F(0),F(1),F(1,4)),
        ('equal-half',False,F(0),F(1),F(1,2),F(1,2),F(1,2)),
        ('reversed-source',True,F(1,4),F(3,4),F(1,4),F(3,4),F(1,2)),
        ('narrow-neighbor',False,F(0),F(1,2),F(1,m),F(2,m),F(1,m)),
    ]
    rows=['# case\treverse\tfrom_n\tfrom_d\tto_n\tto_d\ta_n\ta_d\tb_n\tb_d\tbound_n\tbound_d\tcompare_ab\tcompare_a_bound\tcompare_b_bound\ta_param\ta_u128']
    compare=lambda a,b:'Less' if a<b else 'Greater' if a>b else 'Equal'
    wide=0; beyond=0
    for name,reverse,low,high,a,b,bound in cases:
        assert 0<=low<high<=1 and 0<=a<=1 and 0<=b<=1 and 0<=bound<=1
        left,right=(low+q*(high-low) for q in (a,b))
        if reverse:left,right=1-left,1-right
        param=left.numerator<=m and left.denominator<=m
        u128=max(left.numerator.bit_length(),left.denominator.bit_length())<=128
        wide+=not param;beyond+=not u128
        values=[name,str(int(reverse))]
        for q in (low,high,a,b,bound):values.extend((str(q.numerator),str(q.denominator)))
        values.extend((compare(left,right),compare(left,bound),compare(right,bound),str(int(param)),str(int(u128))))
        rows.append('\t'.join(values))
    assert wide==3 and beyond==1
    return '\n'.join(rows)+'\n'

if __name__=='__main__':
    output=render()
    if sys.argv[1:]==['--check']:
        root=Path(__file__).resolve().parents[4]
        assert (root/'crates/sc-core/tests/fixtures/original_comparison.tsv').read_bytes()==output.encode()
        print('original comparison:6 Fraction cases;3 bounded-reference refusals/1 beyond-u128;all exact orderings;rc=0')
    elif not sys.argv[1:]:print(output,end='')
    else:raise SystemExit('usage:original_comparison_oracle.py [--check]')
