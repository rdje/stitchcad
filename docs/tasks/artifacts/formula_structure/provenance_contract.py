"""D121: independent executed-contribution and tolerance controls on the actual reference."""
from copy import deepcopy
from fractions import Fraction
from pathlib import Path
import os
import runpy
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
HELPER = runpy.run_path(str(ROOT / 'docs/tasks/artifacts/formula_structure/static_signature_contract.py'))
SOURCE = HELPER['SOURCE']
CLASSES = {'eps_num': 1, 'eps_geo': 10, 'eps_fmt': 25, 'eps_imp': 40, 'eps_phys': 100}
# Curated exact answers authored independently of the implementation/table reader. Every declared
# irrational signature is covered, including calls whose particular answer happens to be rational.
CALLS = (
    ('sqrt', 'sqrt((1 cm)^2)', 'length', 10000),
    ('sqrt', 'sqrt(1.0)', 'ratio', 1000000),
    ('hypot', 'hypot(3 um,4 um)', 'length', 5),
    ('sin', 'sin(90 deg)', 'ratio', 1000000),
    ('cos', 'cos(0 deg)', 'ratio', 1000000),
    ('tan', 'tan(45 deg)', 'ratio', 1000000),
    ('atan', 'atan(1.0)', 'angle', 45000000),
    ('atan2', 'atan2(1 um,1 um)', 'angle', 45000000),
    ('atan2', 'atan2(1.0,1.0)', 'angle', 45000000),
    ('arc_length', 'arc_length(0 deg,1 cm)', 'length', 0),
    ('dist', 'dist(p,q)', 'length', 5),
    ('dir', 'dir(p,q)', 'angle', 53130102),
    ('round_to', 'round_to(3 um,2 um)', 'length', 4),
    ('round_to', 'round_to(3 deg,2 deg)', 'angle', 4000000),
    ('round_to', 'round_to((3 um)^2,(2 um)^2)', 'area', 8),
    ('round_to', 'round_to(3.0,2.0)', 'ratio', 4000000),
    ('round_to', 'round_to(3,2)', 'count', 4),
)


def entry(kind, value, sources=()):
    return {'kind': kind, 'value': value, 'origin': 'recipe', 'state': 'derived',
            'sources': frozenset(sources)}


