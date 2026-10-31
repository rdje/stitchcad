//! Actual truing copy/Piece context and complete typed edge-role metadata.
#[path = "truing_copy_context.rs"]
pub mod copies;
use copies::id;
use sc_core::{construction::*, name::MachineToken, ontology::LocalTag};
pub fn with_roles(materials: bool, run: impl FnOnce(BoundTruingEdgeRoles<'_>)) {
    let bind = |copies: BoundTruingCopyInventory<'_>| {
        let raw = EdgeOutputRoleCatalog::new(
            id(1000),
            [(9, "refit"), (2, "retained"), (7, "inactive")]
                .into_iter()
                .map(|(tag, key)| {
                    EdgeOutputRoleDefinition::new(
                        LocalTag::new(tag),
                        MachineToken::new(key).expect("key"),
                    )
                })
                .collect(),
            vec![LocalTag::new(2), LocalTag::new(9)],
        );
        let limits = EdgeOutputRoleCatalogLimits::new(3, 2, 21);
        let catalog = raw.admit(limits).expect("whole edge catalogue");
        let roles = copies
            .bind_edge_roles(&catalog, limits)
            .expect("actual truing operation");
        run(roles);
    };
    if materials {
        copies::with_materials(false, Some([id(777), id(888)]), bind)
    } else {
        copies::with_copies(false, bind)
    }
}
