use super::{
    interval::rounded_quotient, FormulaTangentError, FormulaTangentPlan, FormulaTangentResource,
    FormulaTangentResourceError, Reduction, FOLD_BITS,
};
use crate::recipe::{
    FormulaKind, FormulaNumericError, FormulaNumericValue, FormulaRational,
    FormulaRationalComponent, FormulaRationalError,
};
use core::fmt;

#[derive(Clone, Copy, Debug)]
struct Progress {
    precision: Option<usize>,
    reserved_integer_bits: usize,
    charged_series_terms: usize,
}

/// A certified native tangent result retaining the actual prepared operand and execution policy.
#[derive(Debug)]
pub struct FormulaExecutedTangent {
    plan: FormulaTangentPlan,
    value: FormulaNumericValue,
    progress: Progress,
}
impl FormulaExecutedTangent {
    /// Original completed operand,private reduction and selected quotas.
    #[must_use]
    pub const fn plan(&self) -> &FormulaTangentPlan {
        &self.plan
    }
    /// Correctly rounded Ratio in integer parts per million,without binding/storage conversion.
    #[must_use]
    pub const fn value(&self) -> FormulaNumericValue {
        self.value
    }
    /// Certifying dyadic precision,or None for an exact zero/unit case.
    #[must_use]
    pub const fn precision(&self) -> Option<usize> {
        self.progress.precision
    }
    /// Highest admitted integer reservation,not measured allocation storage.
    #[must_use]
    pub const fn reserved_integer_bits(&self) -> usize {
        self.progress.reserved_integer_bits
    }
    /// Cumulative charged pi/component-series work;exact cases charge zero.
    #[must_use]
    pub const fn charged_series_terms(&self) -> usize {
        self.progress.charged_series_terms
    }
}

/// Failed native tangent execution retaining its original plan,progress and exact cause.
/// No uncertain numerical value or accepted prefix is exposed.
#[derive(Debug)]
pub struct FormulaTangentExecutionError {
    plan: FormulaTangentPlan,
    cause: FormulaTangentError,
    progress: Progress,
}
impl FormulaTangentExecutionError {
    /// Actual original input and immutable selected policy.
    #[must_use]
    pub const fn plan(&self) -> &FormulaTangentPlan {
        &self.plan
    }
    /// Original mathematical or execution-resource cause.
    #[must_use]
    pub const fn cause(&self) -> FormulaTangentError {
        self.cause
    }
    /// Current certifying or next refused precision;None for an exact-case computation.
    #[must_use]
    pub const fn precision(&self) -> Option<usize> {
        self.progress.precision
    }
    /// Highest admitted reservation before this failure.
    #[must_use]
    pub const fn reserved_integer_bits(&self) -> usize {
        self.progress.reserved_integer_bits
    }
    /// Work charged before failure;refused iterations are never charged.
    #[must_use]
    pub const fn charged_series_terms(&self) -> usize {
        self.progress.charged_series_terms
    }
    /// Existing mathematical diagnostic,or None for resource admission.
    #[must_use]
    pub const fn diagnostic_code(&self) -> Option<&'static str> {
        self.cause.diagnostic_code()
    }
}
impl fmt::Display for FormulaTangentExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.cause, f)
    }
}
impl std::error::Error for FormulaTangentExecutionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.cause)
    }
}

