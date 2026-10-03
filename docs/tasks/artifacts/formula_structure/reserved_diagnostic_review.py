"""D131 reproduction only: reserved refusals cannot supply two recipe indices.

This records the existing reference and canonical table conflict. It does not
accept a new diagnostic schema or certify the product namespace.
"""
from pathlib import Path
import runpy
import sys

ROOT = Path(__file__).resolve().parents[4]
LOADER = runpy.run_path(str(ROOT / 'docs/tasks/artifacts/formula_structure/static_signature_contract.py'))
SOURCE = LOADER['SOURCE']
CONTRACT = ROOT / 'docs/book/src/spec/formula-language.md'
NAMES = ('eps_num', 'eps_geo', 'eps_fmt', 'eps_imp', 'eps_phys',
         'size_index', 'size_count', 'is_base_size')
ORIGINS = ('measurement', 'ease', 'parameter', 'profile', 'material',
           'geometry', 'recipe', 'size', 'tolerance')
KINDS = ('length', 'angle', 'area', 'ratio', 'count', 'boolean')
CURRENT_ROW = '| `formula_rebinding` | a `let` re-binds a name this recipe already bound | the name, both statement indices |'


def review(replacement=None, verbose=True):
    assert CURRENT_ROW in CONTRACT.read_text(), 'D131 canonical row changed; revisit the proposal'
    namespace, reference = LOADER['load_reference'](replacement)
    cases = 0
    examples = []

    def refused(action, expected_name, expected_origin, role):
        nonlocal cases
        try:
            action()
        except namespace['FErr'] as error:
            assert error.token == 'formula_rebinding', ('reserved refusal token', error.token)
            assert 'reserved' in error.msg and expected_name in error.msg, ('reserved refusal detail', error.msg)
            if expected_origin is not None:
                assert expected_origin in error.msg, ('declaration origin lost', error.msg)
            assert error.arguments == {}, ('current reference arguments changed', error.arguments)
            if expected_name == 'eps_num' and expected_origin in (None, 'measurement'):
                examples.append((role, error.token, error.msg, error.arguments))
        else:
            raise AssertionError(('reserved refusal accepted', expected_name, role))
        cases += 1

    for name in NAMES:
        for origin in ORIGINS:
            declarations = [(name, {'kind': 'length', 'origin': origin})]
            before = [(key, dict(value)) for key, value in declarations]
            refused(lambda: reference.namespace(declarations), name, origin, 'initial declaration')
            assert declarations == before, 'namespace mutated authored declarations'
        for kind in KINDS:
            env = {}
            refused(lambda: reference.static_statement('let %s:%s=1' % (name, kind), env),
                    name, None, 'let %s' % kind)
            assert env == {}, 'static refusal mutated the caller namespace'

    # The ordinary recipe rebinding is a different case: two real statements can exist.
    try:
        reference.static_statement('let width:length=1 cm',
                                   {'width': {'kind': 'length', 'origin': 'recipe'}})
    except namespace['FErr'] as error:
        assert error.token == 'formula_rebinding' and 'bound twice' in error.msg
        assert error.arguments == {}, 'single-statement reference scope changed'
    else:
        raise AssertionError('ordinary rebinding accepted')
    cases += 1

    if verbose:
        print('D131 reproduction: %d reserved refusals / 1 ordinary rebinding; canonical row requires two indices' % (cases - 1))
        print('Reserved initial declarations have no recipe statement; reserved metadata has no prior recipe statement.')
        for role, token, message, arguments in examples:
            print('  %s: %s; %s; arguments=%r' % (role, token, message, arguments))
        print('Decision pending: this is conflict evidence, not argument-schema acceptance.')


FAULTS = (
    ('initial reserved token',
     'raise FErr("formula_rebinding", "reserved `%s` cannot be declared by %s" % (name, origin))',
     'raise FErr("formula_ambiguous_name", "reserved `%s` cannot be declared by %s" % (name, origin))'),
    ('let reserved token',
     'raise FErr("formula_rebinding", "reserved `%s` cannot be rebound" % name)',
     'raise FErr("formula_ambiguous_name", "reserved `%s` cannot be rebound" % name)'),
    ('invented initial statement index',
     'raise FErr("formula_rebinding", "reserved `%s` cannot be declared by %s" % (name, origin))',
     'raise FErr("formula_rebinding", "reserved `%s` cannot be declared by %s" % (name, origin), {"statement_index": 0})'),
)


if __name__ == '__main__':
    original = SOURCE.read_bytes()
    review()
    if '--mutations' in sys.argv:
        for label, before, after in FAULTS:
            try:
                review((before, after), False)
            except AssertionError as error:
                assert any(marker in str(error) for marker in
                           ('reserved refusal token', 'current reference arguments changed')), (label, error)
                print('  actual compiled reproduction assertion red:', label)
            else:
                raise AssertionError(('reproduction fault escaped', label))
        assert SOURCE.read_bytes() == original, 'producer bytes changed'
        print('D131 reproduction controls: 3 actual assertion reds; producer unchanged')
