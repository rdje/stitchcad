"""D136/D137: exact call payloads against the compiled book reference, never copied dispatch."""
from pathlib import Path
import runpy
import re
import sys

ROOT = Path(__file__).resolve().parents[4]
LOADER = runpy.run_path(str(ROOT / 'docs/tasks/artifacts/formula_structure/static_signature_contract.py'))
SOURCE = LOADER['SOURCE']
# Independently authored from the normative function and selector rows.
CALLS = ('sqrt(a)', 'hypot(l,l)', 'abs(r)', 'min(l)', 'max(l,l,l)', 'clamp(l,l,l)',
         'round_to(l,l)', 'sin(1 deg)', 'cos(1 deg)', 'tan(1 deg)', 'atan(r)',
         'atan2(l,l)', 'arc_length(1 deg,l)', 'within(l,l,eps_num)',
         'x(p)', 'y(p)', 'dist(p,p)', 'dir(p,p)', 'len(e)', 'param_at(e,l)', 'point_at(e,r)')
RESULTS = ('length', 'length', 'ratio', 'length', 'length', 'length', 'length',
           'ratio', 'ratio', 'ratio', 'angle', 'angle', 'length', 'boolean',
           'length', 'length', 'length', 'angle', 'length', 'ratio', 'point')
UNBOUND = ('loop', 'unlisted_call', 'sin_missing', 'nurbs_extra', 'solve_extra', 'waist',
           'macro', 'fn', 'function', 'while', 'repeat', 'eps_num', 'eps_geo', 'eps_fmt',
           'eps_imp', 'eps_phys', 'size_index', 'size_count', 'is_base_size',
           'line_segment', 'circular_arc', 'cubic_bezier', 'ordered_construction_recipe')
CURVES = ('nurbs', 'spline', 'bspline')
CONSTRAINTS = ('solve', 'constraint', 'fixpoint')


class Unreadable:
    def __getitem__(self, key):
        raise AssertionError(('D136/D137 argument/namespace read', key))

    def __contains__(self, key):
        raise AssertionError(('D136/D137 argument/namespace read', key))

    def __len__(self):
        raise AssertionError('D136/D137 argument/namespace read:len')

    def __iter__(self):
        raise AssertionError('D136/D137 argument/namespace read:iter')


def curve_contract(replacement=None):
    text = (ROOT / 'docs/book/src/spec/units-and-tolerances.md').read_text()
    if replacement:
        before, after = replacement
        assert text.count(before) == 1, ('curve documentation fault anchor', before)
        text = text.replace(before, after, 1)
    section = text.split('## 4. Curve representation\n', 1)[1].split('\n## ', 1)[0]
    curves = tuple(re.findall(r'^\d+\. \*\*(.+?)\*\*', section, re.M))
    assert curves == ('Line segment', 'Circular arc', 'Cubic Bézier'), ('D136/D137 normative curve set', curves)


def contracts(replacement=None, verbose=True):
    curve_contract()
    assert len(CALLS) == len(RESULTS) == 21, "D136/D137 known call oracle coverage"
    ns, reference = LOADER['load_reference'](replacement)
    assert set(reference.sigs) == {s.split('(')[0] for s in CALLS} | {'if'}, 'D136/D137 call population'
    assert reference.envelope == {**dict.fromkeys(CURVES, 'env_nurbs'),
                                  **dict.fromkeys(CONSTRAINTS, 'env_sketch_constraints')}, 'D136/D137 envelope population'
    for method in ('evaluate', '_evaluate', 'value_of_name', 'resolve_geometry', 'stored'):
        def trapped(*args, **kwargs):
            raise AssertionError('D136/D137 execution/value read')
        setattr(reference, method, trapped)
    env = {name: LOADER['KindOnly'](kind) for name, kind in
           (('a', 'area'), ('l', 'length'), ('r', 'ratio'), ('p', 'point'), ('e', 'edge'))}
    checks = 0
    for source, expected in zip(CALLS, RESULTS):
        assert reference.infer(reference.parse(source), env) == expected, ('D136/D137 known call', source)
        checks += 1

    def refusal(action, token, expected):
        nonlocal checks
        try:
            action()
        except ns['FErr'] as error:
            assert error.token == token, ('D136/D137 token', error.token, token)
            assert error.arguments == expected, ('D136/D137 payload', error.arguments, expected)
        else:
            raise AssertionError(('D136/D137 refusal accepted', token, expected))
        checks += 1

    for name in (*UNBOUND, *CURVES, *CONSTRAINTS):
        payload = {'name': name, 'lookup_scope': 'formula_call', 'origins_searched': ('envelope',)}
        if name in CURVES:
            token = 'env_nurbs'
            payload.update(curve_kind=name, supported_curve_set=('line_segment', 'circular_arc', 'cubic_bezier'))
        elif name in CONSTRAINTS:
            token = 'env_sketch_constraints'
            payload.update(constraint_kind=name, recipe_alternative='ordered_construction_recipe')
        else:
            token = 'formula_unbound_name'
            payload['origins_searched'] = ('envelope', 'builtin_catalog')
        # Direct static/runtime entry points cannot inspect even argument shape or namespace.
        for method in (reference.infer_call, reference.call):
            refusal(lambda: method(name, Unreadable(), Unreadable()), token, payload)
        # Actual parser dispatch, both branches, and a same-spelling scalar cannot grant a callable.
        for source in (name+'(missing_argument)',
                       'if(is_base_size,1 mm,'+name+'(missing_argument))',
                       'if(is_base_size,'+name+'(missing_argument),1 mm)'):
            node = reference.parse(source)
            refusal(lambda: reference.infer(node, {name: LOADER['KindOnly']('length')}), token, payload)
    assert checks == 166, ("D136/D137 exact control coverage", checks)
    if verbose:
        print('call payloads:%d actual cases /21 built-ins /six envelope aliases; exact fields, sources and alternatives; argument/value/execution traps' % checks)
    return checks


