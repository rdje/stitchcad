"""Independent paired tangent recipes:actual book syntax/storage plus exact Fraction Tan.

Neither Rust,native outputs nor frozen expected fixtures are inputs. The existing recursive
Derived executor owns actual source demand and once-only storage. Resource refusals remain
separate exceptions;they never become formula-language diagnostics or computed values.
"""
from fractions import Fraction as F
from functools import lru_cache
from pathlib import Path
import runpy

ROOT = Path(__file__).resolve().parents[4]
BASE = Path(__file__).with_name('scalar_design_recipe_oracle.py')
TAN = Path(__file__).with_name('tangent_value_oracle.py')
BUDGET = Path(__file__).with_name('tangent_budget_plan.py')
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
PROFILE['plan_producer'](ROOT, sources=(Path(__file__), BASE, TAN, BUDGET, PROFILE_SOURCE))
REFERENCE = runpy.run_path(str(BASE))
EXACT = runpy.run_path(str(TAN))
ADMISSION = runpy.run_path(str(BUDGET))
math_outcome = lru_cache(maxsize=None)(EXACT['outcome'])


class TangentRefusal(Exception):
    def __init__(self, token):
        self.token = token
        super().__init__(token)


def select(supplier, caller):
    def configure(namespace, reference, owner):
        original = reference.call
        def call(name, arguments, environment):
            if name != 'tan':
                return original(name, arguments, environment)
            # Execute the original child exactly once,with the actual recursive source owner.
            assert len(arguments) == 1
            value = reference.evaluate(arguments[0], environment)
            assert value.kind == 'angle'
            angle = F(value.v)
            limits = supplier if owner() is not None else caller
            trace = ADMISSION['evaluate'](angle.numerator, angle.denominator,
                ADMISSION['Budget'](*limits))
            if trace.kind == 'resources':
                refusal = trace.refusal
                raise TangentRefusal('resources.%s.%d.%d.%d.%d.%d.%d.%d' % (
                    refusal.resource, angle.numerator, angle.denominator, trace.precision,
                    trace.terms, trace.reserved_integer_bits, refusal.maximum, refusal.requested))
            kind, payload = math_outcome(angle)
            if kind == 'pole':
                raise namespace['FErr']('formula_domain', 'actual exact tangent pole')
            if kind == 'width':
                # The older budget model can stop at a width range before its exact bit count.
                # Only this fixed generously admitted witness is in this finite composition;
                # never infer that a smaller policy reaches the native exact-width boundary.
                assert angle == F(A * (U // A) - 1, U // A) and limits == (16412, 7680)
                raise TangentRefusal('width.' + payload)
            assert kind == 'value'
            return namespace['Val']('ratio', F(int(payload))).influenced(value, sources=('tan',))
        reference.call = call
    return configure


NEAR_N = 247385123034593398372138492992328993860
NEAR_D = 8635369834810561370044731507908276509
U, A = (1 << 128) - 1, 90_000_000

def angle(n, d):
    sign = '-' if n < 0 else ''
    n = abs(n)
    return '%s%d.%06d deg/%d' % (sign, n // 1_000_000, n % 1_000_000, d)


# Source registry and canonical declarations remain the original actual book fixture facts.
# Policies belong to requests,not these declarations or computed roots.
CASES = (
    ('empty_zero', (0, 0), (0, 0), {'9100': 'tan(90 deg)'}, '', None),
    ('source_once', (2073, 512), (156, 0), {'9003': '1 mm*tan(30 deg)/1000'},
     'let a:length=declared_width\nlet b:length=a+a\nlet c:ratio=tan(45 deg)\nassert same:eps_geo=b==0.002 mm', None),
    ('source_once_more', (16412, 7680), (16412, 7680), {'9003': '1 mm*tan(30 deg)/1000'},
     'let a:length=declared_width\nlet b:length=a+a\nlet c:ratio=tan(45 deg)\nassert same:eps_geo=b==0.002 mm', None),
    ('recipe_once', (0, 0), (2073, 512), {},
     'let a:ratio=tan(30 deg)/1000000\nlet b:ratio=a+a\nlet c:ratio=tan(30 deg)\nassert same:eps_geo=b==0.000002', None),
    ('false_flag', (156, 0), (156, 0), {'9102': 'tan(0 deg)!=0.0'},
     'let f:boolean=boolean_value\nlet a:length=if(f,eps_phys,1 mm)\nassert safe:eps_geo=a==1 mm', None),
    ('lazy_poles', (0, 0), (156, 0), {'9100': 'tan(90 deg)'},
     'let a:ratio=if(1==1,tan(45 deg),ratio_value)', None),
    ('signed_turn', (2073, 512), (2073, 512), {'9100': 'tan(-330 deg)'},
     'let a:ratio=ratio_value\nlet b:ratio=tan(-30 deg)\nassert zero:eps_geo=a+b==0.0', None),
    ('unbound_angle', (0, 0), (8219, 3584), {'9103': angle(NEAR_N, NEAR_D)},
     'let a:ratio=tan(angle_value)\nassert zero:eps_geo=a==0.0', None),
    ('near_above', (0, 0), (8219, 3584), {'9103': angle(NEAR_N + 1, NEAR_D)},
     'let a:ratio=tan(angle_value)\nassert one:eps_geo=a==0.000001', None),
    ('nested', (156, 0), (2073, 512), {'9103': '30 deg+tan(45 deg)*0 deg'},
     'let a:ratio=tan(angle_value)\nlet b:ratio=tan(atan(1.0))', None),
    ('source_pole', (156, 0), (16412, 7680), {'9003': '1 mm*tan(90 deg)'},
     'let safe:length=1 mm\nlet blocked:length=tripled_source', '2:blocked:9003:formula_domain'),
    ('recipe_pole_first', (0, 0), (156, 0), {'9003': '1 mm/0'},
     'let blocked:ratio=tan(90 deg)+tripled_source/1 mm', '1:blocked:recipe:formula_domain'),
    ('source_division_first', (156, 0), (156, 0), {'9003': '1 mm/0'},
     'let blocked:ratio=tripled_source/1 mm+tan(90 deg)', '1:blocked:9003:formula_division'),
    ('flags_t1', (156, 0), (156, 0), {'9003': '1 mm*tan(45 deg)'},
     'let a:length=declared_width\nassert blocked:eps_num=a==1 mm', '2:blocked:recipe:formula_domain'),
    ('assert_left', (156, 0), (156, 0), {},
     'assert blocked:eps_phys=tan(90 deg)==1.0/0', '1:blocked:recipe:formula_domain'),
    ('assert_right', (156, 0), (156, 0), {},
     'assert blocked:eps_phys=1.0==tan(90 deg)', '1:blocked:recipe:formula_domain'),
    ('assert_header', (156, 0), (156, 0), {},
     'assert blocked:eps_phys=tan(45 deg)==1.0', '1:blocked:recipe:formula_tolerance_unbound'),
    ('fold_before_pole', (0, 0), (155, 0), {},
     'let blocked:ratio=tan(90 deg)', '1:blocked:recipe:resources.integer_bits.90000000.1.0.0.0.155.156'),
    ('source_initial_bits', (2072, 511), (16412, 7680), {'9003': '1 mm*tan(30 deg)'},
     'let blocked:length=tripled_source', '1:blocked:9003:resources.integer_bits.30000000.1.128.0.156.2072.2073'),
    ('recipe_initial_terms', (16412, 7680), (8219, 511), {},
     'let blocked:ratio=tan(30 deg)', '1:blocked:recipe:resources.series_terms.30000000.1.128.0.156.511.512'),
    ('near_late_bits', (0, 0), (4122, 3584), {'9103': angle(NEAR_N, NEAR_D)},
     'let blocked:ratio=tan(angle_value)', '1:blocked:recipe:resources.integer_bits.%d.%d.512.1536.4122.4122.8219' % (NEAR_N, NEAR_D)),
    ('near_late_terms', (0, 0), (8219, 1536), {'9103': angle(NEAR_N, NEAR_D)},
     'let blocked:ratio=tan(angle_value)', '1:blocked:recipe:resources.series_terms.%d.%d.512.1536.4122.1536.2048' % (NEAR_N, NEAR_D)),
    ('native_wide_storage', (0, 0), (16412, 7680), {'9103': angle(A * 10**20 - 1, 10**20)},
     'let blocked:ratio=tan(angle_value)', '1:blocked:recipe:formula_domain'),
    ('native_width', (0, 0), (16412, 7680), {'9103': angle(A * (U // A) - 1, U // A)},
     'let blocked:ratio=tan(angle_value)', '1:blocked:recipe:width.148'),
)


def fixture_rows():
    yield '# case\tsupplier bits/terms\tcaller bits/terms\toriginal sources\toriginal recipe\tcomplete outcome or first refusal\tunbound roots\trecipe reads\tsource reads'
    seen = set()
    for label, supplier, caller, overrides, recipe, intended in CASES:
        assert label not in seen and set(overrides) <= dict(REFERENCE['DEFAULT_SOURCES']).keys()
        seen.add(label)
        sources = {**dict(REFERENCE['DEFAULT_SOURCES']), **overrides}
        outcomes = REFERENCE['evaluate_case'](sources, recipe, intended,
            configure=select(supplier, caller), extra_failures=(TangentRefusal,))
        yield '\t'.join((label, '%d/%d' % supplier, '%d/%d' % caller,
            ';'.join(identity + '=' + body for identity, body in sources.items()),
            recipe.replace('\n', r'\n'), *outcomes))


if __name__ == '__main__':
    print('\n'.join(fixture_rows()))
