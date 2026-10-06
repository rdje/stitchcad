"""Independent authored headers/operands checked through the actual curated reference."""
from pathlib import Path
from fractions import Fraction
import runpy
import sys

ROOT = Path(__file__).resolve().parents[4]
HERE = ROOT / 'docs/tasks/artifacts/formula_structure'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
BOOT_INPUTS = (Path(__file__), PROFILE_SOURCE, HERE / 'formula_input.py',
               HERE / 'formula_structure.py', HERE / 'static_signature_contract.py',
               ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh',
               HERE / 'canonical_contract_inventory.py')
REFERENCE_INPUTS = (*BOOT_INPUTS, *(ROOT / 'docs/book/src' / name for name in (
    'spec/formula-language.md', 'spec/formula-language/grammar.md',
    'spec/formula-language/examples.md', 'spec/reference-skirt.md', 'spec/units-and-tolerances.md')),
    ROOT / 'docs/decisions/decision_canonical-expression-spelling.md',
    ROOT / 'docs/book/src/annexes/formula-literals.md', HERE / 'statement_cases.tsv')


def source_plan():
    """Validate the complete read-only population without activating stores or creating outputs."""
    try:
        PROFILE['plan_producer'](ROOT, sources=REFERENCE_INPUTS)
    except (PROFILE['Refusal'], OSError) as error:
        print('local-environment: REFUSED — ' + str(error), file=sys.stderr)
        raise SystemExit(2) from None


try:
    PROFILE['plan_producer'](ROOT, sources=BOOT_INPUTS)
    context = runpy.run_path(str(HERE / 'formula_input.py'))
    PROFILE = context['PROFILE']
    source_plan()
    namespace, reference = context['load_context']()
    source_plan()
    inventory = runpy.run_path(str(HERE / 'canonical_contract_inventory.py'))
    PROFILE = inventory['PROFILE']
    source_plan()
except (PROFILE['Refusal'], OSError) as error:
    print('local-environment: REFUSED — ' + str(error), file=sys.stderr)
    raise SystemExit(2) from None
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
source_plan()
rows = [line.split('\t') for line in (ROOT / 'docs/tasks/artifacts/formula_structure/statement_cases.tsv').read_text().splitlines()
        if line and not line.startswith('#')]
assert len(rows) == 15
for source, role, name, annotation, left, right in rows:
    tokens, _ = reference.tokenize(source)
    assert [text for _, text in tokens[:5]] == [role, name, ':', annotation, '=']
    parsed.clear()
    checked = reference.syntax_statement(source)
    assert checked[:3] == (role, name, annotation), ('actual syntax header', source, checked)
    assert list(map(render, parsed)) == ([left] if role == 'let' else [left, right]), source
refusals = [
    ('let if: count = 1', 'formula_parse'),
    ('let a: point = p', 'formula_dimension'),
    ('assert a: eps_chord = 1 cm == 1 cm', 'formula_parse'),
    ('assert a: eps_num = 1 cm == 1 cm == 1 cm', 'formula_parse'),
    ('assert a: eps_num = 1\tcm == 1 cm', 'formula_parse'),
    ('let a: count = 1 ^ 3', 'formula_unsupported'),
]
for source, expected in refusals:
    try:
        reference.syntax_statement(source)
    except namespace['FErr'] as error:
        assert error.token == expected, (source, error.token, expected)
    else:
        raise AssertionError(('reference accepted refusal', source))
print('statement reference:15 authored header/operand rows and6 refusals; inference/evaluation trapped')
