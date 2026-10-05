"""Fourteen production mutations must fail actual regression assertions, then restore exact source."""
from pathlib import Path
import re
import runpy
import subprocess
ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'crates/sc-measure/src/size_chart_collection.rs'
TEST_SOURCE = ROOT / 'crates/sc-measure/tests/size_chart_collection_contract.rs'
SOURCES = (SOURCE, TEST_SOURCE)
WORK = ROOT / 'target/size_chart_collection_mutations'
CASES = [
    ('context identity', [('if records.contains(observation.id())\n                || inventory.insert(observation.id(), observation).is_some()', 'if (records.contains(observation.id())\n                || inventory.insert(observation.id(), observation).is_some()) && false')], 'collection_and_context_identities_cannot_alias_supplied_records'),
    ('chart identity', [('if context.contains(self.id()) {', 'if false && context.contains(self.id()) {')], 'collection_and_context_identities_cannot_alias_supplied_records'),
    ('pinned membership', [('if actual != self.definition.membership {', 'if false && actual != self.definition.membership {')], 'exact_membership_reference_and_stable_member_are_required'),
    ('garment domain', [('if pom.kind != MeasurementKind::Garment {', 'if false && pom.kind != MeasurementKind::Garment {')], 'authored_target_body_domain_is_refused_even_when_current_table_is_valid'),
    ('POM/token uniqueness', [('if !poms.insert(pom.measurement) {', 'if false && !poms.insert(pom.measurement) {'), ('if !tokens.insert(&pom.token) {', 'if false && !tokens.insert(&pom.token) {')], 'duplicate_pom_token_observation_and_valid_same_cell_are_refused'),
    ('observation uniqueness', [('if !identities.insert(entry.observation) {', 'if false && !identities.insert(entry.observation) {')], 'duplicate_pom_token_observation_and_valid_same_cell_are_refused'),
    ('valid duplicate cell', [('if !cells.insert((entry.member, entry.pom.measurement)) {', 'if false && !cells.insert((entry.member, entry.pom.measurement)) {')], 'duplicate_pom_token_observation_and_valid_same_cell_are_refused'),
    ('current POM binding', [('if &actual != binding {', 'if false && &actual != binding {')], 'changed_pom_binding_needs_explicit_chart_and_observation_replacement'),
    ('complete Design target inventory', [('if binding.kind == MeasurementKind::Garment {', 'if false && binding.kind == MeasurementKind::Garment {')], 'reduced_target_list_cannot_certify_omitted_design_pom'),
    ('member/POM cell coverage', [('if !cells.contains(&(member.id, pom.measurement)) {', 'if false && !cells.contains(&(member.id, pom.measurement)) {')], 'incomplete_draft_refuses_exact_missing_cell_without_interpolation'),
    ('saved observation targets', [('if &actual != entry {', 'if false && &actual != entry {')], 'all_saved_observation_targets_require_explicit_collection_replacement'),
    ('named Design table', [('if actual.design_table != self.definition.design_table {', 'if false && actual.design_table != self.definition.design_table {')], 'undeclared_pom_and_observation_from_another_design_table_are_refused'),
    ('unresolved numeric fallback', [('.authored_value(context.records)', '.authored_value(context.records).or(Ok(Length::ZERO))')], 'unknown_and_derived_cells_can_be_structurally_complete_but_never_numeric_defaults'),
    ('nonempty completeness', [('if self.definition.poms.is_empty() {', 'if false && self.definition.poms.is_empty() {')], 'empty_inventory_is_inspectable_but_not_complete_and_keeps_table_revision_checks'),
]
OUTPUT_FILES = tuple(WORK / ('mutation-%d.log' % index) for index in range(1, len(CASES) + 1)) + \
    (WORK / 'restored.log',)


def assertion_sites(source, path):
    """Current curated test macros, with Rust panic byte columns."""
    sites = set()
    for number, line in enumerate(source.splitlines(), 1):
        for match in re.finditer(r'\bassert(?:_eq|_ne)?!\s*\(', line):
            sites.add((path, number, len(line[:match.start()].encode('utf-8')) + 1))
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
    ROOT, directories=(WORK,), output_files=OUTPUT_FILES, sources=SOURCES, mutable_sources=(SOURCE,))
WORK.mkdir(exist_ok=True)
ORIGINAL = SOURCE.read_bytes()
TEXT = ORIGINAL.decode()
ASSERTION_SITES = assertion_sites(TEST_SOURCE.read_text(), TEST_SOURCE.relative_to(ROOT).as_posix())
assert ASSERTION_SITES, 'current collection test has no assertion macros'

try:
    for number, (name, mutations, test) in enumerate(CASES, 1):
        mutated = TEXT
        for before, after in mutations:
            assert mutated.count(before) == 1, (name, 'nonunique mutation anchor')
            mutated = mutated.replace(before, after, 1)
        SOURCE.write_text(mutated)
        result = subprocess.run(['cargo', 'test', '-p', 'sc-measure', '--test', 'size_chart_collection_contract', test, '--', '--exact'], cwd=ROOT, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('mutation-%d.log' % number)).write_bytes(output)
        assert result.returncode == 101 and b'test result: FAILED.' in output and assertion_failure(output) and b'could not compile' not in output, (name, output.decode())
        print('mutation %d %s: rc=101, actual regression assertion failed' % (number, name))
        SOURCE.write_bytes(ORIGINAL)
finally:
    SOURCE.write_bytes(ORIGINAL)
    restored = subprocess.run(['cargo', 'test', '-p', 'sc-measure', '--test', 'size_chart_collection_contract'],
                              cwd=ROOT, capture_output=True)
    (WORK / 'restored.log').write_bytes(restored.stdout + restored.stderr)
    assert restored.returncode == 0, ('restored size chart collection artifact failed', restored.stderr.decode())
assert SOURCE.read_bytes() == ORIGINAL
print('size chart collection mutations: 14 real reds; source and actual compiled artifact restored')
