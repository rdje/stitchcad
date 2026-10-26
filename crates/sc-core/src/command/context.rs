//! Immutable request metadata; claims become trusted only at the later authorization boundary.
use crate::ontology::EntityId;
use core::fmt;
use num_bigint::BigUint;

/// A declared initiator and human principal, not proof of authentication or granted authority.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CommandActorClaim {
    /// A caller declares this human identity as both initiator and principal.
    Human(EntityId),
    /// An agent declares its own identity and the human for whom it is acting.
    Agent {
        /// Declared agent identity.
        identity: EntityId,
        /// Declared granting human; actual grant binding belongs to core authorization.
        principal: EntityId,
    },
}

impl CommandActorClaim {
    /// Exact declared initiator, never replaced with its principal for audit purposes.
    #[must_use]
    pub const fn initiator(self) -> EntityId {
        match self {
            Self::Human(identity) | Self::Agent { identity, .. } => identity,
        }
    }

    /// Exact declared human principal; this getter confers no authentication or permission.
    #[must_use]
    pub const fn principal(self) -> EntityId {
        match self {
            Self::Human(identity) => identity,
            Self::Agent { principal, .. } => principal,
        }
    }
}

/// Exact nonnegative design revision counter, independent of clocks and storage integer widths.
/// A value is a declared precondition, not evidence that it is the current revision of a design.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DesignRevision(BigUint);

impl DesignRevision {
    /// Initial counter value zero.
    #[must_use]
    pub fn initial() -> Self {
        Self::default()
    }

    /// Preserve an explicit counter value without looking up or mutating a design.
    #[must_use]
    pub fn from_counter(value: u128) -> Self {
        Self(BigUint::from(value))
    }

    /// Exact next counter; never wraps, saturates or introduces an undocumented revision limit.
    /// Computing this value neither commits a command nor changes its source revision.
    #[must_use]
    pub fn successor(&self) -> Self {
        Self(&self.0 + 1_u8)
    }
}

impl fmt::Display for DesignRevision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

/// Stable retry identity, with a distinct role from design, actor and group identifiers.
/// Key scope, payload equality and replay resolution belong to the executing bus.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandIdempotencyKey(EntityId);

impl CommandIdempotencyKey {
    /// Preserve the caller's exact identity; no key is minted or authenticated here.
    #[must_use]
    pub const fn from_entity_id(identity: EntityId) -> Self {
        Self(identity)
    }

    /// The exact underlying identity.
    #[must_use]
    pub const fn entity_id(self) -> EntityId {
        self.0
    }
}

/// Stable atomic-group identity; membership and transaction validity belong to the bus.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandGroupId(EntityId);

impl CommandGroupId {
    /// Preserve the supplied group identity without inventing group membership.
    #[must_use]
    pub const fn from_entity_id(identity: EntityId) -> Self {
        Self(identity)
    }

    /// The exact underlying identity.
    #[must_use]
    pub const fn entity_id(self) -> EntityId {
        self.0
    }
}

/// Complete required mutation metadata, before binding a concrete typed payload.
///
/// This data does not authenticate its actor, validate a grant, compare current revisions, assign
/// a missing group, accept an idempotency replay or execute anything. Those checks remain in core.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CommandMutationContext {
    design: EntityId,
    actor: CommandActorClaim,
    expected_revision: DesignRevision,
    idempotency_key: CommandIdempotencyKey,
    group: Option<CommandGroupId>,
}

impl CommandMutationContext {
    /// Preserve all declared metadata. Revision and retry key are mandatory typed arguments.
    /// None for group requests a standalone command; execution still makes it a group of one.
    #[must_use]
    pub const fn new(
        design: EntityId,
        actor: CommandActorClaim,
        expected_revision: DesignRevision,
        idempotency_key: CommandIdempotencyKey,
        group: Option<CommandGroupId>,
    ) -> Self {
        Self {
            design,
            actor,
            expected_revision,
            idempotency_key,
            group,
        }
    }

    /// Declared target design; registry membership is not inferred.
    #[must_use]
    pub const fn design(&self) -> EntityId {
        self.design
    }

    /// Declared initiator/principal claim, retaining both identities.
    #[must_use]
    pub const fn actor(&self) -> CommandActorClaim {
        self.actor
    }

    /// Revision against which the caller authored the mutation.
    #[must_use]
    pub const fn expected_revision(&self) -> &DesignRevision {
        &self.expected_revision
    }

    /// Retry key whose actual scope and first result the bus must check.
    #[must_use]
    pub const fn idempotency_key(&self) -> CommandIdempotencyKey {
        self.idempotency_key
    }

    /// Explicit requested group, or None for the standalone group-of-one case.
    #[must_use]
    pub const fn group(&self) -> Option<CommandGroupId> {
        self.group
    }
}
