//! Exact, bounded numeric literal conversion; operators and bound values are separate layers.
use super::{FormulaNode, FormulaNodeKind, FormulaSourceSpan, FormulaUnit};
use core::fmt;
use sc_units::{round::div_round_half_away_from_zero_unsigned, UnitError};

/// The four input-literal kinds; area and boolean have no numeric literal syntax.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaLiteralKind {
    /// Nonnegative repetition count.
    Count,
    /// Scale factor in parts per million.
    Ratio,
    /// Distance in micrometres.
    Length,
    /// Raw angle in microdegrees; complete turns are retained.
    Angle,
}
impl FormulaLiteralKind {
    /// Canonical kind token.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Count => "count",
            Self::Ratio => "ratio",
            Self::Length => "length",
            Self::Angle => "angle",
        }
    }
}

/// Component of the exact reduced rational that exceeds its width limit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaRationalComponent {
    /// Nonnegative numerator magnitude.
    Numerator,
    /// Positive denominator.
    Denominator,
}

/// Numeric input refusal; it contains no customer source spelling.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaLiteralRule {
    /// A reduced converted rational component requires at least the measured number of bits.
    RationalWidth {
        /// Component with the proved width excess.
        component: FormulaRationalComponent,
        /// Honest lower bound, not a fabricated exact width of an arbitrarily large input.
        measured_bits_at_least: u16,
    },
    /// Rounded input exceeds the scalar length domain.
    LengthDomain {
        /// Maximum nonnegative literal input in micrometres.
        maximum: u128,
        /// Rounded input magnitude in micrometres.
        measured: u128,
    },
    /// The shared rounding primitive refused; no fallback value is fabricated.
    Rounding(UnitError),
}

/// Located formula-domain error during one literal's input conversion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormulaLiteralError {
    span: FormulaSourceSpan,
    rule: FormulaLiteralRule,
}
impl FormulaLiteralError {
    /// Stable diagnostic family, localized at a later command boundary.
    #[must_use]
    pub const fn diagnostic_code(self) -> &'static str {
        "formula_domain"
    }
    /// Exact semantic span of this literal, including its grouping parentheses.
    #[must_use]
    pub const fn span(self) -> FormulaSourceSpan {
        self.span
    }
    /// Typed refusal with its actual input-domain witness.
    #[must_use]
    pub const fn rule(self) -> FormulaLiteralRule {
        self.rule
    }
    /// Rational width limit's fixed bound, when that rule refused.
    #[must_use]
    pub const fn rational_bit_bound(self) -> Option<u16> {
        match self.rule {
            FormulaLiteralRule::RationalWidth { .. } => Some(128),
            _ => None,
        }
    }
    /// Canonical limit token, when a rational width rule refused.
    #[must_use]
    pub const fn limit_token(self) -> Option<&'static str> {
        match self.rule {
            FormulaLiteralRule::RationalWidth { .. } => Some("max_rational_bits"),
            _ => None,
        }
    }
}
impl fmt::Display for FormulaLiteralError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {:?} at bytes {}..{}",
            self.diagnostic_code(),
            self.rule,
            self.span.start(),
            self.span.end()
        )
    }
}
impl std::error::Error for FormulaLiteralError {}

/// Privately normalized positive literal child, still borrowing its validated syntax source.
/// Unary minus, scalar expression results and signed numeric bindings remain separate.
/// ```compile_fail
/// fn forge(literal: &mut sc_core::recipe::FormulaLiteral<'_>) { literal.magnitude = 0; }
/// ```
/// ```compile_fail
/// fn detached() -> sc_core::recipe::FormulaLiteral<'static> {
///     let source = String::from("2.5 cm");
///     let syntax = sc_core::recipe::FormulaExpression::parse(&source).unwrap();
///     syntax.root().normalized_literal().unwrap().unwrap()
/// }
/// ```
#[derive(Clone, Copy)]
pub struct FormulaLiteral<'a> {
    number: &'a str,
    unit: Option<FormulaUnit>,
    span: FormulaSourceSpan,
    kind: FormulaLiteralKind,
    magnitude: u128,
}
impl fmt::Debug for FormulaLiteral<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaLiteral")
            .field("kind", &self.kind)
            .field("span", &self.span)
            .finish_non_exhaustive()
    }
}
impl<'a> FormulaLiteral<'a> {
    /// Original borrowed numeric spelling, exposed only by explicit inspection.
    #[must_use]
    pub const fn number(self) -> &'a str {
        self.number
    }
    /// Original closed unit token, without display normalization.
    #[must_use]
    pub const fn unit(self) -> Option<FormulaUnit> {
        self.unit
    }
    /// Semantic source extent; not a canonical formula identity.
    #[must_use]
    pub const fn span(self) -> FormulaSourceSpan {
        self.span
    }
    /// Kind retained independently of the normalized magnitude.
    #[must_use]
    pub const fn kind(self) -> FormulaLiteralKind {
        self.kind
    }
    /// Once-rounded internal integer, with the full 128-bit positive magnitude available.
    #[must_use]
    pub const fn magnitude(self) -> u128 {
        self.magnitude
    }
}
impl<'a> FormulaNode<'a> {
    /// Normalize this literal only. A nonliteral returns None, without inspecting its children.
    /// This does not fold unary minus, normalize directions, validate kinds/names or evaluate.
    pub fn normalized_literal(self) -> Result<Option<FormulaLiteral<'a>>, FormulaLiteralError> {
        match self.kind() {
            FormulaNodeKind::Literal { number, unit } => {
                normalize(number, unit, self.span()).map(Some)
            }
            _ => Ok(None),
        }
    }
}

