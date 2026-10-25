//! Completed numeric values in exact internal units, before binding or recipe execution.
use super::{
    FormulaBinaryOperator, FormulaBuiltin, FormulaKind, FormulaLiteral, FormulaLiteralKind,
    FormulaRational, FormulaRationalError, FormulaUnaryOperator,
};
use core::fmt;
use sc_units::{MAX_AREA_UM2, MAX_LENGTH_UM};

/// A typed refusal at the completed numeric-value boundary; source context belongs to evaluation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaNumericError {
    /// A variadic numeric built-in was called without its required first operand.
    BuiltinArity {
        /// The attempted built-in.
        builtin: FormulaBuiltin,
        /// Actual operand count.
        actual: usize,
    },
    /// First operand that differs from the numeric built-in's required homogeneous kind.
    BuiltinDimension {
        /// The attempted built-in.
        builtin: FormulaBuiltin,
        /// Zero-based argument position, with the principal value at zero for clamp.
        argument: usize,
        /// Kind of the principal or first operand.
        wanted: FormulaKind,
        /// Actual mismatched argument kind.
        actual: FormulaKind,
    },
    /// Explicit clamp's lower bound exceeds its upper bound in exact internal units.
    InvalidClamp {
        /// Shared numeric kind of value and bounds.
        kind: FormulaKind,
        /// Exact supplied lower bound.
        low: FormulaRational,
        /// Exact supplied upper bound.
        high: FormulaRational,
    },
    /// The attempted binary operation has no accepted signature through this value API.
    BinaryDimension {
        /// The attempted operator.
        operator: FormulaBinaryOperator,
        /// Actual left operand kind.
        left: FormulaKind,
        /// Actual right operand kind.
        right: FormulaKind,
    },
    /// The actual quantity kind has no signature for this unary operator.
    UnaryDimension {
        /// The attempted unary operator.
        operator: FormulaUnaryOperator,
        /// The actual operand kind.
        kind: FormulaKind,
    },
    /// Boolean and geometry kinds cannot carry a numeric rational value.
    NonNumericKind {
        /// The actual refused kind.
        kind: FormulaKind,
    },
    /// An exact completed result exceeds its scalar domain before any binding rounding.
    ScalarDomain {
        /// Actual construction or operator token; never customer source spelling.
        operation: &'static str,
        /// The quantity whose domain was exceeded.
        kind: FormulaKind,
        /// Exact signed internal-unit value, without narrowing or rounding.
        value: FormulaRational,
        /// Inclusive lower bound in internal units.
        minimum: i64,
        /// Inclusive upper bound, or None for an unbounded nonnegative count.
        maximum: Option<i64>,
    },
    /// Exact arithmetic refused before a completed numeric value could be returned.
    Rational(FormulaRationalError),
}

impl FormulaNumericError {
    /// Stable diagnostic family; no expression, name or statement is fabricated here.
    #[must_use]
    pub const fn diagnostic_code(self) -> &'static str {
        match self {
            Self::BuiltinArity { .. } => "formula_parse",
            Self::BuiltinDimension { .. } => "formula_dimension",
            Self::InvalidClamp { .. } => "formula_domain",
            Self::NonNumericKind { .. }
            | Self::BinaryDimension { .. }
            | Self::UnaryDimension { .. } => "formula_dimension",
            Self::ScalarDomain { .. } => "formula_domain",
            Self::Rational(error) => error.diagnostic_code(),
        }
    }

    /// Whether the absent angle/length product requires the normative arc_length hint.
    #[must_use]
    pub const fn arc_length_hint(self) -> bool {
        match self {
            Self::BinaryDimension {
                operator,
                left,
                right,
            } => operator.arc_length_hint(left, right),
            _ => false,
        }
    }
}

impl From<FormulaRationalError> for FormulaNumericError {
    fn from(error: FormulaRationalError) -> Self {
        Self::Rational(error)
    }
}

impl fmt::Display for FormulaNumericError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rational(error) => fmt::Display::fmt(error, f),
            _ => {
                write!(f, "{}: {self:?}", self.diagnostic_code())?;
                if self.arc_length_hint() {
                    write!(f, "; use arc_length")?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for FormulaNumericError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Rational(error) => Some(error),
            _ => None,
        }
    }
}

