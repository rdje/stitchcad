"""Exclusive declaration faults: guarded sources, assertion sites and complete restoration."""
from pathlib import Path
import os
import re
import runpy
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'crates/sc-core/src/recipe/declaration.rs'
CONTRACT = ROOT / 'crates/sc-core/tests/formula_declaration_contract.rs'
SOURCES = (SOURCE, CONTRACT)
WORK = ROOT / 'target/declaration_mutations'
runpy.run_path(str(ROOT / 'scripts/local_environment.py'))['enter_producer'](
    ROOT, directories=(WORK,), sources=SOURCES)
original = SOURCE.read_bytes()
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

def assertion_sites(source, path):
    sites = set()
    for line_number, line in enumerate(source.splitlines(), 1):
        match = re.match(r'^(\s*)assert(?:_eq|_ne)?!\s*\(', line)
        if match is not None:
            sites.add((path, line_number, len(match.group(1).encode()) + 1))
    return sites


def assertion_failure(output):
    parts = output.split(b'\nfailures:\n', 2)
    if len(parts) != 3:
        return False
    locations = re.finditer(rb'panicked at ([^:\n]+):([0-9]+):([0-9]+):\n', parts[1])
    return any((match.group(1).decode('utf-8', 'replace'), int(match.group(2)), int(match.group(3)))
               in ASSERTION_SITES for match in locations)

ASSERTION_SITES = assertion_sites(CONTRACT.read_text(), CONTRACT.relative_to(ROOT).as_posix())
assert ASSERTION_SITES, 'focused contract has no assertion sites'
assert not assertion_failure(b'\nfailures:\nthread panicked:\nexpect-only\n\nfailures:\nassertion_name\n')
assert not assertion_failure(b'assertion: compiler output\n')

assert sys.argv[1:] in [[], ['--classifier-only']]
if sys.argv[1:] == ['--classifier-only']:
    print('declaration fault controls:19 unique actual anchors; name/expect/compiler noise refused')
    sys.exit(0)
environment = dict(os.environ)
try:
    for index, (name, before, after) in enumerate(CASES, 1):
        SOURCE.write_text(original.decode().replace(before, after, 1))
        result = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test',
                                 'formula_declaration_contract'],
                                cwd=ROOT, env=environment, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('fault-%d.log' % index)).write_bytes(output)
        assert result.returncode == 101 and b'test result: FAILED.' in output, (name, output.decode())
        assert assertion_failure(output) and b'could not compile' not in output, (name, output.decode())
        print('declaration fault %d %s:rc=101 actual compiled body assertion red' % (index, name), flush=True)
        SOURCE.write_bytes(original)
    SOURCE.write_text(original.decode().replace('origin: FormulaScalarInputOrigin,', 'origin: FormulaInputOrigin,'))
    result = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--doc',
                             'recipe::declaration::FormulaDeclaration'],
                            cwd=ROOT, env=environment, capture_output=True)
    output = result.stdout + result.stderr
    (WORK / 'scalar-domain-compile-fail.log').write_bytes(output)
    assert result.returncode == 101 and b'test result: FAILED.' in output, output.decode()
    assert b'Test compiled successfully' in output and b'compile_fail' in output, output.decode()
    assert b'could not compile' not in output, output.decode()
    print('scalar domain widening:actual library compiles; forbidden construction compiles and its negative contract fails', flush=True)
finally:
    SOURCE.write_bytes(original)
    try:
        restored = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test',
                                   'formula_declaration_contract'],
                                  cwd=ROOT, env=environment, capture_output=True)
        (WORK / 'restored.log').write_bytes(restored.stdout + restored.stderr)
        assert restored.returncode == 0, ('restored declaration artifact failed', restored.stderr.decode())
    finally:
        restored_docs = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--doc',
                                        'recipe::declaration::FormulaDeclaration'],
                                       cwd=ROOT, env=environment, capture_output=True)
        (WORK / 'restored-docs.log').write_bytes(restored_docs.stdout + restored_docs.stderr)
        assert restored_docs.returncode == 0, ('restored declaration doctests failed', restored_docs.stderr.decode())
assert SOURCE.read_bytes() == original
print('declaration faults:19 actual compiled assertion reds/one negative construction red; source/current native and doc artifacts restored')
