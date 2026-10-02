"""D125: actual reference assertions, independent thresholds/payloads and consumer refusals."""
from fractions import Fraction
from pathlib import Path
import os
import runpy
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
HELPER = runpy.run_path(str(ROOT / 'docs/tasks/artifacts/formula_structure/static_signature_contract.py'))
SOURCE, BOOK = HELPER['SOURCE'], ROOT / 'docs/book/src'
WORK = ROOT / 'target/formula_structure/assertion_refusal'
KINDS = ('length', 'angle', 'area', 'ratio', 'count')
CLASSES = {'eps_num': 1, 'eps_geo': 10, 'eps_fmt': 25, 'eps_imp': 40, 'eps_phys': 100}


def contracts(replacement=None, verbose=True):
    ns, reference = HELPER['load_reference'](replacement)
    assert set(name for name in reference.reserved if name.startswith('eps_')) == set(CLASSES)
    assert reference.reserved['eps_num'][2] == 1 and reference.reserved['eps_geo'][2] == 10
    checks = 0

    def env_for(kind, left, right):
        assert Fraction(left).denominator == Fraction(right).denominator == 1
        return {'a': {'kind': kind, 'origin': 'recipe', 'state': 'derived', 'value': int(left)},
                'b': {'kind': kind, 'origin': 'recipe', 'state': 'derived', 'value': int(right)}}

    for tolerance, bound in CLASSES.items():
        reference.reserved[tolerance] = ('length', True, Fraction(bound))
        for kind in KINDS:
            for delta in (0, Fraction(bound, 2), bound, Fraction(2 * bound + 1, 2), bound + 1):
                for reverse in (False, True):
                    left, right = Fraction(200), Fraction(200) + delta
                    if reverse:
                        left, right = right, left
                    # Supplied bindings are canonical integers; half-unit values arise through
                    # exact expression arithmetic, including fractional Count results.
                    env = env_for(kind, 2 * left, 2 * right)
                    before = {name: dict(entry) for name, entry in env.items()}
                    source = 'assert closure: %s = a / 2.0 == b / 2.0' % tolerance
                    # False numerically still passes static checking without computing a verdict.
                    assert reference.static_statement(source, env)[:3] == ('assert', 'closure', tolerance)
                    result = sentinel = object()
                    try:
                        result = reference.statement(source, env)
                    except ns['FErr'] as error:
                        assert delta > bound and error.token == 'formula_assertion', ('assertion token', delta, bound, error)
                        expected = {'assertion_name': 'closure', 'left_kind': kind, 'left_value': left,
                                    'right_kind': kind, 'right_value': right, 'tolerance_class': tolerance,
                                    'tolerance_value': Fraction(bound)}
                        assert error.arguments == expected, ('assertion arguments', expected, error.arguments)
                        assert result is sentinel and 'closure' in error.msg and tolerance in error.msg
                    else:
                        assert delta <= bound and len(result) == 5 and result[:3] == ('assert', 'closure', True), (
                            'assertion accepted false or changed true tuple', delta, bound, result)
                        assert (result[3].kind, result[3].v, result[4].kind, result[4].v) == (kind, left, kind, right), (
                            'assertion successful values', kind, tolerance, left, right, result[3], result[4])
                    assert env == before, 'assertion mutated caller bindings'
                    checks += 1
    for kind in KINDS[:4]:
        env = env_for(kind, Fraction(-3), Fraction(2))
        try:
            reference.statement('assert signed_closure: eps_num = a == b', env)
        except ns['FErr'] as error:
            assert error.token == 'formula_assertion' and error.arguments['left_value'] == -3 and error.arguments['right_value'] == 2
        else:
            raise AssertionError('assertion accepted signed false')
        checks += 1
    # Original defect and equal-value control, with default actual chapter classes.
    ns, original = HELPER['load_reference'](replacement)
    for source, wanted in [('assert false_closure: eps_num = 1 cm == 2 cm', 'formula_assertion'),
                           ('assert true_closure: eps_num = 1 cm == 1 cm', None),
                           ('assert missing: eps_phys = 1 cm == 2 cm', 'formula_tolerance_unbound'),
                           ('assert divisor: eps_num = 1 cm / 0 == 2 cm', 'formula_division'),
                           ('assert unknown: eps_num = a == 2 cm', 'formula_unknown')]:
        env = {'a': {'kind': 'length', 'origin': 'measurement', 'state': 'unknown', 'value': None}}
        try:
            result = original.statement(source, env)
        except ns['FErr'] as error:
            assert error.token == wanted, ('assertion earlier error', source, wanted, error.token)
        else:
            assert wanted is None and result[:3] == ('assert', 'true_closure', True), ('assertion original defect accepted', source)
        checks += 1
    # Static refusal precedes every runtime callback, including tolerance reads.
    for method in ('evaluate', '_evaluate', 'value_of_name', 'call', 'resolve_geometry', 'stored'):
        def trapped(*args, **kwargs):
            raise AssertionError('assertion static error invoked runtime')
        setattr(original, method, trapped)
    for source, token in [('assert invalid: size_index = 1 cm == 1 cm', 'formula_parse'),
                          ('assert invalid: eps_num = 1 cm == 1 deg', 'formula_dimension'),
                          ('assert invalid: eps_num = missing == 1 cm', 'formula_unbound_name')]:
        try:
            original.statement(source, {})
        except ns['FErr'] as error:
            assert error.token == token, ('assertion static precedence', error.token, token)
        else:
            raise AssertionError('assertion static defect accepted')
        checks += 1
    if verbose:
        print('assertion contract: %d cases / five arithmetic kinds and classes / inclusive threshold / exact failure arguments; caller unchanged' % checks)
    return checks


