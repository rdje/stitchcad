//! Whole-range contract tests, including D55 and a differential oracle from the existing point fold.

#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

use sc_core::ontology::{
    DeterministicIdGenerator, Direction, EdgeRange, EdgeRef, EntityId, IdentityLedger, LocalTag,
    OffsetInterval, OrphaningEdit, Param, RangeIssue, RangePortion, Rational, Resolution,
};
use sc_units::Length;

fn t(n: i64, d: i64) -> Param {
    Param::new(Rational::new(n, d).unwrap()).unwrap()
}

fn fixture() -> (IdentityLedger, DeterministicIdGenerator, EdgeRef) {
    let mut ledger = IdentityLedger::new();
    let mut ids = DeterministicIdGenerator::new();
    let edge = *ledger.declare_edges(&mut ids, 1).unwrap().first().unwrap();
    (ledger, ids, edge)
}

fn live_parts(ledger: &IdentityLedger, held: EdgeRange) -> Vec<(EdgeRef, Param, Param, Direction)> {
    ledger
        .resolve_range(held)
        .portions()
        .iter()
        .filter_map(|part| match part {
            RangePortion::Resolved(part) => Some((
                part.range().edge(),
                part.range().from(),
                part.range().to(),
                part.direction(),
            )),
            RangePortion::Unresolved(_) => None,
        })
        .collect()
}

#[test]
fn ranges_reject_equal_and_descending_bounds_and_never_change_the_held_reference() {
    let (ledger, _, edge) = fixture();
    assert!(EdgeRange::new(edge, Param::START, Param::START).is_err());
    let err = EdgeRange::new(edge, Param::END, Param::START).unwrap_err();
    assert_eq!((err.from, err.to), (Param::END, Param::START));
    assert!(err.to_string().contains("from < to"));
    let held = EdgeRange::new(edge, t(1, 4), t(3, 4)).unwrap();
    let result = ledger.resolve_range(held);
    assert_eq!(result.held(), held);
    assert!(result.has_full_coverage());
    assert_eq!(result.repairs().count(), 0);
    assert_eq!(
        live_parts(&ledger, held),
        vec![(edge, t(1, 4), t(3, 4), Direction::Original)]
    );
}

#[test]
fn unknown_source_is_a_visible_range_task_even_when_the_point_fold_passes_through() {
    let (ledger, _, _) = fixture();
    let edge = EdgeRef::new(EntityId::from_bits(u128::MAX), LocalTag::FIRST);
    let held = EdgeRange::whole(edge);
    let result = ledger.resolve_range(held);
    assert!(!result.has_full_coverage());
    let task = result.repairs().next().unwrap();
    assert_eq!(task.held(), held);
    assert!(task.candidates().is_empty());
    assert_eq!(task.issue(), RangeIssue::UnknownSource);
    assert_eq!(task.affected().range(), held);
    assert!(task.to_string().contains("never created"));
}

#[test]
fn split_intersects_a_partial_range_exactly_and_preserves_the_endpoint_choice() {
    let (mut ledger, mut ids, edge) = fixture();
    let held = EdgeRange::new(edge, t(1, 4), t(3, 4)).unwrap();
    let split = ledger.split(&mut ids, edge, t(1, 2)).unwrap();
    assert_eq!(
        live_parts(&ledger, held),
        vec![
            (split.first(), t(1, 2), Param::END, Direction::Original),
            (split.second(), Param::START, t(1, 2), Direction::Original),
        ]
    );
    let touching = EdgeRange::new(edge, t(1, 2), Param::END).unwrap();
    let result = ledger.resolve_range(touching);
    assert!(result.has_full_coverage());
    assert_eq!(result.portions().len(), 1);
    assert!(matches!(result.start(), Resolution::SplitPoint { .. }));
    assert_eq!(result.end().resolved().unwrap().edge(), split.second());
}

