"""Wide public rounding: compiled production faults must fail independent assertions."""
from pathlib import Path
import re
import runpy
import subprocess

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'crates/sc-units/src/round.rs'
TEST_SOURCE = ROOT / 'crates/sc-units/tests/unsigned_round_contract.rs'
FIXTURE_SOURCE = ROOT / 'docs/tasks/artifacts/formula_structure/unsigned_round_cases.tsv'
SOURCES = (SOURCE, TEST_SOURCE, FIXTURE_SOURCE)
WORK = ROOT / 'target/formula_unsigned_round_mutations'
CALL = '''    round_magnitude(
        numerator,
        denominator,
        "div_round_half_away_from_zero_unsigned",
    )'''
CASES = [
    ('output narrowed', CALL, CALL + '.map(|value| u128::from(value as u64))',
     'full_width_inputs_and_high_remainders_round_without_narrowing'),
    ('numerator narrowed', CALL, CALL.replace('        numerator,', '        u128::from(numerator as u64),'),
     'full_width_inputs_and_high_remainders_round_without_narrowing'),
    ('denominator narrowed', CALL, CALL.replace('        denominator,', '        u128::from(denominator as u64),'),
     'full_width_inputs_and_high_remainders_round_without_narrowing'),
    ('doubled remainder overflow', 'r.cmp(&(denominator - r))', '(r * 2).cmp(&denominator)',
     'full_width_inputs_and_high_remainders_round_without_narrowing'),
    ('half tie dropped', 'if half != Ordering::Less {', 'if half == Ordering::Greater {',
     'even_and_odd_denominator_half_neighborhoods_are_distinct'),
    ('quantum increment dropped', 'q.checked_add(1)', 'q.checked_add(0)',
     'even_and_odd_denominator_half_neighborhoods_are_distinct'),
    ('zero guard bypassed', 'if denominator == 0 {', 'if false && denominator == 0 {',
     'zero_denominator_retains_public_operation_without_panicking'),
    ('public operation lost', '"div_round_half_away_from_zero_unsigned",', '"wide_round",',
     'zero_denominator_retains_public_operation_without_panicking'),
    ('signed forwarding context changed', 'round_magnitude(n, d, "div_round_half_away_from_zero")?',
     'div_round_half_away_from_zero_unsigned(n, d)?',
     'zero_denominator_retains_public_operation_without_panicking'),
]
OUTPUT_FILES = tuple(WORK / ('mutation-%d.log' % index) for index in range(1, len(CASES) + 1)) + \
    (WORK / 'release-remainder.log', WORK / 'restored-debug.log', WORK / 'restored-release.log')


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
ASSERTION_SITES = assertion_sites(TEST_SOURCE.read_text(), TEST_SOURCE.relative_to(ROOT).as_posix())
assert ASSERTION_SITES, 'current unsigned-round test has no assertion macros'

try:
    for number, (name, before, after, test) in enumerate(CASES, 1):
        text = ORIGINAL.decode()
        assert text.count(before) == 1, (name, 'nonunique actual source anchor')
        SOURCE.write_text(text.replace(before, after, 1))
        result = subprocess.run(
            ['cargo', 'test', '-p', 'sc-units', '--test', 'unsigned_round_contract', test, '--', '--exact'],
            cwd=ROOT, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('mutation-%d.log' % number)).write_bytes(output)
        assert result.returncode == 101 and b'test result: FAILED.' in output and assertion_failure(output) and b'could not compile' not in output, (name, output.decode())
        print('unsigned round mutation %d %s: rc=101, compiled actual public assertion red' % (number, name))
        if name == 'doubled remainder overflow':
            result = subprocess.run(
                ['cargo', 'test', '-p', 'sc-units', '--release', '--test', 'unsigned_round_contract', test, '--', '--exact'],
                cwd=ROOT, capture_output=True)
            output = result.stdout + result.stderr
            (WORK / 'release-remainder.log').write_bytes(output)
            assert result.returncode == 101 and b'test result: FAILED.' in output and assertion_failure(output) and b'could not compile' not in output, output.decode()
            print('unsigned round release remainder fault: rc=101, compiled actual wrap assertion red')
        SOURCE.write_bytes(ORIGINAL)
finally:
    SOURCE.write_bytes(ORIGINAL)
    restored_results = []
    for profile, extra in (('debug', []), ('release', ['--release'])):
        restored = subprocess.run(
            ['cargo', 'test', '-p', 'sc-units', *extra, '--test', 'unsigned_round_contract'],
            cwd=ROOT, capture_output=True)
        (WORK / ('restored-' + profile + '.log')).write_bytes(restored.stdout + restored.stderr)
        restored_results.append((profile, restored.returncode, restored.stderr.decode()))
    assert all(status == 0 for _, status, _ in restored_results), ('restored round artifacts failed', restored_results)
assert SOURCE.read_bytes() == ORIGINAL
print('unsigned round mutations: nine debug reds/one release red; source and both compiled profiles restored')
