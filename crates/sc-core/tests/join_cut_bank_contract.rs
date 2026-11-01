//! Literal held A/B cut banks;partial parameters,contour direction and copy reflection stay separate.
#![allow(clippy::expect_used)]
use sc_core::{construction::*, ontology::*};
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn q(n: i64, d: i64) -> Param {
    Param::new(Rational::new(n, d).expect("fraction")).expect("unit interval")
}
#[allow(dead_code)]
#[path = "support/seam_edit_sources.rs"]
mod common;
#[derive(Clone, Copy)]
enum Position {
    Boundary,
    Hole,
    Construction,
}
fn with_sources(
    positions: [Position; 2],
    partial: bool,
    stale: bool,
    run: impl FnOnce(BoundJoinSourcePieces<'_>, &IdentityLedger, &[SewingLandmark]),
) {
    let mut f = common::fixture(false);
    let mut span = common::first_span(&f.graph).definition().clone();
    if partial {
        span.a.range =
            EdgeRange::new(span.a.range.edge(), q(1, 4), q(3, 4)).expect("original A partial");
        span.b.range =
            EdgeRange::new(span.b.range.edge(), q(1, 5), q(4, 5)).expect("original B partial");
    }
    let marks = vec![SewingLandmark::Turn(
        TurnPoint::new(
            id(401),
            EdgeAnchor {
                edge: span.a.range.edge(),
                param: span.a.range.from(),
            },
            f.pieces.first().expect("original A"),
            &f.ledger,
        )
        .expect("actual held landmark"),
    )];
    let retired = span.a.range.edge();
    let graph = SewingGraph::new(
        SewingGraphDefinition {
            id: id(300),
            spans: vec![span],
        },
        &f.plan,
        &f.pieces,
        &marks,
        &f.ledger,
    )
    .expect("actual held graph before supplied inventory");
    let [outer_a, outer_b]: [EdgeRef; 2] = f
        .ledger
        .declare_edges(&mut DeterministicIdGenerator::from_start(10), 2)
        .expect("actual outer edges")
        .try_into()
        .expect("two sources");
    let pieces = f
        .pieces
        .iter()
        .enumerate()
        .map(|(index, piece)| {
            let mut d = piece.definition().clone();
            let (side, position, outer) = if index == 0 {
                (
                    SeamSideId::A,
                    *positions.first().expect("A position"),
                    outer_a,
                )
            } else {
                (
                    SeamSideId::B,
                    *positions.get(1).expect("B position"),
                    outer_b,
                )
            };
            let selected = match side {
                SeamSideId::A => common::first_span(&graph).definition().a.range.edge(),
                SeamSideId::B => common::first_span(&graph).definition().b.range.edge(),
            };
            let field = DirectedEdge {
                edge: selected,
                direction: Direction::Reversed,
            };
            d.boundary = vec![DirectedEdge {
                edge: outer,
                direction: Direction::Original,
            }];
            d.holes.clear();
            match position {
                Position::Boundary => d.boundary.push(field),
                Position::Hole => d.holes.push(vec![field]),
                Position::Construction => {}
            }
            d.construction_lines = vec![DirectedEdge {
                edge: selected,
                direction: Direction::Original,
            }];
            if index == 1 {
                d.quantity = 2;
                d.mirroring = Mirroring::MirroredPairs;
            }
            Piece::new(d, &f.ledger).expect("actual supplied Piece")
        })
        .collect();
    let inventory = PieceInventory::new(pieces).expect("canonical supplied inventory");
    let plan = CutPlan::new(
        vec![
            CutCopyDefinition {
                id: id(201),
                piece: id(101),
                orientation: CopyOrientation::Reflected,
            },
            *f.plan.copy(id(200)).expect("original A copy").definition(),
            CutCopyDefinition {
                id: id(203),
                piece: id(101),
                orientation: CopyOrientation::Authored,
            },
        ],
        inventory.pieces(),
    )
    .expect("complete actual plan");
    let edit = JoinPiecesAtSeam::new(id(1000), id(301), id(100), id(101)).expect("join request");
    let source = edit
        .bind_source(&graph, &plan, inventory.pieces())
        .expect("actual supplied source slice");
    let limits = JoinCopyInventoryLimits::new(3, 2, 11);
    let copies = source
        .bind_copy_inventory(&inventory, &limits)
        .expect("whole actual inventory");
    let request = JoinExactSpliceRequirement::new(JoinSplicePlacement::RigidAlign {
        fixed: SeamSideId::B,
        orientation: JoinMovingOrientation::Preserve,
    });
    let splice = copies.require_exact_splice(&request);
    let fields = JoinPieceMetadataPolicy::new(
        JoinPieceFieldIntent::FromSide(SeamSideId::A),
        JoinPieceFieldIntent::FromSide(SeamSideId::A),
        JoinPieceFieldIntent::FromSide(SeamSideId::A),
        JoinPieceFieldIntent::FromSide(SeamSideId::A),
    );
    let limits = JoinPieceMetadataLimits::new(28);
    let metadata = splice
        .bind_metadata(&fields, &limits)
        .expect("selected fields");
    let raw = JoinCutPopulation::new(1, Mirroring::Single);
    let population = metadata
        .bind_cut_population(&raw)
        .expect("explicit population");
    let sources = population
        .admit_source_pieces(JoinSourcePieceLimits::new(2, 8))
        .expect("complete original source admission");
    if stale {
        f.ledger
            .delete(&mut DeterministicIdGenerator::from_start(5000), retired)
            .expect("actual ledger retirement");
    }
    run(sources, &f.ledger, &marks);
}
#[test]
fn actual_outer_or_hole_fields_keep_positive_partial_ranges_traversal_and_reflected_copy_parents() {
    use JoinSourceOccurrenceLocation as L;
    for a in [Position::Boundary, Position::Hole] {
        for b in [Position::Boundary, Position::Hole] {
            for partial in [false, true] {
                with_sources([a, b], partial, false, |sources, _, _| {
                    let banks = sources
                        .bind_cut_banks()
                        .expect("both literal actual cut banks");
                    assert!(std::ptr::eq(banks.sources(), &sources));
                    assert_eq!(banks.banks().len(), 2);
                    assert_eq!(
                        banks
                            .banks()
                            .rev()
                            .map(JoinCutBank::side)
                            .collect::<Vec<_>>(),
                        vec![SeamSideId::B, SeamSideId::A]
                    );
                    for (side, position) in [(SeamSideId::A, a), (SeamSideId::B, b)] {
                        let bank = banks.bank(side);
                        let original = sources.population().metadata().splice().source().side(side);
                        let d = original.piece().definition();
                        let (location, field) = match position {
                            Position::Boundary => (L::Boundary(1), d.boundary.get(1)),
                            Position::Hole => (
                                L::Hole { hole: 0, index: 0 },
                                d.holes.first().and_then(|h| h.first()),
                            ),
                            Position::Construction => (L::Construction(0), None),
                        };
                        assert!(std::ptr::eq(bank.context(), &banks));
                        assert_eq!(bank.side(), side);
                        assert!(std::ptr::eq(bank.occurrence().context(), &sources));
                        assert_eq!(bank.occurrence().location(), location);
                        assert!(std::ptr::eq(
                            bank.directed(),
                            field.expect("actual cut field")
                        ));
                        assert!(std::ptr::eq(bank.range(), &original.source_side().range));
                        assert!(std::ptr::eq(bank.source_side(), original.source_side()));
                        assert!(std::ptr::eq(bank.piece(), original.piece()));
                        assert!(std::ptr::eq(bank.copy(), original.copy()));
                        assert_eq!(bank.directed().direction, Direction::Reversed);
                        let expected = if partial {
                            match side {
                                SeamSideId::A => (q(1, 4), q(3, 4)),
                                SeamSideId::B => (q(1, 5), q(4, 5)),
                            }
                        } else {
                            (Param::START, Param::END)
                        };
                        assert_eq!((bank.range().from(), bank.range().to()), expected);
                    }
                    assert_eq!(
                        banks.bank(SeamSideId::B).copy().orientation(),
                        CopyOrientation::Reflected
                    );
                    assert!(format!("{banks:?}").contains("BoundJoinCutBanks"));
                });
            }
        }
    }
}
#[test]
fn construction_matches_cannot_substitute_and_first_missing_side_keeps_whole_source_and_range() {
    for (positions, side) in [
        ([Position::Construction, Position::Boundary], SeamSideId::A),
        ([Position::Boundary, Position::Construction], SeamSideId::B),
        (
            [Position::Construction, Position::Construction],
            SeamSideId::A,
        ),
    ] {
        with_sources(positions, true, false, |sources, _, _| {
            let error = sources
                .bind_cut_banks()
                .expect_err("literal cut contour required");
            assert!(std::ptr::eq(error.sources(), &sources));
            assert_eq!(error.refusal(), JoinCutBankRefusal::NoExactCutBank { side });
            assert!(std::ptr::eq(
                error.source_range(),
                &sources
                    .population()
                    .metadata()
                    .splice()
                    .source()
                    .side(side)
                    .source_side()
                    .range
            ));
            assert!(error.to_string().starts_with("join cut bank:"));
            assert!(format!("{error:?}").contains("JoinCutBankError"));
        });
    }
}
#[test]
fn literal_held_bank_membership_does_not_override_actual_current_graph_refusal() {
    with_sources(
        [Position::Boundary, Position::Boundary],
        false,
        true,
        |sources, ledger, marks| {
            let original = sources.population().metadata().splice().source();
            assert!(matches!(
                original.graph().validate_current(
                    original.plan(),
                    original.pieces(),
                    marks,
                    ledger
                ),
                Err(SewingError::UnresolvedRange {
                    side: SeamSideId::A,
                    ..
                })
            ));
            let banks = sources.bind_cut_banks().expect("held literal field only");
            assert_eq!(
                banks.bank(SeamSideId::A).range(),
                &original.span().definition().a.range
            );
        },
    );
}
