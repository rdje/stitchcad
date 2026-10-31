//! Actual canonical Piece and typed point catalogue parents for candidate/provenance contracts.
#[path = "mirror_roles.rs"]
pub mod roles;
use sc_core::{construction::*, name::MachineToken, ontology::*};

pub fn catalog(operation: EntityId, active: &[u32]) -> PointOutputRoleCatalog {
    PointOutputRoleCatalog::new(
        operation,
        [(9, "main"), (2, "aux"), (7, "latent")]
            .into_iter()
            .map(|(tag, key)| {
                PointOutputRoleDefinition::new(
                    LocalTag::new(tag),
                    MachineToken::new(key).expect("key"),
                )
            })
            .collect(),
        active.iter().copied().map(LocalTag::new).collect(),
    )
}
pub fn with_points(run: impl FnOnce(BoundMirrorPiecePointRoles<'_>)) {
    roles::with_roles(|roles| {
        let piece_id = EntityId::from_bits(200);
        let identity = roles
            .declare_piece_identity(&piece_id)
            .expect("actual canonical Piece");
        let raw = catalog(roles.metadata().cut().source().input().operation(), &[2, 9]);
        let limits = PointOutputRoleCatalogLimits::new(3, 2, 13);
        let points = raw.admit(limits).expect("complete metadata");
        let binding = identity
            .bind_point_roles(&points, limits)
            .expect("actual mirror owner");
        run(binding);
    });
}
