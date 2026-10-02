"""D89: mutate actual public length operators; require compiled assertion reds and restore bytes."""
from pathlib import Path
import subprocess
ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'crates/sc-units/src/length.rs'
WORK = ROOT / 'target/length_operator_mutations'
WORK.mkdir(exist_ok=True)
ORIGINAL = SOURCE.read_bytes()
TEXT = ORIGINAL.decode()
CASES = [
    ('addition domain bypass', 'self.checked_add(rhs)', 'Ok(Self(self.0 + rhs.0))', 'signed_addition_crossings_refuse_instead_of_constructing_invalid_lengths'),
    ('subtraction domain bypass', 'self.checked_sub(rhs)', 'Ok(Self(self.0 - rhs.0))', 'signed_subtraction_crossings_refuse_instead_of_constructing_invalid_lengths'),
    ('addition operation', 'self.checked_add(rhs)', 'self.checked_sub(rhs)', 'signed_addition_crossings_refuse_instead_of_constructing_invalid_lengths'),
    ('subtraction operation', 'self.checked_sub(rhs)', 'self.checked_add(rhs)', 'signed_subtraction_crossings_refuse_instead_of_constructing_invalid_lengths'),
    ('addition saturation', 'self.checked_add(rhs)', 'Ok(Self((self.0 + rhs.0).clamp(-MAX_LENGTH_UM, MAX_LENGTH_UM)))', 'signed_addition_crossings_refuse_instead_of_constructing_invalid_lengths'),
    ('subtraction saturation', 'self.checked_sub(rhs)', 'Ok(Self((self.0 - rhs.0).clamp(-MAX_LENGTH_UM, MAX_LENGTH_UM)))', 'signed_subtraction_crossings_refuse_instead_of_constructing_invalid_lengths'),
]
try:
    for number, (name, before, after, test) in enumerate(CASES, 1):
        assert TEXT.count(before) == 1, (name, 'nonunique anchor')
        SOURCE.write_text(TEXT.replace(before, after, 1))
        result = subprocess.run(['cargo', 'test', '-p', 'sc-units', '--test', 'length_operator_contract',
                                 test, '--', '--exact'], cwd=ROOT, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('mutation-%d.log' % number)).write_bytes(output)
        assert result.returncode == 101 and b'test result: FAILED.' in output and b'assertion' in output and b'could not compile' not in output, (name, output.decode())
        print('length operator mutation %d %s: rc=101, actual public-contract assertion failed' % (number, name))
        SOURCE.write_bytes(ORIGINAL)
finally:
    SOURCE.write_bytes(ORIGINAL)
assert SOURCE.read_bytes() == ORIGINAL
print('Length operator mutations: six actual reds; production source restored byte-identically')
