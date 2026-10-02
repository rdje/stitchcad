"""Independent closed signature matrices against the actual book reference, never value execution."""
from collections.abc import Mapping
from contextlib import redirect_stdout
from itertools import product
from pathlib import Path
import io
import sys

ROOT = Path(__file__).resolve().parents[4]
SOURCE = ROOT / 'docs/tasks/artifacts/formula_language/run_formula_language_census.sh'
BOOK = ROOT / 'docs/book/src'
KINDS = ('length', 'angle', 'area', 'ratio', 'count', 'boolean', 'point', 'edge')
ARITH = KINDS[:5]
NEGATABLE = KINDS[:4]
TOLERANCES = ('eps_num', 'eps_geo', 'eps_fmt', 'eps_imp', 'eps_phys')
RESERVED = {**dict.fromkeys(TOLERANCES, 'length'), 'size_index': 'count',
            'size_count': 'count', 'is_base_size': 'boolean'}
# Authored positionally from the dimensional algebra, independent of the tested table reader.
QUOTIENT = {('length', 'length'): 'ratio', ('length', 'ratio'): 'length',
            ('length', 'count'): 'length', ('angle', 'angle'): 'ratio',
            ('angle', 'ratio'): 'angle', ('angle', 'count'): 'angle',
            ('area', 'length'): 'length', ('area', 'ratio'): 'area',
            ('area', 'count'): 'area', ('area', 'area'): 'ratio',
            ('ratio', 'ratio'): 'ratio', ('ratio', 'count'): 'ratio',
            ('count', 'count'): 'ratio', ('count', 'ratio'): 'count'}
MULTIPLICATION = {}
for left, right, result in [('length', 'length', 'area'), ('length', 'ratio', 'length'),
                            ('length', 'count', 'length'), ('angle', 'ratio', 'angle'),
                            ('angle', 'count', 'angle'), ('area', 'ratio', 'area'),
                            ('area', 'count', 'area'), ('ratio', 'ratio', 'ratio'),
                            ('ratio', 'count', 'ratio'), ('count', 'count', 'count')]:
    MULTIPLICATION[left, right] = MULTIPLICATION[right, left] = result
SIGNATURES = {
    'sqrt': [(('area',), False, 'length'), (('ratio',), False, 'ratio')],
    'hypot': [(('length', 'length'), False, 'length')],
    'abs': [(('T',), False, 'T')], 'min': [(('T',), True, 'T')],
    'max': [(('T',), True, 'T')], 'clamp': [(('T', 'T', 'T'), False, 'T')],
    'round_to': [(('T', 'T'), False, 'T')],
    'sin': [(('angle',), False, 'ratio')], 'cos': [(('angle',), False, 'ratio')],
    'tan': [(('angle',), False, 'ratio')], 'atan': [(('ratio',), False, 'angle')],
    'atan2': [(('length', 'length'), False, 'angle'), (('ratio', 'ratio'), False, 'angle')],
    'arc_length': [(('angle', 'length'), False, 'length')],
    'if': [(('boolean', 'T', 'T'), False, 'T')],
    'within': [(('T', 'T', 'tolerance'), False, 'boolean')],
    'x': [(('point',), False, 'length')], 'y': [(('point',), False, 'length')],
    'dist': [(('point', 'point'), False, 'length')],
    'dir': [(('point', 'point'), False, 'angle')], 'len': [(('edge',), False, 'length')],
    'param_at': [(('edge', 'length'), False, 'ratio')],
    'point_at': [(('edge', 'ratio'), False, 'point')],
}


def load_reference(replacement=None):
    source = SOURCE.read_text()
    assert source.count("<<'PY'\n") == 1
    program = source.split("<<'PY'\n", 1)[1].rsplit('\nPY', 1)[0]
    boundary = 'print("-- tables read from the chapter")'
    assert program.count(boundary) == 1
    prefix = program.split(boundary, 1)[0]
    if replacement:
        before, after = replacement
        assert prefix.count(before) == 1, ('actual fault anchor', before)
        prefix = prefix.replace(before, after, 1)
    namespace = {}
    args = sys.argv
    try:
        sys.argv = ['reference', *(str(BOOK / p) for p in [
            'spec/formula-language.md', 'spec/formula-language/grammar.md',
            'spec/formula-language/examples.md', 'spec/reference-skirt.md', '.',
            'spec/formula-language'])]
        with redirect_stdout(io.StringIO()):
            exec(compile(prefix, str(SOURCE), 'exec'), namespace)
    finally:
        sys.argv = args
    assert namespace['fails'] == 0, 'actual table loading failed'
    return namespace, namespace['EV']


