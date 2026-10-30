"""Actual tangent quota/stage/progress faults against the independent committed resource cohort."""
from pathlib import Path
import runpy

ROOT = Path(__file__).resolve().parents[4]
DRIVER = Path(__file__).with_name('root_body_faults.py')
PROFILE_SOURCE = ROOT / 'scripts/local_environment.py'
PROFILE = runpy.run_path(str(PROFILE_SOURCE))
PROFILE['plan_producer'](ROOT, sources=(Path(__file__), DRIVER, PROFILE_SOURCE))
CONTROLLER = runpy.run_path(str(DRIVER))
FAMILIES = (
    ('prepare', 'prepare.rs', (
        ('kind_after_quota', 'if self.kind() != FormulaKind::Angle {',
         'if self.kind() != FormulaKind::Angle && limits.maximum_integer_bits() >= FOLD_BITS {'),
        ('fold_guard_lost', 'if limits.maximum_integer_bits() < FOLD_BITS {', 'if false {'),
        ('first_bits_guard_lost', 'if limits.maximum_integer_bits() < FIRST_BITS {', 'if false {'),
        ('first_terms_guard_lost', 'if limits.maximum_series_terms() < FIRST_TERMS {', 'if false {'),
        ('first_work_before_bits', 'if limits.maximum_integer_bits() < FIRST_BITS {',
         'if limits.maximum_integer_bits() < FIRST_BITS && limits.maximum_series_terms() >= FIRST_TERMS {'),
        ('preparation_refusal_charged', 'charged_series_terms: 0,', 'charged_series_terms: 512,'),
    )),
    ('execute', 'execute.rs', (
        ('later_bits_guard_lost', 'if requested_bits > plan.limits.maximum_integer_bits() as u128 {',
         'if false {'),
        ('later_terms_guard_lost',
         'if terms > plan.limits.maximum_series_terms() - progress.charged_series_terms {', 'if false {'),
        ('later_work_before_bits', 'if requested_bits > plan.limits.maximum_integer_bits() as u128 {',
         'if requested_bits > plan.limits.maximum_integer_bits() as u128 && 4 * b <= '
         'plan.limits.maximum_series_terms().saturating_sub(progress.charged_series_terms) as u128 {'),
        ('refused_iteration_precharged', 'let iteration = admit(plan, *progress, bits)?;',
         'if bits > 128 { progress.charged_series_terms += 4 * bits; }\n'
         '        let iteration = admit(plan, *progress, bits)?;'),
        ('refusal_claims_proposed_reservation', 'reserved_integer_bits: progress.reserved_integer_bits,',
         'reserved_integer_bits: requested_bits as usize,'),
        ('cumulative_charge_lost', 'progress.charged_series_terms.checked_add(terms)',
         '0_usize.checked_add(terms)'),
        ('series_charge_halved', 'bits.checked_mul(4),', 'bits.checked_mul(2),'),
    )),
)

if __name__ == '__main__':
    for family, source, faults in FAMILIES:
        CONTROLLER['main'](dict(entry=Path(__file__),
            source=ROOT / 'crates/sc-core/src/recipe/tangent' / source,
            fixture=ROOT / 'crates/sc-core/tests/fixtures/formula_tangent_resources.tsv',
            run=ROOT / ('target/tangent_resource_body_faults/0638-' + family),
            rows=33, family='tangent-resource-' + family, target='formula_tangent_resource_contract',
            test='independent_resource_rows_preserve_actual_stages_limits_demands_and_prior_progress',
            faults=faults))
