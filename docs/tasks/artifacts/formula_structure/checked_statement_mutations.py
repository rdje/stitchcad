"""Exclusive current-statement compiled body faults; real owners, phase and payloads stay coupled."""
from pathlib import Path
import os
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
STATEMENT = 'crates/sc-core/src/recipe/checked_statement.rs'
EXPRESSION = 'crates/sc-core/src/recipe/checked.rs'
SCOPE = 'crates/sc-core/src/recipe/namespace/ordered.rs'
CASES = [
    ('annotation guard omitted', STATEMENT, 'if expression.kind() != FormulaKind::from(*declared_kind)', 'if false', 1),
    ('actual RHS kind replaced', STATEMENT, 'expression_kind: expression.kind(),', 'expression_kind: FormulaKind::from(*declared_kind),', 1),
    ('actual annotation replaced', STATEMENT, 'declared_kind: *declared_kind,', 'declared_kind: FormulaBindingKind::Ratio,', 1),
    ('wanted kind replaced', STATEMENT, 'self.declared_kind.into()', 'self.expression_kind', 1),
    ('right operand replaced', STATEMENT, 'let right = check(right, FormulaStatementExpression::AssertionRight)?;', 'let right = check(left.expression(), FormulaStatementExpression::AssertionRight)?;', 1),
    ('child order reversed', STATEMENT,
     'let left = check(left, FormulaStatementExpression::AssertionLeft)?;\n            let right = check(right, FormulaStatementExpression::AssertionRight)?;',
     'let right = check(right, FormulaStatementExpression::AssertionRight)?;\n            let left = check(left, FormulaStatementExpression::AssertionLeft)?;', 1),
    ('left part replaced', STATEMENT, 'check(left, FormulaStatementExpression::AssertionLeft)?', 'check(left, FormulaStatementExpression::AssertionRight)?', 1),
    ('right part replaced', STATEMENT, 'check(right, FormulaStatementExpression::AssertionRight)?', 'check(right, FormulaStatementExpression::AssertionLeft)?', 1),
    ('comparison check omitted', STATEMENT, 'if !FormulaBinaryOperator::Equal', 'if false && !FormulaBinaryOperator::Equal', 1),
    ('immediate operands reversed', STATEMENT, '[left.root_operand(), right.root_operand()]', '[right.root_operand(), left.root_operand()]', 1),
    ('second comparison argument omitted', EXPRESSION, 'operands: Vec::from(operands),', 'operands: operands.into_iter().take(1).collect(),', 1),
    ('comparison operation mislabeled', EXPRESSION, 'operation: FormulaCheckedOperation::Binary(FormulaBinaryOperator::Equal),', 'operation: FormulaCheckedOperation::Binary(FormulaBinaryOperator::NotEqual),', 1),
    ('direct class role lost', EXPRESSION, 'return FormulaBuiltinOperand::Tolerance(name);', 'return FormulaBuiltinOperand::Value({ let _ = name; self.kind });', 1),
    ('computed class role forged', EXPRESSION,
     'if matches!(\n            self.expression.root().kind(),\n            super::FormulaNormalizedNodeKind::Name(_)\n        )', 'if true', 1),
    ('refusal ordinal fabricated', STATEMENT,
     'let refuse = |refusal| FormulaStatementCheckError {\n        statement,\n        statement_index,',
     'let refuse = |refusal| FormulaStatementCheckError {\n        statement,\n        statement_index: statement_index + 1,', 1),
    ('proof ordinal fabricated', STATEMENT,
     'Ok(FormulaCheckedStatement {\n        statement,\n        statement_index,',
     'Ok(FormulaCheckedStatement {\n        statement,\n        statement_index: statement_index + 1,', 1),
    ('binding span replaced', STATEMENT, 'FormulaStatementCheckRefusal::BindingDimension(_) => self.statement.annotation_span(),', 'FormulaStatementCheckRefusal::BindingDimension(_) => self.statement.name_span(),', 1),
    ('comparison span fabricated', STATEMENT, 'FormulaStatementCheckRefusal::AssertionDimension(_) => self.statement.span(),', 'FormulaStatementCheckRefusal::AssertionDimension(_) => self.statement.annotation_span(),', 1),
    ('dimension token replaced', STATEMENT, 'Self::BindingDimension(_) | Self::AssertionDimension(_) => "formula_dimension",', 'Self::BindingDimension(_) | Self::AssertionDimension(_) => "formula_domain",', 1),
    ('owner canonical identity fabricated', STATEMENT, 'self.statement.canonical_form()', 'super::FormulaStatement::parse("let fabricated:count=1").unwrap().normalize_literals().unwrap().canonical_form()', 2),
    ('proof customer disclosure', STATEMENT, 'f.debug_struct("FormulaCheckedStatement")', 'f.debug_struct("FormulaCheckedStatement").field("name", &self.statement.name())', 1),
    ('actual scope namespace ignored', SCOPE, '            self.namespace,\n        )', '            &FormulaNamespace::new([]).unwrap(),\n        )', 1),
]
PATHS = sorted({path for _, path, _, _, _ in CASES})
ORIGINAL = {path: (ROOT / path).read_bytes() for path in PATHS}
for name, path, before, _, count in CASES:
    assert ORIGINAL[path].decode().count(before) == count, (name, 'actual anchor count')


def assertion_failure(output):
    parts = output.split(b'\nfailures:\n', 2)
    return len(parts) == 3 and re.search(rb'(?m)^assertion(?:[ :`]|$)', parts[1]) is not None


assert assertion_failure(b'\nfailures:\nthread panicked:\nassertion `left == right` failed\n\nfailures:\nfixture\n')
assert not assertion_failure(b'\nfailures:\nthread panicked:\nexpect-only refusal\n\nfailures:\nassertion_fixture\n')
assert not assertion_failure(b'assertion: compiler output\n')
print('checked statement classifier: actual failed-body assertions required; compiler/expect/test-name noise refused', flush=True)
assert not sys.argv[1:] or sys.argv[1:] == ['--classifier-only']
if sys.argv[1:] == ['--classifier-only']:
    sys.exit(0)
WORK = ROOT / 'target/checked_statement_mutations'
WORK.mkdir(parents=True, exist_ok=True)
environment = dict(os.environ, CARGO_HOME=str(ROOT / 'target/cargo-home'),
                   CARGO_TARGET_DIR=str(ROOT / 'target'), TMPDIR=str(ROOT / 'target/scratch'))
try:
    for index, (name, path, before, after, count) in enumerate(CASES, 1):
        (ROOT / path).write_text(ORIGINAL[path].decode().replace(before, after, count))
        result = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test', 'formula_checked_statement_contract'],
                                cwd=ROOT, env=environment, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('fault-%d.log' % index)).write_bytes(output)
        assert result.returncode == 101 and b'test result: FAILED.' in output, (name, output.decode())
        assert assertion_failure(output) and b'could not compile' not in output, (name, output.decode())
        print('checked statement fault %d %s:rc=101 actual compiled body assertion red' % (index, name), flush=True)
        (ROOT / path).write_bytes(ORIGINAL[path])
        assert all((ROOT / p).read_bytes() == data for p, data in ORIGINAL.items())
finally:
    for path, data in ORIGINAL.items():
        (ROOT / path).write_bytes(data)
    assert all((ROOT / p).read_bytes() == data for p, data in ORIGINAL.items())
print('checked statement faults:%d actual compiled assertion reds; all three sources restored exactly' % len(CASES))