/// An immutable exact value of one numeric kind, admitted by that kind's completed-result domain.
///
/// Values use micrometres, square micrometres, raw microdegrees, parts per million or counts.
/// Fractional counts are valid until binding; angles retain signed complete turns. The rational
/// bound applies here, while signed i64 storage conversion belongs to the later binding boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FormulaNumericValue {
    kind: FormulaKind,
    value: FormulaRational,
}

impl FormulaNumericValue {
    /// Validates an exact completed value without rounding, clamping or signed narrowing.
    ///
    /// # Errors
    ///
    /// Returns NonNumericKind for Boolean/geometry, or ScalarDomain for an excessive signed length,
    /// area or negative count. Angle/ratio have no scalar bound before their later binding step.
    pub fn new(kind: FormulaKind, value: FormulaRational) -> Result<Self, FormulaNumericError> {
        Self::for_operation(kind, value, "numeric_value")
    }

    fn for_operation(
        kind: FormulaKind,
        value: FormulaRational,
        operation: &'static str,
    ) -> Result<Self, FormulaNumericError> {
        let bounds = match kind {
            FormulaKind::Length => Some((-MAX_LENGTH_UM, Some(MAX_LENGTH_UM))),
            FormulaKind::Area => Some((-MAX_AREA_UM2, Some(MAX_AREA_UM2))),
            FormulaKind::Count => Some((0, None)),
            FormulaKind::Angle | FormulaKind::Ratio => None,
            _ => return Err(FormulaNumericError::NonNumericKind { kind }),
        };
        if let Some((minimum, maximum)) = bounds {
            let excessive = maximum.is_some_and(|maximum| {
                // Compare magnitude to a nonnegative integer without overflowing maximum*denominator.
                let maximum = u128::from(maximum.unsigned_abs());
                let integer = value.numerator_magnitude() / value.denominator();
                integer > maximum
                    || (integer == maximum
                        && value.numerator_magnitude() % value.denominator() != 0)
            });
            if excessive || (kind == FormulaKind::Count && value.is_negative()) {
                return Err(FormulaNumericError::ScalarDomain {
                    operation,
                    kind,
                    value,
                    minimum,
                    maximum,
                });
            }
        }
        Ok(Self { kind, value })
    }

    /// Imports a once-normalized literal's internal integer, preserving its kind and full magnitude.
    ///
    /// # Errors
    ///
    /// Propagates a rational or scalar-domain refusal. No source parsing, input conversion, hidden
    /// pre-round fraction, unary sign, binding conversion or expression evaluation is performed.
    pub fn from_literal(literal: FormulaLiteral<'_>) -> Result<Self, FormulaNumericError> {
        let kind = match literal.kind() {
            FormulaLiteralKind::Length => FormulaKind::Length,
            FormulaLiteralKind::Angle => FormulaKind::Angle,
            FormulaLiteralKind::Ratio => FormulaKind::Ratio,
            FormulaLiteralKind::Count => FormulaKind::Count,
        };
        Self::for_operation(
            kind,
            FormulaRational::new(literal.magnitude(), 1, false)?,
            "literal",
        )
    }

    /// The admitted quantity kind.
    #[must_use]
    pub const fn kind(self) -> FormulaKind {
        self.kind
    }

    /// Exact canonical value in that kind's internal units; no binding rounding has occurred.
    #[must_use]
    pub const fn rational(self) -> FormulaRational {
        self.value
    }

    /// Adds same-kind operands exactly, checking the completed result's scalar domain.
    ///
    /// # Errors
    /// Returns a typed dimension, reduced-width or exact scalar-domain refusal.
    pub fn checked_add(self, rhs: Self) -> Result<Self, FormulaNumericError> {
        self.binary(FormulaBinaryOperator::Add, rhs)
    }

    /// Subtracts same-kind operands exactly; a negative completed Count refuses immediately.
    ///
    /// # Errors
    /// Returns a typed dimension, reduced-width or exact scalar-domain refusal.
    pub fn checked_sub(self, rhs: Self) -> Result<Self, FormulaNumericError> {
        self.binary(FormulaBinaryOperator::Subtract, rhs)
    }

    /// Multiplies a permitted kind pair with exact ppm scaling before reduced-width checks.
    ///
    /// # Errors
    /// Returns a typed dimension, reduced-width or exact scalar-domain refusal.
    pub fn checked_mul(self, rhs: Self) -> Result<Self, FormulaNumericError> {
        self.binary(FormulaBinaryOperator::Multiply, rhs)
    }

