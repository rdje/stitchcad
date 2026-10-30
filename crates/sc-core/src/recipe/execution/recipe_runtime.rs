//! Fully reserved original recipe state and one shared ordered statement/storage/assertion loop.
use super::{
    machine::Machine, recipe_read, source_runtime::SourceRuntime,
    FormulaExecutedScalarRecipeStatement, FormulaScalarExecutionContext as Context,
    FormulaScalarExecutionWorkspace as Scratch, FormulaScalarRecipeError as Error,
    FormulaScalarRecipeFailure as Failure, FormulaScalarRecipeOutcome as Outcome,
    FormulaScalarRecipePlan as Plan, FormulaScalarRecipeWorkspace as Workspace,
};
use crate::recipe::{FormulaCheckedStatementKind as Statement, FormulaStatementExpression as Part};

pub(super) mod failure;
use failure::OperandFailure;
pub(super) use failure::RecipeRuntimeError;
mod operand;

pub(super) struct CompletedRecipe<'a> {
    pub(super) plan: Plan<'a>,
    pub(super) statements: Vec<FormulaExecutedScalarRecipeStatement<'a>>,
}
pub(super) struct RecipeRuntime<'a> {
    plan: Plan<'a>,
    context: Context<'a>,
    machine: Machine<'a>,
    statements: Vec<FormulaExecutedScalarRecipeStatement<'a>>,
}
impl<'a> RecipeRuntime<'a> {
    // Complete machine/outcome reservations precede every operand,provider or storage operation.
    pub(super) fn new(plan: Plan<'a>, context: Context<'a>) -> Result<Self, Error<'a>> {
        let proof = plan.admission.proof();
        let machine = Machine::new(plan.admission.node_count()).map_err(|allocation| {
            let workspace = match allocation.workspace {
                Scratch::Tasks => Workspace::Tasks,
                Scratch::Values => Workspace::Values,
            };
            Error::allocation(proof, workspace, allocation.requested, allocation.error)
        })?;
        let mut statements = Vec::new();
        statements
            .try_reserve_exact(plan.admission.output_records())
            .map_err(|error| {
                Error::allocation(
                    proof,
                    Workspace::Statements,
                    plan.admission.output_records(),
                    error,
                )
            })?;
        Ok(Self {
            plan,
            context,
            machine,
            statements,
        })
    }
    // Ownership prevents publishing/reusing a successful prefix after any original failure.
    pub(super) fn execute(
        self,
        mut derived: Option<&mut SourceRuntime<'a>>,
    ) -> Result<CompletedRecipe<'a>, RecipeRuntimeError<'a>> {
        if derived
            .as_deref()
            .is_some_and(|runtime| !runtime.matches(self.plan.inputs(), self.context))
        {
            unreachable!("private paired runtimes share actual canonical inputs and original context sources");
        }
        let Self {
            mut plan,
            context,
            mut machine,
            mut statements,
        } = self;
        let proof = plan.admission.proof();
        let mut remaining = plan.sources.as_mut_slice();
        for statement in proof.statements() {
            let refuse =
                |failure| RecipeRuntimeError::Recipe(Error::evaluation(proof, statement, failure));
            let operand_error = |part, error| match error {
                OperandFailure::Scalar(error) => refuse(Failure::Expression { part, error }),
                OperandFailure::Source(error) => RecipeRuntimeError::Source {
                    statement,
                    part,
                    error,
                },
            };
            let count = match statement.kind() {
                Statement::Let { expression } => expression.dependencies().len(),
                Statement::Assert { left, right } => {
                    1 + left.dependencies().len() + right.dependencies().len()
                }
            };
            let Some((sources, rest)) = remaining.split_at_mut_checked(count) else {
                unreachable!(
                    "complete admitted whole source plan retains this real statement block"
                );
            };
            remaining = rest;
            let outcome = match statement.kind() {
                Statement::Let { expression } => {
                    let value = operand::evaluate(
                        &mut machine,
                        expression.root(),
                        sources,
                        &statements,
                        context,
                        derived.as_deref_mut(),
                    )
                    .map_err(|error| operand_error(Part::Binding, error))?;
                    let binding = value.store_binding().map_err(|rule| {
                        refuse(Failure::Binding {
                            kind: value.kind(),
                            rule,
                        })
                    })?;
                    Outcome::Binding {
                        expression: value,
                        binding,
                    }
                }
                Statement::Assert { left, right } => {
                    let Some((header, sources)) = sources.split_first_mut() else {
                        unreachable!(
                            "actual assertion source block begins with its real tolerance header"
                        );
                    };
                    let Some((left_sources, right_sources)) =
                        sources.split_at_mut_checked(left.dependencies().len())
                    else {
                        unreachable!("actual assertion source block retains both original operand populations");
                    };
                    let a = operand::evaluate(
                        &mut machine,
                        left.root(),
                        left_sources,
                        &statements,
                        context,
                        derived.as_deref_mut(),
                    )
                    .map_err(|error| operand_error(Part::AssertionLeft, error))?;
                    let b = operand::evaluate(
                        &mut machine,
                        right.root(),
                        right_sources,
                        &statements,
                        context,
                        derived.as_deref_mut(),
                    )
                    .map_err(|error| operand_error(Part::AssertionRight, error))?;
                    let tolerance = recipe_read::header(header, context)
                        .map_err(|error| refuse(Failure::ToleranceRead(error)))?;
                    let comparison = a
                        .compare_tolerance(b, tolerance)
                        .map_err(|error| refuse(Failure::Comparison(error)))?;
                    if !comparison.holds() {
                        return Err(refuse(Failure::Assertion(comparison)));
                    }
                    Outcome::Assertion(comparison)
                }
            };
            if statements.len() >= plan.admission.output_records() {
                unreachable!(
                    "whole private admission reserves every actual successful statement outcome"
                );
            }
            statements.push(FormulaExecutedScalarRecipeStatement { statement, outcome });
        }
        if !remaining.is_empty() {
            unreachable!("complete whole execution consumes exactly its original source blocks");
        }
        Ok(CompletedRecipe { plan, statements })
    }
}

#[cfg(test)]
mod tests;
