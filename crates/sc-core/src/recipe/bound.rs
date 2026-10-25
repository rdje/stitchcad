//! One numeric storage conversion and immutable Boolean storage; no namespace mutation or replay.
use super::{FormulaKind, FormulaNumericValue, FormulaRational};
use crate::name::MachineToken;
use core::fmt;
use sc_units::{round::div_round_half_away_from_zero_unsigned, UnitError};

/// Actual refusal while converting a completed numeric value to its binding representation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaBindingValueRule {
    /// The shared rounding primitive refused; no fallback integer is substituted.
    Rounding(UnitError),
    /// The once-rounded integer exceeds the kind's signed storage range.
    Storage {
        /// Inclusive storage lower bound, zero for Count and i64 MIN otherwise.
        minimum: i64,
        /// Inclusive signed i64 upper bound.
        maximum: i64,
        /// Exact rounded integer, retaining its full unsigned magnitude and independent sign.
        measured: FormulaRational,
    },
}

/// Binding conversion refusal with the actual caller-supplied name and numeric kind.
///
/// The name is borrowed without copying or inventing an expression/statement source. This primitive
/// does not prove that the name can be declared or that any namespace mutation was attempted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormulaBindingValueError<'a> {
    name: &'a MachineToken,
    kind: FormulaKind,
    rule: FormulaBindingValueRule,
}

impl<'a> FormulaBindingValueError<'a> {
    /// Actual attempted binding name supplied to the conversion.
    #[must_use]
    pub const fn name(self) -> &'a MachineToken {
        self.name
    }

    /// Numeric kind of the completed value being stored.
    #[must_use]
    pub const fn kind(self) -> FormulaKind {
        self.kind
    }

    /// Typed refusal with actual bounds/rounded measurement when storage was exceeded.
    #[must_use]
    pub const fn rule(self) -> FormulaBindingValueRule {
        self.rule
    }

    /// Stable diagnostic family, without localized presentation or invented source context.
    #[must_use]
    pub const fn diagnostic_code(self) -> &'static str {
        match self.rule {
            FormulaBindingValueRule::Rounding(UnitError::DivisionByZero { .. }) => {
                "formula_division"
            }
            _ => "formula_domain",
        }
    }
}

impl fmt::Display for FormulaBindingValueError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: let {} {} binding: {:?}",
            self.diagnostic_code(),
            self.name,
            self.kind.token(),
            self.rule
        )
    }
}

impl std::error::Error for FormulaBindingValueError<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.rule {
            FormulaBindingValueRule::Rounding(error) => Some(error),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum StoredValue {
    Numeric { kind: FormulaKind, integer: i64 },
    Boolean(bool),
}

/// Immutable scalar in its binding representation, separate from exact expression values.
///
/// Numeric construction rounds once, checks storage, then returns the integer subsequent reads
/// must use. Boolean construction retains Boolean kind. No operation publishes into an environment,
/// checks an annotation/name scope, normalizes angle directions or executes a statement.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FormulaBoundValue(StoredValue);

impl FormulaBoundValue {
    /// Rounds a completed numeric value once, half away from zero, and checks signed i64 storage.
    ///
    /// # Errors
    /// Returns the actual name/kind/rounded measurement and storage bounds on refusal. Count remains
    /// nonnegative. Length/area's exact inclusive integer scalar bounds are already proven by the
    /// completed input; nearest integer rounding cannot cross either bound.
    pub fn from_numeric<'a>(
        name: &'a MachineToken,
        value: FormulaNumericValue,
    ) -> Result<Self, FormulaBindingValueError<'a>> {
        let kind = value.kind();
        let rational = value.rational();
        let magnitude = div_round_half_away_from_zero_unsigned(
            rational.numerator_magnitude(),
            rational.denominator(),
        )
        .map_err(|error| FormulaBindingValueError {
            name,
            kind,
            rule: FormulaBindingValueRule::Rounding(error),
        })?;
        let minimum = if kind == FormulaKind::Count {
            0
        } else {
            i64::MIN
        };
        let maximum = i64::MAX;
        let negative = rational.is_negative();
        let storage_error = || FormulaBindingValueError {
            name,
            kind,
            rule: FormulaBindingValueRule::Storage {
                minimum,
                maximum,
                measured: FormulaRational::from_integer_magnitude(magnitude, negative),
            },
        };
        let limit = if negative {
            minimum.unsigned_abs()
        } else {
            maximum.unsigned_abs()
        };
        if magnitude > u128::from(limit) {
            return Err(storage_error());
        }
        let integer = if negative && magnitude == u128::from(i64::MIN.unsigned_abs()) {
            i64::MIN
        } else {
            let integer = i64::try_from(magnitude).map_err(|_| storage_error())?;
            if negative {
                -integer
            } else {
                integer
            }
        };
        Ok(Self(StoredValue::Numeric { kind, integer }))
    }

    /// Stores a Boolean directly, without converting it to a numeric kind or performing rounding.
    #[must_use]
    pub const fn from_boolean(value: bool) -> Self {
        Self(StoredValue::Boolean(value))
    }

    /// Actual stored scalar kind; geometry references cannot be constructed here.
    #[must_use]
    pub const fn kind(self) -> FormulaKind {
        match self.0 {
            StoredValue::Numeric { kind, .. } => kind,
            StoredValue::Boolean(_) => FormulaKind::Boolean,
        }
    }

    /// Once-rounded stored numeric integer, or None for a Boolean. Later reads use this value.
    #[must_use]
    pub const fn numeric_integer(self) -> Option<i64> {
        match self.0 {
            StoredValue::Numeric { integer, .. } => Some(integer),
            StoredValue::Boolean(_) => None,
        }
    }

    /// Stored Boolean, or None for any numeric kind.
    #[must_use]
    pub const fn boolean(self) -> Option<bool> {
        match self.0 {
            StoredValue::Boolean(value) => Some(value),
            StoredValue::Numeric { .. } => None,
        }
    }
}
