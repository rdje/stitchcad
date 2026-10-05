"""Exclusive actual formula_normalized_contract faults; failed-body assertions and full restoration required."""
from pathlib import Path
import os
import re
import runpy
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'crates/sc-core/src/recipe/normalized.rs'
CONTRACT = ROOT / 'crates/sc-core/tests/formula_normalized_contract.rs'
FIXTURE_SOURCES = tuple(ROOT / ('docs/tasks/artifacts/formula_structure/' + name) for name in (
    'normalized_expression_shapes.tsv', 'literal_normalization_cases.tsv')) + \
    (ROOT / 'docs/book/src/spec/formula-language/examples.md',)
SOURCES = (SOURCE, CONTRACT, *FIXTURE_SOURCES)
WORK = ROOT / 'target/formula_normalized_mutations'
assert sys.argv[1:] in [[], ['--classifier-only']]
CLASSIFIER_ONLY = sys.argv[1:] == ['--classifier-only']
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
OUTPUT_FILES = (() if CLASSIFIER_ONLY else
                tuple(WORK / ('mutation-%d.log' % index) for index in range(1, len(CASES) + 1)) +
                (WORK / 'restored.log',))
MUTABLE_SOURCES = () if CLASSIFIER_ONLY else (SOURCE,)
runpy.run_path(str(ROOT / 'scripts/local_environment.py'))['enter_producer'](
    ROOT, directories=(WORK,), output_files=OUTPUT_FILES, sources=SOURCES, mutable_sources=MUTABLE_SOURCES)
ORIGINAL = SOURCE.read_bytes()
WORK.mkdir(parents=True, exist_ok=True)



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


ASSERTION_SITES = assertion_sites(CONTRACT.read_text(), CONTRACT.relative_to(ROOT).as_posix())
assert ASSERTION_SITES, 'focused contract has no assertion sites'
site = min(ASSERTION_SITES)
location = ("\nfailures:\nthread 'fixture' panicked at %s:%d:%d:\n" % site).encode()
assert assertion_failure(location + b'custom macro message\n\nfailures:\nfixture\n')
assert not assertion_failure(location.replace(b"thread 'fixture'", b'compiler note:') +
                             b'nonthread location mention\n\nfailures:\nfixture\n')
assert not assertion_failure(b'\nfailures:\nthread panicked:\nexpect-only\n\nfailures:\nassertion_name\n')
assert not assertion_failure(b'assertion: compiler output\n')
for name, before, _, _ in CASES:
    assert ORIGINAL.decode().count(before) == 1, (name, 'actual anchor not unique')
if CLASSIFIER_ONLY:
    print('normalization classifier: unique anchors/current test assertion sites; passing-name/expect/compiler noise refused')
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
