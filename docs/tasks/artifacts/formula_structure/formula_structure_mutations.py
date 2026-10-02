"""D75: actual reference walker/depth guard mutations must fail controlled assertions."""
from pathlib import Path
import subprocess
import sys
ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
WORK = ROOT / 'target/formula_structure_mutations'
WORK.mkdir(exist_ok=True)
ORIGINAL = SOURCE.read_bytes()
TEXT = ORIGINAL.decode()
CASES = [
    ('call argument children', 'if node[0] == "call":', 'if False and node[0] == "call":'),
    ('node descendant traversal', 'pending.extend(self.syntax_children(current))', 'pending.extend([])'),
    ('depth descendant traversal', 'pending.extend((child, depth) for child in self.syntax_children(current))', 'pending.extend([])'),
    ('early depth refusal', 'if depth > self.limits["max_if_depth"]:', 'if False and depth > self.limits["max_if_depth"]:'),
]
try:
    for number, (name, before, after) in enumerate(CASES, 1):
        assert TEXT.count(before) == 1, (name, 'nonunique mutation anchor')
        SOURCE.write_text(TEXT.replace(before, after, 1))
        result = subprocess.run([sys.executable, '-I', '-B',
                                 'docs/tasks/artifacts/formula_structure/formula_structure.py',
                                 '--contracts-only'], cwd=ROOT, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('mutation-%d.log' % number)).write_bytes(output)
        assert result.returncode == 1 and b'AssertionError' in output and b'formula_structure.py' in output and b'SyntaxError' not in output, (name, output.decode())
        print('mutation %d %s: rc=1, structural contract assertion failed' % (number, name))
        SOURCE.write_bytes(ORIGINAL)
finally:
    SOURCE.write_bytes(ORIGINAL)
assert SOURCE.read_bytes() == ORIGINAL
print('formula structural mutations: four actual reds; reference source restored byte-identically')
