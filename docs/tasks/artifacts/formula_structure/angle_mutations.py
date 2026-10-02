"""D85/D86/D87: actual angular guards must fail assertions; exact restoration always."""
from pathlib import Path
import subprocess
import sys
ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
WORK = ROOT / 'target/formula_angle_mutations'
WORK.mkdir(exist_ok=True)
ORIGINAL = SOURCE.read_bytes()
TEXT = ORIGINAL.decode()
CASES = [
    ('microdegree scale', 'return dfraction(udeg) * PI / (180 * 1000000)', 'return dfraction(udeg) * PI / 180'),
    ('signed multi-turn sweep', 'return dfraction(udeg) * PI / (180 * 1000000)', 'return dfraction(norm_angle(udeg)) * PI / (180 * 1000000)'),
    ('fractional microdegree', 'return dfraction(udeg) * PI / (180 * 1000000)', 'return dfraction(int(udeg)) * PI / (180 * 1000000)'),
    ('direction nearest quantum', 'return Val("angle", norm_angle(rnd(self._deg2(q.v[1] - p.v[1], q.v[0] - p.v[0]))))', 'return Val("angle", norm_angle(self._deg2(q.v[1] - p.v[1], q.v[0] - p.v[0])))'),
    ('tan exact pole', 'if name == "tan" and vs[0].v % (180 * 1000000) == 90 * 1000000:', 'if False and name == "tan" and vs[0].v % (180 * 1000000) == 90 * 1000000:'),
    ('tan pole period', 'if name == "tan" and vs[0].v % (180 * 1000000) == 90 * 1000000:', 'if name == "tan" and vs[0].v % (360 * 1000000) == 90 * 1000000:'),
    ('tan domain token', 'raise FErr("formula_domain", "tan: angle is an exact odd-quarter-turn pole")', 'raise FErr("formula_division", "tan: angle is an exact odd-quarter-turn pole")'),
]
try:
    for number, (name, before, after) in enumerate(CASES, 1):
        assert TEXT.count(before) == 1, (name, 'nonunique anchor')
        SOURCE.write_text(TEXT.replace(before, after, 1))
        result = subprocess.run([sys.executable, '-I', '-B',
                                 'docs/tasks/artifacts/formula_structure/angle_contract.py'],
                                cwd=ROOT, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('mutation-%d.log' % number)).write_bytes(output)
        assert result.returncode == 1 and b'AssertionError' in output and b'angle_contract.py' in output and b'SyntaxError' not in output, (name, output.decode())
        print('angle mutation %d %s: rc=1, actual contract assertion red' % (number, name))
        SOURCE.write_bytes(ORIGINAL)
finally:
    SOURCE.write_bytes(ORIGINAL)
assert SOURCE.read_bytes() == ORIGINAL
print('reference angle mutations: seven actual reds; source restored byte-identically')
