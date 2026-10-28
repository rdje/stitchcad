//! Bounded current uniqueness uses actual journal data and preserves required choices/repair causes.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

use core::num::NonZeroUsize;
use sc_core::ontology::{
    DeterministicIdGenerator, Direction, EdgeRef, EntityId, IdentityLedger, LocalTag,
    OffsetInterval, Param, PointJournalCollection, Rational, Resolution, SplitSide,
    UniquePointLimits, UniquePointOutcome, UniquePointResolution,
};
use sc_units::Length;

fn t(n: i64, d: i64) -> Param {
    Param::new(Rational::new(n, d).unwrap()).unwrap()
}
fn limits(journal: usize, nested: usize) -> UniquePointLimits {
    UniquePointLimits::new(
        NonZeroUsize::new(journal).unwrap(),
        NonZeroUsize::new(nested).unwrap(),
    )
}
fn fixture(count: usize) -> (IdentityLedger, DeterministicIdGenerator, Vec<EdgeRef>) {
    let mut ledger = IdentityLedger::new();
    let mut ids = DeterministicIdGenerator::new();
    let edges = ledger.declare_edges(&mut ids, count).unwrap();
    (ledger, ids, edges)
}

#[test]
fn limits_and_fixed_outcomes_are_copy_and_have_explicit_positive_inputs() {
    fn copy_only<T: Copy>() {}
    copy_only::<UniquePointLimits>();
    copy_only::<UniquePointOutcome>();
    copy_only::<UniquePointResolution>();
    copy_only::<PointJournalCollection>();
    let selected = limits(7, 13);
    assert_eq!(selected.journal_entries().get(), 7);
    assert_eq!(selected.nested_items().get(), 13);
}

#[test]
fn exact_unique_chain_retains_original_holder_and_relative_direction() {
    let (mut ledger, mut ids, edges) = fixture(2);
    let source = *edges.first().unwrap();
    let other = *edges.get(1).unwrap();
    let fragment = ledger.split(&mut ids, source, t(1, 3)).unwrap().second();
    ledger.reverse(&mut ids, fragment).unwrap();
    let merged = ledger
        .merge(
            &mut ids,
            other,
            Length::from_micrometres(3).unwrap(),
            fragment,
            Length::from_micrometres(7).unwrap(),
        )
        .unwrap();
    let offset = *ledger
        .offset(
            &mut ids,
            merged,
            &[OffsetInterval::new(t(1, 2), Param::END).unwrap()],
        )
        .unwrap()
        .first()
        .unwrap();
    let before = ledger.clone();
    let result = ledger.resolve_unique_point(source, t(1, 2), limits(5, 3));
    assert_eq!((result.reference(), result.param()), (source, t(1, 2)));
    assert_eq!(
        result.resolved(),
        ledger.resolve(source, t(1, 2)).resolved()
    );
    let point = result.resolved().unwrap();
    assert_eq!(
        (point.edge(), point.param(), point.direction()),
        (offset, t(13, 20), Direction::Reversed)
    );
    // An actually created later source is known;the full journal does not turn it into pass-through fiction.
    assert_eq!(
        ledger
            .resolve_unique_point(offset, t(1, 5), limits(5, 3))
            .resolved()
            .unwrap()
            .param(),
        t(1, 5)
    );
    assert_eq!(ledger, before);
}

#[test]
fn complete_admission_precedes_lookup_and_preserves_actual_collection_refusal() {
    let (mut ledger, mut ids, edges) = fixture(3);
    let source = *edges.first().unwrap();
    ledger
        .offset(
            &mut ids,
            source,
            &[
                OffsetInterval::new(Param::START, t(1, 2)).unwrap(),
                OffsetInterval::new(t(1, 2), Param::END).unwrap(),
            ],
        )
        .unwrap();
    let operation = ledger.journal().last().unwrap().operation();
    let unknown = EdgeRef::new(EntityId::from_bits(u128::MAX), LocalTag::FIRST);
    let before = ledger.clone();
    let journal = ledger.resolve_unique_point(unknown, t(1, 4), limits(1, 5));
    assert_eq!(
        journal.outcome(),
        &UniquePointOutcome::JournalEntries {
            required: 2,
            limit: 1
        }
    );
    let nested = ledger.resolve_unique_point(unknown, t(1, 4), limits(2, 4));
    assert_eq!(
        nested.outcome(),
        &UniquePointOutcome::NestedItems {
            operation,
            collection: PointJournalCollection::OffsetFragments,
            consumed: 3,
            requested: 2,
            limit: 4,
        }
    );
    assert_eq!((nested.reference(), nested.param()), (unknown, t(1, 4)));
    assert_eq!(
        ledger
            .resolve_unique_point(unknown, t(1, 4), limits(2, 5))
            .outcome(),
        &UniquePointOutcome::UnknownSource
    );
    assert!(ledger
        .resolve_unique_point(source, t(1, 4), limits(2, 5))
        .resolved()
        .is_some());
    assert_eq!(ledger, before);
}

