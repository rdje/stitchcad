"""D83: actual binding/storage/consumer faults must fail independent assertions."""
from pathlib import Path
import subprocess
import runpy
import sys
ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
WORK = ROOT / 'target/formula_binding_mutations'
ENTRY = ROOT / 'docs/tasks/artifacts/formula_structure/binding_contract.py'
PROVIDER_SOURCE = ROOT / 'docs/tasks/artifacts/formula_structure/binding_fixture.py'
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
    child = FIXTURE['binding_plan']()
    plan = dict(directories=(WORK, *child['directories']),
                output_files=(*child['output_files'], *(WORK / ('mutation-%d.log' % n) for n in range(1, 13))),
                sources=tuple(dict.fromkeys((*BOOT_INPUTS, *child['sources']))),
                mutable_sources=(SOURCE,))
    PROFILE['plan_producer'](ROOT, **plan)
    return plan

CASES = [
    ('binding guard bypassed', 'val = self.stored(val, "let %s binding" % name)', 'None'),
    ('binding truncates ties', 'integer = rnd(value.v)', 'integer = int(value.v)'),
    ('upper bound omitted', 'if integer < low or integer > high:', 'if integer < low:'),
    ('lower bound omitted', 'if integer < low or integer > high:', 'if integer > high:'),
    ('lower endpoint excluded', 'if integer < low or integer > high:', 'if integer <= low or integer > high:'),
    ('upper endpoint excluded', 'if integer < low or integer > high:', 'if integer < low or integer >= high:'),
    ('declared width duplicated', 'bits = int(match[1])', 'bits = 64'),
    ('typed storage token', 'raise FErr("formula_domain", "%s: %s storage=i%d', 'raise FErr("formula_division", "%s: %s storage=i%d'),
    ('operation context lost', '% (operation, value.kind, bits, low, high, integer)', '% ("binding", value.kind, bits, low, high, integer)'),
    ('boolean reclassified', 'if value.kind not in ARITH:\n            return value', 'if value.kind not in ARITH:\n            return Val("count", value.v)'),
    ('caller environment mutated', 'val = self.stored(val, "let %s binding" % name)', 'env[name] = {"kind": kind, "value": val.v, "origin": "recipe"}\n            val = self.stored(val, "let %s binding" % name)'),
    ('census changes returned integer', 'internal = val.v.numerator', 'internal = val.v.numerator + 1'),
]
try:
    plan = producer_plan()
    environment = PROFILE['prepare_producer'](ROOT, **plan)
    WORK.mkdir(exist_ok=True)
    ORIGINAL = SOURCE.read_bytes()
    try:
        for number, (name, before, after) in enumerate(CASES,1):
            text = ORIGINAL.decode()
            assert text.count(before) == 1, (name,'nonunique anchor')
            modified = text.replace(before,after,1)
            program = modified.split("<<'PY'\n",1)[1].rsplit('\nPY',1)[0]
            compile(program,'binding_mutant','exec')
            producer_plan()
            SOURCE.write_text(modified)
            producer_plan()
            result = subprocess.run([sys.executable,'-I','-B','docs/tasks/artifacts/formula_structure/binding_contract.py'],cwd=ROOT,env=environment,capture_output=True)
            output = result.stdout+result.stderr
            producer_plan()
            (WORK / ('mutation-%d.log' % number)).write_bytes(output)
            assert result.returncode == 1 and b'AssertionError' in output and b'binding_contract.py' in output and b'SyntaxError' not in output, (name,output.decode())
            print('binding mutation %d %s: rc=1, actual contract assertion red' % (number,name))
            PROFILE['plan_producer'](ROOT, sources=(SOURCE,), mutable_sources=(SOURCE,))
            SOURCE.write_bytes(ORIGINAL)
    finally:
        PROFILE['plan_producer'](ROOT, sources=(SOURCE,), mutable_sources=(SOURCE,))
        SOURCE.write_bytes(ORIGINAL)
    assert SOURCE.read_bytes()==ORIGINAL
    print('reference binding mutations: twelve compiled actual reds; source restored byte-identically')
except (PROFILE['Refusal'], OSError) as error:
    print('local-environment: REFUSED — ' + str(error), file=sys.stderr)
    raise SystemExit(2) from None
