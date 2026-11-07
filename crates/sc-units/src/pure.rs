//! Trigonometry and lengths built only from IEEE-exact operations, so they give the same bits on every platform and compiler.
//!
//! `f64::hypot`, `sin`, `cos`, `atan2` and `acos` come from each platform's math library, which IEEE 754 does not pin down. A geometry path that
//! calls them can round to a different micrometre on another operating system, and a golden file would then fail there and nowhere else. The
//! units chapter forbids that. Addition, subtraction, multiplication, division and `sqrt` are required to be correctly rounded, so these use only those.
//!
//! These are accurate to a few parts in 10^16, which on a pattern's lengths is far below a micrometre. They are not correctly rounded, and they do not need
//! to be: what geometry needs is the same answer everywhere. The recipe's own functions, which promise a correctly rounded result, are in `sc-core`.

const PI: f64 = core::f64::consts::PI;
const FRAC_PI_2: f64 = core::f64::consts::FRAC_PI_2;

/// The length of a vector. Inputs reach only about 2^61 in this project, so the squares cannot overflow.
#[must_use]
pub fn hypot(x: f64, y: f64) -> f64 {
    (x * x + y * y).sqrt()
}

// The sine near zero, within a quarter turn's half: a Taylor series, whose next term is below an ulp on this range.
fn sine_near_zero(x: f64) -> f64 {
    let square = x * x;
    let mut term = x;
    let mut sum = x;
    let mut n = 1.0;
    while n < 21.0 {
        term = -term * square / ((n + 1.0) * (n + 2.0));
        sum += term;
        n += 2.0;
    }
    sum
}

// The cosine near zero, the same way.
fn cosine_near_zero(x: f64) -> f64 {
    let square = x * x;
    let mut term = 1.0;
    let mut sum = 1.0;
    let mut n = 0.0;
    while n < 20.0 {
        term = -term * square / ((n + 1.0) * (n + 2.0));
        sum += term;
        n += 2.0;
    }
    sum
}

/// The sine and cosine of an angle in radians, at once.
#[must_use]
pub fn sin_cos(angle: f64) -> (f64, f64) {
    // Reduced to within an eighth of a turn of a quarter-turn multiple. A pattern's angles are small multiples of a turn, so the subtraction loses almost nothing.
    let quarter = (angle / FRAC_PI_2).round();
    let reduced = angle - quarter * FRAC_PI_2;
    let (sine, cosine) = (sine_near_zero(reduced), cosine_near_zero(reduced));
    match quarter.rem_euclid(4.0) {
        0.0 => (sine, cosine),
        1.0 => (cosine, -sine),
        2.0 => (-sine, -cosine),
        _ => (-cosine, sine),
    }
}

/// The sine of an angle in radians.
#[must_use]
pub fn sin(angle: f64) -> f64 {
    sin_cos(angle).0
}

/// The cosine of an angle in radians.
#[must_use]
pub fn cos(angle: f64) -> f64 {
    sin_cos(angle).1
}

// The arctangent of a value in [0, 1]. Halving the angle twice with `sqrt` brings the argument under about 0.2, where a short series is exact to an ulp.
fn atan_unit(x: f64) -> f64 {
    let half = |t: f64| t / (1.0 + (1.0 + t * t).sqrt());
    let small = half(half(x));
    let square = small * small;
    let mut term = small;
    let mut sum = small;
    let mut n = 1.0;
    while n < 25.0 {
        term = -term * square;
        sum += term / (n + 2.0);
        n += 2.0;
    }
    4.0 * sum
}

/// The angle of a point, in radians, from −π to π.
#[must_use]
pub fn atan2(y: f64, x: f64) -> f64 {
    if x == 0.0 && y == 0.0 {
        return 0.0;
    }
    let (ax, ay) = (x.abs(), y.abs());
    let first = if ay <= ax {
        atan_unit(ay / ax)
    } else {
        FRAC_PI_2 - atan_unit(ax / ay)
    };
    let quadrant = if x < 0.0 { PI - first } else { first };
    if y < 0.0 {
        -quadrant
    } else {
        quadrant
    }
}

/// The arccosine of a value in [−1, 1], in radians.
#[must_use]
pub fn acos(x: f64) -> f64 {
    atan2((1.0 - x * x).max(0.0).sqrt(), x)
}
