//! Complete scalar source preparation; a prior-let link is not an executed binding.
use super::{plan::scalar_builtin_available, FormulaRecipeExecutionAdmission};
use crate::{
    recipe::{
        FormulaBuiltin, FormulaCanonicalInput, FormulaCanonicalInputError,
        FormulaCheckedExpression, FormulaCheckedNode, FormulaCheckedNodeForm as Form,
        FormulaCheckedRecipe, FormulaCheckedStatement, FormulaCheckedStatementKind as Statement,
        FormulaDeclarationSource, FormulaRecipeNameDependency, FormulaReservedName,
        FormulaStatementExpression,
    },
    value::CanonicalInputs,
};
use core::fmt;
use std::collections::TryReserveError;

/// Actual prospective supplier; no numerical value or executed-prefix certificate is granted.
#[derive(Clone, Copy, Debug)]
pub enum FormulaScalarRecipeSourceKind<'a> {
    /// Complete canonical identity/family/owner join in the actual supplied inventory.
    Canonical(FormulaCanonicalInput<'a>),
    /// Original reserved role, including assertion headers; providers remain unread.
    Reserved(FormulaReservedName),
    /// Actual earlier checked let in this whole proof, still unevaluated and unbound.
    PriorLet(&'a FormulaCheckedStatement<'a>),
}

/// One original whole dependency occurrence and its actual prospective supplier.
/// Private fields prevent attaching a foreign source, statement, ordinal or role.
#[derive(Clone, Copy, Debug)]
pub struct FormulaScalarRecipeSource<'a> {
    pub(super) dependency: FormulaRecipeNameDependency<'a>,
    pub(super) kind: FormulaScalarRecipeSourceKind<'a>,
    pub(super) read: Option<super::recipe_result::RecipeReadData<'a>>,
}
impl<'a> FormulaScalarRecipeSource<'a> {
    /// Actual consumer position, role, span and declaration, including assertion-header metadata.
    #[must_use]
    pub const fn dependency(self) -> FormulaRecipeNameDependency<'a> {
        self.dependency
    }
    /// Original source join or real checked supplier; not a value or execution witness.
    #[must_use]
    pub const fn kind(self) -> FormulaScalarRecipeSourceKind<'a> {
        self.kind
    }
}

/// Whole preparation failure, separate from canonical language and numerical diagnostics.
#[derive(Debug)]
pub enum FormulaScalarRecipePlanRefusal<'a> {
    /// A tangent or geometry selector is outside the actual implemented scalar capability.
    UnavailableBuiltin(FormulaBuiltin),
    /// Geometry source needs its actual implementation owner, even in an untaken branch.
    UnavailableSource(FormulaDeclarationSource<'a>),
    /// Original canonical identity/family/kind/owner failure, without a value read.
    CanonicalInput(FormulaCanonicalInputError<'a>),
    /// Fallible complete D-source reservation failed; the error retains the original allocator cause.
    Allocation {
        /// Complete admitted prospective source count.
        requested: usize,
    },
}

#[derive(Debug)]
enum Issue<'a> {
    Capability {
        statement: &'a FormulaCheckedStatement<'a>,
        part: FormulaStatementExpression,
        node: FormulaCheckedNode<'a, 'a>,
    },
    Canonical {
        dependency_index: usize,
    },
    Allocation {
        requested: usize,
        error: TryReserveError,
    },
}

