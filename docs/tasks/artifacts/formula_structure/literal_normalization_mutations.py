"""Exclusive actual formula_literal_contract faults; failed-body assertions and full restoration required."""
from pathlib import Path
import os
import re
import runpy
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'crates/sc-core/src/recipe/literal.rs'
WORK = ROOT / 'target/formula_literal_mutations'
runpy.run_path(str(ROOT / 'scripts/local_environment.py'))['enter_producer'](
    ROOT, directories=(WORK,), sources=(SOURCE,))
ORIGINAL = SOURCE.read_bytes()
WORK.mkdir(parents=True, exist_ok=True)
FIXTURES = 'independent_fraction_fixtures_cover_conversion_width_and_domain'
KINDS = 'kinds_quanta_and_angle_turns_are_preserved'
CASES = [
    ('inch conversion', '(FormulaLiteralKind::Length, 25400)', '(FormulaLiteralKind::Length, 25000)', FIXTURES),
    ('percent meaning', '(FormulaLiteralKind::Ratio, 10000)', '(FormulaLiteralKind::Ratio, 1000000)', KINDS),
    ('decimal kind', "None if number.contains('.') => (FormulaLiteralKind::Ratio, 1000000)", "None if number.contains('.') => (FormulaLiteralKind::Count, 1)", KINDS),
    ('raw angle direction modulo', '        magnitude,\n', '        magnitude: if kind == FormulaLiteralKind::Angle { magnitude % 360000000 } else { magnitude },\n', KINDS),
    ('literal narrowed', '        magnitude,\n', '        magnitude: u128::from(magnitude as u64),\n', KINDS),
    ('raw mantissa width', 'if significant > scale + 39 {', 'if significant > 39 {', FIXTURES),
    ('fraction zero cancellation', "fraction.trim_end_matches('0')", 'fraction', 'huge_spelling_refuses_or_reduces_without_new_language_caps'),
    ('mantissa reduction', 'divide_decimal(&mut decimal, prime);', 'divide_decimal(&mut decimal, 1);', FIXTURES),
    ('unit denominator cancellation', 'multiplier /= prime;', 'multiplier /= 1;', FIXTURES),
    ('denominator width hidden', '.checked_mul(prime)\n', '.checked_mul(prime).or(Some(1))\n', 'width_checks_precede_rounding_and_length_checks_follow_it'),
    ('length guard bypassed', 'magnitude > maximum {', 'false && magnitude > maximum {', 'width_checks_precede_rounding_and_length_checks_follow_it'),
    ('diagnostic family', '        "formula_domain"', '        "formula_parse"', FIXTURES),
    ('source identity', '        self.number\n', '        "0"\n', 'source_location_privacy_and_unary_identity_survive_conversion'),
]


def assertion_failure(output):
    # Only failed-test bodies supply evidence; passing names and summaries do not.
    parts = output.split(b'\nfailures:\n', 2)
    return len(parts) == 3 and re.search(rb'(?m)^assertion(?:[ :`]|$)', parts[1]) is not None


assert assertion_failure(b'\nfailures:\nthread panicked:\nassertion `left == right` failed\n\nfailures:\nfixture\n')
assert not assertion_failure(b'test assertion_name ... ok\n\nfailures:\nthread panicked:\nexpect-only\n\nfailures:\nassertion_name\n')
assert not assertion_failure(b'assertion: compiler output\n')
assert sys.argv[1:] in [[], ['--classifier-only']]
for name, before, _, _ in CASES:
    assert ORIGINAL.decode().count(before) == 1, (name, 'actual anchor not unique')
if sys.argv[1:] == ['--classifier-only']:
    print('normalization classifier: unique anchors; failed-body assertion accepted; passing-name/expect/compiler noise refused')
    sys.exit(0)
environment = dict(os.environ)
try:
    for index, (name, before, after, test) in enumerate(CASES, 1):
        SOURCE.write_text(ORIGINAL.decode().replace(before, after, 1))
        result = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test', 'formula_literal_contract', test, '--', '--exact'],
                                cwd=ROOT, env=environment, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('mutation-%d.log' % index)).write_bytes(output)
        assert result.returncode == 101 and b'test result: FAILED.' in output, (name, output.decode())
        assert assertion_failure(output) and b'could not compile' not in output, (name, output.decode())
        print('normalization fault %d %s: rc=101 actual compiled failed-body assertion red' % (index, name))
        SOURCE.write_bytes(ORIGINAL)
finally:
    SOURCE.write_bytes(ORIGINAL)
    restored = subprocess.run(['cargo', 'test', '-p', 'sc-core', '--test', 'formula_literal_contract'],
                              cwd=ROOT, env=environment, capture_output=True)
    (WORK / 'restored.log').write_bytes(restored.stdout + restored.stderr)
    assert restored.returncode == 0, ('restored normalization artifact failed', restored.stderr.decode())
assert SOURCE.read_bytes() == ORIGINAL
print('normalization faults:%d actual compiled assertion reds; exact source/current artifact restored' % len(CASES))
