//! Whole-source evaluation failures preserve the actual scope,selection and original scalar cause.
use super::{FormulaScalarExecutionError, FormulaSourceGraphExecutionLimits};
use crate::{
    ontology::EntityId,
    recipe::{FormulaAcyclicSourceDependencies, FormulaSourceSpan},
};
use core::fmt;
use std::collections::TryReserveError;

/// Runtime logical buffer population,not allocator bytes or kernel/CPU resource bounds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaScalarSourceGraphWorkspace {
    /// One admitted scalar machine control per source.
    Sources,
    /// At most one active original source index per acyclic call path.
    CallFrames,
    /// One optional actual completed root/ordinal per source,initially empty.
    Outputs,
    /// Complete actual task reservation for one original source machine.
    Tasks,
    /// Complete actual value reservation for one original source machine.
    Values,
}
/// Original whole-source refusal;no accepted partial result or reusable cache escapes.
#[derive(Debug)]
pub enum FormulaScalarSourceGraphExecutionCause<'a> {
    /// The selected id is absent from the actual source registry;no runtime allocation/read occurs.
    MissingSource,
    /// Full runtime preparation failed before the first value,with the original inline allocator cause.
    Allocation {
        /// Actual population being reserved.
        workspace: FormulaScalarSourceGraphWorkspace,
        /// Actual original source position for Tasks/Values;outer reservations have no failing source.
        source_index: Option<usize>,
        /// Complete admitted population requested.
        requested: usize,
        /// Original fallible reservation failure,without another diagnostic allocation.
        error: TryReserveError,
    },
    /// Original executed source and failing scalar node/value/provider cause.
    Evaluation {
        /// Actual zero-based original supplier/consumer source position.
        source_index: usize,
        /// Existing complete scalar failure;its borrowed owner/typed causes remain inspectable.
        error: Box<FormulaScalarExecutionError<'a>>,
    },
    /// Private runtime frame guard exceeded the admitted S bound;no buffer growth is permitted.
    FrameReservation {
        /// Actual consumer source position.
        source_index: usize,
        /// Original requesting name's local span.
        span: FormulaSourceSpan,
        /// Complete admitted source/frame population.
        admitted: usize,
        /// Attempted next frame population.
        requested: usize,
    },
}
/// Original complete graph/limits and exact source selection that failed;not a language-cycle token.
#[derive(Debug)]
pub struct FormulaScalarSourceGraphExecutionError<'a> {
    pub(super) graph: &'a FormulaAcyclicSourceDependencies<'a>,
    pub(super) limits: FormulaSourceGraphExecutionLimits,
    pub(super) tangent_limits: Option<crate::recipe::FormulaTangentLimits>,
    pub(super) source: EntityId,
    pub(super) cause: FormulaScalarSourceGraphExecutionCause<'a>,
}
impl<'a> FormulaScalarSourceGraphExecutionError<'a> {
    /// Actual same admitted acyclic relation and original source/input owners.
    #[must_use]
    pub const fn graph(&self) -> &'a FormulaAcyclicSourceDependencies<'a> {
        self.graph
    }
    /// Actual explicit complete runtime allowances,not canonical content.
    #[must_use]
    pub const fn limits(&self) -> FormulaSourceGraphExecutionLimits {
        self.limits
    }
    /// Original selected per-call tangent policy,including selection or allocation failure.
    #[must_use]
    pub const fn tangent_limits(&self) -> Option<crate::recipe::FormulaTangentLimits> {
        self.tangent_limits
    }
    /// Exact original requested entry id;no parameter/canonical-id or text fallback.
    #[must_use]
    pub const fn source_id(&self) -> EntityId {
        self.source
    }
    /// Complete original selection,allocation,scalar or logical frame failure.
    #[must_use]
    pub const fn cause(&self) -> &FormulaScalarSourceGraphExecutionCause<'a> {
        &self.cause
    }
}
impl fmt::Display for FormulaScalarSourceGraphExecutionError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "scalar source graph {:?}: ", self.source)?;
        match &self.cause {
            FormulaScalarSourceGraphExecutionCause::MissingSource => {
                f.write_str("source not found")
            }
            FormulaScalarSourceGraphExecutionCause::Allocation {
                workspace,
                source_index,
                requested,
                error,
            } => write!(
                f,
                "{workspace:?} reservation {requested} at source {source_index:?}: {error}"
            ),
            FormulaScalarSourceGraphExecutionCause::Evaluation {
                source_index,
                error,
            } => write!(f, "source {source_index}: {error}"),
            FormulaScalarSourceGraphExecutionCause::FrameReservation {
                source_index,
                span,
                admitted,
                requested,
            } => write!(
                f,
                "source {source_index} at {span:?}:call frames {admitted},requested {requested}"
            ),
        }
    }
}
impl std::error::Error for FormulaScalarSourceGraphExecutionError<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        cause_source(&self.cause)
    }
}

pub(super) fn cause_source<'e>(
    cause: &'e FormulaScalarSourceGraphExecutionCause<'_>,
) -> Option<&'e (dyn std::error::Error + 'static)> {
    match cause {
        FormulaScalarSourceGraphExecutionCause::Allocation { error, .. } => Some(error),
        FormulaScalarSourceGraphExecutionCause::Evaluation { error, .. } => {
            std::error::Error::source(error.as_ref())
        }
        _ => None,
    }
}
