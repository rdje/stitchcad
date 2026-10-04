"""D139: independently authored binding-header schemas, positions and phase refusals."""
from pathlib import Path
import runpy
import sys

ROOT = Path(__file__).resolve().parents[4]
ORACLE = runpy.run_path(str(ROOT / 'docs/tasks/artifacts/formula_structure/static_namespace_contract.py'))
SOURCE = ORACLE['SOURCE']
KINDS, BINDABLE = ORACLE['KINDS'], ORACLE['BINDABLE']


def contracts(replacement=None, verbose=True):
    ns, reference = ORACLE['LOADER']['load_reference'](replacement)
    declaration = ORACLE['Declaration']
    reference.reserved = {name: ORACLE['ReservedKind'](kind) for name, kind in ORACLE['RESERVED'].items()}
    for method in ('evaluate', '_evaluate', 'value_of_name', 'resolve_geometry', 'call', 'stored'):
        def trapped(*args, **kwargs):
            raise AssertionError('D139 execution/value/provider query')
        setattr(reference, method, trapped)
    env = {'v_' + kind: declaration(kind, 'parameter') for kind in KINDS}
    before = dict(env)
    cases = payloads = 0

    def refused(action, token, expected=None):
        nonlocal cases, payloads
        try:
            action()
        except ns['FErr'] as error:
            assert error.token == token, ('D139 token/priority', token, error.token, error.msg)
            if expected is not None:
                assert error.arguments == expected, ('D139 complete payload', error.arguments, expected)
                payloads += 1
        else:
            raise AssertionError(('D139 accepted refusal', token, expected))
        assert env == before, 'D139 caller namespace changed'
        cases += 1

    def expected(source, annotation, actual=None, ordinal=None):
        # Independent source oracle: name/colon header then literal annotation spelling.
        start = source.index(annotation, source.index('saved:') + len('saved:'))
        args = {'diagnostic_scope': 'binding_annotation' if actual is None else 'binding_kind',
                'operation': 'let', 'name': 'saved', 'annotation_span': (start, start + len(annotation))}
        if actual is None:
            args.update(raw_annotation=annotation, wanted_kinds=BINDABLE)
        else:
            args.update(declared_kind=annotation, expression_kind=actual, wanted_kinds=(annotation,))
        if ordinal is not None:
            args['statement_index'] = ordinal
        return args

    for declared in BINDABLE:
        for actual in KINDS:
            source = ' \tlet saved: \t%s = v_%s\n' % (declared, actual)
            whole = ' \nlet earlier:length=1 mm\n' + source
            if declared == actual:
                result = reference.static_statement(source, env)
                assert result[:3] == ('let', 'saved', declared), 'D139 accepted header identity'
                plan = reference.preflight(whole, env.items())
                assert len(plan) == 2 and plan[1][2][:3] == result[:3], 'D139 complete plan'
                assert env == before, 'D139 accepted namespace changed'
                cases += 2
            else:
                refused(lambda: reference.static_statement(source, env), 'formula_dimension', expected(source, declared, actual))
                refused(lambda: reference.preflight(whole, env.items()), 'formula_dimension', expected(whole, declared, actual, 2))

    # Invalid raw annotation: no expression kind is available. It precedes RHS grammar and lookup.
    for annotation in ('point', 'edge', 'unlisted', 'lengths', 'length_point', 'eps_num'):
        for rhs in ('1 mm', '1.0', 'missing', 'bogus(missing)', '(', '1 +'):
            source = '\n\tlet saved:  %s = %s' % (annotation, rhs)
            want = expected(source, annotation)
            for adapter in (reference.syntax_statement,
                            lambda s: reference.static_statement(s, env),
                            lambda s: reference.statement(s, env)):
                refused(lambda: adapter(source), 'formula_dimension', want)
            whole = 'let earlier:length=1 mm\n' + source
            refused(lambda: reference.preflight(whole, env.items()), 'formula_dimension', expected(whole, annotation, ordinal=2))

    # No RHS parse/conversion may occur after an invalid annotation, even with a valid RHS.
    parse = reference.parse
    def parse_trap(*args, **kwargs):
        raise AssertionError('D139 invalid annotation parsed RHS')
    reference.parse = parse_trap
    try:
        for annotation in ('point', 'edge', 'unlisted'):
            source = 'let saved:%s=1 mm' % annotation
            refused(lambda: reference.static_statement(source, env), 'formula_dimension', expected(source, annotation))
    finally:
        reference.parse = parse

    # Whole-source lexical preflight precedes headers; it must never be labeled a typed header.
    for rhs in ('é', 'Upper', '1mm', '1\tmm', '@'):
        source = 'let saved:point=' + rhs
        refused(lambda: reference.static_statement(source, env), 'formula_parse')
        refused(lambda: reference.preflight(source, env.items()), 'formula_parse')
    # Valid annotation: children and collision checks keep their existing diagnostics.
    for rhs, token in [('missing', 'formula_unbound_name'), ('bogus(v_ratio)', 'formula_unbound_name'),
                       ('nurbs(v_ratio)', 'env_nurbs'), ('v_length+v_ratio', 'formula_dimension'),
                       ('if(v_count,v_length,v_length)', 'formula_dimension'), ('(', 'formula_parse')]:
        source = 'let saved:boolean=' + rhs
        try:
            reference.static_statement(source, env)
        except ns['FErr'] as error:
            assert error.token == token, ('D139 child token', rhs, error)
            assert error.arguments.get('diagnostic_scope') not in ('binding_kind', 'binding_annotation'), 'D139 child mislabeled header'
        else:
            raise AssertionError(('D139 child accepted', rhs))
        cases += 1
    for source, token in [('let eps_num:boolean=v_length', 'formula_rebinding'),
                          ('let v_length:boolean=v_ratio', 'formula_ambiguous_name')]:
        refused(lambda: reference.static_statement(source, env), token)
    # Fully resolved runtime-invalid values retain static kind acceptance without execution.
    for rhs in ('1 mm/0.0', 'sqrt(-(1 mm)^2)'):
        assert reference.static_statement('let saved:length=' + rhs, env)[:3] == ('let', 'saved', 'length'), 'D139 static domain separation'
        cases += 1
    # Multiple earlier statements, annotation spelling in RHS/name, and genuine one-based ordinal.
    source = 'let point_length:length=1 mm\nassert ok:eps_num=1 mm==1 mm\n\tlet saved:length=1.0'
    refused(lambda: reference.preflight(source, env.items()), 'formula_dimension', expected(source, 'length', 'ratio', 3))
    assert env == before, 'D139 final caller namespace changed'
    if verbose:
        print('D139 header contract: %d cases / %d exact payloads; kinds, source positions, lexical/child priority and phase traps' % (cases, payloads))
    return cases, payloads


