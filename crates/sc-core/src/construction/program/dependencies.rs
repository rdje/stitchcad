//! Actual source/order/graph provenance and original mixed dependencies before values.
mod owners;
pub use owners::{
    DraftingProgramDependencyInputs, DraftingProgramDependencyOwnerError,
    DraftingProgramDependencyOwnerIssue,
};