def contracts(replacement=None, verbose=True):
    ns, ref = HELPER['load_reference'](replacement)
    checks = 0
    points = {'p': entry('point', (Fraction(0), Fraction(0))),
              'q': entry('point', (Fraction(3), Fraction(4)))}

    def value(source, env, kind, expected, sources):
        nonlocal checks
        actual = ref.statement(source, env)[3]
        assert (actual.kind, actual.v) == (kind, expected), ('provenance value', source, actual, expected)
        assert actual.sources == frozenset(sources), ('provenance sources', source, actual.sources, sources)
        assert isinstance(actual.sources, frozenset), ('provenance mutable sources', source)
        checks += 1
        return actual

    def comparison(source, env, wanted, label=None, sources=()):
        nonlocal checks
        try:
            result = ref.statement(source, env)
        except ns['FErr'] as error:
            assert error.token == wanted, ('provenance diagnostic', source, wanted, error.token)
            if wanted == 'formula_domain':
                assert error.arguments == {'comparison': label, 'tolerance_class': 'eps_num',
                                           'contribution_sources': tuple(sorted(sources))}, (
                    'provenance arguments', source, error.arguments, label, sources)
                assert 'T2 or looser' in error.msg and 'eps_num' in error.msg, ('provenance message', error.msg)
        else:
            assert wanted is None, ('provenance comparison accepted', source, wanted)
            assert result[2] is True if result[0] == 'assert' else result[3].v == 1, (
                'provenance comparison value', source, result)
        checks += 1

    for call, source, kind, expected in CALLS:
        value(source, deepcopy(points), kind, expected, (call,))
        for tolerance, bound in CLASSES.items():
            # Supplied looser contexts are fixture values, never new product defaults.
            ref.reserved[tolerance] = ('length', False, Fraction(bound))
            for reverse in (False, True):
                env = deepcopy(points)
                env['answer'] = entry(kind, Fraction(expected))
                left, right = (source, 'answer') if not reverse else ('answer', source)
                for assertion in (False, True):
                    expression = ('assert closure: %s = %s == %s' % (tolerance, left, right)
                                  if assertion else 'within(%s,%s,%s)' % (left, right, tolerance))
                    comparison(expression, env, 'formula_domain' if tolerance == 'eps_num' else None,
                               'closure' if assertion else 'within', (call,))
    # Execute, store, publish and read actual returned bindings, including a Boolean dependency.
    env = {}
    for source, kind, expected, sources in (
        ('let a:ratio=sin(90 deg)', 'ratio', 1000000, ('sin',)),
        ('let b:ratio=a / 3', 'ratio', 333333, ('sin',)),
        ('let c:boolean=b > 0.0', 'boolean', 1, ('sin',)),
        ('let d:length=if(c,1 cm,2 cm)', 'length', 10000, ('sin',)),
    ):
        returned = value(source, env, kind, expected, sources)
        name = ref.syntax_statement(source)[1]
        env[name] = entry(returned.kind, returned.v, returned.sources)
        value(name, env, kind, expected, sources)
    comparison('assert stored:eps_num=d==1 cm', env, 'formula_domain', 'stored', ('sin',))
    # Each ordinary operator/function keeps all executed dependencies, even at zero or selection.
    for source, kind, expected in (
        ('-a', 'ratio', -1000000), ('a^2', 'ratio', 1000000),
        ('a+0.0', 'ratio', 1000000), ('0.0+a', 'ratio', 1000000),
        ('a-a', 'ratio', 0), ('a*0.0', 'ratio', 0), ('a/a', 'ratio', 1000000),
        ('abs(-a)', 'ratio', 1000000), ('min(a,0.0)', 'ratio', 0),
        ('max(a,2.0)', 'ratio', 2000000), ('clamp(a,2.0,3.0)', 'ratio', 2000000),
        ('if(1==1,a,0.0)', 'ratio', 1000000), ('if(1!=1,0.0,a)', 'ratio', 1000000),
        ('within(a,1.0,eps_geo)', 'boolean', 1),
    ):
        value(source, env, kind, expected, ('sin',))
    for operator, expected in (('==', 1), ('!=', 0), ('<', 0), ('<=', 1), ('>', 0), ('>=', 1)):
        value('a%s1.0' % operator, env, 'boolean', expected, ('sin',))
    comparison('assert union:eps_num=sin(90 deg)+cos(0 deg)==2.0', {}, 'formula_domain', 'union', ('sin', 'cos'))
    # Laziness and plain rational/input/storage rounding: no source is invented by the mere AST.
    for source, kind, expected in (
        ('if(1==1,1.0,sin(90 deg))', 'ratio', 1000000),
        ('if(1!=1,sin(90 deg),1.0)', 'ratio', 1000000),
        ('let exact:length=1 um / 2', 'length', 1),
        ('0.00005 cm', 'length', 1), ('1 cm / 3', 'length', Fraction(10000, 3)),
    ):
        value(source, {}, kind, expected, ())
    comparison('assert lazy:eps_num=if(1==1,1.0,sin(90 deg))==1.0', {}, None)
    comparison('within(if(1!=1,sin(90 deg),1.0),1.0,eps_num)', {}, None)
    # Lazy operation arguments and cache preserve precise per-coordinate evidence.
    for axis in ('x', 'y'):
        expressions = {'x': '1 um', 'y': '2 um'}
        expressions[axis] = 'hypot(3 um,4 um)'
        geo = {'point_value': {'kind': 'point', 'origin': 'geometry', 'state': 'derived',
                              'value': None, 'lazy': True, 'exprs': expressions}}
        for _ in range(2):
            for selector in ('x', 'y'):
                expected = 5 if selector == axis else (1 if selector == 'x' else 2)
                sources = ('hypot',) if selector == axis else ()
                value('%s(point_value)' % selector, geo, 'length', expected, sources)
                comparison('assert coordinate:eps_num=%s(point_value)==%d um' % (selector, expected),
                           geo, 'formula_domain' if sources else None, 'coordinate', sources)
            assert geo['point_value']['lazy'] is False
        failed = {'broken': {'kind': 'point', 'origin': 'geometry', 'state': 'derived', 'value': None,
                             'lazy': True, 'exprs': {'x': expressions[axis], 'y': '1 um / 0'}}}
        before = deepcopy(failed)
        comparison('assert broken:eps_num=x(broken)==1 um', failed, 'formula_division')
        assert failed == before, 'provenance failed geometry published evidence'
    geo = {'edge_value': {'kind': 'edge', 'origin': 'geometry', 'state': 'derived', 'value': None,
                          'lazy': True, 'exprs': {'len': 'hypot(3 um,4 um)'}}}
    for _ in range(2):
        value('len(edge_value)', geo, 'length', 5, ('hypot',))
        value('param_at(edge_value,1 um)', geo, 'ratio', 200000, ('hypot',))
        value('point_at(edge_value,0.5)', geo, 'point', 500000, ('hypot',))
    # Earlier refusals stay earlier; supplied numeric T1 does not change the symbolic class.
    ref.reserved['eps_num'] = ('length', True, Fraction(100))
    comparison('assert symbolic:eps_num=sin(90 deg)==1.0', {}, 'formula_domain', 'symbolic', ('sin',))
    ref.reserved['eps_phys'] = ('length', False, None)
    comparison('assert missing:eps_phys=sin(90 deg)==1.0', {}, 'formula_tolerance_unbound')
    comparison('assert divide:eps_num=sin(90 deg)/0==1.0', {}, 'formula_division')
    comparison('assert kinds:eps_num=sin(90 deg)==1 cm', {}, 'formula_dimension')
    comparison('assert names:eps_num=sin(90 deg)==missing', {}, 'formula_unbound_name')
    comparison('assert loose_false:eps_geo=sin(90 deg)==0.0', {}, 'formula_assertion')
    value('within(sin(90 deg),0.0,eps_geo)', {}, 'boolean', 0, ('sin',))
    if verbose:
        print('provenance contract: %d cases / named T1 refusal / five classes / executed dependencies / stored reads / precise cached geometry / lazy and exact controls' % checks)
    return checks


