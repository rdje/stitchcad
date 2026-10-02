"""D79: actual reference guards must fail literal assertions; restore exact source bytes."""
from pathlib import Path
import subprocess
import sys
ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
WORK = ROOT / 'target/formula_literal_mutations'
WORK.mkdir(exist_ok=True)
ORIGINAL = SOURCE.read_bytes()
TEXT = ORIGINAL.decode()
CASES = [
    ('unit input quantum', 'return self.literal(ukind, value * factor)',
     'return ("lit", ukind, value * factor)'),
    ('bare decimal quantum', 'return self.literal("ratio", from_true("ratio", value))',
     'return ("lit", "ratio", from_true("ratio", value))'),
    ('tie away from zero', 'if 2 * r >= d: q += 1', 'if 2 * r > d: q += 1'),
    ('ratio scale', 'return Fraction(x) * RATIO_SCALE if kind == "ratio" else Fraction(x)',
     'return Fraction(x) * (RATIO_SCALE - 1) if kind == "ratio" else Fraction(x)'),
    ('count kind', 'return ("lit", "count", value)', 'return ("lit", "ratio", value)'),
    ('direct unit factor', 'return self.literal(ukind, value * factor)',
     'return self.literal(ukind, value / factor)'),
]
try:
    for number, (name, before, after) in enumerate(CASES, 1):
        assert TEXT.count(before) == 1, (name, 'nonunique anchor')
        SOURCE.write_text(TEXT.replace(before, after, 1))
        result = subprocess.run([sys.executable, '-I', '-B',
                                 'docs/tasks/artifacts/formula_structure/literal_contract.py'],
                                cwd=ROOT, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('mutation-%d.log' % number)).write_bytes(output)
        assert result.returncode == 1 and b'AssertionError' in output and b'literal_contract.py' in output and b'SyntaxError' not in output, (name, output.decode())
        print('literal mutation %d %s: rc=1, actual contract assertion red' % (number, name))
        SOURCE.write_bytes(ORIGINAL)
finally:
    SOURCE.write_bytes(ORIGINAL)
assert SOURCE.read_bytes() == ORIGINAL
print('reference literal mutations: six actual reds; source restored byte-identically')
