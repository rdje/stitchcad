//! Lazy supplied-value control; complete AST static validation remains a caller prerequisite.
use super::FormulaEvaluatedScalar as V;
use crate::recipe::{FormulaBuiltin, FormulaBuiltinOperand, FormulaKind};
use core::fmt;

/// The selected branch in the existing conditional special form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaConditionalBranch {
    /// The branch selected by Boolean true.
    Then,
    /// The mandatory alternative selected by Boolean false.
    Else,
}

/// Actual metadata, selected computation or returned-kind refusal; no fallback value is supplied.
#[derive(Debug)]
pub enum FormulaConditionalValueError<E> {
    /// Condition/branch metadata has no admitted if signature. Neither callback ran.
    InputKinds {
        /// Actual already-computed condition kind.
        condition: FormulaKind,
        /// Caller-supplied then-branch static kind.
        then_kind: FormulaKind,
        /// Caller-supplied else-branch static kind.
        else_kind: FormulaKind,
    },
    /// Only the selected callback ran and returned its original failure.
    Computation {
        /// Actual branch chosen by the Boolean condition.
        branch: FormulaConditionalBranch,
        /// Original caller error, boxed without transforming its data or token.
        source: Box<E>,
    },
    /// A selected callback violated its supplied static-kind contract.
    OutputKind {
        /// Actual selected branch; the other callback never ran.
        branch: FormulaConditionalBranch,
        /// Required kind from the two matching branch annotations.
        declared: FormulaKind,
        /// Actual returned kind, including a Boolean masquerading as arithmetic.
        actual: FormulaKind,
    },
}
impl<E> FormulaConditionalValueError<E> {
    /// Kind errors use formula_dimension. Computation diagnostics belong to the original error.
    #[must_use]
    pub const fn diagnostic_code(&self) -> Option<&'static str> {
        match self {
            Self::InputKinds { .. } | Self::OutputKind { .. } => Some("formula_dimension"),
            Self::Computation { .. } => None,
        }
    }

    /// Original selected computation error, without requiring it to have a static lifetime.
    #[must_use]
    pub fn computation_error(&self) -> Option<&E> {
        match self {
            Self::Computation { source, .. } => Some(source),
            _ => None,
        }
    }
}

impl<E: fmt::Display> fmt::Display for FormulaConditionalValueError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputKinds {
                condition,
                then_kind,
                else_kind,
            } => write!(
                f,
                "formula_dimension: if condition={}, then={}, else={}",
                condition.token(),
                then_kind.token(),
                else_kind.token()
            ),
            Self::OutputKind {
                branch,
                declared,
                actual,
            } => write!(
                f,
                "formula_dimension: if {branch:?} declared {}, returned {}",
                declared.token(),
                actual.token()
            ),
            Self::Computation { source, .. } => fmt::Display::fmt(source, f),
        }
    }
}
impl<E: std::error::Error + 'static> std::error::Error for FormulaConditionalValueError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.computation_error()
            .map(|source| source as &dyn std::error::Error)
    }
}

impl V {
    /// Validate supplied branch kinds, then evaluate exactly one callback and retain condition sources.
    ///
    /// The complete expression must already have passed name/structure/kind checks for both branches.
    /// This value primitive validates only supplied metadata and the selected callback's result.
    /// It performs no namespace/provider lookup and imposes no statement/atomic-publication policy.
    /// # Errors
    /// Returns actual kind metadata before either callback, the unchanged selected callback error,
    /// or an actual output-kind contract violation. An untaken callback is never invoked.
    pub fn checked_if<E>(
        self,
        then_kind: FormulaKind,
        else_kind: FormulaKind,
        then_value: impl FnOnce() -> Result<Self, E>,
        else_value: impl FnOnce() -> Result<Self, E>,
    ) -> Result<Self, FormulaConditionalValueError<E>> {
        let kinds = [self.kind(), then_kind, else_kind].map(FormulaBuiltinOperand::Value);
        let input_error = || FormulaConditionalValueError::InputKinds {
            condition: self.kind(),
            then_kind,
            else_kind,
        };
        let declared = FormulaBuiltin::If
            .result_kind(&kinds)
            .ok_or_else(input_error)?;
        let condition = self.boolean().ok_or_else(input_error)?;
        let (branch, result) = if condition {
            (FormulaConditionalBranch::Then, then_value())
        } else {
            (FormulaConditionalBranch::Else, else_value())
        };
        let value = result.map_err(|source| FormulaConditionalValueError::Computation {
            branch,
            source: Box::new(source),
        })?;
        if value.kind() != declared {
            return Err(FormulaConditionalValueError::OutputKind {
                branch,
                declared,
                actual: value.kind(),
            });
        }
        Ok(value.influenced_by(self.sources()))
    }
}
