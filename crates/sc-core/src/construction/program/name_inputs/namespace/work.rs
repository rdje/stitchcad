//! Complete prospective mixed-order checking work before any child checker or proof reservation.
use super::DraftingProgramInitialNamespace;
use crate::{
    construction::{
        DraftingInitialInputProvider, DraftingProgramStepKind, NormalizedDraftingOperation,
    },
    design::DesignMetadataHost,
    recipe::{FormulaNormalizedExpression, FormulaNormalizedStatementKind},
};
use core::fmt;

/// Explicit additional mixed-order checker proof allowances,without defaults.
/// All actual scalar operands and native arguments count,including every branch/repetition.
/// Retained records conservatively cover steps+statements+expressions+2*nodes:complete outer
/// slots,statement/expression proof owners,node certificates and at most one dependency per node.
/// Existing held content/name-map/output/initial records retain their separate admitted policies.
/// Root formula-source checking,checker scratch,allocator bytes/capacity,CPU and runtime are separate.
/// ```compile_fail
/// let limits:sc_core::construction::DraftingProgramOrderLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DraftingProgramOrderLimits {
    expressions: usize,
    nodes: usize,
    retained_records: usize,
}
impl DraftingProgramOrderLimits {
    /// Select every additional proof allowance;zero permits an empty mixed programme.
    #[must_use]
    pub const fn new(expressions: usize, nodes: usize, retained_records: usize) -> Self {
        Self {
            expressions,
            nodes,
            retained_records,
        }
    }
    /// Complete mixed scalar-operand/native-argument allowance.
    #[must_use]
    pub const fn expressions(self) -> usize {
        self.expressions
    }
    /// Complete actual normalized semantic-node allowance.
    #[must_use]
    pub const fn nodes(self) -> usize {
        self.nodes
    }
    /// Complete additional retained-checker-record allowance.
    #[must_use]
    pub const fn retained_records(self) -> usize {
        self.retained_records
    }
}
/// Complete actual normalized operand population and conservative future checker records.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DraftingProgramOrderShape {
    steps: usize,
    statements: usize,
    expressions: usize,
    nodes: usize,
    retained_records: usize,
}
impl DraftingProgramOrderShape {
    /// Every genuine mixed slot,including native inputs without scalar documents.
    #[must_use]
    pub const fn step_count(self) -> usize {
        self.steps
    }
    /// Every actual scalar statement,including assertions.
    #[must_use]
    pub const fn statement_count(self) -> usize {
        self.statements
    }
    /// Every scalar operand and native argument occurrence.
    #[must_use]
    pub const fn expression_count(self) -> usize {
        self.expressions
    }
    /// Every node in those actual normalized arenas,without deduplication.
    #[must_use]
    pub const fn node_count(self) -> usize {
        self.nodes
    }
    /// Conservative complete steps+statements+expressions+2*nodes proof records.
    #[must_use]
    pub const fn retained_record_bound(self) -> usize {
        self.retained_records
    }
    fn add<const N: usize>(
        &mut self,
        expressions: [&FormulaNormalizedExpression<'_>; N],
    ) -> Option<()> {
        self.expressions = self.expressions.checked_add(expressions.len())?;
        for expression in expressions {
            self.nodes = self.nodes.checked_add(expression.node_count())?;
        }
        Some(())
    }
}
/// First complete checked population/quota refusal;no child checker has run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftingProgramOrderWorkRefusal {
    /// Complete actual population arithmetic cannot be represented.
    PopulationOverflow,
    /// Complete actual expression occurrences exceed the selected allowance.
    Expressions {
        /// Selected maximum.
        allowed: usize,
        /// Complete actual requirement.
        required: usize,
    },
    /// Complete actual semantic nodes exceed the selected allowance.
    Nodes {
        /// Selected maximum.
        allowed: usize,
        /// Complete actual requirement.
        required: usize,
    },
    /// Complete conservative additional proof records exceed the selected allowance.
    RetainedRecords {
        /// Selected maximum.
        allowed: usize,
        /// Complete conservative requirement.
        required: usize,
    },
}
/// Original private initial map,complete input/policy and first order-work refusal.
pub struct DraftingProgramOrderWorkError<
    'w,
    's,
    'i,
    'r,
    'b,
    'n,
    'p,
    'a,
    H: DesignMetadataHost,
    S: DraftingInitialInputProvider<H> + ?Sized,
