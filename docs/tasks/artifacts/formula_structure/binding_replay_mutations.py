"""D99: mutate actual consumer/display paths, require independent assertion reds."""
from pathlib import Path
import subprocess
import runpy
import sys

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
WORK = ROOT / 'target/formula_binding_replay_mutations'
ENTRY = ROOT / 'docs/tasks/artifacts/formula_structure/binding_replay_contract.py'
PROVIDER_SOURCE = ROOT / 'docs/tasks/artifacts/formula_structure/binding_replay_fixture.py'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
BOOT_INPUTS = (Path(__file__), PROFILE_SOURCE, SOURCE, ENTRY, PROVIDER_SOURCE)
try:
    PROFILE['plan_producer'](ROOT, sources=BOOT_INPUTS)
except (PROFILE['Refusal'], OSError) as error:
    print('local-environment: REFUSED — ' + str(error), file=sys.stderr)
    raise SystemExit(2) from None
FIXTURE = runpy.run_path(str(PROVIDER_SOURCE))
PROFILE = FIXTURE['PROFILE']


def producer_plan():
    child = FIXTURE['fixture_plan']()
    plan = dict(directories=(WORK, *child['directories']),
                output_files=(*child['output_files'], *(WORK / ('mutation-%d.log' % n) for n in range(1, 10))),
                sources=tuple(dict.fromkeys((*BOOT_INPUTS, *child['sources']))),
                mutable_sources=(SOURCE,))
    PROFILE['plan_producer'](ROOT, **plan)
    return plan

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
    plan = producer_plan()
    environment = PROFILE['prepare_producer'](ROOT, **plan)
    WORK.mkdir(exist_ok=True)
    ORIGINAL = SOURCE.read_bytes()
    try:
        for number, (name, before, after) in enumerate(CASES, 1):
            text = ORIGINAL.decode()
            assert text.count(before) == 1, (name, 'nonunique actual anchor')
            modified = text.replace(before, after, 1)
            program = modified.split("<<'PY'\n", 1)[1].rsplit('\nPY', 1)[0]
            compile(program, 'binding_replay_mutant', 'exec')
            producer_plan()
            SOURCE.write_text(modified)
            producer_plan()
            result = subprocess.run(
                [sys.executable, '-I', '-B', 'docs/tasks/artifacts/formula_structure/binding_replay_contract.py'],
                cwd=ROOT, env=environment, capture_output=True)
            output = result.stdout + result.stderr
            producer_plan()
            (WORK / ('mutation-%d.log' % number)).write_bytes(output)
            assert result.returncode == 1 and b'AssertionError' in output and b'binding_replay_contract.py' in output and b'SyntaxError' not in output, (name, output.decode())
            print('binding replay mutation %d %s: rc=1, actual contract assertion red' % (number, name))
            PROFILE['plan_producer'](ROOT, sources=(SOURCE,), mutable_sources=(SOURCE,))
            SOURCE.write_bytes(ORIGINAL)
    finally:
        PROFILE['plan_producer'](ROOT, sources=(SOURCE,), mutable_sources=(SOURCE,))
        SOURCE.write_bytes(ORIGINAL)
    assert SOURCE.read_bytes() == ORIGINAL
    print('reference binding replay mutations: nine compiled actual reds; source restored byte-identically')
except (PROFILE['Refusal'], OSError) as error:
    print('local-environment: REFUSED — ' + str(error), file=sys.stderr)
    raise SystemExit(2) from None