#[test]
fn d55_deleted_interior_blocks_range_coverage_despite_two_resolved_endpoints() {
    let (mut ledger, mut ids, edge) = fixture();
    let held = EdgeRange::whole(edge);
    let first = ledger.split(&mut ids, edge, t(1, 3)).unwrap();
    let second = ledger.split(&mut ids, first.second(), t(1, 2)).unwrap();
    let operation = ledger.delete(&mut ids, second.first()).unwrap();
    let result = ledger.resolve_range(held);
    assert!(result.start().resolved().is_some());
    assert!(result.end().resolved().is_some());
    assert!(!result.has_full_coverage());
    assert_eq!(result.portions().len(), 3);
    let task = result.repairs().next().unwrap();
    assert_eq!(task.held(), held);
    assert_eq!(task.affected().range(), EdgeRange::whole(second.first()));
    assert_eq!(
        task.issue(),
        RangeIssue::Orphaned(OrphaningEdit::Deleted {
            operation,
            edge: second.first()
        })
    );
    assert_eq!(
        live_parts(&ledger, held),
        vec![
            (first.first(), Param::START, Param::END, Direction::Original),
            (
                second.second(),
                Param::START,
                Param::END,
                Direction::Original
            ),
        ]
    );
    // An edit outside the failed portion cannot silently reattach its repair.
    ledger.reverse(&mut ids, first.first()).unwrap();
    assert_eq!(ledger.resolve_range(held).repairs().next().unwrap(), task);
    // A partial range outside the deletion still has full coverage.
    assert!(ledger
        .resolve_range(EdgeRange::new(edge, Param::START, t(1, 4)).unwrap())
        .has_full_coverage());
}

#[test]
fn reversal_reflects_bounds_and_a_later_split_keeps_authored_traversal_order() {
    let (mut ledger, mut ids, edge) = fixture();
    let held = EdgeRange::new(edge, t(1, 5), t(3, 5)).unwrap();
    ledger.reverse(&mut ids, edge).unwrap();
    assert_eq!(
        live_parts(&ledger, held),
        vec![(edge, t(2, 5), t(4, 5), Direction::Reversed)]
    );
    let split = ledger.split(&mut ids, edge, t(1, 2)).unwrap();
    assert_eq!(
        live_parts(&ledger, held),
        vec![
            (split.second(), Param::START, t(3, 5), Direction::Reversed),
            (split.first(), t(4, 5), Param::END, Direction::Reversed),
        ]
    );
}

#[test]
fn merge_maps_both_range_ends_by_declared_arc_length_without_rounding() {
    let (mut ledger, mut ids, first) = fixture();
    let second = *ledger.declare_edges(&mut ids, 1).unwrap().first().unwrap();
    let a = EdgeRange::new(first, t(1, 3), t(2, 3)).unwrap();
    let b = EdgeRange::new(second, t(1, 7), t(5, 7)).unwrap();
    let merged = ledger
        .merge(
            &mut ids,
            first,
            Length::from_micrometres(3).unwrap(),
            second,
            Length::from_micrometres(7).unwrap(),
        )
        .unwrap();
    // Integer-length oracle: [1,2] / 10 and [3+1,3+5] / 10.
    assert_eq!(
        live_parts(&ledger, a),
        vec![(merged, t(1, 10), t(1, 5), Direction::Original)]
    );
    assert_eq!(
        live_parts(&ledger, b),
        vec![(merged, t(2, 5), t(4, 5), Direction::Original)]
    );
}

#[test]
fn offsets_preserve_all_gaps_and_point_ambiguity_separately() {
    let (mut ledger, mut ids, edge) = fixture();
    let held = EdgeRange::whole(edge);
    let fragments = ledger
        .offset(
            &mut ids,
            edge,
            &[
                OffsetInterval::new(t(1, 5), t(2, 5)).unwrap(),
                OffsetInterval::new(t(3, 5), t(4, 5)).unwrap(),
            ],
        )
        .unwrap();
    let result = ledger.resolve_range(held);
    assert!(!result.has_full_coverage());
    assert_eq!(result.portions().len(), 5);
    assert_eq!(
        result
            .repairs()
            .map(|task| (task.affected().range().from(), task.affected().range().to()))
            .collect::<Vec<_>>(),
        vec![
            (Param::START, t(1, 5)),
            (t(2, 5), t(3, 5)),
            (t(4, 5), Param::END)
        ]
    );
    assert_eq!(
        live_parts(&ledger, held)
            .iter()
            .map(|part| part.0)
            .collect::<Vec<_>>(),
        fragments
    );

    let (mut ledger, mut ids, edge) = fixture();
    ledger
        .offset(
            &mut ids,
            edge,
            &[
                OffsetInterval::new(Param::START, t(1, 2)).unwrap(),
                OffsetInterval::new(t(1, 2), Param::END).unwrap(),
            ],
        )
        .unwrap();
    let result = ledger.resolve_range(EdgeRange::new(edge, t(1, 2), Param::END).unwrap());
    assert!(result.has_full_coverage());
    assert!(matches!(result.start(), Resolution::Unresolved(task)
        if matches!(task.orphaned_by(), OrphaningEdit::OffsetAmbiguous { .. })));
}

