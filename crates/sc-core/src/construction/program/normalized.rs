//! Complete actual child arenas; source ordinals remain in their original whole recipe.
use super::{
    AdmittedDraftingProgramInputs, DraftingProgramInputs, DraftingProgramLimits,
    DraftingProgramStep,
};
use crate::{
    construction::{
        CenterArcNormalizationError, CenterArcNormalizationLimits, DraftingContextSource,
        DraftingOperationInput, NormalizedCenterArcConstruction, NormalizedPointConstruction,
        NormalizedSlashSpreadOpening, PointConstructionNormalizationError,
        PointConstructionNormalizationLimits, SlashSpreadConstruction,
        SlashSpreadNormalizationError, SlashSpreadNormalizationLimits,
    },
    design::{DesignMetadataHost, DesignNormalizedFormulaRecipe, DesignNormalizedFormulaSources},
    recipe::{
        FormulaRecipeDocumentError, FormulaSourceNormalizationError,
        FormulaSourceNormalizationLimits,
    },
};
use core::fmt;
use std::collections::TryReserveError;
mod resources;

/// Actual normalized argument owners, or an original family with no scalar argument documents.
/// No variant certifies namespace, kind, source/current-output dependency or geometry acceptance.
pub enum NormalizedDraftingOperation<'a> {
    /// Actual point declaration and both original-source-borrowed argument arenas.
    Point(NormalizedPointConstruction<'a>),
    /// Actual center arc and all three original-source-borrowed argument arenas.
    CenterArc(NormalizedCenterArcConstruction<'a>),
    /// Actual complete slash/spread declaration and its original normalized opening component.
    SlashSpread {
        /// Original spatial declaration, not reconstructed from the opening mode.
        input: &'a SlashSpreadConstruction,
        /// Actual opening factory result borrowing that declaration's document.
        opening: NormalizedSlashSpreadOpening<'a>,
    },
    /// Original input without scalar argument documents; no other local admission is implied.
    Other(DraftingOperationInput<'a>),
}
impl<'a> NormalizedDraftingOperation<'a> {
    /// Exact original native input, with no copied document, role, operation id or spatial target.
    #[must_use]
    pub fn input(&self) -> DraftingOperationInput<'a> {
        match self {
            Self::Point(p) => DraftingOperationInput::ConstructPoint(p.point()),
            Self::CenterArc(a) => DraftingOperationInput::DrawArc(a.arc()),
            Self::SlashSpread { input, .. } => DraftingOperationInput::SlashSpread(input),
            Self::Other(input) => *input,
        }
    }
}
impl fmt::Debug for NormalizedDraftingOperation<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NormalizedDraftingOperation")
            .field("family", &self.input().family())
            .finish_non_exhaustive()
    }
}

/// Original outer allocation or actual child-factory refusal, without replacing typed causes.
/// Allocation errors, including those held by the original source factory, remain inline.
#[derive(Debug)]
pub enum DraftingProgramNormalizationRefusal<'a> {
    /// Original fallible complete mixed-slot reservation failure, before any child factory.
    Allocation(TryReserveError),
    /// Original complete Root formula-source factory error, including original allocator causes.
    Sources(FormulaSourceNormalizationError<'a>),
    /// Original whole scalar recipe factory error with its original source ordinal/spans.
    Recipe(FormulaRecipeDocumentError),
    /// Original point factory error at the genuine one-based mixed-program position.
    Point {
        /// Original containing mixed position, never a scalar source ordinal.
        position: usize,
        /// Original point/argument owner, selected derived child limits and typed cause.
        cause: PointConstructionNormalizationError<'a>,
    },
    /// Original center-arc factory error at its genuine mixed position.
    CenterArc {
        /// Original containing mixed position.
        position: usize,
        /// Original three-argument factory refusal.
        cause: CenterArcNormalizationError<'a>,
    },
    /// Original opening-component factory error at its genuine mixed position.
    SlashSpread {
        /// Original containing mixed position.
        position: usize,
        /// Original mode/document owner and typed factory refusal.
        cause: SlashSpreadNormalizationError<'a>,
    },
}
/// Failed complete normalization, retaining actual complete inputs/policy and original factory error.
pub struct DraftingProgramNormalizationError<
    'p,
    'a,
    H: DesignMetadataHost,
    S: DraftingContextSource<H> + ?Sized,
