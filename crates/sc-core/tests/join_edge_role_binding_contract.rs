//! Actual whole join/Edge metadata ownership and fresh complete quotas;no output or issuance proof.
#![allow(clippy::expect_used)]
#[path = "support/join_banks.rs"]
mod common;
use common::{id, with_banks};
use sc_core::{construction::*, name::MachineToken, ontology::*};
fn raw(operation: u128, active: Vec<LocalTag>) -> EdgeOutputRoleCatalog {
    EdgeOutputRoleCatalog::new(
        id(operation),
        [(9, "bank_a"), (2, "bank_b"), (7, "inactive")]
            .into_iter()
            .map(|(tag, key)| {
                EdgeOutputRoleDefinition::new(
                    LocalTag::new(tag),
                    MachineToken::new(key).expect("key"),
                )
            })
            .collect(),
        active,
    )
}
#[test]
fn actual_banks_catalogue_original_members_and_policy_are_borrowed_intact() {
    with_banks(|banks| {
        let raw = raw(1000, vec![LocalTag::new(2), LocalTag::new(9)]);
        let snapshot = raw.clone();
        let limits = EdgeOutputRoleCatalogLimits::new(3, 2, 20);
        let catalog = raw.admit(limits).expect("complete original metadata");
        let roles = banks
            .bind_edge_roles(&catalog, limits)
            .expect("actual join owner");
        assert!(std::ptr::eq(roles.banks(), &banks));
        assert!(std::ptr::eq(roles.catalog(), &catalog));
        assert!(std::ptr::eq(roles.catalog().catalog(), &raw));
        assert_eq!(roles.limits(), limits);
        assert_eq!(roles.catalog().key_bytes(), 20);
        assert_eq!(
            roles.catalog().catalog().active_tags(),
            &[LocalTag::new(2), LocalTag::new(9)]
        );
        for tag in [2, 9] {
            let member = roles
                .catalog()
                .active_role(LocalTag::new(tag))
                .expect("actual active member");
            assert!(std::ptr::eq(member.catalog(), &catalog));
            assert!(std::ptr::eq(
                member.definition(),
                raw.definitions()
                    .iter()
                    .find(|d| d.tag() == LocalTag::new(tag))
                    .expect("actual original definition")
            ));
        }
        assert!(roles.catalog().active_role(LocalTag::new(7)).is_none());
        assert_eq!(raw, snapshot);
        assert!(format!("{roles:?}").contains("BoundJoinEdgeRoles"));
    });
}
#[test]
fn complete_d_a_key_byte_quotas_precede_wrong_owner_and_errors_keep_all_actual_parents() {
    use JoinEdgeRoleBindingRefusal as R;
    with_banks(|banks| {
        let raw = raw(1001, vec![LocalTag::new(2), LocalTag::new(9)]);
        let snapshot = raw.clone();
        let catalog = raw
            .admit(EdgeOutputRoleCatalogLimits::new(3, 2, 20))
            .expect("valid unrelated metadata owner");
        for (limits, cause) in [
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
                EdgeOutputRoleCatalogLimits::new(3, 2, 19),
                R::KeyBytes {
                    required: 20,
                    allowed: 19,
                },
            ),
            (EdgeOutputRoleCatalogLimits::new(3, 2, 20), R::Operation),
        ] {
            let error = banks
                .bind_edge_roles(&catalog, limits)
                .expect_err("whole quota then owner");
            assert!(std::ptr::eq(error.banks(), &banks));
            assert!(std::ptr::eq(error.catalog(), &catalog));
            assert_eq!(error.limits(), limits);
            assert_eq!(error.refusal(), cause);
            assert_eq!(raw, snapshot);
            assert!(error.to_string().starts_with("join edge-role association:"));
            assert!(format!("{error:?}").contains("JoinEdgeRoleBindingError"));
        }
    });
}
#[test]
fn empty_and_fully_inactive_catalogues_bind_metadata_without_inventing_an_empty_join() {
    with_banks(|banks| {
        for raw in [
            EdgeOutputRoleCatalog::new(id(1000), vec![], vec![]),
            raw(1000, vec![]),
        ] {
            let limits = EdgeOutputRoleCatalogLimits::new(
                raw.definitions().len(),
                0,
                if raw.definitions().is_empty() { 0 } else { 20 },
            );
            let catalog = raw.admit(limits).expect("complete empty/inactive metadata");
            let roles = banks
                .bind_edge_roles(&catalog, limits)
                .expect("owner association only");
            assert!(std::ptr::eq(roles.banks(), &banks));
            assert_eq!(roles.banks().banks().len(), 2);
            assert_eq!(*roles.banks().sources().population().quantity(), 1);
            assert!(roles.catalog().catalog().active_tags().is_empty());
            assert_eq!(
                roles.catalog().catalog().definitions().len(),
                raw.definitions().len()
            );
        }
    });
}
