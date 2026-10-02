"""Independent reference parses the same explicit shape/count fixtures as product contracts."""
from pathlib import Path
import runpy
ROOT = Path(__file__).resolve().parents[4]
namespace, reference = runpy.run_path(str(ROOT / 'docs/tasks/artifacts/formula_structure/formula_structure.py'))['load_reference']()
OPERATORS = {'+': 'Add', '-': 'Subtract', '*': 'Multiply', '/': 'Divide',
             '==': 'Equal', '!=': 'NotEqual', '<': 'Less', '<=': 'LessEqual',
             '>': 'Greater', '>=': 'GreaterEqual'}
def shape(node):
    # Fixture conversion only; reference owns all parsing/precedence. Fixtures are deliberately small.
    if node[0] == 'name':
        return node[1]
    if node[0] in {'bin', 'cmp'}:
        return '(' + OPERATORS[node[1]] + ' ' + shape(node[2]) + ' ' + shape(node[3]) + ')'
    if node[0] in {'neg', 'sq'}:
        return '(' + ('neg' if node[0] == 'neg' else 'square') + ' ' + shape(node[1]) + ')'
    if node[0] == 'call':
        return '(' + node[1] + ' ' + ' '.join(map(shape, node[2])) + ')'
    assert node[0] == 'if', ('unexpected fixture shape', node)
    return '(if ' + ' '.join(map(shape, node[1:])) + ')'
checks = 0
for row in (ROOT / 'docs/tasks/artifacts/formula_structure/expression_shapes.tsv').read_text().splitlines():
    if row.startswith('#') or not row:
        continue
    source, expected, nodes, depth = row.split('\t')
    tree = reference.parse(source)
    assert shape(tree) == expected, (source, shape(tree), expected)
    assert reference.count_nodes(tree) == int(nodes), (source, 'nodes')
    assert reference.if_depth(tree) == int(depth), (source, 'depth')
    checks += 1
assert checks == 12
print('product/reference expression fixtures: 12 independent shape/count/depth controls pass')
