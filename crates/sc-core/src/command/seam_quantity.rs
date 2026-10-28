//! Original-source quantities over an actual admitted command capture;not physical/effect proof.
use super::{CommandEaseCapture, CommandSeamSplitPair, SplitSeamSpan};
use crate::ontology::{
    EdgeAnchor, EdgeRange, EntityId, SeamEaseSource, SeamSideId, SeamSpan, WeightedEaseAdmission,
    WeightedEaseAmountError, WeightedEaseLimits, WeightedEaseWorkspaceError,
};
use core::fmt;
use sc_units::Length;
mod notch;
pub use notch::{CommandNotchEaseError, CommandNotchEasePartition};

/// Actual mathematical query stage;does not imply current-world or physical validation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandEaseQuantityStage {
    /// Full actual target allocation in the original source.
    Target,
    /// First projected interval pair in canonical A order.
    FirstPair,
    /// Second projected interval pair in canonical A order.
    SecondPair,
    /// Explicit original-source cumulative boundary query.
    Boundary,
    /// Explicit original-source interval query.
    Interval,
}
/// Original weighted source admission/query cause plus the actual consuming target and stage.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandWeightedEaseError {
    /// Actual original distribution/amount/population/integer admission refusal.
    Workspace {
        /// Actual requested graph member,which may differ from the original source.
        target_span: EntityId,
        /// Original complete source/limit refusal,also the standard error source.
        issue: WeightedEaseWorkspaceError,
    },
    /// Actual original-domain query refusal;no accepted partial partition.
    Quantity {
        /// Actual requested graph member.
        target_span: EntityId,
        /// Concrete failed target/pair/boundary/interval stage.
        stage: CommandEaseQuantityStage,
        /// Actual full source/frame/bounds/numeric cause,also the standard error source.
        issue: Box<WeightedEaseAmountError>,
    },
}
impl fmt::Display for CommandWeightedEaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Workspace { target_span, issue } => write!(
                f,
                "weighted command target {target_span} admission: {issue}"
            ),
            Self::Quantity {
                target_span,
                stage,
                issue,
            } => write!(
                f,
                "weighted command target {target_span} {stage:?}: {issue}"
            ),
        }
    }
}
impl std::error::Error for CommandWeightedEaseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
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
fn failure(
    target_span: EntityId,
    stage: CommandEaseQuantityStage,
    issue: WeightedEaseAmountError,
) -> CommandWeightedEaseError {
    CommandWeightedEaseError::Quantity {
        target_span,
        stage,
        issue: Box::new(issue),
    }
}

