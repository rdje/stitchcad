//! Shared private actual-node machine for admitted scalar execution.
use super::{
    dispatch, FormulaScalarExecutionContext as Context, FormulaScalarExecutionError as Error,
    FormulaScalarExecutionFailure as Failure, FormulaScalarExecutionWorkspace as Workspace,
};
use crate::recipe::{
    FormulaApproximationSources, FormulaBinaryOperator, FormulaBuiltin, FormulaCanonicalInput,
    FormulaCheckedNode as Node, FormulaCheckedNodeForm as Form, FormulaEvaluatedScalar as V,
    FormulaNumericValue, FormulaToleranceValue, FormulaUnaryOperator,
};
use std::collections::TryReserveError;

// Only runtime-owned implementations can read actual checked nodes or retrieve an executed class.
// This is private to the recipe engine; callers cannot provide AST values or callbacks.
pub(super) trait Sources<'a> {
    fn read_value(
        &mut self,
        node: Node<'a, 'a>,
        context: Context<'a>,
    ) -> Result<ReadValue<'a>, Failure<'a>>;
    fn tolerance(&self, node: Node<'a, 'a>) -> FormulaToleranceValue<'a>;
}

// Control flow is private: only an actual unresolved canonical Derived read may suspend.
#[derive(Debug)]
pub(super) enum ReadValue<'a> {
    Ready(V),
    AwaitDerived(FormulaCanonicalInput<'a>),
}

#[derive(Debug)]
pub(super) enum Progress<'a> {
    Complete(V),
    AwaitDerived {
        node: Node<'a, 'a>,
        input: FormulaCanonicalInput<'a>,
    },
}

#[derive(Clone, Copy)]
struct Active<'a> {
    root: Node<'a, 'a>,
    context: Context<'a>,
}
// Keep the pinned provider context inline in the admitted machine control record. Boxing the
// running variant would add an unreserved allocation to start/resume and obscure its ownership.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Copy)]
enum State<'a> {
    Idle,
    Running(Active<'a>),
    Failed,
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
    state: State<'a>,
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
            state: State::Idle,
        })
    }
    pub(super) fn run(
        &mut self,
        root: Node<'a, 'a>,
        sources: &mut impl Sources<'a>,
        context: Context<'a>,
    ) -> Result<V, Error<'a>> {
        self.start(root, context)?;
        match self.resume(sources)? {
            Progress::Complete(value) => Ok(value),
            Progress::AwaitDerived { node, input } => {
                // One-shot adapters still return the original canonical evaluation requirement.
                // Never leave a resumable prefix behind after returning a one-shot error.
                self.state = State::Failed;
                let Err(failure) = super::read::read(
                    super::FormulaScalarExecutionSourceKind::Canonical(input),
                    context,
                ) else {
                    unreachable!("only an actual unresolved Derived source can suspend");
                };
                Err(Error::evaluation(node, failure))
            }
        }
    }
    pub(super) fn start(
        &mut self,
        root: Node<'a, 'a>,
        context: Context<'a>,
    ) -> Result<(), Error<'a>> {
        if !matches!(self.state, State::Idle) || !self.pending.is_empty() || !self.values.is_empty()
        {
            unreachable!("only successful idle scratch can start an actual operand");
        }
        let n = root.all_nodes().len();
        if n > self.admitted {
            self.state = State::Failed;
            return Err(Error::evaluation(
                root,
                Failure::WorkspaceReservation {
                    workspace: Workspace::Tasks,
                    admitted: self.admitted,
                    requested: n,
                },
            ));
        }
        // Complete N admission precedes this first task and every provider/value read.
        if let Err(error) = push(
            &mut self.pending,
            Task::Enter(root),
            n,
            Workspace::Tasks,
            root,
        ) {
            self.state = State::Failed;
            return Err(error);
        }
        self.state = State::Running(Active { root, context });
        Ok(())
    }
    pub(super) fn resume(
        &mut self,
        sources: &mut impl Sources<'a>,
    ) -> Result<Progress<'a>, Error<'a>> {
        let State::Running(active) = self.state else {
            unreachable!("only an active successful machine may resume");
        };
        // The caller cannot replace the root or contexts while a real operand is suspended.
        let result = self.advance(sources, active);
        match &result {
            Ok(Progress::Complete(_)) => self.state = State::Idle,
            Ok(Progress::AwaitDerived { .. }) => {}
            Err(_) => self.state = State::Failed,
        }
        result
    }
    fn advance(
        &mut self,
        sources: &mut impl Sources<'a>,
        active: Active<'a>,
    ) -> Result<Progress<'a>, Error<'a>> {
        let n = active.root.all_nodes().len();
        let context = active.context;
        let pending = &mut self.pending;
        let values = &mut self.values;
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
                        match sources
                            .read_value(node, context)
                            .map_err(|failure| Error::evaluation(node, failure))?
                        {
                            ReadValue::Ready(value) => {
                                push(values, value, n, Workspace::Values, node)?;
                            }
                            ReadValue::AwaitDerived(input) => {
                                // Restore precisely the popped name,not its ancestors or already
                                // completed operands. The freed slot fits the original N bound.
                                push(pending, Task::Enter(node), n, Workspace::Tasks, node)?;
                                return Ok(Progress::AwaitDerived { node, input });
                            }
                        }
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
        Ok(Progress::Complete(value))
    }
}

#[cfg(test)]
mod tests;
