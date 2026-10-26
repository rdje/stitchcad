//! Construction intents reference canonical content; actual geometry and registry checks follow.
use crate::ontology::{EdgeAnchor, EntityId, NotchDefinition, NotchProfileBindings};

/// Request closure of the named canonical dart, using its held operation, intake and references.
/// No caller-copied geometry or resolved intake substitutes for that canonical content.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CloseDart {
    dart: EntityId,
}

impl CloseDart {
    /// Preserve the actual requested dart identity; registry/currentness checks belong to execution.
    #[must_use]
    pub const fn new(dart: EntityId) -> Self {
        Self { dart }
    }

    /// Canonical dart target whose closing operation the bus must resolve and validate.
    #[must_use]
    pub const fn dart(self) -> EntityId {
        self.dart
    }
}

/// Fields for a new semantic notch, without a client-supplied identity for the new entity.
/// Anchor ownership/currentness and profile bindings are checked against actual records later.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AddNotch {
    piece: EntityId,
    anchor: EdgeAnchor,
    representation: NotchProfileBindings,
}

impl AddNotch {
    /// Preserve canonical anchor and symbolic sample/production/profile bindings without defaults.
    #[must_use]
    pub const fn new(
        piece: EntityId,
        anchor: EdgeAnchor,
        representation: NotchProfileBindings,
    ) -> Self {
        Self {
            piece,
            anchor,
            representation,
        }
    }

    /// Requested owning piece; this getter supplies no registry proof.
    #[must_use]
    pub const fn piece(self) -> EntityId {
        self.piece
    }

    /// Exact persistent edge and parameter, never a tessellation position.
    #[must_use]
    pub const fn anchor(self) -> EdgeAnchor {
        self.anchor
    }

    /// Original symbolic representation bindings, including both sample and production.
    #[must_use]
    pub const fn representation(self) -> NotchProfileBindings {
        self.representation
    }

    /// Compose the actual canonical input with an explicitly supplied new identity.
    /// The executing bus supplies a fresh allocated id; this helper allocates nothing and does
    /// not validate uniqueness, ownership, a current anchor, profile truth or release readiness.
    #[must_use]
    pub const fn definition_with_id(self, identity: EntityId) -> NotchDefinition {
        NotchDefinition {
            id: identity,
            anchor: self.anchor,
            representation: self.representation,
        }
    }
}
