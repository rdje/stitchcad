//! Borrowed actual original-notch context and captured command quantities.
use super::{CommandEaseCapture, CommandEaseQuantityStage, CommandSeamSplitPair, SplitSeamSpan};
use crate::ontology::{
    EdgeAnchor, EdgeRange, EntityId, NotchEaseAdmission, NotchEaseAmountError, NotchEaseLimits,
    NotchEaseWorkspaceError, OriginalNotchContext, SeamEaseSource, SeamSideId, SeamSpan,
};
use core::fmt;
use sc_units::Length;

/// Actual capture/context mismatch or original numeric admission/query cause.
/// The context's earlier source/inverse validation errors remain unchanged at their staged builder.
#[derive(Debug)]
pub enum CommandNotchEaseError<'a> {
    /// Context does not borrow the exact captured original source;matching IDs do not suffice.
    ContextSource {
        /// Actual consuming graph member.
        target_span: EntityId,
        /// Expected immutable captured source,without cloning its metadata.
        expected: &'a SeamEaseSource,
        /// Actual supplied context,including its own source/ledger/projection evidence.
        context: &'a OriginalNotchContext<'a>,
    },
    /// Original signed value or separately selected numeric reservation refusal.
    Workspace {
        /// Actual consuming graph member,possibly distinct from the original source.
        target_span: EntityId,
        /// Original refusal,also the standard error source.
        issue: NotchEaseWorkspaceError,
    },
    /// Actual original-domain query refusal;no accepted partial partition escapes.
    Quantity {
        /// Actual consuming graph member.
        target_span: EntityId,
        /// Concrete failed target/pair/boundary/interval query.
        stage: CommandEaseQuantityStage,
        /// Original source/frame/bounds/numeric cause,also the standard error source.
        issue: Box<NotchEaseAmountError>,
    },
}
impl fmt::Display for CommandNotchEaseError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ContextSource { target_span, expected, context } => write!(f,
                "notch command target {target_span} requires actual captured source {};context borrows source {}",
                expected.span(),context.validation().admission().source().span()),
            Self::Workspace { target_span, issue } => write!(f,
                "notch command target {target_span} admission: {issue}"),
            Self::Quantity { target_span, stage, issue } => write!(f,
                "notch command target {target_span} {stage:?}: {issue}"),
        }
    }
}
impl std::error::Error for CommandNotchEaseError<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ContextSource {
                target_span: _,
                expected: _,
                context: _,
            } => None,
            Self::Workspace {
                target_span: _,
                issue,
            } => Some(issue),
            Self::Quantity {
                target_span: _,
                stage: _,
                issue,
            } => Some(issue.as_ref()),
        }
    }
}
fn failure<'a>(
    target_span: EntityId,
    stage: CommandEaseQuantityStage,
    issue: NotchEaseAmountError,
) -> CommandNotchEaseError<'a> {
    CommandNotchEaseError::Quantity {
        target_span,
        stage,
        issue: Box::new(issue),
    }
}

