"""D138: actual resolved kinds, symbolic roles and complete wanted rules in reference refusals."""
from itertools import product
from pathlib import Path
import runpy
import sys

ROOT = Path(__file__).resolve().parents[4]
ORACLE = runpy.run_path(str(ROOT / 'docs/tasks/artifacts/formula_structure/static_signature_contract.py'))
SOURCE = ORACLE['SOURCE']
KINDS, ARITH = ORACLE['KINDS'], ORACLE['ARITH']
TOLERANCES, RESERVED = ORACLE['TOLERANCES'], ORACLE['RESERVED']
CALL_ROWS = ORACLE['SIGNATURES']
UNARY = {'-': [(('N',), False, 'N')], '^2': [(('length',), False, 'area'),
           (('ratio',), False, 'ratio'), (('count',), False, 'count')]}
BINARY = {op: [(('T', 'T'), False, 'T' if op in ('+', '-') else 'boolean')]
          for op in ('+', '-', '==', '!=', '<', '<=', '>', '>=')}
BINARY['*'] = [(args, False, result) for args, result in ORACLE['MULTIPLICATION'].items()]
BINARY['/'] = [(args, False, result) for args, result in ORACLE['QUOTIENT'].items()]


def expanded(rows):
    """Expand independently authored symbolic rows into a finite exact-kind relation."""
    relation = {}
    for args, variadic, result in rows:
        if variadic:
            continue  # Variadic tests explicitly author one shared arithmetic kind below.
        for kind in ARITH:
            actual = tuple(kind if arg == 'T' else 'length' if arg == 'tolerance' else arg for arg in args)
            relation[actual] = kind if result == 'T' else result
    return relation


def signatures(rows):
    return {tuple(args)+(bool(variadic), result) for args, variadic, result in rows}


class ReservedKind:
    def __init__(self, kind):
        self.kind = kind

    def __getitem__(self, index):
        assert index == 0, ('D138 reserved value/context read', index)
        return self.kind


