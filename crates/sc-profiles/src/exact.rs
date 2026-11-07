//! Numbers as a profile file writes them: whole, or exact decimal text. There is no floating point.
use sc_constraints::Rational;

/// How many digits one number may have, so that its numerator and denominator fit in 64 bits.
const MAX_DIGITS: usize = 18;

/// The exact value of decimal text such as `0.1` or `-1520.25`.
///
/// `None` when the text is not a decimal, or has more digits than an exact number here can hold.
#[must_use]
pub fn parse_decimal(text: &str) -> Option<Rational> {
    let (negative, digits) = match text.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, text),
    };
    let (whole, fraction) = digits.split_once('.').unwrap_or((digits, ""));
    let plain = |part: &str| part.bytes().all(|byte| byte.is_ascii_digit());
    if whole.is_empty()
        || !plain(whole)
        || !plain(fraction)
        || (digits.contains('.') && fraction.is_empty())
        || whole.len() + fraction.len() > MAX_DIGITS
    {
        return None;
    }
    let mut numerator: i64 = 0;
    for byte in whole.bytes().chain(fraction.bytes()) {
        numerator = numerator
            .checked_mul(10)?
            .checked_add(i64::from(byte - b'0'))?;
    }
    let exponent = u32::try_from(fraction.len()).ok()?;
    let denominator = 10_i64.checked_pow(exponent)?;
    Rational::new(if negative { -numerator } else { numerator }, denominator)
}

/// The one way this number is written: whole digits, or the shortest decimal that is exact.
///
/// `None` when no finite decimal is exact, such as one third. Nothing read from a file is like that.
#[must_use]
pub fn decimal_text(number: Rational) -> Option<String> {
    let (numerator, denominator) = (number.numerator(), number.denominator());
    if denominator == 1 {
        return Some(numerator.to_string());
    }
    let mut scale: i128 = 1;
    for places in 1..=MAX_DIGITS {
        scale = scale.checked_mul(10)?;
        if scale % denominator == 0 {
            let scaled = numerator.checked_mul(scale)?.checked_div(denominator)?;
            let digits = format!("{:0>width$}", scaled.abs(), width = places + 1);
            let (whole, fraction) = digits.split_at(digits.len() - places);
            let sign = if scaled < 0 { "-" } else { "" };
            return Some(format!("{sign}{whole}.{fraction}"));
        }
    }
    None
}
