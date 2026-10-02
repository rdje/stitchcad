"""D82: actual reference arithmetic guards discriminate; exact restoration on every path."""
from pathlib import Path
import subprocess
import sys
ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
WORK = ROOT / 'target/formula_arithmetic_mutations'
WORK.mkdir(exist_ok=True)
ORIGINAL = SOURCE.read_bytes()
TEXT = ORIGINAL.decode()
CASES = [
    ('square precision', 'return Val(res, from_true(res, exact))', 'return Val(res, rnd(from_true(res, exact)))'),
    ('product precision', 'return Val(res, from_true(res, ta * tb))', 'return Val(res, rnd(from_true(res, ta * tb)))'),
    ('quotient precision', 'return Val(res, from_true(res, ta / tb))', 'return Val(res, rnd(from_true(res, ta / tb)))'),
    ('selector precision', 'return Val("ratio", from_true("ratio", to_true("length", l.v)\n                                         / to_true("length", e.v)))',
     'return Val("ratio", rnd(from_true("ratio", to_true("length", l.v)\n                                              / to_true("length", e.v))))'),
    ('result kind scale', 'return Val(res, from_true(res, ta * tb))', 'return Val(res, ta * tb)'),
    ('typed zero divisor', 'if tb == 0:', 'if False and tb == 0:'),
    ('taken branch', 'return self.evaluate(node[2] if c.v else node[3], env)', 'return self.evaluate(node[3] if c.v else node[2], env)'),
    ('explicit round_to', 'return Val(k0, rnd(Fraction(vs[0].v) / Fraction(step)) * step)', 'return Val(k0, vs[0].v)'),
    ('irrational quantum', 'return Val("length", rnd(dfraction(vs[0].v).sqrt()))', 'return Val("length", dfraction(vs[0].v).sqrt())'),
]
try:
    for number, (name, before, after) in enumerate(CASES, 1):
        assert TEXT.count(before) == 1, (name, 'nonunique anchor')
        SOURCE.write_text(TEXT.replace(before, after, 1))
        result = subprocess.run([sys.executable, '-I', '-B',
                                 'docs/tasks/artifacts/formula_structure/arithmetic_contract.py'],
                                cwd=ROOT, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('mutation-%d.log' % number)).write_bytes(output)
        assert result.returncode == 1 and b'AssertionError' in output and b'arithmetic_contract.py' in output and b'SyntaxError' not in output, (name, output.decode())
        print('arithmetic mutation %d %s: rc=1, actual contract assertion red' % (number, name))
        SOURCE.write_bytes(ORIGINAL)
finally:
    SOURCE.write_bytes(ORIGINAL)
assert SOURCE.read_bytes() == ORIGINAL
print('reference arithmetic mutations: nine actual reds; source restored byte-identically')
