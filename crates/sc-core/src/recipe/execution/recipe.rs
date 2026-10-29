//! Whole checked-recipe admission only; no runtime prefix or value is constructed here.
use crate::recipe::{FormulaCheckedRecipe, FormulaCheckedStatementKind};
use core::fmt;

/// Explicit whole-recipe logical allowances, independent of the fixed language syntax bounds.
/// Workspace excludes output records: both populations must fit their own selected allowance.
/// These limits do not bound allocator bytes, numeric kernels, provider resources or CPU.
/// ```compile_fail
/// let limits: sc_core::recipe::FormulaRecipeExecutionLimits = Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormulaRecipeExecutionLimits {
    statements: usize,
    nodes: usize,
    workspace_records: usize,
    output_records: usize,
}
impl FormulaRecipeExecutionLimits {
    /// Select every allowance explicitly; zero admits only an empty corresponding population.
    #[must_use]
    pub const fn new(
        statements: usize,
        nodes: usize,
        workspace_records: usize,
        output_records: usize,
    ) -> Self {
        Self {
            statements,
            nodes,
            workspace_records,
            output_records,
        }
    }
    /// Maximum complete statement population, including assertions.
    #[must_use]
    pub const fn statements(self) -> usize {
        self.statements
    }
    /// Maximum sum of nodes across every operand and branch in the complete recipe.
    #[must_use]
    pub const fn nodes(self) -> usize {
        self.nodes
    }
    /// Maximum complete `2 * nodes + name_uses` logical workspace reservation.
    #[must_use]
    pub const fn workspace_records(self) -> usize {
        self.workspace_records
    }
    /// Maximum retained statement outcomes, one per let or assertion.
    #[must_use]
    pub const fn output_records(self) -> usize {
        self.output_records
    }
}

/// Complete-recipe resource refusal, separate from canonical formula-language diagnostics.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormulaRecipeExecutionAdmissionRefusal {
    /// Complete authored statement population exceeds its explicit allowance; checked first.
    Statements {
        /// Explicit selected allowance.
        allowed: usize,
        /// Complete actual population.
        actual: usize,
    },
    /// A complete population or its workspace formula cannot be represented.
    /// Fixed language bounds currently make this defensive refusal unreachable by valid syntax.
    PopulationCountOverflow,
    /// All operand nodes, including both assertion operands and every branch, exceed the quota.
    Nodes {
        /// Explicit selected allowance.
        allowed: usize,
        /// Complete actual population.
        actual: usize,
    },
    /// The full output population cannot fit; no accepted statement prefix is returned.
    OutputRecords {
        /// Explicit selected allowance.
        allowed: usize,
        /// One outcome per actual statement.
        required: usize,
    },
    /// The complete `2 * nodes + name_uses` reservation cannot fit.
    WorkspaceRecords {
        /// Explicit selected allowance.
        allowed: usize,
        /// Complete prospective reservation.
        required: usize,
    },
}

/// Refusal retaining the exact whole proof and selected limits; no invented failing ordinal.
#[derive(Debug)]
pub struct FormulaRecipeExecutionAdmissionError<'p, 'a> {
    proof: &'p FormulaCheckedRecipe<'a>,
    limits: FormulaRecipeExecutionLimits,
    refusal: FormulaRecipeExecutionAdmissionRefusal,
}
impl<'p, 'a> FormulaRecipeExecutionAdmissionError<'p, 'a> {
    /// Actual successful static whole proof, including every statement after any runtime gap.
    #[must_use]
    pub const fn proof(&self) -> &'p FormulaCheckedRecipe<'a> {
        self.proof
    }
    /// Actual selected allowances, without fallback defaults.
    #[must_use]
    pub const fn limits(&self) -> FormulaRecipeExecutionLimits {
        self.limits
    }
    /// Exact quota and measured complete population, or defensive arithmetic refusal.
    #[must_use]
    pub const fn refusal(&self) -> FormulaRecipeExecutionAdmissionRefusal {
        self.refusal
    }
}
impl fmt::Display for FormulaRecipeExecutionAdmissionError<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "whole formula recipe execution admission: {:?}",
            self.refusal
        )
    }
}
impl std::error::Error for FormulaRecipeExecutionAdmissionError<'_, '_> {}

