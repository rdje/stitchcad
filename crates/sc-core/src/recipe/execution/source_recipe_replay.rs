//! Fresh paired recipe/source replay with no retained values, read slots or completed cache.
use super::{
    FormulaExecutedScalarSourceRecipe, FormulaRecipeExecutionAdmissionError,
    FormulaScalarExecutionContext, FormulaScalarRecipePlan, FormulaScalarRecipePlanError,
    FormulaScalarSourceGraphPlan, FormulaScalarSourceGraphPlanError,
    FormulaScalarSourceRecipeError, FormulaScalarSourceRecipeErrorCause,
};
use core::fmt;

/// Original fresh paired-replay stage failure; no partial recipe or source result is accepted.
#[derive(Debug)]
pub enum FormulaScalarSourceRecipeReplayError<'a> {
    /// Complete recipe proof and original logical allowances failed admission.
    RecipeAdmission(FormulaRecipeExecutionAdmissionError<'a, 'a>),
    /// Fresh complete recipe capability/source preparation failed before values.
    RecipePreparation(FormulaScalarRecipePlanError<'a>),
    /// Fresh complete source graph preparation failed before values.
    SourcePreparation(FormulaScalarSourceGraphPlanError<'a>),
    /// Original early canonical ownership or subsequent actual paired runtime refusal.
    Execution(FormulaScalarSourceRecipeError<'a>),
}
impl fmt::Display for FormulaScalarSourceRecipeReplayError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RecipeAdmission(error) => write!(f, "paired replay recipe admission: {error}"),
            Self::RecipePreparation(error) => {
                write!(f, "paired replay recipe preparation: {error}")
            }
            Self::SourcePreparation(error) => {
                write!(f, "paired replay source preparation: {error}")
            }
            Self::Execution(error) => write!(f, "paired replay execution: {error}"),
        }
    }
}
impl std::error::Error for FormulaScalarSourceRecipeReplayError<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::RecipeAdmission(error) => error.source(),
            Self::RecipePreparation(error) => error.source(),
            Self::SourcePreparation(error) => error.source(),
            Self::Execution(error) => error.source(),
        }
    }
}

impl<'p> FormulaScalarRecipePlan<'p> {
    /// Replay a complete prepared recipe/source pair from statement one and an empty source cache.
    /// Borrows both plans,including those retained by an accepted paired result. Exact canonical
    /// owner is checked before either fresh preparation. Complete recipe admission/preparation then
    /// complete source preparation preserve both original proofs, quotas and tangent policies.
    /// Each call uses its supplied context for both runtimes; no prior result or read slot is copied.
    /// ```
    /// use sc_core::recipe::*;
    /// fn replay<'a>(prior:&FormulaExecutedScalarSourceRecipe<'a>,
    ///     context:FormulaScalarExecutionContext<'a>)
    ///     ->Result<FormulaExecutedScalarSourceRecipe<'a>,FormulaScalarSourceRecipeReplayError<'a>> {
    ///     prior.recipe_plan().replay_with_sources(prior.source_plan(),context)
    /// }
    /// ```
    /// # Errors
    /// Original ownership, recipe admission/preparation, source preparation or paired execution
    /// cause survives. Failure leaves both plans and any earlier accepted result unchanged, and
    /// exposes no accepted recipe/source prefix. No containing Design/currentness proof follows.
    // Original inline allocator causes and both actual policy/owner sets require no error box.
    #[allow(clippy::result_large_err)]
    pub fn replay_with_sources<'a>(
        &self,
        sources: &FormulaScalarSourceGraphPlan<'a>,
        context: FormulaScalarExecutionContext<'a>,
    ) -> Result<FormulaExecutedScalarSourceRecipe<'a>, FormulaScalarSourceRecipeReplayError<'a>>
    where
        'p: 'a,
    {
        use FormulaScalarSourceRecipeReplayError as Error;
        let graph = sources.graph();
        let source_inputs = graph
            .dependencies()
            .checked()
            .normalized()
            .inputs()
            .inputs();
        if !core::ptr::eq(self.inputs(), source_inputs) {
            return Err(Error::Execution(FormulaScalarSourceRecipeError {
                recipe: self.admission().proof(),
                inputs: self.inputs(),
                graph,
                source_limits: sources.limits(),
                source_tangent_limits: sources.tangent_limits(),
                recipe_limits: self.admission().limits(),
                recipe_tangent_limits: self.tangent_limits(),
                cause: FormulaScalarSourceRecipeErrorCause::ForeignInputs {
                    sources: source_inputs,
                },
            }));
        }
        let admission = self
            .admission()
            .proof()
            .admit_execution(self.admission().limits())
            .map_err(Error::RecipeAdmission)?;
        let recipe = match self.tangent_limits() {
            Some(limits) => admission.prepare_scalar_with_tangent_limits(self.inputs(), limits),
            None => admission.prepare_scalar(self.inputs()),
        }
        .map_err(Error::RecipePreparation)?;
        let sources = match sources.tangent_limits() {
            Some(limits) => {
                graph.prepare_scalar_sources_with_tangent_limits(sources.limits(), limits)
            }
            None => graph.prepare_scalar_sources(sources.limits()),
        }
        .map_err(Error::SourcePreparation)?;
        recipe
            .evaluate_with_sources(sources, context)
            .map_err(Error::Execution)
    }
}
