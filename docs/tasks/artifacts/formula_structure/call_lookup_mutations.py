"""Exclusive call-lookup faults; actual compiled assertion reds with byte restoration."""
from pathlib import Path
import os
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'crates/sc-core/src/recipe/calls.rs'
original = SOURCE.read_bytes()
WORK = ROOT / 'target/call_lookup_mutations'
WORK.mkdir(parents=True, exist_ok=True)
CASES = (('unknown accepted',
  'return FormulaBuiltin::from_token(name).ok_or(FormulaCallRefusal {',
  'return FormulaBuiltin::from_token(name).or(Some(FormulaBuiltin::Sin)).ok_or(FormulaCallRefusal '
  '{'),
 ('known identity changed',
  'return FormulaBuiltin::from_token(name).ok_or(FormulaCallRefusal {',
  'return FormulaBuiltin::from_token(name).map(|_| FormulaBuiltin::Cos).ok_or(FormulaCallRefusal '
  '{'),
 ('query forged',
  'name,\n                kind: FormulaCallRefusalKind::Unbound,',
  'name: "missing",\n                kind: FormulaCallRefusalKind::Unbound,'),
 ('curve alias omitted', '"nurbs" | "spline" | "bspline" =>', '"nurbs" | "spline" =>'),
 ('envelope alias widened',
  '"solve" | "constraint" | "fixpoint" =>',
  '"solve" | "constraint" | "fixpoint" | "solve_extra" =>'),
 ('curve family changed',
  '=> FormulaCallRefusalKind::Nurbs,',
  '=> FormulaCallRefusalKind::SketchConstraints,'),
 ('unbound token changed',
  'FormulaCallRefusalKind::Unbound => "formula_unbound_name",',
  'FormulaCallRefusalKind::Unbound => "formula_unknown",'),
 ('envelope token changed',
  'FormulaCallRefusalKind::Nurbs => "env_nurbs",',
  'FormulaCallRefusalKind::Nurbs => "formula_unsupported",'),
 ('source order changed',
  'FormulaCallRefusalKind::Unbound => &[Envelope, BuiltinCatalog],',
  'FormulaCallRefusalKind::Unbound => &[BuiltinCatalog, Envelope],'),
 ('envelope invents search', '&[Envelope]\n            }', '&[Envelope, BuiltinCatalog]\n            }'),
 ('curve alternative omitted',
  'FormulaCallRefusalKind::Nurbs => &[LineSegment, CircularArc, CubicBezier],',
  'FormulaCallRefusalKind::Nurbs => &[LineSegment, CircularArc],'),
 ('recipe alternative wrong',
  'FormulaCallRefusalKind::SketchConstraints => &[OrderedConstructionRecipe],',
  'FormulaCallRefusalKind::SketchConstraints => &[LineSegment],'),
 ('unbound false replacement',
  'FormulaCallRefusalKind::Unbound => &[],',
  'FormulaCallRefusalKind::Unbound => &[OrderedConstructionRecipe],'),
 ('scope conflated', '        "formula_call"', '        "data_name"'),
 ('source metadata tag changed',
  'Self::BuiltinCatalog => "builtin_catalog",',
  'Self::BuiltinCatalog => "recipe",'),
 ('alternative metadata tag changed',
  'Self::CircularArc => "circular_arc",',
  'Self::CircularArc => "nurbs",'),
 ('Debug discloses authored callee',
  '.field("kind", &self.kind)',
  '.field("kind", &self.kind).field("name", &self.name)'),
 ('Display discloses authored callee', 'f.write_str(self.token())', 'f.write_str(self.name)'))
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
    print('call lookup fault controls:18 unique actual anchors; name/expect/compiler noise refused')
    sys.exit(0)
environment=dict(os.environ,CARGO_HOME=str(ROOT/'target/cargo-home'),
                 CARGO_TARGET_DIR=str(ROOT/'target'),TMPDIR=str(ROOT/'target/scratch'))
try:
    for index,(name,before,after) in enumerate(CASES,1):
        SOURCE.write_text(original.decode().replace(before,after,1))
        result=subprocess.run(['cargo','test','-p','sc-core','--test','formula_call_lookup_contract'],
                              cwd=ROOT,env=environment,capture_output=True)
        output=result.stdout+result.stderr
        (WORK/('fault-%d.log'%index)).write_bytes(output)
        assert result.returncode==101 and b'test result: FAILED.' in output,(name,output.decode())
        assert assertion_failure(output) and b'could not compile' not in output,(name,output.decode())
        print('call lookup fault %d %s:rc=101 actual compiled body assertion red'%(index,name))
        SOURCE.write_bytes(original)
finally:
    SOURCE.write_bytes(original)
assert SOURCE.read_bytes()==original
print('call lookup faults:18 actual compiled assertion reds; exact source restored')
