//! Bounded exact arc-length rounding using the recorded pi midpoint-exclusion certificate.
use super::{
    pi::{PI_FRACTION_BITS, PI_LOWER_WORDS},
    rational::round_magnitude_wide,
    FormulaRational, FormulaRationalComponent, FormulaRationalError,
};
use num_bigint::BigUint;

/// Caller has proven Angle/Length kinds and completed scalar domains; retain signs and full turns.
pub(super) fn arc_length_value(
    angle: FormulaRational,
    radius: FormulaRational,
) -> Result<FormulaRational, FormulaRationalError> {
    if angle.is_zero() || radius.is_zero() {
        return Ok(FormulaRational::ZERO);
    }
    let numerator = BigUint::from(angle.numerator_magnitude())
        * BigUint::from(radius.numerator_magnitude())
        * BigUint::from_slice(&PI_LOWER_WORDS);
    let denominator = (BigUint::from(angle.denominator())
        * BigUint::from(radius.denominator())
        * BigUint::from(180_000_000_u32))
        << PI_FRACTION_BITS;
    // Every possible midpoint maps to a rational pi target of denominator <=2A<2^257.
    // The certificate proves none is between this endpoint and true pi, so their rounded answers
    // agree for every admitted input. This is not an approximate-value precision cutoff.
    let integer = round_magnitude_wide(numerator, denominator);
    let magnitude = narrow_arc(&integer)?;
    Ok(FormulaRational::from_integer_magnitude(
        magnitude,
        angle.is_negative() ^ radius.is_negative(),
    ))
}

// Completed operands and the certified fixed workspace bound the integer result to at most133 bits.
#[allow(clippy::cast_possible_truncation)]
fn narrow_arc(value: &BigUint) -> Result<u128, FormulaRationalError> {
    u128::try_from(value).map_err(|_| FormulaRationalError::RationalWidth {
        component: FormulaRationalComponent::Numerator,
        measured_bits: value.bits() as u32,
    })
}
