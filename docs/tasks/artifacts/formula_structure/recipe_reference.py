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
rows = [line.split('\t') for line in (ROOT / 'docs/tasks/artifacts/formula_structure/recipe_cases.tsv').read_text().splitlines()
        if line and not line.startswith('#')]
assert len(rows) == 9
statement_count = expression_count = 0
for source, chunks, identities in rows:
    source = '' if source == '-' else bytes(source, 'ascii').decode('unicode_escape')
    chunks = [] if chunks == '-' else [bytes(s, 'ascii').decode('unicode_escape') for s in chunks.split(' | ')]
    identities = [] if identities == '-' else identities.split(' | ')
    assert len(chunks) == len(identities)
    # Independently authored slices must cover every non-whitespace byte, preserving actual order.
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
print('recipe reference:9 authored whole sources / %d original ordered statements / %d operand identities; semantics trapped' %
      (statement_count, expression_count))
