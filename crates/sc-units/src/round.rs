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
    // Work in magnitudes so the half-way rule cannot depend on the sign convention of `/` and `%`.
    let negative = (numerator < 0) != (denominator < 0);
    let n = numerator.unsigned_abs();
    let d = denominator.unsigned_abs();
    let q = round_magnitude(n, d, "div_round_half_away_from_zero")?;
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

/// Rounds a nonnegative exact ratio to a full-width unsigned magnitude, half away from zero.
///
/// This retains all 128 magnitude bits, including values wider than signed i128 or i64. It does
/// not impose a quantity's scalar domain or a binding's storage width. Formula literal conversion
/// can therefore round a positive child beneath unary minus without folding the operator or
/// prematurely narrowing the value. The signed API uses the same magnitude rule and checks its
/// signed i64 result separately.
///
/// For a nonzero denominator every u128 input pair has a representable rounded u128 result:
/// denominator one has no remainder; larger denominators leave room to increment the quotient.
///
/// # Errors
///
/// Returns [`UnitError::DivisionByZero`] when `denominator` is zero.
///
/// ```
/// use sc_units::round::div_round_half_away_from_zero_unsigned as round;
/// assert_eq!(round(1, 2)?, 1);
/// assert_eq!(round(u128::MAX, 1)?, u128::MAX);
/// assert_eq!(round(u128::MAX - 1, u128::MAX)?, 1);
/// # Ok::<(), sc_units::UnitError>(())
/// ```
pub fn div_round_half_away_from_zero_unsigned(
    numerator: u128,
    denominator: u128,
) -> Result<u128, UnitError> {
    round_magnitude(
        numerator,
        denominator,
        "div_round_half_away_from_zero_unsigned",
    )
}

fn round_magnitude(
    numerator: u128,
    denominator: u128,
    operation: &'static str,
) -> Result<u128, UnitError> {
    if denominator == 0 {
        return Err(UnitError::DivisionByZero { operation });
    }
    let mut q = numerator / denominator;
    let r = numerator % denominator;
    // r < denominator, so subtraction is representable. Unlike 2*r, this comparison cannot
    // overflow at a full-width unsigned denominator, and it treats exact halves identically.
    if r >= denominator - r {
        q = q.checked_add(1).ok_or(UnitError::Overflow { operation })?;
    }
    Ok(q)
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
