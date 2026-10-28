//! Actual held Piece/copy-plan content, without rebirth, repair, registry or geometry work.
use super::{
    fixed_slice, fixed_value, DesignResourceLimitError, DesignResourceLocation,
    DesignResourceScope, DesignResourceSink,
};
use crate::ontology::{
    CutPlan, EntityId, LabelText, MaterialAssignment, PieceDefinition, PieceInventory,
};

fn location(
    field: &'static str,
    owner: Option<EntityId>,
    index: Option<usize>,
) -> DesignResourceLocation {
    DesignResourceLocation {
        scope: DesignResourceScope::Structure,
        owner,
        field,
        index,
    }
}
/// Charge actual Piece inventory/held reference vectors and exact authored text, in schema order.
/// Precharge each collection before traversal. Hole-edge locations retain actual Piece owner and
/// original hole index; other member fields retain the actual Piece owner and original Piece index.
/// Exhaustive owner/definition/label matches and Copy-reference constraints guard the present scope.
/// No Piece is reconstructed against a later ledger; historical references and label/Material intent
/// remain untouched. This supplies no registry, geometry, release, authority or whole-model proof.
/// # Errors
/// Returns the original first resource refusal with actual owner, field and authored index.
pub fn charge_piece_inventory_resources(
    inventory: &PieceInventory,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    charge_pieces_projected(inventory, None, sink)
}
pub(super) fn charge_pieces_projected(
    inventory: &PieceInventory,
    replacement: Option<(&[EntityId], &MaterialAssignment)>,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let pieces = inventory.resource_pieces();
    sink.charge_entries(pieces.len(), location("pieces", None, None))?;
    for (index, piece) in pieces.iter().enumerate() {
        let PieceDefinition {
            id,
            boundary,
            holes,
            construction_lines,
            quantity: _,
            mirroring,
            cut_on_fold: _,
            fold_edges,
            cutting_side,
            material,
            layer_index: _,
            label,
        } = piece.resource_definition();
        let owner = Some(*id);
        fixed_value(mirroring);
        fixed_value(cutting_side);
        fixed_slice(boundary);
        sink.charge_entries(
            boundary.len(),
            location("pieces.boundary", owner, Some(index)),
        )?;
        sink.charge_entries(holes.len(), location("pieces.holes", owner, Some(index)))?;
        for (hole_index, hole) in holes.iter().enumerate() {
            fixed_slice(hole);
            sink.charge_entries(
                hole.len(),
                location("pieces.holes.edges", owner, Some(hole_index)),
            )?;
        }
        fixed_slice(construction_lines);
        sink.charge_entries(
            construction_lines.len(),
            location("pieces.construction_lines", owner, Some(index)),
        )?;
        fixed_slice(fold_edges);
        sink.charge_entries(
            fold_edges.len(),
            location("pieces.fold_edges", owner, Some(index)),
        )?;
        let material = replacement
            .filter(|(selected, _)| selected.contains(id))
            .map_or(material, |(_, assignment)| assignment);
        match material {
            MaterialAssignment::Assigned(identity) => fixed_value(identity),
            MaterialAssignment::Unresolved(reason) => sink.charge_text(
                reason,
                location("pieces.material.reason", owner, Some(index)),
            )?,
        }
        let LabelText {
            name,
            size,
            fabric,
            colorway,
        } = label;
        for (field, text) in [
            ("pieces.label.name", name),
            ("pieces.label.size", size),
            ("pieces.label.fabric", fabric),
            ("pieces.label.colorway", colorway),
        ] {
            sink.charge_text(text, location(field, owner, Some(index)))?;
        }
    }
    Ok(())
}

/// Charge the actual physical-copy vector, retaining its authored order and fixed reference values.
/// This borrows only; it does not rebuild copies, validate current Piece ownership or realize transforms.
/// # Errors
/// Returns the original first resource refusal for cut_plan.copies (or a previous latched refusal).
pub fn charge_cut_plan_resources(
    plan: &CutPlan,
    sink: &mut dyn DesignResourceSink,
) -> Result<(), DesignResourceLimitError> {
    let copies = plan.resource_copies();
    fixed_slice(copies);
    sink.charge_entries(copies.len(), location("cut_plan.copies", None, None))
}
