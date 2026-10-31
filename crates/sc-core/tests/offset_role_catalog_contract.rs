//! Declared metadata identity/membership only;keys do not classify geometry or grant authority.
#![allow(clippy::expect_used)]
use sc_core::{construction::*, name::MachineToken, ontology::*};
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn role(tag: u32, key: &str) -> OffsetRoleDefinition {
    OffsetRoleDefinition::new(
        LocalTag::new(tag),
        MachineToken::new(key).expect("explicit machine key"),
    )
}
fn definitions() -> Vec<OffsetRoleDefinition> {
    vec![role(9, "main"), role(2, "trimmed_end"), role(7, "latent")]
}
#[test]
fn all_definitions_and_active_fields_remain_authentic_in_both_original_orders() {
    let raw = OffsetRoleCatalog::new(
        id(10),
        definitions(),
        vec![LocalTag::new(2), LocalTag::new(9)],
    );
    let before = raw.clone();
    let limits = OffsetRoleCatalogLimits::new(3, 2, 21);
    let admitted = raw.admit(limits).expect("complete declared metadata");
    assert!(std::ptr::eq(admitted.catalog(), &raw));
    assert_eq!(admitted.limits(), limits);
    assert_eq!(
        (limits.definitions(), limits.active(), limits.key_bytes()),
        (3, 2, 21)
    );
    assert_eq!(admitted.key_bytes(), 21);
    assert_eq!(admitted.workspace_entries(), 10);
    assert_eq!(raw.operation(), id(10));
    assert_eq!(admitted.active_roles().len(), 2);
    for (index, view) in admitted.active_roles().enumerate() {
        assert!(std::ptr::eq(view.catalog(), &admitted));
        assert_eq!(view.index(), index);
        assert!(std::ptr::eq(
            view.tag(),
            raw.active_tags()
                .get(index)
                .expect("original active occurrence")
        ));
        let actual = raw
            .definitions()
            .iter()
            .find(|role| role.tag() == *view.tag())
            .expect("actual role definition");
        assert!(std::ptr::eq(view.definition(), actual));
        assert!(std::ptr::eq(view.definition().key(), actual.key()));
        assert!(admitted.is_active(actual.tag()));
        assert!(std::ptr::eq(
            admitted
                .definition(actual.tag())
                .expect("issued definition"),
            actual
        ));
    }
    assert_eq!(
        admitted
            .active_roles()
            .map(|view| view.definition().key().as_str())
            .collect::<Vec<_>>(),
        vec!["trimmed_end", "main"]
    );
    assert_eq!(
        admitted
            .active_roles()
            .rev()
            .map(|view| view.index())
            .collect::<Vec<_>>(),
        vec![1, 0]
    );
    let inactive = raw
        .definitions()
        .get(2)
        .expect("inactive issued definition");
    assert!(!admitted.is_active(inactive.tag()));
    assert!(std::ptr::eq(
        admitted.definition(inactive.tag()).expect("still defined"),
        inactive
    ));
    assert!(admitted.definition(LocalTag::new(999)).is_none());
    assert_eq!(raw, before);
}
#[test]
fn complete_counts_and_key_bytes_precede_original_tag_key_and_membership_causes() {
    use OffsetRoleCatalogRefusal as R;
    let dup = || vec![role(9, "main"), role(9, "main"), role(7, "latent")];
    let rows = [
        (
            dup(),
            vec![LocalTag::new(99); 3],
            OffsetRoleCatalogLimits::new(2, 0, 0),
            R::Definitions {
                required: 3,
                allowed: 2,
            },
        ),
        (
            definitions(),
            vec![LocalTag::new(99); 3],
            OffsetRoleCatalogLimits::new(3, 2, 0),
            R::Active {
                required: 3,
                allowed: 2,
            },
        ),
        (
            definitions(),
            vec![LocalTag::new(9), LocalTag::new(9)],
            OffsetRoleCatalogLimits::new(3, 2, 20),
            R::KeyBytes {
                required: 21,
                allowed: 20,
            },
        ),
        (
            dup(),
            vec![LocalTag::new(99)],
            OffsetRoleCatalogLimits::new(3, 2, 100),
            R::DuplicateTag {
                first: 0,
                second: 1,
            },
        ),
        (
            vec![role(9, "main"), role(2, "main")],
            vec![LocalTag::new(99)],
            OffsetRoleCatalogLimits::new(3, 2, 100),
            R::DuplicateKey {
                first: 0,
                second: 1,
            },
        ),
        (
            definitions(),
            vec![LocalTag::new(9), LocalTag::new(9)],
            OffsetRoleCatalogLimits::new(3, 2, 21),
            R::DuplicateActive {
                first: 0,
                second: 1,
            },
        ),
        (
            definitions(),
            vec![LocalTag::new(9), LocalTag::new(99)],
            OffsetRoleCatalogLimits::new(3, 2, 21),
            R::UndefinedActive {
                index: 1,
                tag: LocalTag::new(99),
            },
        ),
    ];
    for (definitions, active, limits, cause) in rows {
        let raw = OffsetRoleCatalog::new(id(10), definitions, active);
        let before = raw.clone();
        let error = raw.admit(limits).expect_err("no accepted prefix");
        assert!(std::ptr::eq(error.catalog(), &raw));
        assert_eq!(error.limits(), limits);
        assert_eq!(error.refusal(), cause);
        assert_eq!(raw, before);
    }
}
#[test]
fn empty_initial_and_fully_inactive_catalogues_are_metadata_states_without_default_roles() {
    let raw = OffsetRoleCatalog::new(id(10), vec![], vec![]);
    let admitted = raw
        .admit(OffsetRoleCatalogLimits::new(0, 0, 0))
        .expect("explicit empty initial catalogue");
    assert_eq!(
        (
            admitted.key_bytes(),
            admitted.workspace_entries(),
            admitted.active_roles().len()
        ),
        (0, 0, 0)
    );
    assert!(admitted.definition(LocalTag::FIRST).is_none());
    let raw = OffsetRoleCatalog::new(id(10), vec![role(u32::MAX, "retained")], vec![]);
    let admitted = raw
        .admit(OffsetRoleCatalogLimits::new(1, 0, 8))
        .expect("inactive definition still reserved metadata");
    assert!(std::ptr::eq(
        admitted
            .definition(LocalTag::new(u32::MAX))
            .expect("explicit sparse definition"),
        raw.definitions().first().expect("original field")
    ));
    assert!(!admitted.is_active(LocalTag::new(u32::MAX)));
    assert_eq!(admitted.active_roles().len(), 0);
}
