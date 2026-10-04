"""Seven guard/order mutations must fail real assertions, with exact source restoration."""
from pathlib import Path
import runpy
import subprocess
ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'crates/sc-measure/src/size_membership.rs'
WORK = ROOT / 'target/size_membership_mutations'
runpy.run_path(str(ROOT / 'scripts/local_environment.py'))['enter_producer'](
    ROOT, directories=(WORK,), sources=(SOURCE,))
WORK.mkdir(exist_ok=True)
ORIGINAL = SOURCE.read_bytes()
TEXT = ORIGINAL.decode()
CASES = [
    ('blank label', 'if label.trim().is_empty() {', 'if false && label.trim().is_empty() {', 'blank_labels_refuse_including_unicode_whitespace'),
    ('empty members', 'if definition.members.is_empty() {', 'if false && definition.members.is_empty() {', 'exactly_one_base_requires_nonempty_members_and_existing_identity'),
    ('member identity', 'if !identities.insert(member.id) {', 'if false && !identities.insert(member.id) {', 'member_ids_and_exact_labels_must_be_unique_and_distinct_from_set'),
    ('exact label uniqueness', 'if !labels.insert(member.label.as_str()) {', 'if false && !labels.insert(member.label.as_str()) {', 'member_ids_and_exact_labels_must_be_unique_and_distinct_from_set'),
    ('base membership', 'if !definition\n            .members\n            .iter()\n            .any(|member| member.id == definition.base)', 'if false && !definition\n            .members\n            .iter()\n            .any(|member| member.id == definition.base)', 'exactly_one_base_requires_nonempty_members_and_existing_identity'),
    ('revision overflow', '.checked_add(1)', '.checked_add(1).or(Some(0))', 'revision_successors_retain_identity_and_refuse_overflow_without_wrapping'),
    ('authored order', '        Ok(Self { definition })', '        let mut definition = definition;\n        definition.members.sort_by(|a, b| a.label.as_str().cmp(b.label.as_str()));\n        Ok(Self { definition })', 'authored_order_and_base_do_not_follow_lexical_or_numeric_sort'),
]
try:
    for number, (name, before, after, test) in enumerate(CASES, 1):
        assert TEXT.count(before) == 1, (name, 'nonunique mutation anchor')
        SOURCE.write_text(TEXT.replace(before, after, 1))
        result = subprocess.run(['cargo','test','-p','sc-measure','--test','size_membership_contract',test,'--','--exact'],cwd=ROOT,capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('mutation-%d.log' % number)).write_bytes(output)
        assert result.returncode == 101 and b'test result: FAILED.' in output and b'assertion' in output and b'could not compile' not in output, (name, output.decode())
        print('mutation %d %s: rc=101, actual regression assertion failed' % (number, name))
        SOURCE.write_bytes(ORIGINAL)
finally:
    SOURCE.write_bytes(ORIGINAL)
    restored = subprocess.run(['cargo', 'test', '-p', 'sc-measure', '--test', 'size_membership_contract'],
                              cwd=ROOT, capture_output=True)
    (WORK / 'restored.log').write_bytes(restored.stdout + restored.stderr)
    assert restored.returncode == 0, ('restored size-membership artifact failed', restored.stderr.decode())
assert SOURCE.read_bytes() == ORIGINAL
print('size membership mutations: 7 real reds; source and actual compiled artifact restored')