def contracts(replacement=None, verbose=True):
    ns, reference = ORACLE['load_reference'](replacement)
    assert set(reference.operator_signatures) == {(op, 1) for op in UNARY} | {(op, 2) for op in BINARY}, 'D138 operator wanted population'
    reference.reserved = {name: ReservedKind(kind) for name, kind in RESERVED.items()}
    env = {'v_'+kind: ORACLE['KindOnly'](kind) for kind in KINDS}
    for method in ('evaluate', '_evaluate', 'value_of_name', 'resolve_geometry', 'call', 'stored'):
        def trapped(*args, **kwargs):
            raise AssertionError('D138 value/execution read')
        setattr(reference, method, trapped)
    groups = {}
    refusals = 0

    def check(source, operation, kinds, roles, rows, result, group):
        nonlocal refusals
        assert len(kinds) == len(roles), 'D138 authored operand coverage'
        try:
            actual = reference.infer(reference.parse(source), env)
        except ns['FErr'] as error:
            assert result is None and error.token == 'formula_dimension', ('D138 refusal', source, result, error)
            args = error.arguments
            assert set(args) == {'operation', 'operand_kinds', 'operand_tolerances', 'wanted_signatures'}, ('D138 schema', source, args)
            assert args['operation'] == operation, ('D138 operation', source, args)
            assert args['operand_kinds'] == tuple(kinds), ('D138 actual kinds/order', source, args)
            assert args['operand_tolerances'] == tuple(roles), ('D138 actual symbolic roles', source, args)
            actual_rows = args['wanted_signatures']
            assert isinstance(actual_rows, tuple) and actual_rows, ('D138 wanted shape', source, args)
            for row in actual_rows:
                assert set(row) == {'operands', 'variadic', 'result'}, ('D138 wanted fields', source, row)
                assert isinstance(row['operands'], tuple) and type(row['variadic']) is bool, ('D138 wanted types', source, row)
            got = [tuple(row['operands'])+(row['variadic'], row['result']) for row in actual_rows]
            assert len(got) == len(set(got)), ('D138 wanted duplicate', source, got)
            assert set(got) == signatures(rows), ('D138 complete wanted alternatives', source, got, rows)
            advice = "an arc's length is arc_length(angle, radius)" in error.msg
            assert advice == (operation == '*' and set(kinds) == {'angle', 'length'}), ('D138 advice', source, error.msg)
            refusals += 1
        else:
            assert result is not None and actual == result, ('D138 acceptance', source, result, actual)
        groups[group] = groups.get(group, 0)+1

    # Multiple defects: actual left-to-right child failure wins; no complete kinds are invented.
    priority = (
        ('if(1,missing_left,v_length)', 'formula_unbound_name', 'missing_left'),
        ('if(1,missing_left,missing_right)', 'formula_unbound_name', 'missing_left'),
        ('if(1,v_length,missing_right)', 'formula_unbound_name', 'missing_right'),
        ('if(missing_condition,missing_left,missing_right)', 'formula_unbound_name', 'missing_condition'),
        ('within(missing_left,v_length,size_count)', 'formula_unbound_name', 'missing_left'),
        ('within(v_length,missing_right,size_count)', 'formula_unbound_name', 'missing_right'),
        ('within(v_length,v_length,missing_class)', 'formula_unbound_name', 'missing_class'),
        ('clamp(missing_left,v_angle,missing_right)', 'formula_unbound_name', 'missing_left'),
        ('clamp(v_length,v_angle,missing_last)', 'formula_unbound_name', 'missing_last'),
        ('v_point+missing_right', 'formula_unbound_name', 'missing_right'),
        ('missing_left+missing_right', 'formula_unbound_name', 'missing_left'),
        ('loop(missing_argument)', 'formula_unbound_name', 'loop'),
        ('if(1,nurbs(missing_argument),missing_right)', 'env_nurbs', 'nurbs'),
        ('within(v_length,solve(missing_argument),size_count)', 'env_sketch_constraints', 'solve'),
    )
    for source, token, name in priority:
        try:
            reference.infer(reference.parse(source), env)
        except ns['FErr'] as error:
            assert error.token == token and error.arguments.get('name') == name, ('D138 error selection', source, error.token, error.arguments)
        else:
            raise AssertionError(('D138 priority refusal accepted', source))
        groups['priority'] = groups.get('priority', 0)+1
    for kind in KINDS:
        name = 'v_'+kind
        check('-'+name, '-', (kind,), (None,), UNARY['-'], kind if kind in KINDS[:4] else None, 'unary')
        check(name+' ^ 2', '^2', (kind,), (None,), UNARY['^2'], {'length':'area', 'ratio':'ratio', 'count':'count'}.get(kind), 'unary')
    for operation, rows in BINARY.items():
        admitted = expanded(rows)
        for kinds in product(KINDS, repeat=2):
            source = ('v_%s '+operation+' v_%s') % kinds
            check(source, operation, kinds, (None, None), rows, admitted.get(kinds), 'binary')
    for name, rows in CALL_ROWS.items():
        if name in ('if', 'within'):
            continue
        variadic = name in ('min', 'max')
        counts = (1, 2, 3) if variadic else (len(rows[0][0]),)
        admitted = expanded(rows)
        for count in counts:
            for kinds in product(KINDS, repeat=count):
                result = kinds[0] if variadic and len(set(kinds)) == 1 and kinds[0] in ARITH else admitted.get(kinds) if not variadic else None
                check(name+'('+','.join('v_'+kind for kind in kinds)+')', name, kinds, (None,)*count, rows, result, 'calls')
        if not variadic:
            for count in sorted({max(1, len(rows[0][0])-1), len(rows[0][0])+1}-{len(rows[0][0])}):
                check(name+'('+','.join(['v_length']*count)+')', name, ('length',)*count, (None,)*count, rows, None, 'arity')
    admitted = expanded(CALL_ROWS['if'])
    for kinds in product(KINDS, repeat=3):
        check('if('+','.join('v_'+kind for kind in kinds)+')', 'if', kinds, (None,)*3, CALL_ROWS['if'], admitted.get(kinds), 'conditional')
    admitted = expanded(CALL_ROWS['within'])
    for name, kind in RESERVED.items():
        for left, right in product(KINDS, repeat=2):
            kinds = (left, right, kind)
            result = admitted.get(kinds) if name in TOLERANCES else None
            check('within(v_%s,v_%s,%s)' % (left, right, name), 'within', kinds, (None, None, name if name in TOLERANCES else None), CALL_ROWS['within'], result, 'tolerance')
    for third in ('1 um', 'v_length', '(eps_num+eps_num)', 'abs(eps_num)', '-eps_geo'):
        check('within(v_length,v_length,'+third+')', 'within', ('length',)*3, (None,)*3, CALL_ROWS['within'], None, 'roles')
    for name in TOLERANCES:
        check('within(v_length,v_length,(('+name+')))', 'within', ('length',)*3, (None,None,name), CALL_ROWS['within'], 'boolean', 'roles')
        check('sqrt('+name+')', 'sqrt', ('length',), (name,), CALL_ROWS['sqrt'], None, 'roles')
    for count in (1, 2, 4):
        check('within('+','.join(['v_length']*count)+')', 'within', ('length',)*count, (None,)*count, CALL_ROWS['within'], None, 'arity')
    for name in ('min', 'max'):
        for kind in ARITH:
            source = name+'('+','.join(['v_'+kind]*255)+')'
            check(source, name, (kind,)*255, (None,)*255, CALL_ROWS[name], kind, 'wide')
            try:
                reference.infer(reference.parse(name+'('+','.join(['v_'+kind]*256)+')'), env)
            except ns['FErr'] as error:
                assert error.token == 'formula_domain', ('D138 syntax-bound priority', error)
            else:
                raise AssertionError('D138 excess structural args accepted')
            groups['wide'] += 1

    assert groups == {'priority': 14, 'unary': 16, 'binary': 640, 'calls': 2264, 'arity': 30,
                      'conditional': 512, 'tolerance': 512, 'roles': 15, 'wide': 20}, ('D138 exact population coverage', groups)
    assert refusals == 3814, ('D138 complete refusal population', refusals)
    if verbose:
        print('dimension payloads:%d actual cases /%d complete dimension refusals; groups=%s; values/execution trapped' % (sum(groups.values()), refusals, groups))
    return groups, refusals


