//! One typed input surface over the original family requests; no authentication or execution.
use super::{
    CommandActorClaim, CommandArtifact, CommandEvaluation, CommandId, CommandMutation,
    CommandMutationContext, CommandQuery, CommandRelease, DesignRevision,
};
use crate::ontology::EntityId;

/// Complete typed command input for core validation, independent of transport or adapter.
/// Each variant retains its original context and payload; its identity comes from that payload.
/// Constructing or inspecting a request grants no authority and advertises no executable handler.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandRequest {
    /// Read-only observation of the explicit Design snapshot.
    Query(CommandQuery),
    /// Authored change with mandatory retry identity and optional atomic-group identity.
    Mutation(CommandMutation),
    /// Explicit instance or profile evaluation against the requested snapshot.
    Evaluation(CommandEvaluation),
    /// Canonical artifact generation request.
    Artifact(CommandArtifact),
    /// Candidate preparation or detached human approval intent.
    Release(CommandRelease),
}
impl CommandRequest {
    /// Identity derived through the original concrete payload; no independent name can retag it.
    #[must_use]
    pub const fn command_id(&self) -> CommandId {
        match self {
            Self::Query(command) => command.command_id(),
            Self::Mutation(command) => command.command_id(),
            Self::Evaluation(command) => command.command_id(),
            Self::Artifact(command) => command.command_id(),
            Self::Release(command) => command.command_id(),
        }
    }

    /// Original requested Design identity; current membership and access still require validation.
    #[must_use]
    pub const fn design(&self) -> EntityId {
        match self {
            Self::Query(command) => command.context().design(),
            Self::Mutation(command) => command.context().design(),
            Self::Evaluation(command) => command.context().design(),
            Self::Artifact(command) => command.context().design(),
            Self::Release(command) => command.context().design(),
        }
    }

    /// Original untrusted initiator/principal claim, without authentication or permission escalation.
    #[must_use]
    pub const fn actor(&self) -> CommandActorClaim {
        match self {
            Self::Query(command) => command.context().actor(),
            Self::Mutation(command) => command.context().actor(),
            Self::Evaluation(command) => command.context().actor(),
            Self::Artifact(command) => command.context().actor(),
            Self::Release(command) => command.context().actor(),
        }
    }

    /// Borrow the exact original precondition; no latest-revision selection, copy or increment.
    #[must_use]
    pub const fn expected_revision(&self) -> &DesignRevision {
        match self {
            Self::Query(command) => command.context().expected_revision(),
            Self::Mutation(command) => command.context().expected_revision(),
            Self::Evaluation(command) => command.context().expected_revision(),
            Self::Artifact(command) => command.context().expected_revision(),
            Self::Release(command) => command.context().expected_revision(),
        }
    }

    /// Original mutation metadata only. Other classes gain no replay key or undo-group membership.
    #[must_use]
    pub const fn mutation_context(&self) -> Option<&CommandMutationContext> {
        match self {
            Self::Mutation(command) => Some(command.context()),
            Self::Query(_) | Self::Evaluation(_) | Self::Artifact(_) | Self::Release(_) => None,
        }
    }
}
