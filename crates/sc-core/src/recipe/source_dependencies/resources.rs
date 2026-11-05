//! Actual relation arrays; source proofs and temporary cycle workspace compose separately.
use super::FormulaSourceDependencies;
use crate::{
    design::{
        fixed_slice, fixed_value, DesignResourceLimitError, DesignResourceLocation,
        DesignResourceScope, DesignResourceSink,
    },
    ontology::EntityId,
};
impl FormulaSourceDependencies<'_> {
    /// Charge the actual Derived-edge vector, then actual source offsets including its terminal
    /// sentinel. Even an empty relation retains that one offset. Fixed metadata/borrowed checked
    /// source proofs own no additional buffers here; those source proofs compose separately.
    /// The caller selects the containing scope/owner and finishes the cumulative meter. Original
    /// policy/reservation totals give no credit; no graph traversal, lookup, values or allocation
    /// occurs. This is logical retained content, not allocator capacity or cycle scratch.
    /// # Errors
    /// Original first edge/offset field refusal, including an earlier latch on zero edges.
    pub fn charge_retained_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
        scope: DesignResourceScope,
        owner: Option<EntityId>,
    ) -> Result<(), DesignResourceLimitError> {
        let Self {
            checked,
            limits,
            retained_records,
            edges,
            offsets,
        } = self;
        fixed_value(checked);
        fixed_value(limits);
        fixed_value(retained_records);
        fixed_slice(edges);
        fixed_slice(offsets);
        let at = |field| DesignResourceLocation {
            scope,
            owner,
            field,
            index: None,
        };
        sink.charge_entries(edges.len(), at("checked.source_dependencies.edges"))?;
        sink.charge_entries(offsets.len(), at("checked.source_dependencies.offsets"))
    }
}
