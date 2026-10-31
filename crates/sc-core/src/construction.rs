//! Typed authored drafting declarations, separate from evaluated geometry and command authority.
//! Records retain original formula documents and persistent logical output references. Complete
//! operation ordering, containing frames, current registries, geometry and publication stay owned.
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
mod offset;
pub use offset::{
    BoundOffsetConstruction, OffsetBindingError, OffsetConstruction, OffsetConstructionError,
    OffsetSide,
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
