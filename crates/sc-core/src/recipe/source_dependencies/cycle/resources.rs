//! The successful acyclic proof retains only its original relation and fixed policy metadata.
use super::FormulaAcyclicSourceDependencies;
use crate::{
    design::{fixed_value, DesignResourceLimitError, DesignResourceScope, DesignResourceSink},
    ontology::EntityId,
};
impl FormulaAcyclicSourceDependencies<'_> {
    /// Freshly account the exact borrowed relation's edge/offset arrays. The successful acyclic
    /// wrapper owns no dynamic workspace; its earlier scratch reservation is not held content.
    /// The relation's borrowed source proofs and normalized owners compose separately. This does
    /// no new cycle traversal, source query, allocation, value read or meter finalization.
    /// # Errors
    /// Original first relation-field refusal or earlier cumulative sink latch.
    pub fn charge_retained_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
        scope: DesignResourceScope,
        owner: Option<EntityId>,
    ) -> Result<(), DesignResourceLimitError> {
        let Self {
            dependencies,
            limits,
            workspace_records,
        } = self;
        fixed_value(limits);
        fixed_value(workspace_records);
        dependencies.charge_retained_resources(sink, scope, owner)
    }
}
