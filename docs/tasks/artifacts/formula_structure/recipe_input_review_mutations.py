"""Exclusive coupled production faults, targeted new assertions and exact restoration."""
from pathlib import Path
import os
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
STATEMENT = ROOT / 'crates/sc-core/src/recipe/statement.rs'
ORDERED = ROOT / 'crates/sc-core/src/recipe/ordered.rs'
CANONICAL = ROOT / 'crates/sc-core/src/recipe/canonical_recipe.rs'
NORMALIZED = ROOT / 'crates/sc-core/src/recipe/normalized_recipe.rs'
originals = {p: p.read_bytes() for p in [STATEMENT, ORDERED, CANONICAL, NORMALIZED]}
worked = 'actual_worked_population_retains_all_headers_global_locations_and_complete_identity'
refusal = 'later_input_refusals_abort_whole_identity_even_in_untaken_branches_and_call_arguments'
cases = [
    ('normalized header span', STATEMENT, 'name_span: self.name_span,', 'name_span: self.annotation_span,', worked),
    ('lost later inputs', ORDERED, 'self.statements.iter().enumerate()',
     'self.statements.iter().take(21).enumerate()', refusal),
    ('flattened input ordinal', ORDERED, 'statement_index: index + 1,', 'statement_index: 1,', refusal),
    ('whole identity order', CANONICAL, 'for statement in self.statements() {',
     'for statement in self.statements().iter().rev() {', worked),
    ('assertion identity right operand', CANONICAL, 'text.push_str(right.canonical_form().as_str());',
     'text.push_str(left.canonical_form().as_str());', worked),
    ('combined recipe bound', ORDERED, 'pub const MAX_STATEMENTS: usize = 4096;',
     'pub const MAX_STATEMENTS: usize = 4097;',
     'combined_maximum_and_each_first_excess_preserve_stage_and_later_context'),
    ('normalized statement privacy', NORMALIZED, '.field("span", &self.span)', '.field("name", &self.name)', worked),
]
for name, path, before, _, _ in cases:
    assert originals[path].decode().count(before) == 1, (name, 'actual anchor not unique')

def assertion_failure(output):
    parts = output.split(b'\nfailures:\n', 2)
    return len(parts) == 3 and re.search(rb'(?m)^assertion(?:[ :`]|$)', parts[1]) is not None

assert not assertion_failure(b'test assertion_ok ... ok\n\nfailures:\nthread panicked:\nexpect-only\n\nfailures:\nassertion_ok\n')
assert not assertion_failure(b'assertion: compiler output\n')
assert assertion_failure(b'\nfailures:\nthread panicked:\nassertion `left == right` failed\n\nfailures:\ncontract\n')
assert sys.argv[1:] in [[], ['--classifier-only']]
if sys.argv[1:] == ['--classifier-only']:
    print('coupled recipe input fault controls:7 unique actual anchors; compiler/expect/passing-name noise refused')
    sys.exit(0)

work = ROOT / 'target/recipe_input_review_mutations'
work.mkdir(parents=True, exist_ok=True)
env = dict(os.environ, CARGO_HOME=str(ROOT / 'target/cargo-home'), TMPDIR=str(ROOT / 'target/scratch'))
try:
    for index, (name, path, before, after, test) in enumerate(cases, 1):
        path.write_text(originals[path].decode().replace(before, after, 1))
        result = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test', 'formula_recipe_input_review', test],
                                cwd=ROOT, env=env, capture_output=True)
        output = result.stdout + result.stderr
        (work / ('fault-%d.log' % index)).write_bytes(output)
        assert result.returncode == 101 and b'test result: FAILED.' in output and assertion_failure(output), (name, output.decode())
        assert b'could not compile' not in output, (name, 'compiler refusal is not assertion proof')
        print('coupled recipe input fault %d %s:rc=101 actual compiled assertion red' % (index, name), flush=True)
        path.write_bytes(originals[path])
finally:
    for path, content in originals.items():
        path.write_bytes(content)
assert all(p.read_bytes() == content for p, content in originals.items())
print('coupled recipe input faults:7 actual compiled assertion reds; all four sources restored exactly')
