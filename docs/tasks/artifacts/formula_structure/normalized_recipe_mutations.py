"""Exclusive whole-input production faults; only compiled failed-body assertions count."""
from pathlib import Path
import os
import re
import runpy
import subprocess
import sys
ROOT = Path(__file__).resolve().parents[4]
STATEMENT = ROOT / 'crates/sc-core/src/recipe/statement.rs'
ORDERED = ROOT / 'crates/sc-core/src/recipe/ordered.rs'
NORMALIZED = ROOT / 'crates/sc-core/src/recipe/normalized_recipe.rs'
WORK = ROOT / 'target/normalized_recipe_mutations'
CONTRACT = ROOT / 'crates/sc-core/tests/formula_normalized_recipe_contract.rs'
MUTATED = (STATEMENT, ORDERED, NORMALIZED)
SOURCES = (*MUTATED, CONTRACT)
runpy.run_path(str(ROOT / 'scripts/local_environment.py'))['enter_producer'](
    ROOT, directories=(WORK,), sources=SOURCES)
originals = {p: p.read_bytes() for p in MUTATED}
WORK.mkdir(parents=True, exist_ok=True)
cases = [
 ('name retained', STATEMENT, 'name: self.name,', 'name: "scrubbed",'),
 ('name span retained', STATEMENT, 'name_span: self.name_span,', 'name_span: self.annotation_span,'),
 ('annotation span retained', STATEMENT, 'annotation_span: self.annotation_span,', 'annotation_span: self.name_span,'),
 ('statement span retained', STATEMENT, 'span: self.span,', 'span: self.name_span,'),
 ('declared kind retained', STATEMENT, 'D::Let(\n                *kind,', 'D::Let(\n                FormulaBindingKind::Boolean,'),
 ('tolerance retained', STATEMENT, 'D::Assert(\n                *tolerance,', 'D::Assert(\n                FormulaToleranceName::Format,'),
 ('right operand retained', STATEMENT, 'convert(right, FormulaStatementExpression::AssertionRight)?',
  'convert(left, FormulaStatementExpression::AssertionRight)?'),
 ('left error role', STATEMENT, 'convert(left, FormulaStatementExpression::AssertionLeft)?',
  'convert(left, FormulaStatementExpression::AssertionRight)?'),
 ('right error role', STATEMENT, 'convert(right, FormulaStatementExpression::AssertionRight)?',
  'convert(right, FormulaStatementExpression::AssertionLeft)?'),
 ('statement order', ORDERED, 'self.statements.iter().enumerate()', 'self.statements.iter().rev().enumerate()'),
 ('full count', ORDERED, 'self.statements.iter().enumerate()', 'self.statements.iter().take(4095).enumerate()'),
 ('later ordinal', ORDERED, 'statement_index: index + 1,', 'statement_index: 1,'),
 ('standalone error span', NORMALIZED, 'pub const fn span(self) -> Span {\n        self.error.span()',
  'pub const fn span(self) -> Span {\n        Span::new(0, 0)'),
 ('recipe error span', NORMALIZED, 'pub fn span(&self) -> Span {\n        self.error.span()',
  'pub fn span(&self) -> Span {\n        Span::new(0, 0)'),
 ('normalized privacy', NORMALIZED, '.field("span", &self.span)', '.field("name", &self.name)'),
 ('literal cause', NORMALIZED, 'Some(&self.error)', 'None'),
 ('statement cause', NORMALIZED, 'Some(self.error.as_ref())', 'None'),
]
for name, path, before, _ in cases:
    assert originals[path].decode().count(before) == 1, (name, 'actual anchor not unique')
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
    print('normalized recipe fault controls:17 unique actual anchors; current test assertion sites; passing/expect/compiler noise refused')
    sys.exit(0)
environment = dict(os.environ)
try:
    for index, (name, path, before, after) in enumerate(cases, 1):
        path.write_text(originals[path].decode().replace(before, after, 1))
        result = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test', 'formula_normalized_recipe_contract'],
                                cwd=ROOT, env=environment, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('fault-%d.log' % index)).write_bytes(output)
        assert result.returncode == 101 and b'test result: FAILED.' in output and assertion_failure(output), (name, output.decode())
        assert b'could not compile' not in output, (name, 'compiler refusal is not assertion proof')
        print('normalized recipe fault %d %s:rc=101 actual compiled assertion red' % (index, name), flush=True)
        path.write_bytes(originals[path])
finally:
    for path, content in originals.items():
        path.write_bytes(content)
    restored = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test', 'formula_normalized_recipe_contract'],
                              cwd=ROOT, env=environment, capture_output=True)
    (WORK / 'restored.log').write_bytes(restored.stdout + restored.stderr)
    assert restored.returncode == 0, ('restored normalized recipe artifact failed', restored.stderr.decode())
assert all(p.read_bytes() == content for p, content in originals.items())
print('normalized recipe faults:17 actual compiled assertion reds; all three sources/current artifact restored')
