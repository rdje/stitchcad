//! Actual common SizeSet metadata; source/canonical contexts and chart families remain separate.
use super::{charge_binding, fixed, fixed_slice, location};
use crate::{
    SizeAxesDefinition, SizeAxisDefinition, SizeAxisValueDefinition, SizeAxisValueSource,
    SizeBreakDefinition, SizeBreakDimension, SizeBreaksDefinition, SizeMember, SizeMemberPoint,
    SizeMembershipDefinition, SizeSetMetadata, SizeSetMetadataDefinition,
};
use sc_core::design::{DesignResourceLimitError, DesignResourceSink};
/// Charge actual member/axis/value/point/coordinate/required-dimension/break collections and text.
/// Exhaustive private owner/definition/label views retain original order and expose owned-field changes.
/// Fixed Count revisions add no unbounded magnitude bits; no revision advance, canonical lookup,
/// numeric/adjacency/coverage evaluation, context construction or inferred Cartesian grid occurs.
/// Saved break-dimension tokens count per held owner even when their spellings match another binding.
/// # Errors
/// Returns the original first refusal with actual set/member/axis/value/break owner and authored index.
pub fn charge_size_set_metadata_resources(
    metadata: &SizeSetMetadata,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let SizeSetMetadataDefinition {
        membership,
        axes,
        breaks,
        design_table,
    } = metadata.resource_definition();
    fixed(design_table);
    let SizeMembershipDefinition {
        reference,
        system,
        members,
        base,
    } = membership.resource_definition();
    fixed(reference);
    fixed(system);
    fixed(base);
    let set = reference.id;
    sink.charge_entries(
        members.len(),
        location("size_sets.members", Some(set), None),
    )?;
    for (index, member) in members.iter().enumerate() {
        let SizeMember { id, label } = member;
        sink.charge_text(
            label.resource_text(),
            location("size_sets.members.label", Some(*id), Some(index)),
        )?;
    }
    let SizeAxesDefinition {
        reference,
        axes,
        points,
    } = axes.resource_definition();
    fixed(reference);
    sink.charge_entries(axes.len(), location("size_sets.axes", Some(set), None))?;
    for (index, axis) in axes.iter().enumerate() {
        let SizeAxisDefinition {
            reference,
            id,
            name,
            kind,
            values,
        } = axis.resource_definition();
        fixed(reference);
        fixed(kind);
        sink.charge_text(
            name,
            location("size_sets.axes.name", Some(*id), Some(index)),
        )?;
        sink.charge_entries(
            values.len(),
            location("size_sets.axes.values", Some(*id), Some(index)),
        )?;
        for (value_index, value) in values.iter().enumerate() {
            let SizeAxisValueDefinition { id, source } = value;
            match source {
                SizeAxisValueSource::Label(label) => sink.charge_text(
                    label.resource_text(),
                    location("size_sets.axes.values.label", Some(*id), Some(value_index)),
                )?,
                SizeAxisValueSource::Length(target) | SizeAxisValueSource::Scalar(target) => {
                    fixed(target)
                }
            }
        }
    }
    sink.charge_entries(points.len(), location("size_sets.points", Some(set), None))?;
    for (index, point) in points.iter().enumerate() {
        let SizeMemberPoint {
            member,
            coordinates,
        } = point;
        fixed(member);
        fixed_slice(coordinates);
        sink.charge_entries(
            coordinates.len(),
            location("size_sets.points.coordinates", Some(set), Some(index)),
        )?;
    }
    let SizeBreaksDefinition {
        reference,
        dimensions,
        breaks,
    } = breaks.resource_definition();
    fixed(reference);
    sink.charge_entries(
        dimensions.len(),
        location("size_sets.break_dimensions", Some(set), None),
    )?;
    for (index, dimension) in dimensions.iter().enumerate() {
        let SizeBreakDimension {
            axis,
            dimension_table,
            dimension,
        } = dimension;
        fixed(axis);
        fixed(dimension_table);
        charge_binding(
            dimension,
            "size_sets.break_dimensions.token",
            set,
            index,
            sink,
        )?;
    }
    sink.charge_entries(breaks.len(), location("size_sets.breaks", Some(set), None))?;
    for (index, declaration) in breaks.iter().enumerate() {
        let SizeBreakDefinition {
            id,
            reference,
            axis,
            from,
            to,
            dimension_table,
            dimension,
            delta,
        } = declaration.resource_definition();
        fixed(reference);
        fixed(axis);
        fixed(from);
        fixed(to);
        fixed(dimension_table);
        fixed(delta);
        charge_binding(
            dimension,
            "size_sets.breaks.dimension.token",
            *id,
            index,
            sink,
        )?;
    }
    Ok(())
}
