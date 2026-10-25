//! Canonical formula values with independent sign and full 128-bit component magnitudes.
//!
//! These values do not carry a formula kind, bind storage or evaluate syntax. Products and quotients
//! cross-cancel before checking reduced widths; sums use bounded private wider intermediates.

use super::FormulaRationalComponent;
use core::fmt;
use num_bigint::{BigInt, BigUint, Sign};

/// A refusal while constructing or computing a canonical formula rational value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaRationalError {
    /// A zero denominator does not define a rational, including when the numerator is zero.
    ZeroDenominator,
    /// A quotient's divisor is zero, including for `0 / 0`.
    ZeroDivisor,
    /// A component of the fully reduced result exceeds the fixed 128-bit magnitude bound.
    RationalWidth {
        /// The excessive reduced component.
        component: FormulaRationalComponent,
        /// Exact bit count of that component after full reduction.
        measured_bits: u32,
    },
}

impl FormulaRationalError {
    /// Stable diagnostic family; expression and statement context belongs to the later evaluator.
    #[must_use]
    pub const fn diagnostic_code(self) -> &'static str {
        match self {
            Self::ZeroDenominator | Self::ZeroDivisor => "formula_division",
            Self::RationalWidth { .. } => "formula_domain",
        }
    }

    /// The fixed reduced-component bound when this is a rational-width refusal.
    #[must_use]
    pub const fn rational_bit_bound(self) -> Option<u32> {
        match self {
            Self::RationalWidth { .. } => Some(128),
            _ => None,
        }
    }

    /// Stable structural-limit token when this is a rational-width refusal.
    #[must_use]
    pub const fn limit_token(self) -> Option<&'static str> {
        match self {
            Self::RationalWidth { .. } => Some("max_rational_bits"),
            _ => None,
        }
    }
}

impl fmt::Display for FormulaRationalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroDenominator => write!(f, "formula_division: zero rational denominator"),
            Self::ZeroDivisor => write!(f, "formula_division: zero rational divisor"),
            Self::RationalWidth {
                component,
                measured_bits,
            } => write!(
                f,
                "formula_domain: {component:?} max_rational_bits=128 measured_bits={measured_bits}"
            ),
        }
    }
}

impl std::error::Error for FormulaRationalError {}

/// An exact reduced value whose numerator magnitude and positive denominator fit `u128`.
///
/// The sign is independent of the numerator's magnitude: both signs admit all 128 magnitude bits.
/// Construction reduces by the greatest common divisor and canonicalizes zero to positive `0/1`.
/// Private immutable fields make structural equality and hashing agree with exact value equality.
/// This primitive applies no scalar domain, storage conversion or implicit rounding.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FormulaRational {
    numerator: u128,
    denominator: u128,
    negative: bool,
}

impl Default for FormulaRational {
    fn default() -> Self {
        Self::ZERO
    }
}

impl FormulaRational {
    /// Canonical positive zero, `0/1`.
    pub const ZERO: Self = Self {
        numerator: 0,
        denominator: 1,
        negative: false,
    };

    /// Canonical positive one, `1/1`.
    pub const ONE: Self = Self {
        numerator: 1,
        denominator: 1,
        negative: false,
    };

    /// Reduces `numerator / denominator`, applying a negative sign when `negative` is true.
    ///
    /// A zero numerator ignores the requested sign after denominator validation. No magnitude is
    /// narrowed to a signed integer, so values above `i128::MAX` remain admitted with either sign.
    ///
    /// # Errors
    ///
    /// Returns [`FormulaRationalError::ZeroDenominator`] when `denominator` is zero.
    pub const fn new(
        numerator: u128,
        denominator: u128,
        negative: bool,
    ) -> Result<Self, FormulaRationalError> {
        if denominator == 0 {
            return Err(FormulaRationalError::ZeroDenominator);
        }
        if numerator == 0 {
            return Ok(Self::ZERO);
        }
        let divisor = gcd(numerator, denominator);
        // Both inputs are positive here, so their gcd is nonzero and divides both exactly.
        Ok(Self {
            numerator: numerator / divisor,
            denominator: denominator / divisor,
            negative,
        })
    }

    /// Converts a signed integer exactly, including `i128::MIN`, without storage-domain checks.
    #[must_use]
    pub const fn from_signed_integer(value: i128) -> Self {
        Self {
            numerator: value.unsigned_abs(),
            denominator: 1,
            negative: value < 0,
        }
    }

    /// The unsigned numerator magnitude of the reduced form.
    #[must_use]
    pub const fn numerator_magnitude(self) -> u128 {
        self.numerator
    }

    /// The positive denominator of the reduced form.
    #[must_use]
    pub const fn denominator(self) -> u128 {
        self.denominator
    }

    /// Whether the value is negative; canonical zero always returns false.
    #[must_use]
    pub const fn is_negative(self) -> bool {
        self.negative
    }

