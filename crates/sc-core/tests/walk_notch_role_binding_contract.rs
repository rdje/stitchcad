//! Actual complete walk and distinct notch catalogue parents;metadata binding issues no semantic mark.
#![allow(clippy::expect_used)]
#[path = "support/walk_copy_context.rs"]
mod common;
use common::{input::id, with_copies};
use sc_core::{construction::*, name::MachineToken, ontology::*};
fn definitions() -> Vec<NotchOutputRoleDefinition> {
    [(9, "balance"), (2, "end"), (7, "latent")]
        .into_iter()
        .map(|(tag, key)| {
            NotchOutputRoleDefinition::new(
                LocalTag::new(tag),
                MachineToken::new(key).expect("machine key"),
            )
        })
        .collect()
}
fn raw(operation: u128, active: &[u32]) -> NotchOutputRoleCatalog {
    NotchOutputRoleCatalog::new(
        id(operation),
        definitions(),
        active.iter().copied().map(LocalTag::new).collect(),
    )
}
#[test]
fn successful_binding_retains_actual_whole_context_catalogue_policy_and_authentic_role_views() {
    with_copies(|copies| {
        let raw = raw(10, &[2, 9]);
        let snapshot = raw.clone();
        let catalog = raw
            .admit(NotchOutputRoleCatalogLimits::new(3, 3, 16))
            .expect("typed notch metadata");
        let policy = NotchOutputRoleCatalogLimits::new(3, 2, 16);
        let old_plan = copies.plan().clone();
        let old_inventory = copies.inventory().clone();
        let old_input = copies.source().admitted().edit().clone();
        let bound = copies
            .bind_notch_roles(&catalog, policy)
            .expect("actual walk operation");
        assert!(std::ptr::eq(bound.copies(), &copies));
        assert!(std::ptr::eq(bound.catalog(), &catalog));
        assert_eq!(bound.limits(), policy);
        assert!(std::ptr::eq(bound.catalog().catalog(), &raw));
        assert_eq!(
            bound.catalog().catalog().operation(),
            bound.copies().source().admitted().edit().operation()
        );
        assert_eq!(
            bound
                .catalog()
                .active_roles()
                .map(|role| role.definition().key().as_str())
                .collect::<Vec<_>>(),
            vec!["end", "balance"]
        );
        for role in bound.catalog().active_roles() {
            assert!(std::ptr::eq(role.catalog(), &catalog));
            assert!(std::ptr::eq(
                role.tag(),
                raw.active_tags()
                    .get(role.index())
                    .expect("actual original field")
            ));
        }
        assert_eq!(bound.copies().copy(SeamSideId::A).id(), id(200));
        assert_eq!(bound.copies().piece(SeamSideId::B).id(), id(101));
        assert_eq!(bound.copies().source().admitted().notch_request_count(), 2);
        assert!(!bound.copies().same_pattern_piece());
        assert_eq!(raw, snapshot);
        assert_eq!(*copies.plan(), old_plan);
        assert_eq!(*copies.inventory(), old_inventory);
        assert_eq!(*copies.source().admitted().edit(), old_input);
    });
}
#[test]
fn whole_metadata_quotas_precede_wrong_operation_and_refusal_retains_original_typed_parents() {
    use WalkNotchRoleBindingRefusal as R;
    with_copies(|copies| {
        let raw = raw(99, &[2, 9]);
        let snapshot = raw.clone();
        let catalog = raw
            .admit(NotchOutputRoleCatalogLimits::new(3, 3, 16))
            .expect("admitted other namespace");
        for (policy, expected) in [
            (
                NotchOutputRoleCatalogLimits::new(2, 0, 0),
                R::Definitions {
                    required: 3,
                    allowed: 2,
                },
            ),
            (
                NotchOutputRoleCatalogLimits::new(3, 1, 0),
                R::Active {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                NotchOutputRoleCatalogLimits::new(3, 2, 15),
                R::KeyBytes {
                    required: 16,
                    allowed: 15,
                },
            ),
            (NotchOutputRoleCatalogLimits::new(3, 2, 16), R::Operation),
        ] {
            let error = copies
                .bind_notch_roles(&catalog, policy)
                .expect_err("complete quota then operation");
            assert!(std::ptr::eq(error.copies(), &copies));
            assert!(std::ptr::eq(error.catalog(), &catalog));
            assert_eq!(error.limits(), policy);
            assert_eq!(error.refusal(), expected);
            assert!(error
                .to_string()
                .starts_with("walk notch-role association:"));
            assert!(format!("{error:?}").contains("WalkNotchRoleBindingError"));
            assert_eq!(raw, snapshot);
        }
    });
}
#[test]
fn matching_empty_or_inactive_metadata_can_bind_without_discarding_original_requested_occurrences()
{
    with_copies(|copies| {
        for (raw, policy) in [
            (
                NotchOutputRoleCatalog::new(id(10), vec![], vec![]),
                NotchOutputRoleCatalogLimits::new(0, 0, 0),
            ),
            (raw(10, &[]), NotchOutputRoleCatalogLimits::new(3, 0, 16)),
        ] {
            let catalog = raw.admit(policy).expect("metadata state");
            let bound = copies
                .bind_notch_roles(&catalog, policy)
                .expect("matching actual operation");
            assert_eq!(bound.catalog().active_roles().len(), 0);
            assert_eq!(bound.copies().source().admitted().notch_request_count(), 2);
            assert_eq!(bound.copies().source().pairs().len(), 1);
            assert_eq!(
                bound.catalog().catalog().definitions().len(),
                policy.definitions()
            );
            assert_eq!(bound.catalog().key_bytes(), policy.key_bytes());
        }
        let zero = NotchOutputRoleCatalog::new(id(0), vec![], vec![]);
        let catalog = zero
            .admit(NotchOutputRoleCatalogLimits::new(0, 0, 0))
            .expect("ordinary zero operation declaration");
        assert_eq!(
            copies
                .bind_notch_roles(&catalog, NotchOutputRoleCatalogLimits::new(0, 0, 0))
                .expect_err("zero cannot stand in for actual operation10")
                .refusal(),
            WalkNotchRoleBindingRefusal::Operation
        );
    });
}