/// Borrowed actual capture and separately admitted full original weighted source,with exact amounts.
/// Target and both A-ordered pairs difference original rounded boundaries;neither a rounded child
/// nor its local domain becomes a new source. The other side's supplied cut stays descriptive;
/// these quantities do not establish matching physical cuts,currentness,authority or effects.
///
/// Private fields prevent fabricated successful quantities:
/// ```compile_fail
/// use sc_core::command::{CommandEaseCapture,CommandWeightedEasePartition};
/// use sc_core::ontology::WeightedEaseAdmission;
/// fn forge<'a>(capture:&'a CommandEaseCapture<'a>,admission:WeightedEaseAdmission<'a>)
///     ->CommandWeightedEasePartition<'a>{
///     CommandWeightedEasePartition {capture,admission,parent:sc_units::Length::ZERO,
///         first:sc_units::Length::ZERO,second:sc_units::Length::ZERO}
/// }
/// ```
/// Its actual capture borrow cannot escape:
/// ```compile_fail
/// use sc_core::command::{CommandEaseCapture,CommandWeightedEasePartition};
/// use sc_core::ontology::WeightedEaseLimits;
/// fn escape(capture:&CommandEaseCapture<'_>,limits:WeightedEaseLimits)
///     ->CommandWeightedEasePartition<'static>{capture.weighted_explicit_ease(limits).unwrap()}
/// ```
#[derive(Clone, Copy)]
pub struct CommandWeightedEasePartition<'a> {
    capture: &'a CommandEaseCapture<'a>,
    admission: WeightedEaseAdmission<'a>,
    parent: Length,
    first: Length,
    second: Length,
}
impl fmt::Debug for CommandWeightedEasePartition<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            capture,
            admission,
            parent,
            first,
            second,
        } = self;
        f.debug_struct("CommandWeightedEasePartition")
            .field("target", &capture.binding().span().id())
            .field("source", &admission.source().span())
            .field("side", &admission.distribution_side())
            .field("parent", parent)
            .field("first", first)
            .field("second", second)
            .finish()
    }
}
impl<'a> CommandWeightedEasePartition<'a> {
    /// Actual materialized capture;no same-ID source reconstruction or cloning.
    #[must_use]
    pub const fn capture(&self) -> &'a CommandEaseCapture<'a> {
        self.capture
    }
    /// Actual original request.
    #[must_use]
    pub fn request(&self) -> &'a SplitSeamSpan {
        self.capture.binding().request()
    }
    /// Actual requested graph member,retaining its copy/domain/direction metadata.
    #[must_use]
    pub fn span(&self) -> &'a SeamSpan {
        self.capture.binding().span()
    }
    /// Complete original source,including its full density and original signed literal.
    #[must_use]
    pub const fn original_source(&self) -> &'a SeamEaseSource {
        self.admission.source()
    }
    /// Exact borrowed full-source workspace/limits evidence.
    #[must_use]
    pub const fn weighted_admission(&self) -> WeightedEaseAdmission<'a> {
        self.admission
    }
    /// Actual original density side;not inferred from physical-copy reflection.
    #[must_use]
    pub const fn distribution_side(&self) -> SeamSideId {
        self.admission.distribution_side()
    }
    /// Full original signed literal,distinct from the requested target allocation.
    #[must_use]
    pub const fn source_parent_amount(&self) -> Length {
        self.admission.parent_amount()
    }
    /// Requested target allocation from original rounded cumulative endpoints.
    #[must_use]
    pub const fn parent_amount(&self) -> Length {
        self.parent
    }
    /// First projected pair's original-source allocation,in canonical A order.
    #[must_use]
    pub const fn first_amount(&self) -> Length {
        self.first
    }
    /// Second projected pair's original-source allocation;not a new rounded child source.
    #[must_use]
    pub const fn second_amount(&self) -> Length {
        self.second
    }
    /// Both actual descriptive interval pairs,unchanged by quantity calculation.
    #[must_use]
    pub fn paired_intervals(&self) -> [CommandSeamSplitPair; 2] {
        self.capture.binding().paired_intervals()
    }
    /// Full original density-side cumulative query,including outside the requested target.
    /// # Errors
    /// Preserves actual original query cause and boundary stage with consuming target identity.
    pub fn cumulative_at(&self, at: EdgeAnchor) -> Result<Length, CommandWeightedEaseError> {
        self.admission
            .cumulative_at(at)
            .map_err(|issue| failure(self.span().id(), CommandEaseQuantityStage::Boundary, issue))
    }
    /// Full original density-side interval query;never renormalizes to the consuming child.
    /// # Errors
    /// Preserves actual original query cause and interval stage with consuming target identity.
    pub fn amount_in(&self, interval: EdgeRange) -> Result<Length, CommandWeightedEaseError> {
        self.admission
            .amount_in(interval)
            .map_err(|issue| failure(self.span().id(), CommandEaseQuantityStage::Interval, issue))
    }
}
impl CommandEaseCapture<'_> {
    /// Compute exact weighted target/pair quantities from this actual captured original source.
    /// Admit the complete original weighted workspace before integer allocation. Use its actual
    /// density side and original rounded boundary differences for target and both A-ordered pairs.
    /// B/Opposite orientation is applied by the source query before shared signed rounding.
    /// No source clone,child normalization or Uniform equal-progress predicate is introduced.
    /// These are mathematical intent quantities;matching physical cuts/current/authority/effects
    /// require their separate real contexts.
    /// ```
    /// use sc_core::command::{CommandEaseCapture,CommandWeightedEasePartition,CommandWeightedEaseError};
    /// use sc_core::ontology::WeightedEaseLimits;
    /// fn allocate<'a>(capture:&'a CommandEaseCapture<'a>,limits:WeightedEaseLimits)
    ///     ->Result<CommandWeightedEasePartition<'a>,CommandWeightedEaseError>{
    ///     capture.weighted_explicit_ease(limits)
    /// }
    /// ```
    /// # Errors
    /// Original workspace refusal first,then target/first/second actual query cause. Source/target
    /// identities and concrete numeric/limit/frame errors survive;no partial partition escapes.
    pub fn weighted_explicit_ease(
        &self,
        limits: WeightedEaseLimits,
    ) -> Result<CommandWeightedEasePartition<'_>, CommandWeightedEaseError> {
        let target_span = self.binding().span().id();
        let admission = self
            .source()
            .admit_weighted_explicit(limits)
            .map_err(|issue| CommandWeightedEaseError::Workspace { target_span, issue })?;
        let side = admission.distribution_side();
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
        Ok(CommandWeightedEasePartition {
            capture: self,
            admission,
            parent,
            first,
            second,
        })
    }
}
