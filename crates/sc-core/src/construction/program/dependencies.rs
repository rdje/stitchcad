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
mod native_geometry;
mod occurrences;
pub use native_geometry::DraftingProgramNativeGeometryUse;
mod owners;
pub use occurrences::{DraftingProgramDependencyRole, DraftingProgramNameDependency};
pub use owners::{
    DraftingProgramDependencyInputs, DraftingProgramDependencyOwnerError,
    DraftingProgramDependencyOwnerIssue,
};
