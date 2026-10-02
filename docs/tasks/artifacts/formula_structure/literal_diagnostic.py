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
