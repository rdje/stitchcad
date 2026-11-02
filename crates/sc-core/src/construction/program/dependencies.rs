//! Actual source/order/graph provenance and original mixed dependencies before values.
mod canonical;
pub use canonical::{
    BoundDraftingProgramCanonicalInputs, DraftingProgramCanonicalBinding,
    DraftingProgramCanonicalError, DraftingProgramCanonicalJoinFailure,
    DraftingProgramCanonicalLimits, DraftingProgramCanonicalOccurrence,
    DraftingProgramCanonicalRefusal, DraftingProgramCanonicalShape, DraftingProgramDerivedBinding,
};
mod occurrences;
mod owners;
pub use occurrences::{DraftingProgramDependencyRole, DraftingProgramNameDependency};
pub use owners::{
    DraftingProgramDependencyInputs, DraftingProgramDependencyOwnerError,
    DraftingProgramDependencyOwnerIssue,
};
