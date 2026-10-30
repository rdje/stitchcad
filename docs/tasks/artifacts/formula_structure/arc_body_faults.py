"""Actual native arc body faults through the shared exact local-workspace Cargo controller."""
from pathlib import Path
import runpy

ROOT = Path(__file__).resolve().parents[4]
DRIVER = Path(__file__).with_name('root_body_faults.py')
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
PROFILE['plan_producer'](ROOT, sources=(Path(__file__), DRIVER, PROFILE_SOURCE))
CONTROLLER = runpy.run_path(str(DRIVER))
CONFIG = dict(entry=Path(__file__), source=ROOT / 'crates/sc-core/src/recipe/arc.rs',
    fixture=ROOT / 'crates/sc-core/tests/fixtures/formula_arcs.tsv',
    run=ROOT / 'target/arc_body_faults/0632-exact-arc', rows=23, family='arc',
    target='formula_arc_value_contract',
    test='actual_sourced_arcs_keep_signed_turns_original_refusals_and_real_upstream_storage_flags',
    faults=(
        ('product_sign_or', 'angle.is_negative() ^ radius.is_negative()',
         'angle.is_negative() || radius.is_negative()'),
        ('discard_full_turns', 'let numerator = BigUint::from(angle.numerator_magnitude())',
         'let numerator = (BigUint::from(angle.numerator_magnitude()) % (BigUint::from(angle.denominator()) * 360_000_000_u32))'),
        ('wrong_pi_scale', '<< PI_FRACTION_BITS;', '<< (PI_FRACTION_BITS + 1);'),
        ('radius_lost', '* BigUint::from(radius.numerator_magnitude())', '* BigUint::from(1_u8)'),
        ('truncate_arc', 'let integer = round_magnitude_wide(numerator, denominator);',
         'let integer = numerator / denominator;'),
    ))
if __name__ == '__main__':
    CONTROLLER['main'](CONFIG)
