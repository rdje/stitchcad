//! Actual held layers and Hems, without envelope, source or physical validation.
use super::{
    fixed_slice, fixed_value, DesignResourceLimitError, DesignResourceLocation,
    DesignResourceScope, DesignResourceSink,
};
use crate::ontology::{
    EntityId, Facing, HemDefinition, HemInventory, Interfacing, LayerDefinition, LayerInventory,
    LayerOffsetRelationship, Lining, MaterialAssignment,
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
/// Charge actual Facing/Lining/Interfacing collections, offset sources and unresolved reason bytes.
/// Families retain schema order, then original member order. Modelled Lining stays modelled: no scope
/// grant, offset execution, source/Material lookup, current validation or generated geometry follows.
/// Exhaustive private owner/definition views and Copy guards expose future owned-field evolution.
/// # Errors
/// Returns the original first refusal with actual layer identity and original within-family index.
pub fn charge_layer_inventory_resources(
    inventory: &LayerInventory,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let (facings, linings, interfacings) = inventory.resource_layers();
    charge_layers(
        facings.iter().map(Facing::resource_definition),
        "layers.facings",
        sink,
    )?;
    charge_layers(
        linings.iter().map(Lining::resource_definition),
        "layers.linings",
        sink,
    )?;
    charge_layers(
        interfacings.iter().map(Interfacing::resource_definition),
        "layers.interfacings",
        sink,
    )
}
fn charge_layers<'a>(
    definitions: impl ExactSizeIterator<Item = &'a LayerDefinition>,
    field: &'static str,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    sink.charge_entries(definitions.len(), location(field, None, None))?;
    for (index, definition) in definitions.enumerate() {
        let LayerDefinition {
            id,
            served_piece,
            offset,
            material,
        } = definition;
        fixed_value(served_piece);
        let LayerOffsetRelationship { operation, sources } = offset;
        fixed_value(operation);
        fixed_slice(sources);
        sink.charge_entries(
            sources.len(),
            location("layers.offset.sources", Some(*id), Some(index)),
        )?;
        match material {
            MaterialAssignment::Assigned(target) => fixed_value(target),
            MaterialAssignment::Unresolved(reason) => sink.charge_text(
                reason,
                location("layers.material.reason", Some(*id), Some(index)),
            )?,
        }
    }
    Ok(())
}
/// Charge the actual Hem vector. Fixed depth/fold/method/edge/source fields own no dynamic payload.
/// Faced intent keeps one stable Facing id; its definition is charged only through the layer owner.
/// No Facing lookup, depth resolution, rebirth, scope grant or folding geometry is performed.
/// # Errors
/// Returns the original first refusal for hems (or an earlier latched error).
pub fn charge_hem_inventory_resources(
    inventory: &HemInventory,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let hems = inventory.resource_hems();
    sink.charge_entries(hems.len(), location("hems", None, None))?;
    for hem in hems {
        let (piece, definition) = hem.resource_parts();
        fixed_value(&piece);
        let HemDefinition {
            id,
            edge,
            depth,
            fold_type,
            method,
        } = definition;
        fixed_value(id);
        fixed_value(edge);
        fixed_value(depth);
        fixed_value(fold_type);
        fixed_value(method);
    }
    Ok(())
}
