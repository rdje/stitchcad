"""D122/D127: independent origin/context read contracts against the actual reference."""
from copy import deepcopy
from fractions import Fraction
from pathlib import Path
import runpy
import os
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
HELPER = runpy.run_path(str(ROOT / 'docs/tasks/artifacts/formula_structure/static_signature_contract.py'))
SOURCE = HELPER['SOURCE']
ORIGINS = ('measurement', 'ease', 'parameter', 'profile', 'material', 'geometry', 'recipe', 'size', 'tolerance')
KINDS = ('length', 'angle', 'area', 'ratio', 'count', 'boolean', 'point', 'edge')
STATES = ('known', 'assumed', 'preference', 'derived', 'unknown')
TOKENS = dict.fromkeys(ORIGINS[:5], 'formula_unknown')
TOKENS.update(dict.fromkeys(('geometry', 'recipe', 'size'), 'formula_unbound_name'))
TOKENS['tolerance'] = 'formula_tolerance_unbound'
TOLERANCES = ('eps_num', 'eps_geo', 'eps_fmt', 'eps_imp', 'eps_phys')
RESERVED = {'eps_num': ('length', 1), 'eps_geo': ('length', 10), 'eps_fmt': ('length', 25),
            'eps_imp': ('length', 40), 'eps_phys': ('length', 100), 'size_index': ('count', 1),
            'size_count': ('count', 3), 'is_base_size': ('boolean', 0)}


