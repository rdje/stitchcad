"""Observe assertion failures from real production guard removals, restoring exact source."""
from pathlib import Path
import runpy
import subprocess
ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'crates/sc-measure/src/ease.rs'
WORK = ROOT / 'target/ease_mutations'
runpy.run_path(str(ROOT / 'scripts/local_environment.py'))['enter_producer'](
    ROOT, directories=(WORK,), sources=(SOURCE,))
WORK.mkdir(exist_ok=True)
ORIGINAL = SOURCE.read_bytes()
TEXT = ORIGINAL.decode()
CASES = [
    ('authored domains', 'if binding.kind != side.kind() {', 'if false && binding.kind != side.kind() {', 'authored_sides_must_be_body_then_garment'),
    ('distinct scalar', 'if definition.declaration == definition.body.declaration', 'if false && definition.declaration == definition.body.declaration', 'amount_must_not_alias_either_measurement_scalar'),
    ('current identity', 'if context.contains(self.id()) {', 'if false && context.contains(self.id()) {', 'current_mapping_identity_collisions_refuse'),
    ('binding reassignment', 'if &actual != expected {', 'if false && &actual != expected {', 'same_id_binding_reassignments_require_explicit_replacement'),
    ('compression permission', 'if amount < Length::ZERO &&', 'if false && amount < Length::ZERO &&', 'all_authored_value_states_enforce_negative_permission_and_preserve_sign'),
    ('current amount permission', 'self.validate_amount(amount)?;', 'let _ = amount;', 'current_amount_edits_are_visible_and_missing_amount_refuses'),
]
# Both sides must undergo current metadata validation. Remove the complete check,
# not a different failing check; compiling mutants must fail a real assertion.
start = TEXT.index('        measurement\n            .validate_current(context.records())')
end = TEXT.index('        Ok(measurement)', start)
CASES.append(('current metadata', TEXT[start:end], '', 'missing_current_metadata_preserves_side_and_underlying_error'))
# The alias predicate spans two alternatives; disable the complete if condition.
before = 'if definition.declaration == definition.body.declaration\n            || definition.declaration == definition.garment.declaration'
CASES[1] = ('distinct scalar', before, 'if false && (definition.declaration == definition.body.declaration\n            || definition.declaration == definition.garment.declaration)', CASES[1][3])
try:
    for number, (name, before, after, test) in enumerate(CASES, 1):
        assert TEXT.count(before) == 1, (name, 'nonunique mutation anchor')
        SOURCE.write_text(TEXT.replace(before, after, 1))
        result = subprocess.run(['cargo', 'test', '-p', 'sc-measure', '--test', 'ease_contract', test, '--', '--exact'], cwd=ROOT, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('mutation-%d.log' % number)).write_bytes(output)
        assert result.returncode == 101 and b'test result: FAILED.' in output and b'assertion' in output and b'could not compile' not in output, (name, output.decode())
        print('mutation %d %s: rc=101, actual regression assertion failed' % (number, name))
        SOURCE.write_bytes(ORIGINAL)
finally:
    SOURCE.write_bytes(ORIGINAL)
    restored = subprocess.run(['cargo', 'test', '-p', 'sc-measure', '--test', 'ease_contract'],
                              cwd=ROOT, capture_output=True)
    (WORK / 'restored.log').write_bytes(restored.stdout + restored.stderr)
    assert restored.returncode == 0, ('restored Ease artifact failed', restored.stderr.decode())
assert SOURCE.read_bytes() == ORIGINAL
print('ease mutations: 7 real reds; source and actual compiled artifact restored')
