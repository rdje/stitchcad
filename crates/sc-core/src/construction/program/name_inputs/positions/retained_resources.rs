//! Fresh actual held proof records; source bodies and previous work policies compose separately.
use super::DraftingProgramOutputPositions;
use crate::{
    construction::DraftingContextSource,
    design::{
        DesignMetadataHost, DesignResourceLimitError, DesignResourceScope, DesignResourceSink,
    },
    ontology::EntityId,
};
impl<'b, 'n, 'p, 'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    DraftingProgramOutputPositions<'b, 'n, 'p, 'a, H, S>
{
    pub(crate) fn charge_retained_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
        scope: DesignResourceScope,
        owner: Option<EntityId>,
    ) -> Result<(), DesignResourceLimitError> {
        let Self { work } = self;
        work.charge_retained_resources(sink, scope, owner)
    }
}
