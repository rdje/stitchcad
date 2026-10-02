"""D76: actual reference input guards must discriminate; restore exact production bytes."""
from pathlib import Path
import subprocess
import sys
ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
WORK = ROOT / 'target/formula_input_mutations'
WORK.mkdir(exist_ok=True)
ORIGINAL = SOURCE.read_bytes()
TEXT = ORIGINAL.decode()
CASES = [
    ('ASCII preflight', 'if not src.isascii():', 'if False and not src.isascii():'),
    ('identifier spelling', 'if not re.fullmatch(r"[a-z][a-z0-9]*(?:_[a-z0-9]+)*", text):', 'if False and not re.fullmatch(r"[a-z][a-z0-9]*(?:_[a-z0-9]+)*", text):'),
    ('keyword reservation', 'if not allow_keyword and text in {"let", "assert", "if"}:', 'if False and not allow_keyword and text in {"let", "assert", "if"}:'),
    ('bare keyword position', '            self._identifier(text)\n            self.take()', '            self.take()'),
    ('call keyword position', 'if name != "if":', 'if False and name != "if":'),
    ('declaration keyword position', 'return self._identifier(self._want("id"))', 'return self._want("id")'),
    ('original unit separator', 'if gap != " ":', 'if False and gap != " ":'),
    ('nonempty call arguments', 'raise FErr("formula_parse", "call arguments require at least one expression")', 'self.take(); return args'),
    ('general ASCII whitespace', r'(?P<sp>[ \t\n\r\f\v]+)', r'(?P<sp>[ \t]+)'),
]
try:
    for number, (name, before, after) in enumerate(CASES, 1):
        assert TEXT.count(before) == 1, (name, 'nonunique mutation anchor')
        SOURCE.write_text(TEXT.replace(before, after, 1))
        result = subprocess.run([sys.executable, '-I', '-B',
                                 'docs/tasks/artifacts/formula_structure/formula_input.py',
                                 '--contracts-only'], cwd=ROOT, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('mutation-%d.log' % number)).write_bytes(output)
        assert result.returncode == 1 and b'AssertionError' in output and b'formula_input.py' in output and b'SyntaxError' not in output, (name, output.decode())
        print('mutation %d %s: rc=1, machine-input contract assertion failed' % (number, name))
        SOURCE.write_bytes(ORIGINAL)
finally:
    SOURCE.write_bytes(ORIGINAL)
assert SOURCE.read_bytes() == ORIGINAL
print('formula input mutations: nine actual reds; reference source restored byte-identically')
