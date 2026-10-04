"""Exclusive actual ordered-name faults; public body reds and exact two-source restoration."""
from pathlib import Path
import os
import re
import runpy
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
PATHS = ('crates/sc-core/src/recipe/namespace.rs', 'crates/sc-core/src/recipe/namespace/ordered.rs')
CONTRACT = ROOT / 'crates/sc-core/tests/formula_ordered_names_contract.rs'
SOURCES = (*(ROOT / path for path in PATHS), CONTRACT)
WORK = ROOT / 'target/ordered_name_mutations'
runpy.run_path(str(ROOT / 'scripts/local_environment.py'))['enter_producer'](
    ROOT, directories=(WORK,), sources=SOURCES)
ORIGINAL = {path: (ROOT / path).read_bytes() for path in PATHS}
WORK.mkdir(parents=True, exist_ok=True)
CASES = (('skipped first statement',
  'crates/sc-core/src/recipe/namespace/ordered.rs',
  '            position: 0,',
  '            position: 1,'),
 ('future names predeclared',
  'crates/sc-core/src/recipe/namespace/ordered.rs',
  '    pub const fn new(\n'
  "        namespace: FormulaNamespace<'a>,\n"
  "        recipe: &'a FormulaNormalizedRecipe<'a>,\n"
  '    ) -> Self {\n'
  '        Self {',
  '    pub fn new(\n'
  "        mut namespace: FormulaNamespace<'a>,\n"
  "        recipe: &'a FormulaNormalizedRecipe<'a>,\n"
  '    ) -> Self {\n'
  '        for ordinal in 1..=recipe.statements().len() {\n'
  '            if let Some(d) = FormulaDeclaration::recipe(recipe, ordinal) { '
  'namespace.entries.insert(d.name(), d); }\n'
  '        }\n'
  '        Self {'),
 ('wrong source ordinal',
  'crates/sc-core/src/recipe/namespace/ordered.rs',
  'let statement_index = self.position + 1;',
  'let statement_index = self.position + 2;'),
 ('header collision ignored',
  'crates/sc-core/src/recipe/namespace/ordered.rs',
  'self.namespace.entries.get(attempted.name())',
  'self.namespace.entries.get("absent_collision_key")'),
 ('recipe collision misclassified',
  'crates/sc-core/src/recipe/namespace/ordered.rs',
  'FormulaNamespaceError::RecipeRebinding {',
  'FormulaNamespaceError::AmbiguousName {'),
 ('earlier binding lost',
  'crates/sc-core/src/recipe/namespace/ordered.rs',
  'FormulaNamespaceError::RecipeRebinding {\n'
  '                            sources: Box::new([*prior, attempted]),\n'
  '                        }',
  'FormulaNamespaceError::RecipeRebinding {\n'
  '                            sources: Box::new([attempted, attempted]),\n'
  '                        }'),
 ('reserved source changed',
  'crates/sc-core/src/recipe/namespace/ordered.rs',
  'FormulaNamespaceError::ReservedBinding {\n                            reserved,',
  'FormulaNamespaceError::ReservedBinding {\n'
  '                            reserved: super::FormulaReservedName::SizeIndex,'),
 ('wrong metadata binding',
  'crates/sc-core/src/recipe/namespace/ordered.rs',
  'if let Some(declaration) = FormulaDeclaration::recipe(self.recipe, statement_index) {',
  'if let Some(declaration) = FormulaDeclaration::recipe(self.recipe, statement_index + 1) {'),
 ('binding source replaced',
  'crates/sc-core/src/recipe/namespace/ordered.rs',
  '.insert(declaration.name(), declaration);',
  '.insert(declaration.name(), '
  'FormulaDeclaration::reserved(super::FormulaReservedName::SizeIndex));'),
 ('advance skips position',
  'crates/sc-core/src/recipe/namespace/ordered.rs',
  'self.position += 1;',
  'self.position += 2;'),
 ('failed advance changes prefix', 'crates/sc-core/src/recipe/namespace/ordered.rs', '        let Some(current) = self.current()? else {\n            return Ok(false);\n        };', '        if let Some(error) = self.current().err() { self.position += 1; return Err(error); }\n        let Some(current) = self.current()? else {\n            return Ok(false);\n        };'),
 ('assertion fails to advance',
  'crates/sc-core/src/recipe/namespace/ordered.rs',
  '        self.position += 1;',
  '        if FormulaDeclaration::recipe(self.recipe, statement_index).is_none() { return '
  'Ok(true); }\n'
  '        self.position += 1;'),
 ('scope ordinal wrong',
  'crates/sc-core/src/recipe/namespace/ordered.rs',
  '        self.statement_index\n',
  '        self.statement_index + 1\n'),
 ('wrong owner statement',
  'crates/sc-core/src/recipe/namespace/ordered.rs',
  '            statement,\n',
  '            statement: self.recipe.statements().first().unwrap(),\n'),
 ('cursor debug payload',
  'crates/sc-core/src/recipe/namespace/ordered.rs',
  'f.debug_struct("FormulaNameCursor")',
  'f.debug_struct("FormulaNameCursor").field("names", &self.recipe.statements().iter().map(|s| '
  's.name()).collect::<Vec<_>>())'),
 ('scope debug payload',
  'crates/sc-core/src/recipe/namespace/ordered.rs',
  'f.debug_struct("FormulaStatementNameScope")',
  'f.debug_struct("FormulaStatementNameScope").field("name", &self.statement.name())'),
 ('recipe token changed',
  'crates/sc-core/src/recipe/namespace.rs',
  'Self::RecipeRebinding { .. } => "formula_rebinding",',
  'Self::RecipeRebinding { .. } => "formula_ambiguous_name",'),
 ('recipe source arguments swapped',
  'crates/sc-core/src/recipe/namespace.rs',
  'Self::RecipeRebinding { sources } => **sources,',
  'Self::RecipeRebinding { sources } => [sources[1], sources[0]],'))
for name, path, before, _ in CASES:
    assert ORIGINAL[path].decode().count(before) == 1, (name, 'actual fault anchor not unique')

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
    print('ordered name fault controls:18 unique actual anchors; name/expect/compiler noise refused')
    sys.exit(0)
environment = dict(os.environ)
try:
    for index, (name, path, before, after) in enumerate(CASES, 1):
        (ROOT / path).write_text(ORIGINAL[path].decode().replace(before, after, 1))
        result = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test',
                                 'formula_ordered_names_contract'],
                                cwd=ROOT, env=environment, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('fault-%d.log' % index)).write_bytes(output)
        assert result.returncode == 101 and b'test result: FAILED.' in output, (name, output.decode())
        assert assertion_failure(output) and b'could not compile' not in output, (name, output.decode())
        print('ordered_name fault %d %s:rc=101 actual compiled body assertion red' % (index, name), flush=True)
        (ROOT / path).write_bytes(ORIGINAL[path])
finally:
    for path, data in ORIGINAL.items():
        (ROOT / path).write_bytes(data)
    restored = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test',
                               'formula_ordered_names_contract'],
                              cwd=ROOT, env=environment, capture_output=True)
    (WORK / 'restored.log').write_bytes(restored.stdout + restored.stderr)
    assert restored.returncode == 0, ('restored ordered_name artifact failed', restored.stderr.decode())
assert all((ROOT / path).read_bytes() == data for path, data in ORIGINAL.items())
print('ordered name faults:18 actual compiled assertion reds; both sources/current artifact restored')
