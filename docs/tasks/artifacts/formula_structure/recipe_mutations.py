"""Exclusive actual recipe/statement faults; compiled assertion reds and exact restoration required."""
from pathlib import Path
import os
import re
import runpy
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
ORDERED = ROOT / 'crates/sc-core/src/recipe/ordered.rs'
STATEMENT = ROOT / 'crates/sc-core/src/recipe/statement.rs'
WORK = ROOT / 'target/recipe_mutations'
TEST_SOURCE = ROOT / 'crates/sc-core/tests/formula_recipe_contract.rs'
FIXTURE_SOURCES = tuple(ROOT / ('docs/tasks/artifacts/formula_structure/' + name) for name in (
    'recipe_cases.tsv', 'recipe_coupled_cases.tsv', 'canonical_worked_cases.tsv'))
BOOK_SOURCE = ROOT / 'docs/book/src/spec/formula-language/examples.md'
MUTABLE_INPUTS = (ORDERED, STATEMENT)
SOURCES = (*MUTABLE_INPUTS, TEST_SOURCE, *FIXTURE_SOURCES, BOOK_SOURCE)
assert sys.argv[1:] in [[], ['--classifier-only'], ['--coupled']]
CLASSIFIER_ONLY = sys.argv[1:] == ['--classifier-only']
COUPLED = sys.argv[1:] == ['--coupled']
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
coupled_cases = [
    ('unrequired boundary whitespace', STATEMENT,
     'recipe_boundary && depth == 0 && matches!(next.kind(), K::Let | K::Assert)',
     'recipe_boundary && depth == 0 && next.span().start() > end && matches!(next.kind(), K::Let | K::Assert)',
     'coupled_token_boundaries_preserve_zero_gap_and_all_whitespace'),
    ('later ordinal flattened', ORDERED,
     '                } else {\n                    Some(index)\n                };',
     '                } else {\n                    Some(2)\n                };',
     'coupled_later_header_and_operand_rules_keep_exact_global_provenance'),
    ('nested rule substituted', ORDERED,
     'rule: FormulaRecipeRule::Statement(error),',
     'rule: FormulaRecipeRule::Statement(match FormulaStatement::parse("invalid") { Err(other) => other, Ok(_) => error }),',
     'coupled_later_header_and_operand_rules_keep_exact_global_provenance'),
    ('header EOF origin', STATEMENT,
     'header(lexer, source.len(), K::Colon, R::ExpectedColon)?',
     'header(lexer, 0, K::Colon, R::ExpectedColon)?',
     'coupled_later_header_and_operand_rules_keep_exact_global_provenance'),
    ('combined maximum lost', ORDERED,
     'statements.len() == Self::MAX_STATEMENTS', 'statements.len() == Self::MAX_STATEMENTS - 1',
     'coupled_all_statement_and_expression_limits_reach_their_maximum_together'),
]
OUTPUT_FILES = (() if CLASSIFIER_ONLY else
                tuple(WORK / ('%s-fault-%d.log' % ('coupled' if COUPLED else 'recipe', index))
                      for index in range(1, len(coupled_cases if COUPLED else cases) + 1)) +
                (WORK / 'restored.log',))
MUTABLE_SOURCES = () if CLASSIFIER_ONLY else MUTABLE_INPUTS

def assertion_sites(source, path):
    """Current curated test macros, with Rust panic byte columns."""
    sites = set()
    for number, line in enumerate(source.splitlines(), 1):
        match = re.match(r'^(\s*)assert(?:_eq|_ne)?!\s*\(', line)
        if match is not None:
            sites.add((path, number, len(match.group(1).encode()) + 1))
    return sites

def assertion_failure(output):
    """A failed-body panic must name a guarded current assertion macro location."""
    parts = output.split(b'\nfailures:\n', 2)
    if len(parts) != 3:
        return False
    for match in re.finditer(
            rb'(?m)^thread\b[^\r\n]*? panicked at ([^:\r\n]+):([0-9]+):([0-9]+):\r?$', parts[1]):
        try:
            path = match.group(1).decode('utf-8')
        except UnicodeDecodeError:
            continue
        if (path, int(match.group(2)), int(match.group(3))) in ASSERTION_SITES:
            return True
    return False

runpy.run_path(str(ROOT / 'scripts/local_environment.py'))['enter_producer'](
    ROOT, directories=(WORK,), output_files=OUTPUT_FILES, sources=SOURCES, mutable_sources=MUTABLE_SOURCES)
originals = {path: path.read_bytes() for path in MUTABLE_INPUTS}
WORK.mkdir(parents=True, exist_ok=True)
ASSERTION_SITES = assertion_sites(TEST_SOURCE.read_text(), TEST_SOURCE.relative_to(ROOT).as_posix())
assert ASSERTION_SITES, 'current recipe contract has no assertion macros'
for name, path, before, _ in cases + [case[:4] for case in coupled_cases]:
    assert originals[path].decode().count(before) == 1, (name, 'actual source anchor not unique')

site = min(ASSERTION_SITES)
location = ("\nfailures:\nthread 'fixture' panicked at %s:%d:%d:\n" % site).encode()
assert assertion_failure(location + b'custom macro message\n\nfailures:\nfixture\n')
assert not assertion_failure(b'test assertion_name ... ok\n\nfailures:\nthread panicked:\nassertion: expect-only\n\nfailures:\nassertion_name\n')
assert not assertion_failure(b'assertion: compiler output\n')
if CLASSIFIER_ONLY:
    print('recipe fault controls:20 uniquely matched actual fault anchors; current macro sites accepted; passing-name/labelled-expect/compiler noise refused')
    sys.exit(0)
selected = coupled_cases if sys.argv[1:] == ['--coupled'] else [(*case, '') for case in cases]
label = 'coupled' if sys.argv[1:] == ['--coupled'] else 'recipe'
environment = dict(os.environ)
try:
    for index, (name, path, before, after, test) in enumerate(selected, 1):
        path.write_text(originals[path].decode().replace(before, after))
        result = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test', 'formula_recipe_contract'] + ([test] if test else []),
                                cwd=ROOT, env=environment, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('%s-fault-%d.log' % (label, index))).write_bytes(output)
        assert result.returncode == 101 and b'test result: FAILED.' in output, (name, output.decode())
        assert assertion_failure(output) and b'could not compile' not in output, (name, output.decode())
        print('%s fault %d %s:rc=101 actual compiled assertion red in %s' % (label, index, name, test or 'whole suite'))
        path.write_bytes(originals[path])
finally:
    for path, content in originals.items():
        path.write_bytes(content)
    restored = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test', 'formula_recipe_contract'],
                              cwd=ROOT, env=environment, capture_output=True)
    (WORK / 'restored.log').write_bytes(restored.stdout + restored.stderr)
    assert restored.returncode == 0, ('restored recipe artifact failed', restored.stderr.decode())
assert all(path.read_bytes() == content for path, content in originals.items())
print('%s faults:%d actual compiled assertion reds; both sources and current artifact restored' % (label, len(selected)))
