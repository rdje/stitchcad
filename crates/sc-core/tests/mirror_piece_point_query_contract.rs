//! Exact original-source eligibility preserves ambiguity and full population limits.
#![allow(clippy::expect_used)]
#[path = "support/mirror_points.rs"]
mod common;
use common::with_points;
use sc_core::{construction::*, ontology::*};

fn source(creator: u128, tag: u32) -> PointRef {
    PointRef::new(EntityId::from_bits(creator), LocalTag::new(tag))
}
fn raw(first: PointRef, second: PointRef) -> MirrorPiecePointManifest {
    MirrorPiecePointManifest::new(
        EntityId::from_bits(10),
        vec![
            MirrorPiecePointCandidate::new(LocalTag::new(9), first),
            MirrorPiecePointCandidate::new(LocalTag::new(2), second),
        ],
    )
}

#[test]
fn unique_source_keeps_authentic_parents_and_original_fields_not_output_lookup() {
    with_points(|roles| {
        let raw = raw(source(90, 7), source(91, 3));
        let original = raw.clone();
        let points = raw
            .admit(&roles, MirrorPiecePointManifestLimits::new(2, 4))
            .expect("whole proposal");
        let query = MirrorPiecePointQuery::new(source(90, 7));
        let limits = MirrorPiecePointQueryLimits::new(2, 1);
        let result = points
            .query_source(&query, limits)
            .expect("one exact source");
        assert!(std::ptr::eq(result.points(), &points));
        assert!(std::ptr::eq(result.query(), &query));
        assert_eq!(result.limits(), limits);
        assert_eq!((limits.candidates(), limits.matches()), (2, 1));
        assert_eq!(result.kind(), MirrorPiecePointMatchKind::Unique);
        assert_eq!(result.match_count(), 1);
        let unique = result.unique().expect("exactly one candidate");
        assert!(std::ptr::eq(unique.owner(), &points));
        assert!(std::ptr::eq(
            unique.source(),
            raw.candidates()
                .first()
                .expect("original candidate")
                .source()
        ));
        assert_eq!(*unique.source(), source(90, 7));
        assert_eq!(unique.logical_output(), source(10, 9));
        assert_eq!(
            result.matches().map(|p| p.index()).collect::<Vec<_>>(),
            vec![0]
        );
        for query in [
            MirrorPiecePointQuery::new(source(90, 8)),
            MirrorPiecePointQuery::new(source(10, 9)),
        ] {
            let gap = points
                .query_source(&query, MirrorPiecePointQueryLimits::new(2, 0))
                .expect("exact original identity only");
            assert_eq!(gap.kind(), MirrorPiecePointMatchKind::Gap);
            assert_eq!(gap.match_count(), 0);
            assert!(gap.unique().is_none());
            assert_eq!(gap.matches().count(), 0);
        }
        assert!(points.point(LocalTag::new(9)).is_some());
        assert_eq!(raw, original);
    });
}

#[test]
fn ambiguous_source_retains_every_role_in_proposal_order_without_choosing_one() {
    with_points(|roles| {
        let raw = raw(source(90, 7), source(90, 7));
        let points = raw
            .admit(&roles, MirrorPiecePointManifestLimits::new(2, 4))
            .expect("distinct roles");
        let query = MirrorPiecePointQuery::new(source(90, 7));
        let result = points
            .query_source(&query, MirrorPiecePointQueryLimits::new(2, 2))
            .expect("all matches fit");
        assert_eq!(result.kind(), MirrorPiecePointMatchKind::Ambiguous);
        assert_eq!(result.match_count(), 2);
        assert!(result.unique().is_none());
        assert_eq!(
            result
                .matches()
                .map(|p| p.candidate().tag().get())
                .collect::<Vec<_>>(),
            vec![9, 2]
        );
        assert_eq!(
            result
                .matches()
                .rev()
                .map(|p| p.candidate().tag().get())
                .collect::<Vec<_>>(),
            vec![2, 9]
        );
        for candidate in result.matches() {
            assert!(std::ptr::eq(candidate.owner(), &points));
            assert!(std::ptr::eq(candidate.role().catalog(), roles.catalog()));
            assert_eq!(*candidate.source(), *query.source());
        }
    });
}

#[test]
fn complete_candidate_count_precedes_scan_and_complete_match_count_refuses_no_prefix() {
    use MirrorPiecePointQueryRefusal as R;
    with_points(|roles| {
        let raw = raw(source(90, 7), source(90, 7));
        let points = raw
            .admit(&roles, MirrorPiecePointManifestLimits::new(2, 4))
            .expect("whole proposal");
        for (query, limits, refusal) in [
            (
                MirrorPiecePointQuery::new(source(92, 1)),
                MirrorPiecePointQueryLimits::new(1, 0),
                R::Candidates {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                MirrorPiecePointQuery::new(source(90, 7)),
                MirrorPiecePointQueryLimits::new(1, 0),
                R::Candidates {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                MirrorPiecePointQuery::new(source(90, 7)),
                MirrorPiecePointQueryLimits::new(2, 1),
                R::Matches {
                    required: 2,
                    allowed: 1,
                },
            ),
        ] {
            let error = points
                .query_source(&query, limits)
                .expect_err("full refusal, no truncated result");
            assert!(std::ptr::eq(error.points(), &points));
            assert!(std::ptr::eq(error.query(), &query));
            assert_eq!(error.limits(), limits);
            assert_eq!(error.refusal(), refusal);
            assert!(format!("{error}").contains("mirror original-point eligibility"));
        }
    });
}

#[test]
fn empty_candidate_query_is_a_metadata_gap_without_retiring_roles() {
    with_points(|roles| {
        let raw = MirrorPiecePointManifest::new(EntityId::from_bits(10), vec![]);
        let points = raw
            .admit(&roles, MirrorPiecePointManifestLimits::new(0, 0))
            .expect("explicit empty metadata");
        let query = MirrorPiecePointQuery::new(source(90, 7));
        let gap = points
            .query_source(&query, MirrorPiecePointQueryLimits::new(0, 0))
            .expect("zero actual work/matches");
        assert_eq!(gap.kind(), MirrorPiecePointMatchKind::Gap);
        assert_eq!(gap.match_count(), 0);
        assert!(gap.unique().is_none());
        assert_eq!(gap.matches().count(), 0);
        assert!(roles.catalog().is_active(LocalTag::new(9)));
        assert!(roles.catalog().definition(LocalTag::new(7)).is_some());
    });
}
