//! Complete checked-expression workspace admission, before allocation or value/provider reads.
use super::{FormulaCheckedExpression, FormulaCheckedNode, FormulaNameDependency};
use core::fmt;

mod plan;
pub use plan::{
    FormulaScalarExecutionPlan, FormulaScalarExecutionPlanError, FormulaScalarExecutionPlanRefusal,
    FormulaScalarExecutionSource, FormulaScalarExecutionSourceKind,
};
mod read;
pub use read::{
    FormulaScalarExecutionContext, FormulaScalarExecutionRead, FormulaScalarExecutionReadKind,
};
mod failure;
pub use failure::{
    FormulaScalarExecutionError, FormulaScalarExecutionErrorCause, FormulaScalarExecutionFailure,
    FormulaScalarExecutionWorkspace,
};
mod machine;
mod worker;
pub use worker::FormulaExecutedScalarExpression;
mod dispatch;

mod source_graph;
pub use source_graph::{
    FormulaScalarSourceGraphPlan, FormulaScalarSourceGraphPlanError,
    FormulaScalarSourceGraphPlanRefusal, FormulaScalarSourceGraphSource,
    FormulaSourceGraphExecutionLimits,
};

mod source_read;
pub use source_read::{
    FormulaExecutedDerivedSourceRead, FormulaScalarSourceGraphRead,
    FormulaScalarSourceGraphReadKind,
};
mod source_failure;
pub use source_failure::{
    FormulaScalarSourceGraphExecutionCause, FormulaScalarSourceGraphExecutionError,
    FormulaScalarSourceGraphWorkspace,
};
mod source_result;
pub use source_result::{FormulaExecutedScalarSource, FormulaExecutedScalarSourceGraph};
mod source_runtime;
mod source_worker;

mod recipe;
pub use recipe::{
    FormulaRecipeExecutionAdmission, FormulaRecipeExecutionAdmissionError,
    FormulaRecipeExecutionAdmissionRefusal, FormulaRecipeExecutionLimits,
};

mod recipe_plan;
pub use recipe_plan::{
    FormulaScalarRecipePlan, FormulaScalarRecipePlanError, FormulaScalarRecipePlanRefusal,
    FormulaScalarRecipeSource, FormulaScalarRecipeSourceKind,
};

/// Explicit per-expression node and logical workspace allowances, without implicit defaults.
/// Zero is a valid allowance and refuses any nonempty complete proof.
/// These limits do not widen the language's fixed syntax bounds or bound allocator bytes/CPU.
/// ```compile_fail
/// let limits: sc_core::recipe::FormulaExecutionLimits = Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormulaExecutionLimits {
    nodes: usize,
    workspace_records: usize,
}
impl FormulaExecutionLimits {
    /// Select both independent allowances explicitly; unused branches remain in the population.
    #[must_use]
    pub const fn new(nodes: usize, workspace_records: usize) -> Self {
        Self {
            nodes,
            workspace_records,
        }
    }
    /// Maximum complete certified node population admitted before runtime preparation.
    #[must_use]
    pub const fn nodes(self) -> usize {
        self.nodes
    }
    /// Maximum sum of task frames, value slots and executed-source-read records.
    #[must_use]
    pub const fn workspace_records(self) -> usize {
        self.workspace_records
    }
}

/// Exact admission refusal, separate from language diagnostics and numeric/domain evaluation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaExecutionAdmissionRefusal {
    /// The complete actual proof exceeds the selected node allowance; checked first.
    Nodes {
        /// Explicit selected allowance.
        allowed: usize,
        /// Complete certified population, including untaken branches.
        actual: usize,
    },
    /// The complete logical reservation cannot be represented; no allocation is attempted.
    WorkspaceCountOverflow {
        /// Complete actual node population.
        nodes: usize,
        /// Every actual name occurrence, including repeated and untaken uses.
        name_uses: usize,
    },
    /// The full checked `2 * nodes + name_uses` reservation exceeds its allowance.
    WorkspaceRecords {
        /// Explicit selected allowance.
        allowed: usize,
        /// Complete prospective reservation, without taking a runtime branch.
        required: usize,
    },
}

/// Refusal borrowing the exact successful proof's root and original expression/source extent.
/// No partial admission, fabricated statement ordinal or canonical language diagnostic escapes.
#[derive(Debug)]
pub struct FormulaExecutionAdmissionError<'p, 'd> {
    root: FormulaCheckedNode<'p, 'd>,
    refusal: FormulaExecutionAdmissionRefusal,
}
impl<'p, 'd> FormulaExecutionAdmissionError<'p, 'd> {
    /// Actual certified root refused by this admission, retaining its original source owner.
    #[must_use]
    pub const fn root(&self) -> FormulaCheckedNode<'p, 'd> {
        self.root
    }
    /// Exact selected limit and measured complete population.
    #[must_use]
    pub const fn refusal(&self) -> FormulaExecutionAdmissionRefusal {
        self.refusal
    }
}
impl fmt::Display for FormulaExecutionAdmissionError<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "formula execution admission at {:?}: {:?}",
            self.root.span(),
            self.refusal
        )
    }
}
impl std::error::Error for FormulaExecutionAdmissionError<'_, '_> {}

