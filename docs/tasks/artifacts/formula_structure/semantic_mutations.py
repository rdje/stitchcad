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
runpy.run_path(str(ROOT / 'scripts/local_environment.py'))['enter_producer'](
    ROOT, directories=(WORK,), sources=(SOURCE,))
WORK.mkdir(parents=True, exist_ok=True)
original = SOURCE.read_bytes()
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
for name, before, _ in CASES:
    assert original.decode().count(before) == 1, (name, 'actual fault anchor not unique')

def assertion_failure(output):
    parts = output.split(b'\nfailures:\n', 2)
    return len(parts) == 3 and re.search(rb'(?m)^assertion(?:[ :`]|$)', parts[1]) is not None

assert assertion_failure(b'\nfailures:\nthread panicked:\nassertion `left == right` failed\n\nfailures:\nfixture\n')
assert not assertion_failure(b'test assertion_name ... ok\n\nfailures:\nthread panicked:\nexpect-only\n\nfailures:\nassertion_name\n')
assert not assertion_failure(b'assertion: compiler output\n')
assert sys.argv[1:] in [[], ['--classifier-only']]
if sys.argv[1:] == ['--classifier-only']:
    print('semantic fault controls:10 unique actual anchors; failed-name/expect-only/compiler noise refused')
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
