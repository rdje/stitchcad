//! Same actual-root proof joins and complete native source/recipe scalar plan preparation.
use super::{DesignCheckedFormulaRecipe, DesignCheckedFormulaSources, DesignFormulaScope};
use crate::recipe::{
    FormulaAcyclicSourceDependencies, FormulaRecipeExecutionAdmissionError,
    FormulaRecipeExecutionLimits, FormulaScalarRecipePlan, FormulaScalarRecipePlanError,
    FormulaScalarSourceGraphPlan, FormulaScalarSourceGraphPlanError,
    FormulaSourceGraphExecutionLimits,
};
use core::fmt;

/// Two actual checked owners came from different Design objects,even if all content compares equal.
pub struct DesignFormulaPlanInputError<'a, H> {
    sources: &'a DesignCheckedFormulaSources<'a, H>,
    recipe: &'a DesignCheckedFormulaRecipe<'a, H>,
}
impl<'a, H> DesignFormulaPlanInputError<'a, H> {
    /// Actual source-proof owner that selected the expected containing root.
    #[must_use]
    pub const fn sources(&self) -> &'a DesignCheckedFormulaSources<'a, H> {
        self.sources
    }
    /// Actual rejected ordered recipe-proof owner,with its original containing root.
    #[must_use]
    pub const fn recipe(&self) -> &'a DesignCheckedFormulaRecipe<'a, H> {
        self.recipe
    }
}
impl<H> fmt::Debug for DesignFormulaPlanInputError<'_, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DesignFormulaPlanInputError")
            .field("sources", &self.sources)
            .field("recipe", &self.recipe)
            .finish_non_exhaustive()
    }
}
impl<H> fmt::Display for DesignFormulaPlanInputError<'_, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            "formula source and recipe proofs belong to different actual Design input roots",
        )
    }
}
impl<H> std::error::Error for DesignFormulaPlanInputError<'_, H> {}

