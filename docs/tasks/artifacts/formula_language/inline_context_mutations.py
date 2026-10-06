"""Guard classifier and nested copied-book mutations; restore exact source on all paths."""
from pathlib import Path
import runpy
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
WORK = ROOT / 'target/formula_inline_mutations'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
CONTRACT_SOURCE = ROOT / 'docs/tasks/artifacts/formula_language/inline_context_contract.py'
BOOT_INPUTS = (Path(__file__), PROFILE_SOURCE, SOURCE, CONTRACT_SOURCE)
try:
    PROFILE['plan_producer'](ROOT, sources=BOOT_INPUTS)
    CONTRACT = runpy.run_path(str(CONTRACT_SOURCE))
    PROFILE = CONTRACT['PROFILE']
except (PROFILE['Refusal'], OSError) as error:
    print('local-environment: REFUSED — ' + str(error), file=sys.stderr)
    raise SystemExit(2) from None

CASES = [
    ('foreign context removed', 'if span.start() not in excluded', 'if True'),
    ('whole-line exemption', '        out.extend(span.group(1)', '        if excluded: continue\n        out.extend(span.group(1)'),
    ('normative exemption', 'if not allow_foreign or not valid:', 'if not valid:'),
    ('annotation verdict omitted', 'l6 = fails - l6_start', 'l6 = l6'),
    ('unknown language accepted', 'marker.group() == "<!-- stitchcad-inline: rust -->" and next_span', 'next_span'),
]

def producer_plan():
    """Include the child's replacement tree before granting classifier write authority."""
    child = CONTRACT['fixture_plan'](activate=False)
    plan = dict(directories=(WORK, *child['directories']),
                sources=tuple(dict.fromkeys((*BOOT_INPUTS, *child['sources']))),
                output_files=(*child['output_files'], *(WORK / ('mutation-' + str(n) + '.log')
                             for n in range(1, len(CASES) + 1))), mutable_sources=(SOURCE,))
    PROFILE['plan_producer'](ROOT, **plan)
    return plan


def main():
    plan = producer_plan()
    environment = PROFILE['prepare_producer'](ROOT, **plan)
    WORK.mkdir(exist_ok=True)
    ORIGINAL = SOURCE.read_bytes()
    TEXT = ORIGINAL.decode()
    try:
        for number, (name, before, after) in enumerate(CASES, 1):
            assert TEXT.count(before) == 1, (name, 'nonunique anchor')
            producer_plan()
            SOURCE.write_text(TEXT.replace(before, after, 1))
            producer_plan()
            result = subprocess.run([sys.executable, '-I', '-B',
                'docs/tasks/artifacts/formula_language/inline_context_contract.py'], cwd=ROOT, env=environment, capture_output=True)
            output = result.stdout + result.stderr
            producer_plan()
            (WORK / ('mutation-%d.log' % number)).write_bytes(output)
            assert result.returncode == 1 and b'AssertionError' in output and b'SyntaxError' not in output, (name, output.decode())
            print('inline mutation %d %s: rc=1, actual contract assertion red' % (number, name))
            PROFILE['plan_producer'](ROOT, sources=(SOURCE,), mutable_sources=(SOURCE,))
            SOURCE.write_bytes(ORIGINAL)
    finally:
        PROFILE['plan_producer'](ROOT, sources=(SOURCE,), mutable_sources=(SOURCE,))
        SOURCE.write_bytes(ORIGINAL)
    assert SOURCE.read_bytes() == ORIGINAL
    print('inline context mutations: five actual reds; classifier restored byte-identically')


if __name__ == '__main__':
    try:
        main()
    except (PROFILE['Refusal'], OSError) as error:
        print('local-environment: REFUSED — ' + str(error), file=sys.stderr)
        raise SystemExit(2) from None
