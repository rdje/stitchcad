//! Explicit two-Piece join source intent;association does not splice or publish a boundary.
use crate::ontology::{EntityId, SeamSideId};
use core::fmt;
mod binding;
pub use binding::{BoundJoinPiecesAtSeam, BoundJoinSide, JoinBindingError, JoinBindingRefusal};
mod inventory;
pub use inventory::{
    BoundJoinCopyInventory, JoinCopyInventoryError, JoinCopyInventoryLimits,
    JoinCopyInventoryRefusal,
};
mod metadata;
mod population;
mod sources;
mod splice;
pub use metadata::{
    BoundJoinPieceMetadata, JoinPieceFieldIntent, JoinPieceMetadataError, JoinPieceMetadataLimits,
    JoinPieceMetadataPolicy, JoinPieceMetadataRefusal,
};
pub use population::{
    BoundJoinCutPopulation, JoinCutPopulation, JoinCutPopulationError, JoinCutPopulationRefusal,
};
pub use sources::{
    BoundJoinSourcePieces, JoinSourceOccurrence, JoinSourceOccurrenceField,
    JoinSourceOccurrenceLocation, JoinSourcePieceError, JoinSourcePieceLimits,
    JoinSourcePieceRefusal,
};
pub use splice::{
    JoinExactSpliceRequirement, JoinMovingOrientation, JoinSpliceEnd, JoinSpliceEndpointPair,
    JoinSplicePlacement, RequiredJoinExactSplice,
};

/// Original local source refusal;global names/currentness and complete dependency checks are separate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinInputError {
    /// The operation cannot supply its own required existing span.
    SelfSpan {
        /// Actual operation and refused span identity.
        operation: EntityId,
    },
    /// The operation cannot supply a required existing pattern Piece.
    SelfPiece {
        /// Actual operation and refused Piece identity.
        operation: EntityId,
        /// Original A/B role.
        side: SeamSideId,
    },
    /// This variant joins two distinct pattern Pieces,not two copies/ranges of one Piece.
    SamePiece {
        /// Actual operation identity.
        operation: EntityId,
        /// Repeated original Piece identity.
        piece: EntityId,
    },
}
impl fmt::Display for JoinInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "join input: {self:?}")
    }
}
impl std::error::Error for JoinInputError {}

/// Deliberate join intent with explicit pattern-Piece identities in canonical seam A/B roles.
/// The two-distinct-Piece variant is explicit;no material reconciliation,source retirement,
/// cut quantity,pose,refit,output identity or geometric weld policy is supplied by association.
/// ```compile_fail
/// let edit:sc_core::construction::JoinPiecesAtSeam=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinPiecesAtSeam {
    operation: EntityId,
    span: EntityId,
    a: EntityId,
    b: EntityId,
}
impl JoinPiecesAtSeam {
    /// Retain explicit source roles;neither ordering nor traversal is inferred from sorted ids.
    /// # Errors
    /// Self-span,then self-Piece A/B,then repeated Piece. Same-Piece copy/seam joins require a
    /// different explicit variant;the underlying SewingGraph may legitimately contain those seams.
    pub fn new(
        operation: EntityId,
        span: EntityId,
        a: EntityId,
        b: EntityId,
    ) -> Result<Self, JoinInputError> {
        if operation == span {
            return Err(JoinInputError::SelfSpan { operation });
        }
        for (side, piece) in [(SeamSideId::A, a), (SeamSideId::B, b)] {
            if operation == piece {
                return Err(JoinInputError::SelfPiece { operation, side });
            }
        }
        if a == b {
            return Err(JoinInputError::SamePiece {
                operation,
                piece: a,
            });
        }
        Ok(Self {
            operation,
            span,
            a,
            b,
        })
    }
    /// Original requesting operation,without command authority or fresh identity issuance.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        self.operation
    }
    /// Required canonical SewingGraph span.
    #[must_use]
    pub const fn span_id(&self) -> EntityId {
        self.span
    }
    /// Required pattern Piece for the original A/B role,not a physical-copy id.
    #[must_use]
    pub const fn piece_id(&self, side: SeamSideId) -> EntityId {
        match side {
            SeamSideId::A => self.a,
            SeamSideId::B => self.b,
        }
    }
}
