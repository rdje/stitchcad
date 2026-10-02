"""Independent authored headers/operands checked through the actual curated reference."""
from pathlib import Path
from fractions import Fraction
import runpy

ROOT = Path(__file__).resolve().parents[4]
context = runpy.run_path(str(ROOT / 'docs/tasks/artifacts/formula_structure/formula_input.py'))
namespace, reference = context['load_context']()
inventory = runpy.run_path(str(ROOT / 'docs/tasks/artifacts/formula_structure/canonical_contract_inventory.py'))
render = inventory['canonical_bytes']
reference.reserved = {name: ('length', True, Fraction(1))
                      for name in ['eps_num', 'eps_geo', 'eps_fmt', 'eps_imp', 'eps_phys']}

class HeaderReached(Exception):
    """Stop before dimensional inference/evaluation; this instrument judges only syntax."""

parsed = []
original_parse = reference.parse
def parse(source):
    tree = original_parse(source)
    parsed.append(tree)
    return tree

def stop(*args):
    raise HeaderReached()

reference.parse = parse
reference.infer = stop
reference.evaluate = stop
rows = [line.split('\t') for line in (ROOT / 'docs/tasks/artifacts/formula_structure/statement_cases.tsv').read_text().splitlines()
        if line and not line.startswith('#')]
assert len(rows) == 15
for source, role, name, annotation, left, right in rows:
    tokens, _ = reference.tokenize(source)
    assert [text for _, text in tokens[:5]] == [role, name, ':', annotation, '=']
    parsed.clear()
    try:
        reference.statement(source, {})
    except HeaderReached:
        pass
    else:
        raise AssertionError(('unexpected reference execution', source))
    assert list(map(render, parsed)) == ([left] if role == 'let' else [left, right]), source
refusals = [
    ('let if: count = 1', 'formula_parse'),
    ('let a: point = p', 'formula_dimension'),
    ('assert a: eps_chord = 1 cm == 1 cm', 'formula_tolerance_unbound'),
    ('assert a: eps_num = 1 cm == 1 cm == 1 cm', 'formula_parse'),
    ('assert a: eps_num = 1\tcm == 1 cm', 'formula_parse'),
    ('let a: count = 1 ^ 3', 'formula_unsupported'),
]
for source, expected in refusals:
    try:
        reference.statement(source, {})
    except namespace['FErr'] as error:
        assert error.token == expected, (source, error.token, expected)
    else:
        raise AssertionError(('reference accepted refusal', source))
print('statement reference:15 authored header/operand rows and6 refusals; inference/evaluation trapped')
