"""Actual native inverse body faults through the shared exact local-workspace Cargo controller."""
from pathlib import Path
import runpy

ROOT = Path(__file__).resolve().parents[4]
DRIVER = Path(__file__).with_name('root_body_faults.py')
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
PROFILE['plan_producer'](ROOT, sources=(Path(__file__), DRIVER, PROFILE_SOURCE))
CONTROLLER = runpy.run_path(str(DRIVER))
CONFIG = dict(entry=Path(__file__), source=ROOT / 'crates/sc-core/src/recipe/inverse.rs',
    fixture=ROOT / 'crates/sc-core/tests/fixtures/formula_inverse.tsv',
    run=ROOT / 'target/inverse_body_faults/0634-workspace', rows=81, family='inverse',
    target='formula_inverse_value_contract',
    test='actual_sourced_inverse_rows_preserve_whole_inputs_signed_storage_and_original_refusals',
    faults=(
        ('atan_ppm_scale_lost', 'BigUint::from(value.denominator()) * 1_000_000_u32,',
         'BigUint::from(value.denominator()),'),
        ('atan_sign_lost', 'negative: value.is_negative(),', 'negative: false,'),
        ('atan2_y_denominator_lost', 'BigUint::from(x.numerator_magnitude()) * y.denominator(),',
         'BigUint::from(x.numerator_magnitude()),'),
        ('negative_x_quadrant', 'complement(phase, &(scale() / 2_u8))',
         'complement(phase, &(scale() / 4_u8))'),
        ('atan2_sign_from_x', 'negative: y.is_negative(),', 'negative: x.is_negative(),'),
        ('reciprocal_complement_axis', 'complement(phase, &(&s / 4_u8))',
         'complement(phase, &(&s / 2_u8))'),
        ('half_angle_count', 'const HALF_ANGLES: u32 = 5;', 'const HALF_ANGLES: u32 = 4;'),
        ('insufficient_series', 'const TERMS: u32 = 64;', 'const TERMS: u32 = 2;'),
        ('truncate_inverse',
         'let integer = round_magnitude_wide(self.magnitude.low * 360_000_000_u32, scale());',
         'let integer = (self.magnitude.low * 360_000_000_u32) / scale();'),
    ))
if __name__ == '__main__':
    CONTROLLER['main'](CONFIG)
