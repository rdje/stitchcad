"""Real guard mutations must produce test assertions, not compilation failures."""
from pathlib import Path
import runpy
import subprocess

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'crates/sc-measure/src/table.rs'
WORK = ROOT / 'target/measurement_table_mutations'
runpy.run_path(str(ROOT / 'scripts/local_environment.py'))['enter_producer'](
    ROOT, directories=(WORK,), sources=(SOURCE,))
WORK.mkdir(exist_ok=True)
ORIGINAL = SOURCE.read_bytes()
TEXT = ORIGINAL.decode()
CASES = [
    ('entry identity', 'if !identities.insert(entry.measurement) {', 'if false && !identities.insert(entry.measurement) {', 'duplicate_entry_id_and_token_refuse_without_coalescing_or_first_peer_selection'),
    ('entry token', 'if !tokens.insert(&entry.token) {', 'if false && !tokens.insert(&entry.token) {', 'duplicate_entry_id_and_token_refuse_without_coalescing_or_first_peer_selection'),
    ('context identities', 'if records.contains(id) || inventory.insert(id, measurement).is_some() {', 'if { let _ = inventory.insert(id, measurement); false } {', 'current_inventory_id_collisions_refuse_before_any_table_lookup'),
    ('token reassignment', 'if current.token != binding.token {', 'if false && current.token != binding.token {', 'current_token_reassignment_is_not_an_implicit_table_rename'),
    ('kind reassignment', 'if current.kind != binding.kind {', 'if false && current.kind != binding.kind {', 'current_domain_reassignment_is_refused_even_when_new_metadata_is_valid_in_its_new_domain'),
    ('scalar reassignment', 'if current.declaration != binding.declaration {', 'if false && current.declaration != binding.declaration {', 'current_scalar_reassignment_requires_explicit_binding_replacement'),
    ('current metadata', '.validate_current(context.records)', '.validate_current(&MeasurementContext::new(&[], &[], &[]).unwrap())', 'required_current_target_failures_keep_table_measurement_and_underlying_identity_evidence'),
    ('table identity', 'if context.contains(self.id()) {', 'if false && context.contains(self.id()) {', 'current_context_collision_of_table_identity_is_not_cached_birth_approval'),
]
# Remove the entire current-check expression for this arm; checking a different
# invalid context would be a failing implementation, not a missing-guard mutation.
start = TEXT.index('        measurement\n            .validate_current(context.records)')
end = TEXT.index('        Ok(measurement)', start)
current_check = TEXT[start:end]
CASES[6] = ('current metadata', current_check, '', CASES[6][3])
try:
    for number, (name, before, after, test) in enumerate(CASES, 1):
        assert TEXT.count(before) == 1, (name, 'mutation anchor is not unique')
        SOURCE.write_text(TEXT.replace(before, after, 1))
        result = subprocess.run(['cargo', 'test', '-p', 'sc-measure', '--test', 'table_contract', test, '--', '--exact'], cwd=ROOT, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('mutation-%d.log' % number)).write_bytes(output)
        assert result.returncode == 101 and b'test result: FAILED.' in output and b'panicked at' in output and b'assertion' in output and b'could not compile' not in output, (name, output.decode())
        print('mutation %d %s: rc=101, actual regression assertion failed' % (number, name))
        SOURCE.write_bytes(ORIGINAL)
finally:
    SOURCE.write_bytes(ORIGINAL)
    restored = subprocess.run(['cargo', 'test', '-p', 'sc-measure', '--test', 'table_contract'],
                              cwd=ROOT, capture_output=True)
    (WORK / 'restored.log').write_bytes(restored.stdout + restored.stderr)
    assert restored.returncode == 0, ('restored table artifact failed', restored.stderr.decode())
assert SOURCE.read_bytes() == ORIGINAL
print('table mutations: 8 real reds; source and actual compiled artifact restored')