/// Actual same-root source/recipe proofs and selected complete native scalar execution allowances.
/// This is a private provenance join,not quota admission or an executed result. Dependency/cycle
/// construction remains the native factory's job;plan preparation verifies exact proof provenance.
/// ```compile_fail
/// fn swap<H>(inputs:&mut sc_core::design::DesignFormulaPlanInputs<'_,H>,
///     other:&sc_core::design::DesignCheckedFormulaRecipe<'_,H>) {inputs.recipe=other;}
/// ```
pub struct DesignFormulaPlanInputs<'a, H> {
    sources: &'a DesignCheckedFormulaSources<'a, H>,
    recipe: &'a DesignCheckedFormulaRecipe<'a, H>,
    source_limits: FormulaSourceGraphExecutionLimits,
    recipe_limits: FormulaRecipeExecutionLimits,
}
impl<'a, H> DesignFormulaPlanInputs<'a, H> {
    /// Actual checked source owner,from which native dependencies and cycle proofs must be built.
    #[must_use]
    pub const fn sources(&self) -> &'a DesignCheckedFormulaSources<'a, H> {
        self.sources
    }
    /// Actual checked recipe owner;no supplied proof can replace it after joining.
    #[must_use]
    pub const fn recipe(&self) -> &'a DesignCheckedFormulaRecipe<'a, H> {
        self.recipe
    }
    /// Actual containing input scope selected by the source owner;the recipe has this same root.
    #[must_use]
    pub const fn scope(&self) -> DesignFormulaScope<'a, H> {
        self.sources.scope()
    }
    /// Explicit original complete source execution allowances.
    #[must_use]
    pub const fn source_limits(&self) -> FormulaSourceGraphExecutionLimits {
        self.source_limits
    }
    /// Explicit original whole recipe execution allowances.
    #[must_use]
    pub const fn recipe_limits(&self) -> FormulaRecipeExecutionLimits {
        self.recipe_limits
    }
    /// Verify exact source-proof provenance,admit the whole recipe,then prepare both native plans.
    /// The graph must come from this exact checked source proof,including when another proof uses
    /// the same root/arenas/content. Original native quota/capability/canonical checks remain intact.
    /// No context/provider/value,let storage or execution order is read/created. Runtime reservation,
    /// namespace/checker/kernel/allocator/CPU and complete Design approval remain separate.
    /// ```compile_fail
    /// fn escape<'a,H>(inputs:sc_core::design::DesignFormulaPlanInputs<'a,H>,
    ///     graph:&'a sc_core::recipe::FormulaAcyclicSourceDependencies<'a>)
    ///     ->sc_core::design::DesignFormulaScalarPlan<'a,H> {inputs.prepare_scalar(graph).unwrap()}
    /// ```
    /// # Errors
    /// Foreign graph precedes recipe admission,source preparation,then recipe preparation failures.
    /// Original errors and both selected policies remain available;no accepted partial plan escapes.
    pub fn prepare_scalar<'p>(
        &'p self,
        graph: &'p FormulaAcyclicSourceDependencies<'p>,
    ) -> Result<DesignFormulaScalarPlan<'p, H>, DesignFormulaScalarPlanError<'p, H>>
    where
        'a: 'p,
    {
        let refuse = |refusal| DesignFormulaScalarPlanError {
            inputs: self,
            refusal,
        };
        if !core::ptr::eq(graph.dependencies().checked(), self.sources.proof()) {
            return Err(refuse(DesignFormulaScalarPlanRefusal::ForeignGraph {
                graph,
            }));
        }
        let admission = self
            .recipe
            .proof()
            .admit_execution(self.recipe_limits)
            .map_err(|error| refuse(DesignFormulaScalarPlanRefusal::RecipeAdmission(error)))?;
        let sources = graph
            .prepare_scalar_sources(self.source_limits)
            .map_err(|error| refuse(DesignFormulaScalarPlanRefusal::Sources(error)))?;
        let recipe = admission
            .prepare_scalar(self.scope().source_inputs().inputs())
            .map_err(|error| refuse(DesignFormulaScalarPlanRefusal::Recipe(error)))?;
        Ok(DesignFormulaScalarPlan {
            inputs: self,
            sources,
            recipe,
        })
    }
}
impl<H> fmt::Debug for DesignFormulaPlanInputs<'_, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DesignFormulaPlanInputs")
            .field("sources", &self.sources)
            .field("recipe", &self.recipe)
            .field("source_limits", &self.source_limits)
            .field("recipe_limits", &self.recipe_limits)
            .finish_non_exhaustive()
    }
}
impl<'r, H> DesignCheckedFormulaSources<'r, H> {
    /// Join actual same-root checked owners and select both native execution policies explicitly.
    /// Equality of ids,revisions,text or model content cannot substitute for actual root identity.
    /// No quotas,dependency/cycle proof,allocation or numerical evaluation follows this join alone.
    /// ```
    /// use sc_core::{design::{DesignCheckedFormulaSources,DesignCheckedFormulaRecipe},recipe::{
    ///     FormulaSourceGraphExecutionLimits,FormulaRecipeExecutionLimits,
    ///     FormulaSourceDependencyLimits,FormulaSourceCycleLimits}};
    /// fn prepare<'a,H>(sources:&'a DesignCheckedFormulaSources<'a,H>,
    ///     recipe:&'a DesignCheckedFormulaRecipe<'a,H>,source_limits:FormulaSourceGraphExecutionLimits,
    ///     recipe_limits:FormulaRecipeExecutionLimits,edges:FormulaSourceDependencyLimits,
    ///     cycles:FormulaSourceCycleLimits) {
    ///     let inputs=sources.plan_inputs(recipe,source_limits,recipe_limits).unwrap();
    ///     let dependencies=inputs.sources().proof().prepare_dependencies(edges).unwrap();
    ///     let graph=dependencies.admit_acyclic(cycles).unwrap();
    ///     let plan=inputs.prepare_scalar(&graph).unwrap();
    ///     assert!(std::ptr::eq(plan.recipe().admission().proof(),recipe.proof()));
    ///     assert!(std::ptr::eq(plan.scope().root(),sources.scope().root()));
    /// }
    /// ```
    /// # Errors
    /// Returns both actual checked owners if their containing Design input root references differ.
    pub fn plan_inputs<'a>(
        &'a self,
        recipe: &'a DesignCheckedFormulaRecipe<'a, H>,
        source_limits: FormulaSourceGraphExecutionLimits,
        recipe_limits: FormulaRecipeExecutionLimits,
    ) -> Result<DesignFormulaPlanInputs<'a, H>, DesignFormulaPlanInputError<'a, H>>
    where
        'r: 'a,
    {
        if !core::ptr::eq(self.scope().root(), recipe.scope().root()) {
            return Err(DesignFormulaPlanInputError {
                sources: self,
                recipe,
            });
        }
        Ok(DesignFormulaPlanInputs {
            sources: self,
            recipe,
            source_limits,
            recipe_limits,
        })
    }
}

