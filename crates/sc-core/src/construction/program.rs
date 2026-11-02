//! Original mixed source order, before whole logical admission or normalization.
use super::{DraftingContext, DraftingContextSource, DraftingOperationInput};
use crate::{design::DesignMetadataHost, recipe::FormulaRecipeDocument};
use core::fmt;
mod admission;
mod limits;
mod name_inputs;
mod normalized;
mod views;
pub use admission::{
    AdmittedDraftingProgramInputs, DraftingProgramAdmissionError, DraftingProgramAdmissionRefusal,
};
pub use limits::{DraftingProgramLimits, DraftingProgramShape};
pub use name_inputs::{
    AdmittedDraftingProgramNameWork, BoundDraftingProgramInitials, BoundDraftingProgramOutputRoles,
    DraftingGeometryNameSource, DraftingInitialBinding, DraftingInitialBindingIssue,
    DraftingInitialError, DraftingInitialRefusal, DraftingOutputName, DraftingOutputPosition,
    DraftingOutputPositionError, DraftingOutputPositionRefusal, DraftingOutputRoleBindingIssue,
    DraftingOutputRoleError, DraftingOutputRoleMismatch, DraftingOutputRoleRefusal,
    DraftingProgramNameInputs, DraftingProgramNameLimits, DraftingProgramNameShape,
    DraftingProgramNameWorkError, DraftingProgramNameWorkRefusal, DraftingProgramOutputPositions,
};
pub use normalized::{
    DraftingProgramNormalizationError, DraftingProgramNormalizationRefusal,
    NormalizedDraftingOperation, NormalizedDraftingProgram,
};
pub use views::{DraftingProgramStepKind, DraftingProgramStepView};

/// One explicit original request in a mixed scalar/drafting sequence, without order acceptance.
#[derive(Clone, Copy, Debug)]
pub enum DraftingProgramStep<'a> {
    /// Requested one-based ordinal in the actual Root's whole original scalar recipe.
    /// Zero, missing, repeated or out-of-order requests are retained for later complete admission.
    /// This field is never the containing mixed-program position or an extracted source span.
    Scalar {
        /// Exact requested ordinal, including invalid input; no clipping or renumbering.
        ordinal: usize,
    },
    /// Actual original native declaration/request and all of its held input fields.
    Drafting(DraftingOperationInput<'a>),
}

/// Raw mixed source owner pinning an actual context, its Root recipe and every native input.
///
/// The original Vec is owned without sorting, deduplication, cloning, source extraction or parsing.
/// Its earlier allocation is separate from later admission. Construction performs no provider
/// callbacks or values and certifies no complete scalar coverage, resource allowance, namespace,
/// dependency, kind, current-output, geometry or executable program. Empty/invalid data survives.
/// The recipe can only come from the held actual context's Root; no replacement recipe is accepted.
///
/// Public callers cannot rewrite the retained sequence:
/// ```compile_fail,E0616
/// use sc_core::{construction::{DraftingProgramInputs, DraftingContextSource},
///     design::DesignMetadataHost};
/// fn replace<H:DesignMetadataHost,S:DraftingContextSource<H>>(
///     inputs:&mut DraftingProgramInputs<'_,H,S>) { inputs.steps.clear(); }
/// ```
/// The owner cannot outlive either its original context or borrowed native inputs:
/// ```compile_fail
/// use sc_core::{construction::{DraftingContext, DraftingContextSource, DraftingProgramInputs,
///     DraftingProgramStep}, design::DesignMetadataHost};
/// fn escape<'a,H:DesignMetadataHost,S:DraftingContextSource<H>>(
///     context:DraftingContext<'a,H,S>, steps:Vec<DraftingProgramStep<'a>>)
///     ->DraftingProgramInputs<'static,H,S> { DraftingProgramInputs::new(context,steps) }
/// ```
pub struct DraftingProgramInputs<'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> {
    context: DraftingContext<'a, H, S>,
    steps: Vec<DraftingProgramStep<'a>>,
}
impl<'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    DraftingProgramInputs<'a, H, S>
{
    /// Retain the actual context and complete authored sequence, without accepting or changing it.
    #[must_use]
    pub const fn new(
        context: DraftingContext<'a, H, S>,
        steps: Vec<DraftingProgramStep<'a>>,
    ) -> Self {
        Self { context, steps }
    }
    /// Original bound context handle; copying it copies no provider, model or geometry.
    #[must_use]
    pub const fn context(&self) -> DraftingContext<'a, H, S> {
        self.context
    }
    /// Actual Root's complete original recipe, with its original authored spans and identity.
    #[must_use]
    pub const fn recipe(&self) -> &'a FormulaRecipeDocument {
        self.context.scope().root().recipe()
    }
    /// Every original request in authored mixed order; scalar ordinals are separate fields.
    #[must_use]
    pub fn steps(&self) -> &[DraftingProgramStep<'a>] {
        &self.steps
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for DraftingProgramInputs<'_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramInputs")
            .field("step_count", &self.steps.len())
            .finish_non_exhaustive()
    }
}
