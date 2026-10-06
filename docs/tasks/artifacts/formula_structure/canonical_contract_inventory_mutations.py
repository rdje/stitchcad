"""Guard actual reference mutations; keep authored assertions and exact source restoration."""
from pathlib import Path
import runpy
import subprocess
import sys
ROOT = Path(__file__).resolve().parents[4]
HERE = ROOT / 'docs/tasks/artifacts/formula_structure'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
TOOL = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
BOOK_INPUTS = tuple(ROOT / 'docs/book/src' / name for name in (
    'spec/formula-language.md', 'spec/formula-language/grammar.md',
    'spec/formula-language/examples.md', 'spec/reference-skirt.md', 'spec/units-and-tolerances.md'))
SOURCE = HERE / 'canonical_contract_inventory.py'
WORK = ROOT / 'target/canonical_contract_inventory_mutations'
INPUTS = (Path(__file__), PROFILE_SOURCE, SOURCE, HERE / 'formula_input.py',
          HERE / 'formula_structure.py', HERE / 'static_signature_contract.py', TOOL, *BOOK_INPUTS,
          ROOT / 'docs/decisions/decision_canonical-expression-spelling.md',
          ROOT / 'docs/book/src/annexes/formula-literals.md')
CASES = [
    ('named minus', "tag = '-' if role == 'neg' else '^2'",
     "tag = 'neg' if role == 'neg' else '^2'"),
    ('named square', "tag = '-' if role == 'neg' else '^2'",
     "tag = '-' if role == 'neg' else 'square'"),
    ('branch order', "map(canonical_bytes, node[1:])",
     "map(canonical_bytes, reversed(node[1:]))"),
    ('call order', "map(canonical_bytes, node[2])",
     "map(canonical_bytes, reversed(node[2]))"),
]

def main():
    plan = dict(directories=(WORK,), sources=INPUTS, mutable_sources=(SOURCE,),
                output_files=tuple(WORK / ('fault-' + str(n) + '.log') for n in range(1, len(CASES) + 1)))
    PROFILE['plan_producer'](ROOT, **plan)
    environment = PROFILE['prepare_producer'](ROOT, **plan)
    ORIGINAL = SOURCE.read_bytes()
    WORK.mkdir(parents=True, exist_ok=True)
    try:
        for index, (name, before, after) in enumerate(CASES, 1):
            source = ORIGINAL.decode()
            assert source.count(before) == 1, (name, 'actual anchor not unique')
            PROFILE['plan_producer'](ROOT, **plan)
            SOURCE.write_text(source.replace(before, after, 1))
            PROFILE['plan_producer'](ROOT, **plan)
            result = subprocess.run([sys.executable, '-I', '-B', str(SOURCE)], cwd=ROOT, env=environment,
                                    capture_output=True)
            output = result.stdout + result.stderr
            PROFILE['plan_producer'](ROOT, **plan)
            (WORK / ('fault-%d.log' % index)).write_bytes(output)
            assert result.returncode == 1 and b'AssertionError' in output, (name, output.decode())
            assert b'assert actual == expected' in output, (name, 'not an authored-byte assertion')
            print('inventory fault %d %s: rc=1, actual interpreter authored-byte assertion red'
                  % (index, name))
            PROFILE['plan_producer'](ROOT, sources=(SOURCE,), mutable_sources=(SOURCE,))
            SOURCE.write_bytes(ORIGINAL)
    finally:
        PROFILE['plan_producer'](ROOT, sources=(SOURCE,), mutable_sources=(SOURCE,))
        SOURCE.write_bytes(ORIGINAL)
    assert SOURCE.read_bytes() == ORIGINAL
    print('inventory faults: four actual assertion reds; exact source restored (not product proof)')


if __name__ == '__main__':
    try:
        main()
    except (PROFILE['Refusal'], OSError) as error:
        print('local-environment: REFUSED — ' + str(error), file=sys.stderr)
        raise SystemExit(2) from None
