//! Fresh whole scalar replay from immutable proof, inventory and explicit selected policies.
use super::{
    FormulaExecutedScalarRecipe, FormulaRecipeExecutionAdmissionError,
    FormulaScalarExecutionContext, FormulaScalarRecipeError, FormulaScalarRecipePlan,
    FormulaScalarRecipePlanError,
};
use core::fmt;

/// Original replay stage failure; no successful statement or read prefix is published.
/// Borrowed proof/source/provider witnesses remain in their actual typed cause.
#[derive(Debug)]
pub enum FormulaScalarRecipeReplayError<'a> {
    /// Complete original proof and selected logical allowances failed admission.
    Admission(FormulaRecipeExecutionAdmissionError<'a, 'a>),
    /// Complete fresh capability/source preparation failed before values.
    Preparation(FormulaScalarRecipePlanError<'a>),
    /// Fresh execution failed at its original allocation, operand, binding or assertion.
    Execution(FormulaScalarRecipeError<'a>),
}
impl fmt::Display for FormulaScalarRecipeReplayError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Admission(error) => write!(f, "scalar recipe replay admission: {error}"),
            Self::Preparation(error) => write!(f, "scalar recipe replay preparation: {error}"),
            Self::Execution(error) => write!(f, "scalar recipe replay execution: {error}"),
        }
    }
}
impl std::error::Error for FormulaScalarRecipeReplayError<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Admission(error) => error.source(),
            Self::Preparation(error) => error.source(),
            Self::Execution(error) => error.source(),
        }
    }
}

impl<'p> FormulaScalarRecipePlan<'p> {
    /// Replay from statement one with newly admitted, prepared and reserved execution state.
    /// The original complete proof, canonical inventory, logical limits and tangent policy stay
    /// pinned. A plan retained by an accepted result may be replayed without consuming that result.
    /// Each call uses the supplied context; no prior binding, read slot or cache is copied.
    /// ```
    /// use sc_core::{recipe::*, value::CanonicalInputs};
    /// let inputs = CanonicalInputs::new(vec![], vec![])?;
    /// let recipe = FormulaRecipe::parse("let a:length=1 um/3\nlet b:length=a+a")?
    ///     .normalize_literals()?;
    /// let proof = recipe.check_kinds(FormulaNamespace::new([])?).expect("complete proof");
    /// let plan = proof.admit_execution(FormulaRecipeExecutionLimits::new(2, 6, 14, 2))
    ///     .expect("explicit allowance").prepare_scalar(&inputs).expect("whole preparation");
    /// let context = FormulaScalarExecutionContext::new(FormulaToleranceContext::empty(),
    ///     FormulaSizeContext::empty());
    /// let first = plan.replay(context).expect("fresh execution");
    /// let second = first.plan().replay(context).expect("fresh replay from accepted plan");
    /// assert_eq!(second.statements().len(), 2);
    /// assert_eq!(first.reads().count(), 2);
    /// assert_eq!(second.reads().count(), 2);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    /// # Errors
    /// Returns the original admission, complete preparation or execution failure. The borrowed
    /// plan, inputs and any earlier accepted result remain unchanged; no partial result escapes.
    /// Derived sources require the source-backed executor, as with ordinary scalar evaluation.
    pub fn replay<'a>(
        &self,
        context: FormulaScalarExecutionContext<'a>,
    ) -> Result<FormulaExecutedScalarRecipe<'a>, FormulaScalarRecipeReplayError<'a>>
    where
        'p: 'a,
    {
        let admission = self
            .admission()
            .proof()
            .admit_execution(self.admission().limits())
            .map_err(FormulaScalarRecipeReplayError::Admission)?;
        let plan = match self.tangent_limits() {
            Some(limits) => admission.prepare_scalar_with_tangent_limits(self.inputs(), limits),
            None => admission.prepare_scalar(self.inputs()),
        }
        .map_err(FormulaScalarRecipeReplayError::Preparation)?;
        plan.evaluate(context)
            .map_err(FormulaScalarRecipeReplayError::Execution)
    }
}