FAULTS = [
    ('missing header arguments', 'return FErr("formula_dimension", message, arguments)', 'return FErr("formula_dimension", message)'),
    ('wrong scope', '"binding_annotation" if expression_kind is None else "binding_kind"', '"binding_kind"'),
    ('invented raw declared kind', 'arguments.update(raw_annotation=annotation, wanted_kinds=tuple(self.bindable))', 'arguments.update(raw_annotation=annotation, declared_kind=annotation, wanted_kinds=tuple(self.bindable))'),
    ('omitted wanted alternative', 'wanted_kinds=tuple(self.bindable)', 'wanted_kinds=tuple(self.bindable[:-1])'),
    ('wrong actual RHS kind', 'expression_kind=expression_kind,', 'expression_kind=annotation,'),
    ('wrong wanted declared kind', 'wanted_kinds=(annotation,)', 'wanted_kinds=(expression_kind,)'),
    ('wrong original span', 'start = starts[3]', 'start = starts[1]'),
    ('lost raw whole offset', 'tuple(offset + index for index in arguments["annotation_span"])', 'tuple(index for index in arguments["annotation_span"])'),
    ('lost valid whole offset', 'self._binding_dimension(src, name, kind, got, ordinal, offset)', 'self._binding_dimension(src, name, kind, got, ordinal, 0)'),
    ('invented detached index', 'arguments["statement_index"] = ordinal\n        return FErr', 'arguments["statement_index"] = ordinal\n        else:\n            arguments["statement_index"] = 1\n        return FErr'),
    ('lost raw recipe index', 'arguments["statement_index"] = ordinal\n            raise FErr(error.token', 'arguments["statement_index"] = 1\n            raise FErr(error.token'),
    ('invalid annotation parses RHS', 'raise self._binding_dimension(src, name, kind)', 'self.parse("1 mm")\n                raise self._binding_dimension(src, name, kind)'),
    ('header execution', 'return FErr("formula_dimension", message, arguments)', 'self.evaluate(self.parse("1 mm"), {})\n        return FErr("formula_dimension", message, arguments)'),
]

if __name__ == '__main__':
    original = SOURCE.read_bytes()
    contracts()
    if '--mutations' in sys.argv:
        for name, before, after in FAULTS:
            try:
                contracts((before, after), False)
            except AssertionError as error:
                assert 'D139' in str(error), (name, 'not a contract body assertion', error)
                print('  actual compiled D139 body assertion red:', name)
            else:
                raise AssertionError(('D139 actual fault escaped', name))
        assert SOURCE.read_bytes() == original, 'D139 source changed on disk'
        print('D139 header faults: %d actual compiled body assertion reds; source unchanged' % len(FAULTS))
