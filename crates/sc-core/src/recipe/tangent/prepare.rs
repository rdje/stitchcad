use super::{
    FormulaTangentError, FormulaTangentLimits, FormulaTangentPlan, FormulaTangentResource,
    FormulaTangentResourceError, Reduction, FIRST_BITS, FIRST_TERMS, FOLD_BITS,
};
use crate::recipe::{FormulaBuiltin, FormulaKind, FormulaNumericError, FormulaNumericValue};
use num_bigint::BigUint;

impl FormulaNumericValue {
    /// Prepares the actual Angle for tangent under explicit immutable execution quotas.
    ///
    /// Kind validation precedes resource admission; the156-bit fold reservation precedes all
    /// period arithmetic. Exact endpoints require no series. Other inputs preflight the complete
    /// first128-bit iteration, without charging or executing its pi/component terms.
    ///
    /// # Errors
    /// Returns the original wrong-kind cause, an exact pole, or a rich resource refusal with no
    /// numerical value or invented formula-language diagnostic. No result rounding happens here.
    // Preserve original input/policy/counters inline: refusing resources must not allocate a box.
    #[allow(clippy::result_large_err)]
    pub fn prepare_tangent(
        self,
        limits: FormulaTangentLimits,
    ) -> Result<FormulaTangentPlan, FormulaTangentError> {
        if self.kind() != FormulaKind::Angle {
            return Err(FormulaTangentError::Numeric(
                FormulaNumericError::BuiltinInputKind {
                    builtin: FormulaBuiltin::Tan,
                    argument: 0,
                    actual: self.kind(),
                },
            ));
        }
        let refusal = |unit, precision, reserved_integer_bits, requested| {
            FormulaTangentError::Resources(FormulaTangentResourceError {
                input: self.rational(),
                limits,
                unit,
                precision,
                reserved_integer_bits,
                charged_series_terms: 0,
                requested,
            })
        };
        if limits.maximum_integer_bits() < FOLD_BITS {
            return Err(refusal(
                FormulaTangentResource::IntegerBits,
                None,
                0,
                FOLD_BITS as u128,
            ));
        }
        // q <=90M*(2^128-1), period=2q <2^156; phase and doubled phase fit that reservation.
        let input = self.rational();
        let quarter = BigUint::from(input.denominator()) * 90_000_000_u32;
        let period = &quarter * 2_u8;
        let mut phase = BigUint::from(input.numerator_magnitude()) % &period;
        let mut negative = input.is_negative();
        if phase > quarter {
            phase = &period - phase;
            negative = !negative;
        }
        if phase == quarter {
            return Err(FormulaTangentError::Pole { input });
        }
        let twice = &phase * 2_u8;
        let reduction = if phase.bits() == 0 {
            Reduction::Exact {
                magnitude: 0,
                negative: false,
            }
        } else if twice == quarter {
            Reduction::Exact {
                magnitude: 1_000_000,
                negative,
            }
        } else {
            let reciprocal = twice > quarter;
            if reciprocal {
                phase = quarter - phase;
            }
            // Both demands fit before any iteration charge or admitted iteration reservation.
            if limits.maximum_integer_bits() < FIRST_BITS {
                return Err(refusal(
                    FormulaTangentResource::IntegerBits,
                    Some(128),
                    FOLD_BITS,
                    FIRST_BITS as u128,
                ));
            }
            if limits.maximum_series_terms() < FIRST_TERMS {
                return Err(refusal(
                    FormulaTangentResource::SeriesTerms,
                    Some(128),
                    FOLD_BITS,
                    FIRST_TERMS as u128,
                ));
            }
            Reduction::Quotient {
                phase,
                negative,
                reciprocal,
            }
        };
        Ok(FormulaTangentPlan {
            input: self,
            limits,
            reduction,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::recipe::FormulaRational;

    #[test]
    #[allow(clippy::expect_used)]
    fn actual_fold_matches_independent_signed_centered_residues() {
        let limits = FormulaTangentLimits::new(FIRST_BITS, FIRST_TERMS);
        let mut cases = 0;
        for d in [1_i128, 3, 7] {
            for degree in (-720_i128..=720).step_by(15) {
                for offset in [-1_i128, 0, 1] {
                    let signed = degree * 1_000_000 * d + offset;
                    let original =
                        FormulaRational::new(signed.unsigned_abs(), d.unsigned_abs(), signed < 0)
                            .expect("small independent input");
                    let actual = FormulaNumericValue::new(FormulaKind::Angle, original)
                        .expect("Angle")
                        .prepare_tangent(limits);
                    let q = 90_000_000 * d;
                    let centered = (signed + q).rem_euclid(2 * q) - q;
                    if centered == -q {
                        assert_eq!(
                            actual.expect_err("pole"),
                            FormulaTangentError::Pole { input: original }
                        );
                    } else {
                        let plan = actual.expect("nonpole prepared");
                        let magnitude = centered.unsigned_abs();
                        let quarter = q.unsigned_abs();
                        let negative = centered < 0;
                        match plan.reduction {
                            Reduction::Exact {
                                magnitude: actual,
                                negative: sign,
                            } => {
                                assert!(magnitude == 0 || 2 * magnitude == quarter);
                                assert_eq!(actual, if magnitude == 0 { 0 } else { 1_000_000 });
                                assert_eq!(sign, negative);
                            }
                            Reduction::Quotient {
                                phase,
                                negative: sign,
                                reciprocal,
                            } => {
                                assert!(magnitude != 0 && 2 * magnitude != quarter);
                                let folded = magnitude.min(quarter - magnitude);
                                // Canonical input reduction scales both phase and its denominator.
                                let common = d.unsigned_abs() / original.denominator();
                                assert_eq!(phase * common, BigUint::from(folded));
                                assert_eq!(sign, negative);
                                assert_eq!(reciprocal, 2 * magnitude > quarter);
                            }
                        }
                    }
                    cases += 1;
                }
            }
        }
        assert_eq!(cases, 873);
    }
}