#[test]
fn unknown_source_is_explicit_without_changing_legacy_point_pass_through() {
    let ledger = IdentityLedger::new();
    let edge = EdgeRef::new(EntityId::from_bits(99), LocalTag::FIRST);
    assert!(ledger.resolve(edge, t(1, 7)).resolved().is_some());
    let result = ledger.resolve_unique_point(edge, t(1, 7), limits(1, 1));
    assert_eq!(result.outcome(), &UniquePointOutcome::UnknownSource);
    assert!(result.resolved().is_none());
    assert_eq!((result.reference(), result.param()), (edge, t(1, 7)));
}

#[test]
fn split_choice_stops_before_growth_and_complete_later_inputs_are_still_admitted() {
    let (mut ledger, mut ids, edges) = fixture(1);
    let source = *edges.first().unwrap();
    let holder = EntityId::from_bits(9999);
    ledger.register(holder, source, t(1, 2)).unwrap();
    let first_split = ledger.split(&mut ids, source, t(1, 2)).unwrap();
    let first_operation = ledger.journal().last().unwrap().operation();
    let one = Length::from_micrometres(1).unwrap();
    let mut current = ledger
        .merge(
            &mut ids,
            first_split.first(),
            one,
            first_split.second(),
            one,
        )
        .unwrap();
    for _ in 1..8 {
        let fragments = ledger.split(&mut ids, current, t(1, 2)).unwrap();
        current = ledger
            .merge(&mut ids, fragments.first(), one, fragments.second(), one)
            .unwrap();
    }
    ledger
        .state_split_choice(holder, source, t(1, 2), SplitSide::First)
        .unwrap();
    fn leaves(result: &Resolution) -> usize {
        match result {
            Resolution::Resolved(_) => 1,
            Resolution::SplitPoint { first, second } => leaves(first) + leaves(second),
            Resolution::Unresolved(_) => panic!("frozen complete split/merge source"),
        }
    }
    assert_eq!(leaves(&ledger.resolve(source, t(1, 2))), 256);
    // Even an unrelated late declaration must be charged before the first split choice is returned.
    ledger.declare_edges(&mut ids, 2).unwrap();
    let last = ledger.journal().last().unwrap().operation();
    let before = ledger.clone();
    assert_eq!(
        ledger
            .resolve_unique_point(source, t(1, 2), limits(18, 2))
            .outcome(),
        &UniquePointOutcome::NestedItems {
            operation: last,
            collection: PointJournalCollection::DeclaredEdges,
            consumed: 1,
            requested: 2,
            limit: 2
        }
    );
    let result = ledger.resolve_unique_point(source, t(1, 2), limits(18, 3));
    assert_eq!(
        result.outcome(),
        &UniquePointOutcome::SplitChoice {
            operation: first_operation,
            source,
            at: t(1, 2),
            first: first_split.first(),
            second: first_split.second(),
            direction: Direction::Original,
        }
    );
    assert!(result.resolved().is_none());
    assert_eq!(ledger, before);
}

#[test]
fn concrete_delete_gap_and_offset_ambiguity_evidence_agrees_with_the_full_fold() {
    for kind in 0..3 {
        let (mut ledger, mut ids, edges) = fixture(1);
        let source = *edges.first().unwrap();
        let held = t(1, 2);
        match kind {
            0 => {
                ledger.delete(&mut ids, source).unwrap();
            }
            1 => {
                ledger
                    .offset(
                        &mut ids,
                        source,
                        &[
                            OffsetInterval::new(Param::START, t(1, 3)).unwrap(),
                            OffsetInterval::new(t(2, 3), Param::END).unwrap(),
                        ],
                    )
                    .unwrap();
            }
            _ => {
                ledger
                    .offset(
                        &mut ids,
                        source,
                        &[
                            OffsetInterval::new(Param::START, held).unwrap(),
                            OffsetInterval::new(held, Param::END).unwrap(),
                        ],
                    )
                    .unwrap();
            }
        }
        let Resolution::Unresolved(task) = ledger.resolve(source, held) else {
            panic!("actual repair")
        };
        let result = ledger.resolve_unique_point(source, held, limits(2, 3));
        let UniquePointOutcome::Repair { cause, candidates } = result.outcome() else {
            panic!("bounded repair")
        };
        assert_eq!(cause, task.orphaned_by());
        assert_eq!(
            candidates.iter().flatten().copied().collect::<Vec<_>>(),
            task.candidates()
        );
        assert_eq!(
            (result.reference(), result.param()),
            (task.reference(), task.param())
        );
        assert!(result.resolved().is_none());
    }
}
