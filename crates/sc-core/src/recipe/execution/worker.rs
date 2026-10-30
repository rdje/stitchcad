//! Iterative execution consumes actual checked node proofs and bound sources in authored order.
use super::{
    machine::{Machine, ReadValue, Sources},
    read, FormulaScalarExecutionContext as Context, FormulaScalarExecutionError as Error,
    FormulaScalarExecutionFailure as Failure, FormulaScalarExecutionPlan as Plan,
    FormulaScalarExecutionRead,
};
use crate::{
    recipe::{FormulaCheckedNode as Node, FormulaEvaluatedScalar as V},
    value::CanonicalInputs,
};
use core::fmt;

/// Actual executed scalar subset result with its original proof/inventory and selected read trace.
/// No binding rounding, statement publication, derived/geometry/tangent/whole recipe or approval
/// follows. The constructor is private; supplied values/branch annotations cannot create a result.
/// ```compile_fail
/// fn substitute(result: &mut sc_core::recipe::FormulaExecutedScalarExpression<'_>,
///     supplied: sc_core::recipe::FormulaEvaluatedScalar) { result.value = supplied; }
/// ```
pub struct FormulaExecutedScalarExpression<'a> {
    plan: Plan<'a>,
    value: V,
}
impl<'a> FormulaExecutedScalarExpression<'a> {
    /// Actual complete scalar source plan and original explicit workspace admission.
    #[must_use]
    pub const fn plan(&self) -> &Plan<'a> {
        &self.plan
    }
    /// Actual root proof/source owner, not a substituted normalized expression.
    #[must_use]
    pub fn root(&self) -> Node<'a, 'a> {
        self.plan.admission.root()
    }
    /// Actual immutable canonical input owner used by all bound canonical reads.
    #[must_use]
    pub const fn inputs(&self) -> &'a CanonicalInputs {
        self.plan.inputs()
    }
    /// Exact unbound result and all executed approximation-call contributions.
    #[must_use]
    pub const fn value(&self) -> V {
        self.value
    }
    /// Only executed name occurrences, in original execution order, including repetitions.
    /// Untaken branch slots remain unread; no complete-static-dependency prefix is substituted.
    pub fn reads(&self) -> impl DoubleEndedIterator<Item = FormulaScalarExecutionRead<'a>> + '_ {
        // Left-to-right execution with a selected-only conditional is a subsequence of the
        // checked proof's ordered name occurrences; each original node executes at most once.
        self.plan.sources.iter().filter_map(|slot| {
            let data = slot.read?;
            let Some(node) = slot.node else {
                unreachable!("private complete source plan retains each executed name node");
            };
            Some(FormulaScalarExecutionRead { node, data })
        })
    }
}
impl fmt::Debug for FormulaExecutedScalarExpression<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaExecutedScalarExpression")
            .field("root", &self.root())
            .field("kind", &self.value.kind())
            .field("read_count", &self.reads().count())
            .finish_non_exhaustive()
    }
}

impl<'p> Plan<'p> {
    /// Execute the actual prepared scalar subset with explicit immutable runtime contexts.
    /// Complete static/capability/canonical-source checking already succeeded. Both N buffers
    /// reserve fallibly before any read; guarded pushes forbid growth beyond admitted populations.
    /// The source plan's existing D slots retain only reads actually performed, without cloning
    /// source records. Existing kernel workspace/error payloads are outside these logical buffers.
    /// ```
    /// use sc_core::{recipe::*, value::CanonicalInputs};
    /// let inputs = CanonicalInputs::new(vec![], vec![])?;
    /// let expression = FormulaExpression::parse("if(1 mm<2 mm,1 mm/3,eps_phys)")?
    ///     .normalize_literals()?;
    /// let proof = expression.check_kinds(&FormulaNamespace::new([])?).expect("static fixture");
    /// let plan = proof.admit_execution(FormulaExecutionLimits::new(256, 768))
    ///     .expect("explicit envelope").prepare_scalar(&inputs).expect("scalar scope");
    /// let result = plan.evaluate(FormulaScalarExecutionContext::new(
    ///     FormulaToleranceContext::empty(), FormulaSizeContext::empty())).expect("taken exact branch");
    /// assert_eq!(result.value().numeric().expect("Length").rational(),
    ///     FormulaRational::new(1000, 3, false)?);
    /// assert_eq!(result.reads().count(), 0); // Untaken eps_phys was not read.
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    /// # Errors
    /// Retains the actual failing node and original allocation/value/provider cause. A failure
    /// exposes no accepted result/read prefix and changes no canonical input. Unknown needs its
    /// actual observation; Derived needs its sole formula's evaluation, without a fallback.
    pub fn evaluate<'a>(
        self,
        context: Context<'a>,
    ) -> Result<FormulaExecutedScalarExpression<'a>, Error<'a>>
    where
        'p: 'a,
    {
        let mut plan: Plan<'a> = self;
        let root = plan.admission.root();
        let mut machine = Machine::new(plan.admission.node_count()).map_err(|allocation| {
            Error::allocation(
                root,
                allocation.workspace,
                allocation.requested,
                allocation.error,
            )
        })?;
        let value = machine.run(root, &mut plan, context)?;
        Ok(FormulaExecutedScalarExpression { plan, value })
    }
}

impl<'a> Sources<'a> for Plan<'a> {
    fn read_value(
        &mut self,
        node: Node<'a, 'a>,
        context: Context<'a>,
    ) -> Result<ReadValue<'a>, Failure<'a>> {
        let Some(index) = node.dependency_index() else {
            unreachable!("actual checked name retains its own source slot");
        };
        let Some(slot) = self.sources.get_mut(index) else {
            unreachable!("private complete plan retains each actual name occurrence");
        };
        let Some(kind) = slot.kind else {
            unreachable!("complete plan binds every source");
        };
        if slot.read.is_some() {
            unreachable!("a normalized tree executes each selected node once");
        }
        let data = read::read(kind, context)?;
        slot.read = Some(data);
        Ok(ReadValue::Ready(data.value))
    }
    fn tolerance(&self, node: Node<'a, 'a>) -> crate::recipe::FormulaToleranceValue<'a> {
        let Some(index) = node.dependency_index() else {
            unreachable!("direct certified tolerance operand retains its name occurrence");
        };
        let Some(read) = self.sources.get(index).and_then(|slot| slot.read) else {
            unreachable!("all actual within operands were evaluated before application");
        };
        let super::FormulaScalarExecutionReadKind::Tolerance(tolerance) = read.direct_source()
        else {
            unreachable!("actual direct reserved class was read from its quantitative context");
        };
        tolerance
    }
}