def contracts(replacement=None, verbose=True):
    ns, ref = HELPER['load_reference'](replacement)
    assert ref.origins == frozenset(ORIGINS) and set(ref.reserved) == set(RESERVED)
    checks = 0

    def result(source, env, wanted=None, arguments=None):
        nonlocal checks
        try:
            value = ref.statement(source, env)
        except Exception as error:
            assert isinstance(error, ns['FErr']), ('origin read host error', source, type(error).__name__)
            assert wanted is not None and error.token == wanted, ('origin read token', source, wanted, error.token)
            assert arguments is None or error.arguments == arguments, ('origin read arguments', arguments, error.arguments)
            value = None
        else:
            assert wanted is None, ('origin missing read accepted', source, wanted, value)
        checks += 1
        return value

    def arguments(name, origin, state=None, context=None):
        data = {'name': name, 'origin': origin}
        if TOKENS[origin] == 'formula_unknown':
            data['state'] = state
        elif TOKENS[origin] == 'formula_unbound_name':
            data['origins_searched'] = tuple(sorted(ORIGINS))
        elif context is not None:
            data['context'] = context
        return data

    # Exact closed populations and independently specified outcomes, not the evaluator's dispatch.
    for origin in ORIGINS:
        for kind in KINDS:
            for state in STATES:
                env = {'input_value': {'kind': kind, 'origin': origin, 'state': state, 'value': None}}
                before = deepcopy(env)
                assert ref.static_statement('input_value', env)[:3] == ('expr', None, kind)
                result('input_value', env, TOKENS[origin], arguments('input_value', origin, state))
                assert env == before, 'origin missing read changed caller'
            for state in STATES[:4]:
                for zero in (False, True):
                    value = ((Fraction(0), Fraction(0)) if zero else (Fraction(2), Fraction(-3))) if kind == 'point' else Fraction(0 if zero else 1)
                    env = {'input_value': {'kind': kind, 'origin': origin, 'state': state, 'value': value}}
                    before = deepcopy(env)
                    actual = result('input_value', env)
                    assert (actual[3].kind, actual[3].v) == (kind, value), ('origin populated read changed', origin, kind, state, value, actual)
                    assert env == before, 'origin populated read changed caller'
    # Explicit states constrain populated records; historical computed fixtures omit state.
    for origin in ORIGINS:
        for kind in KINDS:
            value = (Fraction(1), Fraction(2)) if kind == 'point' else Fraction(1)
            for state in ('unknown', None, 'invalid', False, []):
                env = {'input_value': {'kind': kind, 'origin': origin, 'state': state, 'value': value}}
                before = deepcopy(env)
                result('input_value', env, 'formula_parse')
                assert env == before, 'origin invalid populated record changed caller'
            env = {'input_value': {'kind': kind, 'origin': origin, 'value': value}}
            actual = result('input_value', env)
            assert (actual[3].kind, actual[3].v) == (kind, value), 'origin no-state fixture changed'
    for kind in ('point', 'edge'):
        env = {'geo': {'kind': kind, 'origin': 'geometry', 'state': 'unknown', 'value': None, 'lazy': True}}
        before = deepcopy(env)
        saved = ref.resolve_geometry
        def forbidden(*args):
            raise AssertionError('origin unknown geometry resolved')
        ref.resolve_geometry = forbidden
        result('geo', env, 'formula_unbound_name', arguments('geo', 'geometry', 'unknown'))
        assert env == before, 'origin unknown geometry changed caller'
        ref.resolve_geometry = saved
    # Always-visible metadata never asserts the existence of an optional context value.
    for name, (kind, provided) in RESERVED.items():
        original = ref.reserved[name]
        if name not in ('eps_num', 'eps_geo'):
            origin = 'tolerance' if name in TOLERANCES else 'size'
            result(name, {}, TOKENS[origin], arguments(name, origin))
            for context in ('independent_recipe', 'independent_export', 'independent_profile'):
                ref.context = context
                result(name, {}, TOKENS[origin], arguments(name, origin, context=context))
                ref.context = None
            ref.reserved[name] = (kind, False, provided)
        actual = result(name, {})
        assert (actual[3].kind, actual[3].v) == (kind, Fraction(provided)), ('origin available reserved changed', name, actual)
        ref.reserved[name] = original
    ref.context = 'independent_export'
    result('missing_tol', {'missing_tol': {'kind': 'length', 'origin': 'tolerance', 'value': None}},
           'formula_tolerance_unbound', arguments('missing_tol', 'tolerance', context='independent_export'))
    ref.context = None
    result('undeclared', {}, 'formula_unbound_name', {'name': 'undeclared', 'origins_searched': tuple(sorted(ORIGINS))})
    # Presence, rather than truthiness: Boolean false in a supplied size context selects else.
    ref.reserved['is_base_size'] = ('boolean', False, 0)
    actual = result('let selected: count = if(is_base_size, 9, 2)', {})
    assert actual[3].v == 2, 'origin supplied false size flag became missing or true'
    for origin in ORIGINS:
        env = {'missing': {'kind': 'length', 'origin': origin, 'state': 'unknown', 'value': None}}
        actual = result('let selected: length = if(1 == 2, missing, 1 um)', env)
        assert actual[3].v == 1, 'origin untaken missing branch was read'
        result('let selected: length = if(1 == 1, missing, 1 um)', env,
               TOKENS[origin], arguments('missing', origin, 'unknown'))
    # Malformed namespace records fail by name before touching numerical metadata.
    for malformed in (None, 1, [], {'origin': 'measurement'}, {'kind': 'length'},
                      {'kind': None, 'origin': 'measurement'}, {'kind': [], 'origin': 'measurement'},
                      {'kind': 'length', 'origin': None}, {'kind': 'length', 'origin': []},
                      {'kind': 'length', 'origin': {}}, {'kind': 'invalid', 'origin': 'measurement'},
                      {'kind': 'length', 'origin': 'invalid'}):
        result('input_value', {'input_value': malformed}, 'formula_parse')
    for origin in ORIGINS[:5]:
        for state in (None, '', 'UNKNOWN', False, []):
            entry = {'kind': 'length', 'origin': origin, 'value': None}
            if state is not None:
                entry['state'] = state
            result('input_value', {'input_value': entry}, 'formula_parse')
    # Lazy geometry keeps existing resolution, caching and upstream-error behavior.
    for kind, expressions, expected in [('point', {'x': '1 um', 'y': '-2 um'}, (Fraction(1), Fraction(-2))),
                                        ('edge', {'len': '3 um'}, Fraction(3))]:
        env = {'geo': {'kind': kind, 'origin': 'geometry', 'state': 'derived', 'value': None,
                       'lazy': True, 'exprs': expressions}}
        actual = result('geo', env)
        assert actual[3].v == expected and env['geo']['value'] == expected and env['geo']['lazy'] is False
        def forbidden(*args):
            raise AssertionError('origin cached geometry resolved again')
        saved = ref.resolve_geometry
        ref.resolve_geometry = forbidden
        actual = result('geo', env)
        assert actual[3].v == expected
        ref.resolve_geometry = saved
        env['geo'].update(value=None, lazy=True, exprs={'x': 'missing', 'y': '1 um', 'len': 'missing'})
        env['missing'] = {'kind': 'length', 'origin': 'measurement', 'state': 'unknown', 'value': None}
        before = deepcopy(env)
        result('geo', env, 'formula_unknown', arguments('missing', 'measurement', 'unknown'))
        assert env == before, 'origin failed geometry published cache or lost lazy state'
    if verbose:
        print('origin value contract: %d cases / nine origins, eight kinds, five missing states / reserved supplied contexts / taken-only reads / malformed metadata / lazy geometry' % checks)
    return checks


