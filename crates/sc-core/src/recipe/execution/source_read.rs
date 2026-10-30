//! Actual selected source reads;Derived witnesses are earned only from private completed roots.
use super::{
    machine::{ReadValue, Sources},
    read::{self, ReadData},
    FormulaScalarExecutionContext as Context, FormulaScalarExecutionFailure as Failure,
    FormulaScalarExecutionPlan as Plan, FormulaScalarExecutionReadKind,
};
use crate::recipe::{
    FormulaCanonicalInput, FormulaCheckedNode as Node, FormulaCheckedSource, FormulaCheckedSources,
    FormulaEvaluatedScalar, FormulaToleranceValue,
};

/// An actual canonical Derived read from a successfully executed original supplier root.
/// This preserves the original state/source reference;it never claims an authored value or let.
/// ```compile_fail
/// fn forge(read: &mut sc_core::recipe::FormulaExecutedDerivedSourceRead<'_>,
///     input: sc_core::recipe::FormulaCanonicalInput<'_>) { read.input = input; }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct FormulaExecutedDerivedSourceRead<'a> {
    input: FormulaCanonicalInput<'a>,
    supplier: &'a FormulaCheckedSource<'a>,
}
impl<'a> FormulaExecutedDerivedSourceRead<'a> {
    /// Actual same-inventory canonical join,including its original Derived state and provenance.
    #[must_use]
    pub const fn input(self) -> FormulaCanonicalInput<'a> {
        self.input
    }
    /// Actual source proof selected by that state's sole formula identity and executed successfully.
    #[must_use]
    pub const fn supplier(self) -> &'a FormulaCheckedSource<'a> {
        self.supplier
    }
}

/// Original successful source read;direct authored/provider reads retain their existing witnesses.
#[derive(Clone, Copy, Debug)]
pub enum FormulaScalarSourceGraphReadKind<'a> {
    /// Existing canonical authored,tolerance or size read;no new state/value annotation.
    Direct(FormulaScalarExecutionReadKind<'a>),
    /// Actual original canonical Derived join and successfully executed supplier proof.
    Derived(FormulaExecutedDerivedSourceRead<'a>),
}
/// One original selected name occurrence and its actual direct or unbound Derived result.
#[derive(Clone, Copy, Debug)]
pub struct FormulaScalarSourceGraphRead<'a> {
    pub(super) node: Node<'a, 'a>,
    pub(super) data: ReadData<'a>,
}
impl<'a> FormulaScalarSourceGraphRead<'a> {
    /// Original checked name occurrence,local span and actual declaration.
    #[must_use]
    pub const fn node(self) -> Node<'a, 'a> {
        self.node
    }
    /// Original successful direct/Derived source witness;not a complete static dependency list.
    #[must_use]
    pub const fn source(self) -> FormulaScalarSourceGraphReadKind<'a> {
        self.data.source
    }
    /// Actual read value and complete approximation contributions,without source-edge storage.
    #[must_use]
    pub const fn value(self) -> FormulaEvaluatedScalar {
        self.data.value
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) struct Completed {
    pub(super) value: FormulaEvaluatedScalar,
    pub(super) ordinal: usize,
}
pub(super) fn source_index(
    checked: &FormulaCheckedSources<'_>,
    id: crate::ontology::EntityId,
) -> Option<usize> {
    checked.normalized().inputs().sources().source_index(id)
}
// A witness can be earned only from this exact inventory and an actually completed original root.
pub(super) fn derived_source_index(
    input: FormulaCanonicalInput<'_>,
    checked: &FormulaCheckedSources<'_>,
) -> usize {
    if !core::ptr::eq(input.inputs(), checked.normalized().inputs().inputs()) {
        unreachable!("private source and recipe runtimes share the actual canonical owner");
    }
    let Some(formula) = input.derived_formula() else {
        unreachable!("actual unresolved Derived input");
    };
    let Some(index) = source_index(checked, formula) else {
        unreachable!("actual complete source/input join retains its sole supplier");
    };
    index
}
pub(super) fn completed_derived_read<'a>(
    input: FormulaCanonicalInput<'a>,
    checked: &'a FormulaCheckedSources<'a>,
    completed: &[Option<Completed>],
) -> Option<ReadData<'a>> {
    let index = derived_source_index(input, checked);
    let Some(slot) = completed.get(index) else {
        unreachable!("complete output population shares original source positions");
    };
    let completed = (*slot)?;
    let Some(supplier) = checked.sources().get(index) else {
        unreachable!("original source proof position");
    };
    Some(ReadData {
        source: FormulaScalarSourceGraphReadKind::Derived(FormulaExecutedDerivedSourceRead {
            input,
            supplier,
        }),
        value: completed.value,
    })
}
pub(super) struct GraphSources<'s, 'a> {
    pub(super) plan: &'s mut Plan<'a>,
    pub(super) checked: &'a FormulaCheckedSources<'a>,
    pub(super) completed: &'s [Option<Completed>],
}
impl<'a> Sources<'a> for GraphSources<'_, 'a> {
    fn read_value(
        &mut self,
        node: Node<'a, 'a>,
        context: Context<'a>,
    ) -> Result<ReadValue<'a>, Failure<'a>> {
        let Some(slot) = node
            .dependency_index()
            .and_then(|i| self.plan.sources.get_mut(i))
        else {
            unreachable!("actual complete plan retains the original name occurrence");
        };
        if slot.read.is_some() {
            unreachable!("only unresolved names may be re-entered");
        }
        let Some(kind) = slot.kind else {
            unreachable!("complete source binding");
        };
        let data = if let super::FormulaScalarExecutionSourceKind::Canonical(input) = kind {
            if input.derived_formula().is_some() {
                let Some(data) = completed_derived_read(input, self.checked, self.completed) else {
                    return Ok(ReadValue::AwaitDerived(input));
                };
                data
            } else {
                read::read(kind, context)?
            }
        } else {
            read::read(kind, context)?
        };
        slot.read = Some(data);
        Ok(ReadValue::Ready(data.value))
    }
    fn tolerance(&self, node: Node<'a, 'a>) -> FormulaToleranceValue<'a> {
        self.plan.tolerance(node)
    }
}
