"""Fourteen production mutations must fail actual regression assertions, then restore exact source."""
from pathlib import Path
import subprocess
ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'crates/sc-measure/src/size_chart_collection.rs'
WORK = ROOT / 'target/size_chart_collection_mutations'
WORK.mkdir(exist_ok=True)
ORIGINAL = SOURCE.read_bytes()
TEXT = ORIGINAL.decode()
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
        assert result.returncode == 101 and b'test result: FAILED.' in output and b'assertion' in output and b'could not compile' not in output, (name, output.decode())
        print('mutation %d %s: rc=101, actual regression assertion failed' % (number, name))
        SOURCE.write_bytes(ORIGINAL)
finally:
    SOURCE.write_bytes(ORIGINAL)
assert SOURCE.read_bytes() == ORIGINAL
print('size chart collection mutations: 14 real reds; original source restored byte-identically')
