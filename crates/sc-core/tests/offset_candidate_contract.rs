//! Logical metadata candidates only;these fixtures are not G2-computed fragment results.
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
fn fragment(tag: u32, a: (i64, i64), b: (i64, i64)) -> OffsetCandidateFragment {
    OffsetCandidateFragment::new(
        LocalTag::new(tag),
        OffsetInterval::new(parameter(a.0, a.1), parameter(b.0, b.1)).expect("positive domain"),
    )
}
fn roles() -> Vec<OffsetCandidateFragment> {
    vec![
        fragment(9, (1, 4), (1, 3)),
        fragment(2, (1, 3), (1, 2)),
        fragment(u32::MAX, (2, 3), (3, 4)),
    ]
}
#[test]
fn complete_variable_roles_keep_explicit_tags_original_fields_gaps_and_source_order() {
    let f = fixture();
    let allowance = f.allowance();
    let input = f.input();
    let bound = input
        .bind_allowance(&allowance)
        .expect("actual input binding");
    for fragments in [roles(), vec![fragment(37, (1, 4), (3, 4))]] {
        let raw = OffsetCandidateManifest::new(input.operation(), *input.source(), fragments);
        let before = raw.clone();
        let limit = OffsetCandidateLimits::new(raw.fragments().len());
        let admitted = raw
            .admit(&bound, limit)
            .expect("complete original metadata");
        assert!(std::ptr::eq(admitted.manifest(), &raw));
        assert!(std::ptr::eq(admitted.bound_input(), &bound));
        assert!(std::ptr::eq(admitted.fragments(), raw.fragments()));
        assert_eq!(admitted.limits(), limit);
        assert_eq!(limit.fragments(), raw.fragments().len());
        assert_eq!(raw.operation(), input.operation());
        assert_eq!(raw.source(), input.source());
        let outputs = admitted.outputs();
        assert_eq!(outputs.len(), raw.fragments().len());
        for ((edge, actual), original) in outputs.zip(raw.fragments()) {
            assert!(std::ptr::eq(actual, original));
            assert_eq!(edge, EdgeRef::new(input.operation(), original.tag()));
            assert!(std::ptr::eq(
                actual.source_interval(),
                original.source_interval()
            ));
        }
        assert_eq!(
            admitted
                .outputs()
                .rev()
                .map(|(edge, _)| edge.tag())
                .collect::<Vec<_>>(),
            raw.fragments()
                .iter()
                .rev()
                .map(OffsetCandidateFragment::tag)
                .collect::<Vec<_>>()
        );
        assert_eq!(raw, before);
        assert_eq!(input.direction(), Direction::Reversed);
        assert_eq!(bound.width(), &AllowanceWidth::Formula(id(70)));
        assert!(f.ledger.is_live(f.definition.edge));
    }
    assert_eq!(
        roles().iter().map(|f| f.tag().get()).collect::<Vec<_>>(),
        vec![9, 2, u32::MAX]
    );
}
#[test]
fn complete_quota_then_empty_and_actual_operation_source_association_precede_fragment_checks() {
    use OffsetCandidateRefusal as R;
    let f = fixture();
    let allowance = f.allowance();
    let input = f.input();
    let bound = input.bind_allowance(&allowance).expect("real association");
    let source = *input.source();
    let wrong_direction = DirectedRange {
        direction: Direction::Original,
        ..source
    };
    let wrong_bounds = DirectedRange {
        range: EdgeRange::whole(source.range.edge()),
        ..source
    };
    let wrong_edge = DirectedRange {
        range: EdgeRange::whole(EdgeRef::new(id(999), LocalTag::FIRST)),
        ..source
    };
    let rows = [
        (
            OffsetCandidateManifest::new(id(999), wrong_edge, roles()),
            OffsetCandidateLimits::new(2),
            R::Fragments {
                required: 3,
                allowed: 2,
            },
        ),
        (
            OffsetCandidateManifest::new(id(999), wrong_edge, vec![]),
            OffsetCandidateLimits::new(0),
            R::Empty,
        ),
        (
            OffsetCandidateManifest::new(id(999), wrong_edge, roles()),
            OffsetCandidateLimits::new(3),
            R::Operation,
        ),
        (
            OffsetCandidateManifest::new(input.operation(), wrong_edge, roles()),
            OffsetCandidateLimits::new(3),
            R::Source,
        ),
        (
            OffsetCandidateManifest::new(input.operation(), wrong_bounds, roles()),
            OffsetCandidateLimits::new(3),
            R::Source,
        ),
        (
            OffsetCandidateManifest::new(input.operation(), wrong_direction, roles()),
            OffsetCandidateLimits::new(3),
            R::Source,
        ),
    ];
    for (raw, limits, expected) in rows {
        let before = raw.clone();
        let error = raw.admit(&bound, limits).expect_err("no admitted prefix");
        assert!(std::ptr::eq(error.manifest(), &raw));
        assert!(std::ptr::eq(error.bound_input(), &bound));
        assert_eq!(error.limits(), limits);
        assert_eq!(error.refusal(), expected);
        assert_eq!(raw, before);
    }
}
#[test]
fn original_fragment_tag_bounds_and_order_refusals_keep_real_occurrence_indices() {
    use OffsetCandidateRefusal as R;
    let f = fixture();
    let allowance = f.allowance();
    let input = f.input();
    let bound = input.bind_allowance(&allowance).expect("real association");
    for (fragments, expected) in [
        (
            vec![
                fragment(9, (1, 4), (1, 3)),
                fragment(2, (1, 3), (1, 2)),
                fragment(9, (3, 4), (1, 1)),
            ],
            R::DuplicateTag {
                first: 0,
                second: 2,
            },
        ),
        (
            vec![fragment(9, (0, 1), (1, 3))],
            R::SourceBounds { fragment: 0 },
        ),
        (
            vec![fragment(9, (1, 4), (1, 1))],
            R::SourceBounds { fragment: 0 },
        ),
        (
            vec![fragment(9, (1, 4), (1, 2)), fragment(2, (1, 3), (3, 4))],
            R::SourceOrder {
                previous: 0,
                next: 1,
            },
        ),
        (
            vec![fragment(9, (1, 2), (3, 4)), fragment(2, (1, 4), (1, 3))],
            R::SourceOrder {
                previous: 0,
                next: 1,
            },
        ),
    ] {
        let raw = OffsetCandidateManifest::new(input.operation(), *input.source(), fragments);
        let before = raw.clone();
        let error = raw
            .admit(&bound, OffsetCandidateLimits::new(3))
            .expect_err("actual original fragment refusal");
        assert_eq!(error.refusal(), expected);
        assert!(std::ptr::eq(error.manifest(), &raw));
        assert_eq!(raw, before);
    }
}
