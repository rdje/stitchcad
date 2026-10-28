//! Actual admitted inverse maps preserve creation frames,wide coordinates and all numeric candidates.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use core::num::{NonZeroU64, NonZeroUsize};
use sc_core::ontology::{
    DeterministicIdGenerator, Direction, EdgeRef, IdentityLedger, OriginalPointAdmissionError,
    OriginalPointLimits, OriginalPointProjectionIssue, Param, Rational, UniquePointLimits,
    UniquePointOutcome,
};
use sc_units::{Length, UnitError};
fn t(n: i64, d: i64) -> Param {
    Param::new(Rational::new(n, d).unwrap()).unwrap()
}
fn limits(candidates: usize) -> OriginalPointLimits {
    OriginalPointLimits::new(
        UniquePointLimits::new(
            NonZeroUsize::new(32).unwrap(),
            NonZeroUsize::new(32).unwrap(),
        ),
        NonZeroUsize::new(candidates).unwrap(),
        NonZeroU64::new(10000).unwrap(),
    )
}
fn fixture(count: usize) -> (IdentityLedger, DeterministicIdGenerator, Vec<EdgeRef>) {
    let mut ledger = IdentityLedger::new();
    let mut ids = DeterministicIdGenerator::new();
    let edges = ledger.declare_edges(&mut ids, count).unwrap();
    (ledger, ids, edges)
}
fn one() -> Length {
    Length::from_micrometres(1).unwrap()
}

#[test]
fn split_fragments_project_exactly_without_forcing_original_coordinates_into_param() {
    let (mut ledger, mut ids, edges) = fixture(1);
    let source = *edges.first().unwrap();
    let fragments = ledger.split(&mut ids, source, t(1, 2)).unwrap();
    let wide = ledger
        .admit_original_point(fragments.first(), t(1, i64::MAX), source, limits(1))
        .unwrap()
        .project()
        .unwrap();
    let coordinate = wide.unique().unwrap().coordinate();
    assert_eq!(
        coordinate.as_u128_ratio().unwrap(),
        (1, 2 * u128::try_from(i64::MAX).unwrap())
    );
    assert_eq!(coordinate.component_bit_counts(), (1, 64));
    assert_eq!(
        coordinate.as_param(),
        Err(UnitError::Overflow {
            operation: "OriginalCoordinate::as_param"
        })
    );
    assert_eq!(wide.unique().unwrap().direction(), Direction::Original);
    let second = ledger
        .admit_original_point(fragments.second(), t(1, 2), source, limits(1))
        .unwrap()
        .project()
        .unwrap();
    assert_eq!(
        second.unique().unwrap().coordinate().as_param().unwrap(),
        t(3, 4)
    );
}

#[test]
fn projection_stops_after_actual_source_creation_and_undoes_later_same_identity_reversals() {
    let (mut ledger, mut ids, edges) = fixture(1);
    let root = *edges.first().unwrap();
    let source = ledger.split(&mut ids, root, t(1, 2)).unwrap().first();
    let creation = ledger.journal().last().unwrap().operation();
    ledger.reverse(&mut ids, source).unwrap();
    let token = ledger
        .admit_original_point(source, t(1, 4), source, limits(1))
        .unwrap();
    assert_eq!(token.current().param(), t(3, 4));
    let projection = token.project().unwrap();
    assert_eq!(projection.admission().source_creation(), creation);
    assert_eq!(
        projection
            .unique()
            .unwrap()
            .coordinate()
            .as_param()
            .unwrap(),
        t(1, 4)
    );
    assert_eq!(
        projection.unique().unwrap().direction(),
        Direction::Reversed
    );
}

#[test]
fn unequal_declared_merge_lengths_project_each_side_and_the_exact_join() {
    let (mut ledger, mut ids, edges) = fixture(2);
    let first = *edges.first().unwrap();
    let second = *edges.get(1).unwrap();
    let merged = ledger
        .merge(
            &mut ids,
            first,
            Length::from_micrometres(3).unwrap(),
            second,
            Length::from_micrometres(7).unwrap(),
        )
        .unwrap();
    for (at, goal, expected) in [
        (t(3, 20), first, t(1, 2)),
        (t(13, 20), second, t(1, 2)),
        (t(3, 10), first, Param::END),
        (t(3, 10), second, Param::START),
    ] {
        let projection = ledger
            .admit_original_point(merged, at, goal, limits(2))
            .unwrap()
            .project()
            .unwrap();
        assert_eq!(
            projection
                .unique()
                .unwrap()
                .coordinate()
                .as_param()
                .unwrap(),
            expected
        );
        assert_eq!(
            ledger.resolve(goal, expected).resolved().unwrap().param(),
            at
        );
    }
    for (at, goal) in [(t(3, 20), second), (t(13, 20), first)] {
        let error = ledger
            .admit_original_point(merged, at, goal, limits(1))
            .unwrap()
            .project()
            .unwrap_err();
        assert_eq!(error.issue(), &OriginalPointProjectionIssue::NoOrigin);
        assert_eq!(
            (
                error.source_edge(),
                error.reference(),
                error.param(),
                error.current().param()
            ),
            (goal, merged, at, at)
        );
    }
}

