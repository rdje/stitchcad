//! Point-typed structural metadata preservation over the shared history algorithm.
#![allow(clippy::expect_used)]
use sc_core::{construction::*, name::MachineToken, ontology::*};
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn role(tag: u32, key: &str) -> PointOutputRoleDefinition {
    PointOutputRoleDefinition::new(
        LocalTag::new(tag),
        MachineToken::new(key).expect("local key"),
    )
}
fn raw(
    operation: u128,
    roles: Vec<PointOutputRoleDefinition>,
    active: &[u32],
) -> PointOutputRoleCatalog {
    PointOutputRoleCatalog::new(
        id(operation),
        roles,
        active.iter().copied().map(LocalTag::new).collect(),
    )
}
fn admit(catalog: &PointOutputRoleCatalog) -> AdmittedPointOutputRoleCatalog<'_> {
    catalog
        .admit(PointOutputRoleCatalogLimits::new(10, 10, 100))
        .expect("supplied metadata")
}
#[test]
fn complete_transition_preserves_actual_sources_and_reports_original_added_and_membership_fields() {
    let before_raw = raw(10, vec![role(9, "main"), role(2, "end")], &[9]);
    let after_raw = raw(
        10,
        vec![role(7, "new"), role(2, "end"), role(9, "main")],
        &[2, 7],
    );
    let before = admit(&before_raw);
    let after = admit(&after_raw);
    let old = before_raw.clone();
    let new = after_raw.clone();
    let limits = PointOutputRoleTransitionLimits::new(5, 3, 17);
    let transition = before
        .admit_transition(&after, limits)
        .expect("all issued definitions remain");
    assert!(std::ptr::eq(transition.before(), &before));
    assert!(std::ptr::eq(transition.after(), &after));
    assert_eq!(transition.limits(), limits);
    assert_eq!(
        (limits.definitions(), limits.active(), limits.key_bytes()),
        (5, 3, 17)
    );
    assert_eq!(
        (
            transition.definitions(),
            transition.active(),
            transition.key_bytes()
        ),
        (5, 3, 17)
    );
    let introduced = transition.introduced().collect::<Vec<_>>();
    assert_eq!(introduced.len(), 1);
    let (index, definition) = *introduced.first().expect("one actual new definition");
    assert_eq!(index, 0);
    assert!(std::ptr::eq(
        definition,
        after_raw.definitions().first().expect("actual new field")
    ));
    assert_eq!(
        transition
            .activated()
            .map(|view| view.definition().key().as_str())
            .collect::<Vec<_>>(),
        vec!["end", "new"]
    );
    for view in transition.activated() {
        assert!(std::ptr::eq(view.catalog(), &after));
    }
    assert_eq!(
        transition
            .deactivated()
            .map(|view| view.definition().tag().get())
            .collect::<Vec<_>>(),
        vec![9]
    );
    for view in transition.deactivated() {
        assert!(std::ptr::eq(view.catalog(), &before));
    }
    assert_eq!(
        transition
            .activated()
            .rev()
            .map(|view| view.index())
            .collect::<Vec<_>>(),
        vec![1, 0]
    );
    assert_eq!(before_raw, old);
    assert_eq!(after_raw, new);
}
#[test]
fn combined_quotas_then_owner_and_first_old_definition_refusals_preserve_both_actual_owners() {
    use PointOutputRoleTransitionRefusal as R;
    let before_raw = raw(10, vec![role(9, "main"), role(2, "end")], &[9]);
    let before = admit(&before_raw);
    let wrong_owner = || {
        raw(
            99,
            vec![role(7, "new"), role(2, "end"), role(9, "main")],
            &[2, 7],
        )
    };
    let rows = [
        (
            wrong_owner(),
            PointOutputRoleTransitionLimits::new(4, 0, 0),
            R::Definitions {
                required: 5,
                allowed: 4,
            },
        ),
        (
            wrong_owner(),
            PointOutputRoleTransitionLimits::new(5, 2, 0),
            R::Active {
                required: 3,
                allowed: 2,
            },
        ),
        (
            wrong_owner(),
            PointOutputRoleTransitionLimits::new(5, 3, 16),
            R::KeyBytes {
                required: 17,
                allowed: 16,
            },
        ),
        (
            wrong_owner(),
            PointOutputRoleTransitionLimits::new(5, 3, 17),
            R::Owner,
        ),
        (
            raw(10, vec![role(7, "new"), role(9, "main")], &[7]),
            PointOutputRoleTransitionLimits::new(10, 10, 100),
            R::Forgotten {
                index: 1,
                tag: LocalTag::new(2),
            },
        ),
        (
            raw(10, vec![role(9, "renamed"), role(2, "end")], &[9]),
            PointOutputRoleTransitionLimits::new(10, 10, 100),
            R::ReassignedKey {
                index: 0,
                tag: LocalTag::new(9),
            },
        ),
    ];
    for (after_raw, limits, cause) in rows {
        let after = admit(&after_raw);
        let snapshot = after_raw.clone();
        let error = before
            .admit_transition(&after, limits)
            .expect_err("no accepted prefix");
        assert!(std::ptr::eq(error.before(), &before));
        assert!(std::ptr::eq(error.after(), &after));
        assert_eq!(error.limits(), limits);
        assert_eq!(error.refusal(), cause);
        assert_eq!(after_raw, snapshot);
    }
}
#[test]
fn restoring_membership_must_retain_later_issued_definitions_even_when_they_become_inactive() {
    let current_raw = raw(
        10,
        vec![role(9, "main"), role(2, "end"), role(7, "new")],
        &[2, 7],
    );
    let current = admit(&current_raw);
    let historical_raw = raw(10, vec![role(9, "main"), role(2, "end")], &[9]);
    let historical = admit(&historical_raw);
    let limits = PointOutputRoleTransitionLimits::new(10, 10, 100);
    let error = current
        .admit_transition(&historical, limits)
        .expect_err("old snapshot forgets later-issued tag");
    assert_eq!(
        error.refusal(),
        PointOutputRoleTransitionRefusal::Forgotten {
            index: 2,
            tag: LocalTag::new(7)
        }
    );
    let proposal_raw = raw(10, current_raw.definitions().to_vec(), &[9]);
    let proposal = admit(&proposal_raw);
    let transition = current
        .admit_transition(&proposal, limits)
        .expect("same definitions,restored membership intent");
    assert_eq!(transition.introduced().count(), 0);
    assert_eq!(
        transition
            .activated()
            .map(|view| view.tag().get())
            .collect::<Vec<_>>(),
        vec![9]
    );
    assert_eq!(
        transition
            .deactivated()
            .map(|view| view.tag().get())
            .collect::<Vec<_>>(),
        vec![2, 7]
    );
    assert!(proposal.definition(LocalTag::new(7)).is_some());
    assert!(!proposal.is_active(LocalTag::new(7)));
}
