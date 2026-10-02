//! Lengths and areas: the internal fixed-point micrometre representation (spec §1, §1.1).

use core::fmt;
use core::ops::{Add, Neg, Sub};

use crate::error::UnitError;
use crate::unit::Unit;

/// The declared domain limit for a coordinate or a length: 10⁹ µm = 1 km (spec §1.1).
///
/// The limit deliberately exceeds the garment envelope and stays far below the integer boundary.
/// Intermediate arithmetic is checked separately. An out-of-domain value requires diagnosis; its
/// magnitude alone does not establish its cause.
pub const MAX_LENGTH_UM: i64 = 1_000_000_000;

/// The declared domain limit for an area: 10¹⁸ µm² = 1 km² (spec §1.1).
pub const MAX_AREA_UM2: i64 = 1_000_000_000_000_000_000;

/// A length, stored as a signed count of **micrometres**.
///
/// This is the single internal unit for every length in the product: a coordinate, a seam allowance
/// width, a grade delta, a printed measurement. Fixed-point integers are what make golden files
/// byte-stable across platforms and make segment topology exactly decidable (spec §1, §5).
///
/// A `Length` is always inside [`MAX_LENGTH_UM`]; the constructors enforce it, so code downstream
/// never has to re-check.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Length(i64);

impl Length {
    /// The zero length.
    pub const ZERO: Self = Self(0);

    /// Builds a length from an internal micrometre count.
    ///
    /// # Errors
    ///
    /// Returns [`UnitError::DomainExceeded`] when the count is outside the declared domain.
    pub const fn from_micrometres(micrometres: i64) -> Result<Self, UnitError> {
        Self::from_micrometres_for(micrometres, "Length::from_micrometres")
    }

    /// Checked internal construction with the calling operation's diagnostic context.
    pub(crate) const fn from_micrometres_for(
        micrometres: i64,
        operation: &'static str,
    ) -> Result<Self, UnitError> {
        if micrometres > MAX_LENGTH_UM || micrometres < -MAX_LENGTH_UM {
            return Err(UnitError::DomainExceeded {
                operation,
                kind: "length",
                value: micrometres as i128,
                limit: MAX_LENGTH_UM as i128,
            });
        }
        Ok(Self(micrometres))
    }

    /// Builds a length from an exact rational number of `unit`.
    ///
    /// `3/4` of an inch is `Length::from_rational(3, 4, Unit::Inch)`. This is the entry point for
    /// every user input and every file format value, and it performs exactly one multiply and one
    /// divide (spec §2): no chained conversion, no double rounding.
    ///
    /// # Errors
    ///
    /// Returns [`UnitError::DivisionByZero`] when `denominator` is zero, [`UnitError::Overflow`] when
    /// the intermediate value does not fit, and [`UnitError::DomainExceeded`] when the result is
    /// outside the declared domain.
    pub fn from_rational(numerator: i64, denominator: i64, unit: Unit) -> Result<Self, UnitError> {
        let um = unit.to_micrometres(numerator, denominator)?;
        Self::from_micrometres_for(um, "Length::from_rational")
    }

    /// The internal micrometre count.
    #[must_use]
    pub const fn as_micrometres(self) -> i64 {
        self.0
    }

    /// This length as an exact rational number of `unit`, reduced.
    ///
    /// This is what a serializer reads before it quantizes to the target format's resolution; it
    /// never rounds, so the quantization a writer performs is the only rounding in the pipeline
    /// (spec §2.1).
    ///
    /// # Errors
    ///
    /// Returns [`UnitError::Overflow`] if the reduced ratio would not fit `i64` parts.
    pub fn as_rational_in(self, unit: Unit) -> Result<(i64, i64), UnitError> {
        unit.from_micrometres_exact(self.0)
    }

    /// This length as an `f64` count of `unit`, for the boundaries where a format demands a float.
    ///
    /// ⚠ Boundary only. The result must never be stored back into a `Length`: floating point is how
    /// determinism is lost (spec §1, §2.1).
    #[must_use]
    pub fn as_f64_in(self, unit: Unit) -> f64 {
        let (num, den) = unit.ratio_to_micrometres();
        // Attributes on expressions are unstable Rust, so each cast gets its own statement.
        #[allow(clippy::cast_precision_loss)] // documented boundary conversion, i64 -> f64
        let value = self.0 as f64;
        #[allow(clippy::cast_precision_loss)] // documented boundary conversion, i64 -> f64
        let per_unit = num as f64 / den as f64;
        value / per_unit
    }

    /// Builds a length from an `f64` count of `unit` at an import or input boundary.
    ///
    /// # Errors
    ///
    /// Returns [`UnitError::NonFinite`] for NaN or an infinity, and [`UnitError::DomainExceeded`] for
    /// a value outside the declared domain.
    pub fn from_f64_in(value: f64, unit: Unit) -> Result<Self, UnitError> {
        if !value.is_finite() {
            return Err(UnitError::NonFinite {
                operation: "Length::from_f64_in",
            });
        }
        let (num, den) = unit.ratio_to_micrometres();
        #[allow(clippy::cast_precision_loss)] // documented boundary conversion, i64 -> f64
        let per_unit = num as f64 / den as f64;
        let scaled = value * per_unit;
        #[allow(clippy::cast_possible_truncation)] // rounded, then domain-checked below
        let um = scaled.round() as i64;
        Self::from_micrometres_for(um, "Length::from_f64_in")
    }

    /// The absolute value.
    #[must_use]
    pub const fn abs(self) -> Self {
        // The domain is symmetric about zero, so |v| is inside it whenever v is.
        Self(self.0.abs())
    }

