//! Complete actual higher roots, preserving one cumulative sink and original component locations.
use super::{
    charge_closure_inventory_resources, charge_construction_inventory_resources,
    charge_hem_inventory_resources, charge_layer_inventory_resources,
    charge_material_inventory_resources, charge_notion_placement_inventory_resources,
    charge_parameter_resources, charge_pocket_inventory_resources,
    charge_recipe_document_resources, charge_sewing_graph_resources, fixed_slice, fixed_value,
    DesignResourceLimitError, DesignResourceLocation, DesignResourceScope, DesignResourceSink,
    DesignResourceSource, ResourceProjection,
};
use crate::design::{
    DesignGarment, DesignGarmentDefinition, DesignInputs, DesignInputsDefinition, DesignSewing,
    DesignSewingDefinition,
};
use crate::value::LengthDeclaration;
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
    charge_inputs_projected(inputs, ResourceProjection::Current, sink)
}
fn charge_inputs_projected<H: DesignResourceSource>(
    inputs: &DesignInputs<H>,
    projection: ResourceProjection<'_>,
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
    super::structure::charge_structure_projected(structure, projection, sink)?;
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
    charge_sewing_projected(sewing, ResourceProjection::Current, sink)
}
fn charge_sewing_projected<H: DesignResourceSource>(
    sewing: &DesignSewing<H>,
    projection: ResourceProjection<'_>,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let DesignSewingDefinition {
        inputs,
        graph,
        landmarks,
    } = sewing.resource_definition();
    charge_inputs_projected(inputs, projection, sink)?;
    charge_sewing_graph_resources(graph, sink)?;
    let appended = if let ResourceProjection::AppendedNotch(notch) = projection {
        Some(notch)
    } else {
        None
    };
    super::sewing::charge_landmarks_appended(landmarks, appended, sink)
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
    charge_garment_projected(garment, ResourceProjection::Current, sink)
}
fn charge_garment_projected<H: DesignResourceSource>(
    garment: &DesignGarment<H>,
    projection: ResourceProjection<'_>,
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
    charge_sewing_projected(sewing, projection, sink)?;
    charge_pocket_inventory_resources(pockets, sink)?;
    charge_notion_placement_inventory_resources(notions, sink)?;
    charge_closure_inventory_resources(closures, sink)?;
    charge_layer_inventory_resources(layers, sink)?;
    charge_hem_inventory_resources(hems, sink)?;
    charge_construction_inventory_resources(constructions, sink)
}

// The caller has resolved a real Measurement binding and checked this exact canonical identity.
// This private logical projection neither installs data nor skips subsequent semantic validation.
pub(crate) fn charge_garment_length_replacement<H: DesignResourceSource>(
    garment: &DesignGarment<H>,
    replacement: &LengthDeclaration,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    charge_garment_projected(garment, ResourceProjection::Length(replacement), sink)
}

// The command has checked every selected actual Piece before entering this borrowed projection.
pub(crate) fn charge_garment_material_assignment<H: DesignResourceSource>(
    garment: &DesignGarment<H>,
    pieces: &[crate::ontology::EntityId],
    assignment: &crate::ontology::MaterialAssignment,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    charge_garment_projected(
        garment,
        ResourceProjection::Material { pieces, assignment },
        sink,
    )
}

// The command has peeked an actual reserved ID and validated this fixed semantic payload. No
// staged ID has been consumed and no model/registry vector has been copied or grown yet.
pub(crate) fn charge_garment_notch_append<H: DesignResourceSource>(
    garment: &DesignGarment<H>,
    notch: &crate::ontology::Notch,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    charge_garment_projected(garment, ResourceProjection::AppendedNotch(notch), sink)
}

// This is the private journal's actual independent maximum-issued revision, never the restored
// model revision or an untrusted request claim. Only its next magnitude is forecast here.
pub(crate) fn charge_garment_successor_revision<H: DesignResourceSource>(
    garment: &DesignGarment<H>,
    maximum_issued: &crate::command::DesignRevision,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    charge_garment_projected(
        garment,
        ResourceProjection::SuccessorRevision(maximum_issued),
        sink,
    )
}
