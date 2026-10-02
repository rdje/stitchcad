"""Actual iterative parser guards/order laws must discriminate; restore exact production bytes."""
from pathlib import Path
import subprocess
ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'crates/sc-core/src/recipe/parser.rs'
WORK = ROOT / 'target/formula_expression_mutations'
WORK.mkdir(exist_ok=True)
ORIGINAL = SOURCE.read_bytes()
TEXT = ORIGINAL.decode()
CASES = [
    ('semantic node bound', 'if self.seen_nodes > L::Nodes.bound() {', 'if false && self.seen_nodes > L::Nodes.bound() {', 'semantic_node_bounds_count_calls_and_square_payload_correctly'),
    ('conditional depth bound', 'if self.open_ifs > L::ConditionalDepth.bound() {', 'if false && self.open_ifs > L::ConditionalDepth.bound() {', 'conditional_depth_visits_calls_and_untaken_branches_but_not_siblings'),
    ('original unit gap', 'if self.source.get(gap.start()..gap.end()) != Some(" ") {', 'if false && self.source.get(gap.start()..gap.end()) != Some(" ") {', 'number_text_and_closed_units_remain_unconverted_and_borrowed'),
    ('comparison chaining', 'if self.frame()?.comparison_seen {', 'if false && self.frame()?.comparison_seen {', 'all_binary_comparisons_have_separate_syntax_roles'),
    ('single square suffix', 'if !can_square {', 'if false && !can_square {', 'square_payload_is_exact_and_other_exponents_are_unsupported'),
    ('square exponent payload', 'if !exponent\n', 'if false && !exponent\n', 'square_payload_is_exact_and_other_exponents_are_unsupported'),
    ('exact conditional arity', 'let [condition, then_branch, else_branch] =', 'let [condition, then_branch, else_branch, ..] =', 'ordinary_calls_and_conditionals_keep_every_ordered_child'),
    ('multiplication precedence', 'Self::Binary(B::Multiply | B::Divide) => 3,', 'Self::Binary(B::Multiply | B::Divide) => 1,', 'precedence_and_left_associativity_are_normative'),
    ('unary precedence', 'Self::Negate(_) => 4,', 'Self::Negate(_) => 1,', 'precedence_and_left_associativity_are_normative'),
    ('left associativity', 'previous.precedence() >= pending.precedence()', 'previous.precedence() > pending.precedence()', 'precedence_and_left_associativity_are_normative'),
    ('ordered call arguments', 'FrameKind::Call(name) => {\n                frame.arguments.push(last);', 'FrameKind::Call(name) => {\n                frame.arguments.push(last);\n                frame.arguments.reverse();', 'ordinary_calls_and_conditionals_keep_every_ordered_child'),
]
try:
    for number, (name, before, after, test) in enumerate(CASES, 1):
        assert TEXT.count(before) == 1, (name, 'nonunique mutation anchor')
        SOURCE.write_text(TEXT.replace(before, after, 1))
        result = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test',
                                 'formula_expression_contract', test, '--', '--exact'],
                                cwd=ROOT, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('mutation-%d.log' % number)).write_bytes(output)
        assert result.returncode == 101 and b'test result: FAILED.' in output and b'assertion' in output and b'could not compile' not in output, (name, output.decode())
        print('mutation %d %s: rc=101, actual contract assertion failed' % (number, name))
        SOURCE.write_bytes(ORIGINAL)
finally:
    SOURCE.write_bytes(ORIGINAL)
assert SOURCE.read_bytes() == ORIGINAL
print('Formula expression mutations: eleven actual reds; source restored byte-identically')
