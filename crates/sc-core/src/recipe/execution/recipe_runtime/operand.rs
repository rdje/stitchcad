//! One actual operand continuation;only the private earned runtime may satisfy Derived requests.
use super::failure::{OperandFailure, SourceOperandFailure};
use crate::recipe::execution::{
    machine::{Machine, Progress},
    recipe_read::RecipeSources,
    source_runtime::SourceRuntime,
    FormulaExecutedScalarRecipeStatement, FormulaScalarExecutionContext as Context,
    FormulaScalarRecipeSource,
};
use crate::recipe::{FormulaCheckedNode, FormulaEvaluatedScalar};

pub(super) fn evaluate<'a>(
    machine: &mut Machine<'a>,
    root: FormulaCheckedNode<'a, 'a>,
    sources: &mut [FormulaScalarRecipeSource<'a>],
    completed: &[FormulaExecutedScalarRecipeStatement<'a>],
    context: Context<'a>,
    derived: Option<&mut SourceRuntime<'a>>,
) -> Result<FormulaEvaluatedScalar, OperandFailure<'a>> {
    let Some(runtime) = derived else {
        return machine
            .run(
                root,
                &mut RecipeSources {
                    sources,
                    completed,
                    derived: None,
                },
                context,
            )
            .map_err(OperandFailure::Scalar);
    };
    machine
        .start(root, context)
        .map_err(OperandFailure::Scalar)?;
    loop {
        let progress = machine
            .resume(&mut RecipeSources {
                sources,
                completed,
                derived: Some(runtime),
            })
            .map_err(OperandFailure::Scalar)?;
        match progress {
            Progress::Complete(value) => return Ok(value),
            Progress::AwaitDerived { node, input } => {
                runtime.demand_input(input).map_err(|cause| {
                    OperandFailure::Source(SourceOperandFailure::new(node, input, cause))
                })?;
            }
        }
    }
}