/// Original whole proof/inventory and first preparation refusal, with no partial plan exposed.
/// Debug omits authored source/name content; explicit getters retain available actual context.
#[derive(Debug)]
pub struct FormulaScalarRecipePlanError<'a> {
    proof: &'a FormulaCheckedRecipe<'a>,
    inputs: &'a CanonicalInputs,
    issue: Issue<'a>,
}
impl<'a> FormulaScalarRecipePlanError<'a> {
    /// Actual whole static proof, including its suffix after this refusal.
    #[must_use]
    pub const fn proof(&self) -> &'a FormulaCheckedRecipe<'a> {
        self.proof
    }
    /// Actual immutable canonical inventory used for joins.
    #[must_use]
    pub const fn inputs(&self) -> &'a CanonicalInputs {
        self.inputs
    }
    /// Real failed operand's statement or canonical dependency's consumer; allocation has no ordinal.
    #[must_use]
    pub fn statement(&self) -> Option<&'a FormulaCheckedStatement<'a>> {
        match self.issue {
            Issue::Capability { statement, .. } => Some(statement),
            Issue::Canonical { .. } => {
                let dependency = self.dependency()?;
                self.proof
                    .statements()
                    .get(dependency.statement_index().checked_sub(1)?)
            }
            Issue::Allocation { .. } => None,
        }
    }
    /// Actual operand role for a capability refusal; no synthetic operand for a header/allocation.
    #[must_use]
    pub const fn expression_part(&self) -> Option<FormulaStatementExpression> {
        match self.issue {
            Issue::Capability { part, .. } => Some(part),
            _ => None,
        }
    }
    /// Actual certified failing capability node; canonical failures expose their original dependency.
    #[must_use]
    pub const fn node(&self) -> Option<FormulaCheckedNode<'a, 'a>> {
        match self.issue {
            Issue::Capability { node, .. } => Some(node),
            _ => None,
        }
    }
    /// Original refused canonical occurrence; reconstructed only from this same immutable whole proof.
    #[must_use]
    pub fn dependency(&self) -> Option<FormulaRecipeNameDependency<'a>> {
        match self.issue {
            Issue::Canonical { dependency_index } => {
                self.proof.dependencies().nth(dependency_index)
            }
            _ => None,
        }
    }
    /// Full original typed source/capability cause, without numerical reads or default diagnostics.
    #[must_use]
    pub fn refusal(&self) -> FormulaScalarRecipePlanRefusal<'a> {
        use FormulaScalarRecipePlanRefusal as R;
        match &self.issue {
            Issue::Capability { node, .. } => match node.form() {
                Form::Call { builtin, .. } => R::UnavailableBuiltin(builtin),
                Form::Name { declaration, .. } => R::UnavailableSource(declaration.source()),
                _ => {
                    unreachable!("private capability issue retains only an unavailable call/source")
                }
            },
            Issue::Canonical { .. } => {
                let Some(dependency) = self.dependency() else {
                    unreachable!("private canonical issue retains a real whole dependency index");
                };
                let Err(error) =
                    FormulaCanonicalInput::resolve(dependency.declaration(), self.inputs)
                else {
                    unreachable!(
                        "same immutable source/inventory reproduces its original join refusal"
                    );
                };
                R::CanonicalInput(error)
            }
            Issue::Allocation { requested, .. } => R::Allocation {
                requested: *requested,
            },
        }
    }
    /// Original allocator error, without allocating another diagnostic payload after its failure.
    #[must_use]
    pub const fn allocation_error(&self) -> Option<&TryReserveError> {
        match &self.issue {
            Issue::Allocation { error, .. } => Some(error),
            _ => None,
        }
    }
}
impl fmt::Display for FormulaScalarRecipePlanError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "whole scalar recipe preparation: {:?}", self.issue)
    }
}
impl std::error::Error for FormulaScalarRecipePlanError<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.allocation_error().map(|error| error as _)
    }
}

/// Complete available-scalar source preparation pinned to the actual whole proof/inventory.
/// A prior let remains a checked supplier link, never a supplied or computed runtime value.
/// All capability checks precede one fallible D-source allocation and ordered inventory joins.
/// ```compile_fail
/// fn substitute<'a>(plan: &mut sc_core::recipe::FormulaScalarRecipePlan<'a>,
///     inputs: &'a sc_core::value::CanonicalInputs) { plan.inputs = inputs; }
/// ```
/// ```compile_fail
/// fn escape<'a>(admission: sc_core::recipe::FormulaRecipeExecutionAdmission<'a, 'a>)
///     -> sc_core::recipe::FormulaScalarRecipePlan<'a> {
///     let inputs = sc_core::value::CanonicalInputs::new(vec![], vec![]).unwrap();
///     admission.prepare_scalar(&inputs).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct FormulaScalarRecipePlan<'a> {
    pub(super) admission: FormulaRecipeExecutionAdmission<'a, 'a>,
    inputs: &'a CanonicalInputs,
    pub(super) sources: Vec<FormulaScalarRecipeSource<'a>>,
}
impl<'a> FormulaScalarRecipePlan<'a> {
    /// Original complete proof and selected whole logical allowances.
    #[must_use]
    pub const fn admission(&self) -> &FormulaRecipeExecutionAdmission<'a, 'a> {
        &self.admission
    }
    /// Exact supplied canonical inventory, without a clone of state, values or source records.
    #[must_use]
    pub const fn inputs(&self) -> &'a CanonicalInputs {
        self.inputs
    }
    /// Every original occurrence in whole dependency order, including headers/repetitions/branches.
    /// These are prospective sources; no executed read trace or accepted prefix is implied.
    #[must_use]
    pub fn sources(&self) -> &[FormulaScalarRecipeSource<'a>] {
        &self.sources
    }
}

fn operands<'a>(
    statement: &'a FormulaCheckedStatement<'a>,
) -> impl Iterator<
    Item = (
        FormulaStatementExpression,
        &'a FormulaCheckedExpression<'a, 'a, 'a>,
    ),
> {
    use FormulaStatementExpression as P;
    match statement.kind() {
        Statement::Let { expression } => [Some((P::Binding, expression)), None],
        Statement::Assert { left, right } => [
            Some((P::AssertionLeft, left)),
            Some((P::AssertionRight, right)),
        ],
    }
    .into_iter()
    .flatten()
}

