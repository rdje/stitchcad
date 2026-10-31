//! Actual ordered declaration dependencies and definite local refusals, without geometry.
#![allow(clippy::expect_used)]
use sc_core::{
    construction::{LineConstruction as Line, LineEndpoint as Role},
    ontology::{EdgeRef, EntityId, LocalTag, PointRef},
};
fn id(value: u128) -> EntityId {
    EntityId::from_bits(value)
}
fn point(creator: u128, tag: u32) -> PointRef {
    PointRef::new(id(creator), LocalTag::new(tag))
}

#[test]
fn source_occurrences_keep_declared_direction_and_borrow_the_exact_original_fields() {
    let start = point(99, 12);
    let end = point(80, 7);
    let line = Line::new(id(10), start, end).expect("numeric id order grants no chronology");
    let uses = line.dependencies().collect::<Vec<_>>();
    assert_eq!(uses.len(), 2);
    let first = uses.first().expect("actual start use");
    let last = uses.get(1).expect("actual end use");
    assert_eq!((first.endpoint(), first.point()), (Role::Start, start));
    assert_eq!((last.endpoint(), last.point()), (Role::End, end));
    assert!(uses.iter().all(|use_| std::ptr::eq(use_.line(), &line)));
    assert_eq!(line.output(), EdgeRef::new(id(10), LocalTag::FIRST));
    assert_eq!(
        line.dependencies()
            .rev()
            .map(|use_| use_.endpoint())
            .collect::<Vec<_>>(),
        [Role::End, Role::Start]
    );
}

#[test]
fn definite_self_supply_refuses_original_role_reference_and_first_failure() {
    for (start, end, wanted, reference) in [
        (point(10, 12), point(80, 7), Role::Start, point(10, 12)),
        (point(99, 12), point(10, 7), Role::End, point(10, 7)),
        (point(10, 12), point(10, 7), Role::Start, point(10, 12)),
    ] {
        let error =
            Line::new(id(10), start, end).expect_err("own operation cannot supply a prior point");
        assert_eq!(error.operation(), id(10));
        assert_eq!(error.endpoint(), wanted);
        assert_eq!(error.point(), reference);
    }
}

#[test]
fn repeated_point_uses_remain_distinct_roles_without_silently_certifying_a_nonzero_edge() {
    let source = point(80, 7);
    let line = Line::new(id(10), source, source)
        .expect("declaration is not geometric degeneracy validation");
    let uses = line.dependencies().collect::<Vec<_>>();
    assert_eq!(
        uses.iter().map(|use_| use_.point()).collect::<Vec<_>>(),
        [source, source]
    );
    assert_eq!(
        uses.iter().map(|use_| use_.endpoint()).collect::<Vec<_>>(),
        [Role::Start, Role::End]
    );
    assert!(uses.iter().all(|use_| std::ptr::eq(use_.line(), &line)));
}

#[test]
fn reversing_declaration_preserves_logical_output_but_changes_original_dependency_meaning() {
    let start = point(99, 12);
    let end = point(80, 7);
    let forward = Line::new(id(10), start, end).expect("original declaration");
    let reversed = Line::new(id(10), end, start).expect("distinct reversed intent");
    assert_ne!(forward, reversed);
    assert_eq!(forward.output(), reversed.output());
    assert_eq!(
        forward
            .dependencies()
            .map(|use_| use_.point())
            .collect::<Vec<_>>(),
        [start, end]
    );
    assert_eq!(
        reversed
            .dependencies()
            .map(|use_| use_.point())
            .collect::<Vec<_>>(),
        [end, start]
    );
    // Identity correspondence/current topology repair on publication remains a separate contract.
}