#[test]
fn arbitrarily_narrow_trimmed_gaps_cannot_escape_the_interval_fold() {
    let (mut ledger, mut ids, edge) = fixture();
    let low = t(500_000_001, 1_000_000_000);
    let high = t(500_000_002, 1_000_000_000);
    ledger
        .offset(
            &mut ids,
            edge,
            &[
                OffsetInterval::new(Param::START, low).unwrap(),
                OffsetInterval::new(high, Param::END).unwrap(),
            ],
        )
        .unwrap();
    // Every point on a hundredths grid is resolved; the nanowide interval is still detected.
    assert!((0..=100).all(|n| ledger.resolve(edge, t(n, 100)).resolved().is_some()));
    let result = ledger.resolve_range(EdgeRange::whole(edge));
    assert!(!result.has_full_coverage());
    let affected = result.repairs().next().unwrap().affected().range();
    assert_eq!((affected.from(), affected.to()), (low, high));
}

#[test]
fn a_reversed_offset_orders_live_fragments_and_repairs_in_authored_traversal() {
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
    let result = ledger.resolve_range(held);
    let mut parts = result.portions().iter();
    assert!(matches!(parts.next(), Some(RangePortion::Resolved(part))
        if part.range().edge() == *fragments.last().unwrap() && part.direction() == Direction::Reversed));
    assert!(matches!(parts.next(), Some(RangePortion::Unresolved(task))
        if task.affected().range().from() == t(1, 2) && task.affected().range().to() == t(3, 4)
            && task.affected().direction() == Direction::Reversed));
    assert!(matches!(parts.next(), Some(RangePortion::Resolved(part))
        if part.range().edge() == *fragments.first().unwrap() && part.direction() == Direction::Reversed));
    assert!(matches!(parts.next(), Some(RangePortion::Unresolved(task))
        if task.affected().range().from() == Param::START && task.affected().range().to() == t(1, 4)));
    assert!(parts.next().is_none());
    let uncovered = ledger.resolve_range(EdgeRange::new(edge, t(4, 5), t(9, 10)).unwrap());
    assert_eq!(uncovered.portions().len(), 1);
    assert!(!uncovered.has_full_coverage());
}

#[test]
fn full_range_split_merge_is_exact_even_when_portion_boundaries_are_retained() {
    let (mut ledger, mut ids, edge) = fixture();
    let held = EdgeRange::whole(edge);
    let split = ledger.split(&mut ids, edge, t(2, 5)).unwrap();
    let merged = ledger
        .merge(
            &mut ids,
            split.first(),
            Length::from_micrometres(2).unwrap(),
            split.second(),
            Length::from_micrometres(3).unwrap(),
        )
        .unwrap();
    let result = ledger.resolve_range(held);
    assert!(result.has_full_coverage());
    assert_eq!(
        live_parts(&ledger, held),
        vec![
            (merged, Param::START, t(2, 5), Direction::Original),
            (merged, t(2, 5), Param::END, Direction::Original),
        ]
    );
}

#[test]
fn failed_exact_recomputation_is_a_visible_task_not_an_approximation() {
    let (mut ledger, mut ids, edge) = fixture();
    let held = EdgeRange::new(edge, t(1, i64::MAX), Param::END).unwrap();
    ledger.split(&mut ids, edge, t(2, 3)).unwrap();
    let result = ledger.resolve_range(held);
    assert!(!result.has_full_coverage());
    let task = result.repairs().next().unwrap();
    assert_eq!(task.held(), held);
    assert!(matches!(
        task.issue(),
        RangeIssue::Orphaned(OrphaningEdit::RecomputationFailed { .. })
    ));
}

