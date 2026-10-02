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
