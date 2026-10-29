//! Iterative execution consumes actual checked node proofs and bound sources in authored order.
use super::{
    dispatch, read, FormulaScalarExecutionContext as Context, FormulaScalarExecutionError as Error,
    FormulaScalarExecutionFailure as Failure, FormulaScalarExecutionPlan as Plan,
    FormulaScalarExecutionRead, FormulaScalarExecutionWorkspace as Workspace,
};
use crate::{
    recipe::{
        FormulaApproximationSources, FormulaBinaryOperator, FormulaBuiltin,
        FormulaCheckedNode as Node, FormulaCheckedNodeForm as Form, FormulaEvaluatedScalar as V,
        FormulaNumericValue, FormulaUnaryOperator,
    },
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

#[derive(Clone, Copy, Debug)]
enum Task<'a> {
    Enter(Node<'a, 'a>),
    Unary(Node<'a, 'a>, FormulaUnaryOperator),
    Binary(Node<'a, 'a>, FormulaBinaryOperator),
    Call(Node<'a, 'a>, FormulaBuiltin, usize),
    Select(Node<'a, 'a>, Node<'a, 'a>, Node<'a, 'a>),
    FinishConditional(Node<'a, 'a>, FormulaApproximationSources),
}

fn push<'a, T>(
    buffer: &mut Vec<T>,
    value: T,
    admitted: usize,
    workspace: Workspace,
    node: Node<'a, 'a>,
) -> Result<(), Error<'a>> {
    if buffer.len() >= admitted {
        return Err(Error::evaluation(
            node,
            Failure::WorkspaceReservation {
                workspace,
                admitted,
                requested: buffer.len() + 1,
            },
        ));
    }
    // Both buffers were fallibly reserved for the full N before any value/provider read.
    buffer.push(value);
    Ok(())
}
fn pop(values: &mut Vec<V>) -> V {
    let Some(value) = values.pop() else {
        unreachable!("actual checked operation consumes exactly its completed child values");
    };
    value
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
        let n = plan.admission.node_count();
        let mut pending = Vec::new();
        pending
            .try_reserve_exact(n)
            .map_err(|error| Error::allocation(root, Workspace::Tasks, n, error))?;
        let mut values = Vec::new();
        values
            .try_reserve_exact(n)
            .map_err(|error| Error::allocation(root, Workspace::Values, n, error))?;
        push(&mut pending, Task::Enter(root), n, Workspace::Tasks, root)?;
        while let Some(task) = pending.pop() {
            match task {
                Task::Enter(node) => match node.form() {
                    Form::Literal(literal) => {
                        let value = FormulaNumericValue::from_literal(literal)
                            .map(V::from_numeric)
                            .map_err(|error| Error::evaluation(node, Failure::Numeric(error)))?;
                        push(&mut values, value, n, Workspace::Values, node)?;
                    }
                    Form::Name { .. } => {
                        let Some(index) = node.dependency_index() else {
                            unreachable!("actual checked name retains its own source slot");
                        };
                        let Some(slot) = plan.sources.get_mut(index) else {
                            unreachable!(
                                "private complete plan retains each actual name occurrence"
                            );
                        };
                        let Some(kind) = slot.kind else {
                            unreachable!("complete plan binds every source");
                        };
                        if slot.read.is_some() {
                            unreachable!("a normalized tree executes each selected node once");
                        }
                        let data = read::read(kind, context)
                            .map_err(|failure| Error::evaluation(node, failure))?;
                        slot.read = Some(data);
                        push(&mut values, data.value, n, Workspace::Values, node)?;
                    }
                    Form::Negate(child) | Form::Square(child) => {
                        let operator = if matches!(node.form(), Form::Negate(_)) {
                            FormulaUnaryOperator::Negate
                        } else {
                            FormulaUnaryOperator::Square
                        };
                        push(
                            &mut pending,
                            Task::Unary(node, operator),
                            n,
                            Workspace::Tasks,
                            node,
                        )?;
                        push(&mut pending, Task::Enter(child), n, Workspace::Tasks, child)?;
                    }
                    Form::Binary {
                        operator,
                        left,
                        right,
                    } => {
                        push(
                            &mut pending,
                            Task::Binary(node, operator),
                            n,
                            Workspace::Tasks,
                            node,
                        )?;
                        push(&mut pending, Task::Enter(right), n, Workspace::Tasks, right)?;
                        push(&mut pending, Task::Enter(left), n, Workspace::Tasks, left)?;
                    }
                    Form::Call { builtin, arguments } => {
                        push(
                            &mut pending,
                            Task::Call(node, builtin, arguments.len()),
                            n,
                            Workspace::Tasks,
                            node,
                        )?;
                        for argument in arguments.iter().rev() {
                            push(
                                &mut pending,
                                Task::Enter(argument),
                                n,
                                Workspace::Tasks,
                                argument,
                            )?;
                        }
                    }
                    Form::Conditional {
                        condition,
                        then_branch,
                        else_branch,
                    } => {
                        push(
                            &mut pending,
                            Task::Select(node, then_branch, else_branch),
                            n,
                            Workspace::Tasks,
                            node,
                        )?;
                        push(
                            &mut pending,
                            Task::Enter(condition),
                            n,
                            Workspace::Tasks,
                            condition,
                        )?;
                    }
                },
                Task::Unary(node, operator) => {
                    let value = pop(&mut values)
                        .checked_unary(operator)
                        .map_err(|error| Error::evaluation(node, Failure::Numeric(error)))?;
                    push(&mut values, value, n, Workspace::Values, node)?;
                }
                Task::Binary(node, operator) => {
                    let right = pop(&mut values);
                    let left = pop(&mut values);
                    let value = left
                        .checked_binary(operator, right)
                        .map_err(|error| Error::evaluation(node, Failure::Numeric(error)))?;
                    push(&mut values, value, n, Workspace::Values, node)?;
                }
                Task::Call(node, builtin, arity) => {
                    let Some(start) = values.len().checked_sub(arity) else {
                        unreachable!("actual checked call consumes exactly its completed operands");
                    };
                    let Some(arguments) = values.get(start..) else {
                        unreachable!("private completed operand suffix");
                    };
                    let value = dispatch::call(&plan, node, builtin, arguments)
                        .map_err(|failure| Error::evaluation(node, failure))?;
                    values.truncate(start);
                    push(&mut values, value, n, Workspace::Values, node)?;
                }
                Task::Select(node, then_branch, else_branch) => {
                    let condition = pop(&mut values);
                    let Some(choice) = condition.boolean() else {
                        unreachable!("actual condition proof is Boolean");
                    };
                    let branch = if choice { then_branch } else { else_branch };
                    push(
                        &mut pending,
                        Task::FinishConditional(node, condition.sources()),
                        n,
                        Workspace::Tasks,
                        node,
                    )?;
                    push(
                        &mut pending,
                        Task::Enter(branch),
                        n,
                        Workspace::Tasks,
                        branch,
                    )?;
                }
                Task::FinishConditional(node, condition_sources) => {
                    let value = pop(&mut values).influenced_by(condition_sources);
                    push(&mut values, value, n, Workspace::Values, node)?;
                }
            }
        }
        let value = pop(&mut values);
        if !values.is_empty() {
            unreachable!("complete expression leaves exactly one root value");
        }
        Ok(FormulaExecutedScalarExpression { plan, value })
    }
}
