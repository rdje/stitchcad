//! Actual retained garment/MTM chart fields; common metadata is charged through its own producer.
use super::{charge_binding, fixed, location};
use crate::{
    EaseBinding, EaseSetDefinition, GarmentSizeSet, GarmentSizeSetDefinition, MtmChartDefinition,
    MtmSizeSet, MtmSizeSetDefinition, SizeChartBinding, SizeChartDefinition,
    SizeChartObservationDefinition,
};
use sc_core::design::{DesignResourceLimitError, DesignResourceSink};
/// Charge an actual garment source family's complete observation registry and saved chart fields.
/// Common metadata is separate. Even unused observations and every saved token copy count; no value,
/// current context, provenance, completeness or geometry is read, reconstructed or certified.
/// Exhaustive private family/observation/chart/binding views expose owned-field evolution.
/// # Errors
/// Returns the original first refusal with actual Set, observation or chart owner and authored index.
pub fn charge_garment_size_set_chart_resources(
    set: &GarmentSizeSet,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let GarmentSizeSetDefinition {
        metadata,
        observations,
        chart,
    } = set.resource_definition();
    sink.charge_entries(
        observations.len(),
        location(
            "garment_sets.observations",
            Some(metadata.reference().id),
            None,
        ),
    )?;
    for (index, observation) in observations.iter().enumerate() {
        let SizeChartObservationDefinition {
            id,
            membership,
            member,
            design_table,
            chart_table,
            pom,
            measurement,
            provenance,
        } = observation.resource_definition();
        fixed(membership);
        fixed(member);
        fixed(design_table);
        fixed(chart_table);
        fixed(provenance);
        charge_binding(pom, "garment_sets.observations.pom.token", *id, index, sink)?;
        charge_binding(
            measurement,
            "garment_sets.observations.measurement.token",
            *id,
            index,
            sink,
        )?;
    }
    let SizeChartDefinition {
        id,
        membership,
        design_table,
        poms,
        observations,
    } = chart.resource_definition();
    fixed(membership);
    fixed(design_table);
    sink.charge_entries(
        poms.len(),
        location("garment_sets.chart.poms", Some(*id), None),
    )?;
    for (index, binding) in poms.iter().enumerate() {
        charge_binding(binding, "garment_sets.chart.poms.token", *id, index, sink)?;
    }
    sink.charge_entries(
        observations.len(),
        location("garment_sets.chart.observations", Some(*id), None),
    )?;
    for (index, binding) in observations.iter().enumerate() {
        let SizeChartBinding {
            observation,
            membership,
            member,
            design_table,
            chart_table,
            pom,
            measurement,
        } = binding;
        fixed(observation);
        fixed(membership);
        fixed(member);
        fixed(design_table);
        fixed(chart_table);
        charge_binding(
            pom,
            "garment_sets.chart.observations.pom.token",
            *id,
            index,
            sink,
        )?;
        charge_binding(
            measurement,
            "garment_sets.chart.observations.measurement.token",
            *id,
            index,
            sink,
        )?;
    }
    Ok(())
}
/// Charge the actual MTM family's retained chart snapshot, excluding its common metadata.
/// The saved EaseSetDefinition owns its entry vector and binding-token strings even though its id
/// references the authoritative Ease set. Charge these copies once here; do not look up or recharge
/// that authoritative owner, read canonical body/Ease values or synthesize a garment result.
/// # Errors
/// Returns the original first refusal with actual chart owner and original snapshot-entry index.
pub fn charge_mtm_size_set_chart_resources(
    set: &MtmSizeSet,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let MtmSizeSetDefinition { metadata: _, chart } = set.resource_definition();
    let MtmChartDefinition {
        id,
        membership,
        member,
        ease_set,
        provenance,
    } = chart.resource_definition();
    fixed(membership);
    fixed(member);
    fixed(provenance);
    let EaseSetDefinition {
        id: ease_id,
        body_table,
        garment_table,
        entries,
    } = ease_set;
    fixed(ease_id);
    fixed(body_table);
    fixed(garment_table);
    sink.charge_entries(
        entries.len(),
        location("mtm_sets.chart.ease_snapshot.entries", Some(*id), None),
    )?;
    for (index, binding) in entries.iter().enumerate() {
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
                "mtm_sets.chart.ease_snapshot.entries.token",
                Some(*id),
                Some(index),
            ),
        )?;
        charge_binding(
            body,
            "mtm_sets.chart.ease_snapshot.entries.body.token",
            *id,
            index,
            sink,
        )?;
        charge_binding(
            garment,
            "mtm_sets.chart.ease_snapshot.entries.garment.token",
            *id,
            index,
            sink,
        )?;
    }
    Ok(())
}