def consumer():
    WORK.mkdir(parents=True, exist_ok=True)
    book = WORK / 'false_closure' / 'src'
    if book.exists():
        shutil.rmtree(book)
    shutil.copytree(BOOK, book)
    examples = book / 'spec/formula-language/examples.md'
    text = examples.read_text()
    before = '== 2 * wb_width`'
    assert text.count(before) == 1, 'actual consumer assertion mutation anchor changed'
    examples.write_text(text.replace(before, '== wb_width`'))
    result = subprocess.run(['bash', str(SOURCE)], cwd=ROOT, env=dict(os.environ, FORMULA_BOOK=str(book)),
                            capture_output=True, text=True)
    output = result.stdout + result.stderr
    (WORK / 'false_closure.log').write_text(output)
    assert result.returncode == 1 and 'formula_assertion' in output and 'waistband_width_closure' in output, (
        'assertion consumer token', result.returncode, output)
    assert 'length:80000 != length:40000' in output and '`eps_num` (1)' in output, ('assertion consumer values/class', output)
    print('assertion consumer: actual copied-book false closure raises formula_assertion with name/values/class, rc=1')


FAULTS = (
    ('false guard bypass', 'if abs(a.v - b.v) > tol.v:', 'if False:'),
    ('inclusive boundary lost', 'if abs(a.v - b.v) > tol.v:', 'if abs(a.v - b.v) >= tol.v:'),
    ('false token changed', 'raise FErr("formula_assertion", "assert `%s`', 'raise FErr("formula_domain", "assert `%s`'),
    ('label payload lost', '"assertion_name": name, "left_kind":', '"assertion_name": "other", "left_kind":'),
    ('operand order lost', '"left_value": a.v,', '"left_value": b.v,'),
    ('operand kind lost', '"right_kind": b.kind,', '"right_kind": "length",'),
    ('class payload lost', '"tolerance_class": tol_name,', '"tolerance_class": "eps_num",'),
    ('tolerance value lost', '"tolerance_value": tol.v}', '"tolerance_value": 0}'),
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
                assert any(marker in str(error) for marker in (
                    'assertion token', 'assertion arguments', 'assertion accepted false or changed true tuple')), (
                    name, 'not a body assertion red', error)
                print('  actual compiled assertion refusal red:', name)
            else:
                raise AssertionError(('assertion fault escaped', name))
        assert SOURCE.read_bytes() == original
        print('assertion faults: %d actual body assertion reds; producer unchanged' % len(FAULTS))
