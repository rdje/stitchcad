"""Actual public-rounding boundary/sign/tie/zero guards must discriminate; exact source restoration."""
from pathlib import Path
import runpy
import subprocess
ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'crates/sc-units/src/round.rs'
WORK = ROOT / 'target/formula_round_mutations'
runpy.run_path(str(ROOT / 'scripts/local_environment.py'))['enter_producer'](
    ROOT, directories=(WORK,), sources=(SOURCE,))
WORK.mkdir(exist_ok=True)
ORIGINAL = SOURCE.read_bytes()
TEXT = ORIGINAL.decode()
CASES = [
    ('negative i64 endpoint', 'if negative && q == u128::from(i64::MIN.unsigned_abs()) {', 'if false && negative && q == u128::from(i64::MIN.unsigned_abs()) {', 'both_i64_endpoints_are_representable_with_either_denominator_sign'),
    ('checked quotient width', 'i64::try_from(q).map_err', 'i64::try_from(q as u64).map_err', 'extreme_i128_magnitudes_refuse_with_typed_overflow_without_panicking'),
    ('quotient sign', 'Ok(if negative { -magnitude } else { magnitude })', 'Ok(if negative { magnitude } else { -magnitude })', 'both_i64_endpoints_are_representable_with_either_denominator_sign'),
    ('half-away tie', 'if r >= denominator - r {', 'if r > denominator - r {', 'extreme_denominators_zero_and_signs_preserve_half_away_rounding'),
    ('zero denominator', 'if denominator == 0 {', 'if false && denominator == 0 {', 'extreme_denominators_zero_and_signs_preserve_half_away_rounding'),
]
try:
    for number, (name, before, after, test) in enumerate(CASES, 1):
        assert TEXT.count(before) == 1, (name, 'nonunique anchor')
        SOURCE.write_text(TEXT.replace(before, after, 1))
        result = subprocess.run(['cargo', 'test', '-p', 'sc-units', '--test', 'round_contract',
                                 test, '--', '--exact'], cwd=ROOT, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('mutation-%d.log' % number)).write_bytes(output)
        assert result.returncode == 101 and b'test result: FAILED.' in output and b'assertion' in output and b'could not compile' not in output, (name,output.decode())
        print('mutation %d %s: rc=101, actual public-contract assertion failed' % (number,name))
        SOURCE.write_bytes(ORIGINAL)
finally:
    SOURCE.write_bytes(ORIGINAL)
    restored = subprocess.run(['cargo', 'test', '-p', 'sc-units', '--test', 'round_contract'],
                              cwd=ROOT, capture_output=True)
    (WORK / 'restored.log').write_bytes(restored.stdout + restored.stderr)
    assert restored.returncode == 0, ('restored round artifact failed', restored.stderr.decode())
assert SOURCE.read_bytes() == ORIGINAL
print('Round mutations: five actual reds; source and actual compiled artifact restored')
