"""D148: complete source input phase before ordered static checking; actual reference."""
from pathlib import Path
import runpy
import sys

ROOT = Path(__file__).resolve().parents[4]
ORACLE = runpy.run_path(str(ROOT / 'docs/tasks/artifacts/formula_structure/static_namespace_contract.py'))
SOURCE = ORACLE['SOURCE']


def contracts(replacement=None, verbose=True):
    ns, reference = ORACLE['LOADER']['load_reference'](replacement)
    declaration = ORACLE['Declaration']
    env = [('width', declaration('length', 'measurement')),
           ('flag', declaration('boolean', 'parameter'))]
    before = list(env)
    reference.reserved = {name: ORACLE['ReservedKind'](kind) for name, kind in ORACLE['RESERVED'].items()}
    for method in ('evaluate', '_evaluate', 'value_of_name', 'resolve_geometry', 'call', 'stored'):
        def trapped(*args, **kwargs):
            raise AssertionError('D148 numerical/provider/state query')
        setattr(reference, method, trapped)
    cases = 0

    def refused(source, token, raw=False):
        nonlocal cases
        sentinel = result = object()
        try:
            result = reference.preflight(source, iter(env))
        except ns['FErr'] as error:
            assert error.token == token, ('D148 phase token', token, error.token, error.msg)
            if raw:
                start = source.index('point', source.index('let late:'))
                ordinal = source.count('let ') + source.count('assert ')
                want = {'diagnostic_scope': 'binding_annotation', 'operation': 'let', 'name': 'late',
                        'raw_annotation': 'point', 'annotation_span': (start, start + 5),
                        'wanted_kinds': ('length', 'angle', 'area', 'ratio', 'count', 'boolean'),
                        'statement_index': ordinal}
                assert error.arguments == want, ('D148 raw source arguments', error.arguments, want)
        else:
            raise AssertionError(('D148 invalid source accepted', source[:90]))
        assert result is sentinel and env == before, 'D148 partial plan/caller mutation'
        cases += 1

    early = (
        'let bad:length=missing', 'let bad:length=1.0',
        'let bad:length=if(flag,width,missing)', 'let bad:length=unknown(missing)',
        'let bad:length=nurbs(missing)', 'let width:length=missing',
        'let eps_num:length=missing', 'assert bad:eps_num=flag==flag',
    )
    deep = '1 mm'
    for _ in range(17):
        deep = 'if(flag,1 mm,' + deep + ')'
    late = (
        ('assert late:eps_chord=1 mm==1 mm', 'formula_parse', False),
        ('let late:length=(', 'formula_parse', False),
        ('let late:length=1+', 'formula_parse', False),
        ('let late:length=unknown(1,)', 'formula_parse', False),
        ('assert late:eps_geo=1==1==1', 'formula_parse', False),
        ('let late:point=missing', 'formula_dimension', True),
        ('let late:length=' + '-' * 256 + '1 mm', 'formula_domain', False),
        ('let late:length=' + deep, 'formula_domain', False),
        ('let late:count=' + str(1 << 128), 'formula_domain', False),
    )
    for first in early:
        for tail, token, raw in late:
            for separator in (' ', '\n', '\r\n\t', '\f\v'):
                for prefix in ('', ' \tassert prior:eps_num=1==1\n'):
                    refused(prefix + first + separator + tail, token, raw)

    # Whole-source lexical validation stays before all headers, even invalid annotations.
    for suffix in ('é', '@', 'Upper', '1mm'):
        refused('let bad:point=missing\nlet late:length=' + suffix, 'formula_parse')
    # First-phase errors retain statement order; they do not claim an inferred operand kind.
    source = 'let late:point=missing\nassert after:eps_chord=1==1'
    try:
        reference.preflight(source, env)
    except ns['FErr'] as error:
        assert error.token == 'formula_dimension', 'D148 first-phase source order'
        assert error.arguments['statement_index'] == 1, 'D148 first-phase actual ordinal'
    else:
        raise AssertionError('D148 first annotation accepted')
    cases += 1

    # The production statement ceiling is reached before any earlier static inference.
    middle = '\n'.join('assert c%d:eps_num=1==1' % i for i in range(4095))
    refused('let bad:length=missing\n' + middle + '\nlet excess:count=1', 'formula_domain')

    # Trace the real phase calls: parse every statement exactly once, then check in order.
    # Retain the actual parsed tuple object, rather than reconstructing a replacement AST.
    syntax, static, infer, recipe_source = (reference.syntax_statement, reference._static_statement,
                                          reference.infer, reference._recipe_source)
    parsed, static_ordinals = [], []
    chunks = ['let first:length=width/0\n', 'assert check:eps_phys=first==width\n',
              'let second:length=if(flag,first,first)\n', 'let last:length=second']
    def syntax_trace(source):
        result = syntax(source)
        parsed.append((source, result))
        return result
    def static_trace(source, metadata, ordinal=None, offset=0, prior_sources=None, **kwargs):
        assert len(parsed) == len(chunks), 'D148 static began before complete input phase'
        static_ordinals.append(ordinal)
        return static(source, metadata, ordinal, offset, prior_sources, **kwargs)
    def infer_trace(*args, **kwargs):
        assert len(parsed) == len(chunks), 'D148 inference before complete input phase'
        return infer(*args, **kwargs)
    def source_trace(*args, **kwargs):
        assert len(parsed) == len(chunks), 'D148 prior binding before complete input phase'
        return recipe_source(*args, **kwargs)
    reference.syntax_statement, reference._static_statement = syntax_trace, static_trace
    reference.infer, reference._recipe_source = infer_trace, source_trace
    try:
        try:
            plan = reference.preflight(' \t' + ''.join(chunks), env)
        except ns['FErr'] as error:
            raise AssertionError(('D148 valid ordered source refused', error.token, error.msg)) from error
        assert [source for source, _ in parsed] == chunks, 'D148 source parsed twice/changed'
        assert static_ordinals == [1, 2, 3, 4], 'D148 ordered static ordinals'
        assert len(plan) == len(chunks), 'D148 incomplete accepted plan'
        cursor = 2
        for entry, chunk, (_, original) in zip(plan, chunks, parsed):
            assert entry[:2] == (cursor, cursor + len(chunk)), 'D148 actual original spans'
            assert entry[2] is original, 'D148 parsed operand owner replaced'
            cursor += len(chunk)
        assert env == before, 'D148 accepted caller namespace changed'
        cases += 1
    finally:
        reference.syntax_statement, reference._static_statement = syntax, static
        reference.infer, reference._recipe_source = infer, recipe_source
    # With complete input, existing static priority and real binding mismatch arguments survive.
    source = 'assert prior:eps_fmt=1==1\nlet late:length=1.0'
    try:
        reference.preflight(source, env)
    except ns['FErr'] as error:
        assert error.token == 'formula_dimension', 'D148 static mismatch token'
        start = source.index('length')
        assert error.arguments == {'diagnostic_scope': 'binding_kind', 'operation': 'let', 'name': 'late',
                                  'declared_kind': 'length', 'expression_kind': 'ratio',
                                  'annotation_span': (start, start + 6), 'wanted_kinds': ('length',),
                                  'statement_index': 2}, 'D148 static actual source/ordinal'
    else:
        raise AssertionError('D148 static mismatch accepted')
    cases += 1
    if verbose:
        print('D148 whole phase: %d cases; later input before earlier static, parsed owners/spans/order, execution trapped' % cases)
    return cases


