//! Complete actual higher roots, preserving one cumulative sink and original component locations.
use super::{
    charge_closure_inventory_resources, charge_construction_inventory_resources,
    charge_design_structure_resources, charge_hem_inventory_resources,
    charge_layer_inventory_resources, charge_material_inventory_resources,
    charge_notion_placement_inventory_resources, charge_parameter_resources,
    charge_pocket_inventory_resources, charge_recipe_document_resources,
    charge_sewing_graph_resources, charge_sewing_landmark_resources, fixed_slice, fixed_value,
    DesignResourceLimitError, DesignResourceLocation, DesignResourceScope, DesignResourceSink,
    DesignResourceSource,
};
use crate::design::{
    DesignGarment, DesignGarmentDefinition, DesignInputs, DesignInputsDefinition, DesignSewing,
    DesignSewingDefinition,
};
/// Charge the actual complete input root, including the source host and separately retained census.
/// Exhaustive private views follow schema order. References are fixed Copy payloads; the captured
/// identity vector is precharged without invoking a semantic census or currentness callback.
/// Success is logical accounting under the source port's honest/exhaustive integration obligation,
/// not authorization, currentness, allocator memory/time or runtime admission.
/// # Errors
/// Returns the original first refusal, including the actual containing Design for sources.entities.
pub fn charge_design_input_resources<H: DesignResourceSource>(
    inputs: &DesignInputs<H>,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let DesignInputsDefinition {
        structure,
        parameters,
        recipe,
        materials,
        sources,
        references,
    } = inputs.resource_definition();
    charge_design_structure_resources(structure, sink)?;
    charge_parameter_resources(parameters, sink)?;
    charge_recipe_document_resources(recipe, sink)?;
    charge_material_inventory_resources(materials, sink)?;
    let (host, entities) = sources.resource_parts();
    host.charge_resources(sink)?;
    sink.charge_entries(
        entities.len(),
        DesignResourceLocation {
            scope: DesignResourceScope::Inputs,
            owner: Some(structure.id()),
            field: "sources.entities",
            index: None,
        },
    )?;
    fixed_slice(entities);
    fixed_value(references);
    Ok(())
}
/// Charge the actual complete sewing root: inputs, original graph, then all retained landmarks.
/// No clone, reconstruction, current validation, numeric lookup or topology replay occurs.
/// # Errors
/// Returns the original first component refusal under the same cumulative sink.
pub fn charge_design_sewing_resources<H: DesignResourceSource>(
    sewing: &DesignSewing<H>,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let DesignSewingDefinition {
        inputs,
        graph,
        landmarks,
    } = sewing.resource_definition();
    charge_design_input_resources(inputs, sink)?;
    charge_sewing_graph_resources(graph, sink)?;
    charge_sewing_landmark_resources(landmarks, sink)
}
/// Charge every actual implemented garment root field in schema order, without resetting the sink.
/// Shared logical references are fixed; owned snapshots and vectors count at their actual owners.
/// This is the complete represented model over an honest resource source, not a runtime gate,
/// retained command-journal budget, physical/readiness proof or allocator memory/time bound.
/// # Errors
/// Returns the original first refusal, preserving component scope, owner and authored index.
pub fn charge_design_garment_resources<H: DesignResourceSource>(
    garment: &DesignGarment<H>,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let DesignGarmentDefinition {
        sewing,
        pockets,
        notions,
        closures,
        layers,
        hems,
        constructions,
    } = garment.resource_definition();
    charge_design_sewing_resources(sewing, sink)?;
    charge_pocket_inventory_resources(pockets, sink)?;
    charge_notion_placement_inventory_resources(notions, sink)?;
    charge_closure_inventory_resources(closures, sink)?;
    charge_layer_inventory_resources(layers, sink)?;
    charge_hem_inventory_resources(hems, sink)?;
    charge_construction_inventory_resources(constructions, sink)
}