#[test]
fn reversed_merge_join_retains_distinct_origins_directions_and_exact_branch_quota() {
    let (mut ledger, mut ids, edges) = fixture(1);
    let source = *edges.first().unwrap();
    let fragments = ledger.split(&mut ids, source, t(1, 2)).unwrap();
    ledger.reverse(&mut ids, fragments.second()).unwrap();
    let merged = ledger
        .merge(
            &mut ids,
            fragments.first(),
            one(),
            fragments.second(),
            one(),
        )
        .unwrap();
    let operation = ledger.journal().last().unwrap().operation();
    let before = ledger.clone();
    let error = ledger
        .admit_original_point(merged, t(1, 2), source, limits(1))
        .unwrap()
        .project()
        .unwrap_err();
    assert_eq!(
        error.issue(),
        &OriginalPointProjectionIssue::Candidates {
            operation,
            retained: 1,
            requested: 1,
            limit: 1
        }
    );
    assert_eq!(
        (error.source_edge(), error.reference(), error.param()),
        (source, merged, t(1, 2))
    );
    assert!(std::error::Error::source(&error).is_none());
    let projection = ledger
        .admit_original_point(merged, t(1, 2), source, limits(2))
        .unwrap()
        .project()
        .unwrap();
    assert!(projection.unique().is_none());
    let candidates = projection
        .candidates()
        .iter()
        .map(|candidate| {
            (
                candidate.coordinate().as_param().unwrap(),
                candidate.direction(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        candidates,
        vec![
            (t(1, 2), Direction::Original),
            (Param::END, Direction::Reversed)
        ]
    );
    assert_eq!(
        projection.admission().current().direction(),
        Direction::Original
    );
    assert_eq!(ledger, before);
}

#[test]
fn equal_numeric_states_coalesce_without_canceling_old_reference_choices_or_peak_work() {
    let (mut ledger, mut ids, edges) = fixture(4);
    let source = *edges.first().unwrap();
    let half = t(1, 2);
    let fragments = ledger.split(&mut ids, source, half).unwrap();
    let mut merged = ledger
        .merge(
            &mut ids,
            fragments.first(),
            one(),
            fragments.second(),
            one(),
        )
        .unwrap();
    for _ in 1..8 {
        let fragments = ledger.split(&mut ids, merged, half).unwrap();
        merged = ledger
            .merge(
                &mut ids,
                fragments.first(),
                one(),
                fragments.second(),
                one(),
            )
            .unwrap();
    }
    assert!(ledger
        .admit_original_point(merged, half, source, limits(1))
        .unwrap()
        .project()
        .is_err());
    let projection = ledger
        .admit_original_point(merged, half, source, limits(2))
        .unwrap()
        .project()
        .unwrap();
    assert_eq!(projection.candidates().len(), 1);
    assert_eq!(
        projection
            .unique()
            .unwrap()
            .coordinate()
            .as_param()
            .unwrap(),
        half
    );
    let old = ledger
        .admit_original_point(source, half, source, limits(2))
        .unwrap_err();
    let OriginalPointAdmissionError::Current { resolution, .. } = old else {
        panic!("actual old choice")
    };
    assert!(matches!(
        resolution.outcome(),
        UniquePointOutcome::SplitChoice { .. }
    ));
}

#[test]
fn unrelated_real_source_has_no_origin_and_debug_does_not_dump_journal_or_integers() {
    let (mut ledger, mut ids, edges) = fixture(2);
    let held = *edges.first().unwrap();
    let goal = *edges.get(1).unwrap();
    let error = ledger
        .admit_original_point(held, t(1, 4), goal, limits(1))
        .unwrap()
        .project()
        .unwrap_err();
    assert_eq!(error.issue(), &OriginalPointProjectionIssue::NoOrigin);
    assert_eq!(
        (error.source_edge(), error.reference(), error.param()),
        (goal, held, t(1, 4))
    );
    let projection = ledger
        .admit_original_point(held, t(1, 4), held, limits(1))
        .unwrap()
        .project()
        .unwrap();
    let summary = format!("{projection:?}");
    assert!(!summary.contains("Declared") && !summary.contains("numerator:"));
    let point = format!("{:?}", projection.unique().unwrap().coordinate());
    assert!(point.contains("numerator_bits") && point.contains("denominator_bits"));
    drop(projection);
    let late = *ledger.declare_edges(&mut ids, 1).unwrap().first().unwrap();
    let unrelated = ledger
        .admit_original_point(late, t(1, 3), goal, limits(1))
        .unwrap()
        .project()
        .unwrap_err();
    assert_eq!(unrelated.issue(), &OriginalPointProjectionIssue::NoOrigin);
    assert_eq!(unrelated.reference(), late);
}
