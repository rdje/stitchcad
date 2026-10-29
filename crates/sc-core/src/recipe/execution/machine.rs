//! Shared private actual-node machine for admitted scalar execution.
use super::{
    dispatch, FormulaScalarExecutionContext as Context, FormulaScalarExecutionError as Error,
    FormulaScalarExecutionFailure as Failure, FormulaScalarExecutionWorkspace as Workspace,
};
use crate::recipe::{
    FormulaApproximationSources, FormulaBinaryOperator, FormulaBuiltin, FormulaCheckedNode as Node,
    FormulaCheckedNodeForm as Form, FormulaEvaluatedScalar as V, FormulaNumericValue,
    FormulaToleranceValue, FormulaUnaryOperator,
};
use std::collections::TryReserveError;

// Only runtime-owned implementations can read actual checked nodes or retrieve an executed class.
// This is private to the recipe engine; callers cannot provide AST values or callbacks.
pub(super) trait Sources<'a> {
    fn read_value(&mut self, node: Node<'a, 'a>, context: Context<'a>) -> Result<V, Failure<'a>>;
    fn tolerance(&self, node: Node<'a, 'a>) -> FormulaToleranceValue<'a>;
}

#[derive(Debug)]
pub(super) struct MachineAllocation {
    pub(super) workspace: Workspace,
    pub(super) requested: usize,
    pub(super) error: TryReserveError,
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

pub(super) struct Machine<'a> {
    pending: Vec<Task<'a>>,
    values: Vec<V>,
    admitted: usize,
}
impl<'a> Machine<'a> {
    pub(super) fn new(admitted: usize) -> Result<Self, MachineAllocation> {
        let reserve = |workspace, error| MachineAllocation {
            workspace,
            requested: admitted,
            error,
        };
        let mut pending = Vec::new();
        pending
            .try_reserve_exact(admitted)
            .map_err(|e| reserve(Workspace::Tasks, e))?;
        let mut values = Vec::new();
        values
            .try_reserve_exact(admitted)
            .map_err(|e| reserve(Workspace::Values, e))?;
        Ok(Self {
            pending,
            values,
            admitted,
        })
    }
    pub(super) fn run(
        &mut self,
        root: Node<'a, 'a>,
        sources: &mut impl Sources<'a>,
        context: Context<'a>,
    ) -> Result<V, Error<'a>> {
        let n = root.all_nodes().len();
        if n > self.admitted {
            return Err(Error::evaluation(
                root,
                Failure::WorkspaceReservation {
                    workspace: Workspace::Tasks,
                    admitted: self.admitted,
                    requested: n,
                },
            ));
        }
        let pending = &mut self.pending;
        let values = &mut self.values;
        if !pending.is_empty() || !values.is_empty() {
            unreachable!(
                "only a completed empty scratch machine can start the next actual operand"
            );
        }
        push(pending, Task::Enter(root), n, Workspace::Tasks, root)?;
        while let Some(task) = pending.pop() {
            match task {
                Task::Enter(node) => match node.form() {
                    Form::Literal(literal) => {
                        let value = FormulaNumericValue::from_literal(literal)
                            .map(V::from_numeric)
                            .map_err(|error| Error::evaluation(node, Failure::Numeric(error)))?;
                        push(values, value, n, Workspace::Values, node)?;
                    }
                    Form::Name { .. } => {
                        let value = sources
                            .read_value(node, context)
                            .map_err(|failure| Error::evaluation(node, failure))?;
                        push(values, value, n, Workspace::Values, node)?;
                    }
                    Form::Negate(child) | Form::Square(child) => {
                        let operator = if matches!(node.form(), Form::Negate(_)) {
                            FormulaUnaryOperator::Negate
                        } else {
                            FormulaUnaryOperator::Square
                        };
                        push(
                            pending,
                            Task::Unary(node, operator),
                            n,
                            Workspace::Tasks,
                            node,
                        )?;
                        push(pending, Task::Enter(child), n, Workspace::Tasks, child)?;
                    }
                    Form::Binary {
                        operator,
                        left,
                        right,
                    } => {
                        push(
                            pending,
                            Task::Binary(node, operator),
                            n,
                            Workspace::Tasks,
                            node,
                        )?;
                        push(pending, Task::Enter(right), n, Workspace::Tasks, right)?;
                        push(pending, Task::Enter(left), n, Workspace::Tasks, left)?;
                    }
                    Form::Call { builtin, arguments } => {
                        push(
                            pending,
                            Task::Call(node, builtin, arguments.len()),
                            n,
                            Workspace::Tasks,
                            node,
                        )?;
                        for argument in arguments.iter().rev() {
                            push(
                                pending,
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
                            pending,
                            Task::Select(node, then_branch, else_branch),
                            n,
                            Workspace::Tasks,
                            node,
                        )?;
                        push(
                            pending,
                            Task::Enter(condition),
                            n,
                            Workspace::Tasks,
                            condition,
                        )?;
                    }
                },
                Task::Unary(node, operator) => {
                    let value = pop(values)
                        .checked_unary(operator)
                        .map_err(|error| Error::evaluation(node, Failure::Numeric(error)))?;
                    push(values, value, n, Workspace::Values, node)?;
                }
                Task::Binary(node, operator) => {
                    let right = pop(values);
                    let left = pop(values);
                    let value = left
                        .checked_binary(operator, right)
                        .map_err(|error| Error::evaluation(node, Failure::Numeric(error)))?;
                    push(values, value, n, Workspace::Values, node)?;
                }
                Task::Call(node, builtin, arity) => {
                    let Some(start) = values.len().checked_sub(arity) else {
                        unreachable!("actual checked call consumes exactly its completed operands");
                    };
                    let Some(arguments) = values.get(start..) else {
                        unreachable!("private completed operand suffix");
                    };
                    let value = dispatch::call(sources, node, builtin, arguments)
                        .map_err(|failure| Error::evaluation(node, failure))?;
                    values.truncate(start);
                    push(values, value, n, Workspace::Values, node)?;
                }
                Task::Select(node, then_branch, else_branch) => {
                    let condition = pop(values);
                    let Some(choice) = condition.boolean() else {
                        unreachable!("actual condition proof is Boolean");
                    };
                    let branch = if choice { then_branch } else { else_branch };
                    push(
                        pending,
                        Task::FinishConditional(node, condition.sources()),
                        n,
                        Workspace::Tasks,
                        node,
                    )?;
                    push(pending, Task::Enter(branch), n, Workspace::Tasks, branch)?;
                }
                Task::FinishConditional(node, condition_sources) => {
                    let value = pop(values).influenced_by(condition_sources);
                    push(values, value, n, Workspace::Values, node)?;
                }
            }
        }
        let value = pop(values);
        if !values.is_empty() {
            unreachable!("complete expression leaves exactly one root value");
        }
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{recipe::*, value::CanonicalInputs};

    #[test]
    #[allow(clippy::expect_used)]
    fn actual_operands_reuse_reserved_scratch_and_refuse_insufficient_capacity_before_reads() {
        let inputs = CanonicalInputs::new(vec![], vec![]).expect("inventory");
        let namespace = FormulaNamespace::new([]).expect("namespace");
        let first = FormulaExpression::parse("if(1==1,1 mm/3,eps_phys)")
            .expect("syntax")
            .normalize_literals()
            .expect("inputs");
        let second = FormulaExpression::parse("within(1 mm,1 mm,eps_num)")
            .expect("syntax")
            .normalize_literals()
            .expect("inputs");
        let first_proof = first.check_kinds(&namespace).expect("complete");
        let second_proof = second.check_kinds(&namespace).expect("complete");
        let mut first_plan = first_proof
            .admit_execution(FormulaExecutionLimits::new(8, 17))
            .expect("exact")
            .prepare_scalar(&inputs)
            .expect("plan");
        let mut second_plan = second_proof
            .admit_execution(FormulaExecutionLimits::new(4, 9))
            .expect("exact")
            .prepare_scalar(&inputs)
            .expect("plan");
        let first_root = first_plan.admission().root();
        let second_root = second_plan.admission().root();
        let context = Context::new(
            FormulaToleranceContext::empty(),
            FormulaSizeContext::empty(),
        );
        let error = Machine::new(0)
            .expect("empty allocation")
            .run(first_root, &mut first_plan, context)
            .expect_err("complete actual node count before source reads");
        assert!(
            matches!(error.cause(), FormulaScalarExecutionErrorCause::Evaluation(failure)
            if matches!(failure.as_ref(), Failure::WorkspaceReservation {
                workspace: Workspace::Tasks, admitted: 0, requested: 8 }))
        );
        assert!(first_plan.sources.iter().all(|slot| slot.read.is_none()));
        let mut machine = Machine::new(8).expect("complete reservation");
        let pointers = (machine.pending.as_ptr(), machine.values.as_ptr());
        let first_value = machine
            .run(first_root, &mut first_plan, context)
            .expect("actual first");
        assert_eq!(
            first_value.numeric().expect("Length").rational(),
            FormulaRational::new(1000, 3, false).expect("exact")
        );
        assert!(first_plan.sources.iter().all(|slot| slot.read.is_none()));
        let second_value = machine
            .run(second_root, &mut second_plan, context)
            .expect("actual second");
        assert_eq!(second_value.boolean(), Some(true));
        assert_eq!(
            second_plan
                .sources
                .iter()
                .filter(|slot| slot.read.is_some())
                .count(),
            1
        );
        assert_eq!(
            (machine.pending.as_ptr(), machine.values.as_ptr()),
            pointers
        );
        assert!(machine.pending.is_empty() && machine.values.is_empty());
    }
}
