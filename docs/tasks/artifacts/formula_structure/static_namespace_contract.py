"""Independent declarations and static headers; execution/value metadata are forbidden."""
from collections.abc import Mapping
from pathlib import Path
import runpy
import sys

ROOT = Path(__file__).resolve().parents[4]
HELPER = ROOT / 'docs/tasks/artifacts/formula_structure/static_signature_contract.py'
LOADER = runpy.run_path(str(HELPER))
SOURCE = LOADER['SOURCE']
ORIGINS = ('measurement', 'ease', 'parameter', 'profile', 'material', 'geometry', 'recipe', 'size', 'tolerance')
KINDS = ('length', 'angle', 'area', 'ratio', 'count', 'boolean', 'point', 'edge')
BINDABLE = KINDS[:6]
TOLERANCES = ('eps_num', 'eps_geo', 'eps_fmt', 'eps_imp', 'eps_phys')
RESERVED = {**dict.fromkeys(TOLERANCES, 'length'), 'size_index': 'count', 'size_count': 'count', 'is_base_size': 'boolean'}


class Declaration(Mapping):
    def __init__(self, kind, origin):
        self.metadata = {'kind': kind, 'origin': origin}

    def __getitem__(self, key):
        assert key in self.metadata, ('static namespace read value/state/geometry', key)
        return self.metadata[key]

    def __iter__(self):
        return iter(self.metadata)

    def __len__(self):
        return len(self.metadata)


class ReservedKind:
    def __init__(self, kind):
        self.kind = kind

    def __getitem__(self, index):
        assert index == 0, ('static namespace read reserved availability/value', index)
        return self.kind


def contracts(replacement=None, verbose=True):
    ns, reference = LOADER['load_reference'](replacement)
    assert reference.origins == frozenset(ORIGINS), 'origin population drift'
    assert ns['origins'] == list(ORIGINS), 'actual table origin order/population drift'
    assert set(reference.bindable) == set(BINDABLE)
    assert {name: entry[0] for name, entry in reference.reserved.items()} == RESERVED
    reference.reserved = {name: ReservedKind(kind) for name, kind in RESERVED.items()}
    for method in ('evaluate', '_evaluate', 'value_of_name', 'resolve_geometry', 'call', 'stored'):
        def trapped(*args, **kwargs):
            raise AssertionError('static namespace invoked execution')
        setattr(reference, method, trapped)
    checks = 0

    def refusal(action, token, detail=None):
        nonlocal checks
        try:
            action()
        except ns['FErr'] as error:
            assert error.token == token, ('namespace refusal token', error.token, token, error.msg)
            assert detail is None or detail in error.msg, ('namespace refusal detail', error.msg, detail)
        else:
            raise AssertionError(('static namespace accepted refusal', token, detail))
        checks += 1

    def statement(source, env, role, name, annotation):
        nonlocal checks
        result = reference.static_statement(source, env)
        assert result[:3] == (role, name, annotation), ('static header', source, result)
        assert env == before, ('static statement mutated caller namespace', source)
        checks += 1

    # All declared origin/kind pairs can be checked without observing a value or state.
    for origin in ORIGINS:
        for kind in KINDS:
            declaration = Declaration(kind, origin)
            env = reference.namespace([('input_value', declaration)])
            assert env == {'input_value': declaration} and env['input_value'] is declaration
            assert reference.infer(reference.parse('input_value'), env) == kind
            checks += 1
            before = dict(env)
            for target in BINDABLE:
                if origin == 'recipe':
                    token, detail = 'formula_rebinding', 'input_value'
                else:
                    token, detail = 'formula_ambiguous_name', 'both %s and recipe' % origin
                refusal(lambda: reference.static_statement('let input_value: %s = 1' % target, env), token, detail)
            assert env == before
    # Duplicate declarations are consumed as pairs, not overwritten in an input dictionary.
    for first in ORIGINS:
        for second in ORIGINS:
            entries = [('collision', Declaration('length', first)), ('collision', Declaration('length', second))]
            refusal(lambda: reference.namespace(entries), 'formula_ambiguous_name', 'both %s and %s' % (first, second))
    for name in RESERVED:
        for origin in ORIGINS:
            refusal(lambda: reference.namespace([(name, Declaration('length', origin))]), 'formula_rebinding', 'reserved')
        for kind in BINDABLE:
            refusal(lambda: reference.static_statement('let %s: %s = 1' % (name, kind), {}), 'formula_rebinding', 'reserved')
        assert reference.infer(reference.parse(name), {}) == RESERVED[name]
        checks += 1
    for name in ('a', 'waist_girth', 'point_2_x', 'a2', 'let_value', 'assertion', 'if_else'):
        entry = Declaration('length', 'measurement')
        assert reference.namespace([(name, entry)]) == {name: entry}
        checks += 1
    for name in ('Upper', 'waistGirth', '_a', 'a_', 'a__b', 'é', 'let', 'assert', 'if'):
        refusal(lambda: reference.namespace([(name, Declaration('length', 'measurement'))]), 'formula_parse')
    for kind, origin in [('text', 'parameter'), ('float', 'parameter'), ('length', 'computed_fixture')]:
        refusal(lambda: reference.namespace([('invalid', Declaration(kind, origin))]), 'formula_parse', 'kind/origin')

    env = {('v_' + kind): Declaration(kind, 'parameter') for kind in KINDS}
    before = dict(env)
    for declared in BINDABLE:
        for actual in KINDS:
            source = 'let saved: %s = v_%s' % (declared, actual)
            if declared == actual:
                statement(source, env, 'let', 'saved', declared)
            else:
                refusal(lambda: reference.static_statement(source, env), 'formula_dimension')
    for kind in ('point', 'edge', 'text', 'float'):
        refusal(lambda: reference.static_statement('let saved: %s = v_length' % kind, env), 'formula_dimension')
    refusal(lambda: reference.static_statement('let saved: Length = v_length', env), 'formula_parse')
    for name in ('let', 'assert', 'if', 'saved__value', 'Upper'):
        refusal(lambda: reference.static_statement('let %s: length = v_length' % name, env), 'formula_parse')
        refusal(lambda: reference.static_statement('assert %s: eps_num = v_length == v_length' % name, env), 'formula_parse')
    for name in ('closure', 'v_length', 'eps_num'):
        statement('assert %s: eps_num = v_length == v_length' % name, env, 'assert', name, 'eps_num')
    for tolerance in TOLERANCES:
        for left in KINDS:
            for right in KINDS:
                source = 'assert closure: %s = v_%s == v_%s' % (tolerance, left, right)
                if left == right and left in KINDS[:5]:
                    statement(source, env, 'assert', 'closure', tolerance)
                else:
                    refusal(lambda: reference.static_statement(source, env), 'formula_dimension')
    for tolerance in ('size_index', 'size_count', 'is_base_size', 'eps_chord', 'unbound', 'length'):
        refusal(lambda: reference.static_statement('assert closure: %s = v_length == v_length' % tolerance, env), 'formula_parse', 'TOLERANCE')
    for source in ('let saved: length = missing', 'let saved: length = saved',
                   'let saved: length = if(is_base_size, missing, v_length)',
                   'let saved: length = if(is_base_size, v_length, missing)',
                   'assert closure: eps_num = missing == v_length',
                   'assert closure: eps_num = v_length == missing'):
        refusal(lambda: reference.static_statement(source, env), 'formula_unbound_name', 'missing' if 'missing' in source else 'saved')
    for source in ('let saved: length = if(is_base_size, v_length, v_length)',
                   'let saved: length = if(v_boolean, v_length, v_length)'):
        statement(source, env, 'let', 'saved', 'length')
    for source in ('assert closure: eps_num = v_length == v_length == v_length',
                   'assert closure: eps_num = 1cm == 1 cm',
                   'assert closure: eps_num = 1\tcm == 1 cm'):
        refusal(lambda: reference.static_statement(source, env), 'formula_parse')
    assert env == before
    # The public runtime adapter must complete static refusal before any callback runs.
    for source, context, token in [
        ('let eps_num: length = 1 cm', {}, 'formula_rebinding'),
        ('let v_length: length = 1 cm', env, 'formula_ambiguous_name'),
        ('assert c: size_index = 1 cm == 1 cm', {}, 'formula_parse'),
        ('assert c: eps_num = v_boolean == v_boolean', env, 'formula_dimension'),
    ]:
        refusal(lambda: reference.statement(source, context), token)
    if verbose:
        print('static namespace contract: %d cases / nine origins / eight reserved names; value/state/geometry execution trapped' % checks)
    return checks


