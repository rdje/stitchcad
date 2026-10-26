//! Typed read intents bound to an exact requested Design snapshot, without execution or grants.
use super::{CommandActorClaim, CommandId, DesignRevision};
use crate::ontology::EntityId;

/// Immutable query snapshot/actor claims, before current revision and scoped authority validation.
/// Queries neither enter mutation undo groups nor require a mutation replay key.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CommandQueryContext {
    design: EntityId,
    actor: CommandActorClaim,
    expected_revision: DesignRevision,
}
impl CommandQueryContext {
    /// Preserve the explicit target, actor/principal claim and exact requested snapshot revision.
    /// Constructing a context does not resolve the Design, authenticate an actor or pin live storage.
    #[must_use]
    pub const fn new(
        design: EntityId,
        actor: CommandActorClaim,
        expected_revision: DesignRevision,
    ) -> Self {
        Self {
            design,
            actor,
            expected_revision,
        }
    }

    /// Actual requested Design identity.
    #[must_use]
    pub const fn design(&self) -> EntityId {
        self.design
    }

    /// Unauthenticated actor/principal claim for later scoped inspect validation and attribution.
    #[must_use]
    pub const fn actor(&self) -> CommandActorClaim {
        self.actor
    }

    /// Exact requested Design revision, without a latest-revision fallback.
    #[must_use]
    pub const fn expected_revision(&self) -> &DesignRevision {
        &self.expected_revision
    }
}

/// Implemented query schemas over current canonical entities; handler availability is separate.
/// Artifact-specific uncertainty-closure input follows the concrete artifact request contracts.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QueryPayload {
    /// Read every canonical piece's multiplicity, pairing and material within the requested Design.
    GetPieceList,
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