def consumer():
    book_source = ROOT / 'docs/book/src'
    work = ROOT / 'target/formula_structure/origin_value_consumer'
    for label, expression, token, name in [
        ('missing_size', 'size_index * waist_girth', 'formula_unbound_name', 'size_index'),
        ('missing_tolerance', 'waist_girth + eps_fmt', 'formula_tolerance_unbound', 'eps_fmt'),
    ]:
        book = work / label / 'src'
        if book.exists():
            assert not (book / '.git').exists(), 'origin consumer scratch is not a Git repository'
            shutil.rmtree(book)
        book.parent.mkdir(parents=True, exist_ok=True)
        shutil.copytree(book_source, book)
        examples = book / 'spec/formula-language/examples.md'
        text = examples.read_text()
        before = '`waist_girth + ease_waist`'
        assert text.count(before) == 1, 'origin actual consumer anchor changed'
        examples.write_text(text.replace(before, '`' + expression + '`', 1))
        result = subprocess.run(['bash', str(SOURCE)], cwd=ROOT,
                                env=dict(os.environ, FORMULA_BOOK=str(book)), capture_output=True, text=True)
        output = result.stdout + result.stderr
        (book.parent / 'consumer.log').write_text(output)
        assert result.returncode == 1 and 'examples §2 `garment_waist`: ' + token in output and name in output, (
            'origin consumer refusal', label, result.returncode, output)
        assert 'statically accepted statements: 21' in output, 'origin consumer rejected known kind before runtime'
    print('origin consumer: two actual copied books preflight21, then refuse missing size/tolerance with distinct named tokens, rc=1')


FAULTS = (
    ('geometry misclassified', 'elif origin == "tolerance":', 'elif origin in ("tolerance", "geometry"):'),
    ('size reserved misclassified', 'else "size")', 'else "tolerance")'),
    ('provided optional value rejected', 'if value is None:', 'if not always:'),
    ('unknown token changed', 'token = "formula_unknown"', 'token = "formula_unbound_name"'),
    ('origin payload changed', 'arguments["origin"] = origin', 'arguments["origin"] = "measurement"'),
    ('state payload changed', 'arguments["state"] = state', 'arguments["state"] = "unknown"'),
    ('search payload lost', 'tuple(sorted(self.origins))', '()'),
    ('context payload lost', 'arguments["context"] = self.context', 'arguments["context"] = "other"'),
    ('namespace type guard removed', 'or not isinstance(origin, str)', 'or False'),
    ('populated invalid state accepted', 'if "state" in e and state not in ("known", "assumed", "preference", "derived", "unknown"):', 'if False:'),
    ('unknown populated value accepted', 'if state == "unknown" and e.get("value") is not None:', 'if False:'),
    ('unknown geometry resolved', 'if state == "unknown" and e.get("lazy"):', 'if False:'),
    ('missing state guard bypass', 'if state not in ("known", "assumed", "preference", "derived", "unknown"):', 'if False:'),
)
if __name__ == '__main__':
    original = SOURCE.read_bytes()
    contracts()
    consumer()
    if '--mutations' in sys.argv:
        for name, before, after in FAULTS:
            try:
                contracts((before, after), False)
            except AssertionError as error:
                assert any(marker in str(error) for marker in ('origin read token', 'origin read arguments', 'origin missing read accepted', 'origin read host error')), (name, 'not an actual body assertion red', error)
                print('  actual compiled origin refusal red:', name)
            else:
                raise AssertionError(('origin fault escaped', name))
        assert SOURCE.read_bytes() == original
        print('origin value faults: %d actual body assertion reds; source unchanged' % len(FAULTS))
