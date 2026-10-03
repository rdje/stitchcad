"""Exclusive namespace faults: compiled public assertion reds and exact byte restoration."""
from pathlib import Path
import os
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'crates/sc-core/src/recipe/namespace.rs'
original = SOURCE.read_bytes()
WORK = ROOT / 'target/namespace_mutations'
WORK.mkdir(parents=True, exist_ok=True)
CASES = (
    ('recipe admission', 'Err(declaration)\n            }', 'Ok(Self(declaration))\n            }'),
    ('initial projection', '        self.0\n', '        FormulaDeclaration::reserved(FormulaReservedName::SizeIndex)\n'),
    ('availability read', '        match declaration.source() {',
     '        if let FormulaDeclarationSource::LengthInput { declaration: record, .. } = declaration.source() { if record.authored_value().is_err() { return Err(declaration); } }\n        match declaration.source() {'),
    ('reserved population', 'for reserved in FormulaReservedName::ALL {', 'for reserved in FormulaReservedName::ALL.into_iter().take(7) {'),
    ('optional context hidden', '            let declaration = FormulaDeclaration::reserved(reserved);',
     '            if reserved.required_context() != super::FormulaReservedContext::Always { continue; }\n            let declaration = FormulaDeclaration::reserved(reserved);'),
    ('authored value replaced', 'slot.insert(attempted);', 'slot.insert(FormulaDeclaration::reserved(FormulaReservedName::SizeIndex));'),
    ('collision accepted', '                Entry::Occupied(slot) => {', '                Entry::Occupied(mut slot) => { slot.insert(attempted); continue;'),
    ('authored order reversed', 'for initial in declarations {', 'for initial in declarations.into_iter().collect::<Vec<_>>().into_iter().rev() {'),
    ('reserved token', 'Self::ReservedBinding { .. } => "formula_rebinding",', 'Self::ReservedBinding { .. } => "formula_ambiguous_name",'),
    ('ambiguity token', 'Self::AmbiguousName { .. } => "formula_ambiguous_name",', 'Self::AmbiguousName { .. } => "formula_rebinding",'),
    ('first collision source lost', 'sources: Box::new([*slot.get(), attempted]),', 'sources: Box::new([attempted, attempted]),'),
    ('source argument order swapped', 'Self::AmbiguousName { sources } => **sources,', 'Self::AmbiguousName { sources } => [sources[1], sources[0]],'),
    ('reserved source changed', 'FormulaDeclarationSource::Reserved(reserved) => {',
     'FormulaDeclarationSource::Reserved(_) => { let reserved = FormulaReservedName::SizeIndex;'),
    ('attempted source changed', '} => [FormulaDeclaration::reserved(*reserved), *attempted],',
     '} => [FormulaDeclaration::reserved(*reserved), FormulaDeclaration::reserved(*reserved)],'),
    ('inspection order', 'self.entries.values().copied()', 'self.entries.values().rev().copied()'),
    ('namespace debug payload', 'f.debug_struct("FormulaNamespace")', 'f.debug_struct("FormulaNamespace").field("entries", &self.entries)'),
    ('display payload', "impl fmt::Display for FormulaNamespaceError<'_> {\n    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {\n        f.write_str(self.token())", 'impl fmt::Display for FormulaNamespaceError<\'_> {\n    fn fmt(&self, f: &mut fmt::Formatter<\'_>) -> fmt::Result {\n        write!(f, "{}: {}", self.token(), self.name())'),
)
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
    print('namespace fault controls:17 unique actual anchors; name/expect/compiler noise refused')
    sys.exit(0)
environment=dict(os.environ,CARGO_HOME=str(ROOT/'target/cargo-home'),
                 CARGO_TARGET_DIR=str(ROOT/'target'),TMPDIR=str(ROOT/'target/scratch'))
try:
    for index,(name,before,after) in enumerate(CASES,1):
        SOURCE.write_text(original.decode().replace(before,after,1))
        result=subprocess.run(['cargo','test','-p','sc-core','--test','formula_namespace_contract'],
                              cwd=ROOT,env=environment,capture_output=True)
        output=result.stdout+result.stderr
        (WORK/('fault-%d.log'%index)).write_bytes(output)
        assert result.returncode==101 and b'test result: FAILED.' in output,(name,output.decode())
        assert assertion_failure(output) and b'could not compile' not in output,(name,output.decode())
        print('namespace fault %d %s:rc=101 actual compiled body assertion red'%(index,name))
        SOURCE.write_bytes(original)
finally:
    SOURCE.write_bytes(original)
assert SOURCE.read_bytes()==original
print('namespace faults:17 actual compiled assertion reds; exact source restored')
