//! Original runtime causes and bounded-workspace refusals with actual proof-node context.
use crate::recipe::{
    FormulaCheckedNode, FormulaEvaluatedScalar, FormulaEvaluatedTangentError,
    FormulaLengthInputError, FormulaNumericError, FormulaScalarInputError, FormulaSizeReadError,
    FormulaTangentLimits, FormulaToleranceComparisonError, FormulaToleranceReadError,
};
use core::fmt;
use std::collections::TryReserveError;

/// Explicit machine buffer population; these are logical records, not allocator bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaScalarExecutionWorkspace {
    /// Pending iterative node/operation frames, bounded by N.
    Tasks,
    /// Completed intermediate scalar values, bounded by N.
    Values,
}

/// Original value/provider refusal after an actual certified node was selected for execution.
/// Unknown/Derived retain their canonical observation/evaluation requirements; no value is invented.
// Retain the original tangent payload until the existing outer evaluation-error box.
// Its measured>=368-byte variant must not introduce a second diagnostic allocation.
#[allow(clippy::large_enum_variant)]
pub enum FormulaScalarExecutionFailure<'a> {
    /// Existing exact operator/call/conversion refusal, without another rounding boundary.
    Numeric(FormulaNumericError),
    /// Original supplied tangent input,selected policy and native mathematical/resource failure.
    Tangent(FormulaEvaluatedTangentError),
    /// Private runtime guard:actual call lacks a policy despite complete preparation requirements.
    MissingTangentPolicy {
        /// Actual completed operand and its original contributions;no successful Tan is invented.
        input: FormulaEvaluatedScalar,
    },
    /// Original dedicated length read and canonical requirement.
    LengthInput(FormulaLengthInputError<'a>),
    /// Original general scalar read and canonical requirement.
    ScalarInput(FormulaScalarInputError<'a>),
    /// Actual missing/intrinsic reserved tolerance failure.
    ToleranceRead(FormulaToleranceReadError<'a>),
    /// Actual absent size-role value, without a guessed member or default false/zero.
    SizeRead(FormulaSizeReadError),
    /// Existing comparison values, quantitative class/source and approximation refusal.
    Comparison(FormulaToleranceComparisonError<'a>),
    /// A private worker exceeded its admitted logical reservation; no buffer growth/result escapes.
    /// This is an implementation-boundary refusal, not a canonical language diagnostic.
    WorkspaceReservation {
        /// Actual machine population.
        workspace: FormulaScalarExecutionWorkspace,
        /// Complete selected per-buffer reservation.
        admitted: usize,
        /// Actual attempted next population.
        requested: usize,
    },
}
impl fmt::Debug for FormulaScalarExecutionFailure<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Numeric(_) => "Numeric(..)",
            Self::Tangent(_) => "Tangent(..)",
            Self::MissingTangentPolicy { .. } => "MissingTangentPolicy(..)",
            Self::LengthInput(_) => "LengthInput(..)",
            Self::ScalarInput(_) => "ScalarInput(..)",
            Self::ToleranceRead(_) => "ToleranceRead(..)",
            Self::SizeRead(_) => "SizeRead(..)",
            Self::Comparison(_) => "Comparison(..)",
            Self::WorkspaceReservation { .. } => "WorkspaceReservation(..)",
        })
    }
}
impl fmt::Display for FormulaScalarExecutionFailure<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Numeric(error) => error.fmt(f),
            Self::Tangent(error) => error.fmt(f),
            Self::MissingTangentPolicy { .. } => {
                f.write_str("tangent requires explicit execution limits")
            }
            Self::LengthInput(error) => error.fmt(f),
            Self::ScalarInput(error) => error.fmt(f),
            Self::ToleranceRead(error) => error.fmt(f),
            Self::SizeRead(error) => error.fmt(f),
            Self::Comparison(error) => error.fmt(f),
            Self::WorkspaceReservation {
                workspace,
                admitted,
                requested,
            } => write!(
                f,
                "scalar {workspace:?} reservation {admitted}, requested {requested}"
            ),
        }
    }
}
impl std::error::Error for FormulaScalarExecutionFailure<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Numeric(error) => Some(error),
            Self::Tangent(error) => Some(error),
            Self::MissingTangentPolicy { .. } => None,
            Self::LengthInput(error) => error.source(),
            Self::ScalarInput(error) => error.source(),
            Self::ToleranceRead(error) => error.source(),
            Self::SizeRead(error) => Some(error),
            Self::Comparison(error) => error.source(),
            Self::WorkspaceReservation { .. } => None,
        }
    }
}

