//! Complete available scalar source preparation; bare source roots are not stored let bindings.
use super::{
    plan::scalar_builtin_available, FormulaExecutionAdmissionError, FormulaExecutionLimits,
    FormulaScalarExecutionPlan, FormulaScalarExecutionPlanError,
};
use crate::recipe::{
    FormulaAcyclicSourceDependencies, FormulaBuiltin, FormulaCheckedNode,
    FormulaCheckedNodeForm as Form, FormulaCheckedSource, FormulaDeclarationSource, FormulaKind,
};
use core::fmt;
use std::collections::TryReserveError;

/// Explicit complete scalar-source populations and prospective logical runtime allowances.
/// For S sources,N nodes,D names,workspace is `2S + 2N + D`:source control records,graph call
/// frames,task/value buffers and original read slots. Up to S output records have a separate quota.
/// Retained graph/proof data,allocator bytes/capacity,numeric backends and CPU are separate.
/// ```compile_fail
/// let limits: sc_core::recipe::FormulaSourceGraphExecutionLimits = Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormulaSourceGraphExecutionLimits {
    sources: usize,
    nodes: usize,
    name_uses: usize,
    workspace_records: usize,
    output_records: usize,
}
impl FormulaSourceGraphExecutionLimits {
    /// Select all complete allowances explicitly;empty sources fit zero populations.
    #[must_use]
    pub const fn new(
        sources: usize,
        nodes: usize,
        name_uses: usize,
        workspace_records: usize,
        output_records: usize,
    ) -> Self {
        Self {
            sources,
            nodes,
            name_uses,
            workspace_records,
            output_records,
        }
    }
    /// Maximum complete source population,including unused sources.
    #[must_use]
    pub const fn sources(self) -> usize {
        self.sources
    }
    /// Maximum complete actual source node population,including every branch.
    #[must_use]
    pub const fn nodes(self) -> usize {
        self.nodes
    }
    /// Maximum original name occurrences,including repetitions/untaken uses.
    #[must_use]
    pub const fn name_uses(self) -> usize {
        self.name_uses
    }
    /// Maximum prospective `2S + 2N + D` runtime workspace reservation.
    #[must_use]
    pub const fn workspace_records(self) -> usize {
        self.workspace_records
    }
    /// Maximum prospective source outputs;complete preparation requires room for S.
    #[must_use]
    pub const fn output_records(self) -> usize {
        self.output_records
    }
}

/// Preparation/resource failure,distinct from language diagnostics or a computed graph result.
#[derive(Debug)]
pub enum FormulaScalarSourceGraphPlanRefusal<'a> {
    /// Complete source quota,checked first.
    SourceCount {
        /// Selected allowance.
        allowed: usize,
        /// Complete actual population.
        required: usize,
    },
    /// Complete population or prospective runtime reservation cannot be represented.
    PopulationOverflow,
    /// Complete actual normalized node quota.
    Nodes {
        /// Selected allowance.
        allowed: usize,
        /// Complete actual population.
        required: usize,
    },
    /// Complete original name-use quota.
    NameUses {
        /// Selected allowance.
        allowed: usize,
        /// Complete actual population.
        required: usize,
    },
    /// Complete prospective source-output quota.
    OutputRecords {
        /// Selected allowance.
        allowed: usize,
        /// Complete prospective population.
        required: usize,
    },
    /// Complete prospective runtime workspace quota.
    WorkspaceRecords {
        /// Selected allowance.
        allowed: usize,
        /// Complete prospective reservation.
        required: usize,
    },
    /// Root is an actual non-scalar kind,not a numeric implementation in this scope.
    UnavailableRootKind {
        /// Original authored source index.
        source_index: usize,
        /// Actual source root,without a fabricated statement.
        node: FormulaCheckedNode<'a, 'a>,
        /// Actual inferred root kind.
        actual: FormulaKind,
    },
    /// Actual tangent/geometry implementation is absent from the shared scalar catalog.
    UnavailableBuiltin {
        /// Original authored source index.
        source_index: usize,
        /// Actual original certified call node.
        node: FormulaCheckedNode<'a, 'a>,
        /// Actual unavailable builtin.
        builtin: FormulaBuiltin,
    },
    /// Actual geometry/prior-let metadata requires its real runtime integration.
    UnavailableSource {
        /// Original authored source index.
        source_index: usize,
        /// Actual original certified name node.
        node: FormulaCheckedNode<'a, 'a>,
    },
    /// Original outer source-control vector reservation cause;no new diagnostic allocation.
    Allocation(TryReserveError),
    /// Original per-source read-slot reservation cause,extracted without allocating a box.
    SourceAllocation {
        /// Original source index.
        source_index: usize,
        /// Actual complete source name-use population requested.
        requested: usize,
        /// Original allocator cause.
        error: TryReserveError,
    },
    /// Original actual expression admission refusal,not a substituted source annotation.
    ExpressionAdmission {
        /// Original source index.
        source_index: usize,
        /// Original semantic cause;allocation never occurs in this factory.
        error: Box<FormulaExecutionAdmissionError<'a, 'a>>,
    },
    /// Original semantic canonical/source preparation cause;allocator errors remain inline above.
    SourcePreparation {
        /// Original source index.
        source_index: usize,
        /// Original typed cause with actual node/inventory witnesses.
        error: Box<FormulaScalarExecutionPlanError<'a>>,
    },
}

