//! Angles: fixed-point microdegrees (spec §1.2).

use core::fmt;
use core::ops::{Add, Neg, Sub};

use crate::error::UnitError;

/// Microdegrees in one full turn.
pub const MICRODEGREES_PER_TURN: i64 = 360_000_000;

/// Microdegrees in one degree.
pub const MICRODEGREES_PER_DEGREE: i64 = 1_000_000;

/// An angle, stored as a signed count of **microdegrees**, normalized to `[0, 360°)`.
///
/// Microdegrees rather than radians because the domain speaks in degrees — grain deviation, dart
/// intake, bias at 45°, notch direction — and because a rational number of degrees is stored exactly,
/// while π-based storage would make the commonest angles approximate. Radians remain the currency of
/// trigonometric *evaluation*, at the boundary only.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Angle(i64);

impl Angle {
    /// The zero angle.
    pub const ZERO: Self = Self(0);

    /// A right angle.
    pub const RIGHT: Self = Self(90 * MICRODEGREES_PER_DEGREE);

    /// A straight angle.
    pub const STRAIGHT: Self = Self(180 * MICRODEGREES_PER_DEGREE);

    /// Builds an angle from microdegrees, normalizing into `[0, 360°)`.
    ///
    /// Normalization is part of the stored form (spec §1.2), so two angles that differ by whole turns
    /// compare equal without a caller remembering to normalize first.
    #[must_use]
    pub const fn from_microdegrees(microdegrees: i64) -> Self {
        Self(microdegrees.rem_euclid(MICRODEGREES_PER_TURN))
    }

    /// Builds an angle from an exact rational number of degrees, rounding half away from zero.
    ///
    /// # Errors
    ///
    /// Returns [`UnitError::DivisionByZero`] when `denominator` is zero and [`UnitError::Overflow`]
    /// when the intermediate value does not fit.
    pub fn from_degrees_rational(numerator: i64, denominator: i64) -> Result<Self, UnitError> {
        if denominator == 0 {
            return Err(UnitError::DivisionByZero {
                operation: "Angle::from_degrees_rational",
            });
        }
        let n = i128::from(numerator) * i128::from(MICRODEGREES_PER_DEGREE);
        let d = i128::from(denominator);
        let micro = crate::round::div_round_half_away_from_zero(n, d)?;
        Ok(Self::from_microdegrees(micro))
    }

    /// The normalized microdegree count.
    #[must_use]
    pub const fn as_microdegrees(self) -> i64 {
        self.0
    }

    /// This angle in degrees as an `f64` — boundary only, never stored back.
    #[must_use]
    pub fn as_degrees_f64(self) -> f64 {
        #[allow(clippy::cast_precision_loss)] // documented boundary conversion
        let microdegrees = self.0 as f64;
        #[allow(clippy::cast_precision_loss)] // documented boundary conversion
        let per_degree = MICRODEGREES_PER_DEGREE as f64;
        microdegrees / per_degree
    }

    /// This angle in radians as an `f64`, for trigonometric evaluation at a boundary.
    #[must_use]
    pub fn as_radians_f64(self) -> f64 {
        self.as_degrees_f64().to_radians()
    }

    /// The sine of this angle, evaluated in `f64` (boundary only).
    #[must_use]
    pub fn sin(self) -> f64 {
        self.as_radians_f64().sin()
    }

    /// The cosine of this angle, evaluated in `f64` (boundary only).
    #[must_use]
    pub fn cos(self) -> f64 {
        self.as_radians_f64().cos()
    }

    /// The smallest turn between two angles, in `[0, 180°]`.
    ///
    /// This is what "is this grainline within 1° of vertical?" means; the raw difference of two
    /// normalized angles is not, because it wraps.
    #[must_use]
    pub fn difference(self, other: Self) -> Self {
        let raw = (self.0 - other.0).rem_euclid(MICRODEGREES_PER_TURN);
        let half = MICRODEGREES_PER_TURN / 2;
        Self(if raw > half {
            MICRODEGREES_PER_TURN - raw
        } else {
            raw
        })
    }

    /// Adds two angles, normalizing the result.
    #[must_use]
    pub const fn add_angle(self, other: Self) -> Self {
        Self::from_microdegrees(self.0 + other.0)
    }

    /// Subtracts two angles, normalizing the result.
    #[must_use]
    pub const fn sub_angle(self, other: Self) -> Self {
        Self::from_microdegrees(self.0 - other.0)
    }

    /// Reverses the direction of this angle.
    #[must_use]
    pub const fn reversed(self) -> Self {
        Self::from_microdegrees(self.0 + MICRODEGREES_PER_TURN / 2)
    }
}

impl Add for Angle {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        self.add_angle(rhs)
    }
}

impl Sub for Angle {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self.sub_angle(rhs)
    }
}

impl Neg for Angle {
    type Output = Self;
    /// Mirrors an angle about zero degrees. Mirroring geometry uses this together with a coordinate
    /// negation; neither may change a winding order silently (spec §1.2, and the RTL rule in §7.6 of
    /// the roadmap: layout mirrors, geometry does not).
    fn neg(self) -> Self {
        Self::from_microdegrees(-self.0)
    }
}

impl fmt::Display for Angle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} µ°", self.0)
    }
}
