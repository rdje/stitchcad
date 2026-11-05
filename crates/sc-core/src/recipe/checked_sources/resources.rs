//! Actual source-proof owners and every original expression-proof buffer, without syntax copies.
use super::{FormulaCheckedSource, FormulaCheckedSources};
use crate::{
    design::{
        fixed_value, DesignResourceLimitError, DesignResourceLocation, DesignResourceScope,
        DesignResourceSink,
    },
    ontology::EntityId,
};
impl FormulaCheckedSources<'_> {
    /// Precharge the complete actual source-proof vector, then each certificate/dependency vector
    /// in original source order. Outer scope/owner come from the containing caller; operand fields
    /// use the actual original source ID and zero-based source index. Unused sources count too.
    /// Normalized syntax and canonical/declaration bodies remain borrowed and compose separately.
    /// Earlier reservation limits/demand provide no credit; no lookup, values, allocation, clone
    /// or meter reset occurs. This component grants no complete source/content/work admission.
    /// # Errors
    /// Original first source-vector or operand field refusal, including a prior latch when empty.
    pub fn charge_retained_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
        scope: DesignResourceScope,
        owner: Option<EntityId>,
    ) -> Result<(), DesignResourceLimitError> {
        let Self {
            normalized,
            limits,
            target_count,
            retained_records,
            sources,
        } = self;
        fixed_value(normalized);
        fixed_value(limits);
        fixed_value(target_count);
        fixed_value(retained_records);
        sink.charge_entries(
            sources.len(),
            DesignResourceLocation {
                scope,
                owner,
                field: "checked.sources",
                index: None,
            },
        )?;
        for (index, source) in sources.iter().enumerate() {
            let FormulaCheckedSource { normalized, proof } = source;
            fixed_value(normalized);
            let at = |field| DesignResourceLocation {
                scope,
                owner: Some(normalized.source().id()),
                field,
                index: Some(index),
            };
            proof.charge_retained_resources(
                sink,
                at("checked.sources.certificates"),
                at("checked.sources.dependencies"),
            )?;
        }
        Ok(())
    }
}
