//! Admitted complete interval coverage retains repairs without constructing endpoint choice trees.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
use core::num::NonZeroUsize;
use sc_core::ontology::{
    DeterministicIdGenerator, Direction, EdgeRange, EdgeRef, EntityId, IdentityLedger, LocalTag,
    OffsetInterval, OrphaningEdit, Param, PointJournalAdmissionIssue, PointJournalCollection,
    RangeCoverage, RangeCoverageIssue, RangeCoverageLimits, RangeIssue, RangePortion, Rational,
    Resolution, UniquePointLimits,
};
use sc_units::{Length, UnitError};

fn t(n: i64, d: i64) -> Param {
    Param::new(Rational::new(n, d).unwrap()).unwrap()
}
fn limits(journal: usize, nested: usize, portions: usize) -> RangeCoverageLimits {
    RangeCoverageLimits::new(
        UniquePointLimits::new(
            NonZeroUsize::new(journal).unwrap(),
            NonZeroUsize::new(nested).unwrap(),
        ),
        NonZeroUsize::new(portions).unwrap(),
    )
}
fn fixture() -> (IdentityLedger, DeterministicIdGenerator, EdgeRef) {
    let mut ledger = IdentityLedger::new();
    let mut ids = DeterministicIdGenerator::new();
    let edge = ledger.declare_edges(&mut ids, 1).unwrap().remove(0);
    (ledger, ids, edge)
}
fn coverage(ledger: &IdentityLedger, held: EdgeRange, cap: usize) -> RangeCoverage {
    let result = ledger
        .resolve_range_coverage(held, limits(64, 64, cap))
        .unwrap();
    let legacy = ledger.resolve_range(held);
    assert_eq!(result.held(), held);
    assert_eq!(result.portions(), legacy.portions());
    assert_eq!(result.has_full_coverage(), legacy.has_full_coverage());
    result
}

#[test]
fn complete_input_admission_precedes_unknown_source_and_portion_allocation() {
    let (mut ledger, mut ids, edge) = fixture();
    ledger.declare_edges(&mut ids, 3).unwrap();
    let held = EdgeRange::whole(EdgeRef::new(
        EntityId::from_bits(u128::MAX),
        LocalTag::FIRST,
    ));
    let error = ledger
        .resolve_range_coverage(held, limits(1, 1, 1))
        .unwrap_err();
    assert_eq!(error.held(), held);
    assert_eq!(
        error.issue(),
        &RangeCoverageIssue::Input(PointJournalAdmissionIssue::JournalEntries {
            required: 2,
            limit: 1,
        })
    );
    let error = ledger
        .resolve_range_coverage(held, limits(2, 3, 1))
        .unwrap_err();
    assert_eq!(
        error.issue(),
        &RangeCoverageIssue::Input(PointJournalAdmissionIssue::NestedItems {
            operation: ledger.journal().last().unwrap().operation(),
            collection: PointJournalCollection::DeclaredEdges,
            consumed: 1,
            requested: 3,
            limit: 3,
        })
    );
    let absent = ledger
        .resolve_range_coverage(held, limits(2, 4, 1))
        .unwrap();
    assert!(!absent.has_full_coverage());
    let [RangePortion::Unresolved(task)] = absent.portions() else {
        panic!("one unknown source")
    };
    assert_eq!(task.held(), held);
    assert_eq!(task.issue(), RangeIssue::UnknownSource);
    let selected = limits(2, 4, 2);
    assert_eq!(selected.current().journal_entries().get(), 2);
    assert_eq!(selected.current().nested_items().get(), 4);
    assert_eq!(selected.portions().get(), 2);
    assert!(ledger
        .resolve_range_coverage(EdgeRange::whole(edge), selected)
        .unwrap()
        .has_full_coverage());
}

#[test]
fn peak_budget_includes_active_input_and_never_returns_partial_coverage() {
    let (mut ledger, mut ids, edge) = fixture();
    let held = EdgeRange::whole(edge);
    let error = ledger
        .resolve_range_coverage(held, limits(1, 1, 1))
        .unwrap_err();
    assert_eq!(
        error.issue(),
        &RangeCoverageIssue::Portions {
            operation: ledger.journal().first().unwrap().operation(),
            retained: 1,
            requested: 1,
            limit: 1,
        }
    );
    assert_eq!(coverage(&ledger, held, 2).portions().len(), 1);
    ledger.split(&mut ids, edge, t(1, 2)).unwrap();
    let error = ledger
        .resolve_range_coverage(held, limits(2, 1, 2))
        .unwrap_err();
    assert_eq!(error.held(), held);
    assert_eq!(
        error.issue(),
        &RangeCoverageIssue::Portions {
            operation: ledger.journal().last().unwrap().operation(),
            retained: 2,
            requested: 1,
            limit: 2,
        }
    );
    assert_eq!(coverage(&ledger, held, 3).portions().len(), 2);
}