/// Exact allocator refusal or original value failure; allocation failure itself needs no new box.
#[derive(Debug)]
pub enum FormulaScalarExecutionErrorCause<'a> {
    /// The admitted machine buffer could not be reserved before any execution read.
    Allocation {
        /// Actual buffer being prepared.
        workspace: FormulaScalarExecutionWorkspace,
        /// Actual admitted record count requested.
        requested: usize,
        /// Original allocator failure.
        error: TryReserveError,
    },
    /// An actual executed node failed; no accepted expression or partial read trace escapes.
    /// Boxing keeps uncommon full value/source witnesses out of every Result's Err footprint.
    Evaluation(Box<FormulaScalarExecutionFailure<'a>>),
}

/// Original failing certified node and complete typed cause. No guessed statement ordinal/token.
#[derive(Debug)]
pub struct FormulaScalarExecutionError<'a> {
    node: FormulaCheckedNode<'a, 'a>,
    cause: FormulaScalarExecutionErrorCause<'a>,
    tangent_limits: Option<FormulaTangentLimits>,
}
impl<'a> FormulaScalarExecutionError<'a> {
    pub(super) fn allocation(
        node: FormulaCheckedNode<'a, 'a>,
        workspace: FormulaScalarExecutionWorkspace,
        requested: usize,
        error: TryReserveError,
    ) -> Self {
        Self {
            node,
            tangent_limits: None,
            cause: FormulaScalarExecutionErrorCause::Allocation {
                workspace,
                requested,
                error,
            },
        }
    }
    pub(super) fn evaluation(
        node: FormulaCheckedNode<'a, 'a>,
        failure: FormulaScalarExecutionFailure<'a>,
    ) -> Self {
        Self {
            node,
            tangent_limits: None,
            cause: FormulaScalarExecutionErrorCause::Evaluation(Box::new(failure)),
        }
    }
    pub(super) fn with_tangent_limits(mut self, limits: Option<FormulaTangentLimits>) -> Self {
        self.tangent_limits = limits;
        self
    }
    /// Original selected per-call policy,also retained for allocation or earlier unrelated failure.
    #[must_use]
    pub const fn tangent_limits(&self) -> Option<FormulaTangentLimits> {
        self.tangent_limits
    }
    /// Actual failing source/operator/call; allocation errors retain the actual expression root.
    #[must_use]
    pub const fn node(&self) -> FormulaCheckedNode<'a, 'a> {
        self.node
    }
    /// Complete original failure, including borrowed canonical records/providers and typed values.
    #[must_use]
    pub const fn cause(&self) -> &FormulaScalarExecutionErrorCause<'a> {
        &self.cause
    }
}
impl fmt::Display for FormulaScalarExecutionError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "scalar execution at {:?}: ", self.node.span())?;
        match &self.cause {
            FormulaScalarExecutionErrorCause::Allocation {
                workspace,
                requested,
                error,
            } => write!(f, "{workspace:?} reservation {requested}: {error}"),
            FormulaScalarExecutionErrorCause::Evaluation(error) => error.fmt(f),
        }
    }
}
impl std::error::Error for FormulaScalarExecutionError<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.cause {
            FormulaScalarExecutionErrorCause::Allocation { error, .. } => Some(error),
            FormulaScalarExecutionErrorCause::Evaluation(error) => error.source(),
        }
    }
}
