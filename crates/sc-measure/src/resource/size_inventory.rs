//! Complete actual external source-family composition, not a host or runtime admission certificate.
use super::{
    charge_garment_size_set_chart_resources, charge_mtm_size_set_chart_resources,
    charge_size_set_metadata_resources, location,
};
use crate::{GarmentSizeSetDefinition, MtmSizeSetDefinition, SizeSet, SizeSetInventory};
use sc_core::design::{DesignResourceLimitError, DesignResourceSink};
/// Charge the actual ordered set vector and every owned common/chart field in family/schema order.
/// Exhaustive private inventory/family views and explicit enum matching preserve Garment/MTM origin;
/// a singleton garment source never becomes MTM by inference. All observations, saved bindings and
/// explicit partial drafts remain actual held data; shared canonical/measurement owners compose
/// separately. No current validation, reference lookup, context, grading or instantiation is performed.
/// # Errors
/// Returns the original first refusal with the original component owner/field/index and cumulative units.
pub fn charge_size_set_inventory_resources(
    inventory: &SizeSetInventory,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let sets = inventory.resource_sets();
    sink.charge_entries(sets.len(), location("size_sets", None, None))?;
    for set in sets {
        match set {
            SizeSet::Garment(value) => {
                let GarmentSizeSetDefinition {
                    metadata,
                    observations: _,
                    chart: _,
                } = value.resource_definition();
                charge_size_set_metadata_resources(metadata, sink)?;
                charge_garment_size_set_chart_resources(value, sink)?;
            }
            SizeSet::Mtm(value) => {
                let MtmSizeSetDefinition { metadata, chart: _ } = value.resource_definition();
                charge_size_set_metadata_resources(metadata, sink)?;
                charge_mtm_size_set_chart_resources(value, sink)?;
            }
        }
    }
    Ok(())
}
