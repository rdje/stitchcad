//! One declaration-order pass; completed lets become visible only after real RHS/storage success.
use super::{
    machine::Machine,
    recipe_read::{self, RecipeSources},
    FormulaExecutedScalarRecipe, FormulaExecutedScalarRecipeStatement,
    FormulaScalarExecutionContext as Context, FormulaScalarExecutionWorkspace as Scratch,
    FormulaScalarRecipeError as Error, FormulaScalarRecipeFailure as Failure,
    FormulaScalarRecipeOutcome as Outcome, FormulaScalarRecipePlan as Plan,
    FormulaScalarRecipeWorkspace as Workspace,
};
use crate::recipe::{FormulaCheckedStatementKind as Statement, FormulaStatementExpression as Part};

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
        let mut plan: Plan<'a> = self;
        let proof = plan.admission.proof();
        let mut machine = Machine::new(plan.admission.node_count()).map_err(|allocation| {
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
        Ok(FormulaExecutedScalarRecipe { plan, statements })
    }
}
