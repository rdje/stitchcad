"""Verify authored canonical bytes using the independent recursive book-reference renderer."""
from pathlib import Path
import runpy
import sys

ROOT = Path(__file__).resolve().parents[4]
HERE = ROOT / 'docs/tasks/artifacts/formula_structure'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
BOOT_INPUTS = (Path(__file__),
               PROFILE_SOURCE,
               HERE / 'formula_structure.py',
               HERE / 'static_signature_contract.py',
               ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh',
               HERE / 'formula_input.py',
               HERE / 'canonical_contract_inventory.py')
SOURCE_INPUTS = (*BOOT_INPUTS, *(ROOT / 'docs/book/src' / chapter for chapter in (
    'spec/formula-language.md', 'spec/formula-language/grammar.md',
    'spec/formula-language/examples.md', 'spec/reference-skirt.md', 'spec/units-and-tolerances.md')),
    HERE / 'canonical_expression_cases.tsv',
    HERE / 'canonical_worked_cases.tsv',
    ROOT / 'docs/decisions/decision_canonical-expression-spelling.md',
    ROOT / 'docs/book/src/annexes/formula-literals.md')

def source_plan():
    """Validate the complete sources without activating stores or creating outputs."""
    try:
        PROFILE['plan_producer'](ROOT, sources=SOURCE_INPUTS)
    except (PROFILE['Refusal'], OSError) as error:
        print('local-environment: REFUSED — ' + str(error), file=sys.stderr)
        raise SystemExit(2) from None

try:
    source_plan()
    CONTEXT = runpy.run_path(str(HERE / 'canonical_contract_inventory.py'))
    PROFILE = CONTEXT['PROFILE']
    source_plan()
except (PROFILE['Refusal'], OSError) as error:
    print('local-environment: REFUSED — ' + str(error), file=sys.stderr)
    raise SystemExit(2) from None
inventory = CONTEXT
reference = inventory['reference']
render = inventory['canonical_bytes']
fixture = ROOT / 'docs/tasks/artifacts/formula_structure/canonical_expression_cases.tsv'
source_plan()
rows = [line.split('\t') for line in fixture.read_text().splitlines()
        if line and not line.startswith('#')]
assert len(rows) == 55
for source, expected in rows:
    actual = render(reference.parse(source))
    assert actual == expected, (source, actual, expected)
    assert actual.isascii() and actual == actual.strip() and '\n' not in actual
print('canonical expression reference: 55 authored byte fixtures agree with actual reference ASTs')

# Enumerate the actual book population independently of the product test's reader.
# Authored expected bytes below are data, never captured from a product arena.
source_plan()
chapter = (ROOT / 'docs/book/src/spec/formula-language/examples.md').read_text()
bindings = chapter.split('## 2. Bindings\n', 1)[1].split('\n## 3.', 1)[0]
assertions = chapter.split('## 3. Assertions', 1)[1].split('\n## 4.', 1)[0]
worked_sources = []
for line in bindings.splitlines():
    if line.startswith('| `'):
        worked_sources.append(line.split('|')[3].strip().strip('`'))
for line in assertions.splitlines():
    if line.startswith('| `assert '):
        statement = line.split('|')[1].strip().strip('`')
        worked_sources.extend(statement.split(' = ', 1)[1].split(' == '))
worked_fixture = ROOT / 'docs/tasks/artifacts/formula_structure/canonical_worked_cases.tsv'
source_plan()
worked_rows = [line.split('\t') for line in worked_fixture.read_text().splitlines()
               if line and not line.startswith('#')]
assert len(worked_rows) == 25
assert [source for source, _ in worked_rows] == worked_sources
for source, expected in worked_rows:
    assert render(reference.parse(source)) == expected, (source, expected)
print('canonical worked reference: all25 actual book expressions match authored identity bytes')
