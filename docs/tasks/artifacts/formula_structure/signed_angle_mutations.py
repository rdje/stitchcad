"""D84: compiled actual signed-angle faults require independent contract assertion reds."""
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
WORK = ROOT / 'target/formula_signed_angle_mutations'
WORK.mkdir(exist_ok=True)
ORIGINAL = SOURCE.read_bytes()
CASES = [
    ('atan modulo',
     'return Val("angle", rnd(d_atan(dfraction(to_true("ratio", vs[0].v)))\n                                     * 180 / PI * 1000000))',
     'return Val("angle", norm_angle(rnd(d_atan(dfraction(to_true("ratio", vs[0].v))) * 180 / PI * 1000000)))'),
    ('atan2 modulo', 'return Val("angle", rnd(d_atan2(ya, xb) * 180 / PI * 1000000))',
     'return Val("angle", norm_angle(rnd(d_atan2(ya, xb) * 180 / PI * 1000000)))'),
    ('atan sign lost', 'return -v if neg else v', 'return abs(v)'),
    ('angle binding modulo', 'integer = rnd(value.v)',
     'integer = norm_angle(rnd(value.v)) if value.kind == "angle" else rnd(value.v)'),
    ('angle comparison modulo', 'ok = {"==": a.v == b.v,',
     'if a.kind == "angle":\n                a, b = Val("angle", norm_angle(a.v)), Val("angle", norm_angle(b.v))\n            ok = {"==": a.v == b.v,'),
    ('arc input modulo', 'if name == "arc_length":\n            rad = d_radians(vs[0].v)',
     'if name == "arc_length":\n            rad = d_radians(norm_angle(vs[0].v))'),
    ('dir no normalization',
     'return Val("angle", norm_angle(rnd(self._deg2(q.v[1] - p.v[1], q.v[0] - p.v[0]))))',
     'return Val("angle", rnd(self._deg2(q.v[1] - p.v[1], q.v[0] - p.v[0])))'),
    ('atan2 argument order', 'd_atan2(ya, xb)', 'd_atan2(xb, ya)'),
    ('zero branch wrong sign', 'PI if y >= 0 else -PI', 'PI if y > 0 else -PI'),
    ('atan truncation', 'rnd(d_atan(dfraction(to_true("ratio", vs[0].v)))',
     'int(d_atan(dfraction(to_true("ratio", vs[0].v)))'),
    ('atan2 truncation', 'rnd(d_atan2(ya, xb) * 180 / PI * 1000000)',
     'int(d_atan2(ya, xb) * 180 / PI * 1000000)'),
    ('binding tie truncation', 'integer = rnd(value.v)', 'integer = int(value.v)'),
    ('angle literal modulo', 'return ("lit", kind, rounded)',
     'return ("lit", kind, norm_angle(rounded) if kind == "angle" else rounded)'),
    ('negative vertical axis lost', 'if y < 0: return -PI / 2', 'if y < 0: return PI / 2'),
    ('zero vector accepted', 'raise FErr("formula_domain", "atan2(0, 0) has no direction")',
     'return Decimal(0)'),
]
try:
    for number, (name, before, after) in enumerate(CASES, 1):
        text = ORIGINAL.decode()
        assert text.count(before) == 1, (name, 'nonunique actual source anchor')
        modified = text.replace(before, after, 1)
        program = modified.split("<<'PY'\n", 1)[1].rsplit('\nPY', 1)[0]
        compile(program, 'signed_angle_mutant', 'exec')
        SOURCE.write_text(modified)
        result = subprocess.run(
            [sys.executable, '-I', '-B', 'docs/tasks/artifacts/formula_structure/signed_angle_contract.py'],
            cwd=ROOT, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('mutation-%d.log' % number)).write_bytes(output)
        assert result.returncode == 1 and b'AssertionError' in output and b'signed_angle_contract.py' in output and b'SyntaxError' not in output, (name, output.decode())
        print('signed angle mutation %d %s: rc=1, actual contract assertion red' % (number, name))
        SOURCE.write_bytes(ORIGINAL)
finally:
    SOURCE.write_bytes(ORIGINAL)
assert SOURCE.read_bytes() == ORIGINAL
print('reference signed-angle mutations: fifteen compiled actual reds; source restored byte-identically')
