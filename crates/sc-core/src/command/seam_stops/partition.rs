//! Two ordered borrowed child reference lists under the original admitted aggregate2N cap.
use super::{CommandStopMembership, CommandStopMemberships};
use crate::{
    command::CommandSeamSplitPair,
    ontology::{EntityId, StopLandmark},
};
use core::fmt;

/// Child order is always lower then upper A,including Opposite B correspondence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandStopChild {
    /// First A-ordered pair.
    First,
    /// Second A-ordered pair.
    Second,
}
/// Exact derived reference-bound refusal;no accepted child prefix escapes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandStopPartitionError {
    /// Actual consuming target,not necessarily its original ease root.
    pub target_span: EntityId,
    /// Actual authored row ordinal.
    pub ordinal: usize,
    /// Actual authored stop reference.
    pub stop: StopLandmark,
    /// Child whose next reference was refused.
    pub child: CommandStopChild,
    /// Total already retained across both children.
    pub retained: usize,
    /// One additional borrowed row reference.
    pub requested: usize,
    /// Original input admission's conservative aggregate2N cap.
    pub limit: usize,
}
impl fmt::Display for CommandStopPartitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "command stop target {} row {} {:?} child {:?}: references {} + {} exceed {}",
            self.target_span,
            self.ordinal,
            self.stop,
            self.child,
            self.retained,
            self.requested,
            self.limit
        )
    }
}
impl std::error::Error for CommandStopPartitionError {}
/// Borrowed ordered child rows and exact original pair metadata;no new child span identity.
#[derive(Clone, Copy)]
pub struct CommandStopChildView<'a> {
    child: CommandStopChild,
    pair: CommandSeamSplitPair,
    rows: &'a [&'a CommandStopMembership<'a>],
}
impl fmt::Debug for CommandStopChildView<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { child, pair, rows } = self;
        f.debug_struct("CommandStopChildView")
            .field("child", child)
            .field("pair", pair)
            .field("rows", &rows.len())
            .finish()
    }
}
impl<'a> CommandStopChildView<'a> {
    /// Which A-ordered child this view represents.
    #[must_use]
    pub const fn child(self) -> CommandStopChild {
        self.child
    }
    /// Exact actual binding's original pair,without a new span/lineage identity.
    #[must_use]
    pub const fn pair(self) -> CommandSeamSplitPair {
        self.pair
    }
    /// Original complete membership rows,in unchanged authored order,borrowed without cloning.
    #[must_use]
    pub const fn rows(self) -> &'a [&'a CommandStopMembership<'a>] {
        self.rows
    }
}
/// Complete two-child reference populations bound to externally owned actual membership evidence.
/// Shared boundaries reference the same row and semantic object in both children. These are local
/// stop views,not allocated child spans,source quantity contexts,current authority or effects.
/// Private fields prevent fabricated success:
/// ```compile_fail
/// use sc_core::command::{CommandStopMemberships,CommandStopPartitions};
/// fn forge<'a,H>(memberships:&'a CommandStopMemberships<'a,H>)->CommandStopPartitions<'a,H>{
///     CommandStopPartitions {memberships,children:[Vec::new(),Vec::new()],reference_count:0}
/// }
/// ```
/// The actual source owner cannot escape:
/// ```compile_fail
/// use sc_core::command::{CommandStopMemberships,CommandStopPartitions};
/// fn escape<H>(memberships:&CommandStopMemberships<'_,H>)->CommandStopPartitions<'static,H>{
///     memberships.partition_stops().unwrap()
/// }
/// ```
pub struct CommandStopPartitions<'a, H> {
    memberships: &'a CommandStopMemberships<'a, H>,
    children: [Vec<&'a CommandStopMembership<'a>>; 2],
    reference_count: usize,
}
impl<H> fmt::Debug for CommandStopPartitions<'_, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            memberships,
            children,
            reference_count,
        } = self;
        let [first, second] = children;
        f.debug_struct("CommandStopPartitions")
            .field(
                "target",
                &memberships
                    .context()
                    .ownership()
                    .inputs()
                    .binding()
                    .span()
                    .id(),
            )
            .field("first", &first.len())
            .field("second", &second.len())
            .field("reference_count", reference_count)
            .finish()
    }
}
impl<'a, H> CommandStopPartitions<'a, H> {
    /// Exact externally owned actual membership token,including context/input/limit provenance.
    #[must_use]
    pub const fn memberships(&self) -> &'a CommandStopMemberships<'a, H> {
        self.memberships
    }
    /// Exact child pair and ordered row references;no lookup/query/source capture occurs.
    #[must_use]
    pub fn child(&self, child: CommandStopChild) -> CommandStopChildView<'_> {
        let [first_rows, second_rows] = &self.children;
        let [first_pair, second_pair] = self
            .memberships
            .context()
            .ownership()
            .inputs()
            .binding()
            .paired_intervals();
        let (pair, rows) = match child {
            CommandStopChild::First => (first_pair, first_rows.as_slice()),
            CommandStopChild::Second => (second_pair, second_rows.as_slice()),
        };
        CommandStopChildView { child, pair, rows }
    }
    /// Actual aggregate borrowed row references across both children,including shared rows twice.
    #[must_use]
    pub const fn reference_count(&self) -> usize {
        self.reference_count
    }
}
impl<H> CommandStopMemberships<'_, H> {
    /// Build both ordered borrowed child row lists under the original aggregate2N reservation.
    /// Preserve actual authored order and every closed membership. No mark/root clone,new span,
    /// point/range query,inverse,narrowing,value read or effect occurs. Actual ease-capture source
    /// binding and whole paired/physical/current authority remain separate requirements.
    /// ```
    /// use sc_core::command::{CommandStopMemberships,CommandStopPartitions,CommandStopPartitionError};
    /// fn children<'a,H>(memberships:&'a CommandStopMemberships<'a,H>)
    ///     ->Result<CommandStopPartitions<'a,H>,CommandStopPartitionError>{
    ///     memberships.partition_stops()
    /// }
    /// ```
    /// # Errors
    /// Retains actual target,ordinal,stop,child and original aggregate reference allowance if
    /// checked growth refuses. Neither partial child list escapes any refusal.
    pub fn partition_stops(
        &self,
    ) -> Result<CommandStopPartitions<'_, H>, CommandStopPartitionError> {
        let inputs = self.context().ownership().inputs();
        let limit = inputs.child_reference_item_bound();
        let mut children = [Vec::new(), Vec::new()];
        let mut reference_count = 0usize;
        for row in self.rows() {
            for (child, belongs) in [
                (CommandStopChild::First, row.membership().includes_first()),
                (CommandStopChild::Second, row.membership().includes_second()),
            ] {
                if belongs {
                    let next = reference_count
                        .checked_add(1)
                        .filter(|next| *next <= limit)
                        .ok_or(CommandStopPartitionError {
                            target_span: inputs.binding().span().id(),
                            ordinal: row.ordinal(),
                            stop: *row.stop(),
                            child,
                            retained: reference_count,
                            requested: 1,
                            limit,
                        })?;
                    let [first, second] = &mut children;
                    match child {
                        CommandStopChild::First => first.push(row),
                        CommandStopChild::Second => second.push(row),
                    };
                    reference_count = next;
                }
            }
        }
        Ok(CommandStopPartitions {
            memberships: self,
            children,
            reference_count,
        })
    }
}
