//! Fresh actual held proof records; source bodies and previous work policies compose separately.
use super::AdmittedDraftingProgramGeometryWork;
use crate::{
    construction::DraftingInitialInputProvider,
    design::{
        fixed_value, DesignMetadataHost, DesignResourceLimitError, DesignResourceScope,
        DesignResourceSink,
    },
    ontology::EntityId,
};
impl<
        'g,
        'k,
        'j,
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignMetadataHost,
        S: DraftingInitialInputProvider<H> + ?Sized,
    > AdmittedDraftingProgramGeometryWork<'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    pub(crate) fn charge_retained_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
        scope: DesignResourceScope,
        owner: Option<EntityId>,
    ) -> Result<(), DesignResourceLimitError> {
        let Self {
            inputs,
            limits,
            demand,
            shape,
        } = self;
        fixed_value(limits);
        fixed_value(demand);
        fixed_value(shape);
        inputs.charge_retained_resources(sink, scope, owner)
    }
}
