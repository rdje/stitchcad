"""Actual inventory-renderer faults must fail independently authored byte assertions."""
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'docs/tasks/artifacts/formula_structure/canonical_contract_inventory.py'
ORIGINAL = SOURCE.read_bytes()
WORK = ROOT / 'target/canonical_contract_inventory_mutations'
WORK.mkdir(parents=True, exist_ok=True)
CASES = [
    ('named minus', "tag = '-' if role == 'neg' else '^2'",
     "tag = 'neg' if role == 'neg' else '^2'"),
    ('named square', "tag = '-' if role == 'neg' else '^2'",
     "tag = '-' if role == 'neg' else 'square'"),
    ('branch order', "map(canonical_bytes, node[1:])",
     "map(canonical_bytes, reversed(node[1:]))"),
    ('call order', "map(canonical_bytes, node[2])",
     "map(canonical_bytes, reversed(node[2]))"),
]
try:
    for index, (name, before, after) in enumerate(CASES, 1):
        source = ORIGINAL.decode()
        assert source.count(before) == 1, (name, 'actual anchor not unique')
        SOURCE.write_text(source.replace(before, after, 1))
        result = subprocess.run([sys.executable, '-I', '-B', str(SOURCE)], cwd=ROOT,
                                capture_output=True)
        output = result.stdout + result.stderr
        (WORK / ('fault-%d.log' % index)).write_bytes(output)
        assert result.returncode == 1 and b'AssertionError' in output, (name, output.decode())
        assert b'assert actual == expected' in output, (name, 'not an authored-byte assertion')
        print('inventory fault %d %s: rc=1, actual interpreter authored-byte assertion red'
              % (index, name))
        SOURCE.write_bytes(ORIGINAL)
finally:
    SOURCE.write_bytes(ORIGINAL)
assert SOURCE.read_bytes() == ORIGINAL
print('inventory faults: four actual assertion reds; exact source restored (not product proof)')
