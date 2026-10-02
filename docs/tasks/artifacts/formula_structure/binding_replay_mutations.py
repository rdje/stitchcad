"""D99: mutate actual consumer/display paths, require independent assertion reds."""
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
WORK = ROOT / 'target/formula_binding_replay_mutations'
WORK.mkdir(exist_ok=True)
ORIGINAL = SOURCE.read_bytes()
CASES = [
    ('four-kind whitelist', 'if val.kind not in EV.bindable:',
     'if val.kind not in ("length", "angle", "ratio", "count"):'),
    ('wrong Area display quantum', '"area": 100000000', '"area": 1000000'),
    ('Boolean display reversed', 'return "true" if internal == 1 else "false"',
     'return "false" if internal == 1 else "true"'),
    ('invalid Boolean state hidden', 'if internal not in (0, 1):', 'if False:'),
    ('scientific display spelling',
     'return format(value.quantize(Decimal(1).scaleb(-dp), rounding=ROUND_HALF_UP), "f")',
     'return str(value.quantize(Decimal(1).scaleb(-dp), rounding=ROUND_HALF_UP))'),
    ('consumer changes stored quantum', 'internal = val.v.numerator',
     'internal = val.v.numerator + 1'),
    ('Boolean name loses kind',
     'env[token] = {"kind": val.kind, "value": internal, "origin": "recipe", "state": "derived"}',
     'env[token] = {"kind": "count" if val.kind == "boolean" else val.kind, "value": internal, "origin": "recipe", "state": "derived"}'),
    ('Area unit relabeled', '"cm²": "area", None: val.kind',
     '"cm²": "length", None: val.kind'),
    ('Boolean value mismatch ignored', 'if got != expected:',
     'if got != expected and val.kind != "boolean":'),
]
try:
    for number, (name, before, after) in enumerate(CASES, 1):
        text = ORIGINAL.decode()
        assert text.count(before) == 1, (name, 'nonunique actual anchor')
        modified = text.replace(before, after, 1)
        program = modified.split("<<'PY'\n", 1)[1].rsplit('\nPY', 1)[0]
        compile(program, 'binding_replay_mutant', 'exec')
        SOURCE.write_text(modified)
        result = subprocess.run(
            [sys.executable, '-I', '-B', 'docs/tasks/artifacts/formula_structure/binding_replay_contract.py'],
            cwd=ROOT, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('mutation-%d.log' % number)).write_bytes(output)
        assert result.returncode == 1 and b'AssertionError' in output and b'binding_replay_contract.py' in output and b'SyntaxError' not in output, (name, output.decode())
        print('binding replay mutation %d %s: rc=1, actual contract assertion red' % (number, name))
        SOURCE.write_bytes(ORIGINAL)
finally:
    SOURCE.write_bytes(ORIGINAL)
assert SOURCE.read_bytes() == ORIGINAL
print('reference binding replay mutations: nine compiled actual reds; source restored byte-identically')