class KindOnly(Mapping):
    """Only kind is observable, including through Mapping.get; values never default to None."""
    def __init__(self, kind):
        self._kind = kind

    def __getitem__(self, key):
        assert key == 'kind', ('static checker accessed non-kind metadata', key)
        return self._kind

    def __iter__(self):
        return iter(('kind',))

    def __len__(self):
        return 1


def contracts(replacement=None, verbose=True):
    namespace, reference = load_reference(replacement)
    actual = {name: [(tuple(args), variadic, result) for args, variadic, result in rows]
              for name, rows in reference.sigs.items()}
    assert actual == SIGNATURES, 'function/selector population or payload drift'
    assert {name: entry[0] for name, entry in reference.reserved.items()} == RESERVED
    actual_product = {pair: values['*'] for pair, values in reference.pairs.items() if values.get('*')}
    actual_quotient = {pair: values['/'] for pair, values in reference.pairs.items() if values.get('/')}
    assert actual_product == MULTIPLICATION and actual_quotient == QUOTIENT
    assert set(namespace['ARITH']) == set(ARITH) and set(namespace['NEGATABLE']) == set(NEGATABLE)
    env = {'v_' + kind: KindOnly(kind=kind) for kind in KINDS}
    for method in ['evaluate', '_evaluate', 'value_of_name', 'resolve_geometry', 'call', 'stored']:
        def trapped(*args, **kwargs):
            raise AssertionError('static checker invoked numerical/geometry execution')
        setattr(reference, method, trapped)
    checks = 0

    def check(source, expected, token='formula_dimension', hint=None):
        nonlocal checks
        try:
            result = reference.infer(reference.parse(source), env)
        except namespace['FErr'] as error:
            assert expected is None and error.token == token, ('signature refusal', source, expected, error)
            assert hint is None or hint in error.msg, ('missing signature hint', source, error)
        else:
            assert result == expected, ('signature mismatch', source, expected, result)
        checks += 1

    for kind in KINDS:
        check('-v_' + kind, kind if kind in NEGATABLE else None)
        check('v_' + kind + ' ^ 2', {'length': 'area', 'ratio': 'ratio', 'count': 'count'}.get(kind))
    for left, right in product(KINDS, repeat=2):
        same = left == right and left in ARITH
        operands = 'v_' + left + ' %s v_' + right
        for op in ['+', '-']:
            check(operands % op, left if same else None)
        for op in ['==', '!=', '<', '<=', '>', '>=']:
            check(operands % op, 'boolean' if same else None)
        check(operands % '*', MULTIPLICATION.get((left, right)),
              hint='arc_length' if {left, right} == {'angle', 'length'} else None)
        check(operands % '/', QUOTIENT.get((left, right)),
              hint='arc_length' if {left, right} == {'angle', 'length'} else None)

    # Exhaustive positional kinds for every fixed signature at its arity; every variadic
    # kind combination at one/two/three args, plus widest accepted source argument boundary.
    for name, rows in SIGNATURES.items():
        if name in ('if', 'within'):
            continue
        arity = len(rows[0][0])
        variadic = name in ('min', 'max')
        arities = (1, 2, 3) if variadic else (arity,)
        for count in arities:
            for kinds in product(KINDS, repeat=count):
                expected = None
                if name in ('abs', 'min', 'max', 'clamp', 'round_to'):
                    if kinds[0] in ARITH and len(set(kinds)) == 1:
                        expected = kinds[0]
                else:
                    for args, _, result in rows:
                        if kinds == args:
                            expected = result
                check(name + '(' + ', '.join('v_' + k for k in kinds) + ')', expected)
        check(name + '()', None, token='formula_parse')
        if not variadic:
            for count in {max(1, arity - 1), arity + 1} - {arity}:
                check(name + '(' + ', '.join(['v_length'] * count) + ')', None)
        else:
            for kind in ARITH:
                check(name + '(' + ', '.join(['v_' + kind] * 255) + ')', kind)
                check(name + '(' + ', '.join(['v_' + kind] * 256) + ')', None, token='formula_domain')

    for condition, left, right in product(KINDS, repeat=3):
        expected = left if condition == 'boolean' and left == right and left in ARITH else None
        check('if(v_%s, v_%s, v_%s)' % (condition, left, right), expected)
    # Both branches must resolve even if the condition kind is available without a value.
    for expression in ['if(v_boolean, missing_name, v_length)',
                       'if(v_boolean, v_length, missing_name)']:
        check(expression, None, token='formula_unbound_name')
    for count in (0, 1, 2, 4):
        check('if(' + ', '.join(['v_boolean'] * count) + ')', None, token='formula_parse')
    for tolerance in RESERVED:
        for left, right in product(KINDS, repeat=2):
            expected = 'boolean' if left == right and left in ARITH and tolerance in TOLERANCES else None
            check('within(v_%s, v_%s, %s)' % (left, right, tolerance), expected)
    for third in ['1 um', 'v_length', '(eps_num + eps_num)', 'abs(eps_num)']:
        check('within(v_length, v_length, ' + third + ')', None)
    for count in (0, 1, 2, 4):
        check('within(' + ', '.join(['v_length'] * count) + ')', None,
              token='formula_parse' if count == 0 else 'formula_dimension')
    for construct, token in [('nurbs', 'env_nurbs'), ('spline', 'env_nurbs'),
                             ('bspline', 'env_nurbs'), ('solve', 'env_sketch_constraints'),
                             ('constraint', 'env_sketch_constraints'), ('fixpoint', 'env_sketch_constraints')]:
        check(construct + '(missing_name)', None, token=token)
    check('unlisted_call(v_length)', None, token='formula_unbound_name')
    if verbose:
        print('static signature contracts:', checks, 'actual parse/infer cases / 0 fail;',
              len(SIGNATURES), 'closed names; value/geometry access trapped')
    return checks


