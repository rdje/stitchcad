"""Exclusive operator-signature faults; actual compiled assertion reds with byte restoration."""
from pathlib import Path
import os
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'crates/sc-core/src/recipe/operators.rs'
original = SOURCE.read_bytes()
WORK = ROOT / 'target/operator_signature_mutations'
WORK.mkdir(parents=True, exist_ok=True)
CASES = (('square token', 'Self::Square => "^2",', 'Self::Square => "square",'),
 ('binary token', 'Self::Less => "<",', 'Self::Less => ">",'),
 ('count negation',
  '(Self::Negate, K::Length | K::Angle | K::Area | K::Ratio)',
  '(Self::Negate, K::Length | K::Angle | K::Area | K::Ratio | K::Count)'),
 ('square length result',
  '(Self::Square, K::Length) => Some(K::Area),',
  '(Self::Square, K::Length) => Some(K::Length),'),
 ('boolean arithmetic',
  'matches!(kind, K::Length | K::Angle | K::Area | K::Ratio | K::Count)',
  'matches!(kind, K::Length | K::Angle | K::Area | K::Ratio | K::Count | K::Boolean)'),
 ('mismatched addition',
  'Self::Add | Self::Subtract if left == right && arithmetic(left)',
  'Self::Add | Self::Subtract if arithmetic(left)'),
 ('comparison result', '                Some(K::Boolean)\n', '                Some(left)\n'),
 ('length product result',
  '(K::Length, K::Length) => Some(K::Area),',
  '(K::Length, K::Length) => Some(K::Length),'),
 ('noncommutative product',
  '(K::Length, K::Ratio | K::Count) | (K::Ratio | K::Count, K::Length)',
  '(K::Length, K::Ratio | K::Count)'),
 ('reversed quotient',
  'Self::Divide => match (left, right)',
  'Self::Divide => match (right, left)'),
 ('count quotient result',
  '| (K::Count, K::Count) => Some(K::Ratio),',
  '=> Some(K::Ratio),\n                (K::Count, K::Count) => Some(K::Count),'),
 ('count ratio quotient',
  '(K::Count, K::Ratio) => Some(K::Count),',
  '(K::Count, K::Ratio) => Some(K::Ratio),'),
 ('asymmetric arc hint',
  '(Self::Multiply, K::Angle, K::Length) | (Self::Multiply, K::Length, K::Angle)',
  '(Self::Multiply, K::Angle, K::Length)'),
 ('wrong arc hint operator',
  '(Self::Multiply, K::Angle, K::Length) | (Self::Multiply, K::Length, K::Angle)',
  '(Self::Divide, K::Angle, K::Length) | (Self::Divide, K::Length, K::Angle)'))
for name,before,_ in CASES:
    assert original.decode().count(before)==1, (name,'actual fault anchor not unique')

def assertion_failure(output):
    parts=output.split(b'\nfailures:\n',2)
    return len(parts)==3 and re.search(rb'(?m)^assertion(?:[ :`]|$)',parts[1]) is not None

assert assertion_failure(b'\nfailures:\nthread panicked:\nassertion `left == right` failed\n\nfailures:\nfixture\n')
assert not assertion_failure(b'\nfailures:\nthread panicked:\nexpect-only\n\nfailures:\nassertion_name\n')
assert not assertion_failure(b'assertion: compiler output\n')
assert sys.argv[1:] in [[],['--classifier-only']]
if sys.argv[1:]==['--classifier-only']:
    print('operator signature fault controls:14 unique actual anchors; name/expect/compiler noise refused')
    sys.exit(0)
environment=dict(os.environ,CARGO_HOME=str(ROOT/'target/cargo-home'),
                 CARGO_TARGET_DIR=str(ROOT/'target'),TMPDIR=str(ROOT/'target/scratch'))
try:
    for index,(name,before,after) in enumerate(CASES,1):
        SOURCE.write_text(original.decode().replace(before,after,1))
        result=subprocess.run(['cargo','test','-p','sc-core','--test','formula_operator_signature_contract'],
                              cwd=ROOT,env=environment,capture_output=True)
        output=result.stdout+result.stderr
        (WORK/('fault-%d.log'%index)).write_bytes(output)
        assert result.returncode==101 and b'test result: FAILED.' in output,(name,output.decode())
        assert assertion_failure(output) and b'could not compile' not in output,(name,output.decode())
        print('operator signature fault %d %s:rc=101 actual compiled body assertion red'%(index,name))
        SOURCE.write_bytes(original)
finally:
    SOURCE.write_bytes(original)
assert SOURCE.read_bytes()==original
print('operator signature faults:14 actual compiled assertion reds; exact source restored')
