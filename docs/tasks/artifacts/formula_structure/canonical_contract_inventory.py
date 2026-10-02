"""Check the pre-serializer contract inventory against the actual book reference."""
from fractions import Fraction
from pathlib import Path
import re
import runpy

ROOT = Path(__file__).resolve().parents[4]
_, reference = runpy.run_path(str(
    ROOT / 'docs/tasks/artifacts/formula_structure/formula_input.py'
))['load_context']()
grammar = (ROOT / 'docs/book/src/spec/formula-language/grammar.md').read_text()
canonical = grammar.split('## 4. The canonical form\n', 1)[1].split('\n## 5.', 1)[0]
assert 'One space separates parts' in canonical
assert '(bind garment_waist length (+ waist_girth ease_waist))' in canonical
assert 'Unary minus remains an operator' in canonical

literal = ('lit', 'length', Fraction(1))
cases = [
    ('1 um', literal),
    ('a', ('name', 'a')),
    ('-1 um', ('neg', literal)),
    ('1 um ^ 2', ('sq', literal)),
    ('1 um + 1 um', ('bin', '+', literal, literal)),
    ('probe(1 um)', ('call', 'probe', [literal])),
    ('if(a, 1 um, 1 um)', ('if', ('name', 'a'), literal, literal)),
]
for source, expected in cases:
    assert reference.parse(source) == expected, (source, expected)

# A name tag cannot distinguish these operators from existing ordinary-call syntax.
# The independent expected tuples prove that they are different grammatical roles.
for source, name, operator in [('neg(1 um)', 'neg', 'neg'),
                               ('square(1 um)', 'square', 'sq')]:
    call = reference.parse(source)
    assert call == ('call', name, [literal])
    assert call != (operator, literal)
    assert name not in {'let', 'assert', 'if'}
    assert re.fullmatch(r'[a-z]+', name)
for symbolic in ['-', '^', '^2']:
    assert re.fullmatch(r'[a-z][a-z0-9]*(?:_[a-z0-9]+)*', symbolic) is None

binary = {'+', '-', '*', '/', '==', '!=', '<', '<=', '>', '>='}
for symbol in binary:
    node = reference.parse('a ' + symbol + ' b')
    assert node[0] in {'bin', 'cmp'} and node[1] == symbol
    assert node[2:] == (('name', 'a'), ('name', 'b'))

def canonical_bytes(node):
    """Independent recursive rendering of the book reference, never a product arena."""
    role = node[0]
    if role == 'name':
        return node[1]
    if role == 'lit':
        assert node[2].denominator == 1
        return '%s:%d' % (node[1], node[2].numerator)
    if role in {'neg', 'sq'}:
        tag = '-' if role == 'neg' else '^2'
        return '(%s %s)' % (tag, canonical_bytes(node[1]))
    if role in {'bin', 'cmp'}:
        return '(%s %s %s)' % (node[1], canonical_bytes(node[2]), canonical_bytes(node[3]))
    if role == 'call':
        return '(' + node[1] + ' ' + ' '.join(map(canonical_bytes, node[2])) + ')'
    assert role == 'if', node
    return '(if ' + ' '.join(map(canonical_bytes, node[1:])) + ')'


decision = (ROOT / 'docs/decisions/decision_canonical-expression-spelling.md').read_text()
assert '(- child)' in decision and '(^2 child)' in decision
assert '(^ child count:2)' in decision
assert 'operator payload' in decision
assert 'Statement bind/assert serialization' in decision
annex = (ROOT / 'docs/book/src/annexes/formula-literals.md').read_text()
examples = re.findall(r'^(.*?)\s+=>\s+(.*)$', annex, re.M)
assert len(examples) == 6
assert examples == re.findall(r'^(.*?)\s+=>\s+(.*)$', decision, re.M)
for source, expected in examples:
    actual = canonical_bytes(reference.parse(source))
    assert actual == expected, (source, actual, expected)
print('canonical contract inventory: seven roles / ten binary symbols / two distinct named calls; '
      'six authored byte examples / symbolic opcode-name separation / received contract checked')
