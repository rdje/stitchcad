//! Stable physical-copy placements for closure notions, without generated hardware geometry.
use super::anchor::{validate_anchor, validate_current_anchor};
use super::range::range_is_owned;
use super::{
    AnchorError, CutPlan, DirectedRange, DirectedRangeResolution, EdgeAnchor, EntityId,
    GeometricValidation, IdentityLedger, Piece, RangeResolution, Resolution,
};
use core::fmt;

/// Authored physical notion placement, separate from component size and closure kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NotionPlacementDefinition {
    /// Stable placement identity, never an array index.
    pub id: EntityId,
    /// Stable physical copy of the served pattern Piece.
    pub copy: EntityId,
    /// Held attachment point in the pattern Piece's source frame.
    pub anchor: EdgeAnchor,
    /// Authored orientation reference in that source frame, independent of copy reflection.
    pub direction: DirectedRange,
}
/// Immutable structural placement with its original source Piece binding.
/// ```compile_fail
/// fn move_notion(placement: &mut sc_core::ontology::NotionPlacement) {
///     placement.definition.anchor.param = sc_core::ontology::Param::START;
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NotionPlacement {
    piece: EntityId,
    definition: NotionPlacementDefinition,
}
/// Typed target or current geometry-reference refusal; no implicit physical-copy retargeting.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NotionPlacementError {
    /// The held physical copy is missing from the supplied plan.
    MissingCopy(EntityId),
    /// A supplied Piece is not the placement's original owner.
    WrongPiece {
        /// Original Piece binding.
        held: EntityId,
        /// Provided owner.
        provided: EntityId,
    },
    /// The current physical copy names a different source Piece.
    CopyOutsidePiece {
        /// Held physical copy.
        copy: EntityId,
        /// Expected source Piece.
        expected: EntityId,
        /// Source Piece in the current plan.
        actual: EntityId,
    },
    /// Attachment point lacks unique owned resolution.
    Anchor(AnchorError),
    /// Direction includes missing positive intervals or ambiguous/unresolved endpoints.
    UnresolvedDirection(Box<RangeResolution>),
    /// Some current positive direction interval is outside the original source Piece.
    DirectionOutsidePiece(EntityId),
}
impl fmt::Display for NotionPlacementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingCopy(copy) => {
                write!(f, "notion placement physical copy {copy} is missing")
            }
            Self::WrongPiece { held, provided } => write!(
                f,
                "notion placement owner {held} differs from provided {provided}"
            ),
            Self::CopyOutsidePiece {
                copy,
                expected,
                actual,
            } => write!(
                f,
                "notion placement copy {copy} names {actual}, expected {expected}"
            ),
            Self::Anchor(error) => write!(f, "notion placement anchor: {error}"),
            Self::UnresolvedDirection(_) => {
                f.write_str("notion direction needs interval/endpoint repair or choice")
            }
            Self::DirectionOutsidePiece(piece) => write!(
                f,
                "notion direction includes geometry outside piece {piece}"
            ),
        }
    }
}
impl std::error::Error for NotionPlacementError {}
impl NotionPlacement {
    /// Validate born-live anchoring, original copy/owner binding and complete owned direction.
    /// # Errors
    /// Returns [`NotionPlacementError`] for absent/wrong targets or invalid geometry references.
    /// Neither source frame nor reflected-copy orientation grants geometric/release certification.
    pub fn new(
        definition: NotionPlacementDefinition,
        plan: &CutPlan,
        piece: &Piece,
        ledger: &IdentityLedger,
    ) -> Result<Self, NotionPlacementError> {
        let placement = Self {
            piece: piece.id(),
            definition,
        };
        placement.validate_copy(plan, piece)?;
        validate_anchor(definition.anchor, piece, ledger).map_err(NotionPlacementError::Anchor)?;
        placement.validate_direction(piece, ledger)?;
        Ok(placement)
    }
    /// Stable placement identity.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        self.definition.id
    }
    /// Original source Piece identity, guarding later copy reassignment.
    #[must_use]
    pub const fn piece(&self) -> EntityId {
        self.piece
    }
    /// Stable physical copy identity.
    #[must_use]
    pub const fn copy(&self) -> EntityId {
        self.definition.copy
    }
    /// Authored input, exposed without mutation.
    #[must_use]
    pub const fn definition(&self) -> &NotionPlacementDefinition {
        &self.definition
    }
    /// Validate current copy/owner and resolved geometry without requiring the held edge to be live.
    /// # Errors
    /// Returns [`NotionPlacementError`] for absent/reassigned targets, unresolved/foreign anchors
    /// or incomplete/ambiguous/foreign direction. Design still owns global registry validation.
    pub fn validate_current(
        &self,
        plan: &CutPlan,
        piece: &Piece,
        ledger: &IdentityLedger,
    ) -> Result<(), NotionPlacementError> {
        self.validate_copy(plan, piece)?;
        validate_current_anchor(self.definition.anchor, piece, ledger)
            .map_err(NotionPlacementError::Anchor)?;
        self.validate_direction(piece, ledger)
    }
    /// Raw current point evidence, independent of copy/owner validation.
    #[must_use]
    pub fn anchor_resolution(&self, ledger: &IdentityLedger) -> Resolution {
        ledger.resolve(self.definition.anchor.edge, self.definition.anchor.param)
    }
    /// Directed current interval/endpoint evidence without rewriting authored references.
    #[must_use]
    pub fn direction_resolution(&self, ledger: &IdentityLedger) -> DirectedRangeResolution {
        self.definition.direction.resolve(ledger)
    }
    /// Hardware shape, actual placement and reflected geometry remain unproved.
    #[must_use]
    pub const fn geometric_validation(&self) -> GeometricValidation {
        GeometricValidation::DeferredToG2
    }
    fn validate_copy(&self, plan: &CutPlan, piece: &Piece) -> Result<(), NotionPlacementError> {
        if piece.id() != self.piece {
            return Err(NotionPlacementError::WrongPiece {
                held: self.piece,
                provided: piece.id(),
            });
        }
        let copy = plan
            .copy(self.copy())
            .ok_or(NotionPlacementError::MissingCopy(self.copy()))?;
        if copy.piece() != self.piece {
            return Err(NotionPlacementError::CopyOutsidePiece {
                copy: self.copy(),
                expected: self.piece,
                actual: copy.piece(),
            });
        }
        Ok(())
    }
    fn validate_direction(
        &self,
        piece: &Piece,
        ledger: &IdentityLedger,
    ) -> Result<(), NotionPlacementError> {
        let evidence = ledger.resolve_range(self.definition.direction.range);
        if !evidence.has_full_coverage()
            || evidence.start().resolved().is_none()
            || evidence.end().resolved().is_none()
        {
            return Err(NotionPlacementError::UnresolvedDirection(Box::new(
                evidence,
            )));
        }
        if !range_is_owned(&evidence, piece, ledger) {
            return Err(NotionPlacementError::DirectionOutsidePiece(self.piece));
        }
        Ok(())
    }
}
