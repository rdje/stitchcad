"""Read-only curated graph oracle using Python's independent standard-library topological sorter.

No Rust implementation or native outcome is read. The native contract invokes this verifier and
reads these same tracked authored graphs; original cycle edges/spans are checked natively.
"""
from graphlib import CycleError, TopologicalSorter
from pathlib import Path
import runpy

ROOT = Path(__file__).resolve().parents[4]
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
FIXTURE = ROOT / 'docs/tasks/artifacts/formula_structure/source_cycle_cases.tsv'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
PROFILE['plan_producer'](ROOT, sources=(Path(__file__), PROFILE_SOURCE, FIXTURE))
rows = [line.split('\t') for line in FIXTURE.read_text().splitlines()
        if line and not line.startswith('#')]
assert len(rows) == 9
for name, vertices, encoded, expected in rows:
    size = int(vertices)
    graph = {index: [] for index in range(size)}
    edges = [] if encoded == '-' else [tuple(map(int, pair.split('>'))) for pair in encoded.split(',')]
    for consumer, supplier in edges:
        assert 0 <= consumer < size and 0 <= supplier < size, name
        graph[consumer].append(supplier)
    try:
        order = tuple(TopologicalSorter(graph).static_order())
    except CycleError:
        actual = False
    else:
        assert len(order) == size and set(order) == set(range(size)), name
        positions = {vertex: index for index, vertex in enumerate(order)}
        assert all(positions[supplier] < positions[consumer] for consumer, supplier in edges), name
        actual = True
    assert expected in {'true', 'false'} and actual == (expected == 'true'), name
print('source cycle reference: 9 curated graphlib cases pass')
