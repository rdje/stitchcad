//! Actual whole truing/edge-role association;metadata proves no output,history,fit or issuance.
#![allow(clippy::expect_used)]
#[path = "support/truing_copy_context.rs"]
mod common;
use common::{id, with_copies};
use sc_core::{construction::*, name::MachineToken, ontology::*};
fn catalog(operation: u128, active: Vec<LocalTag>) -> EdgeOutputRoleCatalog {
    EdgeOutputRoleCatalog::new(
        id(operation),
        [(9, "refit"), (2, "retained"), (7, "inactive")]
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
fn actual_complete_copy_context_catalogue_original_fields_members_and_limits_are_retained() {
    for shared in [false, true] {
        with_copies(shared, |copies| {
            let raw = catalog(1000, vec![LocalTag::new(2), LocalTag::new(9)]);
            let snapshot = raw.clone();
            let limits = EdgeOutputRoleCatalogLimits::new(3, 2, 21);
            let catalog = raw.admit(limits).expect("complete edge metadata");
            let roles = copies
                .bind_edge_roles(&catalog, limits)
                .expect("same actual creating operation");
            assert!(std::ptr::eq(roles.copies(), &copies));
            assert!(std::ptr::eq(roles.catalog(), &catalog));
            assert!(std::ptr::eq(roles.catalog().catalog(), &raw));
            assert_eq!(roles.limits(), limits);
            assert_eq!(
                roles.catalog().catalog().active_tags(),
                &[LocalTag::new(2), LocalTag::new(9)]
            );
            assert_eq!(roles.catalog().key_bytes(), 21);
            for tag in [2, 9] {
                let role = roles
                    .catalog()
                    .active_role(LocalTag::new(tag))
                    .expect("actual active member");
                assert!(std::ptr::eq(role.catalog(), &catalog));
                assert!(std::ptr::eq(
                    role.definition(),
                    raw.definitions()
                        .iter()
                        .find(|d| d.tag() == LocalTag::new(tag))
                        .expect("actual definition")
                ));
            }
            assert!(roles.catalog().active_role(LocalTag::new(7)).is_none());
            assert_eq!(roles.copies().same_pattern_piece(), shared);
            assert!(std::ptr::eq(
                roles.copies().source().fit_provider().tolerance,
                copies.source().fit_provider().tolerance
            ));
            assert_eq!(raw, snapshot);
            assert!(format!("{roles:?}").contains("BoundTruingEdgeRoles"));
        });
    }
}
#[test]
fn whole_d_a_key_byte_quotas_precede_wrong_operation_without_dropping_original_error_context() {
    use TruingEdgeRoleBindingRefusal as R;
    with_copies(false, |copies| {
        let raw = catalog(1001, vec![LocalTag::new(2), LocalTag::new(9)]);
        let snapshot = raw.clone();
        let catalog = raw
            .admit(EdgeOutputRoleCatalogLimits::new(3, 2, 21))
            .expect("valid unrelated owner metadata");
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
                EdgeOutputRoleCatalogLimits::new(3, 2, 20),
                R::KeyBytes {
                    required: 21,
                    allowed: 20,
                },
            ),
            (EdgeOutputRoleCatalogLimits::new(3, 2, 21), R::Operation),
        ] {
            let error = copies
                .bind_edge_roles(&catalog, limits)
                .expect_err("whole quota then actual operation");
            assert!(std::ptr::eq(error.copies(), &copies));
            assert!(std::ptr::eq(error.catalog(), &catalog));
            assert_eq!(error.limits(), limits);
            assert_eq!(error.refusal(), cause);
            assert!(error
                .to_string()
                .starts_with("truing edge-role association:"));
            assert!(format!("{error:?}").contains("TruingEdgeRoleBindingError"));
            assert_eq!(raw, snapshot);
        }
    });
}
#[test]
fn empty_and_fully_inactive_metadata_keep_complete_shared_source_and_prove_no_empty_refit_result() {
    with_copies(true, |copies| {
        for raw in [
            EdgeOutputRoleCatalog::new(id(1000), vec![], vec![]),
            catalog(1000, vec![]),
        ] {
            let limits = EdgeOutputRoleCatalogLimits::new(
                raw.definitions().len(),
                0,
                if raw.definitions().is_empty() { 0 } else { 21 },
            );
            let catalog = raw.admit(limits).expect("empty/inactive metadata");
            let roles = copies
                .bind_edge_roles(&catalog, limits)
                .expect("actual association only");
            assert!(roles.catalog().catalog().active_tags().is_empty());
            assert_eq!(
                roles.catalog().catalog().definitions().len(),
                raw.definitions().len()
            );
            assert_eq!(roles.copies().plan().copies().len(), 2);
            assert!(roles.copies().same_pattern_piece());
            assert_eq!(roles.copies().source().span().definition().stops.len(), 1);
            assert_eq!(roles.copies().source().affected_copies().count(), 2);
            assert!(std::ptr::eq(roles.copies(), &copies));
        }
    });
}