/// Refusal pinned to the actual acyclic relation,source proofs and selected complete allowances.
#[derive(Debug)]
pub struct FormulaScalarSourceGraphPlanError<'a> {
    graph: &'a FormulaAcyclicSourceDependencies<'a>,
    limits: FormulaSourceGraphExecutionLimits,
    refusal: FormulaScalarSourceGraphPlanRefusal<'a>,
}
impl<'a> FormulaScalarSourceGraphPlanError<'a> {
    /// Actual complete acyclic canonical Derived relation and its original source/input owners.
    #[must_use]
    pub const fn graph(&self) -> &'a FormulaAcyclicSourceDependencies<'a> {
        self.graph
    }
    /// Selected complete allowances,without defaults.
    #[must_use]
    pub const fn limits(&self) -> FormulaSourceGraphExecutionLimits {
        self.limits
    }
    /// Original complete resource/capability/source cause.
    #[must_use]
    pub const fn refusal(&self) -> &FormulaScalarSourceGraphPlanRefusal<'a> {
        &self.refusal
    }
}
impl fmt::Display for FormulaScalarSourceGraphPlanError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "scalar source graph preparation: {:?}", self.refusal)
    }
}
impl std::error::Error for FormulaScalarSourceGraphPlanError<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.refusal {
            FormulaScalarSourceGraphPlanRefusal::Allocation(error)
            | FormulaScalarSourceGraphPlanRefusal::SourceAllocation { error, .. } => Some(error),
            // Other causes borrow original proofs;refusal() retains their exact typed witnesses.
            _ => None,
        }
    }
}

/// Actual original bare source and its existing scalar expression plan,without any values.
#[derive(Debug)]
pub struct FormulaScalarSourceGraphSource<'a> {
    source: &'a FormulaCheckedSource<'a>,
    plan: FormulaScalarExecutionPlan<'a>,
}
impl<'a> FormulaScalarSourceGraphSource<'a> {
    /// Actual original normalized source and complete kind proof.
    #[must_use]
    pub const fn source(&self) -> &'a FormulaCheckedSource<'a> {
        self.source
    }
    /// Actual existing complete source plan;shared views cannot consume or seed its execution.
    #[must_use]
    pub const fn plan(&self) -> &FormulaScalarExecutionPlan<'a> {
        &self.plan
    }
}

