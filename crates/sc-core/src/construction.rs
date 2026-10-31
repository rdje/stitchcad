//! Typed authored drafting declarations, separate from evaluated geometry and command authority.
//! Records retain original formula documents and persistent logical output references. Complete
//! operation ordering, containing frames, current registries, geometry and publication stay owned.
// Compile the actual maintained examples, so rendered prose cannot drift from public Rust APIs.
#[cfg(doctest)]
#[doc = include_str!("../../../docs/book/src/annexes/drafting-edit-inputs.md")]
mod editing_examples {}
#[cfg(doctest)]
#[doc = include_str!("../../../docs/book/src/annexes/drafting-mirror-contracts.md")]
mod mirror_examples {}
#[cfg(doctest)]
#[doc = include_str!("../../../docs/book/src/annexes/drafting-output-contracts.md")]
mod output_examples {}
#[cfg(doctest)]
#[doc = include_str!("../../../docs/book/src/annexes/drafting-slash-output-contracts.md")]
mod slash_output_examples {}
#[cfg(doctest)]
#[doc = include_str!("../../../docs/book/src/annexes/drafting-slash-piece-contracts.md")]
mod slash_piece_examples {}
mod arc;
mod arguments;
mod bezier;
pub use arc::{
    CenterArcArgument, CenterArcCheckError, CenterArcCheckRefusal, CenterArcConstruction,
    CenterArcConstructionError, CenterArcNormalizationError, CenterArcNormalizationLimits,
    CenterArcNormalizationRefusal, CenterArcPointDependency, CheckedCenterArcConstruction,
    NormalizedCenterArcConstruction,
};
pub use bezier::{
    BezierConstruction, BezierConstructionError, BezierControlPoints, BezierDegree,
    BezierPointDependency, BezierPointRole,
};
mod grainline;
pub use grainline::{BoundGrainlinePlacement, GrainlinePlacement, GrainlinePlacementError};
mod line;
pub use line::{LineConstruction, LineConstructionError, LineEndpoint, LinePointDependency};
mod join;
pub use join::{
    BoundJoinPiecesAtSeam, BoundJoinSide, JoinBindingError, JoinBindingRefusal, JoinInputError,
    JoinPiecesAtSeam,
};
mod mirror;
pub use mirror::{
    BoundMirrorPieceCopy, BoundMirrorPieceCutPolicy, BoundMirrorPieceEdgeRoles,
    BoundMirrorPieceMetadata, DeclaredMirrorPieceIdentity, MirrorAxis, MirrorAxisDependency,
    MirrorAxisSource, MirrorPieceCopy, MirrorPieceCopyError, MirrorPieceCutPolicy,
    MirrorPieceCutPolicyError, MirrorPieceCutPolicyRefusal, MirrorPieceEdge,
    MirrorPieceEdgeLocation, MirrorPieceEdgeSource, MirrorPieceFieldIntent, MirrorPieceFoldIntent,
    MirrorPieceIdentityError, MirrorPieceIdentityRefusal, MirrorPieceMetadataError,
    MirrorPieceMetadataLimits, MirrorPieceMetadataPolicy, MirrorPieceMetadataRefusal,
    MirrorPieceRoleBindingError, MirrorPieceRoleBindingRefusal, MirrorPieceSourceError,
    MirrorPieceSourceLimits, MirrorPieceSourceRefusal,
};
mod offset;
pub use offset::{
    BoundOffsetConstruction, OffsetBindingError, OffsetConstruction, OffsetConstructionError,
    OffsetSide,
};
mod edge_output_roles;
mod point_output_roles;
pub use point_output_roles::{
    ActivePointOutputRole, AdmittedPointOutputRoleCatalog, AdmittedPointOutputRoleTransition,
    PointOutputRoleCatalog, PointOutputRoleCatalogError, PointOutputRoleCatalogLimits,
    PointOutputRoleCatalogRefusal, PointOutputRoleDefinition, PointOutputRoleTransitionError,
    PointOutputRoleTransitionLimits, PointOutputRoleTransitionRefusal,
};
mod offset_manifest;
pub use edge_output_roles::{
    ActiveEdgeOutputRole, AdmittedEdgeOutputRoleCatalog, AdmittedEdgeOutputRoleTransition,
    EdgeOutputRoleCatalog, EdgeOutputRoleCatalogError, EdgeOutputRoleCatalogLimits,
    EdgeOutputRoleCatalogRefusal, EdgeOutputRoleDefinition, EdgeOutputRoleTransitionError,
    EdgeOutputRoleTransitionLimits, EdgeOutputRoleTransitionRefusal,
};
/// Existing offset names remain aliases of the shared edge-output role metadata types.
pub use edge_output_roles::{
    ActiveEdgeOutputRole as ActiveOffsetRole,
    AdmittedEdgeOutputRoleCatalog as AdmittedOffsetRoleCatalog,
    AdmittedEdgeOutputRoleTransition as AdmittedOffsetRoleTransition,
    EdgeOutputRoleCatalog as OffsetRoleCatalog,
    EdgeOutputRoleCatalogError as OffsetRoleCatalogError,
    EdgeOutputRoleCatalogLimits as OffsetRoleCatalogLimits,
    EdgeOutputRoleCatalogRefusal as OffsetRoleCatalogRefusal,
    EdgeOutputRoleDefinition as OffsetRoleDefinition,
    EdgeOutputRoleTransitionError as OffsetRoleTransitionError,
    EdgeOutputRoleTransitionLimits as OffsetRoleTransitionLimits,
    EdgeOutputRoleTransitionRefusal as OffsetRoleTransitionRefusal,
};
pub use offset_manifest::{
    ActiveOffsetCandidate, ActiveOffsetCandidates, AdmittedOffsetCandidates,
    DeclaredOffsetCandidate, DeclaredOffsetCandidates, OffsetCandidateError,
    OffsetCandidateFragment, OffsetCandidateLimits, OffsetCandidateManifest, OffsetCandidateMatch,
    OffsetCandidateMatchKind, OffsetCandidateQuery, OffsetCandidateQueryError,
    OffsetCandidateQueryRefusal, OffsetCandidateRefusal, OffsetCorrespondenceKind,
    OffsetDeclarationError, OffsetDeclarationLimits, OffsetDeclarationRefusal,
    OffsetMappingErrorAllocation, OffsetMappingErrorPart, OffsetMappingEvidenceRequest,
    OffsetMappingRequirement, OffsetMappingRequirementError, OffsetMappingRequirementRefusal,
    OffsetRolePresenceError, OffsetRolePresenceLimits, OffsetRolePresenceRefusal,
    RequiredOffsetMapping,
};
mod point;
pub use point::{
    CheckedPointConstruction, NormalizedPointConstruction, PointConstruction,
    PointConstructionCheckError, PointConstructionCheckRefusal,
    PointConstructionNormalizationError, PointConstructionNormalizationLimits,
    PointConstructionNormalizationRefusal, PointCoordinate,
};
mod slash_opening;
pub use slash_opening::{
    CheckedSlashSpreadOpening, NormalizedSlashSpreadOpening, SlashSpreadCheckError,
    SlashSpreadCheckRefusal, SlashSpreadNormalizationError, SlashSpreadNormalizationLimits,
    SlashSpreadNormalizationRefusal, SlashSpreadOpening, SlashSpreadOpeningMode,
};
mod slash_spread;
pub use slash_spread::{
    AdmittedSlashSpreadContours, AdmittedSlashSpreadEdges, BoundSlashSpreadCopies,
    BoundSlashSpreadEdgeRoles, BoundSlashSpreadPieceMetadata, BoundSlashSpreadRegistrationPlan,
    BoundSlashSpreadSourcePlan, RequiredSlashSpreadBoundary, RequiredSlashSpreadConsumerEvidence,
    RequiredSlashSpreadEffect, RequiredSlashSpreadOutcome, SlashSpreadBoundaryEvidenceRequest,
    SlashSpreadBoundaryPolicy, SlashSpreadBoundaryRequirement, SlashSpreadBoundaryRequirementError,
    SlashSpreadBridgeDirection, SlashSpreadBridgeEndpoint, SlashSpreadConstruction,
    SlashSpreadConstructionError, SlashSpreadConsumerEvidenceKind,
    SlashSpreadConsumerEvidenceRequirement, SlashSpreadConsumerEvidenceSources,
    SlashSpreadContourEdge, SlashSpreadContourError, SlashSpreadContourLimits,
    SlashSpreadContourLocation, SlashSpreadContourRefusal, SlashSpreadContourUse,
    SlashSpreadContours, SlashSpreadCopyError, SlashSpreadCopyLimits, SlashSpreadCopyRefusal,
    SlashSpreadCutEnd, SlashSpreadDependency, SlashSpreadEdge, SlashSpreadEdgeCandidate,
    SlashSpreadEdgeManifest, SlashSpreadEdgeManifestError, SlashSpreadEdgeManifestLimits,
    SlashSpreadEdgeManifestRefusal, SlashSpreadEdgeOrigin, SlashSpreadEffectRequirement,
    SlashSpreadFoldIntent, SlashSpreadFullnessIntent, SlashSpreadGlobalSourceIntent,
    SlashSpreadOutcomeError, SlashSpreadOutcomeIntent, SlashSpreadOutcomeRefusal,
    SlashSpreadOutputIntent, SlashSpreadPieceMetadataError, SlashSpreadPieceMetadataRefusal,
    SlashSpreadPieceMetadataRequirement, SlashSpreadReferenceMatch, SlashSpreadReferenceMatchKind,
    SlashSpreadReferenceOrigin, SlashSpreadReferenceQuery, SlashSpreadReferenceQueryError,
    SlashSpreadReferenceQueryLimits, SlashSpreadReferenceQueryRefusal,
    SlashSpreadRegisteredSourceRequest, SlashSpreadRegistrationDisposition,
    SlashSpreadRegistrationIntent, SlashSpreadRegistrationPlan, SlashSpreadRegistrationPlanError,
    SlashSpreadRegistrationPlanLimits, SlashSpreadRegistrationPlanRefusal,
    SlashSpreadRoleBindingError, SlashSpreadRoleBindingRefusal, SlashSpreadSide, SlashSpreadSource,
    SlashSpreadSourceDisposition, SlashSpreadSourceMotion, SlashSpreadSourcePlan,
    SlashSpreadSourcePlanError, SlashSpreadSourcePlanLimits, SlashSpreadSourcePlanRefusal,
    SlashSpreadSourceRequest,
};
mod walk;
pub use walk::{
    AdmittedWalkSeamEdit, BoundWalkBalancePair, BoundWalkSeamEdit, WalkBalanceNotch,
    WalkBalancePair, WalkEditAdmissionError, WalkEditAdmissionRefusal, WalkEditBindingError,
    WalkEditBindingRefusal, WalkEditLimits, WalkSeamEdit,
};
mod truing;
pub use truing::{
    BoundConstrainedTruing, ConstrainedSeamTruing, TruingBindingError, TruingBindingRefusal,
    TruingFitProvider, TruingInputError, TruingInputRole,
};