> {
    inputs: &'w DraftingProgramInitialNamespace<'s, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
    limits: &'w DraftingProgramOrderLimits,
    refusal: DraftingProgramOrderWorkRefusal,
}
impl<
        'w,
        's,
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignMetadataHost,
        S: DraftingInitialInputProvider<H> + ?Sized,
    > DraftingProgramOrderWorkError<'w, 's, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Exact complete original private initial namespace and its full input/source owners.
    #[must_use]
    pub const fn inputs(
        &self,
    ) -> &'w DraftingProgramInitialNamespace<'s, 'i, 'r, 'b, 'n, 'p, 'a, H, S> {
        self.inputs
    }
    /// Original selected policy;no derived/default replacement.
    #[must_use]
    pub const fn limits(&self) -> &'w DraftingProgramOrderLimits {
        self.limits
    }
    /// First complete arithmetic/quota refusal,with actual required population.
    #[must_use]
    pub const fn refusal(&self) -> DraftingProgramOrderWorkRefusal {
        self.refusal
    }
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> fmt::Debug
    for DraftingProgramOrderWorkError<'_, '_, '_, '_, '_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DraftingProgramOrderWorkError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> fmt::Display
    for DraftingProgramOrderWorkError<'_, '_, '_, '_, '_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "drafting order work: {:?}", self.refusal)
    }
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> std::error::Error
    for DraftingProgramOrderWorkError<'_, '_, '_, '_, '_, '_, '_, '_, H, S>
{
}

/// Complete additional order-checker bounds under the actual original normalized owners.
/// This grants no checked kind/order/dependency,metadata advance,value or executable prefix.
/// ```compile_fail,E0616
/// use sc_core::{construction::{AdmittedDraftingProgramOrderWork,DraftingInitialInputProvider},design::DesignMetadataHost};
/// fn alter<H:DesignMetadataHost,S:DraftingInitialInputProvider<H>>(
///     work:&mut AdmittedDraftingProgramOrderWork<'_, '_, '_, '_, '_, '_, '_, '_,H,S>) {work.shape=todo!();}
/// ```
pub struct AdmittedDraftingProgramOrderWork<
    'w,
    's,
    'i,
    'r,
    'b,
    'n,
    'p,
    'a,
    H: DesignMetadataHost,
    S: DraftingInitialInputProvider<H> + ?Sized,
