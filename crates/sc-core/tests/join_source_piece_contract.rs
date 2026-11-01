//! Complete authentic A/B held source occurrences;no currentness or geometric bank certificate.
#![allow(clippy::expect_used)]
use sc_core::{construction::*, ontology::*};
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
#[path = "support/seam_edit_sources.rs"]
mod common;
fn with_population(
    cycles: [bool; 2],
    stale: bool,
    run: impl FnOnce(BoundJoinCutPopulation<'_>, &IdentityLedger, &[SewingLandmark]),
) {
    let mut f = common::fixture(false);
    let old = common::first_span(&f.graph).definition();
    let a = old.a.range.edge();
    let b = old.b.range.edge();
    let [outer, hole_a, hole_a2, hole_b]: [EdgeRef; 4] = f
        .ledger
        .declare_edges(&mut DeterministicIdGenerator::from_start(10), 4)
        .expect("additional actual source edges")
        .try_into()
        .expect("four sources");
    let cycle = if cycles.into_iter().any(|v| v) {
        Some(
            *f.ledger
                .declare_edges(&mut DeterministicIdGenerator::from_start(1000), 1)
                .expect("actual cyclic prerequisite")
                .first()
                .expect("source"),
        )
    } else {
        None
    };
    let directed = |edge, direction| DirectedEdge { edge, direction };
    let pieces = f
        .pieces
        .iter()
        .enumerate()
        .map(|(index, piece)| {
            let mut d = piece.definition().clone();
            if index == 0 {
                d.boundary = vec![
                    directed(a, Direction::Original),
                    directed(outer, Direction::Reversed),
                ];
                d.holes = vec![
                    vec![directed(hole_a, Direction::Reversed)],
                    vec![directed(hole_a2, Direction::Original)],
                ];
                d.construction_lines = vec![
                    directed(
                        if cycles.first().copied().expect("A cycle flag") {
                            cycle.expect("cycle source")
                        } else {
                            a
                        },
                        Direction::Reversed,
                    ),
                    directed(outer, Direction::Original),
                    directed(a, Direction::Original),
                ];
                d.cut_on_fold = true;
                d.fold_edges = vec![a];
            } else {
                d.holes = vec![vec![directed(hole_b, Direction::Original)]];
                d.construction_lines = vec![directed(
                    if cycles.get(1).copied().expect("B cycle flag") {
                        cycle.expect("cycle source")
                    } else {
                        b
                    },
                    Direction::Reversed,
                )];
            }
            Piece::new(d, &f.ledger).expect("actual rich source Piece")
        })
        .collect();
    let inventory = PieceInventory::new(pieces).expect("canonical original inventory");
    let graph = SewingGraph::new(
        SewingGraphDefinition {
            id: id(300),
            spans: vec![old.clone()],
        },
        &f.plan,
        inventory.pieces(),
        &f.marks,
        &f.ledger,
    )
    .expect("actual original graph");
    let edit = JoinPiecesAtSeam::new(id(1000), id(301), id(100), id(101)).expect("request");
    let source = edit
        .bind_source(&graph, &f.plan, inventory.pieces())
        .expect("original inventory slice");
    let limits = JoinCopyInventoryLimits::new(2, 2, 8);
    let copies = source
        .bind_copy_inventory(&inventory, &limits)
        .expect("complete copy inventory");
    let request = JoinExactSpliceRequirement::new(JoinSplicePlacement::KeepOriginal);
    let splice = copies.require_exact_splice(&request);
    let metadata = JoinPieceMetadataPolicy::new(
        JoinPieceFieldIntent::FromSide(SeamSideId::A),
        JoinPieceFieldIntent::FromSide(SeamSideId::A),
        JoinPieceFieldIntent::FromSide(SeamSideId::A),
        JoinPieceFieldIntent::FromSide(SeamSideId::A),
    );
    let text_limits = JoinPieceMetadataLimits::new(28);
    let fields = splice
        .bind_metadata(&metadata, &text_limits)
        .expect("selected metadata");
    let population = JoinCutPopulation::new(1, Mirroring::Single);
    let bound = fields
        .bind_cut_population(&population)
        .expect("canonical population intent");
    if stale {
        f.ledger
            .delete(&mut DeterministicIdGenerator::from_start(5000), a)
            .expect("actual retirement after source birth");
    }
    run(bound, &f.ledger, &f.marks);
}
#[test]
fn every_original_field_and_repeated_use_keeps_actual_owner_family_order_and_reverse() {
    use JoinSourceOccurrenceField as F;
    use JoinSourceOccurrenceLocation as L;
    with_population([false, false], false, |population, _, _| {
        let limits = JoinSourcePieceLimits::new(3, 11);
        let source = population
            .admit_source_pieces(limits)
            .expect("whole original H3/R11");
        assert!(std::ptr::eq(source.population(), &population));
        assert_eq!(source.limits(), limits);
        assert_eq!(
            (
                limits.holes(),
                limits.occurrences(),
                source.hole_count(),
                source.occurrence_count()
            ),
            (3, 11, 3, 11)
        );
        let a = source.piece(SeamSideId::A).definition();
        let b = source.piece(SeamSideId::B).definition();
        let expected = vec![
            (
                SeamSideId::A,
                L::Boundary(0),
                F::Directed(a.boundary.first().expect("A boundary0")),
            ),
            (
                SeamSideId::A,
                L::Boundary(1),
                F::Directed(a.boundary.get(1).expect("A boundary1")),
            ),
            (
                SeamSideId::A,
                L::Hole { hole: 0, index: 0 },
                F::Directed(a.holes.first().expect("A hole0").first().expect("field")),
            ),
            (
                SeamSideId::A,
                L::Hole { hole: 1, index: 0 },
                F::Directed(a.holes.get(1).expect("A hole1").first().expect("field")),
            ),
            (
                SeamSideId::A,
                L::Construction(0),
                F::Directed(a.construction_lines.first().expect("A construction0")),
            ),
            (
                SeamSideId::A,
                L::Construction(1),
                F::Directed(a.construction_lines.get(1).expect("A construction1")),
            ),
            (
                SeamSideId::A,
                L::Construction(2),
                F::Directed(a.construction_lines.get(2).expect("A construction2")),
            ),
            (
                SeamSideId::A,
                L::Fold(0),
                F::Fold(a.fold_edges.first().expect("A separate fold")),
            ),
            (
                SeamSideId::B,
                L::Boundary(0),
                F::Directed(b.boundary.first().expect("B boundary0")),
            ),
            (
                SeamSideId::B,
                L::Hole { hole: 0, index: 0 },
                F::Directed(b.holes.first().expect("B hole0").first().expect("field")),
            ),
            (
                SeamSideId::B,
                L::Construction(0),
                F::Directed(b.construction_lines.first().expect("B construction0")),
            ),
        ];
        let actual: Vec<_> = source.occurrences().collect();
        assert_eq!(actual.len(), 11);
        for (view, (side, location, field)) in actual.iter().copied().zip(expected.iter().copied())
        {
            assert!(std::ptr::eq(view.context(), &source));
            assert_eq!((view.side(), view.location()), (side, location));
            assert!(std::ptr::eq(
                view.piece(),
                population.metadata().splice().source().side(side).piece()
            ));
            assert!(std::ptr::eq(
                view.copy(),
                population.metadata().splice().source().side(side).copy()
            ));
            assert!(std::ptr::eq(
                view.source_side(),
                population
                    .metadata()
                    .splice()
                    .source()
                    .side(side)
                    .source_side()
            ));
            assert!(std::ptr::eq(view.field().edge(), field.edge()));
            assert!(match (view.field(), field) {
                (F::Directed(a), F::Directed(b)) => std::ptr::eq(a, b),
                (F::Fold(a), F::Fold(b)) => std::ptr::eq(a, b),
                _ => false,
            });
        }
        assert_eq!(
            source
                .occurrences()
                .rev()
                .map(|f| (f.side(), f.location()))
                .collect::<Vec<_>>(),
            expected
                .iter()
                .rev()
                .map(|(side, location, _)| (*side, *location))
                .collect::<Vec<_>>()
        );
        assert!(format!("{source:?}").contains("BoundJoinSourcePieces"));
    });
}
#[test]
fn complete_h_then_r_quotas_precede_first_original_a_b_self_source_and_lend_actual_refused_edge() {
    use JoinSourceOccurrenceLocation as L;
    use JoinSourcePieceRefusal as R;
    for (cycles, first) in [
        ([true, true], SeamSideId::A),
        ([true, false], SeamSideId::A),
        ([false, true], SeamSideId::B),
    ] {
        with_population(cycles, false, |population, _, _| {
            for (limits, cause) in [
                (
                    JoinSourcePieceLimits::new(2, 0),
                    R::Holes {
                        required: 3,
                        allowed: 2,
                    },
                ),
                (
                    JoinSourcePieceLimits::new(3, 10),
                    R::Occurrences {
                        required: 11,
                        allowed: 10,
                    },
                ),
            ] {
                let error = population
                    .admit_source_pieces(limits)
                    .expect_err("whole populations before fields");
                assert!(std::ptr::eq(error.population(), &population));
                assert_eq!(error.limits(), limits);
                assert_eq!(error.refusal(), cause);
                assert!(error.refused_source().is_none());
            }
            let error = population
                .admit_source_pieces(JoinSourcePieceLimits::new(3, 11))
                .expect_err("first original direct self-source");
            assert_eq!(
                error.refusal(),
                R::SelfOperation {
                    side: first,
                    location: L::Construction(0)
                }
            );
            let original = &population
                .metadata()
                .splice()
                .source()
                .side(first)
                .piece()
                .definition()
                .construction_lines
                .first()
                .expect("original cyclic field")
                .edge;
            assert!(std::ptr::eq(
                error.refused_source().expect("actual refused field"),
                original
            ));
            assert_eq!(original.creator(), id(1000));
            assert!(error.to_string().starts_with("join source pieces:"));
            assert!(format!("{error:?}").contains("JoinSourcePieceError"));
        });
    }
}
#[test]
fn held_occurrence_admission_does_not_replace_actual_current_source_validation() {
    with_population([false, false], true, |population, ledger, marks| {
        let original = population.metadata().splice().source();
        assert!(matches!(
            original
                .graph()
                .validate_current(original.plan(), original.pieces(), marks, ledger),
            Err(SewingError::UnresolvedRange {
                side: SeamSideId::A,
                ..
            })
        ));
        let source = population
            .admit_source_pieces(JoinSourcePieceLimits::new(3, 11))
            .expect("held fields only");
        assert_eq!(source.occurrence_count(), 11);
        assert!(std::ptr::eq(
            source.piece(SeamSideId::A),
            original.side(SeamSideId::A).piece()
        ));
    });
}
