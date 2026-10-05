"""Exclusive product semantic metadata faults; compiled body reds and exact restoration required."""
from pathlib import Path
import os
import re
import runpy
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'crates/sc-core/src/recipe/semantic.rs'
WORK = ROOT / 'target/semantic_mutations'
TEST_SOURCE = ROOT / 'crates/sc-core/tests/formula_semantic_contract.rs'
BOOK_SOURCE = ROOT / 'docs/book/src/spec/formula-language.md'
SOURCES = (SOURCE, TEST_SOURCE, BOOK_SOURCE)
assert sys.argv[1:] in [[], ['--classifier-only']]
CLASSIFIER_ONLY = sys.argv[1:] == ['--classifier-only']
CASES = (
    ('kind population', '        Self::Point,', '        Self::Length,'),
    ('kind token', 'Self::Boolean => "boolean",', 'Self::Boolean => "count",'),
    ('geometry binding', 'Self::Point | Self::Edge => None,', 'Self::Point | Self::Edge => Some(FormulaBindingKind::Length),'),
    ('binding inverse', 'FormulaBindingKind::Ratio => Self::Ratio,', 'FormulaBindingKind::Ratio => Self::Count,'),
    ('origin token', 'Self::Profile => "profile",', 'Self::Profile => "parameter",'),
    ('size kind', 'Self::SizeIndex | Self::SizeCount => FormulaKind::Count,', 'Self::SizeIndex | Self::SizeCount => FormulaKind::Length,'),
    ('tolerance origin', 'Self::Tolerance(_) => FormulaOrigin::Tolerance,', 'Self::Tolerance(_) => FormulaOrigin::Profile,'),
    ('physical context', 'Self::Tolerance(FormulaToleranceName::Physical) => FormulaReservedContext::Profile,',
     'Self::Tolerance(FormulaToleranceName::Physical) => FormulaReservedContext::Always,'),
    ('size tolerance role', 'Self::SizeIndex | Self::SizeCount | Self::IsBaseSize => None,',
     'Self::SizeIndex | Self::SizeCount | Self::IsBaseSize => Some(FormulaToleranceName::Numerical),'),
    ('name lookup normalization', '.find(|name| name.token() == token)', '.find(|name| name.token() == token.trim())'),
)
OUTPUT_FILES = (() if CLASSIFIER_ONLY else
                tuple(WORK / ('fault-%d.log' % index) for index in range(1, len(CASES) + 1)) +
                (WORK / 'restored.log',))
MUTABLE_SOURCES = () if CLASSIFIER_ONLY else (SOURCE,)


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
    ROOT, directories=(WORK,), output_files=OUTPUT_FILES, sources=SOURCES, mutable_sources=MUTABLE_SOURCES)
WORK.mkdir(parents=True, exist_ok=True)
original = SOURCE.read_bytes()
ASSERTION_SITES = assertion_sites(TEST_SOURCE.read_text(), TEST_SOURCE.relative_to(ROOT).as_posix())
assert ASSERTION_SITES, 'current semantic test has no assertion macros'

for name, before, _ in CASES:
    assert original.decode().count(before) == 1, (name, 'actual fault anchor not unique')

site = min(ASSERTION_SITES)
location = ("\nfailures:\nthread 'fixture' panicked at %s:%d:%d:\n" % site).encode()
assert assertion_failure(location + b'custom macro message\n\nfailures:\nfixture\n')
assert not assertion_failure(b'test assertion_name ... ok\n\nfailures:\nthread panicked:\nassertion: expect-only\n\nfailures:\nassertion_name\n')
assert not assertion_failure(b'assertion: compiler output\n')
if CLASSIFIER_ONLY:
    print('semantic fault controls:10 unique actual anchors; current macro sites accepted; failed-name/labelled-expect/compiler noise refused')
    sys.exit(0)
environment = dict(os.environ)
try:
    for index, (name, before, after) in enumerate(CASES, 1):
        SOURCE.write_text(original.decode().replace(before, after))
        result = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test', 'formula_semantic_contract'],
                                cwd=ROOT, env=environment, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('fault-%d.log' % index)).write_bytes(output)
        assert result.returncode == 101 and b'test result: FAILED.' in output, (name, output.decode())
        assert assertion_failure(output) and b'could not compile' not in output, (name, output.decode())
        print('semantic fault %d %s:rc=101 actual compiled body assertion red' % (index, name))
        SOURCE.write_bytes(original)
finally:
    SOURCE.write_bytes(original)
    restored = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test', 'formula_semantic_contract'],
                              cwd=ROOT, env=environment, capture_output=True)
    (WORK / 'restored.log').write_bytes(restored.stdout + restored.stderr)
    assert restored.returncode == 0, ('restored semantic artifact failed', restored.stderr.decode())
assert SOURCE.read_bytes() == original
print('semantic faults:10 actual compiled assertion reds; source and actual compiled artifact restored')
