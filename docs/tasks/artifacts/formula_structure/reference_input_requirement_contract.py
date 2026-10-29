"""D338 focused reference-port controls: requirements are not canonical language diagnostics."""
from copy import deepcopy
from fractions import Fraction
from pathlib import Path
import runpy

ROOT = Path(__file__).resolve().parents[4]
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
HELPER = ROOT / 'docs/tasks/artifacts/formula_structure/static_signature_contract.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
PROFILE['plan_producer'](ROOT, sources=(Path(__file__), PROFILE_SOURCE, HELPER))
CONTRACT = runpy.run_path(str(HELPER))
PROFILE['plan_producer'](ROOT, sources=(Path(__file__), *CONTRACT['REFERENCE_INPUTS']))


def contracts():
    namespace, reference = CONTRACT['load_reference']()
    checks = 0
    for state, requirement in [('derived', 'derived_evaluation'), ('known', 'authored_value'),
                               ('assumed', 'authored_value'), ('preference', 'authored_value')]:
        for context in (None, '', 'actual_fixture_profile'):
            reference.context = context
            env = {'input_value': dict(kind='length', origin='parameter', state=state, value=None,
                                       lazy=True)}
            before = deepcopy(env)
            def forbidden(*args):
                raise AssertionError('external missing input entered geometry fallback')
            original = reference.resolve_geometry
            reference.resolve_geometry = forbidden
            try:
                reference.statement('input_value', env)
            except namespace['ReferenceInputRequired'] as error:
                wanted = dict(name='input_value', origin='parameter', state=state, requirement=requirement)
                if context is not None:
                    wanted['context'] = context
                assert error.arguments == wanted and error.context == context, (wanted, error.arguments)
                assert not isinstance(error, namespace['FErr']) and not hasattr(error, 'token')
                assert not hasattr(error, 'formula') and not hasattr(error, 'value')
            else:
                raise AssertionError(('missing actual input context was accepted', state, context))
            finally:
                reference.resolve_geometry = original
            assert env == before, 'reference requirement changed input/cached a result'
            checks += 1
    reference.context = None
    env = {'input_value': dict(kind='length', origin='parameter', state='unknown', value=None)}
    try:
        reference.statement('input_value', env)
    except namespace['FErr'] as error:
        assert error.token == 'formula_unknown' and error.arguments == dict(
            name='input_value', origin='parameter', state='unknown')
    else:
        raise AssertionError('actual Unknown acquired a value')
    checks += 1

    # These available Derived bindings come only from actual returned statement results. Zero,
    # false and original approximation contributions remain available, not missing by truthiness.
    env = {}
    for source in ('let a:ratio=sin(90 deg)', 'let zero:ratio=a-a', 'let flag:boolean=zero>0.0'):
        returned = reference.statement(source, env)
        name, value = returned[1], returned[3]
        env[name] = dict(kind=value.kind, origin='recipe', state='derived', value=value.v,
                         sources=value.sources)
        actual = reference.statement(name, env)[3]
        assert (actual.kind, actual.v, actual.sources) == (value.kind, value.v, value.sources)
        assert actual.sources == frozenset(('sin',)), (source, actual.sources)
        checks += 1
    assert env['zero']['value'] == Fraction(0) and env['flag']['value'] == Fraction(0)
    return checks


if __name__ == '__main__':
    print('reference input requirements: %d focused controls;no canonical token/invented identity/value' % contracts())
