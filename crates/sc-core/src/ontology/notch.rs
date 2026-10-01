//! Semantic notch anchors and symbolic Factory Profile bindings (ontology §4.5).
//!
//! Anchor identity is validated now; physical representation belongs to the target profile at
//! instantiation/export. There is no method that turns an unread binding into geometry or an encoding.

use super::anchor::validate_anchor;
use super::{EntityId, IdentityLedger, Piece, Resolution};

pub use super::anchor::{AnchorError as NotchError, EdgeAnchor};

/// A logical profile-parameter declaration, resolved by the target profile at instantiation.
///
/// This is an identity, not a scalar, selected enum value or a pinned factory/profile revision.
/// G4 validates existence, field kind, value/state and evidence against the actual target profile.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProfileParameterRef(EntityId);

impl ProfileParameterRef {
    /// Name a logical parameter. This alone does not assert that a profile contains or resolves it.
    #[must_use]
    pub const fn new(id: EntityId) -> Self {
        Self(id)
    }

    /// The logical declaration's stable identity.
    #[must_use]
    pub const fn id(self) -> EntityId {
        self.0
    }
}

/// Profile-binding validation is a visible obligation, distinct from valid semantic anchoring.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProfileBindingValidation {
    /// G4 must validate and resolve each binding; no physical value is supplied by G1.
    DeferredToG4,
}

/// Possible notch styles in ontology §4.5, selected by a resolved profile, never defaulted by a notch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NotchStyle {
    /// Single matching mark.
    Single,
    /// Double matching mark.
    Double,
    /// V-shaped mark.
    V,
    /// I-shaped mark.
    I,
    /// T-shaped mark.
    T,
    /// U-shaped mark.
    U,
    /// Castle-shaped mark.
    Castle,
    /// Slit mark.
    Slit,
    /// Drill mark.
    Drill,
}

/// Possible export encodings; selection belongs to the target profile, not the semantic anchor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NotchEncoding {
    /// Coded point with position, direction, depth and type.
    CodedPoint,
    /// Explicit drawn geometry.
    DrawnGeometry,
}

/// Profile-owned length declarations for one notch-production context.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NotchDimensionBindings {
    /// Notch depth, expected to resolve to a length.
    pub depth: ProfileParameterRef,
    /// Notch width, expected to resolve to a length.
    pub width: ProfileParameterRef,
}

/// Symbolic physical representation; sample and production bindings are deliberately separate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NotchProfileBindings {
    /// Declaration expected to resolve to a [`NotchStyle`].
    pub style: ProfileParameterRef,
    /// Sample-room length declarations.
    pub sample: NotchDimensionBindings,
    /// Production length declarations; may deliberately refer to the same declarations as sample.
    pub production: NotchDimensionBindings,
    /// Declaration expected to resolve to a [`NotchEncoding`].
    pub encoding: ProfileParameterRef,
}

/// Editable input, distinct from a validated semantic [`Notch`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NotchDefinition {
    /// Stable semantic matching-mark identity.
    pub id: EntityId,
    /// Position along an edge, never a tessellation vertex index.
    pub anchor: EdgeAnchor,
    /// Symbolic target-profile declarations, with no copied or default values.
    pub representation: NotchProfileBindings,
}

/// An immutable semantic mark with a born-valid anchor and explicitly deferred profile bindings.
///
/// ```compile_fail
/// fn move_notch(notch: &mut sc_core::ontology::Notch) {
///     notch.definition.anchor.param = sc_core::ontology::Param::START;
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Notch {
    piece: EntityId,
    definition: NotchDefinition,
}

impl Notch {
    /// Validate the anchor against the current topology and its named piece's surviving intervals.
    ///
    /// # Errors
    /// Returns [`NotchError`] when the edge is absent, the position does not belong to this piece,
    /// or the reference is not born uniquely resolved. Profile bindings remain visibly deferred.
    pub fn new(
        definition: NotchDefinition,
        piece: &Piece,
        ledger: &IdentityLedger,
    ) -> Result<Self, NotchError> {
        validate_anchor(definition.anchor, piece, ledger)?;
        Ok(Self {
            piece: piece.id(),
            definition,
        })
    }

    /// Stable semantic mark identity.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        self.definition.id
    }

    /// Named piece owner.
    #[must_use]
    pub const fn piece(&self) -> EntityId {
        self.piece
    }

    /// Authored content, available by shared reference only.
    #[must_use]
    pub const fn definition(&self) -> &NotchDefinition {
        &self.definition
    }

    /// Expose the outstanding target-profile binding obligation without supplying physical defaults.
    #[must_use]
    pub const fn profile_binding_validation(&self) -> ProfileBindingValidation {
        ProfileBindingValidation::DeferredToG4
    }

    /// Resolve the unchanged held anchor; at a split point the consumer must explicitly choose a side.
    #[must_use]
    pub fn resolve(&self, ledger: &IdentityLedger) -> Resolution {
        let anchor = self.definition.anchor;
        ledger.resolve(anchor.edge, anchor.param)
    }
}
