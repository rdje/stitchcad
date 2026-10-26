//! Correctly rounded inverse values under the complete inverse threshold certificate.
use super::{
    pi::{PI_FRACTION_BITS, PI_LOWER_WORDS},
    rational::round_magnitude_wide,
    FormulaRational, FormulaRationalComponent, FormulaRationalError,
};
use num_bigint::BigUint;

const FRACTION_BITS: u32 = 576;
const HALF_ANGLES: u32 = 5;
const TERMS: u32 = 64;

#[derive(Debug)]
struct Interval {
    low: BigUint,
    high: BigUint,
}

/// Only this module constructs phases, including exact axes and principal signed branches.
#[derive(Debug)]
pub(super) struct InversePhase {
    magnitude: Interval,
    negative: bool,
}

impl InversePhase {
    pub(super) fn rounded(self) -> Result<FormulaRational, FormulaRationalError> {
        // Width<2^-564 turns is less than the certified distance>2^-540 to every midpoint.
        // Restoring the sign after magnitude rounding preserves a rounded negative180 degrees.
        let integer = round_magnitude_wide(self.magnitude.low * 360_000_000_u32, scale());
        Ok(FormulaRational::from_integer_magnitude(
            narrow_angle(&integer)?,
            self.negative,
        ))
    }
}

fn scale() -> BigUint {
    BigUint::from(1_u8) << FRACTION_BITS
}

fn ceil_div(numerator: BigUint, denominator: &BigUint) -> BigUint {
    (numerator + denominator - 1_u8) / denominator
}

pub(super) fn atan_value(value: FormulaRational) -> Result<FormulaRational, FormulaRationalError> {
    InversePhase {
        magnitude: positive_phase(
            BigUint::from(value.numerator_magnitude()),
            BigUint::from(value.denominator()) * 1_000_000_u32,
        ),
        negative: value.is_negative(),
    }
    .rounded()
}

/// None means precisely the undefined zero vector, after the caller's homogeneous kind checks.
pub(super) fn atan2_phase(y: FormulaRational, x: FormulaRational) -> Option<InversePhase> {
    let y_zero = y.numerator_magnitude() == 0;
    let x_zero = x.numerator_magnitude() == 0;
    if y_zero && x_zero {
        return None;
    }
    let magnitude = if y_zero || x_zero {
        let exact = if x_zero {
            scale() / 4_u8
        } else if x.is_negative() {
            scale() / 2_u8
        } else {
            BigUint::from(0_u8)
        };
        Interval {
            low: exact.clone(),
            high: exact,
        }
    } else {
        // The quotient is private: its components may need256 bits despite admitted operands.
        let phase = positive_phase(
            BigUint::from(y.numerator_magnitude()) * x.denominator(),
            BigUint::from(x.numerator_magnitude()) * y.denominator(),
        );
        if x.is_negative() {
            complement(phase, &(scale() / 2_u8))
        } else {
            phase
        }
    };
    Some(InversePhase {
        magnitude,
        negative: y.is_negative(),
    })
}

fn complement(value: Interval, axis: &BigUint) -> Interval {
    Interval {
        low: axis - value.high,
        high: axis - value.low,
    }
}

fn positive_phase(mut numerator: BigUint, mut denominator: BigUint) -> Interval {
    if numerator == BigUint::from(0_u8) {
        return Interval {
            low: numerator.clone(),
            high: numerator,
        };
    }
    let reciprocal = numerator > denominator;
    if reciprocal {
        core::mem::swap(&mut numerator, &mut denominator);
    }
    let s = scale();
    let scaled = numerator * &s;
    let mut x = Interval {
        low: &scaled / &denominator,
        high: ceil_div(scaled, &denominator),
    };
    for _ in 0..HALF_ANGLES {
        let low_n = &s * (&s + &x.low * &x.low / &s);
        let high_n = &s * (&s + ceil_div(&x.high * &x.high, &s));
        let root = floor_root(&low_n, &s);
        // This upper endpoint's square bounds high_n; one root solve suffices per half angle.
        let upper = &root + ceil_div(high_n - &root * &root, &(&root * 2_u8));
        x = Interval {
            low: x.low * &s / (&s + upper),
            high: ceil_div(x.high * &s, &(&s + root)),
        };
    }
    let radians = series_interval(x, &s);
    let pi_low = BigUint::from_slice(&PI_LOWER_WORDS) >> (PI_FRACTION_BITS - FRACTION_BITS);
    let phase = Interval {
        low: radians.low * 16_u8 * &s / (&pi_low + 1_u8),
        high: ceil_div(radians.high * 16_u8 * &s, &pi_low),
    };
    if reciprocal {
        complement(phase, &(&s / 4_u8))
    } else {
        phase
    }
}

/// On[S²,2S²], decreasing integer Newton reaches floor(sqrt(n)) in at most13 divisions.
/// The proved budget needs no iteration cutoff or new precision-failure result.
fn floor_root(n: &BigUint, s: &BigUint) -> BigUint {
    let mut t = s * 2_u8;
    loop {
        let next = (&t + n / &t) / 2_u8;
        if next >= t {
            return t;
        }
        t = next;
    }
}

fn series_interval(x: Interval, s: &BigUint) -> Interval {
    let z = Interval {
        low: &x.low * &x.low / s,
        high: ceil_div(&x.high * &x.high, s),
    };
    let mut power = x;
    let mut positive_low = BigUint::from(0_u8);
    let mut positive_high = BigUint::from(0_u8);
    let mut negative_low = BigUint::from(0_u8);
    let mut negative_high = BigUint::from(0_u8);
    for index in 0..TERMS {
        let divisor = BigUint::from(2 * index + 1);
        let low = &power.low / &divisor;
        let high = ceil_div(power.high.clone(), &divisor);
        if index % 2 == 0 {
            positive_low += low;
            positive_high += high;
        } else {
            negative_low += low;
            negative_high += high;
        }
        power = Interval {
            low: power.low * &z.low / s,
            high: ceil_div(power.high * &z.high, s),
        };
    }
    // Final x<=1/32, width<=11.64 terms plus the tail enclose atan(x) within328 units.
    let negative_with_tail = negative_high + 1_u8;
    Interval {
        low: if positive_low > negative_with_tail {
            positive_low - negative_with_tail
        } else {
            BigUint::from(0_u8)
        },
        high: positive_high + 1_u8 - negative_low,
    }
}

// The proved principal result magnitude<=180M also bounds this diagnostic conversion.
#[allow(clippy::cast_possible_truncation)]
fn narrow_angle(value: &BigUint) -> Result<u128, FormulaRationalError> {
    u128::try_from(value).map_err(|_| FormulaRationalError::RationalWidth {
        component: FormulaRationalComponent::Numerator,
        measured_bits: value.bits() as u32,
    })
}
