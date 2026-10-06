"""Exclusive coupled production faults, targeted new assertions and exact restoration."""
from pathlib import Path
import os
import re
import runpy
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
STATEMENT = ROOT / 'crates/sc-core/src/recipe/statement.rs'
ORDERED = ROOT / 'crates/sc-core/src/recipe/ordered.rs'
CANONICAL = ROOT / 'crates/sc-core/src/recipe/canonical_recipe.rs'
NORMALIZED = ROOT / 'crates/sc-core/src/recipe/normalized_recipe.rs'
MUTATED = (STATEMENT, ORDERED, CANONICAL, NORMALIZED)
CONTRACT = ROOT / 'crates/sc-core/tests/formula_recipe_input_review.rs'
WORKED = ROOT / 'docs/tasks/artifacts/formula_structure/canonical_worked_recipe_cases.tsv'
EXAMPLES = ROOT / 'docs/book/src/spec/formula-language/examples.md'
REFUSALS = ROOT / 'docs/tasks/artifacts/formula_structure/recipe_refusal_stage_cases.tsv'
SOURCES = (*MUTATED, CONTRACT, WORKED, EXAMPLES, REFUSALS)
WORK = ROOT / 'target/recipe_input_review_mutations'
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
assert sys.argv[1:] in [[], ['--classifier-only']]
CLASSIFIER_ONLY = sys.argv[1:] == ['--classifier-only']
OUTPUT_FILES = (() if CLASSIFIER_ONLY else
                tuple(WORK / ('fault-%d.log' % index) for index in range(1, len(cases) + 1)) +
                (WORK / 'restored.log',))
MUTABLE_SOURCES = () if CLASSIFIER_ONLY else MUTATED
runpy.run_path(str(ROOT / 'scripts/local_environment.py'))['enter_producer'](
    ROOT, directories=(WORK,), output_files=OUTPUT_FILES, sources=SOURCES, mutable_sources=MUTABLE_SOURCES)
originals = {path: path.read_bytes() for path in MUTATED}
WORK.mkdir(parents=True, exist_ok=True)
for name, path, before, _, _ in cases:
    assert originals[path].decode().count(before) == 1, (name, 'actual anchor not unique')

def assertion_sites(source, path):
    sites = set()
    for line_number, line in enumerate(source.splitlines(), 1):
        match = re.match(r'^(\s*)assert(?:_eq|_ne)?!\s*\(', line)
        if match is not None:
            sites.add((path, line_number, len(match.group(1).encode()) + 1))
    return sites


def assertion_failure(output):
    """A failed-body thread panic must name a current guarded assertion macro."""
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
if CLASSIFIER_ONLY:
    print('coupled recipe input fault controls:7 unique actual anchors; current test assertion sites; thread/expect/name/compiler noise refused')
    sys.exit(0)

environment = dict(os.environ)
try:
    for index, (name, path, before, after, test) in enumerate(cases, 1):
        path.write_text(originals[path].decode().replace(before, after, 1))
        result = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test', 'formula_recipe_input_review', test],
                                cwd=ROOT, env=environment, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('fault-%d.log' % index)).write_bytes(output)
        assert result.returncode == 101 and b'test result: FAILED.' in output and assertion_failure(output), (name, output.decode())
        assert b'could not compile' not in output, (name, 'compiler refusal is not assertion proof')
        print('coupled recipe input fault %d %s:rc=101 actual compiled assertion red' % (index, name), flush=True)
        path.write_bytes(originals[path])
finally:
    for path, content in originals.items():
        path.write_bytes(content)
    assert all(p.read_bytes() == content for p, content in originals.items()), 'recipe inputs not restored'
    restored = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test', 'formula_recipe_input_review'],
                              cwd=ROOT, env=environment, capture_output=True)
    (WORK / 'restored.log').write_bytes(restored.stdout + restored.stderr)
    assert restored.returncode == 0, ('restored coupled recipe-input artifact failed', restored.stderr.decode())
assert all(p.read_bytes() == content for p, content in originals.items())
print('coupled recipe input faults:7 actual compiled assertion reds; all four sources/current full target restored exactly')