FAULTS = (
    ('unknown name missing', '{"name": name, "lookup_scope": "formula_call",\n                        "origins_searched": ("envelope", "builtin_catalog")}',
     '{"lookup_scope": "formula_call", "origins_searched": ("envelope", "builtin_catalog")}'),
    ('unknown name forged', '{"name": name, "lookup_scope": "formula_call",\n                        "origins_searched": ("envelope", "builtin_catalog")}',
     '{"name": "missing", "lookup_scope": "formula_call", "origins_searched": ("envelope", "builtin_catalog")}'),
    ('unknown source order', '("envelope", "builtin_catalog")}', '("builtin_catalog", "envelope")}'),
    ('unknown source omitted', '("envelope", "builtin_catalog")}', '("builtin_catalog",)}'),
    ('call/data scope conflated', '"lookup_scope": "formula_call",\n                        "origins_searched"', '"lookup_scope": "data_name",\n                        "origins_searched"'),
    ('envelope searched catalog invented', '"origins_searched": ("envelope",)}', '"origins_searched": ("envelope", "builtin_catalog")}'),
    ('curve request forged', 'payload.update(curve_kind=name,', 'payload.update(curve_kind="nurbs",'),
    ('curve alternatives omitted', 'supported_curve_set=("line_segment", "circular_arc", "cubic_bezier")', 'supported_curve_set=("line_segment", "circular_arc")'),
    ('constraint request forged', 'payload.update(constraint_kind=name,', 'payload.update(constraint_kind="parallel",'),
    ('recipe alternative forged', 'recipe_alternative="ordered_construction_recipe"', 'recipe_alternative="implicit_solver"'),
    ('static argument observed before refusal', 'def infer_call(self, name, args, env):\n        self._check_callee(name)', 'def infer_call(self, name, args, env):\n        len(args)\n        self._check_callee(name)'),
    ('runtime argument observed before refusal', 'def call(self, name, args, env):\n        self._check_callee(name)', 'def call(self, name, args, env):\n        len(args)\n        self._check_callee(name)'),
)


if __name__ == '__main__':
    assert sys.argv[1:] in [[], ['--mutations']]
    before = SOURCE.read_bytes()
    contracts()
    if sys.argv[1:] == ['--mutations']:
        for name, old, new in FAULTS:
            try:
                contracts((old, new), verbose=False)
            except AssertionError as error:
                assert 'D136/D137' in str(error), (name, 'not a payload/body assertion red', error)
                print('actual compiled call payload assertion red:', name)
            else:
                raise AssertionError(('call payload fault escaped', name))
        print('call payload faults:%d actual compiled body assertion reds; source bytes unchanged' % len(FAULTS))
        for old, new in (('1. **Line segment**', '1. **NURBS**'),
                         ('2. **Circular arc**', '**Circular arc**'),
                         ('3. **Cubic Bézier**', '3. **Cubic Bézier**\n4. **NURBS**')):
            try:
                curve_contract((old, new))
            except AssertionError as error:
                assert 'D136/D137 normative curve set' in str(error), ('not a normative body red', error)
            else:
                raise AssertionError('normative curve fault escaped')
        print('curve documentation:three actual loaded-set assertion reds; tracked book unchanged')
    assert SOURCE.read_bytes() == before
