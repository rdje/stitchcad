"""Independent reference parses the same explicit shape/count fixtures as product contracts."""
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
               ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh')
SOURCE_INPUTS = (*BOOT_INPUTS, *(ROOT / 'docs/book/src' / chapter for chapter in (
    'spec/formula-language.md', 'spec/formula-language/grammar.md',
    'spec/formula-language/examples.md', 'spec/reference-skirt.md', 'spec/units-and-tolerances.md')),
    HERE / 'expression_shapes.tsv')

def source_plan():
    """Validate the complete sources without activating stores or creating outputs."""
    try:
        PROFILE['plan_producer'](ROOT, sources=SOURCE_INPUTS)
    except (PROFILE['Refusal'], OSError) as error:
        print('local-environment: REFUSED — ' + str(error), file=sys.stderr)
        raise SystemExit(2) from None

try:
    source_plan()
    CONTEXT = runpy.run_path(str(HERE / 'formula_structure.py'))
    PROFILE = CONTEXT['PROFILE']
    source_plan()
    namespace, reference = CONTEXT['load_reference']()
except (PROFILE['Refusal'], OSError) as error:
    print('local-environment: REFUSED — ' + str(error), file=sys.stderr)
    raise SystemExit(2) from None
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
source_plan()
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
