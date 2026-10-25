//! Canonical formula values with independent sign and full 128-bit component magnitudes.
//!
//! These values do not carry a formula kind, bind storage or evaluate syntax. Arithmetic and
//! its wider intermediate workspace are separate from this construction boundary.

use core::fmt;

/// A refusal while constructing a canonical formula rational value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaRationalError {
    /// A zero denominator does not define a rational, including when the numerator is zero.
    ZeroDenominator,
}

impl FormulaRationalError {
    /// Stable diagnostic family; expression and statement context belongs to the later evaluator.
    #[must_use]
    pub const fn diagnostic_code(self) -> &'static str {
        match self {
            Self::ZeroDenominator => "formula_division",
        }
    }
}

impl fmt::Display for FormulaRationalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: zero rational denominator", self.diagnostic_code())
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
}

const fn gcd(mut lhs: u128, mut rhs: u128) -> u128 {
    while rhs != 0 {
        let remainder = lhs % rhs;
        lhs = rhs;
        rhs = remainder;
    }
    lhs
}
