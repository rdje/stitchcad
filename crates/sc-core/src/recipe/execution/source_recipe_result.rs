//! Complete actual paired outcomes and earned source/recipe read views,without a fake source entry.
use super::{
    recipe_result::{read_records, RecipeReadData, RecipeReadSource},
    recipe_runtime::CompletedRecipe,
    source_read::source_index,
    source_result::completed_source_view,
    source_runtime::CompletedSources,
    FormulaExecutedDerivedSourceRead, FormulaExecutedScalarRecipeStatement,
    FormulaExecutedScalarSource, FormulaScalarExecutionContext, FormulaScalarRecipePlan,
    FormulaScalarRecipeReadKind, FormulaScalarSourceGraphPlan,
};
use crate::{
    ontology::EntityId,
    recipe::{FormulaCheckedNode, FormulaEvaluatedScalar, FormulaRecipeNameDependency},
};
use core::fmt;

/// Actual successful direct/prior-let or earned unbound Derived recipe read.
#[derive(Clone, Copy, Debug)]
pub enum FormulaScalarSourceRecipeReadKind<'a> {
    /// Existing actual canonical/reserved or real once-stored prior-let witness.
    Recipe(FormulaScalarRecipeReadKind<'a>),
    /// Original canonical join and successfully executed supplier source proof.
    Derived(FormulaExecutedDerivedSourceRead<'a>),
}
/// One actually executed original recipe dependency;fields/constructor are private.
/// ```compile_fail
/// fn seed(read:&mut sc_core::recipe::FormulaScalarSourceRecipeRead<'_>) {read.data.value=todo!();}
/// ```
#[derive(Clone, Copy, Debug)]
pub struct FormulaScalarSourceRecipeRead<'a> {
    dependency: FormulaRecipeNameDependency<'a>,
    data: RecipeReadData<'a>,
}
impl<'a> FormulaScalarSourceRecipeRead<'a> {
    /// Original statement ordinal/operand role/span and actual declaration metadata.
    #[must_use]
    pub const fn dependency(self) -> FormulaRecipeNameDependency<'a> {
        self.dependency
    }
    /// Actual checked operand name node;an original assertion header has no fabricated AST node.
    #[must_use]
    pub const fn node(self) -> Option<FormulaCheckedNode<'a, 'a>> {
        self.data.node
    }
    /// Original successful direct/prior-let/Derived witness.
    #[must_use]
    pub const fn source(self) -> FormulaScalarSourceRecipeReadKind<'a> {
        match self.data.source {
            RecipeReadSource::Original(source) => FormulaScalarSourceRecipeReadKind::Recipe(source),
            RecipeReadSource::Derived(source) => FormulaScalarSourceRecipeReadKind::Derived(source),
        }
    }
    /// Actual unbound source value or actual once-stored prior-let read,with all contributions.
    #[must_use]
    pub const fn value(self) -> FormulaEvaluatedScalar {
        self.data.value
    }
}
/// Complete actual scalar recipe/source execution;no cache,statement prefix or supplied constructor.
/// It certifies the supplied canonical/proof scope,not containing Design/currentness or approval.
/// ```compile_fail
/// fn seed(result:&mut sc_core::recipe::FormulaExecutedScalarSourceRecipe<'_>) {result.recipe.statements.clear();}
/// ```
/// ```compile_fail
/// fn escape<'a>(result:sc_core::recipe::FormulaExecutedScalarSourceRecipe<'a>)
///     ->sc_core::recipe::FormulaExecutedScalarSource<'a,'a> {result.sources().next().unwrap()}
/// ```
pub struct FormulaExecutedScalarSourceRecipe<'a> {
    pub(super) recipe: CompletedRecipe<'a>,
    pub(super) sources: CompletedSources<'a>,
}
impl<'a> FormulaExecutedScalarSourceRecipe<'a> {
    /// Complete original admitted recipe/source occurrences and original whole allowances.
    #[must_use]
    pub const fn recipe_plan(&self) -> &FormulaScalarRecipePlan<'a> {
        &self.recipe.plan
    }
    /// Complete original source graph/plans/quotas,including unused sources.
    #[must_use]
    pub const fn source_plan(&self) -> &FormulaScalarSourceGraphPlan<'a> {
        &self.sources.plan
    }
    /// Original single immutable tolerance/size context used by both runtimes.
    #[must_use]
    pub const fn context(&self) -> FormulaScalarExecutionContext<'a> {
        self.sources.context
    }
    /// Every original successful let/assertion in declaration order;no mutable prefix is exposed.
    #[must_use]
    pub fn statements(&self) -> &[FormulaExecutedScalarRecipeStatement<'a>] {
        &self.recipe.statements
    }
    /// Actual completed source by exact original id;None for an absent or unused source.
    #[must_use]
    pub fn source(&self, id: EntityId) -> Option<FormulaExecutedScalarSource<'_, 'a>> {
        let index = source_index(self.sources.plan.graph().dependencies().checked(), id)?;
        Some(completed_source_view(
            self.sources.plan.sources().get(index)?,
            self.sources.completed.get(index).copied().flatten()?,
        ))
    }
    /// Only actually completed sources in original registry order,with actual once-only ordinals.
    pub fn sources(&self) -> impl Iterator<Item = FormulaExecutedScalarSource<'_, 'a>> {
        self.sources
            .plan
            .sources()
            .iter()
            .zip(&self.sources.completed)
            .filter_map(|(source, completed)| Some(completed_source_view(source, (*completed)?)))
    }
    /// Original selected recipe reads,including real assertion headers after their operand reads.
    pub fn reads(&self) -> impl Iterator<Item = FormulaScalarSourceRecipeRead<'a>> + '_ {
        read_records(&self.recipe.plan)
            .map(|(dependency, data)| FormulaScalarSourceRecipeRead { dependency, data })
    }
}
impl fmt::Debug for FormulaExecutedScalarSourceRecipe<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaExecutedScalarSourceRecipe")
            .field("statements", &self.recipe.statements.len())
            .field("completed_sources", &self.sources().count())
            .finish_non_exhaustive()
    }
}
