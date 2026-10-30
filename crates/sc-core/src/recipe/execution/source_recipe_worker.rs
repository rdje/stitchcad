//! Atomic paired entry point;actual canonical ownership precedes complete runtime reservation.
use super::{
    recipe_runtime::{failure::SourceOperandFailure, RecipeRuntime, RecipeRuntimeError},
    source_runtime::SourceRuntime,
    FormulaExecutedScalarSourceRecipe, FormulaScalarExecutionContext as Context,
    FormulaScalarRecipePlan as Plan, FormulaScalarSourceGraphPlan,
    FormulaScalarSourceRecipeError as Error, FormulaScalarSourceRecipeErrorCause as Cause,
    FormulaScalarSourceRecipeSourceError,
};

impl<'p> Plan<'p> {
    /// Execute this whole scalar recipe with actual lazily demanded original source roots.
    /// Exact canonical owner precedes both full runtime reservations;all buffers precede values.
    /// Source roots remain unbound,actual lets store once,and prior lets expose only those bindings.
    /// No containing Design/currentness,whole allocator/CPU,missing-family or approval proof follows.
    /// ```
    /// use sc_core::recipe::*;
    /// fn finish<'a>(recipe:FormulaScalarRecipePlan<'a>,sources:FormulaScalarSourceGraphPlan<'a>,
    ///     context:FormulaScalarExecutionContext<'a>)
    ///     ->Result<FormulaExecutedScalarSourceRecipe<'a>,FormulaScalarSourceRecipeError<'a>> {
    ///     recipe.evaluate_with_sources(sources,context)
    /// }
    /// ```
    /// # Errors
    /// Returns original ownership/preparation/recipe/requester/supplier causes and both policies.
    /// Any failure drops the complete private source/outcome prefix;no accepted partial result.
    // Both actual policy sets/owner witnesses stay inline so reporting an allocator failure needs
    // no additional allocation. Semantic request context is already boxed by the private runtime.
    #[allow(clippy::result_large_err)]
    pub fn evaluate_with_sources<'a>(
        self,
        sources: FormulaScalarSourceGraphPlan<'a>,
        context: Context<'a>,
    ) -> Result<FormulaExecutedScalarSourceRecipe<'a>, Error<'a>>
    where
        'p: 'a,
    {
        let recipe: Plan<'a> = self;
        let proof = recipe.admission().proof();
        let inputs = recipe.inputs();
        let graph = sources.graph();
        let source_limits = sources.limits();
        let recipe_limits = recipe.admission().limits();
        let refuse = |cause| Error {
            recipe: proof,
            inputs,
            graph,
            source_limits,
            recipe_limits,
            cause,
        };
        let source_inputs = graph
            .dependencies()
            .checked()
            .normalized()
            .inputs()
            .inputs();
        if !core::ptr::eq(inputs, source_inputs) {
            return Err(refuse(Cause::ForeignInputs {
                sources: source_inputs,
            }));
        }
        let mut sources =
            SourceRuntime::new(sources, context).map_err(|cause| refuse(Cause::Sources(cause)))?;
        let recipe = RecipeRuntime::new(recipe, context)
            .map_err(|error| refuse(Cause::Recipe(error)))?
            .execute(Some(&mut sources))
            .map_err(|error| {
                refuse(match error {
                    RecipeRuntimeError::Recipe(error) => Cause::Recipe(error),
                    RecipeRuntimeError::Source {
                        statement,
                        part,
                        error,
                    } => match error {
                        SourceOperandFailure::Allocation { span, cause } => {
                            Cause::RequestedSourceAllocation {
                                statement,
                                part,
                                span,
                                cause,
                            }
                        }
                        SourceOperandFailure::Execution(request) => {
                            Cause::RequestedSource(FormulaScalarSourceRecipeSourceError {
                                statement,
                                part,
                                request,
                            })
                        }
                    },
                })
            })?;
        Ok(FormulaExecutedScalarSourceRecipe {
            recipe,
            sources: sources.finish(),
        })
    }
}
