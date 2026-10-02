"""Authored whole-source boundaries, independently checked by actual reference token/statement APIs."""
from pathlib import Path
from fractions import Fraction
import runpy

ROOT = Path(__file__).resolve().parents[4]
namespace, reference = runpy.run_path(str(ROOT / 'docs/tasks/artifacts/formula_structure/formula_input.py'))['load_context']()
render = runpy.run_path(str(ROOT / 'docs/tasks/artifacts/formula_structure/canonical_contract_inventory.py'))['canonical_bytes']
reference.reserved = {name: ('length', True, Fraction(1))
                      for name in ['eps_num', 'eps_geo', 'eps_fmt', 'eps_imp', 'eps_phys']}

class SyntaxReached(Exception):
    """The actual header and recursive expression parser completed; stop before semantics."""

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
def verify_fixture(name, expected_rows):
    rows = [line.split('\t') for line in (ROOT / 'docs/tasks/artifacts/formula_structure' / name).read_text().splitlines()
            if line and not line.startswith('#')]
    assert len(rows) == expected_rows
    statement_count = expression_count = 0
    for source, chunks, identities in rows:
        source = '' if source == '-' else bytes(source, 'ascii').decode('unicode_escape')
        chunks = [] if chunks == '-' else [bytes(s, 'ascii').decode('unicode_escape') for s in chunks.split(' | ')]
        identities = [] if identities == '-' else identities.split(' | ')
        assert len(chunks) == len(identities)
        # Authored slices cover every non-whitespace byte, preserving actual complete token order.
        cursor = 0
        tokens = []
        for chunk, identity in zip(chunks, identities):
            start = source.index(chunk, cursor)
            assert not source[cursor:start].strip()
            cursor = start + len(chunk)
            chunk_tokens, starts = reference.tokenize(chunk)
            assert chunk_tokens[0][1] in ['let', 'assert'] and chunk_tokens[2][1] == ':' and chunk_tokens[4][1] == '='
            tokens.extend(chunk_tokens)
            parsed.clear()
            try:
                reference.statement(chunk, {})
            except SyntaxReached:
                pass
            else:
                raise AssertionError(('unexpected semantic execution', chunk))
            assert list(map(render, parsed)) == identity.split(' ~ '), chunk
            statement_count += 1
            expression_count += len(parsed)
        assert not source[cursor:].strip()
        assert reference.tokenize(source)[0] == tokens
    print('recipe reference %s:%d authored whole sources / %d original ordered statements / %d operand identities; semantics trapped' %
          (name, len(rows), statement_count, expression_count))
    return statement_count, expression_count

assert verify_fixture('recipe_cases.tsv', 9) == (14, 18)
assert verify_fixture('recipe_coupled_cases.tsv', 4) == (8, 10)
# Independently constructed shape: literal1 +16*(conditional/test/else) +207 unary nodes =256.
shape = '1 um'
for _ in range(16):
    shape = 'if(flag, ' + shape + ', 2 um)'
shape = '-' * 207 + shape
tree = original_parse(shape)
assert reference.count_nodes(tree) == 256 and reference.if_depth(tree) == 16
print('coupled reference shape:256 nodes /16 if levels, independently constructed before product max4096 test')

refusals = [
    ('let', 'formula_parse'), ('let let: count = 1', 'formula_parse'),
    ('assert if: eps_num = a == b', 'formula_parse'), ('let b', 'formula_parse'),
    ('let b count = 1', 'formula_parse'), ('let b:', 'formula_parse'),
    ('let b: 1 = 1', 'formula_parse'), ('let b: point = p', 'formula_dimension'),
    ('assert b: eps_chord = a == b', 'formula_tolerance_unbound'),
    ('let b: count', 'formula_parse'), ('let b: count == 1', 'formula_parse'),
    ('let Upper: count = 1', 'formula_parse'),
    ('let a: count = blet b: count = 2', 'formula_parse'),
    ('let a: length = 1 mmlet b: count = 2', 'formula_parse'),
    ('let a: count = 1let_value(2)', 'formula_parse'),
    ('let a: count = 1assertion(2)', 'formula_parse'),
]
for source, code in refusals:
    try:
        reference.statement(source, {})
    except namespace['FErr'] as error:
        assert error.token == code, (source, error.token, code)
    else:
        raise AssertionError(('reference accepted malformed statement', source))
print('coupled reference refusals:12 header /4 keyword-prefix cases /0 fail; syntax-only families')