    /// The smaller of two lengths.
    #[must_use]
    pub const fn min(self, other: Self) -> Self {
        if self.0 <= other.0 {
            self
        } else {
            other
        }
    }

    /// The larger of two lengths.
    #[must_use]
    pub const fn max(self, other: Self) -> Self {
        if self.0 >= other.0 {
            self
        } else {
            other
        }
    }

    /// Adds two lengths.
    ///
    /// # Errors
    ///
    /// Returns [`UnitError::Overflow`] if the sum leaves the internal representation and
    /// [`UnitError::DomainExceeded`] if it leaves the declared domain. Neither happens silently.
    pub fn checked_add(self, other: Self) -> Result<Self, UnitError> {
        let sum = i128::from(self.0) + i128::from(other.0);
        let truncated = i64::try_from(sum).map_err(|_| UnitError::Overflow {
            operation: "Length::checked_add",
        })?;
        Self::from_micrometres_for(truncated, "Length::checked_add")
    }

    /// Subtracts `other` from `self`.
    ///
    /// # Errors
    ///
    /// As [`Length::checked_add`].
    pub fn checked_sub(self, other: Self) -> Result<Self, UnitError> {
        let diff = i128::from(self.0) - i128::from(other.0);
        let truncated = i64::try_from(diff).map_err(|_| UnitError::Overflow {
            operation: "Length::checked_sub",
        })?;
        Self::from_micrometres_for(truncated, "Length::checked_sub")
    }

    /// Scales a length by an integer factor, as grading and multiplicity do.
    ///
    /// # Errors
    ///
    /// As [`Length::checked_add`].
    pub fn checked_mul_i64(self, factor: i64) -> Result<Self, UnitError> {
        let product = i128::from(self.0) * i128::from(factor);
        let truncated = i64::try_from(product).map_err(|_| UnitError::Overflow {
            operation: "Length::checked_mul_i64",
        })?;
        Self::from_micrometres_for(truncated, "Length::checked_mul_i64")
    }

    /// Divides a length by an integer divisor, rounding half away from zero.
    ///
    /// # Errors
    ///
    /// Returns [`UnitError::DivisionByZero`] when `divisor` is zero, otherwise as
    /// [`Length::checked_add`].
    pub fn checked_div_i64(self, divisor: i64) -> Result<Self, UnitError> {
        if divisor == 0 {
            return Err(UnitError::DivisionByZero {
                operation: "Length::checked_div_i64",
            });
        }
        let um =
            crate::round::div_round_half_away_from_zero(i128::from(self.0), i128::from(divisor))?;
        Self::from_micrometres_for(um, "Length::checked_div_i64")
    }

    /// The area of a rectangle with these two sides.
    ///
    /// # Errors
    ///
    /// Returns [`UnitError::DomainExceeded`] when the product leaves the declared area domain.
    pub fn checked_area(self, other: Self) -> Result<Area, UnitError> {
        Area::from_square_micrometres_for(
            i128::from(self.0) * i128::from(other.0),
            "Length::checked_area",
        )
    }

    /// Whether two lengths that should be identical by construction differ by no more than
    /// `tolerance` — the T1 numerical class in ordinary use (spec §3).
    #[must_use]
    pub fn eq_within(self, other: Self, tolerance: crate::tolerance::Tolerance) -> bool {
        tolerance.accepts(self, other)
    }
}

impl Neg for Length {
    type Output = Self;
    fn neg(self) -> Self {
        // The domain is symmetric, and |i64::MIN| cannot arise here because the domain is 10^9.
        Self(-self.0)
    }
}

impl fmt::Display for Length {
    /// Prints the lossless internal form. Presentation formatting (millimetres with three decimals,
    /// fractional inches, locale separators) belongs to the view layer, not here (spec §2.2).
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} µm", self.0)
    }
}

/// An area in square micrometres, always derived — never an input (spec §1.3).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Area(i64);

impl Area {
    /// Builds an area from a product of two micrometre counts.
    ///
    /// # Errors
    ///
    /// Returns [`UnitError::DomainExceeded`] when the value is outside [`MAX_AREA_UM2`].
    pub fn from_square_micrometres(value: i128) -> Result<Self, UnitError> {
        Self::from_square_micrometres_for(value, "Area::from_square_micrometres")
    }

    fn from_square_micrometres_for(
        value: i128,
        operation: &'static str,
    ) -> Result<Self, UnitError> {
        if value > i128::from(MAX_AREA_UM2) || value < i128::from(-MAX_AREA_UM2) {
            return Err(UnitError::DomainExceeded {
                operation,
                kind: "area",
                value,
                limit: i128::from(MAX_AREA_UM2),
            });
        }
        #[allow(clippy::cast_possible_truncation)] // bounded by the domain check above
        let truncated = value as i64;
        Ok(Self(truncated))
    }

    /// The internal square-micrometre count.
    #[must_use]
    pub const fn as_square_micrometres(self) -> i64 {
        self.0
    }
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} µm²", self.0)
    }
}

/// Adds two lengths with the same domain checks as [`Length::checked_add`].
///
/// The result is fallible even when both operands are valid lengths: their sum may leave the
/// declared domain. Use `(left + right)?` to propagate the typed refusal.
impl Add for Length {
    type Output = Result<Self, UnitError>;
    fn add(self, rhs: Self) -> Self::Output {
        self.checked_add(rhs)
    }
}

/// Subtracts two lengths with the same domain checks as [`Length::checked_sub`].
///
/// Use `(left - right)?` to propagate the typed refusal; a valid pair of operands does not imply
/// that their difference is inside the declared domain.
impl Sub for Length {
    type Output = Result<Self, UnitError>;
    fn sub(self, rhs: Self) -> Self::Output {
        self.checked_sub(rhs)
    }
}
