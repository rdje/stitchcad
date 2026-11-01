//! Typed authored drafting declarations, separate from evaluated geometry and command authority.
//! Records retain original formula documents and persistent logical output references. Complete
//! operation ordering, containing frames, current registries, geometry and publication stay owned.
// Compile the actual maintained examples, so rendered prose cannot drift from public Rust APIs.
#[cfg(doctest)]
#[doc = include_str!("../../../docs/book/src/annexes/drafting-edit-inputs.md")]
mod editing_examples {}
#[cfg(doctest)]
#[doc = include_str!("../../../docs/book/src/annexes/drafting-join-contracts.md")]
mod join_examples {}
#[cfg(doctest)]
#[doc = include_str!("../../../docs/book/src/annexes/drafting-join-sources.md")]
mod join_source_examples {}
#[cfg(doctest)]
#[doc = include_str!("../../../docs/book/src/annexes/drafting-join-topology.md")]
mod join_topology_examples {}
#[cfg(doctest)]
#[doc = include_str!("../../../docs/book/src/annexes/drafting-truing-outputs.md")]
mod truing_output_examples {}
#[cfg(doctest)]
#[doc = include_str!("../../../docs/book/src/annexes/drafting-truing-piece.md")]
mod truing_piece_examples {}
#[cfg(doctest)]
#[doc = include_str!("../../../docs/book/src/annexes/drafting-walk-outputs.md")]
mod walk_output_examples {}
#[cfg(doctest)]
#[doc = include_str!("../../../docs/book/src/annexes/drafting-walk-identities.md")]
mod walk_identity_examples {}
#[cfg(doctest)]
#[doc = include_str!("../../../docs/book/src/annexes/drafting-mirror-contracts.md")]
mod mirror_examples {}
#[cfg(doctest)]
#[doc = include_str!("../../../docs/book/src/annexes/drafting-mirror-outputs.md")]
mod mirror_output_examples {}
#[cfg(doctest)]
#[doc = include_str!("../../../docs/book/src/annexes/drafting-mirror-copies.md")]
mod mirror_copy_examples {}
#[cfg(doctest)]
#[doc = include_str!("../../../docs/book/src/annexes/drafting-mirror-semantics.md")]
mod mirror_semantic_examples {}
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
    AdmittedJoinDirections, AdmittedJoinEdges, BoundJoinCopyInventory, BoundJoinCutBanks,
    BoundJoinCutPopulation, BoundJoinEdgeRoles, BoundJoinPieceMetadata, BoundJoinPiecesAtSeam,
    BoundJoinSide, BoundJoinSourcePieces, CoveredJoinConstructionGroups, CoveredJoinCutSources,
    DeclaredJoinConstructionGroups, DeclaredJoinConstructionSources,
    DeclaredJoinConstructionTargets, DeclaredJoinConstructionUses, DeclaredJoinCutContours,
    DeclaredJoinFoldSources, DeclaredJoinPieceIdentity, JoinAdditionalConstructionIntent,
    JoinBindingError, JoinBindingRefusal, JoinConstructionCoverageError,
    JoinConstructionCoverageLimits, JoinConstructionCoverageRefusal, JoinConstructionGroupError,
    JoinConstructionGroupLimits, JoinConstructionGroupManifest, JoinConstructionGroupOutput,
    JoinConstructionGroupRefusal, JoinConstructionManifest, JoinConstructionMappingGroup,
    JoinConstructionOutputGroup, JoinConstructionSourceAction, JoinConstructionSourceAssignment,
    JoinConstructionSourceError, JoinConstructionSourceIntent, JoinConstructionSourceLimits,
    JoinConstructionSourceManifest, JoinConstructionSourceRefusal,
    JoinConstructionTargetAssignment, JoinConstructionTargetCoverageError,
    JoinConstructionTargetCoverageLimits, JoinConstructionTargetCoverageRefusal,
    JoinConstructionTargetError, JoinConstructionTargetLimits, JoinConstructionTargetManifest,
    JoinConstructionTargetRefusal, JoinConstructionUse, JoinConstructionUseError,
    JoinConstructionUseLimits, JoinConstructionUseRefusal, JoinCopyInventoryError,
    JoinCopyInventoryLimits, JoinCopyInventoryRefusal, JoinCutBank, JoinCutBankError,
    JoinCutBankRefusal, JoinCutConnectivityError, JoinCutConnectivityLimits,
    JoinCutConnectivityRefusal, JoinCutContourError, JoinCutContourLimits, JoinCutContourManifest,
    JoinCutContourRefusal, JoinCutContourUse, JoinCutContourUseLocation, JoinCutCoverageError,
    JoinCutCoverageLimits, JoinCutCoverageRefusal, JoinCutHole, JoinCutPopulation,
    JoinCutPopulationError, JoinCutPopulationRefusal, JoinDirectionAssignment, JoinDirectionError,
    JoinDirectionLimits, JoinDirectionManifest, JoinDirectionRefusal, JoinDirectionalOutput,
    JoinEdgeCandidate, JoinEdgeManifest, JoinEdgeManifestError, JoinEdgeManifestLimits,
    JoinEdgeManifestRefusal, JoinEdgeRoleBindingError, JoinEdgeRoleBindingRefusal,
    JoinExactSpliceRequirement, JoinFoldError, JoinFoldIntent, JoinFoldLimits, JoinFoldRefusal,
    JoinFoldRequirement, JoinFoldSourceAction, JoinFoldSourceAssignment, JoinFoldSourceError,
    JoinFoldSourceIntent, JoinFoldSourceLimits, JoinFoldSourceManifest, JoinFoldSourceRefusal,
    JoinIdentityStopOrigin, JoinInputError, JoinMappedConstructionTarget, JoinMovingOrientation,
    JoinOutputEdge, JoinParameterDirection, JoinPieceFieldIntent, JoinPieceIdentityError,
    JoinPieceIdentityLimits, JoinPieceIdentityRefusal, JoinPieceMetadataError,
    JoinPieceMetadataLimits, JoinPieceMetadataPolicy, JoinPieceMetadataRefusal, JoinPiecesAtSeam,
    JoinPlacedConstructionUse, JoinPlacedCutUse, JoinSourceOccurrence, JoinSourceOccurrenceField,
    JoinSourceOccurrenceLocation, JoinSourcePieceError, JoinSourcePieceLimits,
    JoinSourcePieceRefusal, JoinSpliceEnd, JoinSpliceEndpointPair, JoinSplicePlacement,
    RequiredJoinConstructionTargets, RequiredJoinCutConnectivity, RequiredJoinExactSplice,
    RequiredJoinFold,
};
mod mirror;
pub use mirror::{
    AdmittedMirrorPieceEdges, AdmittedMirrorPiecePoints, AdmittedMirrorPieceResultCopies,
    BoundMirrorPieceCopy, BoundMirrorPieceCopyIdentities, BoundMirrorPieceCopyInventory,
    BoundMirrorPieceCopyPlan, BoundMirrorPieceCutPolicy, BoundMirrorPieceEdgeRoles,
    BoundMirrorPieceMetadata, BoundMirrorPiecePointRoles, BoundMirrorPieceSourceCopies,
    DeclaredMirrorPieceIdentity, MirrorAxis, MirrorAxisDependency, MirrorAxisSource,
    MirrorPieceAxisEvidenceRequest, MirrorPieceContourError, MirrorPieceContourLimits,
    MirrorPieceContourPolicy, MirrorPieceContourRefusal, MirrorPieceContours, MirrorPieceCopy,
    MirrorPieceCopyError, MirrorPieceCopyIdentityError, MirrorPieceCopyIdentityLimits,
    MirrorPieceCopyIdentityRefusal, MirrorPieceCopyInventoryError, MirrorPieceCopyInventoryField,
    MirrorPieceCopyInventoryLimits, MirrorPieceCopyInventoryRefusal, MirrorPieceCopyPlanError,
    MirrorPieceCopyPlanLimits, MirrorPieceCopyPlanRefusal, MirrorPieceCorrespondenceError,
    MirrorPieceCorrespondenceEvidenceKind, MirrorPieceCorrespondenceLimits,
    MirrorPieceCorrespondenceRefusal, MirrorPieceCorrespondenceRequirement, MirrorPieceCutPolicy,
    MirrorPieceCutPolicyError, MirrorPieceCutPolicyRefusal, MirrorPieceEdge,
    MirrorPieceEdgeCandidate, MirrorPieceEdgeCoverage, MirrorPieceEdgeCoverageError,
    MirrorPieceEdgeCoverageLimits, MirrorPieceEdgeCoverageRefusal, MirrorPieceEdgeLocation,
    MirrorPieceEdgeManifest, MirrorPieceEdgeManifestError, MirrorPieceEdgeManifestLimits,
    MirrorPieceEdgeManifestRefusal, MirrorPieceEdgeParameterization, MirrorPieceEdgeSource,
    MirrorPieceFieldIntent, MirrorPieceFoldIntent, MirrorPieceIdentityError,
    MirrorPieceIdentityRefusal, MirrorPieceMappedEdgeOccurrence, MirrorPieceMetadataError,
    MirrorPieceMetadataLimits, MirrorPieceMetadataPolicy, MirrorPieceMetadataRefusal,
    MirrorPieceOutputEdge, MirrorPiecePoint, MirrorPiecePointCandidate, MirrorPiecePointManifest,
    MirrorPiecePointManifestError, MirrorPiecePointManifestLimits, MirrorPiecePointManifestRefusal,
    MirrorPiecePointMatch, MirrorPiecePointMatchKind, MirrorPiecePointQuery,
    MirrorPiecePointQueryError, MirrorPiecePointQueryLimits, MirrorPiecePointQueryRefusal,
    MirrorPiecePointRoleBindingError, MirrorPiecePointRoleBindingRefusal,
    MirrorPieceResultCopyAction, MirrorPieceResultCopyError, MirrorPieceResultCopyIntent,
    MirrorPieceResultCopyLimits, MirrorPieceResultCopyManifest, MirrorPieceResultCopyRefusal,
    MirrorPieceResultCopySource, MirrorPieceRoleBindingError, MirrorPieceRoleBindingRefusal,
    MirrorPieceSemanticConsumerSources, MirrorPieceSemanticEffectSources,
    MirrorPieceSemanticEvidenceKind, MirrorPieceSemanticEvidenceRequirement,
    MirrorPieceSemanticIntent, MirrorPieceSemanticPolicy, MirrorPieceSourceCopyError,
    MirrorPieceSourceCopyLimits, MirrorPieceSourceCopyRefusal, MirrorPieceSourceError,
    MirrorPieceSourceLimits, MirrorPieceSourceRefusal, RequiredMirrorPieceCorrespondence,
    RequiredMirrorPieceCorrespondenceSource, RequiredMirrorPieceSemantics,
};
mod offset;
pub use offset::{
    BoundOffsetConstruction, OffsetBindingError, OffsetConstruction, OffsetConstructionError,
    OffsetSide,
};
mod edge_output_roles;
mod notch_output_roles;
pub use notch_output_roles::{
    ActiveNotchOutputRole, AdmittedNotchOutputRoleCatalog, AdmittedNotchOutputRoleTransition,
    NotchOutputRoleCatalog, NotchOutputRoleCatalogError, NotchOutputRoleCatalogLimits,
    NotchOutputRoleCatalogRefusal, NotchOutputRoleDefinition, NotchOutputRoleTransitionError,
    NotchOutputRoleTransitionLimits, NotchOutputRoleTransitionRefusal,
};
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
    AdmittedWalkNotchAssignments, AdmittedWalkNotchFields, AdmittedWalkNotchIdentities,
    AdmittedWalkSeamEdit, BoundWalkBalancePair, BoundWalkCopyInventory, BoundWalkExistingNotches,
    BoundWalkNotchAssignment, BoundWalkNotchIdentityExclusions, BoundWalkNotchRoles,
    BoundWalkSeamEdit, RequiredWalkNotchEvidence, WalkBalanceNotch, WalkBalancePair,
    WalkCopyInventoryError, WalkCopyInventoryLimits, WalkCopyInventoryRefusal,
    WalkEditAdmissionError, WalkEditAdmissionRefusal, WalkEditBindingError, WalkEditBindingRefusal,
    WalkEditLimits, WalkExistingNotchError, WalkExistingNotchLimits, WalkExistingNotchRefusal,
    WalkNotchAssignmentError, WalkNotchAssignmentLimits, WalkNotchAssignmentManifest,
    WalkNotchAssignmentRefusal, WalkNotchContextSources, WalkNotchEffectSources,
    WalkNotchEvidenceKind, WalkNotchEvidencePolicy, WalkNotchEvidenceRequirement, WalkNotchField,
    WalkNotchFieldError, WalkNotchFieldGroup, WalkNotchFieldIntent, WalkNotchFieldLimits,
    WalkNotchFieldManifest, WalkNotchFieldPolicy, WalkNotchFieldRefusal, WalkNotchIdentityAction,
    WalkNotchIdentityError, WalkNotchIdentityExclusionError, WalkNotchIdentityExclusionLimits,
    WalkNotchIdentityExclusionRefusal, WalkNotchIdentityIntent, WalkNotchIdentityLimits,
    WalkNotchIdentityManifest, WalkNotchIdentityRefusal, WalkNotchIdentitySource,
    WalkNotchRoleAssignment, WalkNotchRoleBindingError, WalkNotchRoleBindingRefusal,
    WalkNotchSharingPolicy, WalkSeamEdit,
};
mod truing;
pub use truing::{
    AdmittedTruingCurveManifest, AdmittedTruingDirections, AdmittedTruingEdges,
    AdmittedTruingOutsideRoles, BoundConstrainedTruing, BoundExactTruingPieceSource,
    BoundTruingCopyInventory, BoundTruingEdgeRoles, BoundTruingSourceDomain, ConstrainedSeamTruing,
    CoveredTruingSourceDomain, RequiredTruingEvidence, RequiredTruingFold, RequiredTruingLineage,
    RequiredTruingLocks, RequiredTruingOutcome, TruingBindingError, TruingBindingRefusal,
    TruingContextSources, TruingContourError, TruingContourLimits, TruingContourRefusal,
    TruingCopyInventoryError, TruingCopyInventoryLimits, TruingCopyInventoryRefusal,
    TruingCurveAlternative, TruingCurveAlternatives, TruingCurveError, TruingCurveGroup,
    TruingCurveLimits, TruingCurveManifest, TruingCurveRefusal, TruingCurveSelectionPolicy,
    TruingDirectionAssignment, TruingDirectionError, TruingDirectionLimits,
    TruingDirectionManifest, TruingDirectionRefusal, TruingDirectionSource,
    TruingDirectionalOutput, TruingEdgeCandidate, TruingEdgeManifest, TruingEdgeManifestError,
    TruingEdgeManifestLimits, TruingEdgeManifestRefusal, TruingEdgeRoleBindingError,
    TruingEdgeRoleBindingRefusal, TruingEffectSources, TruingEndpoint, TruingEndpointRequirement,
    TruingEvidenceKind, TruingEvidencePolicy, TruingEvidenceRequirement, TruingFitProvider,
    TruingFitSources, TruingFoldError, TruingFoldIntent, TruingFoldLimits, TruingFoldRefusal,
    TruingFoldRequirement, TruingInputError, TruingInputRole, TruingLineageError,
    TruingLineageLimits, TruingLineageRefusal, TruingLockError, TruingLockLimits,
    TruingLockRefusal, TruingOutcomeError, TruingOutcomeIntent, TruingOutcomeRefusal,
    TruingOutputEdge, TruingOutsideIntent, TruingOutsideOutput, TruingOutsidePart,
    TruingOutsideRoleError, TruingOutsideRoleLimits, TruingOutsideRoleRefusal, TruingOutsideRoles,
    TruingParameterDirection, TruingPieceContours, TruingPieceOccurrence,
    TruingPieceOccurrenceField, TruingPieceOccurrenceLocation, TruingPieceSourceError,
    TruingPieceSourceLimits, TruingPieceSourceRefusal, TruingPlacedContourEdge,
    TruingSourceCoverageError, TruingSourceCoverageLimits, TruingSourceCoverageRefusal,
    TruingSourceDomainEdge, TruingSourceDomainError, TruingSourceDomainLimits,
    TruingSourceDomainRefusal, TruingSourceIdentityIntent, TruingStopRequirement,
};
