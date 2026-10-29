//! Actual authored stop rows over the exact immutable bounded current context;no new queries.
use super::{CommandStopCurrentContext, CommandStopLandmarkEvidence};
use crate::ontology::{
    CutCopy, EntityId, RangeCoverage, RangePortion, SeamSideId, SewingError, StopLandmark,
};
use core::fmt;

/// Every closed current membership in the two A-ordered pairs;never an arbitrary boundary winner.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandStopPairMembership {
    /// Only the first A-ordered pair contains the actual current point.
    First,
    /// Only the second A-ordered pair contains the actual current point.
    Second,
    /// Both closed pairs contain the same semantic point.
    Both,
}
impl CommandStopPairMembership {
    /// Whether the first child references this actual authored stop.
    #[must_use]
    pub const fn includes_first(self) -> bool {
        matches!(self, Self::First | Self::Both)
    }
    /// Whether the second child references this actual authored stop.
    #[must_use]
    pub const fn includes_second(self) -> bool {
        matches!(self, Self::Second | Self::Both)
    }
}
/// One actual target stop reference and its complete local current membership,without cloning marks.
pub struct CommandStopMembership<'a> {
    ordinal: usize,
    stop: &'a StopLandmark,
    copy: &'a CutCopy,
    landmark: CommandStopLandmarkEvidence<'a>,
    membership: CommandStopPairMembership,
}
impl fmt::Debug for CommandStopMembership<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            ordinal,
            stop,
            copy,
            landmark,
            membership,
        } = self;
        f.debug_struct("CommandStopMembership")
            .field("ordinal", ordinal)
            .field("stop", stop)
            .field("copy", &copy.id())
            .field("landmark", landmark)
            .field("membership", membership)
            .finish()
    }
}
impl<'a> CommandStopMembership<'a> {
    /// Original zero-based target stop position,not sorted spatial order.
    #[must_use]
    pub const fn ordinal(&self) -> usize {
        self.ordinal
    }
    /// Exact original authored side/landmark reference in the actual target.
    #[must_use]
    pub const fn stop(&self) -> &'a StopLandmark {
        self.stop
    }
    /// Exact physical copy on that named side;reflection does not change source coordinates.
    #[must_use]
    pub const fn copy(&self) -> &'a CutCopy {
        self.copy
    }
    /// Actual semantic object,held reference and unique current point from the same context.
    #[must_use]
    pub const fn landmark(&self) -> CommandStopLandmarkEvidence<'a> {
        self.landmark
    }
    /// All closed pair memberships;this supplies no unique original-preimage proof.
    #[must_use]
    pub const fn membership(&self) -> CommandStopPairMembership {
        self.membership
    }
}
/// Actual target stop refusal;original semantic source errors are retained unchanged.
#[derive(Debug, PartialEq, Eq)]
pub enum CommandStopMembershipIssue {
    /// Actual named copy or semantic stop is absent,or the stop is outside its actual side/Piece.
    Structural(SewingError),
    /// Appending another row exceeds the original admitted N bound or count representation.
    Rows {
        /// Complete rows already retained internally.
        retained: usize,
        /// One actual authored reference requested.
        requested: usize,
        /// Exact original input reservation for membership rows.
        limit: usize,
    },
}
#[derive(Debug, PartialEq, Eq)]
struct Failure {
    target_span: EntityId,
    ordinal: usize,
    stop: StopLandmark,
    issue: CommandStopMembershipIssue,
}
/// Actual consuming target,authored ordinal/stop and complete refusal;no accepted prefix escapes.
#[derive(Debug, PartialEq, Eq)]
pub struct CommandStopMembershipError(Box<Failure>);
impl CommandStopMembershipError {
    fn new(
        target_span: EntityId,
        ordinal: usize,
        stop: StopLandmark,
        issue: CommandStopMembershipIssue,
    ) -> Self {
        Self(Box::new(Failure {
            target_span,
            ordinal,
            stop,
            issue,
        }))
    }
    /// Actual target,which may differ from its full retained ease root.
    #[must_use]
    pub const fn target_span(&self) -> EntityId {
        self.0.target_span
    }
    /// Actual zero-based failing authored stop index.
    #[must_use]
    pub const fn ordinal(&self) -> usize {
        self.0.ordinal
    }
    /// Actual failing authored side/landmark pair.
    #[must_use]
    pub const fn stop(&self) -> StopLandmark {
        self.0.stop
    }
    /// Complete original source or checked row-bound refusal.
    #[must_use]
    pub const fn issue(&self) -> &CommandStopMembershipIssue {
        &self.0.issue
    }
}
impl fmt::Display for CommandStopMembershipError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "command stop target {} reference {} {:?}: {:?}",
            self.target_span(),
            self.ordinal(),
            self.stop(),
            self.issue()
        )
    }
}
impl std::error::Error for CommandStopMembershipError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.issue() {
            CommandStopMembershipIssue::Structural(issue) => Some(issue),
            CommandStopMembershipIssue::Rows {
                retained: _,
                requested: _,
                limit: _,
            } => None,
        }
    }
}
/// Complete authored stop rows bound to the exact externally owned current context.
/// No semantic object,source root or point/range projection is copied or queried.
/// Private fields prevent fabricated success:
/// ```compile_fail
/// use sc_core::command::{CommandStopCurrentContext,CommandStopMemberships};
/// fn forge<'a,H>(context:&'a CommandStopCurrentContext<'a,H>)->CommandStopMemberships<'a,H>{
///     CommandStopMemberships {context,rows:Vec::new()}
/// }
/// ```
/// It cannot escape the actual current context:
/// ```compile_fail
/// use sc_core::command::{CommandStopCurrentContext,CommandStopMemberships};
/// fn escape<H>(context:&CommandStopCurrentContext<'_,H>)->CommandStopMemberships<'static,H>{
///     context.derive_stop_memberships().unwrap()
/// }
/// ```
pub struct CommandStopMemberships<'a, H> {
    context: &'a CommandStopCurrentContext<'a, H>,
    rows: Vec<CommandStopMembership<'a>>,
}
impl<H> fmt::Debug for CommandStopMemberships<'_, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { context, rows } = self;
        f.debug_struct("CommandStopMemberships")
            .field(
                "target",
                &context.ownership().inputs().binding().span().id(),
            )
            .field("rows", &rows.len())
            .finish()
    }
}
impl<'a, H> CommandStopMemberships<'a, H> {
    /// Exact current context that licensed these local memberships.
    #[must_use]
    pub const fn context(&self) -> &'a CommandStopCurrentContext<'a, H> {
        self.context
    }
    /// One row per actual target reference,in unchanged authored order.
    #[must_use]
    pub fn rows(&self) -> &[CommandStopMembership<'a>] {
        &self.rows
    }
}
fn contains(coverage: &RangeCoverage, landmark: CommandStopLandmarkEvidence<'_>) -> bool {
    let point = landmark.current();
    coverage.portions().iter().any(|part| match part {
        RangePortion::Resolved(part) => {
            part.range().edge() == point.edge()
                && part.range().from() <= point.param()
                && point.param() <= part.range().to()
        }
        RangePortion::Unresolved(_) => false,
    })
}
impl<H> CommandStopCurrentContext<'_, H> {
    /// Derive every actual target stop's closed current memberships in authored order.
    /// Uses only this exact immutable context and its original N reservation;no host trait,point/
    /// range query,inverse,coordinate narrowing,value evaluation or additional collection is needed.
    /// Shared current boundaries retain Both,even if original preimages are not unique. Full-root
    /// ease stops are never backfilled into an empty retained target. No physical/effect proof.
    /// ```
    /// use sc_core::command::{CommandStopCurrentContext,CommandStopMemberships,CommandStopMembershipError};
    /// fn rows<'a,H>(context:&'a CommandStopCurrentContext<'a,H>)
    ///     ->Result<CommandStopMemberships<'a,H>,CommandStopMembershipError>{
    ///     context.derive_stop_memberships()
    /// }
    /// ```
    /// # Errors
    /// Retains actual target,ordinal,side/landmark and original missing-copy/mark/outside-side cause,
    /// or the exact original membership-row cap. No partial accepted rows escape any refusal.
    pub fn derive_stop_memberships(
        &self,
    ) -> Result<CommandStopMemberships<'_, H>, CommandStopMembershipError> {
        let inputs = self.ownership().inputs();
        let definition = inputs.binding().span().definition();
        let mut rows = Vec::new();
        for (ordinal, stop) in definition.stops.iter().enumerate() {
            let fail =
                |issue| CommandStopMembershipError::new(definition.id, ordinal, *stop, issue);
            let structural = |issue| fail(CommandStopMembershipIssue::Structural(issue));
            let side = match stop.side {
                SeamSideId::A => definition.a,
                SeamSideId::B => definition.b,
            };
            let copy = self.copy(side.copy).ok_or_else(|| {
                structural(SewingError::MissingCopy {
                    span: definition.id,
                    side: stop.side,
                    copy: side.copy,
                })
            })?;
            let landmark = self.landmark(stop.landmark).ok_or_else(|| {
                structural(SewingError::MissingLandmark {
                    span: definition.id,
                    stop: *stop,
                })
            })?;
            if landmark.landmark().piece() != copy.piece() {
                return Err(structural(SewingError::StopOutsideSide {
                    span: definition.id,
                    stop: *stop,
                }));
            }
            let membership = match (
                contains(self.first_pair_coverage(stop.side), landmark),
                contains(self.second_pair_coverage(stop.side), landmark),
            ) {
                (true, true) => CommandStopPairMembership::Both,
                (true, false) => CommandStopPairMembership::First,
                (false, true) => CommandStopPairMembership::Second,
                (false, false) => {
                    return Err(structural(SewingError::StopOutsideSide {
                        span: definition.id,
                        stop: *stop,
                    }))
                }
            };
            if rows
                .len()
                .checked_add(1)
                .filter(|next| *next <= inputs.membership_item_bound())
                .is_none()
            {
                return Err(fail(CommandStopMembershipIssue::Rows {
                    retained: rows.len(),
                    requested: 1,
                    limit: inputs.membership_item_bound(),
                }));
            }
            rows.push(CommandStopMembership {
                ordinal,
                stop,
                copy,
                landmark,
                membership,
            });
        }
        Ok(CommandStopMemberships {
            context: self,
            rows,
        })
    }
}
