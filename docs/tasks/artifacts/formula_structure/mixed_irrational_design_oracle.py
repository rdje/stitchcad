"""Independent mixed irrational recipes with exact per-call certificates and actual book execution.

No Rust,native answers or frozen fixtures are inputs. The book evaluator owns syntax,ordered
arguments,lazy Derived demand,storage and read witnesses. Exact integer/Fraction enclosures
certify each executed irrational call;ordinary book Decimal answers provide a separate check.
"""
from fractions import Fraction as F
from functools import lru_cache
from pathlib import Path
import runpy

ROOT = Path(__file__).resolve().parents[4]
HERE = Path(__file__).parent
TANGENT_SOURCE = HERE / 'tangent_design_recipe_oracle.py'
EXACT_SOURCE = HERE / 'tangent_value_oracle.py'
INVERSE_SOURCE = HERE / 'inverse_value_oracle.py'
ROOT_SOURCE = HERE / 'root_value_oracle.py'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
PROFILE['plan_producer'](ROOT, sources=(Path(__file__), TANGENT_SOURCE, EXACT_SOURCE,
    INVERSE_SOURCE, ROOT_SOURCE, PROFILE_SOURCE))
TANGENT = runpy.run_path(str(TANGENT_SOURCE))
EXACT = runpy.run_path(str(EXACT_SOURCE))
INVERSE = runpy.run_path(str(INVERSE_SOURCE))
ROOTS = runpy.run_path(str(ROOT_SOURCE))
M, A = 1_000_000, 90_000_000


def nearest(value):
    quotient, remainder = divmod(abs(value.numerator), value.denominator)
    magnitude = quotient + (2 * remainder >= value.denominator)
    return -magnitude if value < 0 else magnitude


@lru_cache(maxsize=1)
def inverse_constants():
    return INVERSE['pi_bounds'](), INVERSE['series'](F(1, 2))


@lru_cache(maxsize=None)
def forward(angle, cosine):
    # Cosine is independently represented by shifted sine,then signed principal reflection.
    centered = (angle + (A if cosine else 0) + 2 * A) % (4 * A) - 2 * A
    if centered > A:
        centered = 2 * A - centered
    elif centered < -A:
        centered = -2 * A - centered
    if not centered:
        return 0
    if abs(centered) == A:
        return M if centered > 0 else -M
    pi = EXACT['pi_bounds']()
    low, high = abs(centered) * pi[0] / (2 * A), abs(centered) * pi[1] / (2 * A)
    assert 0 < low <= high < pi[0] / 2, 'monotone sine interval stays below true pi/2'
    a = nearest(M * EXACT['alternating'](low, 32, False)[0])
    b = nearest(M * EXACT['alternating'](high, 32, False)[1])
    assert a == b, ('unresolved finite forward call', angle, cosine)
    return -a if centered < 0 else a


@lru_cache(maxsize=None)
def inverse(name, y, x=F(0)):
    pi, anchor = inverse_constants()
    interval = (INVERSE['atan'](y / M, pi, anchor) if name == 'atan' else
                INVERSE['atan2'](y, x, pi, anchor))
    low, high = map(INVERSE['rounded'], interval)
    assert low == high, ('unresolved finite inverse call', name, y, x)
    return low


@lru_cache(maxsize=None)
def arc(angle, radius):
    coefficient = angle * radius / (2 * A)
    low, high = sorted(coefficient * bound for bound in EXACT['pi_bounds']())
    a, b = nearest(low), nearest(high)
    assert a == b, ('unresolved finite arc call', angle, radius)
    return a


def configure(supplier, caller):
    def install(namespace, reference, owner):
        TANGENT['select'](supplier, caller)(namespace, reference, owner)
        delegate = reference.call
        def call(name, arguments, environment):
            if name not in {'sqrt', 'hypot', 'sin', 'cos', 'atan', 'atan2', 'arc_length'}:
                return delegate(name, arguments, environment)
            reference._check_callee(name)
            values = [reference.evaluate(argument, environment) for argument in arguments]
            raw = [F(value.v) for value in values]
            dfraction, rnd = namespace['dfraction'], namespace['rnd']
            if name == 'sqrt':
                if raw[0] < 0:
                    raise namespace['FErr']('formula_domain', 'negative root in actual mixed call')
                kind = 'ratio' if values[0].kind == 'ratio' else 'length'
                radicand = raw[0] * (M if kind == 'ratio' else 1)
                answer = ROOTS['rounded_root'](radicand)
                other = rnd(dfraction(radicand).sqrt())
            elif name == 'hypot':
                kind, radicand = 'length', raw[0]**2 + raw[1]**2
                answer = ROOTS['rounded_root'](radicand)
                other = rnd(dfraction(radicand).sqrt())
            elif name in ('sin', 'cos'):
                kind, answer = 'ratio', forward(raw[0], name == 'cos')
                other = rnd(M * namespace['d_' + name](namespace['d_radians'](raw[0])))
            elif name in ('atan', 'atan2'):
                if name == 'atan2' and raw[0] == raw[1] == 0:
                    raise namespace['FErr']('formula_domain', 'actual zero mixed vector')
                kind, answer = 'angle', inverse(name, *raw)
                radians = (namespace['d_atan'](dfraction(raw[0] / M)) if name == 'atan' else
                           namespace['d_atan2'](*map(dfraction, raw)))
                other = rnd(radians * (180 * M) / namespace['PI'])
            else:
                kind, answer = 'length', arc(*raw)
                other = rnd(namespace['d_radians'](raw[0]) * dfraction(raw[1]))
            assert answer == other, ('independent exact/book Decimal disagreement', name, raw, answer, other)
            return namespace['Val'](kind, F(answer)).influenced(*values, sources=(name,))
        reference.call = call
    return install