> {
    inputs: &'p DraftingProgramInputs<'a, H, S>,
    limits: &'p DraftingProgramLimits,
    refusal: DraftingProgramNormalizationRefusal<'a>,
}
impl<'p, 'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    DraftingProgramNormalizationError<'p, 'a, H, S>
{
    /// Actual original complete mixed input owner; no successful normalized prefix is exposed.
    #[must_use]
    pub const fn inputs(&self) -> &'p DraftingProgramInputs<'a, H, S> {
        self.inputs
    }
    /// Actual original complete explicit policy used by admission and every derived child policy.
    #[must_use]
    pub const fn limits(&self) -> &'p DraftingProgramLimits {
        self.limits
    }
    /// Original typed factory/allocation cause, retaining its actual original source owners.
    #[must_use]
    pub const fn refusal(&self) -> &DraftingProgramNormalizationRefusal<'a> {
        &self.refusal
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for DraftingProgramNormalizationError<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Source/body diagnostics remain available by deliberate typed access, not implicit logging.
        f.debug_struct("DraftingProgramNormalizationError")
            .finish_non_exhaustive()
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Display
    for DraftingProgramNormalizationError<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("complete drafting normalization refused")
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> std::error::Error
    for DraftingProgramNormalizationError<'_, '_, H, S>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use DraftingProgramNormalizationRefusal as R;
        match &self.refusal {
            R::Allocation(cause) => Some(cause),
            R::Recipe(cause) => Some(cause),
            R::Sources(cause) => cause.source(),
            R::Point { cause, .. } => cause.source(),
            R::CenterArc { cause, .. } => cause.source(),
            R::SlashSpread { cause, .. } => cause.source(),
        }
    }
}

/// Complete actual source, whole-recipe and native argument arena owner after whole input admission.
/// Arenas borrow original source documents; there are no self-references or fabricated spans.
/// This grants no complete name/kind/dependency/current-output, geometry, value or execution proof.
/// ```compile_fail,E0616
/// use sc_core::{construction::{NormalizedDraftingProgram,DraftingContextSource},design::DesignMetadataHost};
/// fn replace<H:DesignMetadataHost,S:DraftingContextSource<H>>(
///     program:&mut NormalizedDraftingProgram<'_, '_,H,S>) { program.operations.clear(); }
/// ```
pub struct NormalizedDraftingProgram<
    'p,
    'a,
    H: DesignMetadataHost,
    S: DraftingContextSource<H> + ?Sized,
