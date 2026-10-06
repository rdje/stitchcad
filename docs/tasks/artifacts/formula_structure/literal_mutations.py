"""D79: actual reference guards must fail literal assertions; restore exact source bytes."""
from pathlib import Path
import subprocess
import runpy
import sys
ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
WORK = ROOT / 'target/formula_literal_mutations'
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
CONTRACT = ROOT / 'docs/tasks/artifacts/formula_structure/literal_contract.py'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
SOURCES = (Path(__file__), PROFILE_SOURCE, SOURCE, CONTRACT,
           *(ROOT / ('docs/tasks/artifacts/formula_structure/' + name) for name in (
               'formula_input.py', 'formula_structure.py', 'static_signature_contract.py', 'literal_cases.tsv')),
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
                                     'docs/tasks/artifacts/formula_structure/literal_contract.py'],
                                    cwd=ROOT, env=environment, capture_output=True)
            output = result.stdout + result.stderr
            PROFILE['plan_producer'](ROOT, **PLAN)
            (WORK / ('mutation-%d.log' % number)).write_bytes(output)
            assert result.returncode == 1 and b'AssertionError' in output and b'literal_contract.py' in output and b'SyntaxError' not in output, (name, output.decode())
            print('literal mutation %d %s: rc=1, actual contract assertion red' % (number, name))
            PROFILE['plan_producer'](ROOT, sources=(SOURCE,), mutable_sources=(SOURCE,))
            SOURCE.write_bytes(ORIGINAL)
    finally:
        PROFILE['plan_producer'](ROOT, sources=(SOURCE,), mutable_sources=(SOURCE,))
        SOURCE.write_bytes(ORIGINAL)
    assert SOURCE.read_bytes() == ORIGINAL
    print('reference literal mutations: six actual reds; source restored byte-identically')
except (PROFILE['Refusal'], OSError) as error:
    print('local-environment: REFUSED — ' + str(error), file=sys.stderr)
    raise SystemExit(2) from None