/// Complete prepared available scalar source collection,with actual acyclic/source/input owners.
/// No source expression has been evaluated or bound. Bare roots remain unbound;only a real let
/// may invoke actual storage. Runtime resumption/earned caches/recipe integration remain separate.
/// ```compile_fail
/// fn clear(plan:&mut sc_core::recipe::FormulaScalarSourceGraphPlan<'_>) {plan.sources.clear();}
/// ```
/// ```compile_fail
/// fn escape<'a>(deps:&'a sc_core::recipe::FormulaSourceDependencies<'a>)
///     ->sc_core::recipe::FormulaScalarSourceGraphPlan<'a> {
///     use sc_core::recipe::{FormulaSourceCycleLimits,FormulaSourceGraphExecutionLimits};
///     let graph=deps.admit_acyclic(FormulaSourceCycleLimits::new(usize::MAX,usize::MAX)).unwrap();
///     graph.prepare_scalar_sources(FormulaSourceGraphExecutionLimits::new(
///         usize::MAX,usize::MAX,usize::MAX,usize::MAX,usize::MAX)).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct FormulaScalarSourceGraphPlan<'a> {
    graph: &'a FormulaAcyclicSourceDependencies<'a>,
    limits: FormulaSourceGraphExecutionLimits,
    nodes: usize,
    name_uses: usize,
    workspace_records: usize,
    sources: Vec<FormulaScalarSourceGraphSource<'a>>,
}
impl<'a> FormulaScalarSourceGraphPlan<'a> {
    /// Actual complete acyclic relation;no substituted input/source snapshot.
    #[must_use]
    pub const fn graph(&self) -> &'a FormulaAcyclicSourceDependencies<'a> {
        self.graph
    }
    /// Explicit selected complete allowances.
    #[must_use]
    pub const fn limits(&self) -> FormulaSourceGraphExecutionLimits {
        self.limits
    }
    /// Complete actual source node count.
    #[must_use]
    pub const fn node_count(&self) -> usize {
        self.nodes
    }
    /// Complete original name-use count,including repeated/untaken occurrences.
    #[must_use]
    pub const fn name_use_count(&self) -> usize {
        self.name_uses
    }
    /// Complete admitted prospective `2S + 2N + D` runtime reservation.
    #[must_use]
    pub const fn workspace_records(&self) -> usize {
        self.workspace_records
    }
    /// Maximum actual source output population charged before preparation;not computed results.
    #[must_use]
    pub fn output_records(&self) -> usize {
        self.sources.len()
    }
    /// Every actual original source and prepared scalar plan in original source order.
    #[must_use]
    pub fn sources(&self) -> &[FormulaScalarSourceGraphSource<'a>] {
        &self.sources
    }
}
impl<'s> FormulaAcyclicSourceDependencies<'s> {
    /// Prepare all available scalar sources after complete populations and capability checking.
    /// Every branch/unused source participates. Existing per-expression source factories remain
    /// authoritative. No canonical value,context provider,let binding or execution order is read/
    /// created. Prospective runtime scratch/output allowances are retained for the actual worker.
    /// # Errors
    /// Source quota,checked populations,node/name/output/workspace quotas precede capability
    /// scanning;complete capability scanning precedes the outer vector and any per-source plan.
    /// Actual unavailable nodes and original factory causes survive;no accepted prefix or fake
    /// language refusal for an unavailable implementation is produced. Allocator causes stay inline.
    pub fn prepare_scalar_sources<'a>(
        &'a self,
        limits: FormulaSourceGraphExecutionLimits,
    ) -> Result<FormulaScalarSourceGraphPlan<'a>, FormulaScalarSourceGraphPlanError<'a>>
    where
        's: 'a,
    {
        use FormulaScalarSourceGraphPlanRefusal as Refusal;
        let graph: &'a FormulaAcyclicSourceDependencies<'a> = self;
        let refuse = |refusal| FormulaScalarSourceGraphPlanError {
            graph,
            limits,
            refusal,
        };
        let checked = graph.dependencies().checked();
        let count = checked.sources().len();
        if count > limits.sources {
            return Err(refuse(Refusal::SourceCount {
                allowed: limits.sources,
                required: count,
            }));
        }
        let mut nodes = 0usize;
        let mut name_uses = 0usize;
        for source in checked.sources() {
            nodes = nodes
                .checked_add(source.proof().node_count())
                .ok_or_else(|| refuse(Refusal::PopulationOverflow))?;
            name_uses = name_uses
                .checked_add(source.proof().dependencies().len())
                .ok_or_else(|| refuse(Refusal::PopulationOverflow))?;
        }
        let workspace_records = count
            .checked_mul(2)
            .and_then(|control| {
                nodes
                    .checked_mul(2)
                    .and_then(|scratch| control.checked_add(scratch))
            })
            .and_then(|records| records.checked_add(name_uses))
            .ok_or_else(|| refuse(Refusal::PopulationOverflow))?;
        if nodes > limits.nodes {
            return Err(refuse(Refusal::Nodes {
                allowed: limits.nodes,
                required: nodes,
            }));
        }
        if name_uses > limits.name_uses {
            return Err(refuse(Refusal::NameUses {
                allowed: limits.name_uses,
                required: name_uses,
            }));
        }
        if count > limits.output_records {
            return Err(refuse(Refusal::OutputRecords {
                allowed: limits.output_records,
                required: count,
            }));
        }
        if workspace_records > limits.workspace_records {
            return Err(refuse(Refusal::WorkspaceRecords {
                allowed: limits.workspace_records,
                required: workspace_records,
            }));
        }
        for (source_index, source) in checked.sources().iter().enumerate() {
            let proof = source.proof();
            if proof.kind().binding_kind().is_none() {
                return Err(refuse(Refusal::UnavailableRootKind {
                    source_index,
                    node: proof.root(),
                    actual: proof.kind(),
                }));
            }
            for node in proof.root().all_nodes() {
                match node.form() {
                    Form::Call { builtin, .. } if !scalar_builtin_available(builtin) => {
                        return Err(refuse(Refusal::UnavailableBuiltin {
                            source_index,
                            node,
                            builtin,
                        }))
                    }
                    Form::Name { declaration, .. } => match declaration.source() {
                        FormulaDeclarationSource::Point(_)
                        | FormulaDeclarationSource::Edge(_)
                        | FormulaDeclarationSource::Recipe { .. } => {
                            return Err(refuse(Refusal::UnavailableSource { source_index, node }))
                        }
                        FormulaDeclarationSource::Input { .. }
                        | FormulaDeclarationSource::LengthInput { .. }
                        | FormulaDeclarationSource::Reserved(_) => {}
                    },
                    Form::Call { .. }
                    | Form::Literal(_)
                    | Form::Negate(_)
                    | Form::Square(_)
                    | Form::Binary { .. }
                    | Form::Conditional { .. } => {}
                }
            }
        }
        let mut sources = Vec::new();
        sources
            .try_reserve_exact(count)
            .map_err(|error| refuse(Refusal::Allocation(error)))?;
        let inputs = checked.normalized().inputs().inputs();
        for (source_index, source) in checked.sources().iter().enumerate() {
            let n = source.proof().node_count();
            let d = source.proof().dependencies().len();
            let records = n
                .checked_mul(2)
                .and_then(|n| n.checked_add(d))
                .ok_or_else(|| refuse(Refusal::PopulationOverflow))?;
            let admission = source
                .proof()
                .admit_execution(FormulaExecutionLimits::new(n, records))
                .map_err(|error| {
                    refuse(Refusal::ExpressionAdmission {
                        source_index,
                        error: Box::new(error),
                    })
                })?;
            let plan = admission.prepare_scalar(inputs).map_err(|error| {
                match error.into_allocation_error() {
                    Ok(error) => refuse(Refusal::SourceAllocation {
                        source_index,
                        requested: d,
                        error,
                    }),
                    Err(error) => refuse(Refusal::SourcePreparation {
                        source_index,
                        error: Box::new(error),
                    }),
                }
            })?;
            sources.push(FormulaScalarSourceGraphSource { source, plan });
        }
        Ok(FormulaScalarSourceGraphPlan {
            graph,
            limits,
            nodes,
            name_uses,
            workspace_records,
            sources,
        })
    }
}
