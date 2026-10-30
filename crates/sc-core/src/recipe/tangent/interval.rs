//! Fresh outward pi and unrounded component enclosures at an admitted dyadic precision.
use num_bigint::BigUint;

pub(super) struct Interval {
    pub(super) low: BigUint,
    pub(super) high: BigUint,
}

fn ceil_div(numerator: BigUint, denominator: &BigUint) -> BigUint {
    (numerator + denominator - 1_u8) / denominator
}

fn atan_interval(base: u32, bits: usize, scale: &BigUint) -> Interval {
    let mut power = BigUint::from(base);
    let mut positive_low = BigUint::from(0_u8);
    let mut positive_high = BigUint::from(0_u8);
    let mut negative_low = BigUint::from(0_u8);
    let mut negative_high = BigUint::from(0_u8);
    for index in 0..bits {
        // The admitted I(B) bounds power,denominator and all directed-sum candidates.
        let denominator = &power * (2 * index + 1);
        let floor = scale / &denominator;
        let ceiling = ceil_div(scale.clone(), &denominator);
        if index.is_multiple_of(2) {
            positive_low += floor;
            positive_high += ceiling;
        } else {
            negative_low += floor;
            negative_high += ceiling;
        }
        power *= base * base;
    }
    let tail = ceil_div(scale.clone(), &(&power * (2 * bits + 1)));
    // B>=128;the positive first term dominates all negative ceilings,including rounding slack.
    let mut low = positive_low - negative_high;
    let mut high = positive_high - negative_low;
    if bits.is_multiple_of(2) {
        high += tail;
    } else {
        low -= tail;
    }
    Interval { low, high }
}

fn pi_interval(bits: usize, scale: &BigUint) -> Interval {
    let a = atan_interval(5, bits, scale);
    let b = atan_interval(239, bits, scale);
    Interval {
        low: (&a.low * 16_u8 - &b.high * 4_u8).max(scale * 3_u8),
        high: (&a.high * 16_u8 - &b.low * 4_u8).min(scale * 4_u8),
    }
}

fn component(x: &Interval, cosine: bool, bits: usize, scale: &BigUint) -> Interval {
    let square = Interval {
        low: &x.low * &x.low / scale,
        high: ceil_div(&x.high * &x.high, scale),
    };
    let mut term = if cosine {
        Interval {
            low: scale.clone(),
            high: scale.clone(),
        }
    } else {
        Interval {
            low: x.low.clone(),
            high: x.high.clone(),
        }
    };
    let mut positive_low = BigUint::from(0_u8);
    let mut positive_high = BigUint::from(0_u8);
    let mut negative_low = BigUint::from(0_u8);
    let mut negative_high = BigUint::from(0_u8);
    for index in 0..bits {
        if index.is_multiple_of(2) {
            positive_low += &term.low;
            positive_high += &term.high;
        } else {
            negative_low += &term.low;
            negative_high += &term.high;
        }
        // Primitive factors fit the admitted extent;their product is a bounded BigUint candidate.
        let first = 2 * index + if cosine { 1 } else { 2 };
        let divisor = (scale * first) * (first + 1);
        term = Interval {
            low: &term.low * &square.low / &divisor,
            high: ceil_div(term.high * &square.high, &divisor),
        };
    }
    // The analytic remainder is below one scaled unit;clip only with the proved[0,1] range.
    let subtract = negative_high + 1_u8;
    Interval {
        low: if positive_low > subtract {
            positive_low - subtract
        } else {
            BigUint::from(0_u8)
        },
        high: (positive_high + 1_u8 - negative_low).min(scale.clone()),
    }
}

/// Return rounded endpoints of the entire outward quotient,not a quotient of rounded components.
pub(super) fn rounded_quotient(
    phase: &BigUint,
    denominator: u128,
    reciprocal: bool,
    bits: usize,
) -> Interval {
    let scale = BigUint::from(1_u8) << bits;
    let pi = pi_interval(bits, &scale);
    let divisor = BigUint::from(denominator) * 180_000_000_u32;
    let x = Interval {
        low: phase * pi.low / &divisor,
        high: ceil_div(phase * pi.high, &divisor),
    };
    let sine = component(&x, false, bits, &scale);
    let cosine = component(&x, true, bits, &scale);
    let (n, mut d) = if reciprocal {
        (cosine, sine)
    } else {
        (sine, cosine)
    };
    // Analytic floors:cos(x)>1/2;nearest admitted pole gives sin(x)>=2^-128.
    let floor = if reciprocal {
        BigUint::from(1_u8) << (bits - 128)
    } else {
        &scale / 2_u8
    };
    d.low = d.low.max(floor);
    Interval {
        low: super::super::rational::round_magnitude_wide(n.low * 1_000_000_u32, d.high),
        high: super::super::rational::round_magnitude_wide(n.high * 1_000_000_u32, d.low),
    }
}
