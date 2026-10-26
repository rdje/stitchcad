//! Construction intents reference canonical content; actual geometry and registry checks follow.
use crate::ontology::{EdgeAnchor, EntityId, NotchDefinition, NotchProfileBindings, SeamSideId};

/// Split a canonical sewing correspondence at explicit source-frame anchors on both named sides.
/// The original span supplies physical copies, ranges, direction, ease and stops. This intent does
/// not allocate result identities or assume that equal parameters imply sewing correspondence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SplitSeamSpan {
    span: EntityId,
    side_a: EdgeAnchor,
    side_b: EdgeAnchor,
}

impl SplitSeamSpan {
    /// Retain both explicit cuts without changing either original edge/parameter frame.
    /// Current span/copy ownership, strict interior cuts, paired correspondence and ease/stop/lineage
    /// preservation require actual registry and geometric validation before execution can mutate.
    #[must_use]
    pub const fn new(span: EntityId, side_a: EdgeAnchor, side_b: EdgeAnchor) -> Self {
        Self {
            span,
            side_a,
            side_b,
        }
    }

    /// Canonical sewing correspondence to resolve; this is not a drafting-edge split target.
    #[must_use]
    pub const fn span(self) -> EntityId {
        self.span
    }

    /// Exact anchor for the named canonical side, without direction/reflection remapping.
    #[must_use]
    pub const fn cut(self, side: SeamSideId) -> EdgeAnchor {
        match side {
            SeamSideId::A => self.side_a,
            SeamSideId::B => self.side_b,
        }
    }
}

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
