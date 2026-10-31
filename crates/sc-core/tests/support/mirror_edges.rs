//! Actual Piece identity and distinct edge roles for candidate and complete coverage checks.
#[path = "mirror_metadata.rs"]
pub mod metadata;
use sc_core::{construction::*, name::MachineToken, ontology::*};

pub fn with_edges(run: impl FnOnce(DeclaredMirrorPieceIdentity<'_>)) {
    metadata::with_metadata(|metadata| {
        let raw = EdgeOutputRoleCatalog::new(
            metadata.cut().source().input().operation(),
            [
                (9, "main"),
                (2, "aux"),
                (7, "latent"),
                (4, "hole"),
                (6, "other"),
            ]
            .into_iter()
            .map(|(tag, key)| {
                EdgeOutputRoleDefinition::new(
                    LocalTag::new(tag),
                    MachineToken::new(key).expect("key"),
                )
            })
            .collect(),
            [2, 9, 4, 6].into_iter().map(LocalTag::new).collect(),
        );
        let limits = EdgeOutputRoleCatalogLimits::new(5, 4, 22);
        let catalog = raw.admit(limits).expect("complete edge catalogue");
        let roles = metadata
            .bind_edge_roles(&catalog, limits)
            .expect("actual mirror operation");
        let piece = EntityId::from_bits(200);
        let identity = roles
            .declare_piece_identity(&piece)
            .expect("actual Piece declaration");
        run(identity);
    });
}
