"""D147: independent ordered-source ambiguity arguments; actual reference, metadata only."""
from pathlib import Path
import runpy
import sys

ROOT = Path(__file__).resolve().parents[4]
ORACLE = runpy.run_path(str(ROOT / 'docs/tasks/artifacts/formula_structure/static_namespace_contract.py'))
SOURCE = ORACLE['SOURCE']
KINDS, ORIGINS, BINDABLE = ORACLE['KINDS'], ORACLE['ORIGINS'], ORACLE['BINDABLE']


def contracts(replacement=None, verbose=True):
    ns, reference = ORACLE['LOADER']['load_reference'](replacement)
    declaration = ORACLE['Declaration']
    reference.reserved = {name: ORACLE['ReservedKind'](kind) for name, kind in ORACLE['RESERVED'].items()}
    for method in ('evaluate', '_evaluate', 'value_of_name', 'resolve_geometry', 'call', 'stored'):
        def trapped(*args, **kwargs):
            raise AssertionError('D147 numerical/provider/state query')
        setattr(reference, method, trapped)
    cases = 0

    def initial(kind, origin, index=None):
        source = {'role': 'initial_declaration', 'kind': kind, 'origin': origin}
        if index is not None:
            source['declaration_index'] = index
        return source

    def expected(prior, attempt):
        return {'name': 'collision', 'origins': (prior['origin'], attempt['origin']),
                'prior_source': prior, 'attempted_source': attempt}

    def refused(action, want):
        nonlocal cases
        sentinel = result = object()
        try:
            result = action()
        except ns['FErr'] as error:
            assert error.token == 'formula_ambiguous_name', ('D147 token', error.token)
            assert error.arguments == want, ('D147 exact arguments', error.arguments, want)
        else:
            raise AssertionError(('D147 collision accepted', want))
        assert result is sentinel, 'D147 partial namespace/plan escaped'
        cases += 1

    # Both orders, equal origins, every kind, and positions interrupted by other declarations.
    for first in ORIGINS:
        for second in ORIGINS:
            for index, kind in enumerate(KINDS):
                other = KINDS[(index + 1) % len(KINDS)]
                for prior_index, attempt_index in ((1, 2), (2, 4)):
                    entries = [('input_%d' % i, declaration('length', 'measurement'))
                               for i in range(1, attempt_index + 1)]
                    entries[prior_index - 1] = ('collision', declaration(kind, first))
                    entries[attempt_index - 1] = ('collision', declaration(other, second))
                    before = list(entries)
                    want = expected(initial(kind, first, prior_index), initial(other, second, attempt_index))
                    refused(lambda: reference.namespace(iter(entries)), want)
                    refused(lambda: reference.preflight('', iter(entries)), want)
                    assert entries == before, 'D147 authored declarations changed'
    # Full independent kind cross-product, rather than relying on a single paired permutation.
    for first in KINDS:
        for second in KINDS:
            entries = [('collision', declaration(first, 'measurement')),
                       ('collision', declaration(second, 'parameter'))]
            refused(lambda: reference.namespace(entries),
                    expected(initial(first, 'measurement', 1), initial(second, 'parameter', 2)))

    # Dictionary adapters have metadata, not original declaration positions. Recipe attempts
    # have actual spans; only complete ordered source supplies their genuine statement ordinal.
    for origin in ORIGINS:
        if origin == 'recipe':
            continue  # Repeated recipe bindings retain their separate formula_rebinding contract.
        for prior_kind in KINDS:
            for attempted_kind in BINDABLE:
                source = '\n\tlet collision: %s = missing ' % attempted_kind
                name_start = source.index('collision')
                attempt = {'role': 'recipe', 'kind': attempted_kind, 'origin': 'recipe',
                           'span': (0, len(source)), 'name_span': (name_start, name_start + 9)}
                env = {'collision': declaration(prior_kind, origin)}
                before = dict(env)
                want = expected(initial(prior_kind, origin), attempt)
                for adapter in (reference.static_statement, reference.statement):
                    refused(lambda: adapter(source, env), want)
                    assert env == before, 'D147 detached namespace changed'
                for prefix, ordinal in [(' \nassert check:eps_num=1==1\n', 2),
                                        ('let earlier:length=1 mm\nassert check:eps_fmt=earlier==earlier\r\n', 3)]:
                    tail = source.lstrip()
                    whole = prefix + tail
                    start = len(prefix)
                    whole_attempt = {'role': 'recipe', 'kind': attempted_kind, 'origin': 'recipe',
                                     'span': (start, len(whole)), 'name_span': (start + 4, start + 13),
                                     'statement_index': ordinal}
                    refused(lambda: reference.preflight(whole, env.items()),
                            expected(initial(prior_kind, origin), whole_attempt))
                    assert env == before, 'D147 whole namespace changed'
    # A shared valid declaration is retained by identity, without extracting any value.
    entry = declaration('point', 'geometry')
    assert reference.namespace([('unique', entry)])['unique'] is entry, 'D147 declaration identity lost'
    if verbose:
        print('D147 ambiguity contract: %d exact payloads; ordered origins/sources, genuine positions/spans, execution trapped' % cases)
    return cases