POLICY = (2073, 512)
EXACT_POLICY = (156, 0)
ZERO_POLICY = (0, 0)
CASES = (
    ('inverse_of_rounded_tangent', POLICY, POLICY, {},
     'let a:angle=atan(tan(30 deg))\nlet b:ratio=tan(a)', None),
    ('rounded_components_differ', POLICY, POLICY, {},
     'let a:ratio=tan(30 deg)\nlet b:ratio=sin(30 deg)/cos(30 deg)\nlet f:boolean=a!=b', None),
    ('root_hypot_inverse_arc', POLICY, POLICY, {},
     'let a:length=arc_length(atan(1.0),hypot(sqrt((3 mm)^2),4 mm))', None),
    ('all_eight_calls', POLICY, POLICY, {},
     'let a:ratio=tan(atan2(sin(atan(3.0/4.0))*hypot(3 mm,4 mm),'
     'cos(atan(3.0/4.0))*sqrt(arc_length(45 deg,4 mm)^2)))', None),
    ('shared_source_cancellation', POLICY, POLICY,
     {'9003': 'hypot(sqrt((3 mm)^2),arc_length(atan(1.0),4 mm))'},
     'let a:length=declared_width-width_alias\nassert zero:eps_geo=a==0 mm', None),
    ('false_mixed_condition', POLICY, POLICY, {},
     'let f:boolean=sin(atan2(0 mm,1 mm))!=0.0\nlet a:length=if(f,eps_phys,cos(0 deg)*1 mm)', None),
    ('signed_multiturn', POLICY, EXACT_POLICY, {'9103': 'atan2(-1 mm,-1 mm)-360 deg'},
     'let a:ratio=tan(angle_value)\nlet b:length=arc_length(angle_value,1 mm)', None),
    ('rounded_zero_changes_axis', POLICY, POLICY,
     {'9103': 'atan2(sin(-0.000001 deg),cos(-180 deg))'},
     'let a:angle=angle_value\nlet b:ratio=tan(a)', None),
    ('negative_axis_principal', POLICY, POLICY,
     {'9103': 'atan2(-1 um/1000000000000000000,-1 mm)'},
     'let a:angle=angle_value\nlet b:ratio=cos(a)', None),
    ('root_tie_before_storage', POLICY, POLICY, {'9003': 'sqrt((3 um/2)^2)'},
     'let a:length=declared_width*3\nlet b:length=sqrt((declared_width*3)^2)', None),
    ('root_source_unbound_storage', POLICY, POLICY, {'9003': 'sqrt((3 um/2)^2)/3'},
     'let a:length=declared_width*3\nlet b:length=declared_width\nlet c:length=b*3', None),
    ('arc_root_hypot_rounding', POLICY, POLICY,
     {'9003': 'hypot(arc_length(90 deg,1 um),sqrt((3 um)^2/2))'},
     'let a:length=declared_width\nlet b:length=width_alias+length_mirror', None),
    ('sine_of_inverse', POLICY, POLICY, {}, 'let a:ratio=sin(atan(3.0/4.0))', None),
    ('cosine_of_inverse_vector', POLICY, POLICY, {}, 'let a:ratio=cos(atan2(4 mm,3 mm))', None),
    ('shared_mixed_dependency', POLICY, POLICY,
     {'9103': 'atan(tan(30 deg))', '9003': 'arc_length(angle_value,hypot(3 mm,4 mm))',
      '9004': 'declared_width+length_mirror'},
     'let a:length=tripled_source\nlet b:length=declared_width+width_alias\nassert same:eps_geo=a==b', None),
    ('independent_caller_exact', POLICY, EXACT_POLICY, {'9103': 'atan(tan(30 deg))'},
     'let a:ratio=tan(angle_value-angle_value)', None),
    ('untaken_mixed_failure', POLICY, POLICY, {'9003': 'sqrt(-(1 um)^2)'},
     'let a:length=if(cos(0 deg)==1.0,arc_length(atan(1.0),1 mm),tripled_source)', None),
    ('empty_mixed_sources', ZERO_POLICY, ZERO_POLICY,
     {'9003': 'sqrt(-(1 um)^2)+1 mm*tan(90 deg)'}, '', None),
    ('inverse_of_root_ratio', POLICY, POLICY, {}, 'let a:angle=atan(sqrt(2.0))', None),
    ('inverse_multiturn_arc', POLICY, POLICY, {},
     'let a:angle=atan2(1 mm,-1 mm)+360 deg\nlet b:length=arc_length(a,1 mm)', None),
    ('root_before_tangent_resources', POLICY, ZERO_POLICY, {},
     'let blocked:ratio=tan(atan(sqrt(-1.0)))', '1:blocked:recipe:formula_domain'),
    ('resources_after_root_inverse', POLICY, (155, 0), {},
     'let blocked:ratio=tan(atan(sqrt(1.0)))',
     '1:blocked:recipe:resources.integer_bits.45000000.1.0.0.0.155.156'),
    ('inverse_rounds_to_pole', POLICY, EXACT_POLICY, {},
     'let blocked:ratio=tan(atan(1000000000000000000.0))', '1:blocked:recipe:formula_domain'),
    ('source_root_before_resources', ZERO_POLICY, POLICY,
     {'9003': 'sqrt(-(1 um)^2)+tan(45 deg)*1 mm'},
     'let blocked:length=tripled_source', '1:blocked:9003:formula_domain'),
    ('source_resources_before_root', (155, 0), POLICY,
     {'9003': 'tan(atan(sqrt(1.0)))*1 mm+sqrt(-(1 um)^2)'},
     'let blocked:length=tripled_source',
     '1:blocked:9003:resources.integer_bits.45000000.1.0.0.0.155.156'),
    ('division_before_inverse', POLICY, POLICY, {},
     'let blocked:angle=atan(1.0/0)+atan2(0 mm,0 mm)', '1:blocked:recipe:formula_division'),
    ('inverse_before_division', POLICY, POLICY, {},
     'let blocked:angle=atan2(0 mm,0 mm)+atan(1.0/0)', '1:blocked:recipe:formula_domain'),
    ('arc_argument_order', POLICY, POLICY, {},
     'let blocked:length=arc_length(atan2(0 mm,0 mm),sqrt((1 mm)^2/0))', '1:blocked:recipe:formula_domain'),
    ('hypot_division_first', POLICY, POLICY, {},
     'let blocked:length=hypot(1 mm/0,sqrt(-(1 um)^2))', '1:blocked:recipe:formula_division'),
    ('hypot_root_first', POLICY, POLICY, {},
     'let blocked:length=hypot(sqrt(-(1 um)^2),1 mm/0)', '1:blocked:recipe:formula_domain'),
    ('mixed_flags_t1', POLICY, POLICY, {'9003': 'arc_length(atan(1.0),hypot(3 mm,4 mm))'},
     'let a:length=declared_width-declared_width\nassert blocked:eps_num=a==0 mm', '2:blocked:recipe:formula_domain'),
    ('mixed_assert_left_before_header', POLICY, POLICY, {},
     'assert blocked:eps_phys=atan2(0 mm,0 mm)==atan(1.0/0)', '1:blocked:recipe:formula_domain'),
)