FAULTS = [
    ('one-argument variadic', 'if len(kinds) < len(want_list): return False',
     'if len(kinds) < max(2, len(want_list)): return False'),
    ('tolerance name role', 'and args[2][1] in {"eps_num", "eps_geo", "eps_fmt", "eps_imp", "eps_phys"}):',
     'and args[2][1] in self.reserved):'),
    ('unary count', 'if k not in NEGATABLE:', 'if k not in ARITH:'),
    ('fixed square area', 'return {"length": "area", "ratio": "ratio", "count": "count"}[k]',
     'return {"length": "length", "ratio": "ratio", "count": "count"}[k]'),
    ('addition consistency', 'if a != b or a not in ARITH:', 'if a not in ARITH:'),
    ('division direction', 'pair = self.pairs.get((a, b))\n            res =',
     'pair = self.pairs.get((a, b)) or self.pairs.get((b, a))\n            res ='),
    ('variadic homogeneous kinds', 'if not all(k == first for k in kinds): return False', 'if False: return False'),
    ('generic arithmetic membership', 'if got not in ARITH: return False', 'if False: return False'),
    ('conditional Boolean condition', 'if c != "boolean":', 'if False:'),
    ('conditional consistent branches', 'if a != b or a not in ARITH:\n                raise FErr("formula_dimension", "a conditional',
     'if a not in ARITH:\n                raise FErr("formula_dimension", "a conditional'),
    ('value read through get', 'if name in env: return env[name]["kind"]',
     'if name in env: return env[name].get("value") or env[name]["kind"]'),
    ('envelope priority', 'if name in self.envelope:\n            raise FErr(self.envelope[name], "the envelope owns this construct, not the language")\n        if name not in self.sigs:',
     'if name not in self.sigs:'),
]


def mutations():
    before = SOURCE.read_bytes()
    for name, old, new in FAULTS:
        # Some generic predicates appear in multiple operator branches. Scope the selected
        # addition guard explicitly so an accidental unmatched mutation cannot pass.
        if name == 'addition consistency':
            old = 'if op in ("+", "-"):\n                ' + old
            new = 'if op in ("+", "-"):\n                ' + new
        try:
            contracts((old, new), verbose=False)
        except AssertionError as error:
            signature = "('static checker accessed non-kind metadata'" if name == 'value read through get' else "('signature "
            assert str(error).startswith(signature), (name, 'unexpected failure', error)
            print('  actual compiled static assertion red:', name)
        else:
            raise AssertionError(('static assertion missed actual fault', name))
    assert SOURCE.read_bytes() == before, 'in-memory faults changed producer on disk'
    print('static signature faults:', len(FAULTS), 'actual assertion reds; producer unchanged')


if __name__ == '__main__':
    contracts()
    if '--mutations' in sys.argv:
        mutations()
