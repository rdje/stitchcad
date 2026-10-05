"""Eight real guard/substitution mutations must fail regression assertions and restore exact source."""
from pathlib import Path
import re
import runpy
import subprocess
ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'crates/sc-measure/src/size_chart.rs'
TEST_SOURCE = ROOT / 'crates/sc-measure/tests/size_chart_contract.rs'
SOURCES = (SOURCE, TEST_SOURCE)
WORK = ROOT / 'target/size_chart_mutations'
CASES = [
    ('set/member context identity', 'if measurements.contains(id) {', 'if false && measurements.contains(id) {', 'context_refuses_duplicate_tables_and_cross_kind_set_member_identities'),
    ('table context identity', 'if measurements.contains(id) || !identities.insert(id) {', 'if false && (measurements.contains(id) || !identities.insert(id)) {', 'context_refuses_duplicate_tables_and_cross_kind_set_member_identities'),
    ('garment domains', 'if binding.kind != MeasurementKind::Garment {', 'if false && binding.kind != MeasurementKind::Garment {', 'both_authored_roles_require_garment_domain'),
    ('observation identity', 'if context.contains(self.id()) {', 'if false && context.contains(self.id()) {', 'observation_identity_cannot_alias_any_supplied_semantic_record'),
    ('pinned set revision', 'if self.definition.membership != context.membership.reference() {', 'if false && self.definition.membership != context.membership.reference() {', 'exact_set_identity_and_revision_are_required'),
    ('stable member identity', '.member(self.definition.member)', '.base()', 'values_members_and_both_roles_are_borrowed_without_recomputing'),
    ('named table identity', '.get(&table_id)', '.get(&table_id).or_else(|| context.tables.values().find(|table| table.definition().entries.iter().any(|entry| entry.measurement == expected.measurement)))', 'same_content_table_peer_does_not_replace_named_table'),
    ('saved observation binding', 'if &actual != expected {', 'if false && &actual != expected {', 'explicit_table_rebinding_does_not_silently_rebind_an_observation'),
]
OUTPUT_FILES = tuple(WORK / ('mutation-%d.log' % index) for index in range(1, len(CASES) + 1)) + \
    (WORK / 'restored.log',)


def assertion_sites(source, path):
    """Current curated test macros, with Rust panic byte columns."""
    sites = set()
    for number, line in enumerate(source.splitlines(), 1):
        match = re.match(r'^(\s*)assert(?:_eq|_ne)?!\s*\(', line)
        if match is not None:
            sites.add((path, number, len(match.group(1).encode()) + 1))
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
assert ASSERTION_SITES, 'current size chart test has no assertion macros'

try:
    for number, (name, before, after, test) in enumerate(CASES, 1):
        assert TEXT.count(before) == 1, (name, 'nonunique mutation anchor')
        SOURCE.write_text(TEXT.replace(before, after, 1))
        result = subprocess.run(['cargo', 'test', '-p', 'sc-measure', '--test', 'size_chart_contract', test, '--', '--exact'], cwd=ROOT, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('mutation-%d.log' % number)).write_bytes(output)
        assert result.returncode == 101 and b'test result: FAILED.' in output and assertion_failure(output) and b'could not compile' not in output, (name, output.decode())
        print('mutation %d %s: rc=101, actual regression assertion failed' % (number, name))
        SOURCE.write_bytes(ORIGINAL)
finally:
    SOURCE.write_bytes(ORIGINAL)
    restored = subprocess.run(['cargo', 'test', '-p', 'sc-measure', '--test', 'size_chart_contract'],
                              cwd=ROOT, capture_output=True)
    (WORK / 'restored.log').write_bytes(restored.stdout + restored.stderr)
    assert restored.returncode == 0, ('restored size chart artifact failed', restored.stderr.decode())
assert SOURCE.read_bytes() == ORIGINAL
print('size chart mutations: 8 real reds; source and actual compiled artifact restored')
