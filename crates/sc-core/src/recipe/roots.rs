//! Exact rounded roots in a private, input-bounded integer workspace.
use super::{FormulaRational, FormulaRationalComponent, FormulaRationalError};
use num_bigint::BigUint;

/// The caller has checked the sqrt signature and refused a negative completed value.
pub(super) fn sqrt_value(
    value: FormulaRational,
    ratio: bool,
) -> Result<FormulaRational, FormulaRationalError> {
    let scale = if ratio { 1_000_000_u128 } else { 1 };
    rounded_root(
        BigUint::from(value.numerator_magnitude()) * BigUint::from(scale),
        BigUint::from(value.denominator()),
    )
}

fn rounded_root(
    numerator: BigUint,
    denominator: BigUint,
) -> Result<FormulaRational, FormulaRationalError> {
    // Canonical operands guarantee a positive denominator. An integer square is <= N/D exactly
    // when it is <= floor(N/D), so this finite bisection also finds floor(sqrt(N/D)).
    let quotient = &numerator / &denominator;
    let one = BigUint::from(1_u8);
    let mut lower = BigUint::from(0_u8);
    let mut upper = &one << ((quotient.bits() + 1) / 2);
    while &upper - &lower > one {
        let midpoint = (&lower + &upper) >> 1_u8;
        if &midpoint * &midpoint <= quotient {
            lower = midpoint;
        } else {
            upper = midpoint;
        }
    }
    // sqrt(N/D) >= lower+1/2 iff 4N >= D*(2lower+1)^2. Equality rounds away from zero.
    let midpoint_twice = (&lower << 1_u8) + &one;
    if (numerator << 2_u8) >= denominator * &midpoint_twice * &midpoint_twice {
        lower += one;
    }
    let magnitude = narrow_root(&lower)?;
    Ok(FormulaRational::from_integer_magnitude(magnitude, false))
}

// All callers satisfy the sealed root-family plan: sqrt's largest integer result is <=75 bits.
// This also bounds the diagnostic bit-count conversion, even though narrowing never fails there.
#[allow(clippy::cast_possible_truncation)]
fn narrow_root(value: &BigUint) -> Result<u128, FormulaRationalError> {
    u128::try_from(value).map_err(|_| FormulaRationalError::RationalWidth {
        component: FormulaRationalComponent::Numerator,
        measured_bits: value.bits() as u32,
    })
}
