"""Exclusive exact-name read faults; actual compiled assertion reds with byte restoration."""
from pathlib import Path
import os
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'crates/sc-core/src/recipe/namespace.rs'
original = SOURCE.read_bytes()
WORK = ROOT / 'target/name_read_mutations'
WORK.mkdir(parents=True, exist_ok=True)
CASES = (('value availability read',
  '        self.entries\n'
  '            .get(name)\n'
  '            .copied()\n'
  '            .ok_or(FormulaUnboundName { name })',
  '        if let Some(d) = self.entries.get(name) { if let FormulaDeclarationSource::LengthInput '
  '{ declaration, .. } = d.source() { if declaration.authored_value().is_err() { return '
  'Err(FormulaUnboundName { name }); } } }\n'
  '        self.entries\n'
  '            .get(name)\n'
  '            .copied()\n'
  '            .ok_or(FormulaUnboundName { name })'),
 ('wrong declaration', '.get(name)', '.get("size_index")'),
 ('implicit name repair', '.get(name)', '.get(name.strip_prefix("missing_").unwrap_or(name))'),
 ('missing name fallback',
  '.ok_or(FormulaUnboundName { name })',
  '.or_else(|| self.entries.get("size_index").copied()).ok_or(FormulaUnboundName { name })'),
 ('searched domains',
  '&FormulaOrigin::ALL',
  '&[FormulaOrigin::Measurement, FormulaOrigin::Ease, FormulaOrigin::Parameter, '
  'FormulaOrigin::Profile, FormulaOrigin::Material, FormulaOrigin::Geometry, FormulaOrigin::Size, '
  'FormulaOrigin::Tolerance, FormulaOrigin::Tolerance]'),
 ('unbound token', '        "formula_unbound_name"', '        "formula_unknown"'),
 ('query lost',
  "pub const fn name(self) -> &'a str {\n        self.name",
  'pub const fn name(self) -> &\'a str {\n        "different_query"'),
 ('debug payload',
  'f.debug_struct("FormulaUnboundName")',
  'f.debug_struct("FormulaUnboundName").field("name", &self.name)'),
 ('display payload',
  "impl fmt::Display for FormulaUnboundName<'_> {\n"
  "    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {\n"
  '        f.write_str(self.token())',
  "impl fmt::Display for FormulaUnboundName<'_> {\n"
  "    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {\n"
  '        write!(f, "{}: {}", self.token(), self.name())'))
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
    print('name read fault controls:9 unique actual anchors; name/expect/compiler noise refused')
    sys.exit(0)
environment=dict(os.environ,CARGO_HOME=str(ROOT/'target/cargo-home'),
                 CARGO_TARGET_DIR=str(ROOT/'target'),TMPDIR=str(ROOT/'target/scratch'))
try:
    for index,(name,before,after) in enumerate(CASES,1):
        SOURCE.write_text(original.decode().replace(before,after,1))
        result=subprocess.run(['cargo','test','-p','sc-core','--test','formula_name_read_contract'],
                              cwd=ROOT,env=environment,capture_output=True)
        output=result.stdout+result.stderr
        (WORK/('fault-%d.log'%index)).write_bytes(output)
        assert result.returncode==101 and b'test result: FAILED.' in output,(name,output.decode())
        assert assertion_failure(output) and b'could not compile' not in output,(name,output.decode())
        print('name read fault %d %s:rc=101 actual compiled body assertion red'%(index,name))
        SOURCE.write_bytes(original)
finally:
    SOURCE.write_bytes(original)
assert SOURCE.read_bytes()==original
print('name read faults:9 actual compiled assertion reds; exact source restored')
