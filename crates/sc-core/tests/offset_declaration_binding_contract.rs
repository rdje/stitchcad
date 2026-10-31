//! Actual journal association and complete admission;not geometry,current-root or issuer proof.
#![allow(clippy::expect_used)]
use sc_core::{construction::*, ontology::*};
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn parameter(n: i64, d: i64) -> Param {
    Param::new(Rational::new(n, d).expect("rational")).expect("unit interval")
}
#[path = "support/offset_sources.rs"]
mod sources;
use sources::fixture;
fn roles() -> Vec<OffsetCandidateFragment> {
    [
        (9, (1, 4), (1, 3)),
        (2, (1, 3), (1, 2)),
        (u32::MAX, (2, 3), (3, 4)),
    ]
    .into_iter()
    .map(|(tag, a, b)| {
        OffsetCandidateFragment::new(
            LocalTag::new(tag),
            OffsetInterval::new(parameter(a.0, a.1), parameter(b.0, b.1)).expect("positive domain"),
        )
    })
    .collect()
}
#[test]
fn actual_declared_fields_bind_by_explicit_tag_and_keep_original_source_order_and_owners() {
    let mut f = fixture();
    let allowance = f.allowance();
    let input = f.input();
    let source = input.bind_allowance(&allowance).expect("real source");
    let raw = OffsetCandidateManifest::new(input.operation(), *input.source(), roles());
    let admitted = raw
        .admit(&source, OffsetCandidateLimits::new(3))
        .expect("complete metadata");
    f.ledger
        .declare_tagged_edges(
            input.operation(),
            &[LocalTag::new(u32::MAX), LocalTag::new(2), LocalTag::new(9)],
        )
        .expect("actual differently ordered declaration");
    let before = f.ledger.clone();
    let limits = OffsetDeclarationLimits::new(3, 2);
    let bound = admitted
        .bind_declaration(&f.ledger, limits)
        .expect("actual complete tag association");
    assert!(std::ptr::eq(bound.admitted(), &admitted));
    assert!(std::ptr::eq(bound.ledger(), &f.ledger));
    let entry = f.ledger.journal().get(1).expect("actual declared entry");
    assert!(std::ptr::eq(bound.entry(), entry));
    assert_eq!(bound.entry_index(), 1);
    assert_eq!(bound.limits(), limits);
    assert_eq!(limits.roles(), 3);
    assert_eq!(limits.journal_entries(), 2);
    assert_eq!(bound.workspace_entries(), 6);
    let edges = match entry.edit() {
        TopologyEdit::Declared { edges } => Some(edges),
        _ => None,
    }
    .expect("actual declaration kind");
    assert_eq!(bound.outputs().len(), 3);
    for view in bound.outputs() {
        let original = raw
            .fragments()
            .get(view.index())
            .expect("actual candidate occurrence");
        let edge = edges
            .iter()
            .find(|edge| edge.tag() == original.tag())
            .expect("actual edge field");
        assert!(std::ptr::eq(view.declaration(), &bound));
        assert!(std::ptr::eq(view.candidate(), original));
        assert!(std::ptr::eq(view.edge(), edge));
        assert_eq!(
            *view.edge(),
            EdgeRef::new(input.operation(), original.tag())
        );
    }
    assert_eq!(
        bound
            .outputs()
            .map(|view| view.edge().tag().get())
            .collect::<Vec<_>>(),
        vec![9, 2, u32::MAX]
    );
    assert_eq!(
        bound
            .outputs()
            .rev()
            .map(|view| view.index())
            .collect::<Vec<_>>(),
        vec![2, 1, 0]
    );
    assert!(f.ledger.is_live(f.definition.edge));
    assert_eq!(f.ledger, before);
}
#[test]
fn complete_admission_and_real_missing_ambiguous_kind_count_and_tag_causes_keep_all_sources() {
    use OffsetDeclarationRefusal as R;
    let f = fixture();
    let allowance = f.allowance();
    let input = f.input();
    let source = input.bind_allowance(&allowance).expect("real source");
    let raw = OffsetCandidateManifest::new(input.operation(), *input.source(), roles());
    let admitted = raw
        .admit(&source, OffsetCandidateLimits::new(3))
        .expect("complete metadata");
    let mut wrong_kind = f.ledger.clone();
    wrong_kind
        .offset(
            &mut DeterministicIdGenerator::from_start(10),
            f.definition.edge,
            &[OffsetInterval::new(Param::START, Param::END).expect("whole interval")],
        )
        .expect("destructive topology is not a declaration");
    let mut wrong_count = f.ledger.clone();
    wrong_count
        .declare_tagged_edges(id(10), &[LocalTag::new(9)])
        .expect("different complete population");
    let mut wrong_tag = f.ledger.clone();
    wrong_tag
        .declare_tagged_edges(
            id(10),
            &[LocalTag::new(9), LocalTag::new(2), LocalTag::new(37)],
        )
        .expect("different actual role");
    let mut ambiguous = f.ledger.clone();
    // Deliberately misuse separate external id streams;this is not a globally certified ledger.
    // Association must refuse both actual records instead of silently selecting one of them.
    for _ in 0..2 {
        ambiguous
            .declare_edges(&mut DeterministicIdGenerator::from_start(10), 1)
            .expect("legacy allocating interface trusts host nonreuse");
    }
    let rows = [
        (
            f.ledger.clone(),
            OffsetDeclarationLimits::new(2, 0),
            R::Roles {
                required: 3,
                allowed: 2,
            },
        ),
        (
            f.ledger.clone(),
            OffsetDeclarationLimits::new(3, 0),
            R::Journal {
                required: 1,
                allowed: 0,
            },
        ),
        (
            f.ledger.clone(),
            OffsetDeclarationLimits::new(3, 3),
            R::MissingOperation,
        ),
        (
            ambiguous,
            OffsetDeclarationLimits::new(3, 3),
            R::AmbiguousOperation {
                first: 1,
                second: 2,
            },
        ),
        (
            wrong_kind,
            OffsetDeclarationLimits::new(3, 3),
            R::Kind { entry: 1 },
        ),
        (
            wrong_count,
            OffsetDeclarationLimits::new(3, 3),
            R::Count {
                entry: 1,
                required: 3,
                actual: 1,
            },
        ),
        (
            wrong_tag,
            OffsetDeclarationLimits::new(3, 3),
            R::Tag {
                entry: 1,
                fragment: 2,
                tag: LocalTag::new(u32::MAX),
            },
        ),
    ];
    for (ledger, limits, cause) in rows {
        let before = ledger.clone();
        let error = admitted
            .bind_declaration(&ledger, limits)
            .expect_err("no accepted association prefix");
        assert!(std::ptr::eq(error.admitted(), &admitted));
        assert!(std::ptr::eq(error.ledger(), &ledger));
        assert_eq!(error.limits(), limits);
        assert_eq!(error.refusal(), cause);
        assert_eq!(ledger, before);
    }
}
#[test]
fn authentic_historical_declaration_binding_does_not_claim_current_liveness_or_geometric_mapping() {
    let mut f = fixture();
    let allowance = f.allowance();
    let input = f.input();
    let source = input.bind_allowance(&allowance).expect("original source");
    let raw = OffsetCandidateManifest::new(input.operation(), *input.source(), roles());
    let admitted = raw
        .admit(&source, OffsetCandidateLimits::new(3))
        .expect("metadata");
    let outputs = f
        .ledger
        .declare_tagged_edges(
            input.operation(),
            &[LocalTag::new(9), LocalTag::new(2), LocalTag::new(u32::MAX)],
        )
        .expect("actual declaration");
    let retired = *outputs.get(1).expect("second actual role");
    f.ledger
        .delete(&mut DeterministicIdGenerator::from_start(1000), retired)
        .expect("actual retirement");
    assert!(matches!(
        f.ledger.resolve(retired, parameter(1, 2)),
        Resolution::Unresolved(_)
    ));
    let bound = admitted
        .bind_declaration(&f.ledger, OffsetDeclarationLimits::new(3, 3))
        .expect("historical declaration remains authentic");
    assert_eq!(bound.outputs().count(), 3);
    assert_eq!(
        bound
            .outputs()
            .filter(|view| !f.ledger.is_live(*view.edge()))
            .count(),
        1
    );
    assert!(f.ledger.is_live(f.definition.edge));
    assert_eq!(bound.entry().operation(), input.operation());
}
