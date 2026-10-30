//! Completed original source roots and selected read traces;no caller-owned result constructors.
use super::{
    source_read::Completed, FormulaScalarExecutionContext, FormulaScalarSourceGraphPlan,
    FormulaScalarSourceGraphRead, FormulaScalarSourceGraphSource,
};
use crate::{
    ontology::EntityId,
    recipe::{FormulaCheckedSource, FormulaEvaluatedScalar},
};
use core::fmt;

/// Actual selected entry and demanded suppliers,earned by executing their original unbound roots.
/// Unused sources remain unexecuted. Canonical states are unchanged;this is no let,Design,geometry,
/// currentness,evidence,approval or complete-resource certificate.
/// ```compile_fail
/// fn seed(result:&mut sc_core::recipe::FormulaExecutedScalarSourceGraph<'_>) {
///     result.completed.clear();
/// }
/// ```
pub struct FormulaExecutedScalarSourceGraph<'a> {
    pub(super) plan: FormulaScalarSourceGraphPlan<'a>,
    pub(super) context: FormulaScalarExecutionContext<'a>,
    pub(super) entry: usize,
    pub(super) completed: Vec<Option<Completed>>,
}
impl<'a> FormulaExecutedScalarSourceGraph<'a> {
    /// Actual complete source plans,limits and acyclic relation;not only an executed prefix.
    #[must_use]
    pub const fn plan(&self) -> &FormulaScalarSourceGraphPlan<'a> {
        &self.plan
    }
    /// Actual immutable contexts pinned before any selected source value/provider read.
    #[must_use]
    pub const fn context(&self) -> FormulaScalarExecutionContext<'a> {
        self.context
    }
    /// Actual successful selected entry;its demanded suppliers also completed before it.
    #[must_use]
    pub fn entry(&self) -> FormulaExecutedScalarSource<'_, 'a> {
        let Some(source) = self.plan.sources().get(self.entry) else {
            unreachable!("private actual entry index");
        };
        let Some(completed) = self.completed.get(self.entry).copied().flatten() else {
            unreachable!("only complete entry succeeds");
        };
        FormulaExecutedScalarSource { source, completed }
    }
    /// Actual completed source by exact id;None for an absent or unused source.
    #[must_use]
    pub fn source(&self, id: EntityId) -> Option<FormulaExecutedScalarSource<'_, 'a>> {
        let index =
            super::source_read::source_index(self.plan.graph().dependencies().checked(), id)?;
        Some(FormulaExecutedScalarSource {
            source: self.plan.sources().get(index)?,
            completed: self.completed.get(index).copied().flatten()?,
        })
    }
    /// Only actually completed sources,in original registry order. Completion ordinals retain
    /// their distinct actual execution order without allocating a sorted or copied result vector.
    pub fn sources(&self) -> impl DoubleEndedIterator<Item = FormulaExecutedScalarSource<'_, 'a>> {
        self.plan
            .sources()
            .iter()
            .zip(&self.completed)
            .filter_map(|(source, completed)| {
                Some(FormulaExecutedScalarSource {
                    source,
                    completed: (*completed)?,
                })
            })
    }
}
impl fmt::Debug for FormulaExecutedScalarSourceGraph<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaExecutedScalarSourceGraph")
            .field("entry", &self.entry)
            .field("completed_sources", &self.sources().count())
            .finish_non_exhaustive()
    }
}
/// Borrowed actual completed source and its original prepared/read plan;cannot outlive the result.
/// ```compile_fail
/// fn escape<'a>(plan:sc_core::recipe::FormulaScalarSourceGraphPlan<'a>,
///     context:sc_core::recipe::FormulaScalarExecutionContext<'a>,id:sc_core::ontology::EntityId)
///     ->sc_core::recipe::FormulaExecutedScalarSource<'a,'a> {
///     let result=plan.evaluate_source(id,context).unwrap();
///     result.entry()
/// }
/// ```
#[derive(Clone, Copy)]
pub struct FormulaExecutedScalarSource<'r, 'a> {
    source: &'r FormulaScalarSourceGraphSource<'a>,
    completed: Completed,
}
pub(super) fn completed_source_view<'r, 'a>(
    source: &'r FormulaScalarSourceGraphSource<'a>,
    completed: Completed,
) -> FormulaExecutedScalarSource<'r, 'a> {
    FormulaExecutedScalarSource { source, completed }
}
impl<'r, 'a> FormulaExecutedScalarSource<'r, 'a> {
    /// Actual original checked source,not a copied body,canonical alias or substituted proof.
    #[must_use]
    pub const fn source(self) -> &'a FormulaCheckedSource<'a> {
        self.source.source()
    }
    /// Actual unbound root and all executed approximation contributions;no source-edge rounding.
    #[must_use]
    pub const fn value(self) -> FormulaEvaluatedScalar {
        self.completed.value
    }
    /// Actual one-based completion ordinal within this execution;shared suppliers finish once.
    #[must_use]
    pub const fn completion_ordinal(self) -> usize {
        self.completed.ordinal
    }
    /// Only original name occurrences actually read,in their left-to-right execution order.
    /// Repeated names remain distinct;untaken branches remain unread.
    pub fn reads(self) -> impl DoubleEndedIterator<Item = FormulaScalarSourceGraphRead<'a>> + 'r {
        self.source.plan.sources.iter().filter_map(|slot| {
            let data = slot.read?;
            let Some(node) = slot.node else {
                unreachable!("actual prepared name occurrence");
            };
            Some(FormulaScalarSourceGraphRead { node, data })
        })
    }
}
impl fmt::Debug for FormulaExecutedScalarSource<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormulaExecutedScalarSource")
            .field("source", &self.source.source().normalized().source().id())
            .field("ordinal", &self.completed.ordinal)
            .field("kind", &self.completed.value.kind())
            .finish_non_exhaustive()
    }
}
