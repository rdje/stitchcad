"""Authored product shape fixtures checked by the independently implemented book reference."""
from pathlib import Path
import runpy
import sys
ROOT=Path(__file__).resolve().parents[4]
HERE = ROOT / 'docs/tasks/artifacts/formula_structure'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
BOOT_INPUTS = (Path(__file__),
               PROFILE_SOURCE,
               HERE / 'formula_structure.py',
               HERE / 'static_signature_contract.py',
               ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh',
               HERE / 'formula_input.py')
SOURCE_INPUTS = (*BOOT_INPUTS, *(ROOT / 'docs/book/src' / chapter for chapter in (
    'spec/formula-language.md', 'spec/formula-language/grammar.md',
    'spec/formula-language/examples.md', 'spec/reference-skirt.md', 'spec/units-and-tolerances.md')),
    HERE / 'normalized_expression_shapes.tsv')

def source_plan():
    """Validate the complete sources without activating stores or creating outputs."""
    try:
        PROFILE['plan_producer'](ROOT, sources=SOURCE_INPUTS)
    except (PROFILE['Refusal'], OSError) as error:
        print('local-environment: REFUSED — ' + str(error), file=sys.stderr)
        raise SystemExit(2) from None

try:
    source_plan()
    CONTEXT = runpy.run_path(str(HERE / 'formula_input.py'))
    PROFILE = CONTEXT['PROFILE']
    source_plan()
    namespace, reference = CONTEXT['load_context']()
except (PROFILE['Refusal'], OSError) as error:
    print('local-environment: REFUSED — ' + str(error), file=sys.stderr)
    raise SystemExit(2) from None
OPERATORS={'+':'Add','-':'Subtract','*':'Multiply','/':'Divide','==':'Equal','!=':'NotEqual',
           '<':'Less','<=':'LessEqual','>':'Greater','>=':'GreaterEqual'}
def shape(node):
    # Read the reference's parsed AST, never its evaluator or product normalization.
    if node[0]=='name': return node[1]
    if node[0]=='lit': return '%s:%s'%(node[1],node[2])
    if node[0] in {'bin','cmp'}:
        return '('+OPERATORS[node[1]]+' '+shape(node[2])+' '+shape(node[3])+')'
    if node[0] in {'neg','sq'}:
        return '('+('neg' if node[0]=='neg' else 'square')+' '+shape(node[1])+')'
    if node[0]=='call': return '('+node[1]+' '+' '.join(map(shape,node[2]))+')'
    assert node[0]=='if',node
    return '(if '+' '.join(map(shape,node[1:]))+')'
source_plan()
rows=[line.split('\t') for line in (ROOT/'docs/tasks/artifacts/formula_structure/normalized_expression_shapes.tsv').read_text().splitlines() if line and not line.startswith('#')]
assert len(rows)==24
for source,want,nodes,depth in rows:
    tree=reference.parse(source)
    assert shape(tree)==want,(source,shape(tree),want)
    assert reference.count_nodes(tree)==int(nodes),(source,'count')
    assert reference.if_depth(tree)==int(depth),(source,'depth')
print('normalized expression reference: 24 independently authored shape/count/depth rows pass')
