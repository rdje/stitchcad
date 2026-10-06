"""D85/D86/D87: actual angular guards must fail assertions; exact restoration always."""
from pathlib import Path
import subprocess
import runpy
import sys
ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
WORK = ROOT / 'target/formula_angle_mutations'
CASES = [
    ('microdegree scale', 'return dfraction(udeg) * PI / (180 * 1000000)', 'return dfraction(udeg) * PI / 180'),
    ('signed multi-turn sweep', 'return dfraction(udeg) * PI / (180 * 1000000)', 'return dfraction(norm_angle(udeg)) * PI / (180 * 1000000)'),
    ('fractional microdegree', 'return dfraction(udeg) * PI / (180 * 1000000)', 'return dfraction(int(udeg)) * PI / (180 * 1000000)'),
    ('direction nearest quantum', 'return Val("angle", norm_angle(rnd(self._deg2(q.v[1] - p.v[1], q.v[0] - p.v[0]))))', 'return Val("angle", norm_angle(self._deg2(q.v[1] - p.v[1], q.v[0] - p.v[0])))'),
    ('tan exact pole', 'if name == "tan" and vs[0].v % (180 * 1000000) == 90 * 1000000:', 'if False and name == "tan" and vs[0].v % (180 * 1000000) == 90 * 1000000:'),
    ('tan pole period', 'if name == "tan" and vs[0].v % (180 * 1000000) == 90 * 1000000:', 'if name == "tan" and vs[0].v % (360 * 1000000) == 90 * 1000000:'),
    ('tan domain token', 'raise FErr("formula_domain", "tan: angle is an exact odd-quarter-turn pole")', 'raise FErr("formula_division", "tan: angle is an exact odd-quarter-turn pole")'),
]
CONTRACT = ROOT / 'docs/tasks/artifacts/formula_structure/angle_contract.py'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
SOURCES = (Path(__file__), PROFILE_SOURCE, SOURCE, CONTRACT,
           *(ROOT / ('docs/tasks/artifacts/formula_structure/' + name) for name in (
               'formula_input.py', 'formula_structure.py', 'static_signature_contract.py', 'angle_cases.tsv')),
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
                                     'docs/tasks/artifacts/formula_structure/angle_contract.py'],
                                    cwd=ROOT, env=environment, capture_output=True)
            output = result.stdout + result.stderr
            PROFILE['plan_producer'](ROOT, **PLAN)
            (WORK / ('mutation-%d.log' % number)).write_bytes(output)
            assert result.returncode == 1 and b'AssertionError' in output and b'angle_contract.py' in output and b'SyntaxError' not in output, (name, output.decode())
            print('angle mutation %d %s: rc=1, actual contract assertion red' % (number, name))
            PROFILE['plan_producer'](ROOT, sources=(SOURCE,), mutable_sources=(SOURCE,))
            SOURCE.write_bytes(ORIGINAL)
    finally:
        PROFILE['plan_producer'](ROOT, sources=(SOURCE,), mutable_sources=(SOURCE,))
        SOURCE.write_bytes(ORIGINAL)
    assert SOURCE.read_bytes() == ORIGINAL
    print('reference angle mutations: seven actual reds; source restored byte-identically')
except (PROFILE['Refusal'], OSError) as error:
    print('local-environment: REFUSED — ' + str(error), file=sys.stderr)
    raise SystemExit(2) from None
