//! Explicit two-Piece join source intent;association does not splice or publish a boundary.
use crate::ontology::{EntityId, SeamSideId};
use core::fmt;
mod banks;
mod binding;
pub use banks::{BoundJoinCutBanks, JoinCutBank, JoinCutBankError, JoinCutBankRefusal};
pub use binding::{BoundJoinPiecesAtSeam, BoundJoinSide, JoinBindingError, JoinBindingRefusal};
mod coverage;
pub use coverage::{
    CoveredJoinCutSources, JoinCutCoverageError, JoinCutCoverageLimits, JoinCutCoverageRefusal,
};
mod connectivity;
pub use connectivity::{
    JoinCutConnectivityError, JoinCutConnectivityLimits, JoinCutConnectivityRefusal,
    RequiredJoinCutConnectivity,
};
mod construction_uses;
pub use construction_uses::{
    DeclaredJoinConstructionUses, JoinConstructionManifest, JoinConstructionUse,
    JoinConstructionUseError, JoinConstructionUseLimits, JoinConstructionUseRefusal,
    JoinPlacedConstructionUse,
};
mod cut_contours;
pub use cut_contours::{
    DeclaredJoinCutContours, JoinCutContourError, JoinCutContourLimits, JoinCutContourManifest,
    JoinCutContourRefusal, JoinCutContourUse, JoinCutContourUseLocation, JoinCutHole,
    JoinPlacedCutUse,
};
mod directions;
pub use directions::{
    AdmittedJoinDirections, JoinDirectionAssignment, JoinDirectionError, JoinDirectionLimits,
    JoinDirectionManifest, JoinDirectionRefusal, JoinDirectionalOutput, JoinParameterDirection,
};
mod identity;
mod inventory;
pub use identity::{
    DeclaredJoinPieceIdentity, JoinIdentityStopOrigin, JoinPieceIdentityError,
    JoinPieceIdentityLimits, JoinPieceIdentityRefusal,
};
pub use inventory::{
    BoundJoinCopyInventory, JoinCopyInventoryError, JoinCopyInventoryLimits,
    JoinCopyInventoryRefusal,
};
mod manifest;
pub use manifest::{
    AdmittedJoinEdges, JoinEdgeCandidate, JoinEdgeManifest, JoinEdgeManifestError,
    JoinEdgeManifestLimits, JoinEdgeManifestRefusal, JoinOutputEdge,
};
mod fold;
pub use fold::{
    JoinFoldError, JoinFoldIntent, JoinFoldLimits, JoinFoldRefusal, JoinFoldRequirement,
    RequiredJoinFold,
};
mod fold_sources;
pub use fold_sources::{
    DeclaredJoinFoldSources, JoinFoldSourceAction, JoinFoldSourceAssignment, JoinFoldSourceError,
    JoinFoldSourceIntent, JoinFoldSourceLimits, JoinFoldSourceManifest, JoinFoldSourceRefusal,
};
mod companion_repairs;
mod consumers;
mod effects;
mod source_identities;
pub use effects::{
    JoinEffectEvidenceKind, JoinEffectRequirement, JoinEffectSources, RequiredJoinEffects,
};
pub use source_identities::{
    JoinSourceIdentity, JoinSourceIdentityAssignment, JoinSourceIdentityError,
    JoinSourceIdentityIntent, JoinSourceIdentityLimits, JoinSourceIdentityManifest,
    JoinSourceIdentityRefusal, RequiredJoinSourceIdentities,
};
mod original_pieces;
mod result_piece;
pub use companion_repairs::{
    JoinCompanionRepair, JoinCompanionRepairError, JoinCompanionRepairLimits,
    JoinCompanionRepairManifest, JoinCompanionRepairRefusal, JoinCompanionRepairRequest,
    RequiredJoinCompanionRepairs,
};
pub use consumers::{
    JoinConsumerEvidenceKind, JoinConsumerRequirement, JoinConsumerSources, RequiredJoinConsumers,
};
pub use original_pieces::{
    JoinOriginalPiece, JoinOriginalPieceError, JoinOriginalPieceField, JoinOriginalPieceRefusal,
    MatchedJoinOriginalPieces,
};
pub use result_piece::{JoinResultPieceError, JoinResultPieceField, MatchedJoinPieceInventory};
mod inventory_fields;
pub use inventory_fields::{
    BoundJoinInventoryFields, JoinInventoryFieldError, JoinInventoryFieldLimits,
    JoinInventoryFieldRefusal,
};
mod candidate_plan;
pub use candidate_plan::{
    BoundJoinCandidatePlan, JoinCandidatePlanError, JoinCandidatePlanLimits,
    JoinCandidatePlanRefusal,
};
mod copy_pairs;
pub use copy_pairs::{
    JoinCopyPair, JoinCopyPairError, JoinCopyPairLimits, JoinCopyPairRefusal,
    RequiredJoinCopyCorrespondence,
};
mod copy_sources;
mod metadata;
mod population;
pub use copy_sources::{
    DeclaredJoinCopySources, JoinCopySourceAction, JoinCopySourceAssignment, JoinCopySourceError,
    JoinCopySourceIntent, JoinCopySourceLimits, JoinCopySourceManifest, JoinCopySourceRefusal,
};
mod result_copies;
pub use result_copies::{
    DeclaredJoinResultCopies, JoinResultCopyError, JoinResultCopyLimits, JoinResultCopyManifest,
    JoinResultCopyRefusal, JoinResultCopyRequest,
};
mod roles;
pub use roles::{BoundJoinEdgeRoles, JoinEdgeRoleBindingError, JoinEdgeRoleBindingRefusal};
mod source_actions;
pub use source_actions::{
    DeclaredJoinConstructionSources, JoinConstructionSourceAction,
    JoinConstructionSourceAssignment, JoinConstructionSourceError, JoinConstructionSourceIntent,
    JoinConstructionSourceLimits, JoinConstructionSourceManifest, JoinConstructionSourceRefusal,
};
mod source_coverage;
pub use source_coverage::{
    CoveredJoinConstructionGroups, JoinConstructionCoverageError, JoinConstructionCoverageLimits,
    JoinConstructionCoverageRefusal,
};
mod source_groups;
pub use source_groups::{
    DeclaredJoinConstructionGroups, JoinConstructionGroupError, JoinConstructionGroupLimits,
    JoinConstructionGroupManifest, JoinConstructionGroupOutput, JoinConstructionGroupRefusal,
    JoinConstructionMappingGroup, JoinConstructionOutputGroup,
};
mod source_targets;
pub use source_targets::{
    DeclaredJoinConstructionTargets, JoinAdditionalConstructionIntent,
    JoinConstructionTargetAssignment, JoinConstructionTargetError, JoinConstructionTargetLimits,
    JoinConstructionTargetManifest, JoinConstructionTargetRefusal, JoinMappedConstructionTarget,
};
mod sources;
mod target_coverage;
pub use target_coverage::{
    JoinConstructionTargetCoverageError, JoinConstructionTargetCoverageLimits,
    JoinConstructionTargetCoverageRefusal, RequiredJoinConstructionTargets,
};
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