    /// Divides a permitted ordered kind pair, preserving exact fractions and ppm scaling.
    ///
    /// # Errors
    /// Returns a typed dimension, zero-divisor, reduced-width or exact scalar-domain refusal.
    pub fn checked_div(self, rhs: Self) -> Result<Self, FormulaNumericError> {
        self.binary(FormulaBinaryOperator::Divide, rhs)
    }

    /// Negates an admitted length, area, angle or ratio; Count negation is a dimensional refusal.
    ///
    /// # Errors
    /// Returns UnaryDimension for Count, including zero. No implicit rounding or direction modulo.
    pub fn checked_neg(self) -> Result<Self, FormulaNumericError> {
        let operator = FormulaUnaryOperator::Negate;
        let kind = operator
            .result_kind(self.kind)
            .ok_or(FormulaNumericError::UnaryDimension {
                operator,
                kind: self.kind,
            })?;
        Self::for_operation(kind, self.value.negated(), operator.token())
    }

    /// Squares length, ratio or Count exactly, including the ratio's ppm scale and result domain.
    ///
    /// # Errors
    /// Returns a unary dimension, reduced-width or completed scalar-domain refusal.
    pub fn checked_square(self) -> Result<Self, FormulaNumericError> {
        let operator = FormulaUnaryOperator::Square;
        let kind = operator
            .result_kind(self.kind)
            .ok_or(FormulaNumericError::UnaryDimension {
                operator,
                kind: self.kind,
            })?;
        let value = self.scaled_product(self, kind, false)?;
        Self::for_operation(kind, value, operator.token())
    }

    /// Evaluates one of the six exact comparison operators on same-kind numeric operands.
    ///
    /// # Errors
    /// Returns BinaryDimension for different kinds or a noncomparison operator. No tolerance,
    /// irrational provenance rule, source evaluation or Boolean binding is inferred here.
    pub fn checked_compare(
        self,
        operator: FormulaBinaryOperator,
        rhs: Self,
    ) -> Result<bool, FormulaNumericError> {
        let dimension = FormulaNumericError::BinaryDimension {
            operator,
            left: self.kind,
            right: rhs.kind,
        };
        if operator.result_kind(self.kind, rhs.kind) != Some(FormulaKind::Boolean) {
            return Err(dimension);
        }
        let ordering = self.value.cmp(&rhs.value);
        match operator {
            FormulaBinaryOperator::Equal => Ok(ordering.is_eq()),
            FormulaBinaryOperator::NotEqual => Ok(!ordering.is_eq()),
            FormulaBinaryOperator::Less => Ok(ordering.is_lt()),
            FormulaBinaryOperator::LessEqual => Ok(!ordering.is_gt()),
            FormulaBinaryOperator::Greater => Ok(ordering.is_gt()),
            FormulaBinaryOperator::GreaterEqual => Ok(!ordering.is_lt()),
            _ => Err(dimension),
        }
    }

    /// Exact absolute value, with no rounding or narrowing.
    ///
    /// Length/area domains are symmetric; Count is already nonnegative, and angle/ratio have no
    /// scalar bound. Every admitted input therefore has an admitted absolute value.
    #[must_use]
    pub const fn abs(self) -> Self {
        Self {
            kind: self.kind,
            value: if self.value.is_negative() {
                self.value.negated()
            } else {
                self.value
            },
        }
    }

    /// Returns the least of one or more exact, same-kind numeric operands.
    ///
    /// # Errors
    /// Returns BuiltinArity for an empty slice or BuiltinDimension for the first mismatched kind.
    pub fn checked_min(values: &[Self]) -> Result<Self, FormulaNumericError> {
        Self::extremum(values, FormulaBuiltin::Min)
    }

    /// Returns the greatest of one or more exact, same-kind numeric operands.
    ///
    /// # Errors
    /// Returns BuiltinArity for an empty slice or BuiltinDimension for the first mismatched kind.
    pub fn checked_max(values: &[Self]) -> Result<Self, FormulaNumericError> {
        Self::extremum(values, FormulaBuiltin::Max)
    }

