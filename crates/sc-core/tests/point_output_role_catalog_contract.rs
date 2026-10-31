//! Point-domain catalogue ownership over unchanged bounded raw role metadata admission.
#![allow(clippy::expect_used)]
use sc_core::{
    construction::*,
    name::MachineToken,
    ontology::{EntityId, LocalTag},
};

fn definition(tag: u32, key: &str) -> PointOutputRoleDefinition {
    PointOutputRoleDefinition::new(
        LocalTag::new(tag),
        MachineToken::new(key).expect("machine key"),
    )
}
fn raw(active: &[u32]) -> PointOutputRoleCatalog {
    PointOutputRoleCatalog::new(
        EntityId::from_bits(10),
        vec![
            definition(9, "pivot"),
            definition(2, "tip"),
            definition(7, "latent"),
        ],
        active.iter().copied().map(LocalTag::new).collect(),
    )
}
fn limits() -> PointOutputRoleCatalogLimits {
    PointOutputRoleCatalogLimits::new(3, 3, 14)
}

#[test]
fn point_views_borrow_actual_fields_in_both_original_orders_and_share_only_raw_metadata_values() {
    let raw = raw(&[2, 9]);
    let original = raw.clone();
    let catalog = raw.admit(limits()).expect("complete point metadata");
    assert!(std::ptr::eq(catalog.catalog(), &raw));
    assert_eq!(catalog.catalog().operation(), EntityId::from_bits(10));
    assert_eq!(catalog.limits(), limits());
    assert_eq!(catalog.key_bytes(), 14);
    assert_eq!(catalog.workspace_entries(), 10);
    for (role, index, definition_index) in catalog
        .active_roles()
        .zip([0, 1])
        .map(|(role, index)| (role, index, 1 - index))
    {
        assert!(std::ptr::eq(role.catalog(), &catalog));
        assert_eq!(role.index(), index);
        assert!(std::ptr::eq(
            role.tag(),
            raw.active_tags().get(index).expect("original tag")
        ));
        assert!(std::ptr::eq(
            role.definition(),
            raw.definitions()
                .get(definition_index)
                .expect("original definition")
        ));
        let shared: &EdgeOutputRoleDefinition = role.definition();
        assert!(std::ptr::eq(shared, role.definition()));
    }
    assert_eq!(
        catalog
            .active_roles()
            .rev()
            .map(|r| r.definition().key().as_str())
            .collect::<Vec<_>>(),
        vec!["pivot", "tip"]
    );
    assert_eq!(
        catalog
            .active_role(LocalTag::new(2))
            .expect("actual membership")
            .index(),
        0
    );
    assert!(catalog.is_active(LocalTag::new(9)));
    assert!(!catalog.is_active(LocalTag::new(7)));
    assert!(catalog.active_role(LocalTag::new(7)).is_none());
    assert!(catalog.active_role(LocalTag::new(88)).is_none());
    assert!(catalog.definition(LocalTag::new(88)).is_none());
    assert!(std::ptr::eq(
        catalog
            .definition(LocalTag::new(7))
            .expect("inactive definition"),
        raw.definitions().get(2).expect("original latent field")
    ));
    assert_eq!(raw, original);
}

#[test]
fn whole_populations_and_key_bytes_precede_duplicate_fields_and_preserve_point_context() {
    use PointOutputRoleCatalogRefusal as R;
    let raw = PointOutputRoleCatalog::new(
        EntityId::from_bits(10),
        vec![definition(9, "pivot"), definition(9, "pivot")],
        vec![LocalTag::new(2), LocalTag::new(2)],
    );
    let original = raw.clone();
    for (limits, refusal) in [
        (
            PointOutputRoleCatalogLimits::new(1, 0, 0),
            R::Definitions {
                required: 2,
                allowed: 1,
            },
        ),
        (
            PointOutputRoleCatalogLimits::new(2, 1, 0),
            R::Active {
                required: 2,
                allowed: 1,
            },
        ),
        (
            PointOutputRoleCatalogLimits::new(2, 2, 9),
            R::KeyBytes {
                required: 10,
                allowed: 9,
            },
        ),
        (
            PointOutputRoleCatalogLimits::new(2, 2, 10),
            R::DuplicateTag {
                first: 0,
                second: 1,
            },
        ),
    ] {
        let error = raw
            .admit(limits)
            .expect_err("whole refusal,not prefix admission");
        assert!(std::ptr::eq(error.catalog(), &raw));
        assert_eq!(error.limits(), limits);
        assert_eq!(error.refusal(), refusal);
        assert!(format!("{error}").contains("point output role catalogue admission"));
        assert!(format!("{error:?}").contains("PointOutputRoleCatalogError"));
        assert_eq!(raw, original);
    }
}

#[test]
fn duplicate_key_and_original_active_duplicate_or_undefined_causes_are_not_reordered() {
    use PointOutputRoleCatalogRefusal as R;
    let duplicate_key = PointOutputRoleCatalog::new(
        EntityId::from_bits(10),
        vec![definition(9, "pivot"), definition(2, "pivot")],
        vec![],
    );
    assert_eq!(
        duplicate_key
            .admit(limits())
            .expect_err("same key,new tag")
            .refusal(),
        R::DuplicateKey {
            first: 0,
            second: 1
        }
    );
    let repeated = raw(&[2, 2, 88]);
    assert_eq!(
        repeated
            .admit(limits())
            .expect_err("original duplicate before later unknown")
            .refusal(),
        R::DuplicateActive {
            first: 0,
            second: 1
        }
    );
    let undefined = raw(&[88, 2, 2]);
    assert_eq!(
        undefined
            .admit(limits())
            .expect_err("original unknown before later duplicate")
            .refusal(),
        R::UndefinedActive {
            index: 0,
            tag: LocalTag::new(88)
        }
    );
}

#[test]
fn empty_and_inactive_point_states_remain_metadata_without_generated_point_refs_or_retirement() {
    for raw in [
        PointOutputRoleCatalog::new(EntityId::from_bits(10), vec![], vec![]),
        raw(&[]),
    ] {
        let original = raw.clone();
        let catalog = raw.admit(limits()).expect("metadata state only");
        assert_eq!(catalog.active_roles().len(), 0);
        assert_eq!(
            catalog.key_bytes(),
            if raw.definitions().is_empty() { 0 } else { 14 }
        );
        assert_eq!(catalog.workspace_entries(), 2 * raw.definitions().len());
        assert_eq!(raw, original);
    }
}

#[test]
fn equal_contents_do_not_reparent_membership_and_explicit_point_and_edge_owners_stay_distinct() {
    let point = raw(&[2, 9]);
    let equal = point.clone();
    let first = point.admit(limits()).expect("first original owner");
    let second = equal.admit(limits()).expect("separate original owner");
    let role = first
        .active_roles()
        .next()
        .expect("first original membership");
    assert!(std::ptr::eq(role.catalog(), &first));
    assert!(!std::ptr::eq(role.catalog(), &second));
    let edge = EdgeOutputRoleCatalog::new(
        point.operation(),
        point.definitions().to_vec(),
        point.active_tags().to_vec(),
    );
    let edges = edge
        .admit(limits())
        .expect("explicit independent edge declaration");
    assert_eq!(
        first.active_roles().map(|r| *r.tag()).collect::<Vec<_>>(),
        edges.active_roles().map(|r| *r.tag()).collect::<Vec<_>>()
    );
    assert!(std::ptr::eq(first.catalog(), &point));
    assert!(std::ptr::eq(edges.catalog(), &edge));
}
