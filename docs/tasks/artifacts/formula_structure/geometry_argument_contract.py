"""D140: geometry-provider arguments must pass complete static checking before execution."""
from copy import deepcopy
from fractions import Fraction
from itertools import product
from pathlib import Path
import runpy
import sys

ROOT = Path(__file__).resolve().parents[4]
HELPER = runpy.run_path(str(ROOT / 'docs/tasks/artifacts/formula_structure/static_signature_contract.py'))
KINDS = ('length', 'angle', 'area', 'ratio', 'count', 'boolean', 'point', 'edge')
SOURCE = HELPER['SOURCE']


def provider(kind, expressions):
    return {'kind': kind, 'origin': 'geometry', 'state': 'derived', 'lazy': True,
            'value': None, 'exprs': expressions}


def contracts(replacement=None, verbose=True):
    ns, ref = HELPER['load_reference'](replacement)
    assert set(ns['kinds']) == set(KINDS), 'D140 kind population drift'
    env = {'v_' + kind: HELPER['KindOnly'](kind) for kind in KINDS}
    originals = {name: getattr(ref, name) for name in ('evaluate', '_evaluate', 'value_of_name', 'call', 'stored')}

    def trapped(*args, **kwargs):
        raise AssertionError('D140 premature numerical execution')

    for name in originals:
        setattr(ref, name, trapped)
    checks = 0
    refusals = 0

    def reject(kind, expressions, token, kinds=None, child=None):
        nonlocal checks, refusals
        entry = provider(kind, expressions)
        before = deepcopy(entry)
        identities = dict(env)
        metadata = {name: value._kind for name, value in env.items()}
        try:
            ref.resolve_geometry(entry, env)
        except ns['FErr'] as error:
            assert error.token == token, ('D140 token', expressions, error.token, token)
            if kinds is not None:
                names = ('len',) if kind == 'edge' else ('x', 'y')
                expected = {'diagnostic_scope': 'geometry_arguments', 'operation': kind,
                            'operand_names': names, 'operand_kinds': kinds,
                            'wanted_kinds': ('length',) * len(names)}
                assert error.arguments == expected, ('D140 complete payload', expressions, error.arguments, expected)
                refusals += 1
            elif child is not None:
                field, value = child
                assert error.arguments[field] == value, ('D140 child priority', expressions, error.arguments)
        else:
            raise AssertionError(('D140 invalid provider accepted', expressions))
        assert entry == before, ('D140 static refusal changed provider', expressions, entry)
        assert env.keys() == identities.keys() and all(env[k] is identities[k] for k in identities), 'D140 environment identity changed'
        assert {name: value._kind for name, value in env.items()} == metadata, 'D140 environment metadata changed'
        checks += 1

    for x, y in product(KINDS, repeat=2):
        if (x, y) != ('length', 'length'):
            reject('point', {'x': 'v_' + x, 'y': 'v_' + y}, 'formula_dimension', (x, y))
    for kind in KINDS:
        if kind != 'length':
            reject('edge', {'len': 'v_' + kind}, 'formula_dimension', (kind,))
    reject('point', {'x': '1.0', 'y': '2 deg'}, 'formula_dimension', ('ratio', 'angle'))
    reject('edge', {'len': '1 deg'}, 'formula_dimension', ('angle',))
    reject('point', {'x': '1 um / 0', 'y': '1 deg'}, 'formula_dimension', ('length', 'angle'))
    reject('point', {'x': 'hypot(3 um,4 um)', 'y': '1 deg'}, 'formula_dimension', ('length', 'angle'))
    reject('edge', {'len': 'sin(90 deg)'}, 'formula_dimension', ('ratio',))
    for expressions, token, child in (
        ({'x': 'v_ratio', 'y': 'missing'}, 'formula_unbound_name', ('name', 'missing')),
        ({'x': 'missing', 'y': 'v_ratio'}, 'formula_unbound_name', ('name', 'missing')),
        ({'x': 'missing_first', 'y': 'missing_last'}, 'formula_unbound_name', ('name', 'missing_first')),
        ({'x': 'v_ratio', 'y': 'nurbs(missing)'}, 'env_nurbs', ('name', 'nurbs')),
        ({'x': 'v_ratio', 'y': 'loop(missing)'}, 'formula_unbound_name', ('name', 'loop')),
        ({'x': 'v_ratio', 'y': 'v_length + v_angle'}, 'formula_dimension', ('operation', '+')),
        ({'x': 'missing', 'y': '('}, 'formula_parse', None),
        ({'x': 'if(1==1,1 um,missing)', 'y': '2 um'}, 'formula_unbound_name', ('name', 'missing')),
    ):
        reject('point', expressions, token, child=child)
    reject('edge', {'len': 'v_angle + missing'}, 'formula_unbound_name', child=('name', 'missing'))
    reject('edge', {'len': 'nurbs(missing)'}, 'env_nurbs', child=('name', 'nurbs'))
    for name, method in originals.items():
        setattr(ref, name, method)

    # Independently authored values and executed contributions, including asymmetric coordinates.
    for expressions, expected, components in (
        ({'x': '1 mm', 'y': '2 mm'}, (1000, 2000), ((), ())),
        ({'x': 'hypot(3 um,4 um)', 'y': '7 um'}, (5, 7), (('hypot',), ())),
        ({'x': '7 um', 'y': 'hypot(3 um,4 um)'}, (7, 5), ((), ('hypot',))),
        ({'x': 'sqrt((5 um)^2)', 'y': 'hypot(5 um,12 um)'}, (5, 13), (('sqrt',), ('hypot',))),
        ({'x': 'supplied', 'y': 'hypot(3 um,4 um)'}, (11, 5), (('upstream',), ('hypot',))),
    ):
        entry = provider('point', expressions)
        values = {'p': entry, 'supplied': {'kind': 'length', 'origin': 'recipe', 'state': 'derived',
                                         'value': Fraction(11), 'sources': frozenset(('upstream',))}}
        count = []
        resolve = ref.resolve_geometry

        def observed(entry, environment):
            count.append(1)
            return resolve(entry, environment)

        ref.resolve_geometry = observed
        try:
            for _ in range(2):
                whole = ref.value_of_name('p', values)
                assert whole.v == tuple(map(Fraction, expected)), ('D140 exact point value', expressions, whole.v)
                wanted = tuple(frozenset(c) for c in components)
                assert whole.components == wanted, ('D140 coordinate contributions', expressions, whole.components)
                assert whole.sources == wanted[0] | wanted[1], ('D140 point contributions', expressions, whole.sources)
                for axis, number, sources in zip(('x', 'y'), expected, wanted):
                    actual = ref.evaluate(ref.parse(axis + '(p)'), values)
                    assert actual.kind == 'length' and actual.v == number and actual.sources == sources, ('D140 selector replay', expressions, axis, actual)
                assert entry['lazy'] is False and entry['value'] == whole.v, 'D140 completed point cache'
                checks += 1
            assert count == [1], 'D140 point cache resolved twice'
        finally:
            ref.resolve_geometry = resolve
    edge = provider('edge', {'len': 'hypot(3 um,4 um)'})
    values = {'e': edge}
    for _ in range(2):
        actual = ref.evaluate(ref.parse('len(e)'), values)
        assert actual.kind == 'length' and actual.v == 5 and actual.sources == frozenset(('hypot',)), 'D140 edge contribution/cache replay'
        assert edge['lazy'] is False and edge['value'] == Fraction(5), 'D140 completed edge cache'
        checks += 1
    if verbose:
        print('geometry argument contracts:', checks, 'independent cases /', refusals,
              'complete dimension refusals / execution traps and cache/provenance replay / 0 fail')
    return checks