def consumer(replacement=None, verbose=True):
    work = ROOT / 'target/formula_structure/provenance_consumer'
    book = work / 'src'
    if book.exists():
        assert not (book / '.git').exists(), 'provenance consumer is a Git repository'
        shutil.rmtree(book)
    book.parent.mkdir(parents=True, exist_ok=True)
    shutil.copytree(ROOT / 'docs/book/src', book)
    examples = book / 'spec/formula-language/examples.md'
    text = examples.read_text()
    before = 'assert allocation_balance: eps_num = 4 * (ss_suppress + dart_intake) == garment_hip - garment_waist'
    assert text.count(before) == 1, 'provenance consumer book anchor changed'
    examples.write_text(text.replace(before, 'assert allocation_balance: eps_num = side_seam_length == side_seam_length'))
    script = SOURCE
    if replacement:
        before, after = replacement
        program = SOURCE.read_text()
        assert program.count(before) == 1, ('provenance consumer fault anchor', before)
        script = work / 'mutant.sh'
        script.write_text(program.replace(before, after, 1))
    result = subprocess.run(['bash', str(script)], cwd=ROOT, env=dict(os.environ, FORMULA_BOOK=str(book)),
                            capture_output=True, text=True)
    output = result.stdout + result.stderr
    (work / 'consumer.log').write_text(output)
    assert result.returncode == 1 and 'formula_domain' in output and 'allocation_balance' in output \
        and 'eps_num' in output and 'contributions=hypot' in output, ('provenance consumer refusal', result.returncode, output)
    assert 'statically accepted statements: 21' in output, 'provenance consumer failed statically'
    if verbose:
        print('provenance consumer: actual copied-book equal bound irrational operands refuse named T1 at runtime, rc=1')


