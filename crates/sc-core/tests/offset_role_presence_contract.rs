//! Actual metadata association,without an evaluated geometry instance or role-retirement effect.
#![allow(clippy::expect_used)]
use sc_core::{construction::*, name::MachineToken, ontology::*};
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn parameter(n: i64, d: i64) -> Param {
    Param::new(Rational::new(n, d).expect("exact rational")).expect("exact unit parameter")
}
#[path = "support/offset_sources.rs"]
mod sources;
fn role(tag: u32, key: &str) -> OffsetRoleDefinition {
    OffsetRoleDefinition::new(
        LocalTag::new(tag),
        MachineToken::new(key).expect("local key"),
    )
}
fn raw(operation: u128, active: &[u32]) -> OffsetRoleCatalog {
    OffsetRoleCatalog::new(
        id(operation),
        vec![
            role(7, "latent"),
            role(2, "end"),
            role(9, "main"),
            role(5, "optional"),
        ],
        active.iter().copied().map(LocalTag::new).collect(),
    )
}
fn catalog(raw: &OffsetRoleCatalog) -> AdmittedOffsetRoleCatalog<'_> {
    raw.admit(OffsetRoleCatalogLimits::new(4, 3, 21))
        .expect("complete catalogue")
}
fn manifest(input: &OffsetConstruction, second: Option<u32>) -> OffsetCandidateManifest {
    let mut fragments = vec![OffsetCandidateFragment::new(
        LocalTag::new(9),
        OffsetInterval::new(parameter(1, 4), parameter(1, 2)).expect("first domain"),
    )];
    if let Some(tag) = second {
        fragments.push(OffsetCandidateFragment::new(
            LocalTag::new(tag),
            OffsetInterval::new(parameter(1, 2), parameter(3, 4)).expect("second domain"),
        ));
    }
    OffsetCandidateManifest::new(input.operation(), *input.source(), fragments)
}
#[test]
fn authentic_pairs_keep_candidate_order_and_actual_membership_fields() {
    let fixture = sources::fixture();
    let allowance = fixture.allowance();
    let input = fixture.input();
    let bound = input.bind_allowance(&allowance).expect("actual input");
    let raw_manifest = manifest(&input, Some(2));
    let admitted = raw_manifest
        .admit(&bound, OffsetCandidateLimits::new(2))
        .expect("candidate admission");
    let raw_catalog = raw(10, &[2, 5, 9]);
    let catalog = catalog(&raw_catalog);
    let limits = OffsetRolePresenceLimits::new(2, 4, 3);
    let presence = admitted
        .bind_active_roles(&catalog, limits)
        .expect("all candidates active");
    assert!(std::ptr::eq(presence.admitted(), &admitted));
    assert!(std::ptr::eq(presence.catalog(), &catalog));
    assert_eq!(presence.limits(), limits);
    assert_eq!(
        (limits.candidates(), limits.definitions(), limits.active()),
        (2, 4, 3)
    );
    assert_eq!(presence.workspace_entries(), 2);
    assert_eq!(presence.outputs().len(), 2);
    let mut actual = presence.outputs();
    let first = actual.next().expect("first candidate");
    let second = actual.next().expect("second candidate");
    for (view, index, active_index) in [(first, 0, 2), (second, 1, 0)] {
        assert!(std::ptr::eq(view.presence(), &presence));
        assert_eq!(view.index(), index);
        assert!(std::ptr::eq(
            view.fragment(),
            raw_manifest.fragments().get(index).expect("raw field")
        ));
        assert!(std::ptr::eq(view.role().catalog(), &catalog));
        assert_eq!(view.role().index(), active_index);
        assert!(std::ptr::eq(
            view.role().tag(),
            raw_catalog
                .active_tags()
                .get(active_index)
                .expect("active field")
        ));
        assert!(std::ptr::eq(
            view.role().definition(),
            catalog
                .definition(view.fragment().tag())
                .expect("definition")
        ));
        assert_eq!(
            view.logical_output(),
            EdgeRef::new(id(10), view.fragment().tag())
        );
    }
    assert_eq!(
        presence
            .outputs()
            .rev()
            .map(|v| v.logical_output().tag().get())
            .collect::<Vec<_>>(),
        vec![2, 9]
    );
    assert!(catalog.active_role(LocalTag::new(7)).is_none());
    assert!(catalog.active_role(LocalTag::new(99)).is_none());
}
#[test]
fn resource_then_owner_then_original_candidate_causes_keep_complete_actual_inputs() {
    use OffsetRolePresenceRefusal as R;
    let fixture = sources::fixture();
    let allowance = fixture.allowance();
    let input = fixture.input();
    let bound = input.bind_allowance(&allowance).expect("actual input");
    let rows = [
        (
            99,
            Some(99),
            vec![2, 5, 9],
            OffsetRolePresenceLimits::new(1, 0, 0),
            R::Candidates {
                required: 2,
                allowed: 1,
            },
        ),
        (
            99,
            Some(99),
            vec![2, 5, 9],
            OffsetRolePresenceLimits::new(2, 3, 0),
            R::Definitions {
                required: 4,
                allowed: 3,
            },
        ),
        (
            99,
            Some(99),
            vec![2, 5, 9],
            OffsetRolePresenceLimits::new(2, 4, 2),
            R::Active {
                required: 3,
                allowed: 2,
            },
        ),
        (
            99,
            Some(99),
            vec![2, 5, 9],
            OffsetRolePresenceLimits::new(2, 4, 3),
            R::Operation,
        ),
        (
            10,
            Some(99),
            vec![2, 5, 9],
            OffsetRolePresenceLimits::new(2, 4, 3),
            R::Undefined {
                candidate: 1,
                tag: LocalTag::new(99),
            },
        ),
        (
            10,
            Some(2),
            vec![5, 9],
            OffsetRolePresenceLimits::new(2, 4, 3),
            R::Inactive {
                candidate: 1,
                tag: LocalTag::new(2),
            },
        ),
        (
            10,
            Some(99),
            vec![2, 5],
            OffsetRolePresenceLimits::new(2, 4, 3),
            R::Inactive {
                candidate: 0,
                tag: LocalTag::new(9),
            },
        ),
    ];
    for (operation, second, active, limits, cause) in rows {
        let raw_manifest = manifest(&input, second);
        let raw_catalog = raw(operation, &active);
        let old_manifest = raw_manifest.clone();
        let old_catalog = raw_catalog.clone();
        let admitted = raw_manifest
            .admit(&bound, OffsetCandidateLimits::new(2))
            .expect("candidate metadata");
        let catalog = catalog(&raw_catalog);
        let error = admitted
            .bind_active_roles(&catalog, limits)
            .expect_err("whole association refused");
        assert!(std::ptr::eq(error.admitted(), &admitted));
        assert!(std::ptr::eq(error.catalog(), &catalog));
        assert_eq!(error.limits(), limits);
        assert_eq!(error.refusal(), cause);
        assert!(format!("{error}").contains("active-role association"));
        assert!(format!("{error:?}").contains("OffsetRolePresenceError"));
        assert_eq!(raw_manifest, old_manifest);
        assert_eq!(raw_catalog, old_catalog);
    }
}
#[test]
fn different_nonempty_subsets_never_retire_absent_canonical_roles() {
    let fixture = sources::fixture();
    let allowance = fixture.allowance();
    let input = fixture.input();
    let bound = input.bind_allowance(&allowance).expect("actual input");
    let raw_catalog = raw(10, &[2, 5, 9]);
    let original = raw_catalog.clone();
    let catalog = catalog(&raw_catalog);
    for second in [None, Some(2), Some(5)] {
        let raw_manifest = manifest(&input, second);
        let admitted = raw_manifest
            .admit(&bound, OffsetCandidateLimits::new(2))
            .expect("subset declaration");
        let presence = admitted
            .bind_active_roles(&catalog, OffsetRolePresenceLimits::new(2, 4, 3))
            .expect("active subset");
        assert_eq!(
            presence.outputs().len(),
            if second.is_some() { 2 } else { 1 }
        );
        for tag in [2, 5, 9] {
            assert!(catalog.is_active(LocalTag::new(tag)));
        }
        assert_eq!(raw_catalog, original);
    }
    assert_eq!(catalog.catalog().definitions().len(), 4);
    assert!(!catalog.is_active(LocalTag::new(7)));
}