> {
    inputs: &'w DraftingProgramInitialNamespace<'s, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
    limits: &'w DraftingProgramOrderLimits,
    shape: DraftingProgramOrderShape,
}
impl<
        'w,
        's,
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignMetadataHost,
        S: DraftingInitialInputProvider<H> + ?Sized,
    > AdmittedDraftingProgramOrderWork<'w, 's, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Exact original private initial namespace and complete original bound input owners.
    #[must_use]
    pub const fn inputs(
        &self,
    ) -> &'w DraftingProgramInitialNamespace<'s, 'i, 'r, 'b, 'n, 'p, 'a, H, S> {
        self.inputs
    }
    /// Original explicit additional checker policy.
    #[must_use]
    pub const fn limits(&self) -> &'w DraftingProgramOrderLimits {
        self.limits
    }
    /// Complete actual normalized cohort and conservative prospective proof records.
    #[must_use]
    pub const fn shape(&self) -> DraftingProgramOrderShape {
        self.shape
    }
}
impl<H: DesignMetadataHost, S: DraftingInitialInputProvider<H> + ?Sized> fmt::Debug
    for AdmittedDraftingProgramOrderWork<'_, '_, '_, '_, '_, '_, '_, '_, H, S>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AdmittedDraftingProgramOrderWork")
            .field("shape", &self.shape)
            .finish_non_exhaustive()
    }
}
impl<
        's,
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignMetadataHost,
        S: DraftingInitialInputProvider<H> + ?Sized,
    > DraftingProgramInitialNamespace<'s, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Count every actual mixed scalar operand/native argument before any child checker or outer
    /// proof allocation. The original arenas,including all branches and repeated native occurrences,
    /// supply node counts;prospective normalization totals or caller hints never substitute.
    /// Quotas follow complete checked arithmetic,then expressions,nodes,retained records in order.
    /// No source callback,value,namespace insertion,kind check or static proof prefix occurs.
    ///
    /// ```
    /// use sc_core::{construction::{DraftingProgramInitialNamespace,DraftingInitialInputProvider,
    ///     DraftingProgramOrderLimits,DraftingProgramOrderShape},design::DesignMetadataHost};
    /// fn order_shape<H:DesignMetadataHost,S:DraftingInitialInputProvider<H> + ?Sized>(
    ///     names:&DraftingProgramInitialNamespace<'_, '_, '_, '_, '_, '_, '_,H,S>,
    ///     policy:DraftingProgramOrderLimits)->Result<DraftingProgramOrderShape,String> {
    ///     names.admit_order_work(&policy).map(|work|work.shape()).map_err(|error|error.to_string())
    /// }
    /// ```
    /// # Errors
    /// Complete arithmetic or first quota refusal with original private namespace/input and policy.
    pub fn admit_order_work<'w>(
        &'w self,
        limits: &'w DraftingProgramOrderLimits,
    ) -> Result<
        AdmittedDraftingProgramOrderWork<'w, 's, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
        DraftingProgramOrderWorkError<'w, 's, 'i, 'r, 'b, 'n, 'p, 'a, H, S>,
    > {
        let refuse = |refusal| DraftingProgramOrderWorkError {
            inputs: self,
            limits,
            refusal,
        };
        let overflow = || refuse(DraftingProgramOrderWorkRefusal::PopulationOverflow);
        let program = self.inputs().roles().positions().work().inputs().program();
        let mut shape = DraftingProgramOrderShape {
            steps: program.steps().len(),
            statements: 0,
            expressions: 0,
            nodes: 0,
            retained_records: 0,
        };
        for step in program.steps() {
            match step.kind() {
                DraftingProgramStepKind::Scalar { statement, .. } => {
                    shape.statements = shape.statements.checked_add(1).ok_or_else(overflow)?;
                    match statement.kind() {
                        FormulaNormalizedStatementKind::Let { expression, .. } => {
                            shape.add([expression])
                        }
                        FormulaNormalizedStatementKind::Assert { left, right, .. } => {
                            shape.add([left, right])
                        }
                    }
                    .ok_or_else(overflow)?;
                }
                DraftingProgramStepKind::Drafting(operation) => match operation {
                    NormalizedDraftingOperation::Point(point) => {
                        shape.add(point.resource_expressions())
                    }
                    NormalizedDraftingOperation::CenterArc(arc) => {
                        shape.add(arc.resource_expressions())
                    }
                    NormalizedDraftingOperation::SlashSpread { input, opening } => {
                        crate::design::fixed_value(input);
                        shape.add(opening.resource_expressions())
                    }
                    NormalizedDraftingOperation::Other(input) => {
                        crate::design::fixed_value(input);
                        Some(())
                    }
                }
                .ok_or_else(overflow)?,
            }
        }
        shape.retained_records = shape
            .nodes
            .checked_mul(2)
            .and_then(|n| n.checked_add(shape.expressions))
            .and_then(|n| n.checked_add(shape.statements))
            .and_then(|n| n.checked_add(shape.steps))
            .ok_or_else(overflow)?;
        let refusal = if shape.expressions > limits.expressions {
            Some(DraftingProgramOrderWorkRefusal::Expressions {
                allowed: limits.expressions,
                required: shape.expressions,
            })
        } else if shape.nodes > limits.nodes {
            Some(DraftingProgramOrderWorkRefusal::Nodes {
                allowed: limits.nodes,
                required: shape.nodes,
            })
        } else if shape.retained_records > limits.retained_records {
            Some(DraftingProgramOrderWorkRefusal::RetainedRecords {
                allowed: limits.retained_records,
                required: shape.retained_records,
            })
        } else {
            None
        };
        if let Some(refusal) = refusal {
            return Err(refuse(refusal));
        }
        Ok(AdmittedDraftingProgramOrderWork {
            inputs: self,
            limits,
            shape,
        })
    }
}
