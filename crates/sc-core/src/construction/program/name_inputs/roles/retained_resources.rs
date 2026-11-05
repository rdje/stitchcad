//! Fresh actual held proof records; source bodies and previous work policies compose separately.
use super::BoundDraftingProgramOutputRoles;
use crate::{
    construction::DraftingContextSource,
    design::{
        fixed_slice, DesignMetadataHost, DesignResourceLimitError, DesignResourceLocation,
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
impl<'r, 'b, 'n, 'p, 'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>
    BoundDraftingProgramOutputRoles<'r, 'b, 'n, 'p, 'a, H, S>
{
    pub(crate) fn charge_retained_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
        scope: DesignResourceScope,
        owner: Option<EntityId>,
    ) -> Result<(), DesignResourceLimitError> {
        let Self { positions, sources } = self;
        positions.charge_retained_resources(sink, scope, owner)?;
        fixed_slice(sources);
        sink.charge_entries(
            sources.len(),
            at(scope, owner, "checked.drafting.role_sources", None),
        )
    }
}
