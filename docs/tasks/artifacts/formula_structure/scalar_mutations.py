"""D83: mutate actual scalar guards; require assertion reds and exact restoration."""
from pathlib import Path
import subprocess
import runpy
import sys
ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
LOADER = ROOT / 'docs/tasks/artifacts/formula_structure/formula_input.py'
WORK = ROOT / 'target/formula_scalar_mutations'
ENTRY = ROOT / 'docs/tasks/artifacts/formula_structure/scalar_contract.py'
PROVIDER_SOURCE = ROOT / 'docs/tasks/artifacts/formula_structure/scalar_fixture.py'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
BOOT_INPUTS = (Path(__file__), PROFILE_SOURCE, SOURCE, LOADER, ENTRY, PROVIDER_SOURCE)
try:
    PROFILE['plan_producer'](ROOT, sources=BOOT_INPUTS)
except (PROFILE['Refusal'], OSError) as error:
    print('local-environment: REFUSED — ' + str(error), file=sys.stderr)
    raise SystemExit(2) from None
FIXTURE = runpy.run_path(str(PROVIDER_SOURCE))
PROFILE = FIXTURE['PROFILE']


def producer_plan():
    child = FIXTURE['scalar_plan']()
    plan = dict(directories=(WORK, *child['directories']),
                output_files=(*child['output_files'], *(WORK / ('mutation-%d.log' % n) for n in range(1, 12))),
                sources=tuple(dict.fromkeys((*BOOT_INPUTS, *child['sources']))),
                mutable_sources=(SOURCE, LOADER))
    PROFILE['plan_producer'](ROOT, **plan)
    return plan

CASES = [
    ('input phase guard', 'self.scalar(kind, rounded, "literal %s input" % kind)', 'None'),
    ('completed value guard', 'self.scalar(value.kind, value.v, "%s %s result" % (operation, value.kind))', 'None'),
    ('domain guard omitted', 'if value < low or (high is not None and value > high):', 'if False:'),
    ('lower endpoint excluded', 'if value < low or (high is not None and value > high):', 'if value <= low or (high is not None and value > high):'),
    ('upper endpoint excluded', 'if value < low or (high is not None and value > high):', 'if value < low or (high is not None and value >= high):'),
    ('negative count accepted', 'domains["count"] = (0, None)', 'domains["count"] = (-1, None)'),
    ('fraction hidden by rounding', 'value = Fraction(value)', 'value = Fraction(rnd(value))'),
    ('duplicated numeric limits', 'bound = 10 ** int(match[1].translate(digits))', 'bound = 10 ** (9 if kind == "length" else 18)'),
    ('typed token', 'raise FErr("formula_domain", "%s: %s domain=%s, measured=%s"', 'raise FErr("formula_division", "%s: %s domain=%s, measured=%s"'),
    ('quiet context', '"""Table-driven numeric fixture context only; never execute a contract as an import."""', '"""Table-driven numeric fixture context only; never execute a contract as an import."""\n    print("unexpected contract output")'),
    ('actual operation context', 'node[0] in ("name", "bin", "call")', 'False'),
]
try:
    plan = producer_plan()
    environment = PROFILE['prepare_producer'](ROOT, **plan)
    WORK.mkdir(exist_ok=True)
    ORIGINAL = {path: path.read_bytes() for path in [SOURCE, LOADER]}
    try:
        for number, (name, before, after) in enumerate(CASES, 1):
            target = LOADER if name == 'quiet context' else SOURCE
            TEXT = ORIGINAL[target].decode()
            assert TEXT.count(before) == 1, (name, 'nonunique anchor')
            producer_plan()
            target.write_text(TEXT.replace(before, after, 1))
            producer_plan()
            result = subprocess.run([sys.executable, '-I', '-B',
                'docs/tasks/artifacts/formula_structure/scalar_contract.py'], cwd=ROOT, env=environment, capture_output=True)
            output = result.stdout + result.stderr
            producer_plan()
            (WORK / ('mutation-%d.log' % number)).write_bytes(output)
            assert result.returncode == 1 and b'AssertionError' in output and b'SyntaxError' not in output, (name, output.decode())
            print('scalar mutation %d %s: rc=1, actual contract assertion red' % (number, name))
            PROFILE['plan_producer'](ROOT, sources=(target,), mutable_sources=(target,))
            target.write_bytes(ORIGINAL[target])
    finally:
        for path, data in ORIGINAL.items():
            PROFILE['plan_producer'](ROOT, sources=(path,), mutable_sources=(path,))
            path.write_bytes(data)
    assert all(path.read_bytes() == data for path, data in ORIGINAL.items())
    print('reference scalar mutations: eleven actual reds; all sources restored byte-identically')
except (PROFILE['Refusal'], OSError) as error:
    print('local-environment: REFUSED — ' + str(error), file=sys.stderr)
    raise SystemExit(2) from None
