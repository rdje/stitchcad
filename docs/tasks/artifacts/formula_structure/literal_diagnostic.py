"""Actual reference observations; diagnostic output alone is not a pass verdict."""
from pathlib import Path
import runpy
ROOT = Path(__file__).resolve().parents[4]
namespace, reference = runpy.run_path(str(ROOT / 'docs/tasks/artifacts/formula_structure/formula_input.py'))['load_context']()
reference.pairs = {('length', 'count'): {'/': 'length'},
                   ('ratio', 'ratio'): {'*': 'ratio', '/': 'ratio'}}
for source in ['0.00004 cm + 0.00004 cm', '0 um + 0 um',
               '0.0000004 + 0.0000004', '0.0 + 0.0',
               '1 um / 2 + 1 um / 2', '0.000001 ^ 2',
               str(2 ** 128), '1000000001 um', '360 deg']:
    try:
        node = reference.parse(source)
        reference.infer(node, {})
        value = reference.evaluate(node, {})
        reference.see(value.v)
        print(repr(source), 'tree=', node, 'value=', value,
              'binding-round=', namespace['rnd'](value.v), 'seen-bits=', reference.max_bits)
    except namespace['FErr'] as error:
        print(repr(source), error.token, error.msg)

# Compare the chosen raw sweep to the superseded normalized-binding model that exposed D84.
# The director preserves raw formula sweeps; modulo here is a counterexample, not the chosen rule.
reference.sigs = {'arc_length': [(['angle', 'length'], False, 'length')],
                  'dir': [(['point', 'point'], False, 'angle')]}
statement = reference.statement('let turn: angle = 360 deg', {})
raw = namespace['rnd'](statement[3].v)
normalized = namespace['norm_angle'](raw)
for label, source, environment in [
    ('direct full-turn sweep', 'arc_length(360 deg, 1 um)', {}),
    ('kind-table normalized angle binding', 'arc_length(turn, 1 um)',
     {'turn': {'kind': 'angle', 'value': normalized, 'origin': 'recipe'}}),
]:
    node = reference.parse(source)
    reference.infer(node, environment)
    print(label, 'raw-bound-angle=', raw, 'normalized=', normalized,
          'value=', reference.evaluate(node, environment))
points = {'p': {'kind': 'point', 'value': (0, 0), 'origin': 'geometry'},
          'q': {'kind': 'point', 'value': (1, 6), 'origin': 'geometry'}}
direction = reference.evaluate(reference.parse('dir(p, q)'), points)
nearest = namespace['norm_angle'](namespace['rnd'](reference._deg2(6, 1)))
print('direction p=(0,0), q=(1,6): actual=', direction, 'nearest-rounded=', nearest)

reference.sigs.update({name: [(['angle'], False, 'ratio')] for name in ['sin', 'cos', 'tan']})
for source in ['sin(90 deg)', 'cos(90 deg)', 'tan(45 deg)', 'tan(90 deg)',
               'tan(-90 deg)', 'tan(270 deg)']:
    try:
        node = reference.parse(source)
        reference.infer(node, {})
        print('angular diagnostic', source, reference.evaluate(node, {}))
    except namespace['FErr'] as error:
        print('angular diagnostic', source, error.token, error.msg)