FAULTS = (
    ('class guard bypass', 'if name == "eps_num" and sources:', 'if False:'),
    ('within guard omitted', 'self.tolerance_class("within", args[2][1], a, b)', 'pass'),
    ('assertion guard omitted', 'self.tolerance_class(name, tol_name, a, b)', 'pass'),
    ('numeric class substitution', 'if name == "eps_num" and sources:', 'if name == "eps_geo" and sources:'),
    ('class token', 'raise FErr("formula_domain", "%s: `%s` requires T2', 'raise FErr("formula_assertion", "%s: `%s` requires T2'),
    ('comparison argument', '{"comparison": comparison, "tolerance_class": name,', '{"comparison": "wrong", "tolerance_class": name,'),
    ('source arguments', '"contribution_sources": tuple(sorted(sources))', '"contribution_sources": ()'),
    ('stored provenance', 'return Val(value.kind, Fraction(integer)).influenced(value)', 'return Val(value.kind, Fraction(integer))'),
    ('read provenance', 'e.get("sources", ()), e.get("components")', '(), None'),
    ('operator union', 'self.sources.union(sources, *(value.sources for value in values))', 'self.sources.union(sources)'),
    ('conditional contribution', 'node[3], env).influenced(c)', 'node[3], env)'),
    ('untaken contribution', 'node[3], env).influenced(c)',
     'node[3], env).influenced(c, self.evaluate(node[3] if c.v else node[2], env))'),
    ('edge cache provenance', 'entry["sources"] = v.sources', 'entry["sources"] = frozenset()'),
    ('coordinate separation', 'vs[0].components[index] if vs[0].components is not None else vs[0].sources', 'vs[0].sources'),
    ('coordinate cache provenance', 'entry["components"] = (xs.sources, ys.sources)', 'entry["components"] = (frozenset(), frozenset())'),
)
# Each approximation-producing branch has its own actual fault, including overloaded branches.
PRODUCERS = (
    'return Val("length", rnd(dfraction(vs[0].v).sqrt()))',
    'return Val("ratio", rnd(from_true("ratio", d_sqrt_ratio(vs[0].v))))',
    'return Val("length", self._hypot(vs[0].v, vs[1].v))',
    'return Val("ratio", rnd(from_true("ratio", r)))',
    'return Val("angle", rnd(d_atan(dfraction(to_true("ratio", vs[0].v)))\n                                     * 180 / PI * 1000000))',
    'return Val("angle", rnd(d_atan2(ya, xb) * 180 / PI * 1000000))',
    'return Val("length", rnd(rad * dfraction(to_true("length", vs[1].v))))',
    'return Val("length", self._hypot(p.v[0] - q.v[0], p.v[1] - q.v[1]))',
    'return Val("angle", norm_angle(rnd(self._deg2(q.v[1] - p.v[1], q.v[0] - p.v[0]))))',
    'return Val(k0, rnd(Fraction(vs[0].v) / Fraction(step)) * step)',
)
if __name__ == '__main__':
    original = SOURCE.read_bytes()
    contracts()
    consumer()
    if '--mutations' in sys.argv:
        faults = list(FAULTS) + [('producer %d' % i, code + '.influenced(*vs, sources=(name,))',
                                 code + '.influenced(*vs)') for i, code in enumerate(PRODUCERS, 1)]
        for name, before, after in faults:
            try:
                contracts((before, after), False)
            except AssertionError as error:
                assert 'provenance' in str(error) and 'fault anchor' not in str(error), (name, 'not a body assertion red', error)
                print('  actual compiled provenance red:', name)
            else:
                raise AssertionError(('provenance fault escaped', name))
        try:
            consumer(('env[token]["sources"] = val.sources', 'env[token]["sources"] = frozenset()'), False)
        except AssertionError as error:
            assert 'provenance consumer refusal' in str(error), ('consumer not a body assertion red', error)
            print('  actual compiled provenance red: consumer drops binding evidence')
        else:
            raise AssertionError('provenance consumer fault escaped')
        assert SOURCE.read_bytes() == original
        print('provenance faults: %d actual body assertion reds; source unchanged' % (len(faults) + 1))
