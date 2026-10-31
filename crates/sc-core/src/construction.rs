//! Typed authored drafting declarations, separate from evaluated geometry and command authority.
//! Records retain original formula documents and persistent logical output references. Complete
//! operation ordering, containing frames, current registries, geometry and publication stay owned.
// Compile the actual maintained examples, so rendered prose cannot drift from public Rust APIs.
#[cfg(doctest)]
#[doc = include_str!("../../../docs/book/src/annexes/drafting-edit-inputs.md")]
mod editing_examples {}
#[cfg(doctest)]
#[doc = include_str!("../../../docs/book/src/annexes/drafting-output-contracts.md")]
mod output_examples {}
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
    MirrorAxis, MirrorAxisDependency, MirrorAxisSource, MirrorPieceCopy, MirrorPieceCopyError,
};
mod offset;
pub use offset::{
    BoundOffsetConstruction, OffsetBindingError, OffsetConstruction, OffsetConstructionError,
    OffsetSide,
};
mod offset_manifest;
pub use offset_manifest::{
    AdmittedOffsetCandidates, DeclaredOffsetCandidate, DeclaredOffsetCandidates,
    OffsetCandidateError, OffsetCandidateFragment, OffsetCandidateLimits, OffsetCandidateManifest,
    OffsetCandidateMatch, OffsetCandidateMatchKind, OffsetCandidateQuery,
    OffsetCandidateQueryError, OffsetCandidateQueryRefusal, OffsetCandidateRefusal,
    OffsetDeclarationError, OffsetDeclarationLimits, OffsetDeclarationRefusal,
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
    SlashSpreadConstruction, SlashSpreadConstructionError, SlashSpreadDependency, SlashSpreadSide,
    SlashSpreadSource,
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
