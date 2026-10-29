//! Separate actual ownership index/cache allowance before any current Piece/mark/range queries.
use super::CommandStopPartitionAdmission;
use crate::{design::DesignReferenceHost, ontology::EntityId};
use core::{fmt, num::NonZeroUsize};

/// Explicit positive ownership workspace allowance,separate from stop rows/pair caches.
/// Counts actual copy/Piece indexes,owned Piece interval cache and one transient range workspace.
/// No default,allocator/RSS/CPU policy or currentness proof is supplied.
/// ```compile_fail
/// let limits:sc_core::command::CommandStopOwnershipLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandStopOwnershipLimits {
    workspace_items: NonZeroUsize,
}
impl CommandStopOwnershipLimits {
    /// Select the logical ownership index/cache allowance explicitly.
    #[must_use]
    pub const fn new(workspace_items: NonZeroUsize) -> Self {
        Self { workspace_items }
    }
    /// Actual caller-selected conservative population cap.
    #[must_use]
    pub const fn workspace_items(self) -> NonZeroUsize {
        self.workspace_items
    }
}
/// Actual complete schema census or conservative ownership reservation refusal;no query ran.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandStopOwnershipAdmissionError {
    /// Counting another actual canonical Piece edge cannot be represented.
    EdgeCountOverflow {
        /// Actual target span.
        target_span: EntityId,
        /// Actual Piece whose next authored edge would be counted.
        piece: EntityId,
        /// Complete preceding edge population.
        retained: usize,
    },
    /// Actual counts and selected range peak cannot form the conservative reservation.
    BoundOverflow {
        /// Actual target span.
        target_span: EntityId,
        /// Complete actual copy population C.
        copies: usize,
        /// Complete actual Piece population P.
        pieces: usize,
        /// Complete canonical Piece edge population E.
        edges: usize,
        /// Actual previously selected per-range peak B.
        range_portions: usize,
    },
    /// Checked C+P+(E+1)B exceeds the separately selected allowance.
    Workspace {
        /// Actual target span.
        target_span: EntityId,
        /// Complete actual copy population C.
        copies: usize,
        /// Complete actual Piece population P.
        pieces: usize,
        /// Complete canonical Piece edge population E.
        edges: usize,
        /// Actual per-range peak B,without widening it.
        range_portions: usize,
        /// Conservative ownership index/cache/transient reservation.
        required: usize,
        /// Actual independently selected allowance.
        limit: usize,
    },
}
impl fmt::Display for CommandStopOwnershipAdmissionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "command stop ownership admission: {self:?}")
    }
}
impl std::error::Error for CommandStopOwnershipAdmissionError {}

