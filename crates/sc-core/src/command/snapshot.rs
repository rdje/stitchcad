//! Exact nonmutating command snapshot claims shared by reads and derived-result requests.
use super::{CommandActorClaim, DesignRevision};
use crate::ontology::EntityId;

/// Immutable Design snapshot/actor claims, before current revision and scoped authority validation.
/// Reading or deriving a result neither enters mutation undo groups nor needs a mutation replay key.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CommandSnapshotContext {
    design: EntityId,
    actor: CommandActorClaim,
    expected_revision: DesignRevision,
}
impl CommandSnapshotContext {
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

    /// Unauthenticated actor/principal claim for later class-specific scoped authority validation and attribution.
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
