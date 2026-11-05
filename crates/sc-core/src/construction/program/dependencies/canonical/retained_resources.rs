//! Fresh actual held proof records; source bodies and previous work policies compose separately.
use super::BoundDraftingProgramCanonicalInputs;
use crate::{
    construction::DraftingInitialInputProvider,
    design::{
        fixed_slice, fixed_value, DesignMetadataHost, DesignResourceLimitError,
        DesignResourceLocation, DesignResourceScope, DesignResourceSink,
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
    > BoundDraftingProgramCanonicalInputs<'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
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
            shape,
            occurrences,
        } = self;
        fixed_value(limits);
        fixed_value(shape);
        inputs.charge_retained_resources(sink, scope, owner)?;
        fixed_slice(occurrences);
        sink.charge_entries(
            occurrences.len(),
            at(scope, owner, "checked.drafting.canonical_occurrences", None),
        )
    }
}
