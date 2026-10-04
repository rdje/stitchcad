"""Exclusive actual formula_normalized_contract faults; failed-body assertions and full restoration required."""
from pathlib import Path
import os
import re
import runpy
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'crates/sc-core/src/recipe/normalized.rs'
WORK = ROOT / 'target/formula_normalized_mutations'
runpy.run_path(str(ROOT / 'scripts/local_environment.py'))['enter_producer'](
    ROOT, directories=(WORK,), sources=(SOURCE,))
ORIGINAL = SOURCE.read_bytes()
WORK.mkdir(parents=True, exist_ok=True)
SHAPE = 'authored_shapes_match_reference_kind_order_nodes_and_depth'
SPAN = 'normalized_arena_retains_source_spans_and_borrows_after_syntax_drop'
CALL = 'argument_views_are_ordered_exact_size_fused_and_arena_bound'
CASES = [
    ('root identity', 'root: self.root,', 'root: 0,', SHAPE),
    ('name identity', 'NodeData::Name(name) => NormalizedData::Name(name),', 'NodeData::Name(_) => NormalizedData::Name("changed"),', SHAPE),
    ('unary identity', 'NodeData::Negate(child) => NormalizedData::Negate(*child),', 'NodeData::Negate(child) => NormalizedData::Square(*child),', SHAPE),
    ('square identity', 'NodeData::Square(child) => NormalizedData::Square(*child),', 'NodeData::Square(child) => NormalizedData::Negate(*child),', SHAPE),
    ('binary operator', 'operator: *operator,\n                    left: *left,', 'operator: FormulaBinaryOperator::Add,\n                    left: *left,', SHAPE),
    ('binary child order', 'left: *left,\n                    right: *right,', 'left: *right,\n                    right: *left,', SHAPE),
    ('call child order', 'arguments: arguments.clone(),', 'arguments: arguments.iter().rev().copied().collect(),', SHAPE),
    ('call child loss', 'arguments: arguments.clone(),', 'arguments: arguments.iter().take(1).copied().collect(),', SHAPE),
    ('conditional branch order', 'then_branch: *then_branch,\n                    else_branch: *else_branch,', 'then_branch: *else_branch,\n                    else_branch: *then_branch,', SHAPE),
    ('conditional condition', 'condition: *condition,', 'condition: *then_branch,', SHAPE),
    ('depth metadata', 'if_depth: self.if_depth,', 'if_depth: 0,', SHAPE),
    ('source span', 'span: node.span,', 'span: FormulaSourceSpan::new(0, 0),', SPAN),
    ('literal unit', 'super::literal::normalize(number, *unit, node.span)?', 'super::literal::normalize(number, None, node.span)?', SHAPE),
    ('literal refusal bypass', 'super::literal::normalize(number, *unit, node.span)?', 'super::literal::normalize(number, *unit, node.span).or_else(|_| super::literal::normalize("0", None, node.span))?', 'every_literal_position_refuses_atomically_with_original_span'),
    ('argument iteration', 'self.indices.next().map(|index|', 'self.indices.nth(1).map(|index|', CALL),
    ('argument size hint', 'self.indices.size_hint()', '(0, Some(0))', CALL),
    ('debug privacy', '.field("node_count", &self.nodes.len())', '.field("node_count", &self.nodes.len()).field("nodes", &self.nodes)', 'every_worked_expression_normalizes_and_debug_omits_customer_content'),
]


def assertion_failure(output):
    # Only failed-test bodies supply evidence; passing names and summaries do not.
    parts = output.split(b'\nfailures:\n', 2)
    return len(parts) == 3 and re.search(rb'(?m)^assertion(?:[ :`]|$)', parts[1]) is not None


assert assertion_failure(b'\nfailures:\nthread panicked:\nassertion `left == right` failed\n\nfailures:\nfixture\n')
assert not assertion_failure(b'test assertion_name ... ok\n\nfailures:\nthread panicked:\nexpect-only\n\nfailures:\nassertion_name\n')
assert not assertion_failure(b'assertion: compiler output\n')
assert sys.argv[1:] in [[], ['--classifier-only']]
for name, before, _, _ in CASES:
    assert ORIGINAL.decode().count(before) == 1, (name, 'actual anchor not unique')
if sys.argv[1:] == ['--classifier-only']:
    print('normalization classifier: unique anchors; failed-body assertion accepted; passing-name/expect/compiler noise refused')
    sys.exit(0)
environment = dict(os.environ)
try:
    for index, (name, before, after, test) in enumerate(CASES, 1):
        SOURCE.write_text(ORIGINAL.decode().replace(before, after, 1))
        result = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test', 'formula_normalized_contract', test, '--', '--exact'],
                                cwd=ROOT, env=environment, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('mutation-%d.log' % index)).write_bytes(output)
        assert result.returncode == 101 and b'test result: FAILED.' in output, (name, output.decode())
        assert assertion_failure(output) and b'could not compile' not in output, (name, output.decode())
        print('normalization fault %d %s: rc=101 actual compiled failed-body assertion red' % (index, name))
        SOURCE.write_bytes(ORIGINAL)
finally:
    SOURCE.write_bytes(ORIGINAL)
    restored = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test', 'formula_normalized_contract'],
                              cwd=ROOT, env=environment, capture_output=True)
    (WORK / 'restored.log').write_bytes(restored.stdout + restored.stderr)
    assert restored.returncode == 0, ('restored normalization artifact failed', restored.stderr.decode())
assert SOURCE.read_bytes() == ORIGINAL
print('normalization faults:%d actual compiled assertion reds; exact source/current artifact restored' % len(CASES))
