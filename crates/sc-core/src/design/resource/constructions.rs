//! Actual construction content; declared uses never expand into other owning collections.
use super::{
    fixed_slice, fixed_value, DesignResourceLimitError, DesignResourceLocation,
    DesignResourceScope, DesignResourceSink,
};
use crate::ontology::{
    ConstructionInventory, DartDefinition, EntityId, FoldDefinition, GatherDefinition,
    GrainlineDefinition, Pleat, SeamAllowanceDefinition, Tuck,
};
fn location(
    field: &'static str,
    owner: Option<EntityId>,
    index: Option<usize>,
) -> DesignResourceLocation {
    DesignResourceLocation {
        scope: DesignResourceScope::Garment,
        owner,
        field,
        index,
    }
}
/// Charge all six actual family vectors and Tuck/Pleat fold-line vectors in schema/member order.
/// Exhaustive private owner/definition views and fixed Copy guards preserve all original source,
/// operation, profile, copy and sewing uses. No lookup, rebirth, source resolution, ease/angle/width
/// evaluation, physical count expansion or construction execution is performed.
/// # Errors
/// Returns the original first refusal with actual fold identity and original within-family index.
pub fn charge_construction_inventory_resources(
    inventory: &ConstructionInventory,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let collections = inventory.resource_collections();
    let darts = collections.darts;
    let tucks = collections.tucks;
    let pleats = collections.pleats;
    let gathers = collections.gathers;
    let grainlines = collections.grainlines;
    let allowances = collections.allowances;
    sink.charge_entries(darts.len(), location("constructions.darts", None, None))?;
    for dart in darts {
        let (piece, definition) = dart.resource_parts();
        fixed_value(&piece);
        let DartDefinition {
            id,
            intake,
            apex,
            first_leg,
            second_leg,
            direction,
            closing_operation,
        } = definition;
        fixed_value(id);
        fixed_value(intake);
        fixed_value(apex);
        fixed_value(first_leg);
        fixed_value(second_leg);
        fixed_value(direction);
        fixed_value(closing_operation);
    }
    charge_folds(
        tucks.iter().map(Tuck::resource_parts),
        "constructions.tucks",
        "constructions.tucks.fold_lines",
        sink,
    )?;
    charge_folds(
        pleats.iter().map(Pleat::resource_parts),
        "constructions.pleats",
        "constructions.pleats.fold_lines",
        sink,
    )?;
    sink.charge_entries(gathers.len(), location("constructions.gathers", None, None))?;
    for gather in gathers {
        let (piece, definition) = gather.resource_parts();
        fixed_value(&piece);
        let GatherDefinition {
            id,
            graph,
            span,
            side,
            copy,
            direction,
            closing_operation,
        } = definition;
        fixed_value(id);
        fixed_value(graph);
        fixed_value(span);
        fixed_value(side);
        fixed_value(copy);
        fixed_value(direction);
        fixed_value(closing_operation);
    }
    sink.charge_entries(
        grainlines.len(),
        location("constructions.grainlines", None, None),
    )?;
    for grain in grainlines {
        let (piece, definition) = grain.resource_parts();
        fixed_value(&piece);
        let GrainlineDefinition {
            id,
            arrow,
            alignment,
            stripe,
            plaid,
        } = definition;
        fixed_value(id);
        fixed_value(arrow);
        fixed_value(alignment);
        fixed_value(stripe);
        fixed_value(plaid);
    }
    sink.charge_entries(
        allowances.len(),
        location("constructions.allowances", None, None),
    )?;
    for allowance in allowances {
        let (piece, definition) = allowance.resource_parts();
        fixed_value(&piece);
        let SeamAllowanceDefinition {
            id,
            edge,
            width,
            corner,
            inclusion,
        } = definition;
        fixed_value(id);
        fixed_value(edge);
        fixed_value(width);
        fixed_value(corner);
        fixed_value(inclusion);
    }
    Ok(())
}
fn charge_folds<'a>(
    definitions: impl ExactSizeIterator<Item = (EntityId, &'a FoldDefinition)>,
    family: &'static str,
    lines: &'static str,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    sink.charge_entries(definitions.len(), location(family, None, None))?;
    for (index, (piece, definition)) in definitions.enumerate() {
        fixed_value(&piece);
        let FoldDefinition {
            id,
            intake,
            fold_lines,
            direction,
            closing_operation,
        } = definition;
        fixed_value(intake);
        fixed_value(direction);
        fixed_value(closing_operation);
        fixed_slice(fold_lines);
        sink.charge_entries(fold_lines.len(), location(lines, Some(*id), Some(index)))?;
    }
    Ok(())
}
