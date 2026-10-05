"""Exclusive actual statement faults; compiled assertion reds and exact final restoration required."""
from pathlib import Path
import os
import re
import runpy
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'crates/sc-core/src/recipe/statement.rs'
WORK = ROOT / 'target/statement_mutations'
TEST_SOURCE = ROOT / 'crates/sc-core/tests/formula_statement_contract.rs'
FIXTURE_SOURCES = tuple(ROOT / ('docs/tasks/artifacts/formula_structure/' + name) for name in (
    'statement_cases.tsv', 'canonical_worked_cases.tsv'))
BOOK_SOURCE = ROOT / 'docs/book/src/spec/formula-language/examples.md'
SOURCES = (SOURCE, TEST_SOURCE, *FIXTURE_SOURCES, BOOK_SOURCE)
assert sys.argv[1:] in [[], ['--classifier-only']]
CLASSIFIER_ONLY = sys.argv[1:] == ['--classifier-only']
cases = [
    ('lexical operand role', 'if binding_kind.is_some()', 'if binding_kind.is_none()'),
    ('assert keyword', 'matches!(token.kind(), K::Let | K::Assert)', 'token.kind() == K::Let'),
    ('reserved name', 'Ok(Some(token)) if token.kind() == kind => Ok(token),',
     'Ok(Some(token)) if token.kind() == kind || rule == FormulaStatementRule::ExpectedName => Ok(token),'),
    ('colon role', 'Ok(Some(token)) if token.kind() == kind => Ok(token),',
     'Ok(Some(token)) if token.kind() == kind || kind == K::Colon => Ok(token),'),
    ('area annotation', '"area" => Some(Self::Area),', '"area" => None,'),
    ('physical tolerance', '"eps_phys" => Some(Self::Physical),', '"eps_phys" => Some(Self::Format),'),
    ('kind diagnostic', 'FormulaStatementRule::UnbindableKind => "formula_dimension",',
     'FormulaStatementRule::UnbindableKind => "formula_parse",'),
    ('tolerance diagnostic', 'FormulaStatementRule::UnknownTolerance => "formula_parse",',
     'FormulaStatementRule::UnknownTolerance => "formula_tolerance_unbound",'),
    ('assignment role', 'K::Assign, R::ExpectedAssignment', 'K::Equal, R::ExpectedAssignment'),
    ('nested separator', 'head.kind() == K::Assert && depth == 0', 'head.kind() == K::Assert'),
    ('second separator', 'if current.is_some()', 'if current.is_some() && false'),
    ('left/right coverage', 'Span::new(separator.end(), range_end),',
     'Span::new(assignment.span().end(), separator.start()),'),
    ('node span origin', 'range.start() + node.span.start(),', 'node.span.start(),'),
    ('error span origin', 'range.start() + error.span.start(),', 'error.span.start(),'),
    ('statement customer privacy', '.field("span", &self.span)', '.field("name", &self.name)'),
]
OUTPUT_FILES = (() if CLASSIFIER_ONLY else
                tuple(WORK / ('fault-%d.log' % index) for index in range(1, len(cases) + 1)) +
                (WORK / 'restored.log',))
MUTABLE_SOURCES = () if CLASSIFIER_ONLY else (SOURCE,)

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
ORIGINAL = SOURCE.read_bytes()
text = ORIGINAL.decode()
WORK.mkdir(parents=True, exist_ok=True)
ASSERTION_SITES = assertion_sites(TEST_SOURCE.read_text(), TEST_SOURCE.relative_to(ROOT).as_posix())
assert ASSERTION_SITES, 'current statement contract has no assertion macros'
for name, before, _ in cases:
    assert text.count(before) == 1, (name, 'actual anchor not unique')
site = min(ASSERTION_SITES)
location = ("\nfailures:\nthread 'fixture' panicked at %s:%d:%d:\n" % site).encode()
assert assertion_failure(location + b'custom macro message\n\nfailures:\nfixture\n')
assert not assertion_failure(b'test assertion_name ... ok\n\nfailures:\nthread panicked:\nassertion: expect-only\n\nfailures:\nassertion_name\n')
assert not assertion_failure(b'assertion: compiler output\n')
print('statement assertion classifier: current macro sites accepted; passing-name/labelled-expect/compiler noise refused')
if CLASSIFIER_ONLY:
    sys.exit(0)
environment = dict(os.environ)
try:
    for index, (name, before, after) in enumerate(cases, 1):
        SOURCE.write_text(text.replace(before, after))
        result = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test', 'formula_statement_contract'],
                                cwd=ROOT, env=environment, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('fault-%d.log' % index)).write_bytes(output)
        assert result.returncode == 101 and b'test result: FAILED.' in output, (name, output.decode())
        assert assertion_failure(output) and b'could not compile' not in output, (name, output.decode())
        print('statement fault %d %s:rc=101 actual compiled assertion red' % (index, name))
        SOURCE.write_bytes(ORIGINAL)
finally:
    SOURCE.write_bytes(ORIGINAL)
    restored = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test', 'formula_statement_contract'],
                              cwd=ROOT, env=environment, capture_output=True)
    (WORK / 'restored.log').write_bytes(restored.stdout + restored.stderr)
    assert restored.returncode == 0, ('restored statement artifact failed', restored.stderr.decode())
assert SOURCE.read_bytes() == ORIGINAL
print('statement faults:15 actual compiled assertion reds; source and current artifact restored')
