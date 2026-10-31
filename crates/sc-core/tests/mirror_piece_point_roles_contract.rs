//! Actual declared mirror Piece and typed point-role catalogue association.
#![allow(clippy::expect_used)]
#[path = "support/mirror_roles.rs"]
mod common;
use common::with_roles;
use sc_core::{construction::*, name::MachineToken, ontology::*};

fn raw(operation: u128, active: &[u32]) -> PointOutputRoleCatalog {
    PointOutputRoleCatalog::new(
        EntityId::from_bits(operation),
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
fn admit(raw: &PointOutputRoleCatalog) -> AdmittedPointOutputRoleCatalog<'_> {
    raw.admit(PointOutputRoleCatalogLimits::new(3, 2, 13))
        .expect("complete point metadata")
}

#[test]
fn canonical_piece_and_point_catalogue_keep_actual_parents_and_original_fields() {
    with_roles(|roles| {
        let piece_id = EntityId::from_bits(200);
        let identity = roles
            .declare_piece_identity(&piece_id)
            .expect("canonical Piece declaration");
        let raw = raw(10, &[2, 9]);
        let original = raw.clone();
        let catalog = admit(&raw);
        let limits = PointOutputRoleCatalogLimits::new(3, 2, 13);
        let bound = identity
            .bind_point_roles(&catalog, limits)
            .expect("actual mirror operation");
        assert!(std::ptr::eq(bound.identity(), &identity));
        assert!(std::ptr::eq(bound.identity().piece_id(), &piece_id));
        assert!(std::ptr::eq(bound.catalog(), &catalog));
        assert_eq!(bound.limits(), limits);
        assert_eq!(
            bound.catalog().catalog().operation(),
            bound
                .identity()
                .roles()
                .metadata()
                .cut()
                .source()
                .input()
                .operation()
        );
        for (role, index, definition_index) in bound
            .catalog()
            .active_roles()
            .zip([0, 1])
            .map(|(role, index)| (role, index, 1 - index))
        {
            assert!(std::ptr::eq(role.catalog(), &catalog));
            assert!(std::ptr::eq(
                role.tag(),
                raw.active_tags().get(index).expect("actual active field")
            ));
            assert!(std::ptr::eq(
                role.definition(),
                raw.definitions()
                    .get(definition_index)
                    .expect("actual definition")
            ));
        }
        assert_eq!(
            bound
                .catalog()
                .active_roles()
                .rev()
                .map(|r| r.definition().key().as_str())
                .collect::<Vec<_>>(),
            vec!["main", "aux"]
        );
        assert_eq!(raw, original);
    });
}

#[test]
fn whole_definitions_active_and_key_bytes_precede_operation_and_retain_both_actual_parents() {
    use MirrorPiecePointRoleBindingRefusal as R;
    with_roles(|roles| {
        let piece_id = EntityId::from_bits(200);
        let identity = roles
            .declare_piece_identity(&piece_id)
            .expect("actual Piece declaration");
        let raw = raw(99, &[2, 9]);
        let original = raw.clone();
        let catalog = admit(&raw);
        for (limits, refusal) in [
            (
                PointOutputRoleCatalogLimits::new(2, 0, 0),
                R::Definitions {
                    required: 3,
                    allowed: 2,
                },
            ),
            (
                PointOutputRoleCatalogLimits::new(3, 1, 0),
                R::Active {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                PointOutputRoleCatalogLimits::new(3, 2, 12),
                R::KeyBytes {
                    required: 13,
                    allowed: 12,
                },
            ),
            (PointOutputRoleCatalogLimits::new(3, 2, 13), R::Operation),
        ] {
            let error = identity
                .bind_point_roles(&catalog, limits)
                .expect_err("whole refusal first");
            assert!(std::ptr::eq(error.identity(), &identity));
            assert!(std::ptr::eq(error.catalog(), &catalog));
            assert_eq!(error.limits(), limits);
            assert_eq!(error.refusal(), refusal);
            assert!(format!("{error}").contains("mirror copy point-role association"));
            assert!(format!("{error:?}").contains("MirrorPiecePointRoleBindingError"));
            assert_eq!(raw, original);
        }
    });
}

#[test]
fn empty_and_inactive_point_roles_remain_metadata_without_generated_refs_or_retirement() {
    with_roles(|roles| {
        let piece_id = EntityId::from_bits(200);
        let identity = roles
            .declare_piece_identity(&piece_id)
            .expect("actual Piece declaration");
        for raw in [
            PointOutputRoleCatalog::new(EntityId::from_bits(10), vec![], vec![]),
            raw(10, &[]),
        ] {
            let original = raw.clone();
            let catalog = admit(&raw);
            let bound = identity
                .bind_point_roles(&catalog, PointOutputRoleCatalogLimits::new(3, 0, 13))
                .expect("metadata association only");
            assert_eq!(bound.catalog().active_roles().len(), 0);
            assert_eq!(raw, original);
            assert!(std::ptr::eq(bound.identity(), &identity));
            assert!(std::ptr::eq(bound.catalog().catalog(), &raw));
        }
    });
}