FAULTS = (
    ('actual operation forged', '"operation": operation, "operand_kinds"', '"operation": "unknown", "operand_kinds"'),
    ('last actual kind omitted', '"operand_kinds": tuple(kinds),', '"operand_kinds": tuple(kinds[:-1]),'),
    ('kind order reversed', '"operand_kinds": tuple(kinds),', '"operand_kinds": tuple(reversed(kinds)),'),
    ('symbolic role omitted', '"operand_tolerances": roles,', '"operand_tolerances": (None,)*len(roles),'),
    ('computed tolerance invented', '        return None\n\n    def _dimension', '        return "eps_num"\n\n    def _dimension'),
    ('wanted rule omitted', '"wanted_signatures": wanted}', '"wanted_signatures": wanted[:-1]}'),
    ('wanted result forged', '"result": result}', '"result": "boolean"}'),
    ('wanted operands reversed', '"operands": tuple(args),', '"operands": tuple(reversed(args)),'),
    ('variadic arity lost', '"variadic": variadic,', '"variadic": False,'),
    ('wanted duplicate', '"wanted_signatures": wanted}', '"wanted_signatures": wanted+wanted}'),
    ('if condition hides child', 'a, b = self.infer(node[2], env), self.infer(node[3], env)\n            if c != "boolean":',
     'if c != "boolean":\n                self._dimension("if", (node[1],), (c,), self.sigs["if"], "bad condition")\n            a, b = self.infer(node[2], env), self.infer(node[3], env)\n            if c != "boolean":'),
    ('within role hides child', 'kinds = [self.infer(a, env) for a in args]\n        if name == "within"',
     'if name == "within" and len(args) == 3 and self._tolerance_name(args[2]) is None:\n            self._dimension(name, args, ["length"]*len(args), self.sigs[name], "bad role")\n        kinds = [self.infer(a, env) for a in args]\n        if name == "within"'),
    ('branches read right first', 'a, b = self.infer(node[2], env), self.infer(node[3], env)\n            if c != "boolean":',
     'b = self.infer(node[3], env)\n            a = self.infer(node[2], env)\n            if c != "boolean":'),
    ('call operands read backwards', 'kinds = [self.infer(a, env) for a in args]', 'kinds = [self.infer(a, env) for a in reversed(args)]'),
    ('diagnostic reads value', 'roles = tuple(self._tolerance_name(node) for node in nodes)', 'self.evaluate(nodes[0], {})\n        roles = tuple(self._tolerance_name(node) for node in nodes)'),
)


if __name__ == '__main__':
    assert sys.argv[1:] in [[], ['--mutations']]
    original = SOURCE.read_bytes()
    contracts()
    if sys.argv[1:] == ['--mutations']:
        for name, before, after in FAULTS:
            try:
                contracts((before, after), verbose=False)
            except AssertionError as error:
                assert any(marker in str(error) for marker in ('D138', 'static checker accessed non-kind metadata')), (name, 'not an actual payload/body red', error)
                print('actual compiled dimension payload assertion red:', name)
            else:
                raise AssertionError(('dimension payload fault escaped', name))
        print('dimension payload faults:%d actual compiled body assertion reds; tracked source unchanged' % len(FAULTS))
    assert SOURCE.read_bytes() == original
