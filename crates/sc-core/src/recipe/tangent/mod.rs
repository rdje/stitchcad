//! Explicit-resource tangent preparation and certified unrounded adaptive numerical execution.
mod execute;
mod interval;
mod policy;
mod prepare;
pub use execute::{FormulaExecutedTangent, FormulaTangentExecutionError};
pub use policy::{
    FormulaTangentError, FormulaTangentLimits, FormulaTangentResource, FormulaTangentResourceError,
};

use super::FormulaNumericValue;
use core::fmt;
use num_bigint::BigUint;

const FOLD_BITS: usize = 156;
// I(128)=16*128+bit_length(257)+16; T(128)=4*128. These are scheduled costs,not precision limits.
const FIRST_BITS: usize = 2073;
const FIRST_TERMS: usize = 512;

// Exact folded metadata stays private and cannot be supplied as an alternative authored operand.
enum Reduction {
    Exact {
        magnitude: u32,
        negative: bool,
    },
    Quotient {
        phase: BigUint,
        negative: bool,
        reciprocal: bool,
    },
}

/// Actual admitted Angle and explicit tangent policy, prepared without evaluating any series.
/// No numerical value, storage conversion or source contribution is fabricated by preparation.
///
/// ```
/// use sc_core::recipe::{FormulaKind, FormulaNumericValue, FormulaRational, FormulaTangentLimits};
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let angle = FormulaNumericValue::new(FormulaKind::Angle, FormulaRational::ZERO)?;
/// let plan = angle.prepare_tangent(FormulaTangentLimits::new(156, 0))?;
/// assert_eq!(plan.input(), angle);
/// assert_eq!(plan.initial_series_terms(), 0);
/// # Ok(())
/// # }
/// ```
///
/// ```compile_fail
/// use sc_core::recipe::FormulaTangentPlan;
/// let _ = FormulaTangentPlan { input: todo!(), limits: todo!(), reduction: todo!() };
/// ```
pub struct FormulaTangentPlan {
    input: FormulaNumericValue,
    limits: FormulaTangentLimits,
    reduction: Reduction,
}
impl FormulaTangentPlan {
    /// Original completed Angle, preserving its full rational magnitude, sign and turns.
    #[must_use]
    pub const fn input(&self) -> FormulaNumericValue {
        self.input
    }

    /// Original explicit quotas, without a default or policy substitution.
    #[must_use]
    pub const fn limits(&self) -> FormulaTangentLimits {
        self.limits
    }

    /// Admitted initial integer reservation: folding alone, or the complete first iteration.
    #[must_use]
    pub const fn initial_integer_bits(&self) -> usize {
        match self.reduction {
            Reduction::Exact { .. } => FOLD_BITS,
            Reduction::Quotient { .. } => FIRST_BITS,
        }
    }

    /// Admitted first iteration's planned term charge. Preparation itself charges no series work.
    #[must_use]
    pub const fn initial_series_terms(&self) -> usize {
        match self.reduction {
            Reduction::Exact { .. } => 0,
            Reduction::Quotient { .. } => FIRST_TERMS,
        }
    }
}
impl fmt::Debug for FormulaTangentPlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = f.debug_struct("FormulaTangentPlan");
        out.field("input", &self.input)
            .field("limits", &self.limits);
        match &self.reduction {
            Reduction::Exact {
                magnitude,
                negative,
            } => {
                out.field("exact_magnitude", magnitude)
                    .field("negative", negative);
            }
            Reduction::Quotient {
                phase,
                negative,
                reciprocal,
            } => {
                out.field("folded_phase", phase)
                    .field("negative", negative)
                    .field("reciprocal", reciprocal);
            }
        }
        out.finish()
    }
}