/// Complete prospective iterative workspace bound over one actual successful static proof.
/// For N certified nodes and D name occurrences, reserve N task frames, N value slots and D
/// source-read records. This view performs no allocation, cloning, value read or provider call.
/// It does not certify backend availability, execution, input currentness or whole-recipe success.
/// Private fields prevent substituting a foreign root or reducing the admitted population:
/// ```compile_fail
/// fn forge(admission: &mut sc_core::recipe::FormulaExecutionAdmission<'_, '_>) {
///     admission.node_count = 0;
/// }
/// ```
/// A normalized owner may outlive its static proof; the admission cannot escape that proof:
/// ```compile_fail
/// fn escape<'e, 's>(expression: &'e sc_core::recipe::FormulaNormalizedExpression<'s>)
///     -> sc_core::recipe::FormulaExecutionAdmission<'e, 'static> {
///     use sc_core::recipe::*;
///     let proof = expression.check_kinds(&FormulaNamespace::new([]).unwrap()).unwrap();
///     proof.admit_execution(FormulaExecutionLimits::new(256, 768)).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct FormulaExecutionAdmission<'p, 'd> {
    root: FormulaCheckedNode<'p, 'd>,
    dependencies: &'p [FormulaNameDependency<'d>],
    limits: FormulaExecutionLimits,
    node_count: usize,
    workspace_records: usize,
}
impl<'p, 'd> FormulaExecutionAdmission<'p, 'd> {
    /// Actual certified root; all children remain pinned to the original complete proof.
    #[must_use]
    pub const fn root(&self) -> FormulaCheckedNode<'p, 'd> {
        self.root
    }
    /// Original ordered static name uses, including repeats and untaken branches.
    /// These are prospective capacities, not a record of executed reads.
    #[must_use]
    pub const fn dependencies(&self) -> &'p [FormulaNameDependency<'d>] {
        self.dependencies
    }
    /// Complete certified node count; independently bounds each task and value population.
    #[must_use]
    pub const fn node_count(&self) -> usize {
        self.node_count
    }
    /// Complete maximum source-read population; this does not read any authored state.
    #[must_use]
    pub const fn name_use_count(&self) -> usize {
        self.dependencies.len()
    }
    /// Checked full prospective reservation, `2 * node_count + name_use_count`.
    #[must_use]
    pub const fn workspace_records(&self) -> usize {
        self.workspace_records
    }
    /// Actual explicit allowances selected for this admission.
    #[must_use]
    pub const fn limits(&self) -> FormulaExecutionLimits {
        self.limits
    }
}

impl<'e, 's, 'd> FormulaCheckedExpression<'e, 's, 'd> {
    /// Admit the complete prospective iterative workspace, without allocating or evaluating.
    /// The actual private proof supplies every count and source; callers supply only limits.
    ///
    /// ```
    /// use sc_core::recipe::{FormulaExecutionLimits, FormulaExpression, FormulaNamespace};
    /// let expression = FormulaExpression::parse("if(is_base_size,1 mm,2 mm)")?
    ///     .normalize_literals()?;
    /// let proof = expression.check_kinds(&FormulaNamespace::new([])?).expect("static fixture");
    /// let admitted = proof.admit_execution(FormulaExecutionLimits::new(4, 9)).expect("exact limits");
    /// assert_eq!(admitted.node_count(), 4);
    /// assert_eq!(admitted.name_use_count(), 1);
    /// assert_eq!(admitted.workspace_records(), 9);
    /// assert!(std::ptr::eq(admitted.root().expression(), &expression));
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    /// # Errors
    /// Node quota precedes checked workspace arithmetic, then the workspace quota. Refusals keep
    /// this actual proof's root/span. Prior syntax/static errors cannot produce an admission.
    pub fn admit_execution(
        &self,
        limits: FormulaExecutionLimits,
    ) -> Result<FormulaExecutionAdmission<'_, 'd>, FormulaExecutionAdmissionError<'_, 'd>> {
        use FormulaExecutionAdmissionRefusal as Refusal;
        let root = self.root();
        let refuse = |refusal| FormulaExecutionAdmissionError { root, refusal };
        let node_count = self.node_count();
        if node_count > limits.nodes {
            return Err(refuse(Refusal::Nodes {
                allowed: limits.nodes,
                actual: node_count,
            }));
        }
        let dependencies = self.dependencies();
        let Some(workspace_records) = node_count
            .checked_mul(2)
            .and_then(|slots| slots.checked_add(dependencies.len()))
        else {
            return Err(refuse(Refusal::WorkspaceCountOverflow {
                nodes: node_count,
                name_uses: dependencies.len(),
            }));
        };
        if workspace_records > limits.workspace_records {
            return Err(refuse(Refusal::WorkspaceRecords {
                allowed: limits.workspace_records,
                required: workspace_records,
            }));
        }
        Ok(FormulaExecutionAdmission {
            root,
            dependencies,
            limits,
            node_count,
            workspace_records,
        })
    }
}

mod recipe_result;
pub use recipe_result::{
    FormulaExecutedScalarRecipe, FormulaExecutedScalarRecipeStatement, FormulaScalarRecipeOutcome,
    FormulaScalarRecipeRead, FormulaScalarRecipeReadKind,
};
mod recipe_failure;
pub use recipe_failure::{
    FormulaScalarRecipeError, FormulaScalarRecipeErrorCause, FormulaScalarRecipeFailure,
    FormulaScalarRecipeWorkspace,
};

mod recipe_read;
mod recipe_replay;
mod recipe_runtime;
mod recipe_worker;
pub use recipe_replay::FormulaScalarRecipeReplayError;

mod source_recipe_failure;
pub use source_recipe_failure::{
    FormulaScalarSourceRecipeError, FormulaScalarSourceRecipeErrorCause,
    FormulaScalarSourceRecipeSourceError,
};
mod source_recipe_result;
pub use source_recipe_result::{
    FormulaExecutedScalarSourceRecipe, FormulaScalarSourceRecipeRead,
    FormulaScalarSourceRecipeReadKind,
};
mod source_recipe_replay;
mod source_recipe_worker;
pub use source_recipe_replay::FormulaScalarSourceRecipeReplayError;