/// Actual stop input borrow and separately checked complete ownership workspace bounds.
/// The original input token retains the model,journal and stop/pair reservation. This stage only
/// counts complete immutable copy/Piece/edge collections;no cache,index,current query or clone.
/// It grants no accepted copy/mark/Piece membership or whole Design/physical/effect readiness.
///
/// Private fields prevent fabricated successful ownership admission:
/// ```compile_fail
/// use sc_core::command::{CommandStopPartitionAdmission,CommandStopOwnershipAdmission,CommandStopOwnershipLimits};
/// fn forge<'a,H>(inputs:&'a CommandStopPartitionAdmission<'a,H>,limits:CommandStopOwnershipLimits)
///     ->CommandStopOwnershipAdmission<'a,H>{
///     CommandStopOwnershipAdmission {inputs,limits,copies:0,pieces:0,edges:0,
///         ownership_items:0,workspace_items:0}
/// }
/// ```
/// The actual admitted input borrow cannot escape:
/// ```compile_fail
/// use sc_core::command::{CommandStopPartitionAdmission,CommandStopOwnershipAdmission,CommandStopOwnershipLimits};
/// use sc_core::design::DesignReferenceHost;
/// fn escape<H:DesignReferenceHost>(inputs:&CommandStopPartitionAdmission<'_,H>,limits:CommandStopOwnershipLimits)
///     ->CommandStopOwnershipAdmission<'static,H>{inputs.admit_ownership_workspace(limits).unwrap()}
/// ```
pub struct CommandStopOwnershipAdmission<'a, H> {
    inputs: &'a CommandStopPartitionAdmission<'a, H>,
    limits: CommandStopOwnershipLimits,
    copies: usize,
    pieces: usize,
    edges: usize,
    ownership_items: usize,
    workspace_items: usize,
}
impl<H> fmt::Debug for CommandStopOwnershipAdmission<'_, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            inputs,
            limits,
            copies,
            pieces,
            edges,
            ownership_items,
            workspace_items,
        } = self;
        f.debug_struct("CommandStopOwnershipAdmission")
            .field("target", &inputs.binding().span().id())
            .field("limits", limits)
            .field("copies", copies)
            .field("pieces", pieces)
            .field("edges", edges)
            .field("ownership_items", ownership_items)
            .field("workspace_items", workspace_items)
            .finish()
    }
}
impl<'a, H> CommandStopOwnershipAdmission<'a, H> {
    /// Exact admitted actual model/request/target/stop/registry/journal input token.
    #[must_use]
    pub const fn inputs(&self) -> &'a CommandStopPartitionAdmission<'a, H> {
        self.inputs
    }
    /// Independently selected ownership allowance.
    #[must_use]
    pub const fn limits(&self) -> CommandStopOwnershipLimits {
        self.limits
    }
    /// Complete actual copy population C,including unused copies.
    #[must_use]
    pub const fn copy_count(&self) -> usize {
        self.copies
    }
    /// Complete actual Piece population P,including unused Pieces.
    #[must_use]
    pub const fn piece_count(&self) -> usize {
        self.pieces
    }
    /// Complete canonical held Piece edge population E in boundary/hole/construction order.
    #[must_use]
    pub const fn edge_count(&self) -> usize {
        self.edges
    }
    /// Conservative retained ownership interval cache E*B;no allocator capacity claim.
    #[must_use]
    pub const fn ownership_item_bound(&self) -> usize {
        self.ownership_items
    }
    /// Complete index/cache/one transient query reservation C+P+(E+1)B.
    #[must_use]
    pub const fn workspace_item_bound(&self) -> usize {
        self.workspace_items
    }
}
impl<H: DesignReferenceHost> CommandStopPartitionAdmission<'_, H> {
    /// Admit complete actual ownership indexes/cache before any current semantic queries.
    /// Count copies C,Pieces P and every held Piece edge E from the actual immutable input owner.
    /// Reserve E*B retained intervals plus one B-sized transient range query and C+P index rows.
    /// The original stop input's M+N+2N+4B and per-point/range allowances remain unchanged.
    /// All count arithmetic is checked before allocation;no semantic/current callback is invoked.
    /// ```
    /// use sc_core::command::{CommandStopPartitionAdmission,CommandStopOwnershipAdmission,
    ///     CommandStopOwnershipAdmissionError,CommandStopOwnershipLimits};
    /// use sc_core::design::DesignReferenceHost;
    /// fn admit<'a,H:DesignReferenceHost>(inputs:&'a CommandStopPartitionAdmission<'a,H>,
    ///     limits:CommandStopOwnershipLimits)
    ///     ->Result<CommandStopOwnershipAdmission<'a,H>,CommandStopOwnershipAdmissionError>{
    ///     inputs.admit_ownership_workspace(limits)
    /// }
    /// ```
    /// # Errors
    /// Preserves actual target/Piece and complete counts on census overflow,then reservation
    /// overflow or selected-limit refusal. No partial admitted token or allocated cache escapes.
    pub fn admit_ownership_workspace(
        &self,
        limits: CommandStopOwnershipLimits,
    ) -> Result<CommandStopOwnershipAdmission<'_, H>, CommandStopOwnershipAdmissionError> {
        let target_span = self.binding().span().id();
        let structure = self.state().inputs().structure();
        let copies = structure.cut_plan().copies().len();
        let pieces = structure.pieces().pieces().len();
        let mut edges = 0usize;
        for piece in structure.pieces().pieces() {
            for _ in piece.edges() {
                edges = edges.checked_add(1).ok_or(
                    CommandStopOwnershipAdmissionError::EdgeCountOverflow {
                        target_span,
                        piece: piece.id(),
                        retained: edges,
                    },
                )?;
            }
        }
        let range_portions = self.limits().range_portions().get();
        let overflow = || CommandStopOwnershipAdmissionError::BoundOverflow {
            target_span,
            copies,
            pieces,
            edges,
            range_portions,
        };
        let ownership_items = edges.checked_mul(range_portions).ok_or_else(overflow)?;
        let workspace_items = ownership_items
            .checked_add(range_portions)
            .and_then(|count| count.checked_add(copies))
            .and_then(|count| count.checked_add(pieces))
            .ok_or_else(overflow)?;
        if workspace_items > limits.workspace_items.get() {
            return Err(CommandStopOwnershipAdmissionError::Workspace {
                target_span,
                copies,
                pieces,
                edges,
                range_portions,
                required: workspace_items,
                limit: limits.workspace_items.get(),
            });
        }
        Ok(CommandStopOwnershipAdmission {
            inputs: self,
            limits,
            copies,
            pieces,
            edges,
            ownership_items,
            workspace_items,
        })
    }
}
