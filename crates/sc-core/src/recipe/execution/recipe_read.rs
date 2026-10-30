//! Runtime reads only actual canonical sources or this execution's privately completed prefix.
use super::{
    machine::{ReadValue, Sources},
    read,
    recipe_result::{RecipeReadData, RecipeReadSource},
    source_runtime::SourceRuntime,
    FormulaExecutedScalarRecipeStatement, FormulaScalarExecutionContext as Context,
    FormulaScalarExecutionFailure as Failure, FormulaScalarExecutionReadKind as ScalarRead,
    FormulaScalarExecutionSourceKind as ScalarKind, FormulaScalarRecipeOutcome as Outcome,
    FormulaScalarRecipeReadKind as ReadKind, FormulaScalarRecipeSource,
    FormulaScalarRecipeSourceKind as Kind,
};
use crate::recipe::{
    FormulaCheckedNode as Node, FormulaEvaluatedScalar as V, FormulaToleranceReadError,
    FormulaToleranceValue,
};

pub(super) struct RecipeSources<'s, 'a> {
    pub(super) sources: &'s mut [FormulaScalarRecipeSource<'a>],
    pub(super) completed: &'s [FormulaExecutedScalarRecipeStatement<'a>],
    pub(super) derived: Option<&'s SourceRuntime<'a>>,
}
impl<'a> Sources<'a> for RecipeSources<'_, 'a> {
    fn read_value(
        &mut self,
        node: Node<'a, 'a>,
        context: Context<'a>,
    ) -> Result<ReadValue<'a>, Failure<'a>> {
        let Some(slot) = node
            .dependency_index()
            .and_then(|index| self.sources.get_mut(index))
        else {
            unreachable!("actual operand proof retains its admitted original source occurrence");
        };
        if slot.read.is_some() || slot.dependency.span() != node.span() {
            unreachable!(
                "private current operand/source slice retains an unread same-proof occurrence"
            );
        }
        let (source, value) = match slot.kind {
            Kind::Canonical(joined) => {
                if let Some(runtime) = self.derived.filter(|_| joined.derived_formula().is_some()) {
                    let Some(data) = runtime.read_derived(joined) else {
                        return Ok(ReadValue::AwaitDerived(joined));
                    };
                    let super::FormulaScalarSourceGraphReadKind::Derived(source) = data.source
                    else {
                        unreachable!("actual completed Derived supplier witness");
                    };
                    (RecipeReadSource::Derived(source), data.value)
                } else {
                    let data = read::read(ScalarKind::Canonical(joined), context)?;
                    (
                        RecipeReadSource::Original(ReadKind::Scalar(data.direct_source())),
                        data.value,
                    )
                }
            }
            Kind::Reserved(name) => {
                let data = read::read(ScalarKind::Reserved(name), context)?;
                (
                    RecipeReadSource::Original(ReadKind::Scalar(data.direct_source())),
                    data.value,
                )
            }
            Kind::PriorLet(statement) => {
                let Some(completed) = statement
                    .statement_index()
                    .checked_sub(1)
                    .and_then(|index| self.completed.get(index))
                else {
                    unreachable!(
                        "only a successful earlier private outcome can supply its real let"
                    );
                };
                if !core::ptr::eq(completed.statement, statement) {
                    unreachable!(
                        "private output ordinal must retain the exact whole-proof supplier"
                    );
                }
                let Outcome::Binding { binding, .. } = completed.outcome else {
                    unreachable!("only an actual checked completed let supplies scalar reads");
                };
                (
                    RecipeReadSource::Original(ReadKind::PriorLet { statement, binding }),
                    binding.read().map_err(Failure::Numeric)?,
                )
            }
        };
        slot.read = Some(RecipeReadData {
            node: Some(node),
            source,
            value,
        });
        Ok(ReadValue::Ready(value))
    }
    fn tolerance(&self, node: Node<'a, 'a>) -> FormulaToleranceValue<'a> {
        let Some(data) = node
            .dependency_index()
            .and_then(|index| self.sources.get(index))
            .and_then(|source| source.read)
        else {
            unreachable!("actual within class was already read as its direct operand");
        };
        let RecipeReadSource::Original(ReadKind::Scalar(ScalarRead::Tolerance(value))) =
            data.source
        else {
            unreachable!(
                "certified direct tolerance operand retains its actual quantitative source"
            );
        };
        value
    }
}

pub(super) fn header<'a>(
    source: &mut FormulaScalarRecipeSource<'a>,
    context: Context<'a>,
) -> Result<FormulaToleranceValue<'a>, FormulaToleranceReadError<'a>> {
    let Kind::Reserved(name) = source.kind else {
        unreachable!("actual assertion header source is its original reserved tolerance");
    };
    let Some(name) = name.tolerance_name() else {
        unreachable!("assertion header is a tolerance role");
    };
    if source.read.is_some() {
        unreachable!("an actual assertion header is read once after operands");
    }
    let value = context.tolerances().read(name)?;
    source.read = Some(RecipeReadData {
        node: None,
        source: RecipeReadSource::Original(ReadKind::Scalar(ScalarRead::Tolerance(value))),
        value: V::from_numeric(value.value()),
    });
    Ok(value)
}

#[cfg(test)]
mod tests;
