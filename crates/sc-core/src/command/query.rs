//! Typed read intents bound to an exact requested Design snapshot, without execution or grants.
use super::{ArtifactPayload, CommandId};
use crate::ontology::EntityId;

/// Compatibility name for the shared exact nonmutating snapshot context.
pub use super::CommandSnapshotContext as CommandQueryContext;

/// Implemented query schemas over current canonical entities; handler availability is separate.
/// Uncertainty inspection names the complete concrete artifact input, never a guessed global closure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QueryPayload {
    /// Read every canonical piece's multiplicity, pairing and material within the requested Design.
    GetPieceList,
    /// Inspect only unknowns affecting this exact requested artifact and configuration.
    /// A handler must traverse its actual dependencies; this payload does not compute the closure.
    GetUncertaintyClosure {
        /// Identical concrete input schema used to request generation of the artifact.
        artifact: ArtifactPayload,
    },
    /// Read the named canonical correspondence's differential and declared ease attribution.
    /// Canonical copies/ranges/direction/ease/stops are resolved by execution, never copied here.
    WalkSeamSpan {
        /// Actual correspondence identity, not a drafting edge, range or physical-copy identity.
        span: EntityId,
    },
}
impl QueryPayload {
    /// Identity fixed by the actual typed payload, without a caller-supplied command assertion.
    #[must_use]
    pub const fn command_id(&self) -> CommandId {
        match self {
            Self::GetPieceList => CommandId::GetPieceList,
            Self::GetUncertaintyClosure { .. } => CommandId::GetUncertaintyClosure,
            Self::WalkSeamSpan { .. } => CommandId::WalkSeamSpan,
        }
    }
}

/// Concrete read intent and its exact immutable snapshot context, before validation or execution.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandQuery {
    context: CommandQueryContext,
    payload: QueryPayload,
}
impl CommandQuery {
    /// Bind context and payload without resolving geometry, revisions or authority.
    #[must_use]
    pub const fn new(context: CommandQueryContext, payload: QueryPayload) -> Self {
        Self { context, payload }
    }

    /// Original explicit Design/revision/actor claims.
    #[must_use]
    pub const fn context(&self) -> &CommandQueryContext {
        &self.context
    }

    /// Original canonical query targets.
    #[must_use]
    pub const fn payload(&self) -> &QueryPayload {
        &self.payload
    }

    /// Actual command identity derived from its payload.
    #[must_use]
    pub const fn command_id(&self) -> CommandId {
        self.payload.command_id()
    }
}