/// Allocation-free reservation over one actual complete immutable recipe proof.
/// All name occurrences count, including assertion tolerance headers and untaken branches.
/// Reserves N task frames, N value slots and D source records, plus S separate output records.
/// This is no input-currentness, backend-readiness, executed-prefix or accepted-result certificate.
/// ```compile_fail
/// fn forge(admission: &mut sc_core::recipe::FormulaRecipeExecutionAdmission<'_, '_>) {
///     admission.node_count = 0;
/// }
/// ```
/// ```compile_fail
/// fn escape<'a>(recipe: &'a sc_core::recipe::FormulaNormalizedRecipe<'a>)
///     -> sc_core::recipe::FormulaRecipeExecutionAdmission<'a, 'a> {
///     use sc_core::recipe::*;
///     let proof = recipe.check_kinds(FormulaNamespace::new([]).unwrap()).unwrap();
///     proof.admit_execution(FormulaRecipeExecutionLimits::new(4096, 2097152, 8388608, 4096)).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct FormulaRecipeExecutionAdmission<'p, 'a> {
    proof: &'p FormulaCheckedRecipe<'a>,
    limits: FormulaRecipeExecutionLimits,
    node_count: usize,
    name_use_count: usize,
    binding_count: usize,
    workspace_records: usize,
}
impl<'p, 'a> FormulaRecipeExecutionAdmission<'p, 'a> {
    /// Original complete proof: its actual statements, spans, ordinals and dependencies stay intact.
    #[must_use]
    pub const fn proof(&self) -> &'p FormulaCheckedRecipe<'a> {
        self.proof
    }
    /// Exact selected allowances; admission does not widen them.
    #[must_use]
    pub const fn limits(&self) -> FormulaRecipeExecutionLimits {
        self.limits
    }
    /// Complete statement population, including assertions.
    #[must_use]
    pub fn statement_count(&self) -> usize {
        self.proof.statements().len()
    }
    /// Number of prospective successful binding outcomes, without performing a conversion.
    #[must_use]
    pub const fn binding_count(&self) -> usize {
        self.binding_count
    }
    /// Number of prospective assertion outcomes, without reading a tolerance or comparing values.
    #[must_use]
    pub fn assertion_count(&self) -> usize {
        self.statement_count() - self.binding_count
    }
    /// Sum of all certified operand nodes; assertion headers are not expression nodes.
    #[must_use]
    pub const fn node_count(&self) -> usize {
        self.node_count
    }
    /// Every dependency occurrence, including the actual tolerance header of each assertion.
    #[must_use]
    pub const fn name_use_count(&self) -> usize {
        self.name_use_count
    }
    /// Complete checked `2 * nodes + name_uses` workspace reservation, excluding outputs.
    #[must_use]
    pub const fn workspace_records(&self) -> usize {
        self.workspace_records
    }
    /// One prospective retained outcome per statement, independently bounded from workspace.
    #[must_use]
    pub fn output_records(&self) -> usize {
        self.statement_count()
    }
}

impl<'a> FormulaCheckedRecipe<'a> {
    /// Admit complete populations before runtime allocation or value/provider reads.
    /// Only a successful whole-recipe static proof supplies statements, counts and original sources.
    /// ```
    /// use sc_core::recipe::*;
    /// let recipe = FormulaRecipe::parse("let n:count=1\nassert check:eps_num=n==n")?
    ///     .normalize_literals()?;
    /// let proof = recipe.check_kinds(FormulaNamespace::new([])?).expect("whole static proof");
    /// let admission = proof.admit_execution(FormulaRecipeExecutionLimits::new(2, 3, 9, 2))
    ///     .expect("exact complete allowance");
    /// assert_eq!((admission.binding_count(), admission.assertion_count()), (1, 1));
    /// assert_eq!(admission.name_use_count(), 3); // two n occurrences and the assertion header
    /// assert!(std::ptr::eq(admission.proof(), &proof));
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    /// # Errors
    /// Refusal order: statements, checked population/workspace arithmetic, nodes, outputs, workspace.
    /// No allocation, execution session, partial accepted output or language diagnostic is produced.
    pub fn admit_execution(
        &self,
        limits: FormulaRecipeExecutionLimits,
    ) -> Result<FormulaRecipeExecutionAdmission<'_, 'a>, FormulaRecipeExecutionAdmissionError<'_, 'a>>
    {
        use FormulaRecipeExecutionAdmissionRefusal as R;
        let refuse = |refusal| FormulaRecipeExecutionAdmissionError {
            proof: self,
            limits,
            refusal,
        };
        let statements = self.statements().len();
        if statements > limits.statements {
            return Err(refuse(R::Statements {
                allowed: limits.statements,
                actual: statements,
            }));
        }
        let mut node_count = 0usize;
        let mut binding_count = 0usize;
        for statement in self.statements() {
            match statement.kind() {
                FormulaCheckedStatementKind::Let { expression } => {
                    binding_count = binding_count
                        .checked_add(1)
                        .ok_or_else(|| refuse(R::PopulationCountOverflow))?;
                    node_count = node_count
                        .checked_add(expression.node_count())
                        .ok_or_else(|| refuse(R::PopulationCountOverflow))?;
                }
                FormulaCheckedStatementKind::Assert { left, right } => {
                    node_count = node_count
                        .checked_add(left.node_count())
                        .and_then(|n| n.checked_add(right.node_count()))
                        .ok_or_else(|| refuse(R::PopulationCountOverflow))?;
                }
            }
        }
        let name_use_count = self
            .dependencies()
            .try_fold(0usize, |n, _| n.checked_add(1))
            .ok_or_else(|| refuse(R::PopulationCountOverflow))?;
        let workspace_records = node_count
            .checked_mul(2)
            .and_then(|n| n.checked_add(name_use_count))
            .ok_or_else(|| refuse(R::PopulationCountOverflow))?;
        if node_count > limits.nodes {
            return Err(refuse(R::Nodes {
                allowed: limits.nodes,
                actual: node_count,
            }));
        }
        if statements > limits.output_records {
            return Err(refuse(R::OutputRecords {
                allowed: limits.output_records,
                required: statements,
            }));
        }
        if workspace_records > limits.workspace_records {
            return Err(refuse(R::WorkspaceRecords {
                allowed: limits.workspace_records,
                required: workspace_records,
            }));
        }
        Ok(FormulaRecipeExecutionAdmission {
            proof: self,
            limits,
            node_count,
            name_use_count,
            binding_count,
            workspace_records,
        })
    }
}
