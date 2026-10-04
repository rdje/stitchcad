"""Exclusive native faults; current assertion sites and exact source/full target restoration."""
from pathlib import Path
import os
import re
import runpy
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'crates/sc-core/src/recipe/parser.rs'
CONTRACT = ROOT / 'crates/sc-core/tests/formula_expression_contract.rs'
SOURCES = (SOURCE, CONTRACT)
WORK = ROOT / 'target/formula_expression_mutations'
runpy.run_path(str(ROOT / 'scripts/local_environment.py'))['enter_producer'](
    ROOT, directories=(WORK,), sources=SOURCES)
original = SOURCE.read_bytes()
WORK.mkdir(parents=True, exist_ok=True)
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
for case in CASES:
    assert original.decode().count(case[1]) == 1, (case[0], 'actual fault anchor not unique')


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
    print('formula_expression_mutations classifier controls:11 unique actual anchors; current test assertion sites; expect/name/compiler noise refused')
    sys.exit(0)

environment = dict(os.environ)
try:
    for index, (name, before, after, test) in enumerate(CASES, 1):
        SOURCE.write_text(original.decode().replace(before, after, 1))
        result = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test', 'formula_expression_contract', test, '--', '--exact'],
                                cwd=ROOT, env=environment, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('mutation-%d.log' % index)).write_bytes(output)
        assert result.returncode == 101 and b'test result: FAILED.' in output, (name, output.decode())
        assert assertion_failure(output) and b'could not compile' not in output, (name, output.decode())
        print('formula_expression_mutations fault %d %s:rc=101 actual compiled body assertion red' % (index, name))
        SOURCE.write_bytes(original)
finally:
    SOURCE.write_bytes(original)
    restored = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test', 'formula_expression_contract'],
                              cwd=ROOT, env=environment, capture_output=True)
    (WORK / 'restored.log').write_bytes(restored.stdout + restored.stderr)
    assert restored.returncode == 0, ('restored formula_expression_mutations artifact failed', restored.stderr.decode())
assert SOURCE.read_bytes() == original
print('formula_expression_mutations:11 actual compiled assertion reds; exact source/full target restored')
