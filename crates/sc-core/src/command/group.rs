//! Exact immutable mutation-group inputs; consistency is distinct from authority and execution.
use super::{
    CommandActorClaim, CommandGroupId, CommandId, CommandMutation, CommandSnapshotContext,
    DesignRevision,
};
use crate::ontology::EntityId;
use core::fmt;

/// Original inconsistent group metadata; indices are zero-based in the submitted command order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandMutationGroupError {
    /// An atomic group contains no mutation.
    NoMutations {
        /// Original explicit group identity.
        group: CommandGroupId,
    },
    /// A member omits or names another group; standalone metadata is never rewritten.
    GroupMismatch {
        /// Original member position.
        index: usize,
        /// Actual payload-derived command identity.
        command: CommandId,
        /// Original explicit containing group.
        expected: CommandGroupId,
        /// Original member claim, including absent membership.
        requested: Option<CommandGroupId>,
    },
    /// A member targets another Design than the first command.
    DesignMismatch {
        /// Original member position.
        index: usize,
        /// Actual payload-derived identity.
        command: CommandId,
        /// First member's Design claim.
        expected: EntityId,
        /// Inconsistent member's Design claim.
        requested: EntityId,
    },
    /// A member changes actor role, initiator or principal.
    ActorMismatch {
        /// Original member position.
        index: usize,
        /// Actual payload-derived identity.
        command: CommandId,
        /// First member's complete claim.
        expected: CommandActorClaim,
        /// Inconsistent member's complete claim.
        requested: CommandActorClaim,
    },
    /// A member was authored against another exact revision.
    RevisionMismatch {
        /// Original member position.
        index: usize,
        /// Actual payload-derived identity.
        command: CommandId,
        /// First member's exact revision.
        expected: DesignRevision,
        /// Inconsistent member's exact revision.
        requested: DesignRevision,
    },
}
impl fmt::Display for CommandMutationGroupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mutation group refused: {self:?}")
    }
}
impl std::error::Error for CommandMutationGroupError {}

/// Nonempty ordered mutations with one original Design/actor/revision and explicit group identity.
/// Contexts, payloads and retry keys are retained exactly; no grant, replay result or execution follows.
/// Handler availability is separate, so any typed mutation payload may be represented.
///
/// ```compile_fail
/// fn reorder(group: &mut sc_core::command::CommandMutationGroup) {
///     group.mutations.reverse();
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandMutationGroup {
    id: CommandGroupId,
    context: CommandSnapshotContext,
    mutations: Vec<CommandMutation>,
}
impl CommandMutationGroup {
    pub(crate) fn resource_parts(
        &self,
    ) -> (CommandGroupId, &CommandSnapshotContext, &[CommandMutation]) {
        let Self {
            id,
            context,
            mutations,
        } = self;
        (*id, context, mutations)
    }

    /// Validate original request consistency without sorting, deduplication or context rewriting.
    /// Membership, Design, actor then revision are checked per command in submitted order. Retry
    /// keys remain untouched; their scope/payload equality/replay policy belongs to execution.
    /// # Errors
    /// Returns NoMutations or the first original inconsistent member and exact expected/requested data.
    pub fn new(
        id: CommandGroupId,
        mutations: Vec<CommandMutation>,
    ) -> Result<Self, CommandMutationGroupError> {
        let first = mutations
            .first()
            .ok_or(CommandMutationGroupError::NoMutations { group: id })?
            .context();
        for (index, mutation) in mutations.iter().enumerate() {
            let context = mutation.context();
            let command = mutation.command_id();
            if context.group() != Some(id) {
                return Err(CommandMutationGroupError::GroupMismatch {
                    index,
                    command,
                    expected: id,
                    requested: context.group(),
                });
            }
            if context.design() != first.design() {
                return Err(CommandMutationGroupError::DesignMismatch {
                    index,
                    command,
                    expected: first.design(),
                    requested: context.design(),
                });
            }
            if context.actor() != first.actor() {
                return Err(CommandMutationGroupError::ActorMismatch {
                    index,
                    command,
                    expected: first.actor(),
                    requested: context.actor(),
                });
            }
            if context.expected_revision() != first.expected_revision() {
                return Err(CommandMutationGroupError::RevisionMismatch {
                    index,
                    command,
                    expected: first.expected_revision().clone(),
                    requested: context.expected_revision().clone(),
                });
            }
        }
        let context = CommandSnapshotContext::new(
            first.design(),
            first.actor(),
            first.expected_revision().clone(),
        );
        Ok(Self {
            id,
            context,
            mutations,
        })
    }
    /// Original containing group identity, supplied explicitly rather than minted here.
    #[must_use]
    pub const fn id(&self) -> CommandGroupId {
        self.id
    }
    /// Exact common claims; consistency supplies no authenticated/current snapshot.
    #[must_use]
    pub const fn context(&self) -> &CommandSnapshotContext {
        &self.context
    }
    /// Complete original commands, payloads and retry keys in submitted order.
    #[must_use]
    pub fn mutations(&self) -> &[CommandMutation] {
        &self.mutations
    }
}