FAULTS = (
    ('interleaved static check', 'checked = self._syntax_statement_at(src[start:end], ordinal, start)',
     'checked = self._static_statement(src[start:end], env, ordinal, start)'),
    ('early inference', 'parsed.append((start, end, checked))', 'self.infer(checked[3], env)\n            parsed.append((start, end, checked))'),
    ('raw offset lost', 'self._syntax_statement_at(src[start:end], ordinal, start)', 'self._syntax_statement_at(src[start:end], ordinal, 0)'),
    ('raw ordinal lost', 'self._syntax_statement_at(src[start:end], ordinal, start)', 'self._syntax_statement_at(src[start:end], 1, start)'),
    ('prepared syntax discarded', 'prior_sources, checked=checked)', 'prior_sources)'),
    ('last syntax omitted', 'zip(boundaries, ends), 1', 'zip(boundaries[:-1], ends), 1'),
    ('static order reversed', 'enumerate(parsed, 1)', 'enumerate(reversed(parsed), 1)'),
    ('static ordinal changed', 'self._static_statement(src[start:end], env, ordinal, start, prior_sources, checked=checked)',
     'self._static_statement(src[start:end], env, 1, start, prior_sources, checked=checked)'),
    ('partial plan returned', 'plan.append((start, end, checked))', 'plan.append((start, end, checked))\n            return tuple(plan)'),
    ('input limit bypass', 'if ordinal > self.limits["max_recipe_statements"]:', 'if False:'),
    ('runtime execution', 'parsed.append((start, end, checked))', 'self.evaluate(checked[3], env)\n            parsed.append((start, end, checked))'),
)

if __name__ == '__main__':
    original = SOURCE.read_bytes()
    contracts()
    if '--mutations' in sys.argv:
        for name, before, after in FAULTS:
            try:
                contracts((before, after), False)
            except AssertionError as error:
                assert 'D148' in str(error), (name, 'not a phase body assertion', error)
                print('  actual compiled D148 body assertion red:', name)
            else:
                raise AssertionError(('D148 fault escaped', name))
        assert SOURCE.read_bytes() == original, 'D148 on-disk source changed'
        print('D148 faults: %d actual compiled body assertion reds; source unchanged' % len(FAULTS))
