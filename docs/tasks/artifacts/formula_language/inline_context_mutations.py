"""D91: mutate actual classifier guards, require contract assertion reds and exact restoration."""
from pathlib import Path
import subprocess
import sys
ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
WORK = ROOT / 'target/formula_inline_mutations'
WORK.mkdir(exist_ok=True)
ORIGINAL = SOURCE.read_bytes()
TEXT = ORIGINAL.decode()
CASES = [
    ('foreign context removed', 'if span.start() not in excluded', 'if True'),
    ('whole-line exemption', '        out.extend(span.group(1)', '        if excluded: continue\n        out.extend(span.group(1)'),
    ('normative exemption', 'if not allow_foreign or not valid:', 'if not valid:'),
    ('annotation verdict omitted', 'l6 = fails - l6_start', 'l6 = l6'),
    ('unknown language accepted', 'marker.group() == "<!-- stitchcad-inline: rust -->" and next_span', 'next_span'),
]
try:
    for number, (name, before, after) in enumerate(CASES, 1):
        assert TEXT.count(before) == 1, (name, 'nonunique anchor')
        SOURCE.write_text(TEXT.replace(before, after, 1))
        result = subprocess.run([sys.executable, '-I', '-B',
            'docs/tasks/artifacts/formula_language/inline_context_contract.py'], cwd=ROOT, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('mutation-%d.log' % number)).write_bytes(output)
        assert result.returncode == 1 and b'AssertionError' in output and b'SyntaxError' not in output, (name, output.decode())
        print('inline mutation %d %s: rc=1, actual contract assertion red' % (number, name))
        SOURCE.write_bytes(ORIGINAL)
finally: SOURCE.write_bytes(ORIGINAL)
assert SOURCE.read_bytes() == ORIGINAL
print('inline context mutations: five actual reds; classifier restored byte-identically')
