//! Fresh actual held proof records; source bodies and previous work policies compose separately.
use super::DraftingProgramDependencyInputs;
use crate::{
    construction::DraftingInitialInputProvider,
    design::{
        DesignMetadataHost, DesignResourceLimitError, DesignResourceScope, DesignResourceSink,
    },
    ontology::EntityId,
};
impl<
        'j,
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignMetadataHost,
        S: DraftingInitialInputProvider<H> + ?Sized,
    > DraftingProgramDependencyInputs<'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    pub(crate) fn charge_retained_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
        scope: DesignResourceScope,
        owner: Option<EntityId>,
    ) -> Result<(), DesignResourceLimitError> {
        let Self {
            sources,
            order,
            graph,
        } = self;
        // Exact owner admission already proves that order and source proofs share the same initials,
        // and the graph borrows this exact checked source proof. Charge each held proof component once.
        sources.charge_retained_resources(sink, scope, owner)?;
        order.charge_retained_resources(sink, scope, owner)?;
        graph.charge_retained_resources(sink, scope, owner)
    }
}
