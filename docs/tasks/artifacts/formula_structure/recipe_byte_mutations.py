"""Actual D109 reference-renderer faults, exact authored-byte assertion reds and restoration."""
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'docs/tasks/artifacts/formula_structure/recipe_byte_contract.py'
ORIGINAL = SOURCE.read_bytes()
WORK = ROOT / 'target/recipe_byte_mutations'
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
WORK.mkdir(parents=True, exist_ok=True)
try:
    for index, (name, before, after) in enumerate(CASES, 1):
        SOURCE.write_text(ORIGINAL.decode().replace(before, after, 1))
        result = subprocess.run([sys.executable, '-I', '-B', str(SOURCE)], cwd=ROOT,
                                capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('fault-%d.log' % index)).write_bytes(output)
        assert result.returncode == 1 and assertion_failure(output), (name, output.decode())
        print('recipe byte fault %d %s:rc=1 actual interpreter authored-byte assertion red' % (index, name))
        SOURCE.write_bytes(ORIGINAL)
finally:
    SOURCE.write_bytes(ORIGINAL)
assert SOURCE.read_bytes() == ORIGINAL
print('recipe byte faults:9 authored-byte assertion reds; exact source restored, not product proof')
