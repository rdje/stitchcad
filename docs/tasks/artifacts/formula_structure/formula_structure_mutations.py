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
SOURCE = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
WORK = ROOT / 'target/formula_structure_mutations'
INPUTS = (Path(__file__), PROFILE_SOURCE, SOURCE, HERE / 'formula_structure.py',
          HERE / 'static_signature_contract.py', *BOOK_INPUTS)
CASES = [
    ('call argument children', 'if node[0] == "call":', 'if False and node[0] == "call":'),
    ('node descendant traversal', 'pending.extend(self.syntax_children(current))', 'pending.extend([])'),
    ('depth descendant traversal', 'pending.extend((child, depth) for child in self.syntax_children(current))', 'pending.extend([])'),
    ('early depth refusal', 'if depth > self.limits["max_if_depth"]:', 'if False and depth > self.limits["max_if_depth"]:'),
]

def main():
    plan = dict(directories=(WORK,), sources=INPUTS, mutable_sources=(SOURCE,),
                output_files=tuple(WORK / ('mutation-' + str(n) + '.log') for n in range(1, len(CASES) + 1)))
    PROFILE['plan_producer'](ROOT, **plan)
    environment = PROFILE['prepare_producer'](ROOT, **plan)
    WORK.mkdir(exist_ok=True)
    ORIGINAL = SOURCE.read_bytes()
    TEXT = ORIGINAL.decode()
    try:
        for number, (name, before, after) in enumerate(CASES, 1):
            assert TEXT.count(before) == 1, (name, 'nonunique mutation anchor')
            PROFILE['plan_producer'](ROOT, **plan)
            SOURCE.write_text(TEXT.replace(before, after, 1))
            PROFILE['plan_producer'](ROOT, **plan)
            result = subprocess.run([sys.executable, '-I', '-B',
                                     'docs/tasks/artifacts/formula_structure/formula_structure.py',
                                     '--contracts-only'], cwd=ROOT, env=environment, capture_output=True)
            output = result.stdout + result.stderr
            PROFILE['plan_producer'](ROOT, **plan)
            (WORK / ('mutation-%d.log' % number)).write_bytes(output)
            assert result.returncode == 1 and b'AssertionError' in output and b'formula_structure.py' in output and b'SyntaxError' not in output, (name, output.decode())
            print('mutation %d %s: rc=1, structural contract assertion failed' % (number, name))
            PROFILE['plan_producer'](ROOT, sources=(SOURCE,), mutable_sources=(SOURCE,))
            SOURCE.write_bytes(ORIGINAL)
    finally:
        PROFILE['plan_producer'](ROOT, sources=(SOURCE,), mutable_sources=(SOURCE,))
        SOURCE.write_bytes(ORIGINAL)
    assert SOURCE.read_bytes() == ORIGINAL
    print('formula structural mutations: four actual reds; reference source restored byte-identically')


if __name__ == '__main__':
    try:
        main()
    except (PROFILE['Refusal'], OSError) as error:
        print('local-environment: REFUSED — ' + str(error), file=sys.stderr)
        raise SystemExit(2) from None