> {
    admitted: AdmittedDraftingProgramInputs<'p, 'a, H, S>,
    sources: DesignNormalizedFormulaSources<'a, H>,
    recipe: DesignNormalizedFormulaRecipe<'a, H>,
    operations: Vec<Option<NormalizedDraftingOperation<'a>>>,
}
impl<'p, 'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    NormalizedDraftingProgram<'p, 'a, H, S>
{
    /// Actual complete admitted source owner and explicit whole policy.
    #[must_use]
    pub const fn admitted(&self) -> AdmittedDraftingProgramInputs<'p, 'a, H, S> {
        self.admitted
    }
    /// Complete actual original Root source arenas, including every unused source body.
    #[must_use]
    pub const fn sources(&self) -> &DesignNormalizedFormulaSources<'a, H> {
        &self.sources
    }
    /// Complete actual whole scalar recipe with original source ordinals and spans.
    #[must_use]
    pub const fn recipe(&self) -> &DesignNormalizedFormulaRecipe<'a, H> {
        &self.recipe
    }
    /// Complete mixed-position-aligned native slots: None for scalar and semantic requests.
    /// Both kinds retain their actual Root recipe owners through the separate typed step views.
    #[must_use]
    pub fn operations(&self) -> &[Option<NormalizedDraftingOperation<'a>>] {
        &self.operations
    }
}
impl<H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized> fmt::Debug
    for NormalizedDraftingProgram<'_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NormalizedDraftingProgram")
            .field("shape", &self.admitted.shape())
            .finish_non_exhaustive()
    }
}
impl<'p, 'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    AdmittedDraftingProgramInputs<'p, 'a, H, S>
{
    /// Reserve the complete mixed slot vector,then normalize all Root sources,the whole original
    /// scalar recipe,and native arguments in original mixed order using their existing factories.
    /// Derived child ceilings reuse the explicit admitted whole policy; actual aggregate shape was
    /// checked already,so repeated uses cannot multiply the granted whole allowance. No values,
    /// namespace,geometry/provider lookup,source extraction or accepted normalized prefix occurs.
    /// # Errors
    /// Original complete reservation or actual source/recipe/argument factory error and cause.
    /// Failed allocation is propagated inline without allocating a new diagnostic wrapper.
    pub fn normalized(
        self,
    ) -> Result<
        NormalizedDraftingProgram<'p, 'a, H, S>,
        DraftingProgramNormalizationError<'p, 'a, H, S>,
    > {
        use DraftingProgramNormalizationRefusal as R;
        let inputs = self.inputs();
        let limits = self.limits();
        let refuse = |refusal| DraftingProgramNormalizationError {
            inputs,
            limits,
            refusal,
        };
        let mut operations = Vec::new();
        operations
            .try_reserve_exact(inputs.steps().len())
            .map_err(|e| refuse(R::Allocation(e)))?;
        let scope = inputs.context().scope();
        let sources = scope
            .normalize_sources(FormulaSourceNormalizationLimits::new(
                limits.expressions(),
                limits.nodes(),
                limits.retained_records(),
            ))
            .map_err(|e| refuse(R::Sources(e)))?;
        let recipe = scope.normalize_recipe().map_err(|e| refuse(R::Recipe(e)))?;
        for (index, step) in inputs.steps().iter().copied().enumerate() {
            // A Vec of nonzero-size mixed slots has len <= isize::MAX; one-based position fits.
            let position = index + 1;
            let operation = match step {
                DraftingProgramStep::Scalar { .. } | DraftingProgramStep::Semantic { .. } => None,
                DraftingProgramStep::Drafting(input) => Some(match input {
                    DraftingOperationInput::ConstructPoint(p) => {
                        NormalizedDraftingOperation::Point(
                            p.normalized(PointConstructionNormalizationLimits::new(
                                limits.nodes(),
                                limits.retained_records(),
                            ))
                            .map_err(|cause| refuse(R::Point { position, cause }))?,
                        )
                    }
                    DraftingOperationInput::DrawArc(a) => NormalizedDraftingOperation::CenterArc(
                        a.normalized(CenterArcNormalizationLimits::new(
                            limits.nodes(),
                            limits.retained_records(),
                        ))
                        .map_err(|cause| refuse(R::CenterArc { position, cause }))?,
                    ),
                    DraftingOperationInput::SlashSpread(s) => {
                        NormalizedDraftingOperation::SlashSpread {
                            input: s,
                            opening: s
                                .opening()
                                .normalized(SlashSpreadNormalizationLimits::new(
                                    limits.nodes(),
                                    limits.retained_records(),
                                ))
                                .map_err(|cause| refuse(R::SlashSpread { position, cause }))?,
                        }
                    }
                    DraftingOperationInput::DrawLine(_)
                    | DraftingOperationInput::DrawBezier(_)
                    | DraftingOperationInput::OffsetEdge(_)
                    | DraftingOperationInput::CloseDart { .. }
                    | DraftingOperationInput::AddNotch { .. }
                    | DraftingOperationInput::PlaceGrainline(_)
                    | DraftingOperationInput::WalkSeam(_)
                    | DraftingOperationInput::TrueSeam(_)
                    | DraftingOperationInput::MirrorPiece(_)
                    | DraftingOperationInput::JoinPieces(_) => {
                        NormalizedDraftingOperation::Other(input)
                    }
                }),
            };
            operations.push(operation);
        }
        Ok(NormalizedDraftingProgram {
            admitted: self,
            sources,
            recipe,
            operations,
        })
    }
}