#[test]
fn eight_cycles_keep_interval_coverage_bounded_while_endpoint_choices_remain() {
    let (mut ledger, mut ids, edge) = fixture();
    let mut current = edge;
    let one = Length::from_micrometres(1).unwrap();
    for _ in 0..8 {
        let fragments = ledger.split(&mut ids, current, t(1, 2)).unwrap();
        current = ledger
            .merge(&mut ids, fragments.first(), one, fragments.second(), one)
            .unwrap();
    }
    let held = EdgeRange::new(edge, t(1, 2), Param::END).unwrap();
    let before = ledger.clone();
    let result = coverage(&ledger, held, 2);
    let [RangePortion::Resolved(part)] = result.portions() else {
        panic!("one upper interval")
    };
    assert_eq!(
        part.range(),
        EdgeRange::new(current, t(1, 2), Param::END).unwrap()
    );
    assert_eq!(part.direction(), Direction::Original);
    fn leaves(resolution: &Resolution) -> usize {
        match resolution {
            Resolution::Resolved(_) => 1,
            Resolution::SplitPoint { first, second } => leaves(first) + leaves(second),
            Resolution::Unresolved(_) => panic!("unchanged split choices"),
        }
    }
    assert_eq!(leaves(ledger.resolve_range(held).start()), 256);
    let whole = coverage(&ledger, EdgeRange::whole(edge), 3);
    assert_eq!(whole.portions().len(), 2);
    assert!(whole.has_full_coverage());
    assert_eq!(ledger, before);
    let debug = format!("{whole:?}");
    assert!(debug.contains("portions: 2"));
    assert!(!debug.contains("ResolvedRange"));
}

#[test]
fn reversed_offsets_preserve_all_gaps_and_deletion_causes_in_held_order() {
    let (mut ledger, mut ids, edge) = fixture();
    let held = EdgeRange::whole(edge);
    ledger.reverse(&mut ids, edge).unwrap();
    let fragments = ledger
        .offset(
            &mut ids,
            edge,
            &[
                OffsetInterval::new(t(1, 4), t(1, 2)).unwrap(),
                OffsetInterval::new(t(3, 4), Param::END).unwrap(),
            ],
        )
        .unwrap();
    let result = coverage(&ledger, held, 5);
    assert!(!result.has_full_coverage());
    let [RangePortion::Resolved(last), RangePortion::Unresolved(middle), RangePortion::Resolved(first), RangePortion::Unresolved(low)] =
        result.portions()
    else {
        panic!("complete reversed coverage")
    };
    assert_eq!(last.range().edge(), *fragments.last().unwrap());
    assert_eq!(first.range().edge(), *fragments.first().unwrap());
    assert_eq!(last.direction(), Direction::Reversed);
    assert_eq!(
        middle.affected().range(),
        EdgeRange::new(edge, t(1, 2), t(3, 4)).unwrap()
    );
    assert_eq!(middle.affected().direction(), Direction::Reversed);
    assert_eq!(
        low.affected().range(),
        EdgeRange::new(edge, Param::START, t(1, 4)).unwrap()
    );
    let deleted = *fragments.first().unwrap();
    ledger.delete(&mut ids, deleted).unwrap();
    let operation = ledger.journal().last().unwrap().operation();
    let result = coverage(&ledger, held, 5);
    assert!(result.portions().iter().any(|part| matches!(part,
        RangePortion::Unresolved(task) if task.issue() == RangeIssue::Orphaned(
            OrphaningEdit::Deleted { operation, edge: deleted }
        ) && task.held() == held)));
}

#[test]
fn late_numeric_failure_discards_earlier_emissions_from_that_input() {
    let (mut ledger, mut ids, edge) = fixture();
    let held = EdgeRange::new(edge, Param::START, t(i64::MAX - 1, i64::MAX)).unwrap();
    ledger
        .offset(
            &mut ids,
            edge,
            &[
                OffsetInterval::new(Param::START, t(1, 4)).unwrap(),
                OffsetInterval::new(t(1, 3), Param::END).unwrap(),
            ],
        )
        .unwrap();
    assert!(ledger.resolve(edge, t(1, 8)).resolved().is_some());
    let operation = ledger.journal().last().unwrap().operation();
    let result = coverage(&ledger, held, 4);
    let [RangePortion::Unresolved(task)] = result.portions() else {
        panic!("discard valid first fragment and gap when later mapping fails")
    };
    assert_eq!(task.held(), held);
    assert_eq!(task.affected().range(), held);
    assert_eq!(
        task.issue(),
        RangeIssue::Orphaned(OrphaningEdit::RecomputationFailed {
            operation,
            edge,
            cause: UnitError::Overflow {
                operation: "Rational::checked_relative_progress"
            },
        })
    );
}

#[test]
fn reversed_split_keeps_original_first_failure_precedence() {
    let (mut ledger, mut ids, edge) = fixture();
    let held = EdgeRange::new(edge, t(1, i64::MAX), t(i64::MAX - 1, i64::MAX)).unwrap();
    ledger.reverse(&mut ids, edge).unwrap();
    ledger.split(&mut ids, edge, t(2, 5)).unwrap();
    let result = coverage(&ledger, held, 3);
    let [RangePortion::Unresolved(task)] = result.portions() else {
        panic!("one complete repair")
    };
    assert!(matches!(
        task.issue(),
        RangeIssue::Orphaned(OrphaningEdit::RecomputationFailed {
            cause: UnitError::Overflow {
                operation: "Rational::checked_div"
            },
            ..
        })
    ));
}
