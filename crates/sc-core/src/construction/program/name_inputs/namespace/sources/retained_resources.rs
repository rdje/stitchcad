//! Fresh actual held proof records; source bodies and previous work policies compose separately.
use super::CheckedDraftingProgramSources;
use crate::{
    construction::DraftingInitialInputProvider,
    design::{
        fixed_value, DesignMetadataHost, DesignResourceLimitError, DesignResourceScope,
        DesignResourceSink,
    },
    ontology::EntityId,
};
impl<
        'q,
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignMetadataHost,
        S: DraftingInitialInputProvider<H> + ?Sized,
    > CheckedDraftingProgramSources<'q, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    pub(crate) fn charge_retained_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
        scope: DesignResourceScope,
        owner: Option<EntityId>,
    ) -> Result<(), DesignResourceLimitError> {
        let Self {
            inputs,
            normalized,
            proof,
        } = self;
        fixed_value(normalized);
        inputs.charge_retained_resources(sink, scope, owner)?;
        proof.charge_retained_resources(sink, scope, owner)
    }
}
