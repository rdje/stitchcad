//! Actual whole scalar execution refusal; no partial accepted output or invented ordinal.
use crate::recipe::{
    FormulaBindingValueRule, FormulaCheckedRecipe, FormulaCheckedStatement, FormulaKind,
    FormulaScalarExecutionError, FormulaStatementExpression, FormulaToleranceComparison,
    FormulaToleranceComparisonError, FormulaToleranceReadError,
};
use core::fmt;
use std::collections::TryReserveError;

/// Complete runtime buffer population; source records were reserved by whole preparation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaScalarRecipeWorkspace {
    /// N iterative pending frames.
    Tasks,
    /// N intermediate scalar slots.
    Values,
    /// S successful statement outcomes, including assertions.
    Statements,
}

/// Original semantic failure at one actual whole-proof statement.
pub enum FormulaScalarRecipeFailure<'a> {
    /// Original failing operand/node/cause; statement role is supplied by this real enclosing proof.
    Expression {
        /// Original binding RHS or assertion left/right.
        part: FormulaStatementExpression,
        /// Actual shared-machine error, retaining original proof node and typed cause.
        error: FormulaScalarExecutionError<'a>,
    },
    /// Actual once-only numeric binding/storage refusal; original name comes from the statement.
    Binding {
        /// Kind of the actual completed RHS.
        kind: FormulaKind,
        /// Original exact storage/rounding rule and full measured witness.
        rule: FormulaBindingValueRule,
    },
    /// Original optional/intrinsic tolerance lookup failure after both assertion operands succeed.
    ToleranceRead(FormulaToleranceReadError<'a>),
    /// Actual operands/class/provider and dimensional or approximation refusal.
    Comparison(FormulaToleranceComparisonError<'a>),
    /// False actual named assertion, retaining both values and original quantitative tolerance.
    /// Its canonical diagnostic family is formula_assertion; this is no accepted false outcome.
    Assertion(FormulaToleranceComparison<'a>),
}
impl fmt::Debug for FormulaScalarRecipeFailure<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Expression { .. } => "Expression(..)",
            Self::Binding { .. } => "Binding(..)",
            Self::ToleranceRead(_) => "ToleranceRead(..)",
            Self::Comparison(_) => "Comparison(..)",
            Self::Assertion(_) => "Assertion(..)",
        })
    }
}
impl fmt::Display for FormulaScalarRecipeFailure<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Expression { part, error } => write!(f, "{part:?}: {error}"),
            Self::Binding { kind, rule } => write!(f, "{} binding: {rule:?}", kind.token()),
            Self::ToleranceRead(error) => error.fmt(f),
            Self::Comparison(error) => error.fmt(f),
            Self::Assertion(_) => f.write_str("formula_assertion"),
        }
    }
}
impl std::error::Error for FormulaScalarRecipeFailure<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Expression { error, .. } => error.source(),
            Self::Binding {
                rule: FormulaBindingValueRule::Rounding(error),
                ..
            } => Some(error),
            Self::ToleranceRead(error) => error.source(),
            Self::Comparison(error) => error.source(),
            Self::Binding { .. } | Self::Assertion(_) => None,
        }
    }
}

/// Allocation remains inline, so reporting it requires no new diagnostic allocation.
#[derive(Debug)]
pub enum FormulaScalarRecipeErrorCause<'a> {
    /// Complete selected buffer could not reserve before any runtime read.
    Allocation {
        /// Actual logical buffer.
        workspace: FormulaScalarRecipeWorkspace,
        /// Complete requested logical population, not allocator bytes.
        requested: usize,
        /// Original fallible reservation error.
        error: TryReserveError,
    },
    /// Original semantic failure with no accepted partial prefix; inspect the actual statement.
    Evaluation(Box<FormulaScalarRecipeFailure<'a>>),
}

/// Complete original proof and genuine available statement context at a refused whole execution.
#[derive(Debug)]
pub struct FormulaScalarRecipeError<'a> {
    proof: &'a FormulaCheckedRecipe<'a>,
    statement: Option<&'a FormulaCheckedStatement<'a>>,
    cause: FormulaScalarRecipeErrorCause<'a>,
    tangent_limits: Option<crate::recipe::FormulaTangentLimits>,
}
impl<'a> FormulaScalarRecipeError<'a> {
    pub(super) fn allocation(
        proof: &'a FormulaCheckedRecipe<'a>,
        workspace: FormulaScalarRecipeWorkspace,
        requested: usize,
        error: TryReserveError,
    ) -> Self {
        Self {
            proof,
            tangent_limits: None,
            statement: None,
            cause: FormulaScalarRecipeErrorCause::Allocation {
                workspace,
                requested,
                error,
            },
        }
    }
    pub(super) fn evaluation(
        proof: &'a FormulaCheckedRecipe<'a>,
        statement: &'a FormulaCheckedStatement<'a>,
        failure: FormulaScalarRecipeFailure<'a>,
    ) -> Self {
        Self {
            proof,
            tangent_limits: None,
            statement: Some(statement),
            cause: FormulaScalarRecipeErrorCause::Evaluation(Box::new(failure)),
        }
    }
    pub(super) fn with_tangent_limits(
        mut self,
        limits: Option<crate::recipe::FormulaTangentLimits>,
    ) -> Self {
        self.tangent_limits = limits;
        self
    }
    /// Original recipe-owned per-call policy,including allocation and earlier non-Tan failures.
    #[must_use]
    pub const fn tangent_limits(&self) -> Option<crate::recipe::FormulaTangentLimits> {
        self.tangent_limits
    }
    /// Original complete whole static proof; no successful output prefix escapes.
    #[must_use]
    pub const fn proof(&self) -> &'a FormulaCheckedRecipe<'a> {
        self.proof
    }
    /// Real refused statement and ordinal; aggregate pre-read allocation supplies none.
    #[must_use]
    pub const fn statement(&self) -> Option<&'a FormulaCheckedStatement<'a>> {
        self.statement
    }
    /// Full original allocation or semantic cause, with actual value/source/provider data.
    #[must_use]
    pub const fn cause(&self) -> &FormulaScalarRecipeErrorCause<'a> {
        &self.cause
    }
}
impl fmt::Display for FormulaScalarRecipeError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(statement) = self.statement {
            write!(
                f,
                "scalar recipe statement {} ({}): ",
                statement.statement_index(),
                statement.statement().name()
            )?;
        } else {
            f.write_str("scalar recipe before reads: ")?;
        }
        match &self.cause {
            FormulaScalarRecipeErrorCause::Allocation {
                workspace,
                requested,
                error,
            } => write!(f, "{workspace:?} reservation {requested}: {error}"),
            FormulaScalarRecipeErrorCause::Evaluation(failure) => failure.fmt(f),
        }
    }
}
impl std::error::Error for FormulaScalarRecipeError<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.cause {
            FormulaScalarRecipeErrorCause::Allocation { error, .. } => Some(error),
            FormulaScalarRecipeErrorCause::Evaluation(failure) => failure.source(),
        }
    }
}