impl FormulaTangentPlan {
    /// Executes fresh bounded unrounded enclosures until the complete interval certifies a result.
    /// Exact zero/unit cases execute no series. Every other complete iteration is preflighted
    /// before allocation/charge;an undecided interval refines or gives a separate resource refusal.
    ///
    /// # Errors
    /// Retains original plan/progress and numerical width or resource cause. A width refusal
    /// requires agreement on the actual rounded result's bit count,not an invented lower bound.
    ///
    /// ```
    /// use sc_core::recipe::{FormulaKind,FormulaNumericValue,FormulaRational,FormulaTangentLimits};
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let input = FormulaNumericValue::new(FormulaKind::Angle,
    ///     FormulaRational::from_signed_integer(-60_000_000))?;
    /// let result = input.prepare_tangent(FormulaTangentLimits::new(2073,512))?.evaluate()?;
    /// assert_eq!(result.value().rational(),FormulaRational::from_signed_integer(-1_732_051));
    /// assert_eq!(result.plan().input(),input);
    /// assert_eq!(result.charged_series_terms(),512);
    /// # Ok(())
    /// # }
    /// ```
    // Keep original errors/plans inline;an execution-resource refusal must not allocate a box.
    #[allow(clippy::result_large_err)]
    pub fn evaluate(self) -> Result<FormulaExecutedTangent, FormulaTangentExecutionError> {
        let mut progress = Progress {
            precision: None,
            reserved_integer_bits: FOLD_BITS,
            charged_series_terms: 0,
        };
        match compute(&self, &mut progress) {
            Ok(value) => Ok(FormulaExecutedTangent {
                plan: self,
                value,
                progress,
            }),
            Err(cause) => Err(FormulaTangentExecutionError {
                plan: self,
                cause,
                progress,
            }),
        }
    }
}

struct Iteration {
    next_precision: usize,
    integer_bits: usize,
    charged_terms: usize,
}

// Inline original rich errors avoid allocating while refusing resource admission.
#[allow(clippy::result_large_err)]
fn admit(
    plan: &FormulaTangentPlan,
    progress: Progress,
    bits: usize,
) -> Result<Iteration, FormulaTangentError> {
    let b = bits as u128;
    // All supported usize widths fit u128 extents;the next iteration follows an admitted16B bound.
    let requested_bits = 16 * b + u128::from(128 - (2 * b + 1).leading_zeros()) + 16;
    let refuse = |unit, requested| {
        FormulaTangentError::Resources(FormulaTangentResourceError {
            input: plan.input.rational(),
            limits: plan.limits,
            unit,
            precision: Some(bits),
            reserved_integer_bits: progress.reserved_integer_bits,
            charged_series_terms: progress.charged_series_terms,
            requested,
        })
    };
    if requested_bits > plan.limits.maximum_integer_bits() as u128 {
        return Err(refuse(FormulaTangentResource::IntegerBits, requested_bits));
    }
    // Complete native counters/factors,including the next precision,are checked before allocation.
    let (Some(next_precision), Some(terms), Ok(integer_bits)) = (
        bits.checked_mul(2),
        bits.checked_mul(4),
        usize::try_from(requested_bits),
    ) else {
        return Err(refuse(FormulaTangentResource::IntegerBits, requested_bits));
    };
    if terms > plan.limits.maximum_series_terms() - progress.charged_series_terms {
        return Err(refuse(FormulaTangentResource::SeriesTerms, terms as u128));
    }
    let Some(charged_terms) = progress.charged_series_terms.checked_add(terms) else {
        return Err(refuse(FormulaTangentResource::SeriesTerms, terms as u128));
    };
    Ok(Iteration {
        next_precision,
        integer_bits,
        charged_terms,
    })
}

// Ratio construction has no scalar bound;preserve its real numerical error if the contract changes.
fn value(magnitude: u128, negative: bool) -> Result<FormulaNumericValue, FormulaNumericError> {
    FormulaNumericValue::new(
        FormulaKind::Ratio,
        FormulaRational::from_integer_magnitude(magnitude, negative),
    )
}

// Refusal retains the original inline resource payload and performs no diagnostic allocation.
#[allow(clippy::result_large_err, clippy::cast_possible_truncation)]
fn compute(
    plan: &FormulaTangentPlan,
    progress: &mut Progress,
) -> Result<FormulaNumericValue, FormulaTangentError> {
    let (phase, negative, reciprocal) = match &plan.reduction {
        Reduction::Exact {
            magnitude,
            negative,
        } => return value(u128::from(*magnitude), *negative).map_err(FormulaTangentError::Numeric),
        Reduction::Quotient {
            phase,
            negative,
            reciprocal,
        } => (phase, *negative, *reciprocal),
    };
    let mut bits = 128;
    loop {
        progress.precision = Some(bits);
        let iteration = admit(plan, *progress, bits)?;
        progress.reserved_integer_bits = iteration.integer_bits;
        progress.charged_series_terms = iteration.charged_terms;
        let interval =
            rounded_quotient(phase, plan.input.rational().denominator(), reciprocal, bits);
        // The analytic floor and numerator<=1 bound every rounded endpoint by1M*2^128(148 bits).
        // Thus the exact agreed u64 bit count below fits u32 without truncation.
        let measured_bits = interval.low.bits();
        if measured_bits > 128 && measured_bits == interval.high.bits() {
            return Err(FormulaTangentError::Numeric(FormulaNumericError::Rational(
                FormulaRationalError::RationalWidth {
                    component: FormulaRationalComponent::Numerator,
                    measured_bits: measured_bits as u32,
                },
            )));
        }
        if interval.low == interval.high {
            return u128::try_from(&interval.low)
                .map_err(|_| {
                    FormulaTangentError::Numeric(FormulaNumericError::Rational(
                        FormulaRationalError::RationalWidth {
                            component: FormulaRationalComponent::Numerator,
                            measured_bits: measured_bits as u32,
                        },
                    ))
                })
                .and_then(|magnitude| {
                    value(magnitude, negative).map_err(FormulaTangentError::Numeric)
                });
        }
        bits = iteration.next_precision;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::recipe::FormulaTangentLimits;

    #[test]
    #[allow(clippy::expect_used)]
    fn excessive_native_extent_refuses_before_primitive_counters_or_integer_allocation() {
        let input = FormulaNumericValue::new(
            FormulaKind::Angle,
            FormulaRational::from_signed_integer(30_000_000),
        )
        .expect("Angle");
        let plan = input
            .prepare_tangent(FormulaTangentLimits::new(usize::MAX, usize::MAX))
            .expect("prepared");
        let prior = Progress {
            precision: None,
            reserved_integer_bits: FOLD_BITS,
            charged_series_terms: 0,
        };
        let error = admit(&plan, prior, usize::MAX).err().expect("wide extent");
        let FormulaTangentError::Resources(resource) = error else {
            unreachable!()
        };
        assert_eq!(resource.unit(), FormulaTangentResource::IntegerBits);
        assert!(resource.requested() > usize::MAX as u128);
        assert_eq!(resource.precision(), Some(usize::MAX));
        assert_eq!(resource.charged_series_terms(), 0);
        assert_eq!(resource.reserved_integer_bits(), FOLD_BITS);
    }
}
