//! Actual structural root composition, without higher metadata or whole-garment admission.
use super::{
    charge_cut_plan_resources, charge_piece_inventory_resources, charge_topology_resources,
    DesignResourceLimitError, DesignResourceLocation, DesignResourceScope, DesignResourceSink,
};
use crate::design::{DesignStructure, DesignStructureDefinition};
use crate::value::LengthDeclaration;
/// Charge the actual structural root under the caller's cumulative meter, in schema order.
/// Exact revision magnitude bits precede canonical inputs, Pieces, copies and topology. Zero has zero
/// magnitude bits. No formatting, counter advance, model copy, current validation or value lookup occurs.
/// Exhaustive private owner/definition views expose future root-field accounting obligations.
/// This covers DesignStructure only; higher Inputs/Sewing/Garment and actual source-host work compose
/// separately. Success is not whole-model, runtime, source-currentness, physical or allocator admission.
/// # Errors
/// Returns the original first refusal; revision refusal carries the actual Design identity.
pub fn charge_design_structure_resources(
    structure: &DesignStructure,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    charge_structure_with_length(structure, None, sink)
}
pub(super) fn charge_structure_with_length(
    structure: &DesignStructure,
    replacement: Option<&LengthDeclaration>,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let DesignStructureDefinition {
        id,
        revision,
        inputs,
        pieces,
        cut_plan,
        topology,
    } = structure.resource_definition();
    sink.charge_counter_bits(
        revision.resource_bits(),
        DesignResourceLocation {
            scope: DesignResourceScope::Structure,
            owner: Some(*id),
            field: "design.revision",
            index: None,
        },
    )?;
    super::canonical::charge_canonical_with_length(inputs, replacement, sink)?;
    charge_piece_inventory_resources(pieces, sink)?;
    charge_cut_plan_resources(cut_plan, sink)?;
    charge_topology_resources(topology, sink)
}
