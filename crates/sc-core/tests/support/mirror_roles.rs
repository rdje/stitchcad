//! Actual immutable role catalogue and mirror source/cut/field parents for identity/manifest checks.
#[path = "mirror_metadata.rs"]
pub mod metadata;
use sc_core::{construction::*, name::MachineToken, ontology::*};

pub fn with_roles(run: impl FnOnce(BoundMirrorPieceEdgeRoles<'_>)) {
    metadata::with_metadata(|metadata| {
        let raw = EdgeOutputRoleCatalog::new(
            metadata.cut().source().input().operation(),
            vec![EdgeOutputRoleDefinition::new(
                LocalTag::new(9),
                MachineToken::new("main").expect("key"),
            )],
            vec![LocalTag::new(9)],
        );
        let limits = EdgeOutputRoleCatalogLimits::new(1, 1, 4);
        let catalog = raw.admit(limits).expect("whole catalogue");
        let roles = metadata
            .bind_edge_roles(&catalog, limits)
            .expect("actual owner");
        run(roles);
    });
}