FAULTS = (
    ('skip complete Length guard', 'if any(kind != "length" for kind in kinds):', 'if False:'),
    ('omit y kind', 'if any(kind != "length" for kind in kinds):', 'if kinds[0] != "length":'),
    ('execute during kind checking', 'kinds = tuple(self.infer(node, env) for node in nodes)',
     'kinds = tuple(self.evaluate(node, env).kind for node in nodes)'),
    ('reverse kinds/order', 'kinds = tuple(self.infer(node, env) for node in nodes)',
     'kinds = tuple(self.infer(node, env) for node in reversed(nodes))'),
    ('invent actual kinds', '"operand_kinds": kinds', '"operand_kinds": ("length",) * len(names)'),
    ('omit argument name', '"operand_names": names', '"operand_names": names[:1]'),
    ('wrong wanted kinds', '"wanted_kinds": ("length",) * len(names)', '"wanted_kinds": ("ratio",) * len(names)'),
    ('wrong scope', '"diagnostic_scope": "geometry_arguments"', '"diagnostic_scope": "formula_call"'),
    ('premature cache flag', 'kinds = tuple(self.infer(node, env) for node in nodes)',
     'entry["lazy"] = False\n        kinds = tuple(self.infer(node, env) for node in nodes)'),
    ('lose y contributions', 'entry["sources"] = xs.sources | ys.sources', 'entry["sources"] = xs.sources'),
    ('swap coordinate sources', 'entry["components"] = (xs.sources, ys.sources)', 'entry["components"] = (ys.sources, xs.sources)'),
    ('lose edge contributions', 'entry["sources"] = values[0].sources', 'entry["sources"] = frozenset()'),
)


def mutations():
    original = SOURCE.read_bytes()
    for label, before, after in FAULTS:
        try:
            contracts((before, after), verbose=False)
        except AssertionError as error:
            assert str(error).startswith(('D140 ', "('D140 ")), (label, 'unexpected fault failure', error)
            print('  actual compiled geometry argument assertion red:', label)
        else:
            raise AssertionError(('D140 missed actual fault', label))
    assert SOURCE.read_bytes() == original, 'D140 producer changed on disk'
    print('geometry argument faults:', len(FAULTS), 'actual compiled body assertion reds; source unchanged')


if __name__ == '__main__':
    contracts()
    if '--mutations' in sys.argv:
        mutations()