FAULTS = [
    ('namespace spelling', '            self._identifier(name)\n            kind, origin', '            kind, origin'),
    ('declaration metadata', 'or origin not in self.origins:', 'or False:'),
    ('namespace reserved rebinding', 'if name in self.reserved:\n                raise FErr("formula_rebinding", "reserved `%s` cannot be declared', 'if False:\n                raise FErr("formula_rebinding", "reserved `%s` cannot be declared'),
    ('pair collision erased', 'if name in env:\n                raise FErr("formula_ambiguous_name", "`%s` is declared by both %s and %s"', 'if False:\n                raise FErr("formula_ambiguous_name", "`%s` is declared by both %s and %s"'),
    ('let reserved rebinding', 'if name in self.reserved:\n                raise FErr("formula_rebinding", "reserved `%s` cannot be rebound', 'if False:\n                raise FErr("formula_rebinding", "reserved `%s` cannot be rebound'),
    ('input shadow', 'if name in env:\n                if env[name]["origin"] == "recipe":', 'if name in env and env[name]["origin"] == "recipe":\n                if env[name]["origin"] == "recipe":'),
    ('recipe rebinding token', 'if env[name]["origin"] == "recipe":', 'if False:'),
    ('let kind mismatch', 'if got != kind:', 'if False:'),
    ('assertion tolerance role', 'if tol_name not in {"eps_num", "eps_geo", "eps_fmt", "eps_imp", "eps_phys"}:', 'if tol_name not in self.reserved:'),
    ('assertion comparison omitted', 'self.infer(("cmp", "==", left, right), env)', 'self.infer(left, env)'),
    ('value metadata read', 'kind, origin = entry["kind"], entry["origin"]', 'kind, origin = entry["kind"], entry["origin"]\n            entry.get("value")'),
    ('runtime static phase bypass', 'checked = self.static_statement(src, env)',
     'self.evaluate(self.parse("1"), env)\n        checked = self.static_statement(src, env)'),
    ('numeric execution added', '            return checked\n        if role == "assert":', '            self.evaluate(node, env)\n            return checked\n        if role == "assert":'),
]
if __name__ == '__main__':
    original = SOURCE.read_bytes()
    contracts()
    if '--mutations' in sys.argv:
        for name, before, after in FAULTS:
            try:
                contracts((before, after), False)
            except AssertionError as error:
                expected = ('static namespace accepted refusal', 'namespace refusal token',
                            'namespace refusal detail', 'static namespace read value/state/geometry',
                            'static namespace invoked execution', 'static header')
                assert any(marker in str(error) for marker in expected), (name, 'not a body assertion red', error)
                print('  actual compiled namespace assertion red:', name)
            else:
                raise AssertionError(('actual namespace fault escaped', name))
        assert SOURCE.read_bytes() == original
        print('static namespace faults: %d actual assertion reds; producer unchanged' % len(FAULTS))
