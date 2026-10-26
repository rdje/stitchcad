//! Scalar value computation with immutable approximation sources; no AST or namespace execution.
use super::{
    FormulaApproximationSources, FormulaBinaryOperator, FormulaBindingValueError,
    FormulaBoundValue, FormulaBuiltin, FormulaKind, FormulaNumericError, FormulaNumericValue,
    FormulaUnaryOperator,
};
use crate::name::MachineToken;

mod calls;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum ScalarValue {
    Numeric(FormulaNumericValue),
    Boolean(bool),
}

/// A completed numeric or Boolean value together with its approximation-call sources.
///
/// This is a value primitive, not proof of AST execution or input evidence. Source attachment
/// for successful calls is explicit; numerical results still use their existing exact kernels.
/// Ordinary operators automatically retain both operands' sources, even after cancellation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FormulaEvaluatedScalar {
    value: ScalarValue,
    sources: FormulaApproximationSources,
}
impl FormulaEvaluatedScalar {
    /// Exact numeric value with no approximation-producing dependency.
    #[must_use]
    pub const fn from_numeric(value: FormulaNumericValue) -> Self {
        Self {
            value: ScalarValue::Numeric(value),
            sources: FormulaApproximationSources::EMPTY,
        }
    }

    /// Actual Boolean with no approximation-producing dependency; false is present.
    #[must_use]
    pub const fn from_boolean(value: bool) -> Self {
        Self {
            value: ScalarValue::Boolean(value),
            sources: FormulaApproximationSources::EMPTY,
        }
    }

    /// Actual scalar kind; geometry references are outside this primitive.
    #[must_use]
    pub const fn kind(self) -> FormulaKind {
        match self.value {
            ScalarValue::Numeric(value) => value.kind(),
            ScalarValue::Boolean(_) => FormulaKind::Boolean,
        }
    }

    /// Numeric payload, preserving its exact internal units; None for a Boolean.
    #[must_use]
    pub const fn numeric(self) -> Option<FormulaNumericValue> {
        match self.value {
            ScalarValue::Numeric(value) => Some(value),
            ScalarValue::Boolean(_) => None,
        }
    }

    /// Actual Boolean payload; numeric zero/one is never coerced.
    #[must_use]
    pub const fn boolean(self) -> Option<bool> {
        match self.value {
            ScalarValue::Boolean(value) => Some(value),
            ScalarValue::Numeric(_) => None,
        }
    }

    /// Approximation-producing calls retained beside this value.
    #[must_use]
    pub const fn sources(self) -> FormulaApproximationSources {
        self.sources
    }

    /// Retain additional executed dependencies without removing any existing contribution.
    #[must_use]
    pub const fn influenced_by(self, sources: FormulaApproximationSources) -> Self {
        Self {
            sources: self.sources.union(sources),
            ..self
        }
    }

    /// Attach a successful call's own source; this does not compute or certify that call.
    #[must_use]
    pub const fn with_call_source(self, call: FormulaBuiltin) -> Self {
        self.influenced_by(FormulaApproximationSources::for_call(call))
    }

    /// Compute a unary numeric operator and retain every input contribution.
    /// # Errors
    /// Returns the actual operator/kind or existing numerical-domain/width refusal.
    pub fn checked_unary(
        self,
        operator: FormulaUnaryOperator,
    ) -> Result<Self, FormulaNumericError> {
        let Some(value) = self.numeric() else {
            return Err(FormulaNumericError::UnaryDimension {
                operator,
                kind: self.kind(),
            });
        };
        let result = match operator {
            FormulaUnaryOperator::Negate => value.checked_neg()?,
            FormulaUnaryOperator::Square => value.checked_square()?,
        };
        Ok(Self::from_numeric(result).influenced_by(self.sources))
    }

    /// Compute an ordinary binary operator; exact comparisons produce a sourced Boolean.
    ///
    /// This invokes no tolerance-class rule. A later named tolerance comparison checks the
    /// retained sources separately. Both operands are already evaluated by the caller.
    /// # Errors
    /// Returns actual operator/kinds or the unchanged numeric operation refusal.
    pub fn checked_binary(
        self,
        operator: FormulaBinaryOperator,
        right: Self,
    ) -> Result<Self, FormulaNumericError> {
        let (Some(left), Some(rhs)) = (self.numeric(), right.numeric()) else {
            return Err(FormulaNumericError::BinaryDimension {
                operator,
                left: self.kind(),
                right: right.kind(),
            });
        };
        use FormulaBinaryOperator as B;
        let result = match operator {
            B::Add => Self::from_numeric(left.checked_add(rhs)?),
            B::Subtract => Self::from_numeric(left.checked_sub(rhs)?),
            B::Multiply => Self::from_numeric(left.checked_mul(rhs)?),
            B::Divide => Self::from_numeric(left.checked_div(rhs)?),
            B::Equal | B::NotEqual | B::Less | B::LessEqual | B::Greater | B::GreaterEqual => {
                Self::from_boolean(left.checked_compare(operator, rhs)?)
            }
        };
        Ok(result.influenced_by(self.sources.union(right.sources)))
    }

    /// Store one scalar, rounding numeric payloads once and preserving every contribution.
    /// # Errors
    /// Propagates the existing actual-name/kind/rounded-storage refusal; publishes no binding.
    pub fn bind(
        self,
        name: &MachineToken,
    ) -> Result<FormulaEvaluatedBinding, FormulaBindingValueError<'_>> {
        let value = match self.value {
            ScalarValue::Numeric(value) => FormulaBoundValue::from_numeric(name, value)?,
            ScalarValue::Boolean(value) => FormulaBoundValue::from_boolean(value),
        };
        Ok(FormulaEvaluatedBinding {
            value,
            sources: self.sources,
        })
    }
}

/// Once-rounded stored scalar with the same approximation sources its expression carried.
/// No name/scope publication, annotation proof or recipe replay is implied by construction.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FormulaEvaluatedBinding {
    value: FormulaBoundValue,
    sources: FormulaApproximationSources,
}
impl FormulaEvaluatedBinding {
    /// Actual stored integer or Boolean; no pre-binding fraction is retained.
    #[must_use]
    pub const fn value(self) -> FormulaBoundValue {
        self.value
    }

    /// Original expression's complete contribution set.
    #[must_use]
    pub const fn sources(self) -> FormulaApproximationSources {
        self.sources
    }

    /// Read the stored value in exact internal units, preserving every source.
    /// # Errors
    /// Propagates the existing numeric read's domain refusal without inventing a value.
    pub fn read(self) -> Result<FormulaEvaluatedScalar, FormulaNumericError> {
        let value = if let Some(value) = self.value.boolean() {
            FormulaEvaluatedScalar::from_boolean(value)
        } else {
            FormulaEvaluatedScalar::from_numeric(FormulaNumericValue::try_from(self.value)?)
        };
        Ok(value.influenced_by(self.sources))
    }
}
