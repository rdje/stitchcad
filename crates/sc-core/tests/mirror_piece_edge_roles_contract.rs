//! Shared edge-role metadata and authentic mirror source/cut/field ownership.
#![allow(clippy::expect_used)]
#[path = "support/mirror_metadata.rs"]
mod common;
use common::with_metadata;
use sc_core::{construction::*, name::MachineToken, ontology::*};

fn raw(operation: u128, active: &[u32]) -> EdgeOutputRoleCatalog {
    EdgeOutputRoleCatalog::new(
        EntityId::from_bits(operation),
        [(9, "main"), (2, "aux"), (7, "spare")]
            .into_iter()
            .map(|(tag, key)| {
                EdgeOutputRoleDefinition::new(
                    LocalTag::new(tag),
                    MachineToken::new(key).expect("key"),
                )
            })
            .collect(),
        active.iter().copied().map(LocalTag::new).collect(),
    )
}
fn catalog(raw: &EdgeOutputRoleCatalog) -> AdmittedEdgeOutputRoleCatalog<'_> {
    raw.admit(EdgeOutputRoleCatalogLimits::new(3, 2, 12))
        .expect("whole shared metadata")
}

#[test]
fn actual_metadata_and_shared_catalog_keep_original_owners_roles_and_nonordinal_tags() {
    with_metadata(|metadata| {
        let raw = raw(10, &[2, 9]);
        let catalog = catalog(&raw);
        let limits = EdgeOutputRoleCatalogLimits::new(3, 2, 12);
        let bound = metadata
            .bind_edge_roles(&catalog, limits)
            .expect("actual mirror owner");
        assert!(std::ptr::eq(bound.metadata(), &metadata));
        assert!(std::ptr::eq(bound.catalog(), &catalog));
        assert_eq!(bound.limits(), limits);
        let legacy: &AdmittedOffsetRoleCatalog = bound.catalog();
        assert!(std::ptr::eq(legacy, &catalog));
        assert_eq!(
            bound.metadata().cut().source().input().operation(),
            EntityId::from_bits(10)
        );
        assert_eq!(bound.catalog().key_bytes(), 12);
        for (role, active_index, definition_index) in bound
            .catalog()
            .active_roles()
            .zip([0, 1])
            .map(|(role, index)| (role, index, 1 - index))
        {
            assert!(std::ptr::eq(role.catalog(), &catalog));
            assert_eq!(role.index(), active_index);
            assert!(std::ptr::eq(
                role.tag(),
                raw.active_tags()
                    .get(active_index)
                    .expect("actual tag field")
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
                .map(|role| role.definition().key().as_str())
                .collect::<Vec<_>>(),
            vec!["main", "aux"]
        );
    });
}

#[test]
fn complete_definitions_active_and_key_bytes_precede_operation_and_keep_original_context() {
    use MirrorPieceRoleBindingRefusal as R;
    with_metadata(|metadata| {
        let raw = raw(99, &[2, 9]);
        let original = raw.clone();
        let catalog = catalog(&raw);
        for (limits, refusal) in [
            (
                EdgeOutputRoleCatalogLimits::new(2, 0, 0),
                R::Definitions {
                    required: 3,
                    allowed: 2,
                },
            ),
            (
                EdgeOutputRoleCatalogLimits::new(3, 1, 0),
                R::Active {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                EdgeOutputRoleCatalogLimits::new(3, 2, 11),
                R::KeyBytes {
                    required: 12,
                    allowed: 11,
                },
            ),
            (EdgeOutputRoleCatalogLimits::new(3, 2, 12), R::Operation),
        ] {
            let error = metadata
                .bind_edge_roles(&catalog, limits)
                .expect_err("whole refusal first");
            assert!(std::ptr::eq(error.metadata(), &metadata));
            assert!(std::ptr::eq(error.catalog(), &catalog));
            assert_eq!(error.limits(), limits);
            assert_eq!(error.refusal(), refusal);
            assert!(format!("{error}").contains("mirror copy edge-role association"));
            assert!(format!("{error:?}").contains("MirrorPieceRoleBindingError"));
            assert_eq!(raw, original);
        }
    });
}

#[test]
fn empty_and_inactive_catalogues_remain_metadata_without_minting_or_retiring_outputs() {
    with_metadata(|metadata| {
        for raw in [
            EdgeOutputRoleCatalog::new(EntityId::from_bits(10), vec![], vec![]),
            raw(10, &[]),
        ] {
            let original = raw.clone();
            let catalog = catalog(&raw);
            let bound = metadata
                .bind_edge_roles(&catalog, EdgeOutputRoleCatalogLimits::new(3, 0, 12))
                .expect("metadata state only");
            assert_eq!(bound.catalog().active_roles().len(), 0);
            assert_eq!(raw, original);
            assert!(std::ptr::eq(bound.metadata(), &metadata));
            assert!(std::ptr::eq(bound.catalog().catalog(), &raw));
        }
    });
}