    /// Whether the exact value is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.numerator == 0
    }

    /// Computes the exact product, cross-cancelling before judging the reduced component widths.
    ///
    /// # Errors
    ///
    /// Returns [`FormulaRationalError::RationalWidth`] when the reduced numerator or denominator
    /// requires more than 128 magnitude bits. No intermediate rounding or signed narrowing occurs.
    pub fn checked_mul(self, rhs: Self) -> Result<Self, FormulaRationalError> {
        if self.is_zero() || rhs.is_zero() {
            return Ok(Self::ZERO);
        }
        let left_cancel = gcd(self.numerator, rhs.denominator);
        let right_cancel = gcd(rhs.numerator, self.denominator);
        // Each input is reduced. Removing both cross-gcds makes every remaining numerator factor
        // coprime to every denominator factor, so these products are the fully reduced result.
        let numerator = checked_product(
            self.numerator / left_cancel,
            rhs.numerator / right_cancel,
            FormulaRationalComponent::Numerator,
        )?;
        let denominator = checked_product(
            self.denominator / right_cancel,
            rhs.denominator / left_cancel,
            FormulaRationalComponent::Denominator,
        )?;
        Ok(Self {
            numerator,
            denominator,
            negative: self.negative ^ rhs.negative,
        })
    }

    /// Computes the exact quotient, preserving canonical zero and cross-cancelling before limits.
    ///
    /// # Errors
    ///
    /// Returns [`FormulaRationalError::ZeroDivisor`] for a zero divisor, including `0 / 0`, or
    /// [`FormulaRationalError::RationalWidth`] for an excessive reduced result component.
    pub fn checked_div(self, rhs: Self) -> Result<Self, FormulaRationalError> {
        if rhs.is_zero() {
            return Err(FormulaRationalError::ZeroDivisor);
        }
        // A nonzero reduced value has a positive numerator magnitude. Its reciprocal is therefore
        // another admitted reduced value, and multiplication applies the same exact cross-cancellation.
        self.checked_mul(Self {
            numerator: rhs.denominator,
            denominator: rhs.numerator,
            negative: rhs.negative,
        })
    }

    /// Computes the exact sum and reduces wider intermediates before judging result widths.
    ///
    /// # Errors
    ///
    /// Returns [`FormulaRationalError::RationalWidth`] when a fully reduced component exceeds
    /// 128 magnitude bits. The numerator workspace is bounded to 257 bits by the admitted operands.
    pub fn checked_add(self, rhs: Self) -> Result<Self, FormulaRationalError> {
        self.checked_sum(rhs, false)
    }

    /// Computes the exact difference without rounding or prematurely narrowing cancellation.
    ///
    /// # Errors
    ///
    /// Returns [`FormulaRationalError::RationalWidth`] for an excessive fully reduced component.
    pub fn checked_sub(self, rhs: Self) -> Result<Self, FormulaRationalError> {
        self.checked_sum(rhs, true)
    }

    fn checked_sum(self, rhs: Self, subtract: bool) -> Result<Self, FormulaRationalError> {
        // Each cross-product has at most 256 bits; their signed sum has at most 257. No unbounded
        // integer enters this private workspace from a caller or from source text.
        let mut left = BigInt::from(self.numerator) * BigInt::from(rhs.denominator);
        let mut right = BigInt::from(rhs.numerator) * BigInt::from(self.denominator);
        if self.negative {
            left = -left;
        }
        if rhs.negative ^ subtract {
            right = -right;
        }
        let numerator = left + right;
        if numerator.sign() == Sign::NoSign {
            return Ok(Self::ZERO);
        }
        let negative = numerator.sign() == Sign::Minus;
        let denominator = BigUint::from(self.denominator) * BigUint::from(rhs.denominator);
        let divisor = gcd_wide(numerator.magnitude().clone(), denominator.clone());
        // The denominator is positive, so the gcd is nonzero. Only the fully reduced components
        // cross the u128 return boundary; wide intermediates cannot cause a false refusal.
        let reduced_numerator = numerator.magnitude() / &divisor;
        let reduced_denominator = denominator / divisor;
        Ok(Self {
            numerator: narrow_sum_component(
                &reduced_numerator,
                FormulaRationalComponent::Numerator,
            )?,
            denominator: narrow_sum_component(
                &reduced_denominator,
                FormulaRationalComponent::Denominator,
            )?,
            negative,
        })
    }
}

fn gcd_wide(mut lhs: BigUint, mut rhs: BigUint) -> BigUint {
    let zero = BigUint::from(0_u8);
    while rhs != zero {
        let remainder = &lhs % &rhs;
        lhs = rhs;
        rhs = remainder;
    }
    lhs
}

// Called only on reduced components from checked_sum: reduction cannot increase their maximum
// 257-bit width, so converting the bit count from u64 to u32 is lossless.
#[allow(clippy::cast_possible_truncation)]
fn narrow_sum_component(
    value: &BigUint,
    component: FormulaRationalComponent,
) -> Result<u128, FormulaRationalError> {
    u128::try_from(value).map_err(|_| FormulaRationalError::RationalWidth {
        component,
        measured_bits: value.bits() as u32,
    })
}

fn checked_product(
    lhs: u128,
    rhs: u128,
    component: FormulaRationalComponent,
) -> Result<u128, FormulaRationalError> {
    lhs.checked_mul(rhs)
        .ok_or_else(|| FormulaRationalError::RationalWidth {
            component,
            measured_bits: overflowing_product_bits(lhs, rhs),
        })
}

/// Exact bit count after checked multiplication has proved the product exceeds u128.
fn overflowing_product_bits(lhs: u128, rhs: u128) -> u32 {
    const MASK: u128 = (1_u128 << 64) - 1;
    let left_low = lhs & MASK;
    let right_low = rhs & MASK;
    let left_high = lhs >> 64;
    let right_high = rhs >> 64;
    let low = left_low * right_low;
    let cross = left_high * right_low + (low >> 64);
    let other_cross = left_low * right_high + (cross & MASK);
    let high = left_high * right_high + (cross >> 64) + (other_cross >> 64);
    // All factors/carries are at most 2^64-1. Even the final upper bound is
    // (2^64-1)^2 + 2*(2^64-1) = u128::MAX, so every partial operation fits u128.
    // `high` is the exact upper half and is nonzero because checked_mul already overflowed.
    2 * u128::BITS - high.leading_zeros()
}

const fn gcd(mut lhs: u128, mut rhs: u128) -> u128 {
    while rhs != 0 {
        let remainder = lhs % rhs;
        lhs = rhs;
        rhs = remainder;
    }
    lhs
}
