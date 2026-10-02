"""Verify authored canonical bytes using the independent recursive book-reference renderer."""
from pathlib import Path
import runpy

ROOT = Path(__file__).resolve().parents[4]
inventory = runpy.run_path(str(
    ROOT / 'docs/tasks/artifacts/formula_structure/canonical_contract_inventory.py'
))
reference = inventory['reference']
render = inventory['canonical_bytes']
fixture = ROOT / 'docs/tasks/artifacts/formula_structure/canonical_expression_cases.tsv'
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
worked_rows = [line.split('\t') for line in worked_fixture.read_text().splitlines()
               if line and not line.startswith('#')]
assert len(worked_rows) == 25
assert [source for source, _ in worked_rows] == worked_sources
for source, expected in worked_rows:
    assert render(reference.parse(source)) == expected, (source, expected)
print('canonical worked reference: all25 actual book expressions match authored identity bytes')
