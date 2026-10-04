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
runpy.run_path(str(ROOT / 'scripts/local_environment.py'))['enter_producer'](
    ROOT, directories=(WORK,), sources=(SOURCE,))
ORIGINAL = SOURCE.read_bytes()
text = ORIGINAL.decode()
WORK.mkdir(parents=True, exist_ok=True)
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
for name, before, _ in cases:
    assert text.count(before) == 1, (name, 'actual anchor not unique')
def assertion_failure(output):
    # Passing test names, compiler messages and the failure-name summary cannot supply evidence.
    parts = output.split(b'\nfailures:\n', 2)
    return len(parts) == 3 and re.search(rb'(?m)^assertion(?:[ :`]|$)', parts[1]) is not None

expect_only = (b'test assertion_separator_ignores_all_grouped_and_call_comparisons ... ok\n'
               b'\nfailures:\n\nthread test panicked:\nvalid statement: UnbindableKind\n'
               b'\nfailures:\n    independent_authored_headers\n')
assert b'assertion' in expect_only and not assertion_failure(expect_only)
assert assertion_failure(b'\nfailures:\nthread test panicked:\nassertion `left == right` failed\n\nfailures:\n test\n')
assert not assertion_failure(b'assertion: compiler output\n')
print('statement assertion classifier: passing-name/expect-only and compiler noise refused; failed assertion accepted')
assert not sys.argv[1:] or sys.argv[1:] == ['--classifier-only']
if sys.argv[1:] == ['--classifier-only']:
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
