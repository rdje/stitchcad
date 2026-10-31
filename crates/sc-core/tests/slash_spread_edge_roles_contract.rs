//! Shared edge roles and actual slash intent association;no geometry or issuance evidence.
#![allow(clippy::expect_used)]
use sc_core::{
    construction::*, name::MachineToken, ontology::*, recipe::FormulaExpressionDocument,
};
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn intent() -> SlashSpreadOutputIntent {
    let input = SlashSpreadConstruction::rotation(
        id(10),
        id(50),
        DirectedRange {
            range: EdgeRange::whole(EdgeRef::new(id(20), LocalTag::new(4))),
            direction: Direction::Reversed,
        },
        SlashSpreadSide::Left,
        PointRef::new(id(30), LocalTag::new(7)),
        FormulaExpressionDocument::new("10 deg".into()).expect("angle"),
    )
    .expect("spatial intent");
    SlashSpreadOutputIntent::new(
        input,
        SlashSpreadBoundaryPolicy::StraightBridges,
        SlashSpreadFullnessIntent::Added,
    )
}
fn raw(operation: u128, active: &[u32]) -> EdgeOutputRoleCatalog {
    EdgeOutputRoleCatalog::new(
        id(operation),
        [(9, "moving"), (2, "stationary"), (7, "latent")]
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
    raw.admit(EdgeOutputRoleCatalogLimits::new(3, 2, 22))
        .expect("whole metadata")
}
#[test]
fn actual_slash_intent_and_original_active_fields_are_shared_with_legacy_offset_aliases() {
    let intent = intent();
    let raw = raw(10, &[2, 9]);
    let catalog = catalog(&raw);
    let limits = EdgeOutputRoleCatalogLimits::new(3, 2, 22);
    let binding = intent
        .bind_edge_roles(&catalog, limits)
        .expect("same operation");
    assert!(std::ptr::eq(binding.intent(), &intent));
    assert!(std::ptr::eq(binding.catalog(), &catalog));
    assert_eq!(binding.limits(), limits);
    let legacy: &AdmittedOffsetRoleCatalog = binding.catalog();
    assert!(std::ptr::eq(legacy, &catalog));
    let legacy_raw: &OffsetRoleCatalog = binding.catalog().catalog();
    assert!(std::ptr::eq(legacy_raw, &raw));
    assert_eq!(catalog.key_bytes(), 22);
    for (role, active_index, definition_index) in binding
        .catalog()
        .active_roles()
        .zip([0, 1])
        .map(|(r, i)| (r, i, 1 - i))
    {
        assert!(std::ptr::eq(role.catalog(), &catalog));
        assert_eq!(role.index(), active_index);
        assert!(std::ptr::eq(
            role.tag(),
            raw.active_tags()
                .get(active_index)
                .expect("actual membership")
        ));
        assert!(std::ptr::eq(
            role.definition(),
            raw.definitions()
                .get(definition_index)
                .expect("actual definition")
        ));
    }
    assert_eq!(binding.intent().input().operation(), id(10));
    assert_eq!(
        binding
            .catalog()
            .active_roles()
            .rev()
            .map(|r| r.definition().key().as_str())
            .collect::<Vec<_>>(),
        vec!["moving", "stationary"]
    );
}
#[test]
fn complete_catalogue_quotas_precede_owner_and_keep_both_actual_sources() {
    use SlashSpreadRoleBindingRefusal as R;
    let intent = intent();
    let raw = raw(99, &[2, 9]);
    let original = raw.clone();
    let catalog = catalog(&raw);
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
            EdgeOutputRoleCatalogLimits::new(3, 2, 21),
            R::KeyBytes {
                required: 22,
                allowed: 21,
            },
        ),
        (EdgeOutputRoleCatalogLimits::new(3, 2, 22), R::Operation),
    ] {
        let error = intent
            .bind_edge_roles(&catalog, limits)
            .expect_err("whole refusal");
        assert!(std::ptr::eq(error.intent(), &intent));
        assert!(std::ptr::eq(error.catalog(), &catalog));
        assert_eq!(error.limits(), limits);
        assert_eq!(error.refusal(), cause);
        assert!(format!("{error}").contains("slash/spread edge-role association"));
        assert!(format!("{error:?}").contains("SlashSpreadRoleBindingError"));
        assert_eq!(raw, original);
    }
}
#[test]
fn empty_and_inactive_catalogues_remain_metadata_without_an_executable_manifest() {
    let intent = intent();
    for raw in [
        EdgeOutputRoleCatalog::new(id(10), vec![], vec![]),
        raw(10, &[]),
    ] {
        let original = raw.clone();
        let catalog = catalog(&raw);
        let binding = intent
            .bind_edge_roles(&catalog, EdgeOutputRoleCatalogLimits::new(3, 0, 22))
            .expect("metadata association");
        assert_eq!(binding.catalog().active_roles().len(), 0);
        assert_eq!(raw, original);
        assert!(std::ptr::eq(binding.intent(), &intent));
        assert!(std::ptr::eq(binding.catalog().catalog(), &raw));
    }
}
