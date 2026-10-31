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

#[test]
fn actual_queries_preserve_all_original_candidates_at_endpoints_shared_boundaries_and_gaps() {
    use OffsetCandidateMatchKind as K;
    let f = fixture();
    let allowance = f.allowance();
    let input = f.input();
    let bound = input.bind_allowance(&allowance).expect("actual input");
    let raw = OffsetCandidateManifest::new(input.operation(), *input.source(), roles());
    let admitted = raw
        .admit(&bound, OffsetCandidateLimits::new(3))
        .expect("complete candidate metadata");
    let before = f.ledger.clone();
    for (n, d, kind, expected) in [
        (1, 4, K::Unique, vec![0]),
        (1, 3, K::Ambiguous, vec![0, 1]),
        (5, 12, K::Unique, vec![1]),
        (1, 2, K::Unique, vec![1]),
        (7, 12, K::Gap, vec![]),
        (2, 3, K::Unique, vec![2]),
        (3, 4, K::Unique, vec![2]),
    ] {
        let anchor = EdgeAnchor {
            edge: input.source().range.edge(),
            param: parameter(n, d),
        };
        let query = admitted
            .query_source(anchor)
            .expect("exact original source query");
        assert!(std::ptr::eq(query.admitted(), &admitted));
        assert_eq!(query.anchor(), &anchor);
        assert_eq!(query.kind(), kind);
        assert_eq!(
            query
                .candidates()
                .map(OffsetCandidateMatch::index)
                .collect::<Vec<_>>(),
            expected
        );
        assert_eq!(
            query
                .candidates()
                .rev()
                .map(OffsetCandidateMatch::index)
                .collect::<Vec<_>>(),
            expected.iter().rev().copied().collect::<Vec<_>>()
        );
        for candidate in query.candidates() {
            assert!(std::ptr::eq(candidate.query(), &query));
            let original = raw
                .fragments()
                .get(candidate.index())
                .expect("actual occurrence");
            assert!(std::ptr::eq(candidate.fragment(), original));
            assert!(std::ptr::eq(
                candidate.fragment().source_interval(),
                original.source_interval()
            ));
            assert_eq!(
                candidate.logical_output(),
                EdgeRef::new(input.operation(), original.tag())
            );
        }
        assert_eq!(f.ledger, before);
    }
    assert_eq!(input.direction(), Direction::Reversed);
}
#[test]
fn source_frame_precedes_selected_domain_refusal_and_actual_query_owners_survive() {
    use OffsetCandidateQueryRefusal as R;
    let f = fixture();
    let allowance = f.allowance();
    let input = f.input();
    let bound = input.bind_allowance(&allowance).expect("actual input");
    let raw = OffsetCandidateManifest::new(input.operation(), *input.source(), roles());
    let admitted = raw
        .admit(&bound, OffsetCandidateLimits::new(3))
        .expect("complete metadata");
    for (edge, param, cause) in [
        (
            EdgeRef::new(id(999), LocalTag::FIRST),
            Param::START,
            R::SourceEdge,
        ),
        (input.source().range.edge(), Param::START, R::SourcePosition),
        (input.source().range.edge(), Param::END, R::SourcePosition),
    ] {
        let anchor = EdgeAnchor { edge, param };
        let error = admitted
            .query_source(anchor)
            .expect_err("outside actual source");
        assert!(std::ptr::eq(error.admitted(), &admitted));
        assert_eq!(error.anchor(), &anchor);
        assert_eq!(error.refusal(), cause);
    }
}
#[test]
fn a_current_ledger_alias_is_not_silently_substituted_for_the_original_candidate_source_frame() {
    let mut f = fixture();
    let allowance = f.allowance();
    let input = f.input();
    let bound = input.bind_allowance(&allowance).expect("original metadata");
    let raw = OffsetCandidateManifest::new(input.operation(), *input.source(), roles());
    let admitted = raw
        .admit(&bound, OffsetCandidateLimits::new(3))
        .expect("original candidates");
    let current = f
        .ledger
        .offset(
            &mut DeterministicIdGenerator::from_start(10000),
            f.definition.edge,
            &[OffsetInterval::new(Param::START, Param::END).expect("whole declared interval")],
        )
        .expect("public structural ledger edit");
    let current_edge = *current.first().expect("one declared current role");
    let param = parameter(1, 3);
    assert_eq!(
        f.ledger
            .resolve(f.definition.edge, param)
            .resolved()
            .expect("actual ledger alias")
            .edge(),
        current_edge
    );
    let error = admitted
        .query_source(EdgeAnchor {
            edge: current_edge,
            param,
        })
        .expect_err("no guessed alias or rewritten original source");
    assert_eq!(error.refusal(), OffsetCandidateQueryRefusal::SourceEdge);
    let query = admitted
        .query_source(EdgeAnchor {
            edge: f.definition.edge,
            param,
        })
        .expect("original metadata membership remains inspectable");
    assert_eq!(query.kind(), OffsetCandidateMatchKind::Ambiguous);
    assert_eq!(
        query
            .candidates()
            .map(|candidate| candidate.logical_output().tag().get())
            .collect::<Vec<_>>(),
        vec![9, 2]
    );
    assert!(!f.ledger.is_live(f.definition.edge));
}
