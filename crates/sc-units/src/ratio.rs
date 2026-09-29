//! Dimensionless ratios and counts (spec §1.3).

use core::fmt;

use crate::error::UnitError;
use crate::length::Length;

/// Parts per million in a ratio of exactly 1.0.
pub const PPM_UNITY: i64 = 1_000_000;

/// A dimensionless multiplier stored as parts per million, so that ease ratios, shrinkage
/// percentages and grade multipliers are exact integers rather than floats.
///
/// `1.0` is `1 000 000` ppm; a 2 % shrinkage factor of `1.02` is `1 020 000` ppm.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ratio(i64);

impl Ratio {
    /// The ratio 1.0.
    pub const UNITY: Self = Self(PPM_UNITY);

    /// Builds a ratio from parts per million.
    #[must_use]
    pub const fn from_parts_per_million(ppm: i64) -> Self {
        Self(ppm)
    }

    /// Builds a ratio from an exact rational **percentage value**: `(2, 1)` is 2 %, which is a
    /// multiplier of `0.02`.
    ///
    /// ⚠ A percentage and a multiplier are different numbers, and confusing them is not a rounding
    /// error — it is a pattern piece scaled to nothing. A 2 % fabric shrinkage is expressed as the
    /// **multiplier** `1.02`, i.e. [`Ratio::from_rational(102, 100)`](Self::from_rational), *not* as
    /// `Ratio::from_percent(2, 1)`. Use this constructor when the quantity you hold really is a
    /// percentage: an ease allowance of "4 % of the armscye", a grade break of "1.5 % per size".
    ///
    /// # Errors
    ///
    /// Returns [`UnitError::DivisionByZero`] when `denominator` is zero and [`UnitError::Overflow`]
    /// when the intermediate value does not fit.
    pub fn from_percent(numerator: i64, denominator: i64) -> Result<Self, UnitError> {
        if denominator == 0 {
            return Err(UnitError::DivisionByZero {
                operation: "Ratio::from_percent",
            });
        }
        let n = i128::from(numerator) * i128::from(PPM_UNITY);
        let d = i128::from(denominator) * 100;
        let ppm = crate::round::div_round_half_away_from_zero(n, d)?;
        Ok(Self(ppm))
    }

    /// Builds a ratio from an exact rational **multiplier**: `(3, 2)` is 1.5, `(102, 100)` is the
    /// 1.02 a 2 % shrinkage is applied as.
    ///
    /// This is the constructor a scaling transformation should use, because a profile parameter such
    /// as `shrinkage_xy_pct` is stored as a percentage and converted once, here, at the boundary.
    ///
    /// # Errors
    ///
    /// Returns [`UnitError::DivisionByZero`] when `denominator` is zero and [`UnitError::Overflow`]
    /// when the intermediate value does not fit.
    pub fn from_rational(numerator: i64, denominator: i64) -> Result<Self, UnitError> {
        if denominator == 0 {
            return Err(UnitError::DivisionByZero {
                operation: "Ratio::from_rational",
            });
        }
        let n = i128::from(numerator) * i128::from(PPM_UNITY);
        let d = i128::from(denominator);
        let ppm = crate::round::div_round_half_away_from_zero(n, d)?;
        Ok(Self(ppm))
    }

    /// The parts-per-million value.
    #[must_use]
    pub const fn as_parts_per_million(self) -> i64 {
        self.0
    }

    /// Scales a length by this ratio, rounding half away from zero.
    ///
    /// This is how shrinkage and grade multipliers are applied — as an explicit, single, rounded
    /// operation at a declared stage of the pipeline, never as a floating-point factor folded into a
    /// coordinate.
    ///
    /// # Errors
    ///
    /// Returns [`UnitError::Overflow`] or [`UnitError::DomainExceeded`] when the result leaves the
    /// declared length domain.
    pub fn scale(self, length: Length) -> Result<Length, UnitError> {
        let n = i128::from(length.as_micrometres()) * i128::from(self.0);
        let d = i128::from(PPM_UNITY);
        let um = crate::round::div_round_half_away_from_zero(n, d)?;
        Length::from_micrometres(um)
    }
}

impl fmt::Display for Ratio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ppm", self.0)
    }
}

/// A count of discrete things: pieces, plies, notches, stitches.
///
/// A count is not a length and cannot become one: the type system rejects the arithmetic that would
/// confuse them (spec §1.3).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Count(u32);

impl Count {
    /// Builds a count.
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// The count.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

impl fmt::Display for Count {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
