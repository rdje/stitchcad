//! The one rounding rule and the one conversion rule (spec §2).
//!
//! Both are implemented here, once, so that no call site can invent a variant: rounding is **half
//! away from zero**, and a conversion is a **single multiply followed by a single divide** on an
//! exact integer ratio. Chained conversions are forbidden because they round more than once and
//! disagree with the direct conversion.

use crate::error::UnitError;

/// Rounds `numerator / denominator` half away from zero.
///
/// `+0.5` rounds to `+1` and `-0.5` to `-1`. The rule is symmetric about zero, which is what makes
/// mirroring a piece and then rounding equal rounding and then mirroring (spec §2, and a conformance
/// requirement in §9).
///
/// # Errors
///
/// Returns [`UnitError::DivisionByZero`] when `denominator` is zero, and [`UnitError::Overflow`]
/// when the rounded quotient does not fit an `i64`.
pub fn div_round_half_away_from_zero(numerator: i128, denominator: i128) -> Result<i64, UnitError> {
    if denominator == 0 {
        return Err(UnitError::DivisionByZero {
            operation: "div_round_half_away_from_zero",
        });
    }
    // Work in magnitudes so the half-way rule cannot depend on the sign convention of `/` and `%`.
    let negative = (numerator < 0) != (denominator < 0);
    let n = numerator.unsigned_abs();
    let d = denominator.unsigned_abs();
    let mut q = n / d;
    let r = n % d;
    // half away from zero: a remainder of exactly half, or more, moves the magnitude up by one.
    if r * 2 >= d {
        q += 1;
    }
    // The negative endpoint has one more unit of magnitude than i64::MAX. Handle it without
    // ever casting a wider unsigned quotient to a signed type: 2^127 is a valid i128 magnitude,
    // but casting it first would create i128::MIN and negating that value would panic.
    if negative && q == u128::from(i64::MIN.unsigned_abs()) {
        return Ok(i64::MIN);
    }
    let magnitude = i64::try_from(q).map_err(|_| UnitError::Overflow {
        operation: "div_round_half_away_from_zero",
    })?;
    // Checked conversion proved magnitude <= i64::MAX, so its negation is representable.
    Ok(if negative { -magnitude } else { magnitude })
}

#[cfg(test)]
mod tests {
    use super::div_round_half_away_from_zero as round;
    use crate::UnitError;

    #[test]
    fn half_moves_away_from_zero_in_both_directions() {
        assert_eq!(round(1, 2).unwrap(), 1);
        assert_eq!(round(-1, 2).unwrap(), -1);
        assert_eq!(round(3, 2).unwrap(), 2);
        assert_eq!(round(-3, 2).unwrap(), -2);
    }

    #[test]
    fn below_half_moves_toward_zero() {
        assert_eq!(round(1, 3).unwrap(), 0);
        assert_eq!(round(-1, 3).unwrap(), 0);
        assert_eq!(round(2, 5).unwrap(), 0);
    }

    #[test]
    fn a_negative_denominator_does_not_flip_the_rule() {
        assert_eq!(round(1, -2).unwrap(), -1);
        assert_eq!(round(-1, -2).unwrap(), 1);
    }

    #[test]
    fn division_by_zero_is_a_diagnostic_not_a_panic() {
        assert_eq!(
            round(1, 0).unwrap_err(),
            UnitError::DivisionByZero {
                operation: "div_round_half_away_from_zero"
            }
        );
    }

    #[test]
    fn a_quotient_beyond_i64_is_a_diagnostic_not_a_wrap() {
        let err = round(i128::MAX, 1).unwrap_err();
        assert!(matches!(err, UnitError::Overflow { .. }), "got {err:?}");
    }
}