/// Borrowed actual capture and sealed original-notch context,with exact target/pair amounts.
/// The context owns its projections externally;the view borrows them through its quantity admission.
/// No source/projection clone,self-reference or child normalization is needed. The other supplied
/// cut stays descriptive;physical correspondence,currentness,authority and effects remain separate.
///
/// Private fields prevent fabricated accepted quantities:
/// ```compile_fail
/// use sc_core::command::{CommandEaseCapture,CommandNotchEasePartition};
/// use sc_core::ontology::NotchEaseAdmission;
/// fn forge<'a>(capture:&'a CommandEaseCapture<'a>,admission:NotchEaseAdmission<'a>)
///     ->CommandNotchEasePartition<'a>{
///     CommandNotchEasePartition {capture,admission,parent:sc_units::Length::ZERO,
///         first:sc_units::Length::ZERO,second:sc_units::Length::ZERO}
/// }
/// ```
/// The actual context borrow cannot escape:
/// ```compile_fail
/// use sc_core::command::{CommandEaseCapture,CommandNotchEasePartition};
/// use sc_core::ontology::{OriginalNotchContext,NotchEaseLimits};
/// fn escape(capture:&CommandEaseCapture<'_>,context:&OriginalNotchContext<'_>,limits:NotchEaseLimits)
///     ->CommandNotchEasePartition<'static>{capture.notch_explicit_ease(context,limits).unwrap()}
/// ```
#[derive(Clone, Copy)]
pub struct CommandNotchEasePartition<'a> {
    capture: &'a CommandEaseCapture<'a>,
    admission: NotchEaseAdmission<'a>,
    parent: Length,
    first: Length,
    second: Length,
}
impl fmt::Debug for CommandNotchEasePartition<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            capture,
            admission,
            parent,
            first,
            second,
        } = self;
        let source = admission.context().validation().admission();
        f.debug_struct("CommandNotchEasePartition")
            .field("target", &capture.binding().span().id())
            .field("source", &source.source().span())
            .field("side", &source.distribution_side())
            .field("notches", &source.notch_ids())
            .field("parent", parent)
            .field("first", first)
            .field("second", second)
            .finish()
    }
}
impl<'a> CommandNotchEasePartition<'a> {
    /// Exact captured owner;no same-ID reconstruction.
    #[must_use]
    pub const fn capture(&self) -> &'a CommandEaseCapture<'a> {
        self.capture
    }
    /// Actual original request.
    #[must_use]
    pub fn request(&self) -> &'a SplitSeamSpan {
        self.capture.binding().request()
    }
    /// Actual requested graph member with its copy/domain/direction metadata.
    #[must_use]
    pub fn span(&self) -> &'a SeamSpan {
        self.capture.binding().span()
    }
    /// Complete original source,not the rounded child allocation.
    #[must_use]
    pub const fn original_source(&self) -> &'a SeamEaseSource {
        self.admission.context().validation().admission().source()
    }
    /// Actual context/ledger/named Notches/projections,without cloning any of them.
    #[must_use]
    pub const fn original_context(&self) -> &'a OriginalNotchContext<'a> {
        self.admission.context()
    }
    /// Original separate numeric admission and selected allowance.
    #[must_use]
    pub const fn notch_admission(&self) -> NotchEaseAdmission<'a> {
        self.admission
    }
    /// Actual original declared distribution side.
    #[must_use]
    pub const fn distribution_side(&self) -> SeamSideId {
        self.original_context()
            .validation()
            .admission()
            .distribution_side()
    }
    /// Full original signed literal,distinct from target allocation.
    #[must_use]
    pub const fn source_parent_amount(&self) -> Length {
        self.admission.parent_amount()
    }
    /// Requested target allocation from original rounded cumulative endpoints.
    #[must_use]
    pub const fn parent_amount(&self) -> Length {
        self.parent
    }
    /// First projected pair allocation in canonical A order.
    #[must_use]
    pub const fn first_amount(&self) -> Length {
        self.first
    }
    /// Second projected pair allocation in canonical A order.
    #[must_use]
    pub const fn second_amount(&self) -> Length {
        self.second
    }
    /// Actual descriptive pairs,unchanged by the quantity calculation.
    #[must_use]
    pub fn paired_intervals(&self) -> [CommandSeamSplitPair; 2] {
        self.capture.binding().paired_intervals()
    }
    /// Full original density-side cumulative query,including outside the requested target.
    /// # Errors
    /// Preserves original cause and actual consuming target/boundary stage.
    pub fn cumulative_at(&self, at: EdgeAnchor) -> Result<Length, CommandNotchEaseError<'a>> {
        self.admission
            .cumulative_at(at)
            .map_err(|issue| failure(self.span().id(), CommandEaseQuantityStage::Boundary, issue))
    }
    /// Full original density-side interval query;never renormalizes the rounded child.
    /// # Errors
    /// Preserves original cause and actual consuming target/interval stage.
    pub fn amount_in(&self, interval: EdgeRange) -> Result<Length, CommandNotchEaseError<'a>> {
        self.admission
            .amount_in(interval)
            .map_err(|issue| failure(self.span().id(), CommandEaseQuantityStage::Interval, issue))
    }
}
impl CommandEaseCapture<'_> {
    /// Borrow an actual sealed original-notch context and calculate target/pair quantities.
    /// Build the context from `capture.source()` using the existing explicit input/source/inverse
    /// stages. Their full actual ambiguity/coincidence/quota/numeric errors remain unchanged.
    /// Require the exact captured source pointer before separate quantity admission;matching IDs
    /// or equal bodies cannot substitute a context. Retained captures sharing that same original
    /// source may use the same context. Borrow its actual projections,then query original density-
    /// side target and both A-ordered pair ranges. No cut-progress narrowing or physical predicate.
    /// ```
    /// use sc_core::command::{CommandEaseCapture,CommandNotchEasePartition,CommandNotchEaseError};
    /// use sc_core::ontology::{OriginalNotchContext,NotchEaseLimits};
    /// fn allocate<'a>(capture:&'a CommandEaseCapture<'a>,context:&'a OriginalNotchContext<'a>,
    ///     limits:NotchEaseLimits)->Result<CommandNotchEasePartition<'a>,CommandNotchEaseError<'a>>{
    ///     capture.notch_explicit_ease(context,limits)
    /// }
    /// ```
    /// # Errors
    /// Actual captured-source mismatch first,then original value/quantity-reservation refusal,
    /// then target/first/second query cause. No partial accepted partition is returned.
    pub fn notch_explicit_ease<'a>(
        &'a self,
        context: &'a OriginalNotchContext<'a>,
        limits: NotchEaseLimits,
    ) -> Result<CommandNotchEasePartition<'a>, CommandNotchEaseError<'a>> {
        let target_span = self.binding().span().id();
        if !core::ptr::eq(self.source(), context.validation().admission().source()) {
            return Err(CommandNotchEaseError::ContextSource {
                target_span,
                expected: self.source(),
                context,
            });
        }
        let admission = context
            .admit_explicit(limits)
            .map_err(|issue| CommandNotchEaseError::Workspace { target_span, issue })?;
        let side = context.validation().admission().distribution_side();
        let query = |range, stage| {
            admission
                .amount_in(range)
                .map_err(|issue| failure(target_span, stage, issue))
        };
        let parent = query(
            self.portion().side(side).range,
            CommandEaseQuantityStage::Target,
        )?;
        let [first, second] = self.binding().paired_intervals();
        let first = query(first.side(side).range, CommandEaseQuantityStage::FirstPair)?;
        let second = query(
            second.side(side).range,
            CommandEaseQuantityStage::SecondPair,
        )?;
        Ok(CommandNotchEasePartition {
            capture: self,
            admission,
            parent,
            first,
            second,
        })
    }
}
