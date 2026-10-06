"""Actual worked/refusal populations; authored bytes, reference syntax only, no execution."""
from pathlib import Path
import runpy
import sys

ROOT = Path(__file__).resolve().parents[4]
HERE = ROOT / 'docs/tasks/artifacts/formula_structure'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
BOOT_INPUTS = (Path(__file__), PROFILE_SOURCE, HERE / 'recipe_byte_contract.py',
               HERE / 'formula_input.py', HERE / 'formula_structure.py',
               HERE / 'static_signature_contract.py', HERE / 'canonical_contract_inventory.py',
               ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh')
REFERENCE_INPUTS = (*BOOT_INPUTS, *(ROOT / 'docs/book/src' / name for name in (
    'spec/formula-language.md', 'spec/formula-language/grammar.md',
    'spec/formula-language/examples.md', 'spec/reference-skirt.md', 'spec/units-and-tolerances.md')),
    ROOT / 'docs/decisions/decision_canonical-expression-spelling.md',
    ROOT / 'docs/decisions/decision_recipe-bytes.md', ROOT / 'docs/book/src/annexes/formula-literals.md',
    *(HERE / name for name in ('canonical_statement_cases.tsv', 'canonical_recipe_cases.tsv',
                              'canonical_worked_recipe_cases.tsv', 'recipe_refusal_stage_cases.tsv')))


def source_plan(extra=()):
    """Validate all delegated inputs and the selected borrowed fixture without producing effects."""
    try:
        PROFILE['plan_producer'](ROOT, sources=(*REFERENCE_INPUTS, *extra))
    except (PROFILE['Refusal'], OSError) as error:
        print('local-environment: REFUSED — ' + str(error), file=sys.stderr)
        raise SystemExit(2) from None


try:
    source_plan()
    context = runpy.run_path(str(HERE / 'recipe_byte_contract.py'))
    PROFILE = context['PROFILE']
    source_plan()
    statement_bytes = context['statement_bytes']
    recipe_bytes = context['recipe_bytes']
    reference = context['reference']
    book = (ROOT / 'docs/book/src/spec/formula-language/examples.md').read_text()
except (PROFILE['Refusal'], OSError) as error:
    print('local-environment: REFUSED — ' + str(error), file=sys.stderr)
    raise SystemExit(2) from None


def rows(name):
    fixture_path = HERE / name
    source_plan((fixture_path,))
    return [line.split('\t') for line in fixture_path.read_text().splitlines()
            if line and not line.startswith('#')]

section = ''
worked = []
refusals = []
for line in book.splitlines():
    if line.startswith('## '):
        section = line
    if not line.startswith('| `'):
        continue
    cells = [cell.strip() for cell in line.split('|')]
    if section.startswith('## 2.'):
        name, kind, expression = [cell.strip('`') for cell in cells[1:4]]
        worked.append('let %s: %s = %s' % (name, kind, expression))
    elif section.startswith('## 3.'):
        worked.append(cells[1].strip('`'))
    elif section.startswith('## 4.'):
        refusals.append((cells[1].split('`')[1], cells[2].strip('`')))

authored = rows('canonical_worked_recipe_cases.tsv')
assert len(authored) == len(worked) == 21
assert worked == [source for source, _ in authored], 'actual complete worked population'
assert sum(source.startswith('let ') for source in worked) == 17
assert sum(source.startswith('assert ') for source in worked) == 4
tokens = []
operands = 0
for source, expected in authored:
    assert statement_bytes(source) == expected, ('worked identity', source)
    operands += len(context['parsed'])
    tokens.extend(reference.tokenize(source)[0])
assert reference.tokenize('\n'.join(worked))[0] == tokens
assert operands == 25
assert recipe_bytes([statement_bytes(source) for source in worked]) == recipe_bytes(
    [expected for _, expected in authored])

stages = rows('recipe_refusal_stage_cases.tsv')
assert len(stages) == len(refusals) == 13
assert refusals == [(source, diagnostic) for source, _, diagnostic, _ in stages]
counts = {'syntax': 0, 'semantic': 0}
for source, stage, diagnostic, expected in stages:
    assert stage in counts
    statement = source if source.startswith('let ') else 'let candidate: length = ' + source
    if stage == 'syntax':
        try:
            statement_bytes(statement)
        except Exception as error:
            assert error.__class__.__name__ == 'FErr' and error.token == diagnostic, (
                source, 'actual reference syntax family', error)
        else:
            raise AssertionError((source, 'syntax refusal unexpectedly accepted'))
        assert expected == '-'
    else:
        assert statement_bytes(statement) == expected, ('semantic checks are deferred', source)
    counts[stage] += 1
assert counts == {'syntax': 3, 'semantic': 10}
print('whole input review:actual17 bindings/four assertions/21 statements/25 operands; '
      '13 refusal examples split3 syntax/10 deferred semantic; authored bytes/reference syntax agree')
print('whole input review:ordered token coverage; semantics trapped; no independent whole-recipe parser')
