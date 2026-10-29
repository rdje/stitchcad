//! Private-construction whole scalar outcomes and actual read witnesses.
use super::{FormulaScalarExecutionReadKind, FormulaScalarRecipePlan, FormulaScalarRecipeSource};
use crate::recipe::{
    FormulaCheckedNode, FormulaCheckedStatement, FormulaCheckedStatementKind as Statement,
    FormulaEvaluatedBinding, FormulaEvaluatedScalar, FormulaRecipeNameDependency,
    FormulaToleranceComparison,
};
use core::fmt;

/// Original successful read; prior-let values come only from the engine's completed private prefix.
#[derive(Clone, Copy)]
pub enum FormulaScalarRecipeReadKind<'a> {
    /// Original canonical/reserved read, retaining authored state or actual provider/context.
    Scalar(FormulaScalarExecutionReadKind<'a>),
    /// Real completed earlier let and its once-stored value, with all approximation contributions.
    PriorLet {
        /// Actual checked supplier statement in this same complete proof.
        statement: &'a FormulaCheckedStatement<'a>,
        /// Actual stored binding copied from that private completed outcome, not caller input.
        binding: FormulaEvaluatedBinding,
    },
}
impl fmt::Debug for FormulaScalarRecipeReadKind<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Scalar(_) => "Scalar(..)",
            Self::PriorLet { .. } => "PriorLet(..)",
        })
    }
}
#[derive(Clone, Copy)]
pub(super) struct RecipeReadData<'a> {
    pub(super) node: Option<FormulaCheckedNode<'a, 'a>>,
    pub(super) source: FormulaScalarRecipeReadKind<'a>,
    pub(super) value: FormulaEvaluatedScalar,
}
impl fmt::Debug for RecipeReadData<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RecipeReadData")
            .field("source", &self.source)
            .field("kind", &self.value.kind())
            .finish_non_exhaustive()
    }
}

/// Actual executed occurrence, including a real assertion header with no fabricated AST node.
#[derive(Clone, Copy, Debug)]
pub struct FormulaScalarRecipeRead<'a> {
    dependency: FormulaRecipeNameDependency<'a>,
    data: RecipeReadData<'a>,
}
impl<'a> FormulaScalarRecipeRead<'a> {
    /// Original consumer ordinal, role, span and declaration.
    #[must_use]
    pub const fn dependency(self) -> FormulaRecipeNameDependency<'a> {
        self.dependency
    }
    /// Actual operand name node; an assertion tolerance header is metadata and returns None.
    #[must_use]
    pub const fn node(self) -> Option<FormulaCheckedNode<'a, 'a>> {
        self.data.node
    }
    /// Original successful canonical/provider/prior-binding source.
    #[must_use]
    pub const fn source(self) -> FormulaScalarRecipeReadKind<'a> {
        self.data.source
    }
    /// Actual sourced scalar read; prior lets expose their stored integer or Boolean.
    #[must_use]
    pub const fn value(self) -> FormulaEvaluatedScalar {
        self.data.value
    }
}

/// Actual successful statement outcome; the enclosing private wrapper certifies real execution.
#[derive(Clone, Copy)]
pub enum FormulaScalarRecipeOutcome<'a> {
    /// RHS completes before the one binding conversion; later reads use binding, not expression.
    Binding {
        /// Exact unbound RHS result, retaining all executed contributions.
        expression: FormulaEvaluatedScalar,
        /// Actual once-rounded numeric or unchanged Boolean stored result.
        binding: FormulaEvaluatedBinding,
    },
    /// True original tolerance comparison, including both operands and original class/provider.
    Assertion(FormulaToleranceComparison<'a>),
}
impl fmt::Debug for FormulaScalarRecipeOutcome<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Binding { .. } => "Binding(..)",
            Self::Assertion(_) => "Assertion(..)",
        })
    }
}

/// One genuinely completed actual statement; neither fields nor constructor are public.
#[derive(Clone, Copy, Debug)]
pub struct FormulaExecutedScalarRecipeStatement<'a> {
    pub(super) statement: &'a FormulaCheckedStatement<'a>,
    pub(super) outcome: FormulaScalarRecipeOutcome<'a>,
}
impl<'a> FormulaExecutedScalarRecipeStatement<'a> {
    /// Original whole-proof statement, including its real name, source spans and one-based ordinal.
    #[must_use]
    pub const fn statement(self) -> &'a FormulaCheckedStatement<'a> {
        self.statement
    }
    /// Actual successfully evaluated/bound or asserted outcome.
    #[must_use]
    pub const fn outcome(self) -> FormulaScalarRecipeOutcome<'a> {
        self.outcome
    }
}

/// Successful whole execution of the admitted available scalar subset; no partial constructor.
/// It grants no persistence, missing-family/Derived evaluation, physical or release approval.
/// ```compile_fail
/// fn forge(result: &mut sc_core::recipe::FormulaExecutedScalarRecipe<'_>) {
///     result.statements.clear();
/// }
/// ```
pub struct FormulaExecutedScalarRecipe<'a> {
    pub(super) plan: FormulaScalarRecipePlan<'a>,
    pub(super) statements: Vec<FormulaExecutedScalarRecipeStatement<'a>>,
}
impl<'a> FormulaExecutedScalarRecipe<'a> {
    /// Original whole admission, canonical inventory and complete prospective source plan.
    #[must_use]
    pub const fn plan(&self) -> &FormulaScalarRecipePlan<'a> {
        &self.plan
    }
    /// Every actual successful let/assert outcome in original declaration order.
    #[must_use]
    pub fn statements(&self) -> &[FormulaExecutedScalarRecipeStatement<'a>] {
        &self.statements
    }
    /// Actual successful read order: selected operand names left-to-right, assertion header last.
    /// Streams the same D records without allocating another trace or inventing a header node.
    pub fn reads(&self) -> impl Iterator<Item = FormulaScalarRecipeRead<'a>> + '_ {
        let mut offset = 0usize;
        self.plan
            .admission()
            .proof()
            .statements()
            .iter()
            .flat_map(move |statement| {
                let (count, assertion) = match statement.kind() {
                    Statement::Let { expression } => (expression.dependencies().len(), false),
                    Statement::Assert { left, right } => (
                        1 + left.dependencies().len() + right.dependencies().len(),
                        true,
                    ),
                };
                let Some(end) = offset.checked_add(count) else {
                    unreachable!("complete admitted source count fits");
                };
                let Some(block) = self.plan.sources.get(offset..end) else {
                    unreachable!("whole source plan retains every original occurrence");
                };
                offset = end;
                let (header, operands): (&[FormulaScalarRecipeSource<'a>], _) = if assertion {
                    let Some((header, operands)) = block.split_first() else {
                        unreachable!("actual assertion has its original tolerance header");
                    };
                    (core::slice::from_ref(header), operands)
                } else {
                    (&[], block)
                };
                operands.iter().chain(header.iter()).filter_map(|source| {
                    Some(FormulaScalarRecipeRead {
                        dependency: source.dependency(),
                        data: source.read?,
                    })
                })
            })
    }
}
impl fmt::Debug for FormulaExecutedScalarRecipe<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaExecutedScalarRecipe")
            .field("statement_count", &self.statements.len())
            .field("read_count", &self.reads().count())
            .finish_non_exhaustive()
    }
}
