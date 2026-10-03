"""Exclusive actual declaration faults; compiled body assertion reds and exact restore required."""
from pathlib import Path
import os
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'crates/sc-core/src/recipe/declaration.rs'
original = SOURCE.read_bytes()
WORK = ROOT / 'target/declaration_mutations'
WORK.mkdir(parents=True, exist_ok=True)
CASES = (
    ('scalar source origin', 'FormulaScalarInputOrigin::Material => Self::Material,', 'FormulaScalarInputOrigin::Material => Self::Profile,'),
    ('input origin', 'FormulaInputOrigin::Ease => Self::Ease,', 'FormulaInputOrigin::Ease => Self::Parameter,'),
    ('scalar annotations', 'Self::Input { kind, .. } | Self::Recipe { kind, .. } => kind.into(),',
     'Self::Input { .. } | Self::Recipe { .. } => FormulaKind::Length,'),
    ('length availability read', 'Self::LengthInput { .. } => FormulaKind::Length,',
     'Self::LengthInput { declaration, .. } => if declaration.authored_value().is_ok() { FormulaKind::Length } else { FormulaKind::Count },'),
    ('point kind', 'Self::Point(_) => FormulaKind::Point,', 'Self::Point(_) => FormulaKind::Edge,'),
    ('input identities aliased', 'source: FormulaDeclarationSource::Input {\n                origin,\n                input,\n                declaration,\n                kind,',
     'source: FormulaDeclarationSource::Input {\n                origin,\n                input: declaration,\n                declaration,\n                kind,'),
    ('canonical record cloned', 'source: FormulaDeclarationSource::LengthInput {\n                origin,\n                input,\n                declaration,\n            },',
     'source: FormulaDeclarationSource::LengthInput { origin, input, declaration: Box::leak(Box::new(LengthDeclaration::clone(declaration))) },'),
    ('point tag lost', 'FormulaDeclarationSource::Point(point)',
     'FormulaDeclarationSource::Point(PointRef::new(point.creator(), crate::ontology::LocalTag::FIRST))'),
    ('name changed', '        self.name\n', '        "fixed"\n'),
    ('recipe ordinal', 'source: FormulaDeclarationSource::Recipe {\n                statement_index,',
     'source: FormulaDeclarationSource::Recipe {\n                statement_index: statement_index + 1,'),
    ('assertion label declared', 'let FormulaNormalizedStatementKind::Let { declared_kind, .. } = statement.kind() else {\n            return None;\n        };',
     'let FormulaNormalizedStatementKind::Let { declared_kind, .. } = statement.kind() else {\n            return Some(Self { name: statement.name(), source: FormulaDeclarationSource::Input { origin: FormulaScalarInputOrigin::Parameter, input: EntityId::from_bits(0), declaration: EntityId::from_bits(0), kind: FormulaBindingKind::Count } });\n        };'),
    ('statement span', 'span: statement.span(),', 'span: statement.name_span(),'),
    ('name span', 'name_span: statement.name_span(),', 'name_span: statement.span(),'),
    ('reserved name', 'name: name.token(),', 'name: "eps_num",'),
    ('source debug payload', 'f.debug_struct("FormulaDeclarationSource")',
     'f.debug_struct("FormulaDeclarationSource").field("payload", &match self { FormulaDeclarationSource::LengthInput { declaration, .. } => Some(declaration), _ => None })'),
    ('declaration debug name', 'f.debug_struct("FormulaDeclaration")',
     'f.debug_struct("FormulaDeclaration").field("name", &self.name)'),
    ('zero ordinal accepted', 'statement_index.checked_sub(1)?', 'statement_index.saturating_sub(1)'),
    ('reserved origin', 'Self::Reserved(name) => name.origin(),', 'Self::Reserved(_) => FormulaOrigin::Size,'),
    ('recipe origin', 'Self::Recipe { .. } => FormulaOrigin::Recipe,', 'Self::Recipe { .. } => FormulaOrigin::Geometry,'),
)
assert original.decode().count('origin: FormulaScalarInputOrigin,') == 2, 'actual scalar domain boundaries changed'
for name, before, _ in CASES:
    assert original.decode().count(before) == 1, (name, 'actual fault anchor not unique')

def assertion_failure(output):
    parts = output.split(b'\nfailures:\n', 2)
    return len(parts) == 3 and re.search(rb'(?m)^assertion(?:[ :`]|$)', parts[1]) is not None

assert assertion_failure(b'\nfailures:\nthread panicked:\nassertion `left == right` failed\n\nfailures:\nfixture\n')
assert not assertion_failure(b'test assertion_name ... ok\n\nfailures:\nthread panicked:\nexpect-only\n\nfailures:\nassertion_name\n')
assert not assertion_failure(b'assertion: compiler output\n')
assert sys.argv[1:] in [[], ['--classifier-only']]
if sys.argv[1:] == ['--classifier-only']:
    print('declaration fault controls:19 unique body-fault anchors/two scalar domain boundaries; failed-name/expect-only/compiler noise refused')
    sys.exit(0)
environment = dict(os.environ, CARGO_HOME=str(ROOT / 'target/cargo-home'),
                   CARGO_TARGET_DIR=str(ROOT / 'target'), TMPDIR=str(ROOT / 'target/scratch'))
try:
    for index, (name, before, after) in enumerate(CASES, 1):
        SOURCE.write_text(original.decode().replace(before, after))
        result = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test', 'formula_declaration_contract'],
                                cwd=ROOT, env=environment, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('fault-%d.log' % index)).write_bytes(output)
        assert result.returncode == 101 and b'test result: FAILED.' in output, (name, output.decode())
        assert assertion_failure(output) and b'could not compile' not in output, (name, output.decode())
        print('declaration fault %d %s:rc=101 actual compiled body assertion red' % (index, name))
        SOURCE.write_bytes(original)
    SOURCE.write_text(original.decode().replace('origin: FormulaScalarInputOrigin,', 'origin: FormulaInputOrigin,'))
    result = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--doc', 'recipe::declaration::FormulaDeclaration'],
                            cwd=ROOT, env=environment, capture_output=True)
    output = result.stdout + result.stderr
    (WORK / 'scalar-domain-compile-fail.log').write_bytes(output)
    assert result.returncode == 101 and b'test result: FAILED.' in output, output.decode()
    assert b'Test compiled successfully' in output and b'compile_fail' in output, output.decode()
    assert b'could not compile' not in output, output.decode()
    print('scalar domain widening:actual library compiles; intended negative construction contract fails because the forbidden call compiles')
finally:
    SOURCE.write_bytes(original)
assert SOURCE.read_bytes() == original
print('declaration faults:19 actual compiled assertion reds/one negative construction contract red; exact source restored')
