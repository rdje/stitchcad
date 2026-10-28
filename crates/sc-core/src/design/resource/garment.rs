//! Actual held composition/closure resources, without current-context or physical validation.
use super::{
    fixed_slice, fixed_value, DesignResourceLimitError, DesignResourceLocation,
    DesignResourceScope, DesignResourceSink,
};
use crate::ontology::{
    ClosureDefinition, ClosureInventory, EntityId, NotionPlacementInventory, PocketDefinition,
    PocketInventory,
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

/// Charge actual Pockets and their component vectors in authored order.
/// Held source/copy, anchor, orientation and logical opening bindings remain fixed metadata.
/// No target lookup, rebirth, current validation, opening execution or geometry is performed.
/// # Errors
/// Returns the original first refusal with actual Pocket identity and original collection index.
pub fn charge_pocket_inventory_resources(
    inventory: &PocketInventory,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let pockets = inventory.resource_pockets();
    sink.charge_entries(pockets.len(), location("pockets", None, None))?;
    for (index, pocket) in pockets.iter().enumerate() {
        let PocketDefinition {
            id,
            served,
            position,
            orientation,
            opening,
            components,
        } = pocket.resource_definition();
        fixed_value(served);
        fixed_value(position);
        fixed_value(orientation);
        fixed_value(opening);
        fixed_slice(components);
        sink.charge_entries(
            components.len(),
            location("pockets.components", Some(*id), Some(index)),
        )?;
    }
    Ok(())
}

/// Charge the actual notion-placement vector, including placements unused by any Closure.
/// Exhaustive private views guard fixed original Piece/copy/anchor/direction payloads.
/// No plan lookup, anchor resolution/repair, rebirth or hardware realization is performed.
/// # Errors
/// Returns the original first refusal for notions (or an earlier latched error).
pub fn charge_notion_placement_inventory_resources(
    inventory: &NotionPlacementInventory,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let placements = inventory.resource_placements();
    sink.charge_entries(placements.len(), location("notions", None, None))?;
    for placement in placements {
        fixed_value(&placement.resource_fields());
    }
    Ok(())
}

/// Charge actual Closures and each instance vector without expanding or resolving their targets.
/// Fixed kind/size/derivation intent and the stored derived Count add no dynamic collection/text.
/// Exhaustive private owner/definition views and Copy guards expose owned-field evolution.
/// # Errors
/// Returns the original first refusal with actual Closure identity and original collection index.
pub fn charge_closure_inventory_resources(
    inventory: &ClosureInventory,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let closures = inventory.resource_closures();
    sink.charge_entries(closures.len(), location("closures", None, None))?;
    for (index, closure) in closures.iter().enumerate() {
        let (definition, count) = closure.resource_parts();
        let ClosureDefinition {
            id,
            kind,
            instances,
        } = definition;
        fixed_value(&count);
        fixed_value(kind);
        fixed_slice(instances);
        sink.charge_entries(
            instances.len(),
            location("closures.instances", Some(*id), Some(index)),
        )?;
    }
    Ok(())
}
