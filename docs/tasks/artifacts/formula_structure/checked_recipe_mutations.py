"""Exclusive compiled whole-proof faults; only running public body assertions earn a red."""
from pathlib import Path
import os
import re
import runpy
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'crates/sc-core/src/recipe/checked_recipe.rs'
CONTRACT = ROOT / 'crates/sc-core/tests/formula_checked_recipe_contract.rs'
SOURCES = (SOURCE, CONTRACT)
WORK = ROOT / 'target/checked_recipe_mutations'
runpy.run_path(str(ROOT / 'scripts/local_environment.py'))['enter_producer'](
    ROOT, directories=(WORK,), sources=SOURCES)
CASES = (
    ('accepted prefix', 'self.statements().iter().enumerate()', 'self.statements().iter().take(1).enumerate()', 1),
    ('initial metadata ignored', 'FormulaNameCursor::new(namespace, self)', 'FormulaNameCursor::new({ let _ = namespace; FormulaNamespace::new([]).unwrap() }, self)', 1),
    ('failed statement accepted', '''let proof = scope
                .check_kinds()
                .map_err(|error| refuse(FormulaRecipeCheckRefusal::Statement(error)))?;''',
     'let proof = match scope.check_kinds() { Ok(proof) => proof, Err(_) => break };', 1),
    ('metadata advance omitted', '''cursor
                .advance_metadata()
                .map_err(|error| refuse(FormulaRecipeCheckRefusal::Namespace(error)))?;''', '', 1),
    ('proof omitted', 'statements.push(proof);', 'let _ = proof;', 1),
    ('proof order reversed', '        Ok(FormulaCheckedRecipe {', '        statements.reverse();\n        Ok(FormulaCheckedRecipe {', 1),
    ('refusal position fabricated', 'let statement_index = position + 1;', 'let statement_index = position + 2;', 1),
    ('refusal statement replaced', '''let refuse = |refusal| FormulaRecipeCheckError {
                recipe: self,
                statement,''', '''let refuse = |refusal| FormulaRecipeCheckError {
                recipe: self,
                statement: self.statements().first().unwrap(),''', 1),
    ('collision span replaced', 'FormulaRecipeCheckRefusal::Namespace(_) => self.statement.name_span(),', 'FormulaRecipeCheckRefusal::Namespace(_) => self.statement.annotation_span(),', 1),
    ('statement span replaced', 'FormulaRecipeCheckRefusal::Statement(error) => error.span(),', 'FormulaRecipeCheckRefusal::Statement(_) => self.statement.name_span(),', 1),
    ('canonical owner fabricated', 'self.recipe.canonical_form()', 'super::FormulaRecipe::parse("let fabricated:count=1").unwrap().normalize_literals().unwrap().canonical_form()', 2),
    ('default proof disclosure', 'f.debug_struct("FormulaCheckedRecipe")', 'f.debug_struct("FormulaCheckedRecipe").field("name", &self.recipe.statements().first().map(|s| s.name()))', 1),
    ('header dependency omitted', 'Some(header),', 'None::<FormulaRecipeNameDependency>,', 1),
    ('header role replaced', 'role: R::AssertionTolerance,', 'role: R::AssertionLeft,', 1),
    ('header span replaced', 'span: proof.statement().annotation_span(),', 'span: proof.statement().name_span(),', 1),
    ('header class replaced', '                    tolerance,', '                    super::FormulaToleranceName::Numerical,', 1),
    ('operand source order reversed', '''(R::AssertionLeft, left.dependencies()),
                    (R::AssertionRight, right.dependencies()),''', '''(R::AssertionRight, right.dependencies()),
                    (R::AssertionLeft, left.dependencies()),''', 1),
    ('dependency repeats dropped', '''dependencies
                .iter()
                .copied()''', '''dependencies
                .iter()
                .take(1)
                .copied()''', 1),
    ('consumer position fabricated', 'statement_index: proof.statement_index(),', 'statement_index: proof.statement_index() + 1,', 2),
)
ORIGINAL = SOURCE.read_bytes()
WORK.mkdir(parents=True, exist_ok=True)
for name, before, _, count in CASES:
    assert ORIGINAL.decode().count(before) == count, (name, 'actual source anchor count')


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

assert sys.argv[1:] in ([], ['--classifier-only'])
print('whole-proof classifier: actual test macro location required; compile/link/expect/name noise refused', flush=True)
if sys.argv[1:] == ['--classifier-only']:
    sys.exit(0)
environment = dict(os.environ)
command = ['cargo', 'test', '-p', 'sc-core', '--test', 'formula_checked_recipe_contract']
try:
    baseline = subprocess.run(command, cwd=ROOT, env=environment, capture_output=True)
    (WORK / 'baseline.log').write_bytes(baseline.stdout + baseline.stderr)
    assert baseline.returncode == 0, 'whole-proof baseline failed'
    for index, (name, before, after, count) in enumerate(CASES, 1):
        SOURCE.write_text(ORIGINAL.decode().replace(before, after, count))
        result = subprocess.run(command, cwd=ROOT, env=environment, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('fault-%d.log' % index)).write_bytes(output)
        assert result.returncode == 101 and b'test result: FAILED.' in output, (name, output.decode())
        assert assertion_failure(output) and b'could not compile' not in output, (name, output.decode())
        print('whole-proof fault %d %s:rc=101 actual compiled body assertion red' % (index, name), flush=True)
        SOURCE.write_bytes(ORIGINAL)
finally:
    SOURCE.write_bytes(ORIGINAL)
    assert SOURCE.read_bytes() == ORIGINAL, 'whole-proof source not restored'
    restored = subprocess.run(command, cwd=ROOT, env=environment, capture_output=True)
    (WORK / 'restored.log').write_bytes(restored.stdout + restored.stderr)
    assert restored.returncode == 0, ('restored whole-proof artifact failed', restored.stderr.decode())
print('whole-proof faults:%d actual compiled body reds; source and actual compiled artifact restored; rc=0' % len(CASES))