#[test]
fn reversing_twice_returns_identical_range_evidence() {
    let (mut ledger, mut ids, edge) = fixture();
    let held = EdgeRange::new(edge, t(1, 3), t(4, 5)).unwrap();
    let before = ledger.resolve_range(held);
    ledger.reverse(&mut ids, edge).unwrap();
    ledger.reverse(&mut ids, edge).unwrap();
    assert_eq!(ledger.resolve_range(held), before);
}

#[test]
fn generated_edit_scripts_agree_with_the_independent_point_fold_away_from_boundaries() {
    // House recorded seed; the oracle is existing production point resolution, not the interval code.
    let mut seed = 0x57_49_54_43_48_43_41_44u64;
    let mut next = || {
        seed ^= seed >> 12;
        seed ^= seed << 25;
        seed ^= seed >> 27;
        seed.wrapping_mul(0x2545_F491_4F6C_DD1D)
    };
    for _ in 0..200 {
        let (mut ledger, mut ids, edge) = fixture();
        let held = EdgeRange::new(edge, t(1, 7), t(6, 7)).unwrap();
        for _ in 0..12 {
            let live = ledger.live_edges().collect::<Vec<_>>();
            if live.is_empty() {
                break;
            }
            let index = usize::try_from(next() % u64::try_from(live.len()).unwrap()).unwrap();
            let victim = *live.get(index).unwrap();
            match next() % 5 {
                0 => {
                    ledger
                        .split(
                            &mut ids,
                            victim,
                            t(i64::try_from(next() % 9 + 1).unwrap(), 10),
                        )
                        .unwrap();
                }
                1 => {
                    ledger.reverse(&mut ids, victim).unwrap();
                }
                2 => {
                    ledger.delete(&mut ids, victim).unwrap();
                }
                3 if live.len() > 1 => {
                    let other = *live.iter().find(|other| **other != victim).unwrap();
                    ledger
                        .merge(
                            &mut ids,
                            victim,
                            Length::from_micrometres(3).unwrap(),
                            other,
                            Length::from_micrometres(7).unwrap(),
                        )
                        .unwrap();
                }
                _ => {
                    ledger
                        .offset(
                            &mut ids,
                            victim,
                            &[
                                OffsetInterval::new(Param::START, t(1, 3)).unwrap(),
                                OffsetInterval::new(t(2, 3), Param::END).unwrap(),
                            ],
                        )
                        .unwrap();
                }
            }
            let result = ledger.resolve_range(held);
            assert_eq!(
                result,
                ledger.resolve_range(held),
                "queries must be deterministic"
            );
            let arithmetic_repair = result.repairs().any(|task| {
                matches!(
                    task.issue(),
                    RangeIssue::Orphaned(OrphaningEdit::RecomputationFailed { .. })
                )
            });
            for n in 1..101 {
                let probe = t(n, 101);
                if probe <= held.from() || probe >= held.to() {
                    continue;
                }
                match ledger.resolve(edge, probe) {
                    Resolution::Resolved(point) if !arithmetic_repair => {
                        assert!(
                            result.portions().iter().any(|portion| match portion {
                                RangePortion::Resolved(part) =>
                                    part.range().edge() == point.edge()
                                        && part.range().from() <= point.param()
                                        && point.param() <= part.range().to()
                                        && part.direction() == point.direction(),
                                RangePortion::Unresolved(_) => false,
                            }),
                            "point {point:?} absent from interval evidence {result:?}"
                        );
                    }
                    Resolution::Unresolved(task)
                        if matches!(
                            task.orphaned_by(),
                            OrphaningEdit::Deleted { .. } | OrphaningEdit::OffsetUnmapped { .. }
                        ) =>
                    {
                        assert!(
                            !result.has_full_coverage(),
                            "lost interior reported complete"
                        );
                    }
                    // Shared points and arithmetic refusal are separate, explicitly typed obligations.
                    _ => {}
                }
            }
        }
    }
}
