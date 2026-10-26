"""Twelve real MTM guard/value/path mutations must fail assertions and restore exact production source."""
from pathlib import Path
import re
import runpy
import subprocess
ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'crates/sc-measure/src/mtm_chart.rs'
TEST_SOURCE = ROOT / 'crates/sc-measure/tests/mtm_chart_contract.rs'
TEST_FIXTURE = ROOT / 'crates/sc-measure/tests/support/mtm_fixture.rs'
SOURCES = (SOURCE, TEST_SOURCE, TEST_FIXTURE)
WORK = ROOT / 'target/mtm_chart_mutations'
CASES = [
    ('set/member context identity', 'if eases.contains(id) {', 'if false && eases.contains(id) {', 'chart_and_context_identities_cannot_alias_members_sets_or_canonical_records'),
    ('Ease-set context identity', 'if eases.contains(set.id()) || !identities.insert(set.id()) {', 'if false && (eases.contains(set.id()) || !identities.insert(set.id())) {', 'chart_and_context_identities_cannot_alias_members_sets_or_canonical_records'),
    ('chart identity', 'if context.contains(self.id()) {', 'if false && context.contains(self.id()) {', 'chart_and_context_identities_cannot_alias_members_sets_or_canonical_records'),
    ('pinned membership', 'if self.definition.membership != context.membership.reference() {', 'if false && self.definition.membership != context.membership.reference() {', 'exact_set_revision_and_member_identity_are_required_not_matching_labels'),
    ('custom system', 'if membership.system != SizeSystem::Custom {', 'if false && membership.system != SizeSystem::Custom {', 'mtm_requires_custom_system_and_exactly_one_member'),
    ('sole member', 'if membership.members.len() != 1 {', 'if false && membership.members.len() != 1 {', 'mtm_requires_custom_system_and_exactly_one_member'),
    ('canonical set targets', 'if set.definition() != &self.definition.ease_set {', 'if false && set.definition() != &self.definition.ease_set {', 'every_set_target_and_authored_order_requires_explicit_chart_replacement'),
    ('full Design coverage', 'if binding.kind == crate::MeasurementKind::Garment {', 'if false && binding.kind == crate::MeasurementKind::Garment {', 'new_current_design_pom_invalidates_prior_complete_coverage'),
    ('body numeric default', '.authored_value()\n            .map_err(|issue| MtmChartError::UnavailableBodyValue', '.authored_value().or(Ok(Length::ZERO))\n            .map_err(|issue| MtmChartError::UnavailableBodyValue', 'unknown_and_derived_body_inputs_remain_inspectable_and_refuse_numeric_defaults'),
    ('Ease numeric default', 'ease.authored_value(context.eases.measurements())', 'ease.authored_value(context.eases.measurements()).or(Ok(Length::ZERO))', 'unknown_and_derived_ease_amounts_remain_distinct_unresolved_inputs'),
    ('nonempty coverage', 'if set.definition().entries.is_empty() {', 'if false && set.definition().entries.is_empty() {', 'incomplete_and_empty_drafts_cannot_certify_design_pom_coverage'),
    ('grade-rule refusal', 'Err(MtmChartError::GradeRulesUnsupported {\n            chart: self.id(),\n            member: self.definition.member,\n        })', 'Ok(())', 'grade_rule_input_always_refuses_for_complete_and_empty_authored_mtm_charts'),
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
assert ASSERTION_SITES, 'current MTM test has no assertion macros'

try:
    for number, (name, before, after, test) in enumerate(CASES, 1):
        assert TEXT.count(before) == 1, (name, 'nonunique mutation anchor')
        SOURCE.write_text(TEXT.replace(before, after, 1))
        result = subprocess.run(['cargo', 'test', '-p', 'sc-measure', '--test', 'mtm_chart_contract', test, '--', '--exact'], cwd=ROOT, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('mutation-%d.log' % number)).write_bytes(output)
        assert result.returncode == 101 and b'test result: FAILED.' in output and assertion_failure(output) and b'could not compile' not in output, (name, output.decode())
        print('mutation %d %s: rc=101, actual regression assertion failed' % (number, name))
        SOURCE.write_bytes(ORIGINAL)
finally:
    SOURCE.write_bytes(ORIGINAL)
    restored = subprocess.run(['cargo', 'test', '-p', 'sc-measure', '--test', 'mtm_chart_contract'],
                              cwd=ROOT, capture_output=True)
    (WORK / 'restored.log').write_bytes(restored.stdout + restored.stderr)
    assert restored.returncode == 0, ('restored MTM chart artifact failed', restored.stderr.decode())
assert SOURCE.read_bytes() == ORIGINAL
print('MTM chart mutations: 12 real reds; source and actual compiled artifact restored')
