"""Actual native forward body faults through the shared exact local-workspace Cargo controller."""
from pathlib import Path
import runpy

ROOT = Path(__file__).resolve().parents[4]
DRIVER = Path(__file__).with_name('root_body_faults.py')
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
PROFILE['plan_producer'](ROOT, sources=(Path(__file__), DRIVER, PROFILE_SOURCE))
CONTROLLER = runpy.run_path(str(DRIVER))
CONFIG = dict(entry=Path(__file__), source=ROOT / 'crates/sc-core/src/recipe/trig.rs',
    fixture=ROOT / 'crates/sc-core/tests/fixtures/formula_sincos.tsv',
    run=ROOT / 'target/forward_body_faults/0632-verified-forward', rows=44, family='forward',
    target='formula_sincos_value_contract',
    test='actual_sourced_outputs_keep_upstream_calls_full_inputs_storage_and_t1_refusal',
    faults=(
        ('cosine_odd', 'let mut negative = angle.is_negative() && !cosine;',
         'let mut negative = angle.is_negative();'),
        ('half_period', 'let period = &quarter * 4_u8;', 'let period = &quarter * 2_u8;'),
        ('quarter_shift_lost', 'phase = (phase + &quarter) % &period;', 'phase %= &period;'),
        ('octant_complement_lost', 'phase = &quarter - phase;', 'phase = &phase + 0_u8;'),
        ('wrong_ppm_scale', 'const PPM: u32 = 1_000_000;', 'const PPM: u32 = 100_000;'),
        ('insufficient_series', 'const TERMS: u32 = 64;', 'const TERMS: u32 = 2;'),
        ('truncate_output', 'let integer = round_magnitude_wide(result.low * PPM, scale);',
         'let integer = (result.low * PPM) / scale;'),
    ))
if __name__ == '__main__':
    CONTROLLER['main'](CONFIG)
