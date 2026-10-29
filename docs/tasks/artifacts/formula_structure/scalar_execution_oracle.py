"""Frozen scalar-controller rows from the independently parsed, table-driven book reference.

Pure guarded stdout; no Rust implementation or generated fixture is read. This finite population
checks exact unbound values, executed call contributions, name-read order and selected refusals.
It excludes tangent/geometry/prior-let integration and unexecuted Derived verdicts (D338).
"""
from fractions import Fraction
from pathlib import Path
import runpy

ROOT = Path(__file__).resolve().parents[4]
HERE = ROOT / 'docs/tasks/artifacts/formula_structure'
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
HELPER = HERE / 'static_signature_contract.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
PROFILE['plan_producer'](ROOT, sources=(Path(__file__), PROFILE_SOURCE, HELPER))
CONTRACT = runpy.run_path(str(HELPER))
PROFILE['plan_producer'](ROOT, sources=(Path(__file__), *CONTRACT['REFERENCE_INPUTS']))

# Inputs match explicit canonical fixtures in the native controller target. These are oracle
# fixture facts, not product providers, result annotations or independently authored Derived values.
ENVIRONMENT = {
    'private_width': dict(kind='length', value=None, origin='measurement', state='unknown'),
    'known_zero': dict(kind='length', value=Fraction(0), origin='parameter', state='known'),
    'assumed': dict(kind='length', value=Fraction(1000), origin='parameter', state='assumed'),
    'preferred': dict(kind='length', value=Fraction(-1000), origin='parameter', state='preference'),
    'use_fixed': dict(kind='boolean', value=Fraction(1), origin='parameter', state='preference'),
}
CASES = (
    ('1 um/3', None),
    ('0.0004 mm+0.0004 mm', None),
    ('0.0005 mm+0.0005 mm', None),
    ('-((2 mm)^2)', None),
    ('((1 um/3)+(2 um/3))*3', None),
    ('1 mm/3+1 mm/3', None),
    ('(1 um/3)^2', None),
    ('3/2', None),
    ('round_to(3 um,2 um)', None),
    ('round_to(-3 um,2 um)', None),
    ('sqrt((3 um/2)^2)', None),
    ('hypot(1 um,1 um)', None),
    ('sin(90 deg)', None),
    ('cos(180 deg)', None),
    ('atan(1.0)', None),
    ('atan2(1 um,1 um)', None),
    ('arc_length(0 deg,1 cm)', None),
    ('min(0 mm,round_to(1 mm,1 um))', None),
    ('max(2 mm,round_to(1 mm,1 um))', None),
    ('clamp(0 mm,0 mm,round_to(1 mm,1 um))', None),
    ('if(sin(0 deg)==0.0,1 um,2 um)', None),
    ('if(1==1,1 um,sin(90 deg)*1 um)', None),
    ('if(1!=1,sin(90 deg)*1 um,1 um)', None),
    ('if(use_fixed,known_zero,private_width)', None),
    ('if(use_fixed,assumed,preferred)', None),
    ('if(1!=1,assumed,preferred)', None),
    ('assumed+assumed+preferred', None),
    ('use_fixed', None),
    ('known_zero', None),
    ('within(1 um,1 um,eps_num)', None),
    ('within(sqrt((3 um)^2),3 um,eps_num)', 'formula_domain'),
    ('within(1 um,11 um,eps_geo)', None),
    ('if(1==1,1 um,eps_phys)', None),
    ('eps_phys', 'formula_tolerance_unbound'),
    ('is_base_size', 'formula_unbound_name'),
    ('private_width', 'formula_unknown'),
    ('min(0 um,private_width)', 'formula_unknown'),
    ('private_width+1 um/0', 'formula_unknown'),
    ('1 um/0+private_width', 'formula_division'),
    ('sqrt(-(1 um)^2)', 'formula_domain'),
    ('clamp(0 um,2 um,1 um)', 'formula_domain'),
    ('if(1==1,1 um/0,1 um)', 'formula_division'),
    ('if(1!=1,1 um/0,1 um)', None),
    ('within(sin(90 deg),1.0,eps_geo)', None),
    ('((1 um)^2)/(3 um)', None),
    ('0.0000004+0.0000004', None),
    ('if(sin(0 deg)==0.0,sqrt((3 um)^2),hypot(3 um,4 um))', None),
    ('hypot(round_to(3 um,2 um),sqrt((3 um)^2))', None),
    ('within(sin(90 deg)+cos(0 deg),2.0,eps_geo)', None),
)


def fixture_rows():
    namespace, reference = CONTRACT['load_reference']()
    original_read = reference.value_of_name
    reads = []

    def traced(name, environment):
        reads.append(name)
        return original_read(name, environment)

    reference.value_of_name = traced
    yield '# expression\tkind\tnumerator or refusal\tdenominator\tcall sources\texecuted names'
    for source, expected_error in CASES:
        # Parsing/inference must succeed for every runtime row, including both complete branches.
        environment = {name: dict(entry) for name, entry in ENVIRONMENT.items()}
        tree = reference.parse(source)
        reference.infer(tree, environment)
        reads.clear()
        try:
            value = reference.evaluate(tree, environment)
        except namespace['FErr'] as error:
            assert error.token == expected_error, (source, expected_error, error)
            # No accepted partial trace is compared after a failed execution.
            yield '\t'.join((source, 'error', error.token, '-', '-', '-'))
            continue
        assert expected_error is None, (source, 'missing intended refusal', expected_error)
        if value.kind == 'boolean':
            payload, denominator = ('true' if value.v else 'false'), '-'
        else:
            exact = Fraction(value.v)
            payload, denominator = str(exact.numerator), str(exact.denominator)
        yield '\t'.join((source, value.kind, payload, denominator,
                         ','.join(sorted(value.sources)) or '-', ','.join(reads) or '-'))


if __name__ == '__main__':
    print('\n'.join(fixture_rows()))