def fixture_rows():
    assert len(CASES) == 32 and len({case[0] for case in CASES}) == len(CASES)
    assert ROOTS['rounded_root'](F(9, 4)) == 2
    quotient = nearest(F(M * forward(F(30 * M), False), forward(F(30 * M), True)))
    assert quotient == 577351 and EXACT['outcome'](F(30 * M)) == ('value', '577350')
    assert inverse('atan', F(577350)) != 30 * M
    assert inverse('atan', F(10**18 * M)) == A
    yield '# case\tsupplier bits/terms\tcaller bits/terms\toriginal sources\toriginal recipe\tcomplete outcome or first refusal\tunbound roots\trecipe reads\tsource reads'
    reference = TANGENT['REFERENCE']
    for label, supplier, caller, overrides, recipe, intended in CASES:
        assert set(overrides) <= dict(reference['DEFAULT_SOURCES']).keys()
        sources = {**dict(reference['DEFAULT_SOURCES']), **overrides}
        outcome = reference['evaluate_case'](sources, recipe, intended,
            configure=configure(supplier, caller), extra_failures=(TANGENT['TangentRefusal'],))
        yield '\t'.join((label, '%d/%d' % supplier, '%d/%d' % caller,
            ';'.join(identity + '=' + body for identity, body in sources.items()),
            recipe.replace('\n', r'\n'), *outcome))


if __name__ == '__main__':
    print('\n'.join(fixture_rows()))
