"""D83: mutate actual scalar guards; require assertion reds and exact restoration."""
from pathlib import Path
import subprocess
import sys
ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
LOADER = ROOT / 'docs/tasks/artifacts/formula_structure/formula_input.py'
WORK = ROOT / 'target/formula_scalar_mutations'
WORK.mkdir(exist_ok=True)
ORIGINAL = {path: path.read_bytes() for path in [SOURCE, LOADER]}
CASES = [
    ('input phase guard', 'self.scalar(kind, rounded, "literal %s input" % kind)', 'None'),
    ('completed value guard', 'self.scalar(value.kind, value.v, "%s %s result" % (operation, value.kind))', 'None'),
    ('domain guard omitted', 'if value < low or (high is not None and value > high):', 'if False:'),
    ('lower endpoint excluded', 'if value < low or (high is not None and value > high):', 'if value <= low or (high is not None and value > high):'),
    ('upper endpoint excluded', 'if value < low or (high is not None and value > high):', 'if value < low or (high is not None and value >= high):'),
    ('negative count accepted', 'domains["count"] = (0, None)', 'domains["count"] = (-1, None)'),
    ('fraction hidden by rounding', 'value = Fraction(value)', 'value = Fraction(rnd(value))'),
    ('duplicated numeric limits', 'bound = 10 ** int(match[1].translate(digits))', 'bound = 10 ** (9 if kind == "length" else 18)'),
    ('typed token', 'raise FErr("formula_domain", "%s: %s domain=%s, measured=%s"', 'raise FErr("formula_division", "%s: %s domain=%s, measured=%s"'),
    ('quiet context', '"""Table-driven numeric fixture context only; never execute a contract as an import."""', '"""Table-driven numeric fixture context only; never execute a contract as an import."""\n    print("unexpected contract output")'),
    ('actual operation context', 'node[0] in ("name", "bin", "call")', 'False'),
]
try:
    for number, (name, before, after) in enumerate(CASES, 1):
        target = LOADER if name == 'quiet context' else SOURCE
        TEXT = ORIGINAL[target].decode()
        assert TEXT.count(before) == 1, (name, 'nonunique anchor')
        target.write_text(TEXT.replace(before, after, 1))
        result = subprocess.run([sys.executable, '-I', '-B',
            'docs/tasks/artifacts/formula_structure/scalar_contract.py'], cwd=ROOT, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('mutation-%d.log' % number)).write_bytes(output)
        assert result.returncode == 1 and b'AssertionError' in output and b'SyntaxError' not in output, (name, output.decode())
        print('scalar mutation %d %s: rc=1, actual contract assertion red' % (number, name))
        target.write_bytes(ORIGINAL[target])
finally:
    for path, data in ORIGINAL.items(): path.write_bytes(data)
assert all(path.read_bytes() == data for path, data in ORIGINAL.items())
print('reference scalar mutations: eleven actual reds; all sources restored byte-identically')
