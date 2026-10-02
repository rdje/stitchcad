"""Actual premature-refusal/reduction faults must fail the new coupled public assertion oracle."""
from pathlib import Path
import subprocess
ROOT=Path(__file__).resolve().parents[4]
SOURCE=ROOT/'crates/sc-core/src/recipe/literal.rs'
ORIGINAL=SOURCE.read_bytes()
WORK=ROOT/'target/formula_reduction_mutations';WORK.mkdir(exist_ok=True)
CASES=[('scale narrowed','if scale > 134 {','if scale > 127 {'),
       ('raw digits narrowed','if significant > scale + 39 {','if significant > 39 {'),
       ('mantissa cancellation removed','divide_decimal(&mut decimal, prime);','divide_decimal(&mut decimal, 1);'),
       ('unit cancellation removed','multiplier /= prime;','multiplier /= 1;')]
try:
 for index,(name,before,after) in enumerate(CASES,1):
  text=ORIGINAL.decode();assert text.count(before)==1,(name,'actual anchor not unique')
  SOURCE.write_text(text.replace(before,after,1))
  result=subprocess.run(['cargo','test','-p','sc-core','--test','formula_literal_contract',
    'coupled_reduction_frontier_preserves_valid_inputs_and_located_width_refusals','--','--exact'],cwd=ROOT,capture_output=True)
  output=result.stdout+result.stderr;(WORK/('mutation-%d.log'%index)).write_bytes(output)
  assert result.returncode==101 and b'test result: FAILED.' in output and b'assertion' in output and b'could not compile' not in output,(name,output.decode())
  print('reduction boundary mutation %d %s: rc=101, actual compiled public assertion red'%(index,name))
  SOURCE.write_bytes(ORIGINAL)
finally:SOURCE.write_bytes(ORIGINAL)
assert SOURCE.read_bytes()==ORIGINAL
print('reduction boundary mutations: four compiled actual assertion reds; exact source restored')
