//! Actual source/order/graph provenance and original mixed dependencies before values.
mod canonical;
pub use canonical::{
    BoundDraftingProgramCanonicalInputs, DraftingProgramCanonicalBinding,
    DraftingProgramCanonicalError, DraftingProgramCanonicalJoinFailure,
    DraftingProgramCanonicalLimits, DraftingProgramCanonicalOccurrence,
    DraftingProgramCanonicalRefusal, DraftingProgramCanonicalShape, DraftingProgramDerivedBinding,
};
mod formula_geometry;
pub use formula_geometry::{
    DraftingProgramFormulaGeometryConsumer, DraftingProgramFormulaGeometryUse,
};
mod geometry_requests;
mod geometry_work;
pub use geometry_work::{
    AdmittedDraftingProgramGeometryWork, DraftingProgramGeometryLimits,
    DraftingProgramGeometryShape, DraftingProgramGeometryWorkError,
    DraftingProgramGeometryWorkRefusal,
};
mod native_geometry;
pub use geometry_requests::{
    DraftingGeometryStepRequest, DraftingProgramGeometryCreatorRefusal,
    DraftingProgramGeometryOccurrence, DraftingProgramGeometryRequest,
    DraftingProgramGeometryTarget,
};
mod occurrences;
pub use native_geometry::DraftingProgramNativeGeometryUse;
mod owners;
pub use occurrences::{DraftingProgramDependencyRole, DraftingProgramNameDependency};
pub use owners::{
    DraftingProgramDependencyInputs, DraftingProgramDependencyOwnerError,
    DraftingProgramDependencyOwnerIssue,
};

mod geometry_bind;
pub use geometry_bind::{
    BoundDraftingProgramGeometry, DraftingProgramGeometryBindingRefusal,
    DraftingProgramGeometryError, DraftingProgramGeometryRefusal, DraftingProgramGeometrySupplier,
    DraftingProgramPlannedGeometryIssue,
};