fn prior_let<'a>(
    proof: &'a FormulaCheckedRecipe<'a>,
    dependency: FormulaRecipeNameDependency<'a>,
) -> &'a FormulaCheckedStatement<'a> {
    let declaration = dependency.declaration();
    let FormulaDeclarationSource::Recipe {
        statement_index,
        kind,
        span,
        name_span,
    } = declaration.source()
    else {
        unreachable!("only an actual prior-let occurrence enters its private join");
    };
    let Some(statement) = statement_index
        .checked_sub(1)
        .and_then(|i| proof.statements().get(i))
    else {
        unreachable!("complete private recipe proof retains every original supplier");
    };
    if statement_index >= dependency.statement_index()
        || statement.statement_index() != statement_index
        || statement.statement().name() != declaration.name()
        || statement.statement().span() != span
        || statement.statement().name_span() != name_span
        || !matches!(statement.kind(), Statement::Let { expression } if expression.kind() == kind.into())
    {
        unreachable!("whole static proof certifies this exact earlier binding metadata/owner");
    }
    statement
}

impl<'p, 'd> FormulaRecipeExecutionAdmission<'p, 'd> {
    /// Prepare every prospective source in the available scalar subset before runtime values.
    /// All statements/operands pass the shared capability scan before allocation or inventory joins.
    /// ```
    /// use sc_core::{recipe::*, value::CanonicalInputs};
    /// let inputs = CanonicalInputs::new(vec![], vec![])?;
    /// let recipe = FormulaRecipe::parse("let n:count=1\nassert check:eps_phys=n==n")?
    ///     .normalize_literals()?;
    /// let proof = recipe.check_kinds(FormulaNamespace::new([])?).expect("whole proof");
    /// let plan = proof.admit_execution(FormulaRecipeExecutionLimits::new(2, 3, 9, 2))
    ///     .expect("exact allowance").prepare_scalar(&inputs).expect("sources only");
    /// assert_eq!(plan.sources().len(), 3); // physical tolerance header plus two actual prior-let links
    /// assert!(std::ptr::eq(plan.admission().proof(), &proof));
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    /// # Errors
    /// Unavailable capability precedes allocation and source joins; joins follow original whole
    /// dependency order. No partial plan/session/value escapes; optional providers remain unread.
    pub fn prepare_scalar<'a>(
        self,
        inputs: &'a CanonicalInputs,
    ) -> Result<FormulaScalarRecipePlan<'a>, FormulaScalarRecipePlanError<'a>>
    where
        'p: 'a,
        'd: 'a,
    {
        let admission: FormulaRecipeExecutionAdmission<'a, 'a> = self;
        let proof = admission.proof();
        let refuse = |issue| FormulaScalarRecipePlanError {
            proof,
            inputs,
            issue,
        };
        for statement in proof.statements() {
            for (part, expression) in operands(statement) {
                for node in expression.root().all_nodes() {
                    let unavailable = match node.form() {
                        Form::Call { builtin, .. } => !scalar_builtin_available(builtin),
                        Form::Name { declaration, .. } => matches!(
                            declaration.source(),
                            FormulaDeclarationSource::Point(_) | FormulaDeclarationSource::Edge(_)
                        ),
                        _ => false,
                    };
                    if unavailable {
                        return Err(refuse(Issue::Capability {
                            statement,
                            part,
                            node,
                        }));
                    }
                }
            }
        }
        let mut sources = Vec::new();
        sources
            .try_reserve_exact(admission.name_use_count())
            .map_err(|error| {
                refuse(Issue::Allocation {
                    requested: admission.name_use_count(),
                    error,
                })
            })?;
        for (dependency_index, dependency) in proof.dependencies().enumerate() {
            let declaration = dependency.declaration();
            let kind = match declaration.source() {
                FormulaDeclarationSource::Reserved(name) => {
                    FormulaScalarRecipeSourceKind::Reserved(name)
                }
                FormulaDeclarationSource::Recipe { .. } => {
                    FormulaScalarRecipeSourceKind::PriorLet(prior_let(proof, dependency))
                }
                FormulaDeclarationSource::Input { .. }
                | FormulaDeclarationSource::LengthInput { .. } => {
                    FormulaScalarRecipeSourceKind::Canonical(
                        FormulaCanonicalInput::resolve(declaration, inputs)
                            .map_err(|_| refuse(Issue::Canonical { dependency_index }))?,
                    )
                }
                FormulaDeclarationSource::Point(_) | FormulaDeclarationSource::Edge(_) => {
                    unreachable!("all actual geometry sources refused before allocation")
                }
            };
            if sources.len() >= admission.name_use_count() {
                unreachable!(
                    "complete private admission counts this same immutable dependency iterator"
                );
            }
            sources.push(FormulaScalarRecipeSource {
                dependency,
                kind,
                read: None,
            });
        }
        Ok(FormulaScalarRecipePlan {
            admission,
            inputs,
            sources,
        })
    }
}
