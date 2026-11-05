//! Fresh actual held proof records; source bodies and previous work policies compose separately.
use super::AdmittedDraftingProgramNameWork;
use crate::{
    construction::DraftingContextSource,
    design::{
        fixed_value, DesignMetadataHost, DesignResourceLimitError, DesignResourceLocation,
        DesignResourceScope, DesignResourceSink,
    },
    ontology::EntityId,
};
fn at(
    scope: DesignResourceScope,
    owner: Option<EntityId>,
    field: &'static str,
    index: Option<usize>,
) -> DesignResourceLocation {
    DesignResourceLocation {
        scope,
        owner,
        field,
        index,
    }
}
impl<'b, 'n, 'p, 'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    AdmittedDraftingProgramNameWork<'b, 'n, 'p, 'a, H, S>
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
        fixed_value(inputs);
        fixed_value(limits);
        fixed_value(demand);
        fixed_value(shape);
        sink.charge_entries(0, at(scope, owner, "checked.drafting.name_work", None))
    }
}
