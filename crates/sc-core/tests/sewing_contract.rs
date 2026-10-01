//! Sewing topology/intent tests; no fixture pretends to measure geometry or certify seam lengths.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
use sc_core::ontology::{
    CopyOrientation, CutCopyDefinition, CutPlan, CutPlanError, CuttingSide, DeclaredEase,
    DeterministicIdGenerator, DirectedEdge, Direction, EaseAmount, EaseDistribution,
    EaseValidation, EdgeAnchor, EdgeRange, EntityId, GeometricValidation, IdentityLedger,
    LabelText, MaterialAssignment, Mirroring, Notch, NotchDefinition, NotchDimensionBindings,
    NotchProfileBindings, Param, Piece, PieceDefinition, ProfileParameterRef, Rational,
    SeamDirection, SeamSide, SeamSideId, SeamSpanDefinition, SewingError, SewingGraph,
    SewingGraphDefinition, SewingLandmark, StopLandmark, TurnPoint, WeightIssue,
    WeightedEaseRegion,
};
use sc_units::Length;
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn t(n: i64, d: i64) -> Param {
    Param::new(Rational::new(n, d).unwrap()).unwrap()
}
struct Fixture {
    pieces: Vec<Piece>,
    plan: CutPlan,
    ledger: IdentityLedger,
    ids: DeterministicIdGenerator,
    definition: SewingGraphDefinition,
}
fn fixture() -> Fixture {
    let mut ledger = IdentityLedger::new();
    let mut ids = DeterministicIdGenerator::new();
    let edges = ledger.declare_edges(&mut ids, 4).unwrap();
    let pieces = (0..2)
        .map(|index| {
            Piece::new(
                PieceDefinition {
                    id: id(100 + index),
                    boundary: vec![DirectedEdge {
                        edge: *edges.get(usize::try_from(index).unwrap()).unwrap(),
                        direction: Direction::Original,
                    }],
                    holes: vec![],
                    construction_lines: vec![DirectedEdge {
                        edge: *edges.get(usize::try_from(index + 2).unwrap()).unwrap(),
                        direction: Direction::Original,
                    }],
                    quantity: if index == 0 { 2 } else { 1 },
                    mirroring: if index == 0 {
                        Mirroring::MirroredPairs
                    } else {
                        Mirroring::Single
                    },
                    cut_on_fold: false,
                    fold_edges: vec![],
                    cutting_side: CuttingSide::Face,
                    material: MaterialAssignment::Unresolved("selection pending".into()),
                    layer_index: 0,
                    label: LabelText {
                        name: format!("piece {index}"),
                        size: "M".into(),
                        fabric: "woven".into(),
                        colorway: "plain".into(),
                    },
                },
                &ledger,
            )
            .unwrap()
        })
        .collect::<Vec<_>>();
    let copies = vec![
        CutCopyDefinition {
            id: id(200),
            piece: id(100),
            orientation: CopyOrientation::Authored,
        },
        CutCopyDefinition {
            id: id(201),
            piece: id(100),
            orientation: CopyOrientation::Reflected,
        },
        CutCopyDefinition {
            id: id(202),
            piece: id(101),
            orientation: CopyOrientation::Authored,
        },
    ];
    let plan = CutPlan::new(copies, &pieces).unwrap();
    let span = SeamSpanDefinition {
        id: id(301),
        a: SeamSide {
            copy: id(200),
            range: EdgeRange::whole(*edges.first().unwrap()),
        },
        b: SeamSide {
            copy: id(202),
            range: EdgeRange::whole(*edges.get(1).unwrap()),
        },
        direction: SeamDirection::Opposite,
        ease: DeclaredEase {
            amount: EaseAmount::Explicit(Length::ZERO),
            distribution: EaseDistribution::Uniform,
        },
        stops: vec![],
    };
    Fixture {
        pieces,
        plan,
        ledger,
        ids,
        definition: SewingGraphDefinition {
            id: id(300),
            spans: vec![span],
        },
    }
}
impl Fixture {
    fn graph(&self, marks: &[SewingLandmark]) -> Result<SewingGraph, SewingError> {
        SewingGraph::new(
            self.definition.clone(),
            &self.plan,
            &self.pieces,
            marks,
            &self.ledger,
        )
    }
    fn span(&self) -> &SeamSpanDefinition {
        self.definition.spans.first().unwrap()
    }
    fn span_mut(&mut self) -> &mut SeamSpanDefinition {
        self.definition.spans.first_mut().unwrap()
    }
    fn notch(&self, n: u128, param: Param) -> SewingLandmark {
        let parameter = ProfileParameterRef::new(id(999));
        SewingLandmark::Notch(
            Notch::new(
                NotchDefinition {
                    id: id(n),
                    anchor: EdgeAnchor {
                        edge: self.span().a.range.edge(),
                        param,
                    },
                    representation: NotchProfileBindings {
                        style: parameter,
                        sample: NotchDimensionBindings {
                            depth: parameter,
                            width: parameter,
                        },
                        production: NotchDimensionBindings {
                            depth: parameter,
                            width: parameter,
                        },
                        encoding: parameter,
                    },
                },
                self.pieces.first().unwrap(),
                &self.ledger,
            )
            .unwrap(),
        )
    }
}
#[test]
fn immutable_content_retains_direction_signed_ease_and_visible_obligations() {
    let mut f = fixture();
    f.span_mut().ease.amount = EaseAmount::Explicit(Length::from_micrometres(-1500).unwrap());
    let graph = f.graph(&[]).unwrap();
    assert_eq!(graph.id(), f.definition.id);
    let span = graph.spans().first().unwrap();
    assert_eq!(span.definition(), f.span());
    assert_eq!(span.id(), id(301));
    assert_eq!(
        graph.geometric_validation(),
        GeometricValidation::DeferredToG2
    );
    assert_eq!(graph.ease_validation(), EaseValidation::DeferredToG2AndG3);
    let mut copy = span.definition().clone();
    copy.stops.push(StopLandmark {
        side: SeamSideId::A,
        landmark: id(400),
    });
    assert!(span.definition().stops.is_empty());
    for direction in [SeamDirection::Same, SeamDirection::Opposite] {
        for amount in [
            EaseAmount::Formula(id(600)),
            EaseAmount::Profile(ProfileParameterRef::new(id(601))),
        ] {
            f.span_mut().direction = direction;
            f.span_mut().ease.amount = amount;
            assert_eq!(
                f.graph(&[]).unwrap().spans().first().unwrap().definition(),
                f.span()
            );
        }
    }
}
#[test]
fn physical_copies_of_one_pattern_have_different_neighbours_and_same_source_ranges() {
    let mut f = fixture();
    let mut second = f.span().clone();
    second.id = id(302);
    second.a.copy = id(201);
    second.b = f.span().a;
    f.definition.spans.push(second);
    let graph = f.graph(&[]).unwrap();
    assert_eq!(graph.spans().len(), 2);
    let a = graph.spans().first().unwrap().definition().a;
    let b = graph.spans().last().unwrap().definition().a;
    assert_eq!(a.range, b.range);
    assert_ne!(a.copy, b.copy);
    assert_ne!(
        graph.spans().first().unwrap().definition().b.copy,
        graph.spans().last().unwrap().definition().b.copy
    );
    // Joining the two physical copies over an identical source interval is also legal.
    f.span_mut().b = f.span().a;
    f.span_mut().b.copy = id(201);
    assert!(f.graph(&[]).is_ok());
}
#[test]
fn partial_one_to_many_spans_keep_authored_ranges_and_ids() {
    let mut f = fixture();
    let original = f.span().clone();
    f.span_mut().a.range = EdgeRange::new(original.a.range.edge(), Param::START, t(1, 2)).unwrap();
    let mut second = original.clone();
    second.id = id(302);
    second.a.range = EdgeRange::new(original.a.range.edge(), t(1, 2), Param::END).unwrap();
    second.b.copy = id(201);
    second.b.range = original.a.range;
    f.definition.spans.push(second);
    assert_eq!(
        f.graph(&[])
            .unwrap()
            .spans()
            .iter()
            .map(|span| span.definition().clone())
            .collect::<Vec<_>>(),
        f.definition.spans
    );
}
#[test]
fn disjoint_same_copy_ranges_are_legal_but_overlap_and_identical_intervals_are_refused() {
    let mut f = fixture();
    let edge = f.span().a.range.edge();
    f.span_mut().a.range = EdgeRange::new(edge, Param::START, t(1, 2)).unwrap();
    f.span_mut().b = SeamSide {
        copy: id(200),
        range: EdgeRange::new(edge, t(1, 2), Param::END).unwrap(),
    };
    assert!(f.graph(&[]).is_ok()); // Shared endpoint is legal; no positive material is sewn to itself.
    f.span_mut().b.range = EdgeRange::new(edge, t(1, 3), Param::END).unwrap();
    assert_eq!(
        f.graph(&[]),
        Err(SewingError::OverlappingSelfSeam {
            span: id(301),
            copy: id(200)
        })
    );
    f.span_mut().b.range = f.span().a.range;
    assert!(matches!(
        f.graph(&[]),
        Err(SewingError::OverlappingSelfSeam { .. })
    ));
}
#[test]
fn self_seam_disjointness_is_checked_after_merge_instead_of_by_edge_name() {
    let mut f = fixture();
    let source = f.span().a.range.edge();
    let other = f
        .pieces
        .first()
        .unwrap()
        .definition()
        .construction_lines
        .first()
        .unwrap()
        .edge;
    f.span_mut().b = SeamSide {
        copy: id(200),
        range: EdgeRange::whole(other),
    };
    let merged = f
        .ledger
        .merge(
            &mut f.ids,
            source,
            Length::from_micrometres(3).unwrap(),
            other,
            Length::from_micrometres(7).unwrap(),
        )
        .unwrap();
    assert!(f.graph(&[]).is_ok()); // Held edges resolve to disjoint [0,3/10] / [3/10,1].
    f.span_mut().b.range = EdgeRange::new(merged, t(1, 5), t(1, 2)).unwrap();
    assert!(matches!(
        f.graph(&[]),
        Err(SewingError::OverlappingSelfSeam { .. })
    ));
    f.ledger.reverse(&mut f.ids, merged).unwrap();
    assert!(matches!(
        f.graph(&[]),
        Err(SewingError::OverlappingSelfSeam { .. })
    ));
}
#[test]
fn duplicate_and_colliding_entities_are_typed_refusals() {
    let mut f = fixture();
    f.definition.spans.push(f.span().clone());
    assert_eq!(
        f.graph(&[]),
        Err(SewingError::IdentityCollision { id: id(301) })
    );
    f.definition.spans.pop();
    for colliding in [id(100), id(200), id(300)] {
        f.span_mut().id = colliding;
        assert_eq!(
            f.graph(&[]),
            Err(SewingError::IdentityCollision { id: colliding })
        );
    }
    f.span_mut().id = id(301);
    let mark = f.notch(400, t(1, 2));
    assert_eq!(
        f.graph(&[mark.clone(), mark]),
        Err(SewingError::IdentityCollision { id: id(400) })
    );
    f.definition.id = id(100);
    assert_eq!(
        f.graph(&[]),
        Err(SewingError::IdentityCollision { id: id(100) })
    );
}
#[test]
fn absent_copy_and_stale_cut_quantity_name_the_failed_reference() {
    let mut f = fixture();
    f.span_mut().a.copy = id(999);
    assert_eq!(
        f.graph(&[]),
        Err(SewingError::MissingCopy {
            span: id(301),
            side: SeamSideId::A,
            copy: id(999)
        })
    );
    f.span_mut().a.copy = id(200);
    let mut definition = f.pieces.first().unwrap().definition().clone();
    definition.quantity = 4;
    *f.pieces.first_mut().unwrap() = Piece::new(definition, &f.ledger).unwrap();
    assert_eq!(
        f.graph(&[]),
        Err(SewingError::InvalidCutPlan(
            CutPlanError::QuantityMismatch {
                piece: id(100),
                expected: 4,
                actual: 2
            }
        ))
    );
}
#[test]
fn live_foreign_remainder_of_a_merged_edge_is_refused_after_reversal() {
    let mut f = fixture();
    let owned = f.span().a.range.edge();
    let foreign = *f
        .ledger
        .declare_edges(&mut f.ids, 1)
        .unwrap()
        .first()
        .unwrap();
    let merged = f
        .ledger
        .merge(
            &mut f.ids,
            owned,
            Length::from_micrometres(3).unwrap(),
            foreign,
            Length::from_micrometres(7).unwrap(),
        )
        .unwrap();
    f.span_mut().a.range = EdgeRange::new(merged, Param::START, t(1, 5)).unwrap();
    assert!(f.graph(&[]).is_ok());
    f.span_mut().a.range = EdgeRange::new(merged, t(1, 5), t(4, 5)).unwrap();
    assert_eq!(
        f.graph(&[]),
        Err(SewingError::RangeOutsidePiece {
            span: id(301),
            side: SeamSideId::A,
            piece: id(100)
        })
    );
    f.ledger.reverse(&mut f.ids, merged).unwrap();
    assert!(matches!(
        f.graph(&[]),
        Err(SewingError::RangeOutsidePiece { .. })
    ));
}
#[test]
fn piece_interval_union_cannot_hide_an_unowned_interior_between_owned_endpoints() {
    let mut f = fixture();
    let first = f.span().a.range.edge();
    let last = f
        .pieces
        .first()
        .unwrap()
        .definition()
        .construction_lines
        .first()
        .unwrap()
        .edge;
    let foreign = *f
        .ledger
        .declare_edges(&mut f.ids, 1)
        .unwrap()
        .first()
        .unwrap();
    let two = f
        .ledger
        .merge(
            &mut f.ids,
            first,
            Length::from_micrometres(1).unwrap(),
            foreign,
            Length::from_micrometres(1).unwrap(),
        )
        .unwrap();
    let three = f
        .ledger
        .merge(
            &mut f.ids,
            two,
            Length::from_micrometres(2).unwrap(),
            last,
            Length::from_micrometres(1).unwrap(),
        )
        .unwrap();
    f.span_mut().a.range = EdgeRange::whole(three);
    assert_eq!(
        f.graph(&[]),
        Err(SewingError::RangeOutsidePiece {
            span: id(301),
            side: SeamSideId::A,
            piece: id(100)
        })
    );
}
#[test]
fn editing_a_valid_graph_exposes_interior_repairs_and_point_choices_without_rewriting() {
    let mut f = fixture();
    let graph = f.graph(&[]).unwrap();
    let held = f.span().a.range;
    let first = f.ledger.split(&mut f.ids, held.edge(), t(1, 3)).unwrap();
    let second = f.ledger.split(&mut f.ids, first.second(), t(1, 2)).unwrap();
    f.ledger.delete(&mut f.ids, second.first()).unwrap();
    let resolution = graph
        .spans()
        .first()
        .unwrap()
        .resolve_side(SeamSideId::A, &f.ledger);
    assert!(resolution.start().resolved().is_some());
    assert!(resolution.end().resolved().is_some());
    assert!(!resolution.has_full_coverage());
    assert_eq!(resolution.repairs().count(), 1);
    assert_eq!(graph.spans().first().unwrap().definition(), f.span());
    assert!(
        matches!(f.graph(&[]),Err(SewingError::UnresolvedRange {evidence,..}) if evidence==Box::new(resolution))
    );
    let mut f = fixture();
    let held = f.span().a.range;
    f.ledger.split(&mut f.ids, held.edge(), t(1, 2)).unwrap();
    f.span_mut().a.range = EdgeRange::new(held.edge(), t(1, 2), Param::END).unwrap();
    assert!(
        matches!(f.graph(&[]),Err(SewingError::UnresolvedRange {evidence,..}) if evidence.has_full_coverage() && evidence.start().resolved().is_none())
    );
}
#[test]
fn new_unknown_range_is_a_visible_source_failure_not_passthrough() {
    let mut f = fixture();
    let foreign = sc_core::ontology::EdgeRef::new(id(999), sc_core::ontology::LocalTag::FIRST);
    f.span_mut().a.range = EdgeRange::whole(foreign);
    assert!(
        matches!(f.graph(&[]),Err(SewingError::UnresolvedRange {evidence,..}) if matches!(evidence.repairs().next().unwrap().issue(),sc_core::ontology::RangeIssue::UnknownSource))
    );
}
#[test]
fn semantic_notches_and_turn_points_are_valid_stops_on_an_explicit_side() {
    let mut f = fixture();
    let notch = f.notch(400, t(1, 3));
    let turn = SewingLandmark::Turn(
        TurnPoint::new(
            id(401),
            EdgeAnchor {
                edge: f.span().a.range.edge(),
                param: t(2, 3),
            },
            f.pieces.first().unwrap(),
            &f.ledger,
        )
        .unwrap(),
    );
    f.span_mut().stops = vec![
        StopLandmark {
            side: SeamSideId::A,
            landmark: id(400),
        },
        StopLandmark {
            side: SeamSideId::A,
            landmark: id(401),
        },
    ];
    let SewingLandmark::Turn(turn_point) = &turn else {
        panic!("turn registry entry")
    };
    assert_eq!(
        turn_point.geometric_validation(),
        GeometricValidation::DeferredToG2
    );
    assert_eq!(turn_point.anchor().param, t(2, 3));
    let marks = [notch, turn];
    assert!(f.graph(&marks).is_ok());
    let stop = *f.span().stops.first().unwrap();
    f.span_mut().stops.push(stop);
    assert_eq!(
        f.graph(&marks),
        Err(SewingError::DuplicateStop {
            span: id(301),
            stop
        })
    );
    f.span_mut().stops.pop();
    f.span_mut().stops.first_mut().unwrap().side = SeamSideId::B;
    assert!(matches!(
        f.graph(&marks),
        Err(SewingError::StopOutsideSide { .. })
    ));
}
#[test]
fn missing_outside_and_orphaned_stops_are_distinguished() {
    let mut f = fixture();
    let stop = StopLandmark {
        side: SeamSideId::A,
        landmark: id(400),
    };
    f.span_mut().stops.push(stop);
    assert_eq!(
        f.graph(&[]),
        Err(SewingError::MissingLandmark {
            span: id(301),
            stop
        })
    );
    let notch = f.notch(400, t(3, 4));
    f.span_mut().a.range = EdgeRange::new(f.span().a.range.edge(), Param::START, t(1, 2)).unwrap();
    assert_eq!(
        f.graph(std::slice::from_ref(&notch)),
        Err(SewingError::StopOutsideSide {
            span: id(301),
            stop
        })
    );
    let edge = f.span().a.range.edge();
    let split = f.ledger.split(&mut f.ids, edge, t(1, 2)).unwrap();
    f.ledger.delete(&mut f.ids, split.second()).unwrap();
    // The span ends at the split point, so choose the live first fragment for a born-unique endpoint.
    f.span_mut().a.range = EdgeRange::whole(split.first());
    assert!(matches!(
        f.graph(&[notch]),
        Err(SewingError::UnresolvedStop { .. })
    ));
}
#[test]
fn weighted_regions_have_positive_density_and_explicit_order_without_defaulting() {
    let mut f = fixture();
    let region = WeightedEaseRegion {
        from: t(1, 4),
        to: t(3, 4),
        weight: Rational::new(3, 2).unwrap(),
    };
    f.span_mut().ease.distribution = EaseDistribution::Weighted {
        side: SeamSideId::B,
        regions: vec![region],
    };
    assert!(f.graph(&[]).is_ok());
    for (regions, expected) in [
        (vec![], (None, WeightIssue::Empty)),
        (
            vec![WeightedEaseRegion {
                to: region.from,
                ..region
            }],
            (Some(0), WeightIssue::InvalidBounds),
        ),
        (
            vec![WeightedEaseRegion {
                weight: Rational::ZERO,
                ..region
            }],
            (Some(0), WeightIssue::NonPositive),
        ),
        (
            vec![region, region],
            (Some(1), WeightIssue::UnorderedOrOverlapping),
        ),
    ] {
        f.span_mut().ease.distribution = EaseDistribution::Weighted {
            side: SeamSideId::A,
            regions,
        };
        assert_eq!(
            f.graph(&[]),
            Err(SewingError::InvalidWeights {
                span: id(301),
                region: expected.0,
                issue: expected.1
            })
        );
    }
}
#[test]
fn between_notches_requires_two_distinct_noncoincident_notch_stops_on_its_side() {
    let mut f = fixture();
    let a = f.notch(400, t(1, 4));
    let b = f.notch(401, t(3, 4));
    let marks = [a, b];
    f.span_mut().stops = vec![
        StopLandmark {
            side: SeamSideId::A,
            landmark: id(400),
        },
        StopLandmark {
            side: SeamSideId::A,
            landmark: id(401),
        },
    ];
    f.span_mut().ease.distribution = EaseDistribution::BetweenNotches {
        side: SeamSideId::A,
        start: id(400),
        end: id(401),
    };
    assert!(f.graph(&marks).is_ok());
    f.span_mut().ease.distribution = EaseDistribution::BetweenNotches {
        side: SeamSideId::B,
        start: id(400),
        end: id(401),
    };
    assert!(matches!(
        f.graph(&marks),
        Err(SewingError::InvalidEaseAnchors {
            side: SeamSideId::B,
            ..
        })
    ));
    f.span_mut().ease.distribution = EaseDistribution::BetweenNotches {
        side: SeamSideId::A,
        start: id(400),
        end: id(400),
    };
    assert!(matches!(
        f.graph(&marks),
        Err(SewingError::InvalidEaseAnchors { .. })
    ));
    f.span_mut().ease.distribution = EaseDistribution::BetweenNotches {
        side: SeamSideId::A,
        start: id(400),
        end: id(401),
    };
    let coincident = [f.notch(400, t(1, 4)), f.notch(401, t(1, 4))];
    assert!(matches!(
        f.graph(&coincident),
        Err(SewingError::InvalidEaseAnchors { .. })
    ));
    let turn = SewingLandmark::Turn(
        TurnPoint::new(
            id(401),
            EdgeAnchor {
                edge: f.span().a.range.edge(),
                param: t(3, 4),
            },
            f.pieces.first().unwrap(),
            &f.ledger,
        )
        .unwrap(),
    );
    assert!(matches!(
        f.graph(&[marks.first().unwrap().clone(), turn]),
        Err(SewingError::InvalidEaseAnchors { .. })
    ));
}
#[test]
fn removed_copy_and_landmark_targets_remain_missing_without_automatic_transfer() {
    let mut f = fixture();
    let notch = f.notch(400, t(1, 2));
    f.span_mut().stops.push(StopLandmark {
        side: SeamSideId::A,
        landmark: id(400),
    });
    let graph = f.graph(std::slice::from_ref(&notch)).unwrap();
    let replacements = f
        .plan
        .copies()
        .iter()
        .map(|copy| {
            let mut input = *copy.definition();
            if input.id == id(200) {
                input.id = id(203);
            }
            input
        })
        .collect();
    let replacement = CutPlan::new(replacements, &f.pieces).unwrap();
    assert_eq!(
        graph.missing_copies(&replacement).collect::<Vec<_>>(),
        vec![id(200)]
    );
    assert!(graph.missing_copies(&f.plan).next().is_none());
    assert_eq!(
        graph.missing_landmarks(&[]).collect::<Vec<_>>(),
        vec![id(400)]
    );
    assert!(graph.missing_landmarks(&[notch]).next().is_none());
    assert_eq!(graph.spans().first().unwrap().definition().a.copy, id(200));
}
#[test]
fn empty_graph_is_structurally_valid_without_asserting_assembly_completeness() {
    let mut f = fixture();
    f.definition.spans.clear();
    let graph = f.graph(&[]).unwrap();
    assert!(graph.spans().is_empty());
    assert_eq!(graph.ease_validation(), EaseValidation::DeferredToG2AndG3);
}

#[test]
fn reversal_reports_each_sides_direction_without_rewriting_correspondence() {
    let mut f = fixture();
    let graph = f.graph(&[]).unwrap();
    let held_a = f.span().a.range;
    let held_b = f.span().b.range;
    f.ledger.reverse(&mut f.ids, held_a.edge()).unwrap();
    f.ledger.reverse(&mut f.ids, held_b.edge()).unwrap();
    let span = graph.spans().first().unwrap();
    for side in [SeamSideId::A, SeamSideId::B] {
        let result = span.resolve_side(side, &f.ledger);
        assert!(result.has_full_coverage());
        assert_eq!(result.start().resolved().unwrap().param(), Param::END);
        assert_eq!(
            result.start().resolved().unwrap().direction(),
            Direction::Reversed
        );
    }
    assert_eq!(span.definition(), f.span());
    assert_eq!(span.definition().direction, SeamDirection::Opposite);
}