/// Original first plan-stage failure;retained native allocation errors stay inline.
#[derive(Debug)]
pub enum DesignFormulaScalarPlanRefusal<'a> {
    /// This graph does not originate at the exact source proof selected by the input join.
    ForeignGraph {
        /// Actual supplied graph,including its original dependency/source proof owner.
        graph: &'a FormulaAcyclicSourceDependencies<'a>,
    },
    /// Original complete whole-recipe quota refusal,before source-plan preparation.
    RecipeAdmission(FormulaRecipeExecutionAdmissionError<'a, 'a>),
    /// Original complete source population/capability/allocation/preparation cause.
    Sources(FormulaScalarSourceGraphPlanError<'a>),
    /// Original whole-recipe capability/canonical/allocation preparation cause.
    Recipe(FormulaScalarRecipePlanError<'a>),
}
/// First whole plan failure retains the actual joined owners and both explicit selected policies.
/// No accepted source-only plan,prefix,value or new allocation-error box is returned.
pub struct DesignFormulaScalarPlanError<'a, H> {
    inputs: &'a DesignFormulaPlanInputs<'a, H>,
    refusal: DesignFormulaScalarPlanRefusal<'a>,
}
impl<'a, H> DesignFormulaScalarPlanError<'a, H> {
    /// Actual same-root proof join and original source/recipe execution allowances.
    #[must_use]
    pub const fn inputs(&self) -> &'a DesignFormulaPlanInputs<'a, H> {
        self.inputs
    }
    /// Original first provenance/admission/source/recipe cause with its actual factory witnesses.
    #[must_use]
    pub const fn refusal(&self) -> &DesignFormulaScalarPlanRefusal<'a> {
        &self.refusal
    }
}
impl<H> fmt::Debug for DesignFormulaScalarPlanError<'_, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DesignFormulaScalarPlanError")
            .field("inputs", &self.inputs)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl<H> fmt::Display for DesignFormulaScalarPlanError<'_, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.refusal {
            DesignFormulaScalarPlanRefusal::ForeignGraph { .. } => {
                f.write_str("formula graph belongs to another actual checked source proof")
            }
            DesignFormulaScalarPlanRefusal::RecipeAdmission(error) => fmt::Display::fmt(error, f),
            DesignFormulaScalarPlanRefusal::Sources(error) => fmt::Display::fmt(error, f),
            DesignFormulaScalarPlanRefusal::Recipe(error) => fmt::Display::fmt(error, f),
        }
    }
}
impl<H> std::error::Error for DesignFormulaScalarPlanError<'_, H> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.refusal {
            DesignFormulaScalarPlanRefusal::ForeignGraph { .. } => None,
            DesignFormulaScalarPlanRefusal::RecipeAdmission(error) => {
                std::error::Error::source(error)
            }
            DesignFormulaScalarPlanRefusal::Sources(error) => std::error::Error::source(error),
            DesignFormulaScalarPlanRefusal::Recipe(error) => std::error::Error::source(error),
        }
    }
}
/// Complete actual-root native source graph and ordered recipe scalar plans,without execution.
/// Plans and their original proof/resource policies are immutable;no standalone partial plan escapes.
/// ```compile_fail
/// fn replace<H>(plan:&mut sc_core::design::DesignFormulaScalarPlan<'_,H>,
///     other:sc_core::recipe::FormulaScalarRecipePlan<'_>) {plan.recipe=other;}
/// ```
pub struct DesignFormulaScalarPlan<'a, H> {
    inputs: &'a DesignFormulaPlanInputs<'a, H>,
    sources: FormulaScalarSourceGraphPlan<'a>,
    recipe: FormulaScalarRecipePlan<'a>,
}
impl<'a, H> DesignFormulaScalarPlan<'a, H> {
    /// Actual paired proof owners and explicit source/recipe allowances.
    #[must_use]
    pub const fn inputs(&self) -> &'a DesignFormulaPlanInputs<'a, H> {
        self.inputs
    }
    /// Actual original containing Design input scope.
    #[must_use]
    pub const fn scope(&self) -> DesignFormulaScope<'a, H> {
        self.inputs.scope()
    }
    /// Complete actual source plan,including original graph,all source plans and selected quotas.
    #[must_use]
    pub const fn sources(&self) -> &FormulaScalarSourceGraphPlan<'a> {
        &self.sources
    }
    /// Complete actual ordered recipe plan over this root's canonical inventory and genuine lets.
    #[must_use]
    pub const fn recipe(&self) -> &FormulaScalarRecipePlan<'a> {
        &self.recipe
    }
}
impl<H> fmt::Debug for DesignFormulaScalarPlan<'_, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DesignFormulaScalarPlan")
            .field("inputs", &self.inputs)
            .field("sources", &self.sources)
            .field("recipe", &self.recipe)
            .finish_non_exhaustive()
    }
}
