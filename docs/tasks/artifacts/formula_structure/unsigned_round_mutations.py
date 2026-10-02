"""Wide public rounding: compiled production faults must fail independent assertions."""
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'crates/sc-units/src/round.rs'
WORK = ROOT / 'target/formula_unsigned_round_mutations'
WORK.mkdir(exist_ok=True)
ORIGINAL = SOURCE.read_bytes()
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
    ('doubled remainder overflow', 'if r >= denominator - r {', 'if r * 2 >= denominator {',
     'full_width_inputs_and_high_remainders_round_without_narrowing'),
    ('half tie dropped', 'if r >= denominator - r {', 'if r > denominator - r {',
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
        assert result.returncode == 101 and b'test result: FAILED.' in output and b'assertion' in output and b'could not compile' not in output, (name, output.decode())
        print('unsigned round mutation %d %s: rc=101, compiled actual public assertion red' % (number, name))
        if name == 'doubled remainder overflow':
            result = subprocess.run(
                ['cargo', 'test', '-p', 'sc-units', '--release', '--test', 'unsigned_round_contract', test, '--', '--exact'],
                cwd=ROOT, capture_output=True)
            output = result.stdout + result.stderr
            (WORK / 'release-remainder.log').write_bytes(output)
            assert result.returncode == 101 and b'test result: FAILED.' in output and b'assertion' in output and b'could not compile' not in output, output.decode()
            print('unsigned round release remainder fault: rc=101, compiled actual wrap assertion red')
        SOURCE.write_bytes(ORIGINAL)
finally:
    SOURCE.write_bytes(ORIGINAL)
assert SOURCE.read_bytes() == ORIGINAL
print('unsigned round mutations: nine compiled debug reds/one release red; production source restored byte-identically')