    /// Explicitly limits this value to inclusive exact bounds of its own kind, without rounding.
    ///
    /// # Errors
    /// Returns BuiltinDimension for a mismatched bound or InvalidClamp when low exceeds high.
    pub fn checked_clamp(self, low: Self, high: Self) -> Result<Self, FormulaNumericError> {
        for (argument, bound) in [(1, low), (2, high)] {
            if bound.kind != self.kind {
                return Err(FormulaNumericError::BuiltinDimension {
                    builtin: FormulaBuiltin::Clamp,
                    argument,
                    wanted: self.kind,
                    actual: bound.kind,
                });
            }
        }
        if low.value > high.value {
            return Err(FormulaNumericError::InvalidClamp {
                kind: self.kind,
                low: low.value,
                high: high.value,
            });
        }
        if self.value < low.value {
            Ok(low)
        } else if self.value > high.value {
            Ok(high)
        } else {
            Ok(self)
        }
    }

    /// Explicitly quantizes to the nearest multiple of a same-kind step, ties away from zero.
    ///
    /// # Errors
    /// Returns BuiltinDimension for a different step kind, a zero-divisor refusal for zero step,
    /// or the final reduced-width/scalar-domain refusal. Negative steps admit the same multiples.
    /// This value API does not attach source or irrational/explicit-quantization provenance.
    pub fn checked_round_to(self, step: Self) -> Result<Self, FormulaNumericError> {
        if step.kind != self.kind {
            return Err(FormulaNumericError::BuiltinDimension {
                builtin: FormulaBuiltin::RoundTo,
                argument: 1,
                wanted: self.kind,
                actual: step.kind,
            });
        }
        let value = self.value.checked_round_to(step.value)?;
        Self::for_operation(self.kind, value, "round_to")
    }

    fn extremum(values: &[Self], builtin: FormulaBuiltin) -> Result<Self, FormulaNumericError> {
        let mut selected = values
            .first()
            .copied()
            .ok_or(FormulaNumericError::BuiltinArity {
                builtin,
                actual: values.len(),
            })?;
        for (argument, &value) in values.iter().enumerate().skip(1) {
            if value.kind != selected.kind {
                return Err(FormulaNumericError::BuiltinDimension {
                    builtin,
                    argument,
                    wanted: selected.kind,
                    actual: value.kind,
                });
            }
            let order = value.value.cmp(&selected.value);
            if (builtin == FormulaBuiltin::Min && order.is_lt())
                || (builtin == FormulaBuiltin::Max && order.is_gt())
            {
                selected = value;
            }
        }
        Ok(selected)
    }

    fn binary(
        self,
        operator: FormulaBinaryOperator,
        rhs: Self,
    ) -> Result<Self, FormulaNumericError> {
        let dimension = FormulaNumericError::BinaryDimension {
            operator,
            left: self.kind,
            right: rhs.kind,
        };
        let kind = operator.result_kind(self.kind, rhs.kind).ok_or(dimension)?;
        let value = match operator {
            FormulaBinaryOperator::Add => self.value.checked_add(rhs.value)?,
            FormulaBinaryOperator::Subtract => self.value.checked_sub(rhs.value)?,
            FormulaBinaryOperator::Multiply | FormulaBinaryOperator::Divide => {
                let divide = operator == FormulaBinaryOperator::Divide;
                self.scaled_product(rhs, kind, divide)?
            }
            _ => return Err(dimension), // Private callers supply only the four arithmetic operators.
        };
        Self::for_operation(kind, value, operator.token())
    }

    fn scaled_product(
        self,
        rhs: Self,
        kind: FormulaKind,
        divide: bool,
    ) -> Result<FormulaRational, FormulaRationalError> {
        let left_scale = ratio_scale(self.kind);
        let right_scale = ratio_scale(rhs.kind);
        let result_scale = ratio_scale(kind);
        // Only Ratio is scaled. These products are at most 10^12 and fit u128 exactly.
        let scale = if divide {
            FormulaRational::new(right_scale * result_scale, left_scale, false)?
        } else {
            FormulaRational::new(result_scale, left_scale * right_scale, false)?
        };
        self.value.checked_scaled(rhs.value, scale, divide)
    }
}

fn ratio_scale(kind: FormulaKind) -> u128 {
    if kind == FormulaKind::Ratio {
        1_000_000
    } else {
        1
    }
}
