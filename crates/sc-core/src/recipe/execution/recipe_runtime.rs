//! Fully reserved original recipe state and one shared ordered statement/storage/assertion loop.
use super::{
    machine::Machine,
    recipe_read::{self, RecipeSources},
    FormulaExecutedScalarRecipeStatement, FormulaScalarExecutionContext as Context,
    FormulaScalarExecutionWorkspace as Scratch, FormulaScalarRecipeError as Error,
    FormulaScalarRecipeFailure as Failure, FormulaScalarRecipeOutcome as Outcome,
    FormulaScalarRecipePlan as Plan, FormulaScalarRecipeWorkspace as Workspace,
};
use crate::recipe::{FormulaCheckedStatementKind as Statement, FormulaStatementExpression as Part};

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
    pub(super) fn execute(self) -> Result<CompletedRecipe<'a>, Error<'a>> {
        let Self {
            mut plan,
            context,
            mut machine,
            mut statements,
        } = self;
        let proof = plan.admission.proof();
        let mut remaining = plan.sources.as_mut_slice();
        for statement in proof.statements() {
            let refuse = |failure| Error::evaluation(proof, statement, failure);
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
                    let value = machine
                        .run(
                            expression.root(),
                            &mut RecipeSources {
                                sources,
                                completed: &statements,
                                derived: None,
                            },
                            context,
                        )
                        .map_err(|error| {
                            refuse(Failure::Expression {
                                part: Part::Binding,
                                error,
                            })
                        })?;
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
                    let a = machine
                        .run(
                            left.root(),
                            &mut RecipeSources {
                                sources: left_sources,
                                completed: &statements,
                                derived: None,
                            },
                            context,
                        )
                        .map_err(|error| {
                            refuse(Failure::Expression {
                                part: Part::AssertionLeft,
                                error,
                            })
                        })?;
                    let b = machine
                        .run(
                            right.root(),
                            &mut RecipeSources {
                                sources: right_sources,
                                completed: &statements,
                                derived: None,
                            },
                            context,
                        )
                        .map_err(|error| {
                            refuse(Failure::Expression {
                                part: Part::AssertionRight,
                                error,
                            })
                        })?;
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
