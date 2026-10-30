//! Shared private source runtime;only executed original roots enter the initially empty cache.
use super::{
    machine::{Machine, Progress},
    source_read::{source_index, Completed, GraphSources},
    FormulaScalarExecutionContext as Context, FormulaScalarExecutionWorkspace,
    FormulaScalarSourceGraphExecutionCause as Cause, FormulaScalarSourceGraphPlan as Plan,
    FormulaScalarSourceGraphWorkspace as Workspace,
};
use crate::recipe::FormulaEvaluatedScalar;
use std::collections::TryReserveError;

pub(super) struct CompletedSources<'a> {
    pub(super) plan: Plan<'a>,
    pub(super) context: Context<'a>,
    pub(super) completed: Vec<Option<Completed>>,
}
pub(super) struct SourceRuntime<'a> {
    plan: Plan<'a>,
    context: Context<'a>,
    machines: Vec<Machine<'a>>,
    frames: Vec<usize>,
    completed: Vec<Option<Completed>>,
    ordinal: usize,
    failed: bool,
}
impl<'a> SourceRuntime<'a> {
    // Fallibly reserve every complete source population before starting any machine or value read.
    pub(super) fn new(plan: Plan<'a>, context: Context<'a>) -> Result<Self, Cause<'a>> {
        let s = plan.sources().len();
        let allocation =
            |workspace, source_index, requested, error: TryReserveError| Cause::Allocation {
                workspace,
                source_index,
                requested,
                error,
            };
        let mut machines = Vec::new();
        machines
            .try_reserve_exact(s)
            .map_err(|e| allocation(Workspace::Sources, None, s, e))?;
        let mut frames = Vec::new();
        frames
            .try_reserve_exact(s)
            .map_err(|e| allocation(Workspace::CallFrames, None, s, e))?;
        let mut completed = Vec::new();
        completed
            .try_reserve_exact(s)
            .map_err(|e| allocation(Workspace::Outputs, None, s, e))?;
        completed.resize(s, None);
        for (index, source) in plan.sources().iter().enumerate() {
            let machine = Machine::new(source.plan().admission().node_count()).map_err(|e| {
                allocation(
                    match e.workspace {
                        FormulaScalarExecutionWorkspace::Tasks => Workspace::Tasks,
                        FormulaScalarExecutionWorkspace::Values => Workspace::Values,
                    },
                    Some(index),
                    e.requested,
                    e.error,
                )
            })?;
            machines.push(machine);
        }
        Ok(Self {
            plan,
            context,
            machines,
            frames,
            completed,
            ordinal: 0,
            failed: false,
        })
    }
    // Entry is selected through the actual source index by the private caller,never an AST value.
    // Any error/panic poisons this runtime;no successful cache/prefix can be published afterwards.
    pub(super) fn demand(&mut self, entry: usize) -> Result<FormulaEvaluatedScalar, Cause<'a>> {
        if self.failed {
            unreachable!("a failed private source runtime cannot be resumed");
        }
        let Some(completed) = self.completed.get(entry) else {
            unreachable!("actual selected source index");
        };
        if let Some(completed) = completed {
            return Ok(completed.value);
        }
        self.failed = true;
        let result = self.walk(entry);
        if result.is_ok() {
            self.failed = false;
        }
        result
    }
    fn walk(&mut self, entry: usize) -> Result<FormulaEvaluatedScalar, Cause<'a>> {
        let Self {
            plan,
            context,
            machines,
            frames,
            completed,
            ordinal,
            failed: _,
        } = self;
        if !frames.is_empty() {
            unreachable!("only an idle successful source runtime starts another demand");
        }
        let context = *context;
        let s = plan.sources().len();
        let checked = plan.graph().dependencies().checked();
        let evaluation = |source_index, error| Cause::Evaluation {
            source_index,
            error: Box::new(error),
        };
        let start = |index: usize, machines: &mut [Machine<'a>], plan: &Plan<'a>| {
            let Some(source) = plan.sources().get(index) else {
                unreachable!("actual exact source index");
            };
            let Some(machine) = machines.get_mut(index) else {
                unreachable!("complete reserved source controls");
            };
            machine
                .start(source.plan().admission().root(), context)
                .map_err(|e| evaluation(index, e))
        };
        start(entry, machines, plan)?;
        frames.push(entry);
        while let Some(index) = frames.last().copied() {
            let Some(current) = plan.sources.get_mut(index) else {
                unreachable!("original active source");
            };
            let Some(machine) = machines.get_mut(index) else {
                unreachable!("original active source machine");
            };
            let progress = machine
                .resume(&mut GraphSources {
                    plan: &mut current.plan,
                    checked,
                    completed,
                })
                .map_err(|e| evaluation(index, e))?;
            match progress {
                Progress::Complete(value) => {
                    let Some(slot) = completed.get_mut(index) else {
                        unreachable!("complete output reservation");
                    };
                    if slot.is_some() {
                        unreachable!("only an unexecuted source enters the acyclic walk");
                    }
                    *ordinal += 1;
                    *slot = Some(Completed {
                        value,
                        ordinal: *ordinal,
                    });
                    frames.pop();
                }
                Progress::AwaitDerived { node, input } => {
                    let Some(supplier) = input
                        .derived_formula()
                        .and_then(|id| source_index(checked, id))
                    else {
                        unreachable!("same immutable complete source/input/edge proofs retain actual supplier");
                    };
                    if frames.len() >= s {
                        return Err(Cause::FrameReservation {
                            source_index: index,
                            span: node.span(),
                            admitted: s,
                            requested: frames.len() + 1,
                        });
                    }
                    start(supplier, machines, plan)?;
                    frames.push(supplier);
                }
            }
        }
        let Some(value) = completed.get(entry).copied().flatten() else {
            unreachable!("successful demand completed its actual entry");
        };
        Ok(value.value)
    }
    // No caller supplies a value/cache:the same actual canonical join selects only earned roots.
    pub(super) fn read_derived(
        &self,
        input: crate::recipe::FormulaCanonicalInput<'a>,
    ) -> Option<super::read::ReadData<'a>> {
        if self.failed {
            unreachable!("a failed source runtime cannot supply a recipe read");
        }
        super::source_read::completed_derived_read(
            input,
            self.plan.graph().dependencies().checked(),
            &self.completed,
        )
    }
    // Only the enclosing successful worker publishes this real plan/cache;an error drops all state.
    pub(super) fn finish(self) -> CompletedSources<'a> {
        if self.failed {
            unreachable!("a failed private source runtime cannot publish a prefix");
        }
        CompletedSources {
            plan: self.plan,
            context: self.context,
            completed: self.completed,
        }
    }
}

#[cfg(test)]
mod tests;
