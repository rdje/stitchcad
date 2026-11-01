//! Bounded actual endpoint/stop/copy preservation requests,not geometric correspondence.
use super::{BoundConstrainedTruing, BoundTruingCopyInventory, RequiredTruingOutcome};
use crate::ontology::{CutCopy, EdgeAnchor, EdgeRange, Piece, SeamSide, SeamSideId, StopLandmark};
use core::fmt;

/// Explicit whole retained-plan copy and original span-stop allowances;no defaults.
/// ```compile_fail
/// let limits:sc_core::construction::TruingLockLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TruingLockLimits {
    copies: usize,
    stops: usize,
}
impl TruingLockLimits {
    /// Choose whole C and S before copy filtering,endpoint or stop inspection.
    #[must_use]
    pub const fn new(copies: usize, stops: usize) -> Self {
        Self { copies, stops }
    }
    /// Every retained-plan copy,including unrelated Pieces.
    #[must_use]
    pub const fn copies(self) -> usize {
        self.copies
    }
    /// Every original span stop,on either side.
    #[must_use]
    pub const fn stops(self) -> usize {
        self.stops
    }
}
/// First complete population refusal;no prefix lock context escapes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TruingLockRefusal {
    /// Complete retained-plan population exceeds policy.
    Copies {
        /// Actual whole population.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete original span-stop population exceeds policy.
    Stops {
        /// Actual whole population.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
}
/// Actual complete outcome/source context,explicit limits and first whole population cause.
pub struct TruingLockError<'a> {
    outcome: &'a RequiredTruingOutcome<'a>,
    limits: TruingLockLimits,
    refusal: TruingLockRefusal,
}
impl<'a> TruingLockError<'a> {
    /// Actual complete original outcome and all source parents.
    #[must_use]
    pub const fn outcome(&self) -> &'a RequiredTruingOutcome<'a> {
        self.outcome
    }
    /// Original whole C/S policy.
    #[must_use]
    pub const fn limits(&self) -> TruingLockLimits {
        self.limits
    }
    /// First complete copy or stop population refusal.
    #[must_use]
    pub const fn refusal(&self) -> TruingLockRefusal {
        self.refusal
    }
}
impl fmt::Debug for TruingLockError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TruingLockError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for TruingLockError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "truing lock requirements: {:?}", self.refusal)
    }
}
impl std::error::Error for TruingLockError<'_> {}
impl<'s> RequiredTruingOutcome<'s> {
    /// Require selected held endpoint,all original span-stop and affected-copy preservation.
    /// Whole C quota precedes whole S quota,before endpoint/stop inspection or copy filtering.
    /// Constant work/space,no allocation;earlier source/copy/role/candidate bounds remain separate.
    /// Both explicit outcomes retain these obligations,including empty source preservation.
    /// # Errors
    /// Complete retained-plan copy count,then complete original span-stop count exceeds policy.
    pub fn require_locks<'a>(
        &'a self,
        limits: TruingLockLimits,
    ) -> Result<RequiredTruingLocks<'a>, TruingLockError<'a>>
    where
        's: 'a,
    {
        let refuse = |refusal| TruingLockError {
            outcome: self,
            limits,
            refusal,
        };
        let source = self.source_domain().candidates().roles().copies().source();
        let copies = source.plan().copies().len();
        if copies > limits.copies {
            return Err(refuse(TruingLockRefusal::Copies {
                required: copies,
                allowed: limits.copies,
            }));
        }
        let stops = source.span().definition().stops.len();
        if stops > limits.stops {
            return Err(refuse(TruingLockRefusal::Stops {
                required: stops,
                allowed: limits.stops,
            }));
        }
        Ok(RequiredTruingLocks {
            outcome: self,
            limits,
        })
    }
}
/// Authentic bounded lock requests,not fitted positions,current repairs or editing authority.
/// ```compile_fail
/// fn forge<'a>(mut locks:sc_core::construction::RequiredTruingLocks<'a>,
///     other:&'a sc_core::construction::RequiredTruingOutcome<'a>) {locks.outcome=other;}
/// ```
/// ```compile_fail
/// fn escape(outcome:sc_core::construction::RequiredTruingOutcome<'_>)
///     ->sc_core::construction::RequiredTruingLocks<'static> {
///     outcome.require_locks(sc_core::construction::TruingLockLimits::new(0,0)).unwrap()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct RequiredTruingLocks<'a> {
    outcome: &'a RequiredTruingOutcome<'a>,
    limits: TruingLockLimits,
}
impl<'a> RequiredTruingLocks<'a> {
    /// Actual complete original outcome and source parents.
    #[must_use]
    pub const fn outcome(&self) -> &'a RequiredTruingOutcome<'a> {
        self.outcome
    }
    /// Original explicit whole copy/stop policy.
    #[must_use]
    pub const fn limits(&self) -> TruingLockLimits {
        self.limits
    }
    /// Actual complete copy/Piece context,not a fixed-counterpart certificate.
    #[must_use]
    pub const fn copies(&self) -> &'a BoundTruingCopyInventory<'a> {
        self.outcome.source_domain().candidates().roles().copies()
    }
    /// Actual original span,graph,plan,selected side and quantitative fit provider.
    #[must_use]
    pub const fn source(&self) -> &'a BoundConstrainedTruing<'a> {
        self.copies().source()
    }
    /// Both selected range endpoints in ascending held parameter order,not sewing traversal order.
    /// These require original positions;anchors supply no actual geometric position or map.
    pub fn endpoints(
        &self,
    ) -> impl ExactSizeIterator<Item = TruingEndpointRequirement<'_>> + DoubleEndedIterator {
        [TruingEndpoint::From, TruingEndpoint::To]
            .into_iter()
            .map(|endpoint| TruingEndpointRequirement {
                context: self,
                endpoint,
            })
    }
    /// Every original stop on either side,in original order;no filtering or deduplication.
    /// O(S) iteration,no allocation;actual landmark/currentness/position lookup is separate.
    pub fn stops(
        &self,
    ) -> impl ExactSizeIterator<Item = TruingStopRequirement<'_>> + DoubleEndedIterator {
        self.source()
            .span()
            .definition()
            .stops
            .iter()
            .enumerate()
            .map(|(index, stop)| TruingStopRequirement {
                context: self,
                index,
                stop,
            })
    }
    /// Every actual selected-Piece copy in original plan order,including shared-Piece counterparts.
    /// Scans at most C entries without allocation;not a complete current semantic/observer census.
    pub fn affected_copies(&self) -> impl Iterator<Item = &'a CutCopy> + 'a {
        self.source().affected_copies()
    }
}
/// Selected domain lower/upper endpoint,independent of physical sewing traversal direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TruingEndpoint {
    /// Original selected held lower parameter.
    From,
    /// Original selected held upper parameter.
    To,
}
/// Authentic endpoint and complete lock context,not an evaluated position.
/// ```compile_fail
/// fn forge<'a>(mut endpoint:sc_core::construction::TruingEndpointRequirement<'a>,
///     other:&'a sc_core::construction::RequiredTruingLocks<'a>) {endpoint.context=other;}
/// ```
#[derive(Clone, Copy, Debug)]
pub struct TruingEndpointRequirement<'a> {
    context: &'a RequiredTruingLocks<'a>,
    endpoint: TruingEndpoint,
}
impl<'a> TruingEndpointRequirement<'a> {
    /// Actual complete lock/outcome/source owner.
    #[must_use]
    pub const fn context(self) -> &'a RequiredTruingLocks<'a> {
        self.context
    }
    /// Exact lower/upper role,not traversal direction or a new point identity.
    #[must_use]
    pub const fn endpoint(self) -> TruingEndpoint {
        self.endpoint
    }
    /// Actual original selected span-side range field.
    #[must_use]
    pub fn source_range(self) -> &'a EdgeRange {
        &self.context.source().source_side().range
    }
    /// Original held edge and exact parameter;no current resolution or geometric map.
    #[must_use]
    pub fn anchor(self) -> EdgeAnchor {
        let range = self.source_range();
        EdgeAnchor {
            edge: range.edge(),
            param: match self.endpoint {
                TruingEndpoint::From => range.from(),
                TruingEndpoint::To => range.to(),
            },
        }
    }
}
/// Authentic original stop requirement,including counterpart-side stops.
/// ```compile_fail
/// fn forge<'a>(mut stop:sc_core::construction::TruingStopRequirement<'a>,
///     other:&'a sc_core::construction::RequiredTruingLocks<'a>) {stop.context=other;}
/// ```
#[derive(Clone, Copy, Debug)]
pub struct TruingStopRequirement<'a> {
    context: &'a RequiredTruingLocks<'a>,
    index: usize,
    stop: &'a StopLandmark,
}
impl<'a> TruingStopRequirement<'a> {
    /// Actual complete lock/outcome/source owner.
    #[must_use]
    pub const fn context(self) -> &'a RequiredTruingLocks<'a> {
        self.context
    }
    /// Original stop position,not persistent identity.
    #[must_use]
    pub const fn index(self) -> usize {
        self.index
    }
    /// Actual original side/landmark field;current registry resolution remains required.
    #[must_use]
    pub const fn stop(self) -> &'a StopLandmark {
        self.stop
    }
    /// Actual span-side field selected by this stop's own side,not the adjusted side.
    #[must_use]
    pub fn side(self) -> &'a SeamSide {
        let span = self.context.source().span().definition();
        match self.stop.side {
            SeamSideId::A => &span.a,
            SeamSideId::B => &span.b,
        }
    }
    /// Actual cached physical copy for the stop's original side.
    #[must_use]
    pub const fn copy(self) -> &'a CutCopy {
        self.context.copies().copy(self.stop.side)
    }
    /// Actual supplied pattern Piece for the stop's original side.
    #[must_use]
    pub const fn piece(self) -> &'a Piece {
        self.context.copies().piece(self.stop.side)
    }
}
