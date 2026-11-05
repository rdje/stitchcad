//! Fresh actual held proof records; source bodies and previous work policies compose separately.
use super::BoundDraftingProgramGeometry;
use crate::{
    construction::{DraftingInitialInputProvider, DraftingProgramGeometrySupplier},
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
    > BoundDraftingProgramGeometry<'g, 'k, 'j, 'i, 'r, 'b, 'n, 'p, 'a, H, S>
{
    /// Freshly charge all actual held source/initial/role, mixed-order operand, graph,
    /// canonical-occurrence and geometry request/supplier proof records in one cumulative sink.
    /// Exact earlier owner joins let shared proof components be accounted once in this chain.
    /// Every vector is precharged before traversal; borrowed raw/normalized bodies, provider
    /// payloads, garment and semantic selection records compose separately through their owners.
    /// Earlier policies/demands and temporary namespace/cycle/checker workspace give no credit.
    ///
    /// The caller supplies the containing scope/owner. Native operand indexes are genuine one-based
    /// mixed positions; scalar operands keep their own original one-based statement ordinals.
    /// Source operand fields retain actual source IDs and zero-based source indexes. This performs
    /// no source/geometry/currentness/member query, allocation, clone, value read or new admission.
    /// The caller owns and finishes the meter; repeated composition charges the same logical
    /// occurrence again. Fixed-Copy and exhaustive owner/variant guards expose new owned fields.
    /// ```
    /// use sc_core::{construction::*,design::*,ontology::EntityId};
    /// fn retained<H:DesignMetadataHost,S:DraftingInitialInputProvider<H> + ?Sized>(
    ///     proof:&BoundDraftingProgramGeometry<'_, '_, '_, '_, '_, '_, '_, '_, '_,H,S>,
    ///     sink:&mut dyn DesignResourceSink,scope:DesignResourceScope,owner:Option<EntityId>)
    ///     ->Result<(),DesignResourceLimitError> {proof.charge_retained_resources(sink,scope,owner)}
    /// ```
    /// # Errors
    /// Original first held proof-record field refusal or earlier sink latch, without translation.
    pub fn charge_retained_resources(
        &self,
        sink: &mut dyn DesignResourceSink,
        scope: DesignResourceScope,
        owner: Option<EntityId>,
    ) -> Result<(), DesignResourceLimitError> {
        let Self {
            work,
            requests,
            suppliers,
        } = self;
        work.charge_retained_resources(sink, scope, owner)?;
        fixed_slice(requests);
        sink.charge_entries(
            requests.len(),
            at(scope, owner, "checked.drafting.geometry_requests", None),
        )?;
        sink.charge_entries(
            suppliers.len(),
            at(scope, owner, "checked.drafting.geometry_suppliers", None),
        )?;
        for supplier in suppliers {
            match supplier {
                DraftingProgramGeometrySupplier::Existing(binding) => fixed_value(binding),
                DraftingProgramGeometrySupplier::Fixed(request) => fixed_value(request),
                DraftingProgramGeometrySupplier::Planned { request, source } => {
                    fixed_value(request);
                    fixed_value(source);
                }
            }
        }
        Ok(())
    }
}
