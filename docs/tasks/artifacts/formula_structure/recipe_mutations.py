"""Exclusive actual recipe/statement faults; compiled assertion reds and exact restoration required."""
from pathlib import Path
import os
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
ORDERED = ROOT / 'crates/sc-core/src/recipe/ordered.rs'
STATEMENT = ROOT / 'crates/sc-core/src/recipe/statement.rs'
originals = {path: path.read_bytes() for path in [ORDERED, STATEMENT]}
WORK = ROOT / 'target/recipe_mutations'
WORK.mkdir(parents=True, exist_ok=True)
cases = [
    ('fixed bound', ORDERED, 'pub const MAX_STATEMENTS: usize = 4096;', 'pub const MAX_STATEMENTS: usize = 4097;'),
    ('early bound', ORDERED, 'statements.len() == Self::MAX_STATEMENTS', 'statements.len() == Self::MAX_STATEMENTS - 1'),
    ('missing bound', ORDERED, 'statements.len() == Self::MAX_STATEMENTS', 'statements.len() == usize::MAX'),
    ('statement ordinal', ORDERED, 'let index = statements.len() + 1;', 'let index = statements.len();'),
    ('global ASCII ordinal', ORDERED, '                    None\n                } else {', '                    Some(index)\n                } else {'),
    ('global refusal span', ORDERED, 'span: error.span(),', 'span: Span::new(0, 0),'),
    ('limit family', ORDERED, 'FormulaRecipeRule::StatementLimit { .. } => "formula_domain",',
     'FormulaRecipeRule::StatementLimit { .. } => "formula_parse",'),
    ('accepted prefix', ORDERED, 'statements.push(statement);', 'statements.push(statement); break;'),
    ('lost statements', ORDERED, 'statements.push(statement);', 'if statements.len() == usize::MAX { statements.push(statement); }'),
    ('authored order', ORDERED, 'Ok(Self { statements })', 'statements.reverse(); Ok(Self { statements })'),
    ('customer privacy', ORDERED, '.field("statement_count", &self.statements.len())',
     '.field("names", &self.statements.iter().map(FormulaStatement::name).collect::<Vec<_>>())'),
    ('statement boundary disabled', STATEMENT, 'recipe_boundary && depth == 0 && matches!(next.kind(), K::Let | K::Assert)',
     'false && depth == 0 && matches!(next.kind(), K::Let | K::Assert)'),
    ('nested keyword boundary', STATEMENT, 'recipe_boundary && depth == 0 && matches!(next.kind(), K::Let | K::Assert)',
     'recipe_boundary && matches!(next.kind(), K::Let | K::Assert)'),
    ('boundary operand range', STATEMENT, 'range_end = next.span().start();', 'range_end = source.len();'),
    ('statement span origin', STATEMENT, 'span: Span::new(head.span().start(), end),', 'span: Span::new(0, end),'),
]
for name, path, before, _ in cases:
    assert originals[path].decode().count(before) == 1, (name, 'actual source anchor not unique')

def assertion_failure(output):
    parts = output.split(b'\nfailures:\n', 2)
    return len(parts) == 3 and re.search(rb'(?m)^assertion(?:[ :`]|$)', parts[1]) is not None

assert not assertion_failure(b'test assertion_contract ... ok\n\nfailures:\nthread panicked:\nexpect-only\n\nfailures:\nassertion_contract\n')
assert assertion_failure(b'\nfailures:\nthread panicked:\nassertion `left == right` failed\n\nfailures:\nfixture\n')
assert not assertion_failure(b'assertion: compiler output\n')
assert not sys.argv[1:] or sys.argv[1:] == ['--classifier-only']
if sys.argv[1:] == ['--classifier-only']:
    print('recipe fault controls:15 unique actual source anchors; passing-name/expect-only/compiler noise refused')
    sys.exit(0)
environment = dict(os.environ, CARGO_HOME=str(ROOT / 'target/cargo-home'), TMPDIR=str(ROOT / 'target/scratch'))
try:
    for index, (name, path, before, after) in enumerate(cases, 1):
        path.write_text(originals[path].decode().replace(before, after))
        result = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test', 'formula_recipe_contract'],
                                cwd=ROOT, env=environment, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('fault-%d.log' % index)).write_bytes(output)
        assert result.returncode == 101 and b'test result: FAILED.' in output, (name, output.decode())
        assert assertion_failure(output) and b'could not compile' not in output, (name, output.decode())
        print('recipe fault %d %s:rc=101 actual compiled assertion red' % (index, name))
        path.write_bytes(originals[path])
finally:
    for path, content in originals.items():
        path.write_bytes(content)
assert all(path.read_bytes() == content for path, content in originals.items())
print('recipe faults:15 actual compiled assertion reds; both sources restored exactly')
