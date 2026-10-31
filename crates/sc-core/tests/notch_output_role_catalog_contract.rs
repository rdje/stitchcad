//! Semantic-notch metadata has authentic distinct owners;raw payload sharing issues no entity id.
#![allow(clippy::expect_used)]
use sc_core::{
    construction::*,
    name::MachineToken,
    ontology::{EntityId, LocalTag},
};
fn definition(tag: u32, key: &str) -> NotchOutputRoleDefinition {
    NotchOutputRoleDefinition::new(
        LocalTag::new(tag),
        MachineToken::new(key).expect("machine key"),
    )
}
fn raw() -> NotchOutputRoleCatalog {
    NotchOutputRoleCatalog::new(
        EntityId::from_bits(10),
        vec![
            definition(9, "balance"),
            definition(2, "end"),
            definition(7, "latent"),
        ],
        vec![LocalTag::new(2), LocalTag::new(9)],
    )
}
fn limits() -> NotchOutputRoleCatalogLimits {
    NotchOutputRoleCatalogLimits::new(3, 3, 16)
}
#[test]
fn authentic_notch_members_keep_original_orders_fields_and_raw_aliases_without_entity_issuance() {
    let raw = raw();
    let snapshot = raw.clone();
    let admitted = raw.admit(limits()).expect("whole notch metadata");
    assert!(std::ptr::eq(admitted.catalog(), &raw));
    assert_eq!(admitted.catalog().operation(), EntityId::from_bits(10));
    assert_eq!(admitted.limits(), limits());
    assert_eq!(
        (admitted.key_bytes(), admitted.workspace_entries()),
        (16, 10)
    );
    for (role, (active_index, definition_index)) in admitted.active_roles().zip([(0, 1), (1, 0)]) {
        assert!(std::ptr::eq(role.catalog(), &admitted));
        assert_eq!(role.index(), active_index);
        assert!(std::ptr::eq(
            role.tag(),
            raw.active_tags().get(active_index).expect("original field")
        ));
        assert!(std::ptr::eq(
            role.definition(),
            raw.definitions()
                .get(definition_index)
                .expect("original definition")
        ));
        let point_raw: &PointOutputRoleDefinition = role.definition();
        let edge_raw: &EdgeOutputRoleDefinition = role.definition();
        assert!(std::ptr::eq(point_raw, edge_raw));
    }
    assert_eq!(
        admitted
            .active_roles()
            .rev()
            .map(|role| role.definition().key().as_str())
            .collect::<Vec<_>>(),
        vec!["balance", "end"]
    );
    assert_eq!(
        admitted
            .active_role(LocalTag::new(2))
            .expect("actual member")
            .index(),
        0
    );
    assert!(admitted.is_active(LocalTag::new(9)));
    assert!(!admitted.is_active(LocalTag::new(7)));
    assert!(admitted.active_role(LocalTag::new(7)).is_none());
    assert!(admitted.active_role(LocalTag::new(88)).is_none());
    assert!(admitted.definition(LocalTag::new(88)).is_none());
    assert!(std::ptr::eq(
        admitted
            .definition(LocalTag::new(7))
            .expect("inactive definition"),
        raw.definitions()
            .get(2)
            .expect("original latent definition")
    ));
    assert_eq!(raw, snapshot);
    for raw in [
        NotchOutputRoleCatalog::new(EntityId::from_bits(0), vec![], vec![]),
        NotchOutputRoleCatalog::new(
            EntityId::from_bits(10),
            snapshot.definitions().to_vec(),
            vec![],
        ),
    ] {
        let admitted = raw.admit(limits()).expect("empty/inactive metadata");
        assert_eq!(admitted.active_roles().len(), 0);
        assert_eq!(admitted.workspace_entries(), 2 * raw.definitions().len());
        assert_eq!(
            admitted.key_bytes(),
            if raw.definitions().is_empty() { 0 } else { 16 }
        );
    }
}
#[test]
fn complete_resource_refusal_and_original_duplicate_cause_keep_actual_notch_owner_without_prefix() {
    use NotchOutputRoleCatalogRefusal as R;
    let raw = NotchOutputRoleCatalog::new(
        EntityId::from_bits(10),
        vec![definition(9, "balance"), definition(9, "balance")],
        vec![LocalTag::new(2), LocalTag::new(2)],
    );
    let snapshot = raw.clone();
    for (limits, expected) in [
        (
            NotchOutputRoleCatalogLimits::new(1, 0, 0),
            R::Definitions {
                required: 2,
                allowed: 1,
            },
        ),
        (
            NotchOutputRoleCatalogLimits::new(2, 1, 0),
            R::Active {
                required: 2,
                allowed: 1,
            },
        ),
        (
            NotchOutputRoleCatalogLimits::new(2, 2, 13),
            R::KeyBytes {
                required: 14,
                allowed: 13,
            },
        ),
        (
            NotchOutputRoleCatalogLimits::new(2, 2, 14),
            R::DuplicateTag {
                first: 0,
                second: 1,
            },
        ),
    ] {
        let error = raw.admit(limits).expect_err("whole typed refusal");
        assert!(std::ptr::eq(error.catalog(), &raw));
        assert_eq!(error.limits(), limits);
        assert_eq!(error.refusal(), expected);
        assert!(error
            .to_string()
            .contains("notch output role catalogue admission"));
        assert!(format!("{error:?}").contains("NotchOutputRoleCatalogError"));
        assert_eq!(raw, snapshot);
    }
}
#[test]
fn equal_contents_and_explicit_point_edge_declarations_do_not_reparent_notch_membership() {
    let notch = raw();
    let equal = notch.clone();
    let first = notch.admit(limits()).expect("first actual owner");
    let second = equal.admit(limits()).expect("equal other owner");
    let role = first.active_roles().next().expect("original member");
    assert!(std::ptr::eq(role.catalog(), &first));
    assert!(!std::ptr::eq(role.catalog(), &second));
    let point = PointOutputRoleCatalog::new(
        notch.operation(),
        notch.definitions().to_vec(),
        notch.active_tags().to_vec(),
    );
    let edge = EdgeOutputRoleCatalog::new(
        notch.operation(),
        notch.definitions().to_vec(),
        notch.active_tags().to_vec(),
    );
    let points = point
        .admit(limits())
        .expect("independent point declaration");
    let edges = edge.admit(limits()).expect("independent edge declaration");
    let tags = first.active_roles().map(|r| *r.tag()).collect::<Vec<_>>();
    assert_eq!(
        tags,
        points.active_roles().map(|r| *r.tag()).collect::<Vec<_>>()
    );
    assert_eq!(
        tags,
        edges.active_roles().map(|r| *r.tag()).collect::<Vec<_>>()
    );
    assert!(std::ptr::eq(first.catalog(), &notch));
    assert!(std::ptr::eq(points.catalog(), &point));
    assert!(std::ptr::eq(edges.catalog(), &edge));
}
