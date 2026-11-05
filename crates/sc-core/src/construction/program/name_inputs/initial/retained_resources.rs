//! Fresh actual held proof records; source bodies and previous work policies compose separately.
use super::BoundDraftingProgramInitials;
use crate::{
    construction::{DraftingInitialBinding, DraftingInitialInputProvider},
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
impl<
        'i,
        'r,
        'b,
        'n,
        'p,
        'a,
        H: DesignMetadataHost,
        S: DraftingInitialInputProvider<H> + ?Sized,
    > BoundDraftingProgramInitials<'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    pub(crate) fn charge_retained_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
        scope: DesignResourceScope,
        owner: Option<EntityId>,
    ) -> Result<(), DesignResourceLimitError> {
        let Self { roles, sources } = self;
        roles.charge_retained_resources(sink, scope, owner)?;
        sink.charge_entries(
            sources.len(),
            at(scope, owner, "checked.drafting.initial_sources", None),
        )?;
        for source in sources {
            match source {
                DraftingInitialBinding::Input(source) => fixed_value(source),
                DraftingInitialBinding::Geometry(source) => fixed_value(source),
            }
        }
        Ok(())
    }
}
