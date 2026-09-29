//! The units a quantity can be expressed in, as exact integer ratios (spec §2).

use crate::error::UnitError;
use crate::round::div_round_half_away_from_zero;

/// A unit of length, expressed as an exact rational number of internal micrometres.
///
/// Every ratio here is exact, so a conversion never loses information before the single rounding
/// step. [`Unit::PdfPoint`] is the instructive case: its ratio is `25400/72`, which is *not* an
/// integer number of micrometres (≈ 352.78 µm), and that is precisely why the format-quantization
/// tolerance class exists (spec §2.1, §3).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Unit {
    /// The internal unit: 1 µm = 10⁻⁶ m.
    Micrometre,
    /// 1 mm = 1 000 µm.
    Millimetre,
    /// 1 cm = 10 000 µm.
    Centimetre,
    /// 1 m = 1 000 000 µm.
    Metre,
    /// 1 in = 25 400 µm, exactly.
    Inch,
    /// One HPGL plotter unit: 1 016 units per inch, so exactly 25 µm (spec §2).
    HpglPlotterUnit,
    /// One PostScript/PDF point: 1/72 in = 25 400/72 µm, not an integer number of micrometres.
    PdfPoint,
}

impl Unit {
    /// The exact ratio of one of this unit to internal micrometres, as `(numerator, denominator)`.
    #[must_use]
    pub const fn ratio_to_micrometres(self) -> (i64, i64) {
        match self {
            Self::Micrometre => (1, 1),
            Self::Millimetre => (1_000, 1),
            Self::Centimetre => (10_000, 1),
            Self::Metre => (1_000_000, 1),
            Self::Inch => (25_400, 1),
            Self::HpglPlotterUnit => (25, 1),
            Self::PdfPoint => (25_400, 72),
        }
    }

    /// Whether one of this unit is a whole number of micrometres.
    ///
    /// A `false` answer does not mean the unit is unusable; it means its quantum is coarser or finer
    /// than the internal unit, so a round trip cannot be exact and the artifact must publish its
    /// quantization (spec §2.1).
    #[must_use]
    pub const fn is_integral_in_micrometres(self) -> bool {
        matches!(
            self,
            Self::Micrometre
                | Self::Millimetre
                | Self::Centimetre
                | Self::Metre
                | Self::Inch
                | Self::HpglPlotterUnit
        )
    }

    /// Converts an exact rational number of this unit into internal micrometres.
    ///
    /// This is the *only* conversion entry point, and it performs exactly one multiply and one
    /// divide (spec §2): `value = numerator / denominator` of `self`, in micrometres.
    ///
    /// # Errors
    ///
    /// Returns [`UnitError::DivisionByZero`] when `denominator` is zero and [`UnitError::Overflow`]
    /// when the result does not fit the internal representation.
    pub fn to_micrometres(self, numerator: i64, denominator: i64) -> Result<i64, UnitError> {
        if denominator == 0 {
            return Err(UnitError::DivisionByZero {
                operation: "Unit::to_micrometres",
            });
        }
        let (unit_num, unit_den) = self.ratio_to_micrometres();
        // i128 holds the product of two i64 magnitudes with room to spare; the rounding step is the
        // single place a conversion may lose information.
        let n = i128::from(numerator) * i128::from(unit_num);
        let d = i128::from(denominator) * i128::from(unit_den);
        div_round_half_away_from_zero(n, d)
    }

    /// Expresses an internal micrometre count in this unit as an exact rational `(num, den)`.
    ///
    /// No rounding happens here: this is the boundary a serializer reads when it must publish the
    /// value it is about to quantize.
    ///
    /// # Errors
    ///
    /// Returns [`UnitError::Overflow`] if reducing the ratio would not fit `i64` parts, which cannot
    /// happen for values inside the declared domain; the `Result` keeps that a fact rather than an
    /// assumption.
    pub fn from_micrometres_exact(self, micrometres: i64) -> Result<(i64, i64), UnitError> {
        let (unit_num, unit_den) = self.ratio_to_micrometres();
        // value_in_unit = micrometres * unit_den / unit_num, kept exact as a fraction.
        let num = i128::from(micrometres) * i128::from(unit_den);
        let den = i128::from(unit_num);
        let sign = if (num < 0) != (den < 0) { -1 } else { 1 };
        let n = num.unsigned_abs();
        let d = den.unsigned_abs();
        let g = gcd(n, d);
        let num_reduced = i128::try_from(n / g).map_err(|_| UnitError::Overflow {
            operation: "Unit::from_micrometres_exact",
        })? * sign;
        let den_reduced = i128::try_from(d / g).map_err(|_| UnitError::Overflow {
            operation: "Unit::from_micrometres_exact",
        })?;
        Ok((
            i64::try_from(num_reduced).map_err(|_| UnitError::Overflow {
                operation: "Unit::from_micrometres_exact",
            })?,
            i64::try_from(den_reduced).map_err(|_| UnitError::Overflow {
                operation: "Unit::from_micrometres_exact",
            })?,
        ))
    }
}

/// Greatest common divisor of two magnitudes, iteratively (no recursion, no allocation).
fn gcd(a: u128, b: u128) -> u128 {
    let (mut a, mut b) = (a, b);
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    // gcd(0, 0) is 0; callers divide by it only when the numerator is 0, so guard to 1.
    if a == 0 {
        1
    } else {
        a
    }
}
