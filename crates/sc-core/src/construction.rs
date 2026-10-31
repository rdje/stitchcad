//! Typed authored drafting declarations, separate from evaluated geometry and command authority.
//! Records retain original formula documents and persistent logical output references. Complete
//! operation ordering, containing frames, current registries, geometry and publication stay owned.
mod arc;
mod arguments;
pub use arc::{
    CenterArcArgument, CenterArcCheckError, CenterArcCheckRefusal, CenterArcConstruction,
    CenterArcConstructionError, CenterArcNormalizationError, CenterArcNormalizationLimits,
    CenterArcNormalizationRefusal, CenterArcPointDependency, CheckedCenterArcConstruction,
    NormalizedCenterArcConstruction,
};
mod line;
pub use line::{LineConstruction, LineConstructionError, LineEndpoint, LinePointDependency};
mod point;
pub use point::{
    CheckedPointConstruction, NormalizedPointConstruction, PointConstruction,
    PointConstructionCheckError, PointConstructionCheckRefusal,
    PointConstructionNormalizationError, PointConstructionNormalizationLimits,
    PointConstructionNormalizationRefusal, PointCoordinate,
};
