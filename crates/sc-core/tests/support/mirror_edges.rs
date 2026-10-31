//! Actual Piece identity and distinct edge roles for candidate and complete coverage checks.
#[path = "mirror_metadata.rs"]
pub mod metadata;
use sc_core::{construction::*, name::MachineToken, ontology::*};

pub fn with_edges(run: impl FnOnce(DeclaredMirrorPieceIdentity<'_>)) {
    metadata::with_metadata(|metadata| with_edges_metadata(metadata, run));
}

pub fn with_edges_metadata(
    metadata: BoundMirrorPieceMetadata<'_>,
    run: impl FnOnce(DeclaredMirrorPieceIdentity<'_>),
) {
    let raw = EdgeOutputRoleCatalog::new(
        metadata.cut().source().input().operation(),
        [
            (9, "main"),
            (2, "aux"),
            (7, "latent"),
            (4, "hole"),
            (6, "other"),
            (8, "spare"),
            (5, "spare_b"),
        ]
        .into_iter()
        .map(|(tag, key)| {
            EdgeOutputRoleDefinition::new(LocalTag::new(tag), MachineToken::new(key).expect("key"))
        })
        .collect(),
        [2, 9, 4, 6, 8, 5].into_iter().map(LocalTag::new).collect(),
    );
    let limits = EdgeOutputRoleCatalogLimits::new(7, 6, 34);
    let catalog = raw.admit(limits).expect("complete edge catalogue");
    let roles = metadata
        .bind_edge_roles(&catalog, limits)
        .expect("actual mirror operation");
    let piece = EntityId::from_bits(200);
    let identity = roles
        .declare_piece_identity(&piece)
        .expect("actual Piece declaration");
    run(identity);
}
