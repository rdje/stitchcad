"""D109 authored exact statement/recipe bytes, using actual reference syntax only."""
from pathlib import Path
from fractions import Fraction
import runpy
import re

ROOT = Path(__file__).resolve().parents[4]
HERE = ROOT / 'docs/tasks/artifacts/formula_structure'
context = runpy.run_path(str(HERE / 'formula_input.py'))
_, reference = context['load_context']()
inventory = runpy.run_path(str(HERE / 'canonical_contract_inventory.py'))
render = inventory['canonical_bytes']
reference.reserved = {name: ('length', True, Fraction(1))
                      for name in ['eps_num', 'eps_geo', 'eps_fmt', 'eps_imp', 'eps_phys']}

class SyntaxReached(Exception):
    """Stop before semantic inference or evaluation."""

parsed = []
original_parse = reference.parse

def parse(source):
    tree = original_parse(source)
    parsed.append(tree)
    return tree

def stop(*args):
    raise SyntaxReached()

reference.parse = parse
reference.infer = stop
reference.evaluate = stop

def statement_bytes(source):
    tokens, _ = reference.tokenize(source)
    role, name, colon, annotation, assignment = [value for _, value in tokens[:5]]
    assert colon == ':' and assignment == '=' and role in ['let', 'assert']
    parsed.clear()
    try:
        reference.statement(source, {})
    except SyntaxReached:
        pass
    else:
        raise AssertionError(('reference unexpectedly executed statement', source))
    operands = list(map(render, parsed))
    assert len(operands) == (1 if role == 'let' else 2)
    opcode = 'bind' if role == 'let' else 'assert'
    return '(' + ' '.join([opcode, name, annotation, *operands]) + ')'

def recipe_bytes(statements):
    if not statements:
        return '(recipe)'
    return '(recipe ' + ' '.join(statements) + ')'

def rows(name):
    return [line.split('\t') for line in (HERE / name).read_text().splitlines()
            if line and not line.startswith('#')]

def decode(source):
    return '' if source == '-' else bytes(source, 'ascii').decode('unicode_escape')

statement_rows = rows('canonical_statement_cases.tsv')
assert len(statement_rows) == 16
kinds = set()
tolerances = set()
for source, expected in statement_rows:
    actual = statement_bytes(source)
    assert actual == expected, ('statement byte contract', source, actual, expected)
    assert actual.isascii() and actual == actual.strip() and '\n' not in actual
    role, _, _, annotation, _ = [value for _, value in reference.tokenize(source)[0][:5]]
    (kinds if role == 'let' else tolerances).add(annotation)
assert kinds == {'length', 'angle', 'area', 'ratio', 'count', 'boolean'}
assert tolerances == {'eps_num', 'eps_geo', 'eps_fmt', 'eps_imp', 'eps_phys'}

recipe_rows = rows('canonical_recipe_cases.tsv')
assert len(recipe_rows) == 9
statement_count = 0
for source, chunks, expected in recipe_rows:
    source = decode(source)
    chunks = [] if chunks == '-' else list(map(decode, chunks.split(' | ')))
    cursor = 0
    tokens = []
    statements = []
    for chunk in chunks:
        start = source.index(chunk, cursor)
        assert not source[cursor:start].strip()
        cursor = start + len(chunk)
        tokens.extend(reference.tokenize(chunk)[0])
        statements.append(statement_bytes(chunk))
    assert not source[cursor:].strip()
    assert reference.tokenize(source)[0] == tokens
    actual = recipe_bytes(statements)
    assert actual == expected, ('recipe byte contract', source, actual, expected)
    assert actual.isascii() and actual == actual.strip() and '\n' not in actual
    statement_count += len(statements)

# Exact templates are also required in the maintained specification and decision, not only fixtures.
grammar = (ROOT / 'docs/book/src/spec/formula-language/grammar.md').read_text()
section = grammar.split('### 4.1 Statement and whole-recipe bytes\n', 1)[1].split('\n## 5.', 1)[0]
decision = (ROOT / 'docs/decisions/decision_recipe-bytes.md').read_text()
for template in ['(bind NAME KIND EXPR)', '(assert NAME TOLERANCE LEFT RIGHT)',
                 '(recipe STATEMENT1 STATEMENT2 ...)', '(recipe)']:
    assert template in section and template in decision, template
assert 'no terminal newline' in section
assert '**unapproved**' in decision and 'Author / applier' in decision
# Independently authored population checks the actual four displayed book/decision examples.
published = [
    ('let width: length = 2.5 cm', '(bind width length length:25000)'),
    ('assert closure: eps_geo = -1 mm == -0.1 cm',
     '(assert closure eps_geo (- length:1000) (- length:1000))'),
    ('let width: length = 25 mm\nassert closure: eps_num = width == target',
     '(recipe (bind width length length:25000) (assert closure eps_num width target))'),
    ('empty or whitespace-only source', '(recipe)'),
]
for text in [section, decision]:
    blocks = re.findall(r'```text\n(.*?)\n```', text, re.S)
    assert len(blocks) == 1
    actual_examples = [(source.strip(), expected) for source, expected in
                       re.findall(r'(.*?)\n  => ([^\n]+)', blocks[0], re.S)]
    assert actual_examples == published, ('published byte population', actual_examples)
for source, expected in published:
    if source == 'empty or whitespace-only source':
        actual = recipe_bytes([])
    elif '\n' in source:
        actual = recipe_bytes([statement_bytes(chunk) for chunk in source.splitlines()])
    else:
        actual = statement_bytes(source)
    assert actual == expected, ('published byte contract', source, actual, expected)
print('published recipe byte contract:all4 actual book/decision examples agree with reference syntax')
# Cross-role text collision is intentional only inside known typed identity fields.
assert render(original_parse('bind(width, length, 25 mm)')) == statement_bytes(
    'let width: length = 25 mm')
assert render(original_parse('recipe(bind(n, count, 1))')) == recipe_bytes([
    statement_bytes('let n: count = 1')])
print('typed identity boundary:2 actual ordinary-call byte collisions require future typed framing')
assert statement_count == 12
print('recipe byte contract:16 authored statements /six kinds /five tolerances; '
      '9 whole sources /12 ordered statements; actual reference syntax only, semantics trapped')
