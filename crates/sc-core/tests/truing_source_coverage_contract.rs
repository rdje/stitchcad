//! Exact selected held-domain partition;no refit,no-op,currentness or geometric certificate.
#![allow(clippy::expect_used)]
#[path = "support/truing_domain.rs"]
mod common;
use common::{q, roles::copies::id, roles::with_roles, with_partial};
use sc_core::{construction::*, ontology::*};

fn proposal(ranges: &[EdgeRange]) -> TruingEdgeManifest {
    TruingEdgeManifest::new(
        id(1000),
        [9, 2]
            .into_iter()
            .zip(ranges)
            .map(|(tag, range)| TruingEdgeCandidate::new(LocalTag::new(tag), *range))
            .collect(),
    )
}

#[test]
fn exact_partition_on_both_selected_sides_preserves_original_order_fields_and_all_actual_parents() {
    for side in [SeamSideId::A, SeamSideId::B] {
        with_partial(side, |roles| {
            let original = &roles.copies().source().source_side().range;
            let left = EdgeRange::new(original.edge(), q(1, 4), q(1, 2)).expect("left");
            let right = EdgeRange::new(original.edge(), q(1, 2), q(3, 4)).expect("right");
            let raw = proposal(&[right, left]);
            let snapshot = raw.clone();
            let edges = raw
                .admit(&roles, TruingEdgeManifestLimits::new(2, 4))
                .expect("whole candidate metadata");
            let membership = edges
                .bind_source_domain(TruingSourceDomainLimits::new(2))
                .expect("whole actual membership");
            let limits = TruingSourceCoverageLimits::new(2, 2);
            let covered = membership
                .bind_source_coverage(limits)
                .expect("exact partition despite reversed declaration order");
            assert!(std::ptr::eq(covered.source_domain(), &membership));
            assert!(std::ptr::eq(covered.domain(), original));
            assert_eq!(covered.limits(), limits);
            assert_eq!((limits.candidates(), limits.workspace_entries()), (2, 2));
            assert_eq!(covered.workspace_entries(), 2);
            assert_eq!(covered.source_order().len(), 2);
            assert_eq!(
                covered
                    .source_order()
                    .map(|edge| edge.candidate().index())
                    .collect::<Vec<_>>(),
                vec![1, 0]
            );
            assert_eq!(
                covered
                    .source_order()
                    .rev()
                    .map(|edge| edge.candidate().index())
                    .collect::<Vec<_>>(),
                vec![0, 1]
            );
            assert_eq!(
                membership
                    .edges()
                    .map(|edge| edge.candidate().candidate().tag())
                    .collect::<Vec<_>>(),
                vec![LocalTag::new(9), LocalTag::new(2)]
            );
            for edge in covered.source_order() {
                assert!(std::ptr::eq(edge.context(), &membership));
                assert!(std::ptr::eq(edge.candidate().owner(), &edges));
                assert!(std::ptr::eq(
                    edge.source(),
                    raw.candidates()
                        .get(edge.candidate().index())
                        .expect("original candidate")
                        .source()
                ));
            }
            assert_eq!(raw, snapshot);
            assert!(format!("{covered:?}").contains("CoveredTruingSourceDomain"));
        });
    }
    with_roles(false, |roles| {
        let original = roles.copies().source().source_side().range;
        let raw = proposal(&[original]);
        let edges = raw
            .admit(&roles, TruingEdgeManifestLimits::new(1, 2))
            .expect("metadata");
        let membership = edges
            .bind_source_domain(TruingSourceDomainLimits::new(1))
            .expect("whole membership");
        let covered = membership
            .bind_source_coverage(TruingSourceCoverageLimits::new(1, 1))
            .expect("one exact whole-domain attribution");
        assert_eq!(covered.workspace_entries(), 1);
        assert_eq!(covered.domain().from(), Param::START);
        assert_eq!(covered.domain().to(), Param::END);
    });
}

