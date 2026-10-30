"""Actual native tangent arithmetic faults through the original-scope Cargo controller."""
from pathlib import Path
import runpy

ROOT = Path(__file__).resolve().parents[4]
DRIVER = Path(__file__).with_name('root_body_faults.py')
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
PROFILE['plan_producer'](ROOT, sources=(Path(__file__), DRIVER, PROFILE_SOURCE))
CONTROLLER = runpy.run_path(str(DRIVER))
ENDPOINTS = (
    '        low: super::super::rational::round_magnitude_wide(n.low * 1_000_000_u32, d.high),\n'
    '        high: super::super::rational::round_magnitude_wide(n.high * 1_000_000_u32, d.low),'
)
FAMILIES = (
    ('fold', 'prepare.rs', (
        ('period_halved', 'let period = &quarter * 2_u8;', 'let period = &quarter * 1_u8;'),
        ('reflection_sign_lost', 'negative = !negative;', 'negative ^= false;'),
        ('exact_unit_scale_lost', 'magnitude: 1_000_000,', 'magnitude: 100_000,'),
    )),
    ('interval', 'interval.rs', (
        ('reciprocal_lost', 'let (n, mut d) = if reciprocal {', 'let (n, mut d) = if false {'),
        ('quotient_scale_lost', ENDPOINTS, ENDPOINTS.replace('1_000_000', '100_000')),
        ('quotient_truncated', ENDPOINTS,
         '        low: (n.low * 1_000_000_u32) / d.high,\n'
         '        high: (n.high * 1_000_000_u32) / d.low,'),
    )),
    ('execution', 'execute.rs', (
        ('premature_width', 'if measured_bits > 128 && measured_bits == interval.high.bits() {',
         'if measured_bits > 128 {'),
        ('quotient_sign_lost', 'value(magnitude, negative).map_err(FormulaTangentError::Numeric)',
         'value(magnitude, false).map_err(FormulaTangentError::Numeric)'),
        ('uncertified_representable_endpoint', 'if interval.low == interval.high {',
         'if interval.low == interval.high || interval.high.bits() <= 128 {'),
    )),
)

if __name__ == '__main__':
    for family, source, faults in FAMILIES:
        CONTROLLER['main'](dict(entry=Path(__file__),
            source=ROOT / 'crates/sc-core/src/recipe/tangent' / source,
            fixture=ROOT / 'crates/sc-core/tests/fixtures/formula_tangent.tsv',
            run=ROOT / ('target/tangent_body_faults/0636-' + family),
            rows=139, family='tangent-' + family, target='formula_tangent_oracle_contract',
            test='actual_upstream_sources_preserve_all_original_tangent_outcomes_through_signed_storage',
            faults=faults))
