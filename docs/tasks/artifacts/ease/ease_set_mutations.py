"""Require real regression assertion failures from ten production guard removals."""
from pathlib import Path
import subprocess
ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'crates/sc-measure/src/ease_set.rs'
WORK = ROOT / 'target/ease_set_mutations'
WORK.mkdir(exist_ok=True)
ORIGINAL = SOURCE.read_bytes()
TEXT = ORIGINAL.decode()
CASES = [
    ('table inventory identity', 'if measurements.contains(table.id()) || !seen.insert(table.id()) {', 'if false && (measurements.contains(table.id()) || !seen.insert(table.id())) {', 'contexts_refuse_duplicate_and_cross_kind_identities_before_lookup'),
    ('mapping inventory identity', 'if measurements.contains(ease.id()) || !seen.insert(ease.id()) {', 'if false && (measurements.contains(ease.id()) || !seen.insert(ease.id())) {', 'contexts_refuse_duplicate_and_cross_kind_identities_before_lookup'),
    ('entry identity', 'if !identities.insert(entry.ease) {', 'if false && !identities.insert(entry.ease) {', 'duplicate_mapping_tokens_and_poms_refuse_without_coalescing'),
    ('entry token', 'if !tokens.insert(&entry.token) {', 'if false && !tokens.insert(&entry.token) {', 'duplicate_mapping_tokens_and_poms_refuse_without_coalescing'),
    ('entry POM', 'if !poms.insert(entry.garment.measurement) {', 'if false && !poms.insert(entry.garment.measurement) {', 'duplicate_mapping_tokens_and_poms_refuse_without_coalescing'),
    ('set identity', 'if context.contains(self.id()) {', 'if false && context.contains(self.id()) {', 'set_identity_collisions_refuse_at_creation_and_in_current_context'),
    ('mapping reassignment', 'if &actual != entry {', 'if false && &actual != entry {', 'current_retargeted_mapping_needs_explicit_new_set_binding'),
    ('empty draft table reference', 'self.table(EaseSide::Body, context)?;', '', 'empty_drafts_require_existing_tables_and_queries_never_default'),
]
start = TEXT.index('        for (side, binding) in [')
end = TEXT.index('        ease.validate_current(context.measurements)', start)
CASES.append(('selected table membership', TEXT[start:end], '', 'selected_current_table_membership_is_required_even_when_metadata_exists'))
start = end
end = TEXT.index('        Ok(ease)', start)
CASES.append(('current mapping validation', TEXT[start:end], '', 'current_invalid_amount_and_table_targets_preserve_underlying_errors'))
try:
    for number, (name, before, after, test) in enumerate(CASES, 1):
        assert TEXT.count(before) == 1, (name, 'nonunique mutation anchor')
        SOURCE.write_text(TEXT.replace(before, after, 1))
        result = subprocess.run(['cargo','test','-p','sc-measure','--test','ease_set_contract',test,'--','--exact'],cwd=ROOT,capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('mutation-%d.log' % number)).write_bytes(output)
        assert result.returncode == 101 and b'test result: FAILED.' in output and b'assertion' in output and b'could not compile' not in output, (name, output.decode())
        print('mutation %d %s: rc=101, actual regression assertion failed' % (number, name))
        SOURCE.write_bytes(ORIGINAL)
finally:
    SOURCE.write_bytes(ORIGINAL)
assert SOURCE.read_bytes() == ORIGINAL
print('ease set mutations: 10 real reds; original source restored byte-identically')
