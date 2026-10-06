"""D82: actual reference arithmetic guards discriminate; exact restoration on every path."""
from pathlib import Path
import subprocess
import runpy
import sys
ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
WORK = ROOT / 'target/formula_arithmetic_mutations'
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
CONTRACT = ROOT / 'docs/tasks/artifacts/formula_structure/arithmetic_contract.py'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
SOURCES = (Path(__file__), PROFILE_SOURCE, SOURCE, CONTRACT,
           *(ROOT / ('docs/tasks/artifacts/formula_structure/' + name) for name in (
               'formula_input.py', 'formula_structure.py', 'static_signature_contract.py', 'arithmetic_cases.tsv')),
           *(ROOT / ('docs/book/src/' + name) for name in (
               'spec/formula-language.md', 'spec/formula-language/grammar.md',
               'spec/formula-language/examples.md', 'spec/reference-skirt.md',
               'spec/units-and-tolerances.md')))
OUTPUT_FILES = tuple(WORK / ('mutation-%d.log' % index) for index in range(1, len(CASES) + 1))
PLAN = dict(directories=(WORK,), output_files=OUTPUT_FILES, sources=SOURCES, mutable_sources=(SOURCE,))
environment = PROFILE['enter_producer'](ROOT, **PLAN)
ORIGINAL = SOURCE.read_bytes()
TEXT = ORIGINAL.decode()
WORK.mkdir(exist_ok=True)

try:
    try:
        for number, (name, before, after) in enumerate(CASES, 1):
            assert TEXT.count(before) == 1, (name, 'nonunique anchor')
            PROFILE['plan_producer'](ROOT, **PLAN)
            SOURCE.write_text(TEXT.replace(before, after, 1))
            PROFILE['plan_producer'](ROOT, **PLAN)
            result = subprocess.run([sys.executable, '-I', '-B',
                                     'docs/tasks/artifacts/formula_structure/arithmetic_contract.py'],
                                    cwd=ROOT, env=environment, capture_output=True)
            output = result.stdout + result.stderr
            PROFILE['plan_producer'](ROOT, **PLAN)
            (WORK / ('mutation-%d.log' % number)).write_bytes(output)
            assert result.returncode == 1 and b'AssertionError' in output and b'arithmetic_contract.py' in output and b'SyntaxError' not in output, (name, output.decode())
            print('arithmetic mutation %d %s: rc=1, actual contract assertion red' % (number, name))
            PROFILE['plan_producer'](ROOT, sources=(SOURCE,), mutable_sources=(SOURCE,))
            SOURCE.write_bytes(ORIGINAL)
    finally:
        PROFILE['plan_producer'](ROOT, sources=(SOURCE,), mutable_sources=(SOURCE,))
        SOURCE.write_bytes(ORIGINAL)
    assert SOURCE.read_bytes() == ORIGINAL
    print('reference arithmetic mutations: nine actual reds; source restored byte-identically')
except (PROFILE['Refusal'], OSError) as error:
    print('local-environment: REFUSED — ' + str(error), file=sys.stderr)
    raise SystemExit(2) from None
