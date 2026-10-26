//! Correctly rounded sin/cos via exact folding and the complete forward threshold certificate.
use super::{
    pi::{PI_FRACTION_BITS, PI_LOWER_WORDS},
    rational::round_magnitude_wide,
    FormulaRational, FormulaRationalComponent, FormulaRationalError,
};
use num_bigint::BigUint;

const FRACTION_BITS: u32 = 512;
const TERMS: u32 = 64;
const PPM: u32 = 1_000_000;

#[derive(Debug)]
struct Interval {
    low: BigUint,
    high: BigUint,
}

fn ceil_div(numerator: BigUint, denominator: &BigUint) -> BigUint {
    (numerator + denominator - 1_u8) / denominator
}

/// Completed Angle input; reduction stays private, preserving all rational component magnitudes.
pub(super) fn sin_cos_value(
    angle: FormulaRational,
    cosine: bool,
) -> Result<FormulaRational, FormulaRationalError> {
    let quarter = BigUint::from(angle.denominator()) * 90_000_000_u32;
    let period = &quarter * 4_u8;
    let mut phase = BigUint::from(angle.numerator_magnitude()) % &period;
    let mut negative = angle.is_negative() && !cosine;
    // Cosine is even. Convert its magnitude to sine by an exact quarter-turn shift.
    if cosine {
        phase = (phase + &quarter) % &period;
    }
    let half = &quarter * 2_u8;
    if phase >= half {
        phase -= &half;
        negative = !negative;
    }
    if phase > quarter {
        phase = &half - phase;
    }
    let use_cosine = (&phase << 1_u8) > quarter;
    if use_cosine {
        phase = &quarter - phase;
    }
    if phase == BigUint::from(0_u8) {
        return Ok(FormulaRational::from_integer_magnitude(
            if use_cosine { u128::from(PPM) } else { 0 },
            negative,
        ));
    }

    let scale = BigUint::from(1_u8) << FRACTION_BITS;
    let pi_low = BigUint::from_slice(&PI_LOWER_WORDS) >> (PI_FRACTION_BITS - FRACTION_BITS);
    let denominator = BigUint::from(angle.denominator()) * 180_000_000_u32;
    let radians = Interval {
        low: &phase * &pi_low / &denominator,
        high: ceil_div(phase * (pi_low + 1_u8), &denominator),
    };
    let result = series_interval(radians, use_cosine, &scale);
    // Recorded full1M threshold coverage excludes every midpoint by >2^-384 in sine value.
    // This interval is at most770*2^-512 wide, so its lower endpoint and true answer round alike.
    // This is a proved algorithm budget, never an input precision cutoff or a convergence refusal.
    let integer = round_magnitude_wide(result.low * PPM, scale);
    Ok(FormulaRational::from_integer_magnitude(
        narrow_ppm(&integer)?,
        negative,
    ))
}

fn series_interval(x: Interval, cosine: bool, scale: &BigUint) -> Interval {
    let z = Interval {
        low: &x.low * &x.low / scale,
        high: ceil_div(&x.high * &x.high, scale),
    };
    let mut term = if cosine {
        Interval {
            low: scale.clone(),
            high: scale.clone(),
        }
    } else {
        x
    };
    let mut positive_low = BigUint::from(0_u8);
    let mut positive_high = BigUint::from(0_u8);
    let mut negative_low = BigUint::from(0_u8);
    let mut negative_high = BigUint::from(0_u8);
    for index in 0..TERMS {
        if index % 2 == 0 {
            positive_low += &term.low;
            positive_high += &term.high;
        } else {
            negative_low += &term.low;
            negative_high += &term.high;
        }
        let k = if cosine {
            (2 * index + 1) * (2 * index + 2)
        } else {
            (2 * index + 2) * (2 * index + 3)
        };
        let divisor = scale * k;
        term = Interval {
            low: &term.low * &z.low / &divisor,
            high: ceil_div(term.high * &z.high, &divisor),
        };
    }
    // On[0,pi/4], the true value is in[0,1]. The64-term remainder magnitude<1/128!<2^-512.
    // X width<=3, Z width<=8 and every term width<=12 prove total width<=770 scaled units.
    let negative_with_tail = negative_high + 1_u8;
    Interval {
        low: if positive_low > negative_with_tail {
            positive_low - negative_with_tail
        } else {
            BigUint::from(0_u8)
        },
        high: (positive_high + 1_u8 - negative_low).min(scale.clone()),
    }
}

// The certified range bounds every result by1M (20 bits), also bounding the diagnostic conversion.
#[allow(clippy::cast_possible_truncation)]
fn narrow_ppm(value: &BigUint) -> Result<u128, FormulaRationalError> {
    u128::try_from(value).map_err(|_| FormulaRationalError::RationalWidth {
        component: FormulaRationalComponent::Numerator,
        measured_bits: value.bits() as u32,
    })
}
