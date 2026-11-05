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

mod semantic_owners;
pub use semantic_owners::{
    DraftingProgramSemanticInputs, DraftingProgramSemanticOwnerError,
    DraftingProgramSemanticOwnerIssue,
};

mod semantic_population;
pub use semantic_population::{
    AdmittedDraftingProgramSemanticPopulation, DraftingProgramSemanticPopulationError,
    DraftingProgramSemanticPopulationLimits, DraftingProgramSemanticPopulationRefusal,
    DraftingProgramSemanticPopulationShape,
};

mod semantic_work;
pub use semantic_work::{
    AdmittedDraftingProgramSemanticWork, DraftingProgramSemanticChildContentLimits,
    DraftingProgramSemanticWorkArithmetic, DraftingProgramSemanticWorkBudget,
    DraftingProgramSemanticWorkError, DraftingProgramSemanticWorkLimits,
    DraftingProgramSemanticWorkRefusal, DraftingProgramSemanticWorkShape,
};
mod semantic_coverage;
pub use semantic_coverage::{
    CheckedDraftingProgramDirectSelections, DraftingProgramSelectionAssociation,
    DraftingProgramSelectionCoverageError, DraftingProgramSelectionCoverageRefusal,
    DraftingProgramSelectionRowKind,
};
mod root_input_scope;
pub use root_input_scope::{
    BoundDraftingProgramProfileHeaders, BoundDraftingProgramProfileParameters,
    BoundDraftingProgramRootInputMembers, BoundDraftingProgramRootLogicalSuppliers,
    BoundDraftingProgramRootNumericSuppliers, DraftingProgramProfileHeaderError,
    DraftingProgramProfileHeaderRefusal, DraftingProgramProfileHeadersResult,
    DraftingProgramProfileInputHeader, DraftingProgramProfileInputParameter,
    DraftingProgramProfileInputRequest, DraftingProgramProfileParameterError,
    DraftingProgramProfileParameterRefusal, DraftingProgramProfileParametersResult,
    DraftingProgramRootInputBinding, DraftingProgramRootInputMember,
    DraftingProgramRootInputMemberError, DraftingProgramRootInputMemberRefusal,
    DraftingProgramRootInputMembersResult, DraftingProgramRootInputScope,
    DraftingProgramRootInputScopeError, DraftingProgramRootInputScopeResult,
    DraftingProgramRootInputSource, DraftingProgramRootInputSupplier,
    DraftingProgramRootLogicalSupplier, DraftingProgramRootLogicalSupplierError,
    DraftingProgramRootLogicalSupplierRefusal, DraftingProgramRootLogicalSuppliersResult,
    DraftingProgramRootNumericSupplier, DraftingProgramRootNumericSupplierError,
    DraftingProgramRootNumericSupplierRefusal, DraftingProgramRootNumericSuppliersResult,
};
