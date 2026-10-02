"""Authored product shape fixtures checked by the independently implemented book reference."""
from pathlib import Path
import runpy
ROOT=Path(__file__).resolve().parents[4]
namespace,reference=runpy.run_path(str(ROOT/'docs/tasks/artifacts/formula_structure/formula_input.py'))['load_context']()
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
rows=[line.split('\t') for line in (ROOT/'docs/tasks/artifacts/formula_structure/normalized_expression_shapes.tsv').read_text().splitlines() if line and not line.startswith('#')]
assert len(rows)==24
for source,want,nodes,depth in rows:
    tree=reference.parse(source)
    assert shape(tree)==want,(source,shape(tree),want)
    assert reference.count_nodes(tree)==int(nodes),(source,'count')
    assert reference.if_depth(tree)==int(depth),(source,'depth')
print('normalized expression reference: 24 independently authored shape/count/depth rows pass')
