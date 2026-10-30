//! Original standalone recipe adapter over the complete shared private recipe runtime.
use super::{
    recipe_runtime::RecipeRuntime, FormulaExecutedScalarRecipe,
    FormulaScalarExecutionContext as Context, FormulaScalarRecipeError as Error,
    FormulaScalarRecipePlan as Plan,
};

impl<'p> Plan<'p> {
    /// Execute the complete admitted available scalar recipe against its actual immutable sources.
    /// The private successful prefix cannot be seeded, substituted or advanced by the caller.
    /// One D source/read vector, N task/value buffers and S outcomes are reserved before values.
    /// ```
    /// use sc_core::{recipe::*, value::CanonicalInputs};
    /// let inputs = CanonicalInputs::new(vec![], vec![])?;
    /// let recipe = FormulaRecipe::parse("let a:length=1 mm/3\nlet b:length=a+a\nassert check:eps_num=b==666 um")?
    ///     .normalize_literals()?;
    /// let proof = recipe.check_kinds(FormulaNamespace::new([])?).expect("whole static proof");
    /// let result = proof.admit_execution(FormulaRecipeExecutionLimits::new(3, 8, 20, 3))
    ///     .expect("exact allowance").prepare_scalar(&inputs).expect("whole sources")
    ///     .evaluate(FormulaScalarExecutionContext::new(FormulaToleranceContext::empty(),
    ///         FormulaSizeContext::empty())).expect("actual statements");
    /// assert_eq!(result.statements().len(), 3);
    /// assert_eq!(result.reads().count(), 4); // a twice, b and the real assertion header
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    /// # Errors
    /// Actual allocator/operand/storage/tolerance/comparison/assertion failure returns no result or
    /// successful statement prefix. Immutable canonical sources remain unchanged. Missing Derived
    /// evaluation retains its original requirement; no value, geometry or language error is invented.
    pub fn evaluate<'a>(
        self,
        context: Context<'a>,
    ) -> Result<FormulaExecutedScalarRecipe<'a>, Error<'a>>
    where
        'p: 'a,
    {
        let plan: Plan<'a> = self;
        let completed = RecipeRuntime::new(plan, context)?.execute()?;
        Ok(FormulaExecutedScalarRecipe {
            plan: completed.plan,
            statements: completed.statements,
        })
    }
}
