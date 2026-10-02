"""D83: mutate actual rational guards; require assertion reds and exact restoration."""
from pathlib import Path
import subprocess
import sys
ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
WORK = ROOT / 'target/formula_rational_mutations'
WORK.mkdir(exist_ok=True)
ORIGINAL = SOURCE.read_bytes()
TEXT = ORIGINAL.decode()
CASES = [
    ('width refusal', 'if bits > bound:', 'if False and bits > bound:'),
    ('inclusive boundary', 'if bits > bound:', 'if bits >= bound:'),
    ('published bound', 'bound = self.limits["max_rational_bits"]', 'bound = 128'),
    ('numerator width', 'bits = max(abs(fr.numerator).bit_length(), fr.denominator.bit_length())', 'bits = fr.denominator.bit_length()'),
    ('denominator width', 'bits = max(abs(fr.numerator).bit_length(), fr.denominator.bit_length())', 'bits = abs(fr.numerator).bit_length()'),
    ('converted unit before round', 'self.see(value * factor, "literal %s conversion" % ukind)', 'self.see(Fraction(rnd(value * factor)), "literal %s conversion" % ukind)'),
    ('converted bare ratio before round', 'self.see(from_true("ratio", value), "literal ratio conversion")', 'self.see(Fraction(rnd(from_true("ratio", value))), "literal ratio conversion")'),
    ('each completed numeric value', 'if value.kind in ARITH:', 'if False and value.kind in ARITH:'),
    ('internal result not temporary', 'return Val(res, from_true(res, ta * tb))', 'self.see(ta * tb, "temporary")\n                return Val(res, from_true(res, ta * tb))'),
    ('within tolerance read', 'tol = self.evaluate(args[2], env)', 'tol = self.value_of_name(args[2][1], env)'),
    ('assert tolerance read', 'tol = self.evaluate(("name", tol_name), env)', 'tol = self.value_of_name(tol_name, env)'),
    ('typed token', 'raise FErr("formula_domain", "%s: rational max_rational_bits=%d, measured=%d"', 'raise FErr("formula_division", "%s: rational max_rational_bits=%d, measured=%d"'),
]
try:
    for number, (name, before, after) in enumerate(CASES, 1):
        assert TEXT.count(before) == 1, (name, 'nonunique anchor')
        SOURCE.write_text(TEXT.replace(before, after, 1))
        result = subprocess.run([sys.executable, '-I', '-B',
                                 'docs/tasks/artifacts/formula_structure/rational_contract.py'],
                                cwd=ROOT, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('mutation-%d.log' % number)).write_bytes(output)
        assert result.returncode == 1 and b'AssertionError' in output and b'rational_contract.py' in output and b'SyntaxError' not in output, (name, output.decode())
        print('rational mutation %d %s: rc=1, actual contract assertion red' % (number, name))
        SOURCE.write_bytes(ORIGINAL)
finally:
    SOURCE.write_bytes(ORIGINAL)
assert SOURCE.read_bytes() == ORIGINAL
print('reference rational mutations: twelve actual reds; source restored byte-identically')
