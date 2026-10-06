"""Actual D109 reference-renderer faults, exact authored-byte assertion reds and restoration."""
from pathlib import Path
import subprocess
import runpy
import sys

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'docs/tasks/artifacts/formula_structure/recipe_byte_contract.py'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
HERE = ROOT / 'docs/tasks/artifacts/formula_structure'
WORK = ROOT / 'target/recipe_byte_mutations'
SOURCES = (Path(__file__), PROFILE_SOURCE, SOURCE, HERE / 'formula_input.py',
           HERE / 'formula_structure.py', HERE / 'static_signature_contract.py',
           ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh',
           HERE / 'canonical_contract_inventory.py', HERE / 'canonical_statement_cases.tsv',
           HERE / 'canonical_recipe_cases.tsv', ROOT / 'docs/decisions/decision_recipe-bytes.md',
           ROOT / 'docs/decisions/decision_canonical-expression-spelling.md',
           ROOT / 'docs/book/src/annexes/formula-literals.md',
           *(ROOT / 'docs/book/src' / name for name in (
               'spec/formula-language.md', 'spec/formula-language/grammar.md',
               'spec/formula-language/examples.md', 'spec/reference-skirt.md', 'spec/units-and-tolerances.md')))
PLAN = dict(sources=SOURCES, directories=(WORK,),
            output_files=tuple(WORK / ('fault-%d.log' % number) for number in range(1, 10)),
            mutable_sources=(SOURCE,))


def source_plan():
    try:
        PROFILE['plan_producer'](ROOT, sources=SOURCES)
    except (PROFILE['Refusal'], OSError) as error:
        print('local-environment: REFUSED — ' + str(error), file=sys.stderr)
        raise SystemExit(2) from None


source_plan()
ORIGINAL = SOURCE.read_bytes()
CASES = [
    ('binding opcode', "opcode = 'bind' if role == 'let' else 'assert'",
     "opcode = 'let' if role == 'let' else 'assert'"),
    ('missing name', '[opcode, name, annotation, *operands]', '[opcode, annotation, *operands]'),
    ('missing annotation', '[opcode, name, annotation, *operands]', '[opcode, name, *operands]'),
    ('assertion operand order', 'operands = list(map(render, parsed))',
     'operands = list(map(render, parsed))[::-1]'),
    ('assertion nested equality', "opcode = 'bind' if role == 'let' else 'assert'",
     "operands = operands if role == 'let' else ['(== ' + ' '.join(operands) + ')']\n    opcode = 'bind' if role == 'let' else 'assert'"),
    ('recipe envelope', "return '(recipe ' + ' '.join(statements) + ')'",
     "return '(ordered ' + ' '.join(statements) + ')'"),
    ('recipe order', "return '(recipe ' + ' '.join(statements) + ')'",
     "return '(recipe ' + ' '.join(reversed(statements)) + ')'"),
    ('terminal newline', "return '(recipe ' + ' '.join(statements) + ')'",
     "return '(recipe ' + ' '.join(statements) + ')\\n'"),
    ('empty envelope lost', "return '(recipe)'", "return ''"),
]
for name, before, _ in CASES:
    assert ORIGINAL.decode().count(before) == 1, (name, 'actual anchor not unique')

def assertion_failure(output):
    return (b'assert actual == expected' in output and
            (b"AssertionError: ('statement byte contract'," in output or
             b"AssertionError: ('recipe byte contract'," in output))

assert assertion_failure(b"assert actual == expected\nAssertionError: ('recipe byte contract', 'a', 'b', 'c')")
assert not assertion_failure(b"recipe byte contract passed\nRuntimeError: fixture expected")
assert not assertion_failure(b"assert actual == expected\nSyntaxError: invalid syntax")
assert not assertion_failure(b"assert actual == expected\nAssertionError: ('unrelated fixture', 'a')")
assert sys.argv[1:] in [[], ['--classifier-only']]
if sys.argv[1:] == ['--classifier-only']:
    print('recipe byte fault controls:9 unique actual anchors; passing/exception/syntax/unrelated noise refused')
    sys.exit(0)
try:
    environment = PROFILE['enter_producer'](ROOT, **PLAN)
    WORK.mkdir(parents=True, exist_ok=True)
    try:
        for index, (name, before, after) in enumerate(CASES, 1):
            PROFILE['plan_producer'](ROOT, **PLAN)
            SOURCE.write_text(ORIGINAL.decode().replace(before, after, 1))
            PROFILE['plan_producer'](ROOT, **PLAN)
            result = subprocess.run([sys.executable, '-I', '-B', str(SOURCE)], cwd=ROOT,
                                    env=environment, capture_output=True)
            output = result.stdout + result.stderr
            PROFILE['plan_producer'](ROOT, **PLAN)
            (WORK / ('fault-%d.log' % index)).write_bytes(output)
            assert result.returncode == 1 and assertion_failure(output), (name, output.decode())
            print('recipe byte fault %d %s:rc=1 actual interpreter authored-byte assertion red' % (index, name))
            PROFILE['plan_producer'](ROOT, sources=(SOURCE,), mutable_sources=(SOURCE,))
            SOURCE.write_bytes(ORIGINAL)
    finally:
        PROFILE['plan_producer'](ROOT, sources=(SOURCE,), mutable_sources=(SOURCE,))
        SOURCE.write_bytes(ORIGINAL)
    source_plan()
    assert SOURCE.read_bytes() == ORIGINAL
    print('recipe byte faults:9 authored-byte assertion reds; exact source restored, not product proof')
except (PROFILE['Refusal'], OSError) as error:
    print('local-environment: REFUSED — ' + str(error), file=sys.stderr)
    raise SystemExit(2) from None