#[test]
fn whole_candidate_then_workspace_quota_precede_discontinuity_and_errors_keep_the_actual_whole_parent(
) {
    use TruingSourceCoverageRefusal as R;
    with_partial(SeamSideId::A, |roles| {
        let original = roles.copies().source().source_side().range;
        let middle = EdgeRange::new(original.edge(), q(3, 8), q(5, 8)).expect("middle");
        let raw = proposal(&[middle, middle]);
        let snapshot = raw.clone();
        let edges = raw
            .admit(&roles, TruingEdgeManifestLimits::new(2, 4))
            .expect("distinct original roles");
        let membership = edges
            .bind_source_domain(TruingSourceDomainLimits::new(2))
            .expect("both in held domain");
        for (limits, cause) in [
            (
                TruingSourceCoverageLimits::new(1, 0),
                R::Candidates {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                TruingSourceCoverageLimits::new(2, 1),
                R::Workspace {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                TruingSourceCoverageLimits::new(2, 2),
                R::Gap {
                    from: q(1, 4),
                    to: q(3, 8),
                    next: Some(0),
                },
            ),
        ] {
            let error = membership
                .bind_source_coverage(limits)
                .expect_err("full quotas before first exact source-order gap");
            assert!(std::ptr::eq(error.source_domain(), &membership));
            assert_eq!(error.limits(), limits);
            assert_eq!(error.refusal(), cause);
            assert!(error
                .to_string()
                .starts_with("truing selected source coverage:"));
            assert!(format!("{error:?}").contains("TruingSourceCoverageError"));
        }
        assert_eq!(raw, snapshot);
    });
}

#[test]
fn empty_leading_interior_trailing_gaps_and_duplicate_nested_crossing_overlaps_have_exact_original_causes(
) {
    use TruingSourceCoverageRefusal as R;
    with_partial(SeamSideId::B, |roles| {
        let original = roles.copies().source().source_side().range;
        let range = |from, to| EdgeRange::new(original.edge(), from, to).expect("positive range");
        let cases = [
            (
                vec![],
                R::Gap {
                    from: q(1, 4),
                    to: q(3, 4),
                    next: None,
                },
            ),
            (
                vec![range(q(1, 2), q(3, 4))],
                R::Gap {
                    from: q(1, 4),
                    to: q(1, 2),
                    next: Some(0),
                },
            ),
            (
                vec![range(q(1, 4), q(1, 2))],
                R::Gap {
                    from: q(1, 2),
                    to: q(3, 4),
                    next: None,
                },
            ),
            (
                vec![range(q(5, 8), q(3, 4)), range(q(1, 4), q(3, 8))],
                R::Gap {
                    from: q(3, 8),
                    to: q(5, 8),
                    next: Some(0),
                },
            ),
            (
                vec![original, original],
                R::Overlap {
                    previous: 0,
                    index: 1,
                    from: q(1, 4),
                    to: q(3, 4),
                },
            ),
            (
                vec![range(q(1, 4), q(1, 2)), original],
                R::Overlap {
                    previous: 0,
                    index: 1,
                    from: q(1, 4),
                    to: q(1, 2),
                },
            ),
            (
                vec![range(q(1, 2), q(3, 4)), range(q(1, 4), q(5, 8))],
                R::Overlap {
                    previous: 1,
                    index: 0,
                    from: q(1, 2),
                    to: q(5, 8),
                },
            ),
        ];
        for (ranges, cause) in cases {
            let n = ranges.len();
            let raw = proposal(&ranges);
            let edges = raw
                .admit(&roles, TruingEdgeManifestLimits::new(n, n * 2))
                .expect("metadata");
            let membership = edges
                .bind_source_domain(TruingSourceDomainLimits::new(n))
                .expect("membership");
            assert_eq!(
                membership
                    .bind_source_coverage(TruingSourceCoverageLimits::new(n, n))
                    .expect_err("first exact discontinuity;no prefix result")
                    .refusal(),
                cause
            );
        }
        // Both comparisons distinguish fractions that round to the same binary floating value.
        let below = q(4_611_686_018_427_387_903, 9_223_372_036_854_775_807);
        let above = q(4_611_686_018_427_387_904, 9_223_372_036_854_775_807);
        for (end, start, cause) in [
            (
                below,
                above,
                R::Gap {
                    from: below,
                    to: above,
                    next: Some(1),
                },
            ),
            (
                above,
                below,
                R::Overlap {
                    previous: 0,
                    index: 1,
                    from: below,
                    to: above,
                },
            ),
        ] {
            let raw = proposal(&[range(q(1, 4), end), range(start, q(3, 4))]);
            let edges = raw
                .admit(&roles, TruingEdgeManifestLimits::new(2, 4))
                .expect("metadata");
            let membership = edges
                .bind_source_domain(TruingSourceDomainLimits::new(2))
                .expect("membership");
            assert_eq!(
                membership
                    .bind_source_coverage(TruingSourceCoverageLimits::new(2, 2))
                    .expect_err("positive sub-float discontinuity cannot disappear")
                    .refusal(),
                cause
            );
        }
    });
}

#[test]
fn bounded_integer_cell_occupancy_oracle_checks_every_ordered_pair_without_sharing_the_partition_algorithm(
) {
    for side in [SeamSideId::A, SeamSideId::B] {
        with_partial(side, |roles| {
            let original = roles.copies().source().source_side().range;
            let intervals: Vec<_> = (2..6)
                .flat_map(|from| ((from + 1)..=6).map(move |to| (from, to)))
                .collect();
            for first in &intervals {
                for second in &intervals {
                    let ranges: Vec<_> = [first, second]
                        .into_iter()
                        .map(|(from, to)| {
                            EdgeRange::new(original.edge(), q(*from, 8), q(*to, 8))
                                .expect("grid interval")
                        })
                        .collect();
                    // Count attribution for each exact open grid cell,without sorting intervals.
                    let expected = (2..6).all(|cell| {
                        [first, second]
                            .into_iter()
                            .filter(|(from, to)| *from <= cell && cell < *to)
                            .count()
                            == 1
                    });
                    let raw = proposal(&ranges);
                    let edges = raw
                        .admit(&roles, TruingEdgeManifestLimits::new(2, 4))
                        .expect("metadata");
                    let membership = edges
                        .bind_source_domain(TruingSourceDomainLimits::new(2))
                        .expect("membership");
                    assert_eq!(
                        membership
                            .bind_source_coverage(TruingSourceCoverageLimits::new(2, 2))
                            .is_ok(),
                        expected,
                        "{side:?} original candidates {first:?}/{second:?}"
                    );
                }
            }
        });
    }
}
