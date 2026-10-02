"""D95: actual width/identity/quantum faults must fail independently authored controls."""
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
WORK = ROOT / 'target/formula_canonical_literal_mutations'
WORK.mkdir(exist_ok=True)
ORIGINAL = SOURCE.read_bytes()
CASES = [
    ('literal width restricted to i64', 'bound = self.limits["max_rational_bits"]',
     'bound = 63 if operation.startswith("literal") else self.limits["max_rational_bits"]'),
    ('unary sign folded into literal', 'return ("neg", self.p_unary())',
     'child = self.p_unary()\n            return ("lit", child[1], -child[2]) if child[0] == "lit" else ("neg", child)'),
    ('unary operator discarded', 'return ("neg", self.p_unary())', 'return self.p_unary()'),
    ('converted node clipped to i64', 'return ("lit", kind, rounded)',
     'return ("lit", kind, rounded % (2**63))'),
    ('bare decimal kind erased', 'return self.literal("ratio", from_true("ratio", value))',
     'return self.literal("count", from_true("ratio", value))'),
    ('canonical input fraction retained', 'return self.literal(ukind, value * factor)',
     'return ("lit", ukind, value * factor)'),
    ('canonical input tie truncated', 'rounded = Fraction(rnd(exact))',
     'rounded = Fraction(int(exact))'),
    ('128-bit endpoint excluded', 'if bits > bound:', 'if bits >= bound:'),
    ('129-bit input accepted', 'if bits > bound:', 'if False and bits > bound:'),
    ('unit width hidden by rounding', 'self.see(value * factor, "literal %s conversion" % ukind)',
     'self.see(Fraction(rnd(value * factor)), "literal %s conversion" % ukind)'),
    ('bare ratio width hidden by rounding', 'self.see(from_true("ratio", value), "literal ratio conversion")',
     'self.see(Fraction(rnd(from_true("ratio", value))), "literal ratio conversion")'),
    ('input scalar guard omitted', 'self.scalar(kind, rounded, "literal %s input" % kind)', 'None'),
]
try:
    for number, (name, before, after) in enumerate(CASES, 1):
        text = ORIGINAL.decode()
        assert text.count(before) == 1, (name, 'nonunique anchor')
        modified = text.replace(before, after, 1)
        program = modified.split("<<'PY'\n", 1)[1].rsplit('\nPY', 1)[0]
        compile(program, 'canonical_literal_mutant', 'exec')
        SOURCE.write_text(modified)
        result = subprocess.run(
            [sys.executable, '-I', '-B',
             'docs/tasks/artifacts/formula_structure/canonical_literal_contract.py'],
            cwd=ROOT, capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('mutation-%d.log' % number)).write_bytes(output)
        assert result.returncode == 1 and b'AssertionError' in output and b'SyntaxError' not in output, (
            name, output.decode())
        print('canonical literal mutation %d %s: rc=1, actual contract assertion red' % (number, name))
        SOURCE.write_bytes(ORIGINAL)
finally:
    SOURCE.write_bytes(ORIGINAL)
assert SOURCE.read_bytes() == ORIGINAL
print('canonical literal mutations: twelve compiled actual reds; source restored byte-identically')
