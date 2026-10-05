"""D89: mutate actual public length operators; require compiled assertion reds and restore bytes."""
from pathlib import Path
import re
import runpy
import subprocess
ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'crates/sc-units/src/length.rs'
TEST_SOURCE = ROOT / 'crates/sc-units/tests/length_operator_contract.rs'
SOURCES = (SOURCE, TEST_SOURCE)
WORK = ROOT / 'target/length_operator_mutations'
CASES = [
    ('addition domain bypass', 'self.checked_add(rhs)', 'Ok(Self(self.0 + rhs.0))', 'signed_addition_crossings_refuse_instead_of_constructing_invalid_lengths'),
    ('subtraction domain bypass', 'self.checked_sub(rhs)', 'Ok(Self(self.0 - rhs.0))', 'signed_subtraction_crossings_refuse_instead_of_constructing_invalid_lengths'),
    ('addition operation', 'self.checked_add(rhs)', 'self.checked_sub(rhs)', 'signed_addition_crossings_refuse_instead_of_constructing_invalid_lengths'),
    ('subtraction operation', 'self.checked_sub(rhs)', 'self.checked_add(rhs)', 'signed_subtraction_crossings_refuse_instead_of_constructing_invalid_lengths'),
    ('addition saturation', 'self.checked_add(rhs)', 'Ok(Self((self.0 + rhs.0).clamp(-MAX_LENGTH_UM, MAX_LENGTH_UM)))', 'signed_addition_crossings_refuse_instead_of_constructing_invalid_lengths'),
    ('subtraction saturation', 'self.checked_sub(rhs)', 'Ok(Self((self.0 - rhs.0).clamp(-MAX_LENGTH_UM, MAX_LENGTH_UM)))', 'signed_subtraction_crossings_refuse_instead_of_constructing_invalid_lengths'),
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
assert ASSERTION_SITES, 'current length-operator test has no assertion macros'

try:
    for number, (name, before, after, test) in enumerate(CASES, 1):
        assert TEXT.count(before) == 1, (name, 'nonunique anchor')
        SOURCE.write_text(TEXT.replace(before, after, 1))
        result = subprocess.run(['cargo', 'test', '-p', 'sc-units', '--test', 'length_operator_contract',
                                 test, '--', '--exact'], cwd=ROOT, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('mutation-%d.log' % number)).write_bytes(output)
        assert result.returncode == 101 and b'test result: FAILED.' in output and assertion_failure(output) and b'could not compile' not in output, (name, output.decode())
        print('length operator mutation %d %s: rc=101, actual public-contract assertion failed' % (number, name))
        SOURCE.write_bytes(ORIGINAL)
finally:
    SOURCE.write_bytes(ORIGINAL)
    restored = subprocess.run(['cargo', 'test', '-p', 'sc-units', '--test', 'length_operator_contract'],
                              cwd=ROOT, capture_output=True)
    (WORK / 'restored.log').write_bytes(restored.stdout + restored.stderr)
    assert restored.returncode == 0, ('restored length artifact failed', restored.stderr.decode())
assert SOURCE.read_bytes() == ORIGINAL
print('Length operator mutations: six actual reds; source and actual compiled artifact restored')
