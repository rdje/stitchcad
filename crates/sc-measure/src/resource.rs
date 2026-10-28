//! Logical held source content; canonical values remain owned by sc-core.
use crate::{
    EaseBinding, EaseDefinition, EaseSetDefinition, LandmarkDefinition, MeasurementBinding,
    MeasurementDefinition, MeasurementInputs, MeasurementInputsDefinition,
    MeasurementInventoryDefinition, MeasurementProcedureDefinition, MeasurementTableDefinition,
};
use sc_core::{
    design::{
        DesignResourceLimitError, DesignResourceLocation, DesignResourceScope, DesignResourceSink,
    },
    ontology::EntityId,
};
mod size_metadata;
pub use size_metadata::charge_size_set_metadata_resources;

fn fixed<T: Copy>(_: &T) {}
fn fixed_slice<T: Copy>(_: &[T]) {}
fn location(
    field: &'static str,
    owner: Option<EntityId>,
    index: Option<usize>,
) -> DesignResourceLocation {
    DesignResourceLocation {
        scope: DesignResourceScope::SourceHost,
        owner,
        field,
        index,
    }
}
fn charge_binding(
    binding: &MeasurementBinding,
    field: &'static str,
    owner: EntityId,
    index: usize,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let MeasurementBinding {
        measurement,
        token,
        kind,
        declaration,
    } = binding;
    fixed(measurement);
    fixed(kind);
    fixed(declaration);
    sink.charge_text(token.as_str(), location(field, Some(owner), Some(index)))
}
/// Charge actual owned measurement/Ease vectors and every retained name/document/token string.
/// Schema/member order is preserved. Inline landmark pairs and other fixed reference/domain fields
/// add no dynamic entries. Every saved binding's token counts, even when another owner's spelling
/// matches. Canonical values/evidence are not duplicated, read, formatted or resolved here.
/// Exhaustive private owner/definition/binding views expose future owned-field obligations.
/// No current validation, context construction, provenance truth, formula evaluation or physical proof.
/// # Errors
/// Returns the original first refusal with actual owner and original field/member index.
/// For table/set entry text the index is the original entry; its owner is the actual table/set.
pub fn charge_measurement_input_resources(
    inputs: &MeasurementInputs,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let MeasurementInputsDefinition {
        measurements,
        eases,
        ease_sets,
    } = inputs.resource_definition();
    let MeasurementInventoryDefinition {
        landmarks,
        procedures,
        measurements,
        tables,
    } = measurements.resource_definition();
    sink.charge_entries(
        landmarks.len(),
        location("measurements.landmarks", None, None),
    )?;
    for (index, landmark) in landmarks.iter().enumerate() {
        let LandmarkDefinition {
            id,
            name,
            kind,
            source,
        } = landmark.resource_definition();
        fixed(kind);
        fixed(source);
        sink.charge_text(
            name,
            location("measurements.landmarks.name", Some(*id), Some(index)),
        )?;
    }
    sink.charge_entries(
        procedures.len(),
        location("measurements.procedures", None, None),
    )?;
    for (index, procedure) in procedures.iter().enumerate() {
        let MeasurementProcedureDefinition {
            id,
            name,
            kind,
            documentation,
            source,
        } = procedure.resource_definition();
        fixed(kind);
        fixed(source);
        sink.charge_text(
            name,
            location("measurements.procedures.name", Some(*id), Some(index)),
        )?;
        sink.charge_text(
            documentation,
            location(
                "measurements.procedures.documentation",
                Some(*id),
                Some(index),
            ),
        )?;
    }
    sink.charge_entries(
        measurements.len(),
        location("measurements.records", None, None),
    )?;
    for (index, measurement) in measurements.iter().enumerate() {
        let MeasurementDefinition {
            id,
            name,
            token,
            entered_unit,
            kind,
            landmarks,
            procedure,
            declaration,
        } = measurement.resource_definition();
        fixed(entered_unit);
        fixed(kind);
        fixed(landmarks);
        fixed(procedure);
        fixed(declaration);
        sink.charge_text(
            name,
            location("measurements.records.name", Some(*id), Some(index)),
        )?;
        sink.charge_text(
            token.as_str(),
            location("measurements.records.token", Some(*id), Some(index)),
        )?;
    }
    sink.charge_entries(tables.len(), location("measurements.tables", None, None))?;
    for (index, table) in tables.iter().enumerate() {
        let MeasurementTableDefinition { id, name, entries } = table.resource_definition();
        sink.charge_text(
            name,
            location("measurements.tables.name", Some(*id), Some(index)),
        )?;
        sink.charge_entries(
            entries.len(),
            location("measurements.tables.entries", Some(*id), Some(index)),
        )?;
        for (entry, binding) in entries.iter().enumerate() {
            charge_binding(
                binding,
                "measurements.tables.entries.token",
                *id,
                entry,
                sink,
            )?;
        }
    }
    sink.charge_entries(eases.len(), location("measurements.eases", None, None))?;
    for (index, ease) in eases.iter().enumerate() {
        let EaseDefinition {
            id,
            body,
            garment,
            declaration,
            fit,
            provenance,
            compression,
        } = ease.resource_definition();
        fixed(declaration);
        fixed(fit);
        fixed(provenance);
        fixed(compression);
        charge_binding(body, "measurements.eases.body.token", *id, index, sink)?;
        charge_binding(
            garment,
            "measurements.eases.garment.token",
            *id,
            index,
            sink,
        )?;
    }
    sink.charge_entries(
        ease_sets.len(),
        location("measurements.ease_sets", None, None),
    )?;
    for (index, set) in ease_sets.iter().enumerate() {
        let EaseSetDefinition {
            id,
            body_table,
            garment_table,
            entries,
        } = set.resource_definition();
        fixed(body_table);
        fixed(garment_table);
        sink.charge_entries(
            entries.len(),
            location("measurements.ease_sets.entries", Some(*id), Some(index)),
        )?;
        for (entry, binding) in entries.iter().enumerate() {
            let EaseBinding {
                ease,
                token,
                body,
                garment,
                declaration,
            } = binding;
            fixed(ease);
            fixed(declaration);
            sink.charge_text(
                token.as_str(),
                location(
                    "measurements.ease_sets.entries.token",
                    Some(*id),
                    Some(entry),
                ),
            )?;
            charge_binding(
                body,
                "measurements.ease_sets.entries.body.token",
                *id,
                entry,
                sink,
            )?;
            charge_binding(
                garment,
                "measurements.ease_sets.entries.garment.token",
                *id,
                entry,
                sink,
            )?;
        }
    }
    Ok(())
}