fn width(component: FormulaRationalComponent) -> FormulaLiteralRule {
    FormulaLiteralRule::RationalWidth {
        component,
        measured_bits_at_least: 129,
    }
}

pub(super) fn normalize(
    number: &str,
    unit: Option<FormulaUnit>,
    span: FormulaSourceSpan,
) -> Result<FormulaLiteral<'_>, FormulaLiteralError> {
    let (kind, multiplier) = match unit {
        Some(FormulaUnit::Micrometre) => (FormulaLiteralKind::Length, 1),
        Some(FormulaUnit::Millimetre) => (FormulaLiteralKind::Length, 1000),
        Some(FormulaUnit::Centimetre) => (FormulaLiteralKind::Length, 10000),
        Some(FormulaUnit::Metre) => (FormulaLiteralKind::Length, 1000000),
        Some(FormulaUnit::Inch) => (FormulaLiteralKind::Length, 25400),
        Some(FormulaUnit::Degree) => (FormulaLiteralKind::Angle, 1000000),
        Some(FormulaUnit::Percent) => (FormulaLiteralKind::Ratio, 10000),
        None if number.contains('.') => (FormulaLiteralKind::Ratio, 1000000),
        None => (FormulaLiteralKind::Count, 1),
    };
    let refuse = |rule| FormulaLiteralError { span, rule };
    let (n, d) = reduced_ratio(number, multiplier).map_err(refuse)?;
    // Width is established on the exact converted rational before the single input rounding.
    let magnitude = div_round_half_away_from_zero_unsigned(n, d)
        .map_err(|error| refuse(FormulaLiteralRule::Rounding(error)))?;
    let maximum = u128::from(sc_units::MAX_LENGTH_UM.unsigned_abs());
    if kind == FormulaLiteralKind::Length && magnitude > maximum {
        return Err(refuse(FormulaLiteralRule::LengthDomain {
            maximum,
            measured: magnitude,
        }));
    }
    Ok(FormulaLiteral {
        number,
        unit,
        span,
        kind,
        magnitude,
    })
}

fn reduced_ratio(number: &str, mut multiplier: u128) -> Result<(u128, u128), FormulaLiteralRule> {
    let (integer, fraction) = number.split_once('.').unwrap_or((number, ""));
    // Remove only fractional trailing zeroes: integer trailing zeroes carry value.
    let fraction = fraction.trim_end_matches('0');
    let digits = integer
        .bytes()
        .chain(fraction.bytes())
        .skip_while(|byte| *byte == b'0');
    let significant = digits.clone().count();
    if significant == 0 {
        return Ok((0, 1));
    }
    let scale = fraction.len();
    // After trimming, a fractional mantissa lacks at least one of factors2/5. Each multiplier
    // has at most six of either factor. scale >134 therefore proves a >128-bit denominator.
    if scale > 134 {
        return Err(width(FormulaRationalComponent::Denominator));
    }
    // More than scale+39 digits implies value >=10^39, even before the multiplier (>=1).
    // Its reduced numerator exceeds u128::MAX. Thus the only allocated workspace is <=173 digits.
    if significant > scale + 39 {
        return Err(width(FormulaRationalComponent::Numerator));
    }
    let mut decimal = Vec::with_capacity(significant);
    decimal.extend(digits.map(|byte| byte - b'0'));
    let mut twos = scale;
    let mut fives = scale;
    for (prime, remaining) in [(2_u128, &mut twos), (5_u128, &mut fives)] {
        while *remaining > 0 && multiplier.is_multiple_of(prime) {
            multiplier /= prime;
            *remaining -= 1;
        }
    }
    for (prime, remaining) in [(2_u8, &mut twos), (5_u8, &mut fives)] {
        while *remaining > 0
            && decimal
                .last()
                .is_some_and(|digit| digit.is_multiple_of(prime))
        {
            divide_decimal(&mut decimal, prime);
            *remaining -= 1;
        }
    }
    let mut numerator = 0_u128;
    for digit in decimal {
        numerator = numerator
            .checked_mul(10)
            .and_then(|n| n.checked_add(u128::from(digit)))
            .ok_or_else(|| width(FormulaRationalComponent::Numerator))?;
    }
    numerator = numerator
        .checked_mul(multiplier)
        .ok_or_else(|| width(FormulaRationalComponent::Numerator))?;
    let mut denominator = 1_u128;
    for (prime, power) in [(2_u128, twos), (5_u128, fives)] {
        for _ in 0..power {
            denominator = denominator
                .checked_mul(prime)
                .ok_or_else(|| width(FormulaRationalComponent::Denominator))?;
        }
    }
    Ok((numerator, denominator))
}

fn divide_decimal(digits: &mut [u8], divisor: u8) {
    let mut carry = 0_u8;
    for digit in digits {
        // Divisor is2 or5, so carry*10+digit is at most49 and cannot overflow u8.
        let value = carry * 10 + *digit;
        *digit = value / divisor;
        carry = value % divisor;
    }
}