FAULTS = (
    ('name lost', 'return {"name": name, "origins":', 'return {"name": "wrong", "origins":'),
    ('origins reversed', '(prior["origin"], attempted["origin"])', '(attempted["origin"], prior["origin"])'),
    ('duplicate origin lost', '(prior["origin"], attempted["origin"])', 'tuple(dict.fromkeys((prior["origin"], attempted["origin"])))'),
    ('prior source lost', '"prior_source": dict(prior), "attempted_source": dict(attempted)', '"prior_source": {}, "attempted_source": dict(attempted)'),
    ('attempted source swapped', '"prior_source": dict(prior), "attempted_source": dict(attempted)', '"prior_source": dict(prior), "attempted_source": dict(prior)'),
    ('initial kind changed', 'source = {"role": "initial_declaration", "kind": kind, "origin": origin}', 'source = {"role": "initial_declaration", "kind": "length", "origin": origin}'),
    ('initial role changed', 'source = {"role": "initial_declaration", "kind": kind, "origin": origin}', 'source = {"role": "recipe", "kind": kind, "origin": origin}'),
    ('initial position changed', 'source["declaration_index"] = declaration_index', 'source["declaration_index"] = declaration_index + 1'),
    ('detached position invented', 'if declaration_index is not None:', 'if True:'),
    ('initial recipe ordinal invented', 'source["declaration_index"] = declaration_index', 'source["declaration_index"] = declaration_index\n            source["statement_index"] = declaration_index'),
    ('prior pair overwritten', 'sources[name] = self._initial_source(kind, origin, declaration_index)', 'sources[name] = self._initial_source(kind, origin, declaration_index + 1)'),
    ('whole ordinal changed', 'source["statement_index"] = ordinal', 'source["statement_index"] = ordinal + 1'),
    ('global span lost', '"span": (offset, offset + len(src))', '"span": (0, len(src))'),
    ('name span changed', 'name_start + len(name)', 'name_start + len(name) + 1'),
    ('input metadata observed', 'kind, origin = entry["kind"], entry["origin"]', 'kind, origin = entry["kind"], entry["origin"]\n                entry["value"]'),
    ('runtime query', 'return {"name": name, "origins":', 'self.evaluate(("name", name), {})\n        return {"name": name, "origins":'),
)

if __name__ == '__main__':
    original = SOURCE.read_bytes()
    contracts()
    if '--mutations' in sys.argv:
        for name, before, after in FAULTS:
            try:
                contracts((before, after), False)
            except AssertionError as error:
                assert any(marker in str(error) for marker in ('D147', 'static namespace read value/state/geometry')), (name, 'unrelated failure', error)
                print('  actual compiled D147 body assertion red:', name)
            else:
                raise AssertionError(('D147 fault escaped', name))
        assert SOURCE.read_bytes() == original, 'D147 on-disk source changed'
        print('D147 faults: %d actual compiled body assertion reds; source unchanged' % len(FAULTS))
