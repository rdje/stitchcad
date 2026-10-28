//! History lifecycle intent keeps the current operator separate from the original authoring group.
use super::{CommandActorClaim, CommandGroupId, CommandMutationContext, CommandMutationGroup};
use crate::ontology::EntityId;
use core::fmt;

/// Whole-group history traversal; neither direction is a new authoring command identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandHistoryDirection {
    /// Restore the selected group's complete original predecessor snapshot.
    Undo,
    /// Restore the selected group's complete original accepted snapshot and author.
    Redo,
}

/// Inconsistent submitted history metadata; no accepted-record lookup has occurred.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandHistoryRequestError {
    /// The operator context omits or names another target group.
    GroupMismatch {
        /// Exact original submitted authoring group identity.
        expected: CommandGroupId,
        /// Original operator-context membership, including absent membership.
        requested: Option<CommandGroupId>,
    },
    /// The operator context names another Design than the original submitted group.
    DesignMismatch {
        /// Exact original submitted authoring Design.
        expected: EntityId,
        /// Inconsistent operator-context Design claim.
        requested: EntityId,
    },
}
impl fmt::Display for CommandHistoryRequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "history request refused: {self:?}")
    }
}
impl std::error::Error for CommandHistoryRequestError {}

/// Immutable history-operation intent with current operator/revision/key and exact original group.
/// The operator may differ from the original author; both claims remain exact and untrusted here.
/// Original revision/payloads/keys are never rewritten as a newly authored mutation group.
/// Construction grants no permission, current snapshot, accepted-record binding, replay or effect.
/// Runtime must authenticate the current operator over every original command scope, bind the exact
/// original accepted record, check the current revision/key and enforce bounded atomic history.
///
/// ```compile_fail
/// use sc_core::command::{CommandHistoryRequest, CommandMutationGroup};
/// fn replace_original(request: &mut CommandHistoryRequest, other: CommandMutationGroup) {
///     request.original = other;
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandHistoryRequest {
    context: CommandMutationContext,
    original: CommandMutationGroup,
    direction: CommandHistoryDirection,
}
impl CommandHistoryRequest {
    pub(crate) fn resource_parts(&self) -> (&CommandMutationContext, &CommandMutationGroup) {
        let Self {
            context,
            original,
            direction,
        } = self;
        super::resource::fixed(direction);
        (context, original)
    }
    /// Check explicit target membership, then Design consistency without looking up model/history.
    /// Current operator and revision need not equal the original author's prior context. Operation
    /// retry-key scope/equality and current authority remain runtime checks, with no silent minting.
    /// # Errors
    /// Returns the original absent/wrong group, then inconsistent Design with exact claim values.
    pub fn new(
        context: CommandMutationContext,
        original: CommandMutationGroup,
        direction: CommandHistoryDirection,
    ) -> Result<Self, CommandHistoryRequestError> {
        if context.group() != Some(original.id()) {
            return Err(CommandHistoryRequestError::GroupMismatch {
                expected: original.id(),
                requested: context.group(),
            });
        }
        if context.design() != original.context().design() {
            return Err(CommandHistoryRequestError::DesignMismatch {
                expected: original.context().design(),
                requested: context.design(),
            });
        }
        Ok(Self {
            context,
            original,
            direction,
        })
    }
    /// Original operator intent, with explicit Design/actor/current revision/retry/target-group data.
    #[must_use]
    pub const fn context(&self) -> &CommandMutationContext {
        &self.context
    }
    /// Exact original submitted authoring group, including its author, base revision and all members.
    #[must_use]
    pub const fn original(&self) -> &CommandMutationGroup {
        &self.original
    }
    /// Original submitted author claim; accepted-record binding remains a runtime obligation.
    #[must_use]
    pub const fn original_actor(&self) -> CommandActorClaim {
        self.original.context().actor()
    }
    /// Requested whole-group traversal, without executing it or changing the original author.
    #[must_use]
    pub const fn direction(&self) -> CommandHistoryDirection {
        self.direction
    }
}
