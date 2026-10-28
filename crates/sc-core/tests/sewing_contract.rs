//! Sewing topology/intent tests; no fixture pretends to measure geometry or certify seam lengths.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
use sc_core::command::{CommandSeamSplitBindingError as SplitError, SplitSeamSpan};
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
        }
        .into(),
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
fn source_cut_binding_borrows_actual_span_and_retains_both_original_frames() {
    let mut f = fixture();
    f.span_mut().a.range = EdgeRange::new(f.span().a.range.edge(), t(1, 7), t(6, 7)).unwrap();
    f.span_mut().b.range = EdgeRange::new(f.span().b.range.edge(), t(1, 5), t(4, 5)).unwrap();
    f.span_mut().ease.declared_mut().unwrap().amount =
        EaseAmount::Profile(ProfileParameterRef::new(id(999)));
    f.span_mut().ease.declared_mut().unwrap().distribution = EaseDistribution::Weighted {
        side: SeamSideId::B,
        regions: vec![WeightedEaseRegion {
            from: Param::START,
            to: t(2, 3),
            weight: Rational::new(3, 2).unwrap(),
        }],
    };
    let mark = f.notch(500, t(1, 3));
    f.span_mut().stops.push(StopLandmark {
        side: SeamSideId::A,
        landmark: mark.id(),
    });
    let graph = f.graph(&[mark]).unwrap();
    let source = graph.spans().first().unwrap();
    let request = SplitSeamSpan::new(
        source.id(),
        EdgeAnchor {
            edge: source.definition().a.range.edge(),
            param: t(2, 3),
        },
        EdgeAnchor {
            edge: source.definition().b.range.edge(),
            param: t(3, 7),
        },
    );
    let binding = request.bind_source_cuts(&graph).unwrap();
    assert!(std::ptr::eq(binding.request(), &request));
    assert!(std::ptr::eq(binding.span(), source));
    assert_eq!(binding.span().definition(), f.span());
    let [lower_a, upper_a] = binding.ranges(SeamSideId::A);
    let [lower_b, upper_b] = binding.ranges(SeamSideId::B);
    assert_eq!(
        (lower_a.from(), lower_a.to(), upper_a.to()),
        (t(1, 7), t(2, 3), t(6, 7))
    );
    assert_eq!(
        (lower_b.from(), lower_b.to(), upper_b.to()),
        (t(1, 5), t(3, 7), t(4, 5))
    );
    assert_eq!(upper_a.from(), lower_a.to());
    assert_eq!(upper_b.from(), lower_b.to());
    assert_eq!(
        binding.span().definition().direction,
        SeamDirection::Opposite
    );
    assert_eq!(
        graph.geometric_validation(),
        GeometricValidation::DeferredToG2
    );
    assert_eq!(graph.ease_validation(), EaseValidation::DeferredToG2AndG3);
}

#[test]
fn source_cut_binding_checks_target_then_a_then_b_and_retains_original_causes() {
    let f = fixture();
    let graph = f.graph(&[]).unwrap();
    let a = EdgeAnchor {
        edge: f.span().a.range.edge(),
        param: t(1, 2),
    };
    let b = EdgeAnchor {
        edge: f.span().b.range.edge(),
        param: t(1, 3),
    };
    let missing = SplitSeamSpan::new(id(998), b, a);
    assert_eq!(
        missing.bind_source_cuts(&graph).unwrap_err(),
        SplitError::MissingSpan { span: id(998) }
    );
    let wrong_a = SplitSeamSpan::new(f.span().id, b, a);
    let error = wrong_a.bind_source_cuts(&graph).unwrap_err();
    assert_eq!(
        error,
        SplitError::SourceFrame {
            span: f.span().id,
            side: SeamSideId::A,
            expected: a.edge,
            cut: b,
        }
    );
    assert!(std::error::Error::source(&error).is_none());
    let wrong_b = SplitSeamSpan::new(f.span().id, a, a);
    assert_eq!(
        wrong_b.bind_source_cuts(&graph).unwrap_err(),
        SplitError::SourceFrame {
            span: f.span().id,
            side: SeamSideId::B,
            expected: b.edge,
            cut: a,
        }
    );
    let endpoint_a = EdgeAnchor {
        param: Param::START,
        ..a
    };
    let both_refuse = SplitSeamSpan::new(f.span().id, endpoint_a, a);
    let error = both_refuse.bind_source_cuts(&graph).unwrap_err();
    let SplitError::Interior { span, side, issue } = error else {
        unreachable!("actual interior cause")
    };
    assert_eq!(
        (span, side, issue.range, issue.at),
        (f.span().id, SeamSideId::A, f.span().a.range, Param::START)
    );
    assert_eq!(
        std::error::Error::source(&error)
            .unwrap()
            .downcast_ref::<sc_core::ontology::RangeSplitError>(),
        Some(&issue)
    );
    let endpoint_b = EdgeAnchor {
        param: Param::END,
        ..b
    };
    let wrong_b = SplitSeamSpan::new(f.span().id, a, endpoint_b);
    let SplitError::Interior { span, side, issue } = wrong_b.bind_source_cuts(&graph).unwrap_err()
    else {
        unreachable!("actual B interior cause")
    };
    assert_eq!(
        (span, side, issue.range, issue.at),
        (f.span().id, SeamSideId::B, f.span().b.range, Param::END)
    );
}

#[test]
fn source_cut_binding_is_an_immutable_supplied_snapshot_not_a_future_decision() {
    let mut f = fixture();
    let graph = f.graph(&[]).unwrap();
    let request = SplitSeamSpan::new(
        f.span().id,
        EdgeAnchor {
            edge: f.span().a.range.edge(),
            param: t(1, 4),
        },
        EdgeAnchor {
            edge: f.span().b.range.edge(),
            param: t(1, 3),
        },
    );
    let original = request.bind_source_cuts(&graph).unwrap();
    f.span_mut().a.copy = id(201);
    f.span_mut().a.range = EdgeRange::new(f.span().a.range.edge(), t(1, 4), t(3, 4)).unwrap();
    let replacement = f.graph(&[]).unwrap();
    let SplitError::Interior { span, side, issue } =
        request.bind_source_cuts(&replacement).unwrap_err()
    else {
        unreachable!("replacement boundary must be rechecked")
    };
    assert_eq!(
        (span, side, issue.range, issue.at),
        (f.span().id, SeamSideId::A, f.span().a.range, t(1, 4))
    );
    assert_eq!(original.span().definition().a.copy, id(200));
    assert_eq!(
        original.ranges(SeamSideId::A).first().unwrap().from(),
        Param::START
    );
    assert_eq!(original.request(), &request);
}

#[test]
fn source_cut_binding_keeps_held_evidence_when_actual_current_resolution_refuses() {
    let mut f = fixture();
    let graph = f.graph(&[]).unwrap();
    let request = SplitSeamSpan::new(
        f.span().id,
        EdgeAnchor {
            edge: f.span().a.range.edge(),
            param: t(1, 2),
        },
        EdgeAnchor {
            edge: f.span().b.range.edge(),
            param: t(1, 3),
        },
    );
    let binding = request.bind_source_cuts(&graph).unwrap();
    let edge = f.span().a.range.edge();
    f.ledger.delete(&mut f.ids, edge).unwrap();
    assert!(graph
        .validate_current(&f.plan, &f.pieces, &[], &f.ledger)
        .is_err());
    let held = request.bind_source_cuts(&graph).unwrap();
    assert!(std::ptr::eq(held.span(), binding.span()));
    assert_eq!(held.ranges(SeamSideId::A), binding.ranges(SeamSideId::A));
    assert!(!held
        .span()
        .resolve_side(SeamSideId::A, &f.ledger)
        .has_full_coverage());
}

#[test]
fn source_interval_pairs_follow_endpoint_direction_and_keep_physical_copies_distinct() {
    for (a_copy, b_copy, same_pattern_edge) in
        [(200, 202, false), (201, 202, false), (200, 201, true)]
    {
        for direction in [SeamDirection::Same, SeamDirection::Opposite] {
            let mut f = fixture();
            f.span_mut().a.copy = id(a_copy);
            f.span_mut().b.copy = id(b_copy);
            f.span_mut().direction = direction;
            f.span_mut().a.range =
                EdgeRange::new(f.span().a.range.edge(), t(1, 7), t(6, 7)).unwrap();
            let b_edge = if same_pattern_edge {
                f.span().a.range.edge()
            } else {
                f.span().b.range.edge()
            };
            f.span_mut().b.range = EdgeRange::new(b_edge, t(1, 5), t(4, 5)).unwrap();
            let graph = f.graph(&[]).unwrap();
            let request = SplitSeamSpan::new(
                f.span().id,
                EdgeAnchor {
                    edge: f.span().a.range.edge(),
                    param: t(2, 3),
                },
                EdgeAnchor {
                    edge: f.span().b.range.edge(),
                    param: t(3, 7),
                },
            );
            let binding = request.bind_source_cuts(&graph).unwrap();
            let [first, second] = binding.paired_intervals();
            for pair in [first, second] {
                assert_eq!(pair.source_span(), f.span().id);
                assert_eq!(pair.direction(), direction);
                assert_eq!(pair.side(SeamSideId::A).copy, id(a_copy));
                assert_eq!(pair.side(SeamSideId::B).copy, id(b_copy));
                assert_eq!(
                    pair.side(SeamSideId::A).range.edge(),
                    f.span().a.range.edge()
                );
                assert_eq!(
                    pair.side(SeamSideId::B).range.edge(),
                    f.span().b.range.edge()
                );
            }
            let bounds = |pair: sc_core::command::CommandSeamSplitPair, side| {
                let range = pair.side(side).range;
                (range.from(), range.to())
            };
            assert_eq!(
                [bounds(first, SeamSideId::A), bounds(second, SeamSideId::A)],
                [(t(1, 7), t(2, 3)), (t(2, 3), t(6, 7))]
            );
            let expected_b = match direction {
                SeamDirection::Same => [(t(1, 5), t(3, 7)), (t(3, 7), t(4, 5))],
                SeamDirection::Opposite => [(t(3, 7), t(4, 5)), (t(1, 5), t(3, 7))],
            };
            assert_eq!(
                [bounds(first, SeamSideId::B), bounds(second, SeamSideId::B)],
                expected_b
            );
            assert_eq!(binding.span().definition(), f.span());
        }
    }
}

#[test]
fn structural_child_graph_acceptance_is_not_parent_ease_conservation() {
    let mut f = fixture();
    f.span_mut().ease.declared_mut().unwrap().amount =
        EaseAmount::Explicit(Length::from_micrometres(10).unwrap());
    let graph = f.graph(&[]).unwrap();
    let request = SplitSeamSpan::new(
        f.span().id,
        EdgeAnchor {
            edge: f.span().a.range.edge(),
            param: t(1, 2),
        },
        EdgeAnchor {
            edge: f.span().b.range.edge(),
            param: t(1, 2),
        },
    );
    let binding = request.bind_source_cuts(&graph).unwrap();
    let [first, second] = binding.paired_intervals();
    let child = |pair: sc_core::command::CommandSeamSplitPair, identity| {
        let mut definition = binding.span().definition().clone();
        definition.id = identity;
        definition.a = pair.side(SeamSideId::A);
        definition.b = pair.side(SeamSideId::B);
        definition.direction = pair.direction();
        definition
    };
    let mut declarations = vec![child(first, id(302)), child(second, id(303))];
    let build = |spans| {
        SewingGraph::new(
            SewingGraphDefinition { id: id(300), spans },
            &f.plan,
            &f.pieces,
            &[],
            &f.ledger,
        )
        .unwrap()
    };
    let total = |candidate: &SewingGraph| {
        candidate
            .spans()
            .iter()
            .try_fold(Length::ZERO, |sum, span| {
                let EaseAmount::Explicit(amount) = span.definition().ease.declaration().amount
                else {
                    unreachable!("actual explicit diagnostic fixture")
                };
                sum.checked_add(amount)
            })
            .unwrap()
    };
    // These are independently supplied child declarations, not a result published by a handler.
    // The structural constructor has no original-parent relation from which to certify conservation.
    let copied = build(declarations.clone());
    assert_eq!(total(&copied), Length::from_micrometres(20).unwrap());
    assert_eq!(copied.ease_validation(), EaseValidation::DeferredToG2AndG3);
    for declaration in &mut declarations {
        declaration.ease.declared_mut().unwrap().amount =
            EaseAmount::Explicit(Length::from_micrometres(5).unwrap());
    }
    let conserving = build(declarations);
    assert_eq!(total(&conserving), Length::from_micrometres(10).unwrap());
    assert_eq!(
        conserving.ease_validation(),
        EaseValidation::DeferredToG2AndG3
    );
    assert_eq!(
        binding.span().definition().ease.declaration().amount,
        EaseAmount::Explicit(Length::from_micrometres(10).unwrap())
    );
}

#[test]
fn immutable_content_retains_direction_signed_ease_and_visible_obligations() {
    let mut f = fixture();
    f.span_mut().ease.declared_mut().unwrap().amount =
        EaseAmount::Explicit(Length::from_micrometres(-1500).unwrap());
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
            f.span_mut().ease.declared_mut().unwrap().amount = amount;
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
fn current_cut_plan_refusal_preserves_the_original_standard_error_cause() {
    let mut f = fixture();
    let graph = f.graph(&[]).unwrap();
    let mut definition = f.pieces.first().unwrap().definition().clone();
    definition.quantity = 4;
    *f.pieces.first_mut().unwrap() = Piece::new(definition, &f.ledger).unwrap();
    let error = graph
        .validate_current(&f.plan, &f.pieces, &[], &f.ledger)
        .unwrap_err();
    let original = CutPlanError::QuantityMismatch {
        piece: id(100),
        expected: 4,
        actual: 2,
    };
    assert_eq!(error, SewingError::InvalidCutPlan(original.clone()));
    assert_eq!(
        std::error::Error::source(&error).and_then(|cause| cause.downcast_ref::<CutPlanError>()),
        Some(&original)
    );
    assert!(std::error::Error::source(&original).is_none());
    assert!(std::error::Error::source(&SewingError::IdentityCollision { id: id(300) }).is_none());
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
    f.span_mut().ease.declared_mut().unwrap().distribution = EaseDistribution::Weighted {
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
        f.span_mut().ease.declared_mut().unwrap().distribution = EaseDistribution::Weighted {
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
    f.span_mut().ease.declared_mut().unwrap().distribution = EaseDistribution::BetweenNotches {
        side: SeamSideId::A,
        start: id(400),
        end: id(401),
    };
    assert!(f.graph(&marks).is_ok());
    f.span_mut().ease.declared_mut().unwrap().distribution = EaseDistribution::BetweenNotches {
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
    f.span_mut().ease.declared_mut().unwrap().distribution = EaseDistribution::BetweenNotches {
        side: SeamSideId::A,
        start: id(400),
        end: id(400),
    };
    assert!(matches!(
        f.graph(&marks),
        Err(SewingError::InvalidEaseAnchors { .. })
    ));
    f.span_mut().ease.declared_mut().unwrap().distribution = EaseDistribution::BetweenNotches {
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

#[test]
fn current_graph_rechecks_actual_copy_identity_and_same_id_source_piece_after_plan_replacement() {
    let f = fixture();
    let graph = f.graph(&[]).unwrap();
    let original = graph.clone();
    assert_eq!(
        graph.validate_current(&f.plan, &f.pieces, &[], &f.ledger),
        Ok(())
    );
    let renamed = CutPlan::new(
        f.plan
            .copies()
            .iter()
            .map(|copy| {
                let mut definition = *copy.definition();
                if definition.id == id(200) {
                    definition.id = id(205);
                }
                definition
            })
            .collect(),
        &f.pieces,
    )
    .unwrap();
    assert_eq!(
        graph.validate_current(&renamed, &f.pieces, &[], &f.ledger),
        Err(SewingError::MissingCopy {
            span: id(301),
            side: SeamSideId::A,
            copy: id(200)
        })
    );
    let rebound = CutPlan::new(
        vec![
            CutCopyDefinition {
                id: id(200),
                piece: id(101),
                orientation: CopyOrientation::Authored,
            },
            CutCopyDefinition {
                id: id(201),
                piece: id(100),
                orientation: CopyOrientation::Reflected,
            },
            CutCopyDefinition {
                id: id(202),
                piece: id(100),
                orientation: CopyOrientation::Authored,
            },
        ],
        &f.pieces,
    )
    .unwrap();
    assert!(graph.missing_copies(&rebound).next().is_none());
    assert_eq!(
        graph.validate_current(&rebound, &f.pieces, &[], &f.ledger),
        Err(SewingError::RangeOutsidePiece {
            span: id(301),
            side: SeamSideId::A,
            piece: id(101)
        })
    );
    assert_eq!(graph, original);
}

#[test]
fn current_graph_rechecks_actual_landmark_source_even_when_the_same_stop_identity_is_present() {
    let mut f = fixture();
    let stop = StopLandmark {
        side: SeamSideId::A,
        landmark: id(400),
    };
    f.span_mut().stops.push(stop);
    let original_mark = f.notch(400, t(1, 2));
    let graph = f.graph(std::slice::from_ref(&original_mark)).unwrap();
    let original = graph.clone();
    assert_eq!(
        graph.validate_current(&f.plan, &f.pieces, &[], &f.ledger),
        Err(SewingError::MissingLandmark {
            span: id(301),
            stop
        })
    );
    let replacement = SewingLandmark::Turn(
        TurnPoint::new(
            id(400),
            EdgeAnchor {
                edge: f.span().b.range.edge(),
                param: t(1, 2),
            },
            f.pieces.get(1).unwrap(),
            &f.ledger,
        )
        .unwrap(),
    );
    assert!(graph
        .missing_landmarks(std::slice::from_ref(&replacement))
        .next()
        .is_none());
    assert_eq!(
        graph.validate_current(&f.plan, &f.pieces, &[replacement], &f.ledger),
        Err(SewingError::StopOutsideSide {
            span: id(301),
            stop
        })
    );
    assert_eq!(graph, original);
}

#[test]
fn current_graph_keeps_whole_lost_interior_evidence_after_previous_construction_success() {
    let mut f = fixture();
    let graph = f.graph(&[]).unwrap();
    let original = graph.clone();
    let held = f.span().a.range;
    let first = f.ledger.split(&mut f.ids, held.edge(), t(1, 3)).unwrap();
    let second = f.ledger.split(&mut f.ids, first.second(), t(1, 2)).unwrap();
    f.ledger.delete(&mut f.ids, second.first()).unwrap();
    let issue = graph
        .validate_current(&f.plan, &f.pieces, &[], &f.ledger)
        .unwrap_err();
    let SewingError::UnresolvedRange {
        span,
        side,
        evidence,
    } = issue
    else {
        panic!("original whole-range refusal");
    };
    assert_eq!((span, side), (id(301), SeamSideId::A));
    assert_eq!(evidence.held(), held);
    assert_eq!(evidence.portions().len(), 3);
    assert_eq!(evidence.repairs().count(), 1);
    assert!(evidence.start().resolved().is_some() && evidence.end().resolved().is_some());
    assert_eq!(graph, original);
}

#[test]
fn uniform_ease_matches_independent_signed_cumulative_oracle_in_both_directions() {
    let mut rows = 0;
    for row in include_str!("fixtures/uniform_ease.tsv")
        .lines()
        .filter(|row| !row.starts_with('#'))
    {
        let fields = row
            .split('\t')
            .map(|value| value.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let [parent, n, d, first, second] = fields.as_slice() else {
            panic!("five oracle columns")
        };
        let progress = Rational::new(*n, *d).unwrap();
        for direction in [SeamDirection::Same, SeamDirection::Opposite] {
            let mut f = fixture();
            f.span_mut().direction = direction;
            f.span_mut().ease.declared_mut().unwrap().amount =
                EaseAmount::Explicit(Length::from_micrometres(*parent).unwrap());
            let b = match direction {
                SeamDirection::Same => progress,
                SeamDirection::Opposite => progress.one_minus().unwrap(),
            };
            let request = SplitSeamSpan::new(
                id(301),
                EdgeAnchor {
                    edge: f.span().a.range.edge(),
                    param: Param::new(progress).unwrap(),
                },
                EdgeAnchor {
                    edge: f.span().b.range.edge(),
                    param: Param::new(b).unwrap(),
                },
            );
            let graph = f.graph(&[]).unwrap();
            let binding = request.bind_source_cuts(&graph).unwrap();
            let allocation = binding.uniform_explicit_ease().unwrap();
            assert!(std::ptr::eq(allocation.request(), &request));
            assert!(std::ptr::eq(
                allocation.span(),
                graph.spans().first().unwrap()
            ));
            assert_eq!(allocation.cut_progress(), progress);
            assert_eq!(
                (
                    allocation.first_amount().as_micrometres(),
                    allocation.second_amount().as_micrometres()
                ),
                (*first, *second)
            );
            assert_eq!(
                allocation
                    .first_amount()
                    .checked_add(allocation.second_amount()),
                Ok(allocation.parent_amount())
            );
            let [first_pair, second_pair] = binding.paired_intervals();
            assert_eq!(
                allocation.amount_in_a(first_pair.side(SeamSideId::A).range),
                Ok(allocation.first_amount())
            );
            assert_eq!(
                allocation.amount_in_a(second_pair.side(SeamSideId::A).range),
                Ok(allocation.second_amount())
            );
            assert_eq!(allocation.cumulative_at_a(Param::START), Ok(Length::ZERO));
            assert_eq!(
                allocation.cumulative_at_a(Param::END),
                Ok(allocation.parent_amount())
            );
            assert_eq!(
                allocation.amount_in_a(f.span().a.range),
                Ok(allocation.parent_amount())
            );
        }
        rows += 1;
    }
    assert_eq!(rows, 35);
    println!("independent uniform-ease oracle: {rows} rows, both declared directions");
}

#[test]
fn uniform_ease_original_root_queries_are_split_order_independent() {
    for parent in [1, -1] {
        for direction in [SeamDirection::Same, SeamDirection::Opposite] {
            let mut f = fixture();
            let a_edge = f.span().a.range.edge();
            let b_edge = f.span().b.range.edge();
            f.span_mut().a.range = EdgeRange::new(a_edge, t(1, 7), t(6, 7)).unwrap();
            f.span_mut().b.range = EdgeRange::new(b_edge, t(1, 5), t(4, 5)).unwrap();
            f.span_mut().direction = direction;
            f.span_mut().ease.declared_mut().unwrap().amount =
                EaseAmount::Explicit(Length::from_micrometres(parent).unwrap());
            let graph = f.graph(&[]).unwrap();
            let mut observed = Vec::new();
            for (a_cut, same_b, opposite_b) in
                [(t(8, 21), t(2, 5), t(3, 5)), (t(13, 21), t(3, 5), t(2, 5))]
            {
                let request = SplitSeamSpan::new(
                    id(301),
                    EdgeAnchor {
                        edge: a_edge,
                        param: a_cut,
                    },
                    EdgeAnchor {
                        edge: b_edge,
                        param: match direction {
                            SeamDirection::Same => same_b,
                            SeamDirection::Opposite => opposite_b,
                        },
                    },
                );
                let allocation = request
                    .bind_source_cuts(&graph)
                    .unwrap()
                    .uniform_explicit_ease()
                    .unwrap();
                let amounts = [
                    (t(1, 7), t(8, 21)),
                    (t(8, 21), t(13, 21)),
                    (t(13, 21), t(6, 7)),
                ]
                .map(|(from, to)| {
                    allocation
                        .amount_in_a(EdgeRange::new(a_edge, from, to).unwrap())
                        .unwrap()
                        .as_micrometres()
                });
                assert_eq!(amounts, [0, parent, 0]);
                observed.push(amounts);
            }
            assert_eq!(observed.first(), observed.get(1));
        }
    }
}

#[test]
fn uniform_ease_retains_actual_eligibility_sources_without_fabricating_values() {
    use sc_core::command::CommandUniformEaseError as Error;
    use std::error::Error as _;
    let mut f = fixture();
    let request = SplitSeamSpan::new(
        id(301),
        EdgeAnchor {
            edge: f.span().a.range.edge(),
            param: t(1, 2),
        },
        EdgeAnchor {
            edge: f.span().b.range.edge(),
            param: t(1, 2),
        },
    );
    for amount in [
        EaseAmount::Formula(id(710)),
        EaseAmount::Profile(ProfileParameterRef::new(id(711))),
    ] {
        f.span_mut().ease.declared_mut().unwrap().amount = amount;
        let graph = f.graph(&[]).unwrap();
        let error = request
            .bind_source_cuts(&graph)
            .unwrap()
            .uniform_explicit_ease()
            .unwrap_err();
        assert_eq!(
            error,
            Error::Amount {
                span: id(301),
                amount
            }
        );
        assert!(error.source().is_none());
    }
    f.span_mut().ease.declared_mut().unwrap().distribution = EaseDistribution::Weighted {
        side: SeamSideId::B,
        regions: vec![WeightedEaseRegion {
            from: Param::START,
            to: Param::END,
            weight: Rational::ONE,
        }],
    };
    let graph = f.graph(&[]).unwrap();
    assert_eq!(
        request
            .bind_source_cuts(&graph)
            .unwrap()
            .uniform_explicit_ease()
            .unwrap_err(),
        Error::Weighted {
            span: id(301),
            side: SeamSideId::B
        }
    );
    let marks = [f.notch(720, t(1, 4)), f.notch(721, t(3, 4))];
    f.span_mut().stops = vec![
        StopLandmark {
            side: SeamSideId::A,
            landmark: id(720),
        },
        StopLandmark {
            side: SeamSideId::A,
            landmark: id(721),
        },
    ];
    f.span_mut().ease.declared_mut().unwrap().distribution = EaseDistribution::BetweenNotches {
        side: SeamSideId::A,
        start: id(720),
        end: id(721),
    };
    let graph = f.graph(&marks).unwrap();
    assert_eq!(
        request
            .bind_source_cuts(&graph)
            .unwrap()
            .uniform_explicit_ease()
            .unwrap_err(),
        Error::BetweenNotches {
            span: id(301),
            side: SeamSideId::A,
            start: id(720),
            end: id(721)
        }
    );
}

#[test]
fn uniform_ease_retains_directed_cut_mismatch_and_original_numeric_chain() {
    use sc_core::command::CommandUniformEaseError as Error;
    use sc_core::ontology::RangeProgressError;
    use std::error::Error as _;
    for direction in [SeamDirection::Same, SeamDirection::Opposite] {
        let mut f = fixture();
        f.span_mut().direction = direction;
        let request = SplitSeamSpan::new(
            id(301),
            EdgeAnchor {
                edge: f.span().a.range.edge(),
                param: t(1, 3),
            },
            EdgeAnchor {
                edge: f.span().b.range.edge(),
                param: t(1, 2),
            },
        );
        let graph = f.graph(&[]).unwrap();
        let error = request
            .bind_source_cuts(&graph)
            .unwrap()
            .uniform_explicit_ease()
            .unwrap_err();
        assert_eq!(
            error,
            Error::CutProgress {
                span: id(301),
                a: Rational::new(1, 3).unwrap(),
                b: Rational::new(1, 2).unwrap(),
                direction
            }
        );
        assert!(error.source().is_none());
    }
    for side in [SeamSideId::A, SeamSideId::B] {
        let mut f = fixture();
        let maximum = i64::MAX;
        let range =
            EdgeRange::new(f.span().a.range.edge(), t(1, maximum), t(2, maximum - 1)).unwrap();
        let request = SplitSeamSpan::new(
            id(301),
            EdgeAnchor {
                edge: f.span().a.range.edge(),
                param: t(1, 2),
            },
            EdgeAnchor {
                edge: f.span().b.range.edge(),
                param: t(1, 2),
            },
        );
        match side {
            SeamSideId::A => {
                f.span_mut().a.range = range;
            }
            SeamSideId::B => {
                f.span_mut().b.range =
                    EdgeRange::new(f.span().b.range.edge(), range.from(), range.to()).unwrap();
            }
        }
        let request = SplitSeamSpan::new(
            request.span(),
            EdgeAnchor {
                edge: f.span().a.range.edge(),
                param: if side == SeamSideId::A {
                    t(1, maximum - 1)
                } else {
                    request.cut(SeamSideId::A).param
                },
            },
            EdgeAnchor {
                edge: f.span().b.range.edge(),
                param: if side == SeamSideId::B {
                    t(1, maximum - 1)
                } else {
                    request.cut(SeamSideId::B).param
                },
            },
        );
        let graph = f.graph(&[]).unwrap();
        let error = request
            .bind_source_cuts(&graph)
            .unwrap()
            .uniform_explicit_ease()
            .unwrap_err();
        let Error::Progress {
            span,
            side: actual_side,
            issue,
        } = &error
        else {
            panic!("actual normalization cause")
        };
        assert_eq!((*span, *actual_side), (id(301), side));
        let RangeProgressError::Arithmetic {
            range: original,
            at,
            issue: numeric,
        } = issue.as_ref()
        else {
            panic!("reduced progress denominator overflows")
        };
        assert_eq!(
            *original,
            match side {
                SeamSideId::A => f.span().a.range,
                SeamSideId::B => f.span().b.range,
            }
        );
        assert_eq!(*at, t(1, maximum - 1));
        assert_eq!(
            numeric.as_ref(),
            &sc_units::UnitError::Overflow {
                operation: "Rational::checked_relative_progress"
            }
        );
        assert_eq!(error.source().unwrap().to_string(), issue.to_string());
        assert_eq!(
            error.source().unwrap().source().unwrap().to_string(),
            numeric.to_string()
        );
    }
}

#[test]
fn uniform_ease_queries_check_original_frame_and_domain_and_keep_snapshot_source() {
    use sc_core::command::CommandUniformEaseError as Error;
    use sc_core::ontology::RangeProgressError;
    let mut f = fixture();
    let edge = f.span().a.range.edge();
    f.span_mut().a.range = EdgeRange::new(edge, t(1, 7), t(6, 7)).unwrap();
    f.span_mut().ease.declared_mut().unwrap().amount =
        EaseAmount::Explicit(Length::from_micrometres(10).unwrap());
    let request = SplitSeamSpan::new(
        id(301),
        EdgeAnchor {
            edge,
            param: t(1, 2),
        },
        EdgeAnchor {
            edge: f.span().b.range.edge(),
            param: t(1, 2),
        },
    );
    let graph = f.graph(&[]).unwrap();
    let allocation = request
        .bind_source_cuts(&graph)
        .unwrap()
        .uniform_explicit_ease()
        .unwrap();
    let wrong = EdgeRange::whole(f.span().b.range.edge());
    assert_eq!(
        allocation.amount_in_a(wrong),
        Err(Error::IntervalSource {
            span: id(301),
            expected: edge,
            interval: wrong
        })
    );
    for (from, to, at) in [
        (Param::START, Param::END, Param::START),
        (t(1, 2), Param::END, Param::END),
    ] {
        assert_eq!(
            allocation.amount_in_a(EdgeRange::new(edge, from, to).unwrap()),
            Err(Error::Progress {
                span: id(301),
                side: SeamSideId::A,
                issue: Box::new(RangeProgressError::Outside {
                    range: f.span().a.range,
                    at
                })
            })
        );
    }
    assert_eq!(allocation.cumulative_at_a(t(1, 7)), Ok(Length::ZERO));
    assert_eq!(
        allocation.cumulative_at_a(t(6, 7)),
        Ok(allocation.parent_amount())
    );
    f.span_mut().ease.declared_mut().unwrap().amount =
        EaseAmount::Explicit(Length::from_micrometres(-3).unwrap());
    let replacement = f.graph(&[]).unwrap();
    let fresh = request
        .bind_source_cuts(&replacement)
        .unwrap()
        .uniform_explicit_ease()
        .unwrap();
    assert_eq!(
        (
            allocation.parent_amount().as_micrometres(),
            fresh.parent_amount().as_micrometres()
        ),
        (10, -3)
    );
    assert!(std::ptr::eq(
        allocation.span(),
        graph.spans().first().unwrap()
    ));
    assert!(std::ptr::eq(
        fresh.span(),
        replacement.spans().first().unwrap()
    ));
}

#[test]
fn retained_ease_portions_share_original_symbolic_sources_across_both_split_orders() {
    for amount in [
        EaseAmount::Explicit(Length::from_micrometres(1).unwrap()),
        EaseAmount::Formula(id(730)),
        EaseAmount::Profile(ProfileParameterRef::new(id(731))),
    ] {
        for direction in [SeamDirection::Same, SeamDirection::Opposite] {
            for distribution in [
                EaseDistribution::Uniform,
                EaseDistribution::Weighted {
                    side: SeamSideId::B,
                    regions: vec![WeightedEaseRegion {
                        from: t(1, 4),
                        to: t(3, 4),
                        weight: Rational::new(7, 3).unwrap(),
                    }],
                },
                EaseDistribution::BetweenNotches {
                    side: SeamSideId::A,
                    start: id(740),
                    end: id(741),
                },
            ] {
                let mut f = fixture();
                let a = f.span().a.range.edge();
                let b = f.span().b.range.edge();
                f.span_mut().a = SeamSide {
                    copy: id(201),
                    range: EdgeRange::new(a, t(1, 7), t(6, 7)).unwrap(),
                };
                f.span_mut().b.range = EdgeRange::new(b, t(1, 5), t(4, 5)).unwrap();
                f.span_mut().direction = direction;
                f.span_mut().ease = DeclaredEase {
                    amount,
                    distribution,
                }
                .into();
                f.span_mut().stops = vec![
                    StopLandmark {
                        side: SeamSideId::A,
                        landmark: id(740),
                    },
                    StopLandmark {
                        side: SeamSideId::A,
                        landmark: id(741),
                    },
                ];
                let marks = [f.notch(740, t(1, 3)), f.notch(741, t(2, 3))];
                let original = f.span().clone();
                let retained = {
                    let graph = f.graph(&marks).unwrap();
                    graph.spans().first().unwrap().retain_ease_source()
                }; // Retained data has its own immutable owner after the actual graph is gone.
                assert_eq!(retained.source().span(), original.id);
                assert_eq!(retained.source().side(SeamSideId::A), original.a);
                assert_eq!(retained.source().side(SeamSideId::B), original.b);
                assert_eq!(retained.source().direction(), direction);
                assert_eq!(retained.source().declaration(), original.ease.declaration());
                let a1 = EdgeAnchor {
                    edge: a,
                    param: t(8, 21),
                };
                let a2 = EdgeAnchor {
                    edge: a,
                    param: t(13, 21),
                };
                let (b1, b2) = match direction {
                    SeamDirection::Same => (t(2, 5), t(3, 5)),
                    SeamDirection::Opposite => (t(3, 5), t(2, 5)),
                };
                let b1 = EdgeAnchor { edge: b, param: b1 };
                let b2 = EdgeAnchor { edge: b, param: b2 };
                let [first, remainder] = retained.split_at(a1, b1).unwrap();
                let [middle, last] = remainder.split_at(a2, b2).unwrap();
                let left_first = [first, middle, last];
                let [prefix, last] = retained.split_at(a2, b2).unwrap();
                let [first, middle] = prefix.split_at(a1, b1).unwrap();
                let right_first = [first, middle, last];
                assert_eq!(left_first, right_first);
                let expected_a = [
                    (t(1, 7), t(8, 21)),
                    (t(8, 21), t(13, 21)),
                    (t(13, 21), t(6, 7)),
                ];
                let same_b = [(t(1, 5), t(2, 5)), (t(2, 5), t(3, 5)), (t(3, 5), t(4, 5))];
                let opposite_b = [(t(3, 5), t(4, 5)), (t(2, 5), t(3, 5)), (t(1, 5), t(2, 5))];
                for (index, portion) in left_first.iter().enumerate() {
                    let (from, to) = *expected_a.get(index).unwrap();
                    assert_eq!(
                        portion.side(SeamSideId::A),
                        SeamSide {
                            copy: id(201),
                            range: EdgeRange::new(a, from, to).unwrap()
                        }
                    );
                    let (from, to) = *match direction {
                        SeamDirection::Same => same_b.get(index),
                        SeamDirection::Opposite => opposite_b.get(index),
                    }
                    .unwrap();
                    assert_eq!(
                        portion.side(SeamSideId::B),
                        SeamSide {
                            copy: id(202),
                            range: EdgeRange::new(b, from, to).unwrap()
                        }
                    );
                    assert!(std::ptr::eq(portion.source(), retained.source()));
                    assert!(std::ptr::eq(portion.clone().source(), retained.source()));
                    assert_eq!(portion.source().declaration(), original.ease.declaration());
                }
                for portion in &right_first {
                    assert!(std::ptr::eq(portion.source(), retained.source()));
                }
            }
        }
    }
}

#[test]
fn retained_ease_portion_refusals_use_actual_current_intervals_in_a_b_order() {
    use sc_core::ontology::SeamEasePortionError as Error;
    use std::error::Error as _;
    let f = fixture();
    let graph = f.graph(&[]).unwrap();
    let retained = graph.spans().first().unwrap().retain_ease_source();
    let a = EdgeAnchor {
        edge: f.span().a.range.edge(),
        param: t(1, 3),
    };
    let b = EdgeAnchor {
        edge: f.span().b.range.edge(),
        param: t(2, 3),
    };
    let [first, second] = retained.split_at(a, b).unwrap();
    let before = second.clone();
    let bad_b = EdgeAnchor {
        edge: a.edge,
        param: t(1, 2),
    };
    let error = second.split_at(bad_b, bad_b).unwrap_err();
    // This A edge is valid, but its cut is valid too; B's wrong frame is the original refusal.
    assert_eq!(
        error,
        Error::SourceFrame {
            source_span: id(301),
            side: SeamSideId::B,
            expected: b.edge,
            cut: bad_b
        }
    );
    assert!(error.source().is_none());
    let error = second.split_at(b, bad_b).unwrap_err();
    assert_eq!(
        error,
        Error::SourceFrame {
            source_span: id(301),
            side: SeamSideId::A,
            expected: a.edge,
            cut: b
        }
    );
    let endpoint = EdgeAnchor {
        edge: a.edge,
        param: a.param,
    };
    let error = second.split_at(endpoint, bad_b).unwrap_err();
    let issue = sc_core::ontology::RangeSplitError {
        range: second.side(SeamSideId::A).range,
        at: a.param,
    };
    assert_eq!(
        error,
        Error::Interior {
            source_span: id(301),
            side: SeamSideId::A,
            issue
        }
    );
    assert_eq!(error.source().unwrap().to_string(), issue.to_string());
    let interior_a = EdgeAnchor {
        edge: a.edge,
        param: t(1, 2),
    };
    let error = second.split_at(interior_a, b).unwrap_err();
    assert_eq!(
        error,
        Error::Interior {
            source_span: id(301),
            side: SeamSideId::B,
            issue: sc_core::ontology::RangeSplitError {
                range: second.side(SeamSideId::B).range,
                at: b.param
            }
        }
    );
    assert_eq!(second, before);
    assert!(std::ptr::eq(first.source(), second.source()));
}

#[test]
fn retained_ease_source_is_an_owned_snapshot_without_currentness_or_amount_rebinding() {
    let mut f = fixture();
    f.span_mut().ease.declared_mut().unwrap().amount = EaseAmount::Formula(id(750));
    let graph = f.graph(&[]).unwrap();
    let held = graph.spans().first().unwrap().retain_ease_source();
    f.span_mut().ease.declared_mut().unwrap().amount =
        EaseAmount::Profile(ProfileParameterRef::new(id(751)));
    let replacement = f.graph(&[]).unwrap();
    let fresh = replacement.spans().first().unwrap().retain_ease_source();
    assert_eq!(held.source().span(), fresh.source().span());
    assert!(!std::ptr::eq(held.source(), fresh.source()));
    assert_eq!(
        held.source().declaration().amount,
        EaseAmount::Formula(id(750))
    );
    assert_eq!(
        fresh.source().declaration().amount,
        EaseAmount::Profile(ProfileParameterRef::new(id(751)))
    );
    let edge = f.span().a.range.edge();
    f.ledger.delete(&mut f.ids, edge).unwrap();
    assert!(graph
        .validate_current(&f.plan, &f.pieces, &[], &f.ledger)
        .is_err());
    let after_edit = graph.spans().first().unwrap().retain_ease_source();
    assert_eq!(after_edit, held);
    assert_eq!(held.source().side(SeamSideId::A).range.edge(), edge);
}

fn retained_children(f: &Fixture, marks: &[SewingLandmark]) -> Vec<SeamSpanDefinition> {
    let original = f.graph(marks).unwrap();
    let root = original.spans().first().unwrap().retain_ease_source();
    let [first, second] = root
        .split_at(
            EdgeAnchor {
                edge: f.span().a.range.edge(),
                param: t(1, 2),
            },
            EdgeAnchor {
                edge: f.span().b.range.edge(),
                param: t(1, 2),
            },
        )
        .unwrap();
    [first, second]
        .into_iter()
        .zip(302_u128..)
        .map(|(portion, identity)| SeamSpanDefinition {
            id: id(identity),
            a: portion.side(SeamSideId::A),
            b: portion.side(SeamSideId::B),
            direction: portion.source().direction(),
            ease: portion.into(),
            stops: vec![],
        })
        .collect()
}

#[test]
fn graph_retained_ease_sources_are_canonical_and_recapture_flattens_to_the_original() {
    use sc_core::command::CommandUniformEaseError;
    let mut f = fixture();
    f.span_mut().ease.declared_mut().unwrap().amount =
        EaseAmount::Explicit(Length::from_micrometres(1).unwrap());
    f.definition.spans = retained_children(&f, &[]);
    let graph = f.graph(&[]).unwrap();
    assert_eq!(graph.ease_sources().len(), 1);
    let root = graph.ease_sources().next().unwrap();
    assert_eq!(root.span(), id(301));
    assert_eq!(
        root.declaration().amount,
        EaseAmount::Explicit(Length::from_micrometres(1).unwrap())
    );
    for span in graph.spans() {
        assert!(span.definition().ease.portion().is_some());
        let captured = span.retain_ease_source();
        assert!(std::ptr::eq(captured.source(), root));
        assert_eq!(captured.side(SeamSideId::A), span.definition().a);
        assert_eq!(captured.side(SeamSideId::B), span.definition().b);
    }
    let span = graph.spans().first().unwrap();
    let request = SplitSeamSpan::new(
        span.id(),
        EdgeAnchor {
            edge: span.definition().a.range.edge(),
            param: t(1, 4),
        },
        EdgeAnchor {
            edge: span.definition().b.range.edge(),
            param: t(3, 4),
        },
    );
    assert_eq!(
        request
            .bind_source_cuts(&graph)
            .unwrap()
            .uniform_explicit_ease()
            .unwrap_err(),
        CommandUniformEaseError::RetainedPortion {
            span: id(302),
            source_span: id(301)
        }
    );
    assert!(
        matches!(SplitSeamSpan::new(id(301), request.cut(SeamSideId::A), request.cut(SeamSideId::B)).bind_source_cuts(&graph), Err(SplitError::MissingSpan { span }) if span==id(301))
    );
}

#[test]
fn graph_retained_ease_rejects_binding_identity_and_conflicting_root_sources() {
    use sc_core::ontology::SeamEaseBindingIssue;
    use std::error::Error as _;
    let mut f = fixture();
    let children = retained_children(&f, &[]);
    let mut changed = children.clone();
    let child = changed.first_mut().unwrap();
    let expected = child.a;
    child.a.copy = id(201);
    let actual = child.a;
    f.definition.spans = changed;
    let error = f.graph(&[]).unwrap_err();
    let mismatch = SeamEaseBindingIssue::Side {
        side: SeamSideId::A,
        expected,
        actual,
    };
    assert_eq!(
        error,
        SewingError::EasePortionBinding {
            span: id(302),
            source_span: id(301),
            issue: Box::new(mismatch)
        }
    );
    assert_eq!(error.source().unwrap().to_string(), mismatch.to_string());
    f.definition.spans = children.clone();
    f.span_mut().direction = SeamDirection::Same;
    assert_eq!(
        f.graph(&[]).unwrap_err(),
        SewingError::EasePortionBinding {
            span: id(302),
            source_span: id(301),
            issue: Box::new(SeamEaseBindingIssue::Direction {
                expected: SeamDirection::Opposite,
                actual: SeamDirection::Same
            })
        }
    );
    f.definition.spans = children.clone();
    f.span_mut().id = id(301);
    assert_eq!(
        f.graph(&[]).unwrap_err(),
        SewingError::IdentityCollision { id: id(301) }
    );
    // Independently captured same-id roots with different actual declarations cannot coexist.
    let mut replacement = fixture();
    replacement.span_mut().ease.declared_mut().unwrap().amount = EaseAmount::Formula(id(790));
    let others = retained_children(&replacement, &[]);
    f.definition.spans = vec![
        children.first().unwrap().clone(),
        others.get(1).unwrap().clone(),
    ];
    assert_eq!(
        f.graph(&[]).unwrap_err(),
        SewingError::EaseSourceConflict {
            source_span: id(301),
            first_span: id(302),
            span: id(303)
        }
    );
}

#[test]
fn retained_notch_source_rechecks_original_full_context_even_with_child_stop_subsets() {
    use std::error::Error as _;
    let mut f = fixture();
    let marks = [f.notch(800, t(1, 4)), f.notch(801, t(3, 4))];
    let stops = vec![
        StopLandmark {
            side: SeamSideId::A,
            landmark: id(800),
        },
        StopLandmark {
            side: SeamSideId::A,
            landmark: id(801),
        },
    ];
    f.span_mut().stops = stops.clone();
    f.span_mut().ease.declared_mut().unwrap().distribution = EaseDistribution::BetweenNotches {
        side: SeamSideId::A,
        start: id(800),
        end: id(801),
    };
    f.definition.spans = retained_children(&f, &marks);
    let graph = f.graph(&marks).unwrap();
    assert_eq!(graph.ease_sources().next().unwrap().stops(), &stops);
    assert!(graph
        .spans()
        .iter()
        .all(|span| span.definition().stops.is_empty()));
    assert_eq!(
        graph.missing_landmarks(&[]).collect::<Vec<_>>(),
        vec![id(800), id(801)]
    );
    let error = graph
        .validate_current(&f.plan, &f.pieces, marks.get(..1).unwrap(), &f.ledger)
        .unwrap_err();
    let inner = SewingError::MissingLandmark {
        span: id(301),
        stop: *stops.get(1).unwrap(),
    };
    assert_eq!(
        error,
        SewingError::EaseSource {
            source_span: id(301),
            issue: Box::new(inner.clone())
        }
    );
    assert_eq!(error.source().unwrap().to_string(), inner.to_string());
    // The retained root covers more than one live child. Losing root-only material remains visible.
    let mut first_only = f.definition.clone();
    first_only.spans.truncate(1);
    let partial = SewingGraph::new(first_only, &f.plan, &f.pieces, &marks, &f.ledger).unwrap();
    let held = partial
        .ease_sources()
        .next()
        .unwrap()
        .side(SeamSideId::A)
        .range;
    let fragments = f.ledger.split(&mut f.ids, held.edge(), t(3, 4)).unwrap();
    f.ledger.delete(&mut f.ids, fragments.second()).unwrap();
    let error = partial
        .validate_current(&f.plan, &f.pieces, &marks, &f.ledger)
        .unwrap_err();
    assert!(
        matches!(&error, SewingError::EaseSource { source_span, .. } if *source_span==id(301)),
        "{error:?}"
    );
}

#[test]
fn retained_source_resources_charge_each_canonical_body_once_per_logical_graph() {
    use sc_core::design::{
        charge_sewing_graph_resources, DesignResourceCount, DesignResourceLimitError,
        DesignResourceLimits, DesignResourceLocation, DesignResourceMeter, DesignResourceScope,
    };
    use std::num::{NonZeroU64, NonZeroUsize};
    let mut f = fixture();
    let marks = [f.notch(810, t(1, 4)), f.notch(811, t(3, 4))];
    f.span_mut().stops = vec![
        StopLandmark {
            side: SeamSideId::A,
            landmark: id(810),
        },
        StopLandmark {
            side: SeamSideId::A,
            landmark: id(811),
        },
    ];
    f.span_mut().ease.declared_mut().unwrap().distribution = EaseDistribution::Weighted {
        side: SeamSideId::A,
        regions: vec![
            WeightedEaseRegion {
                from: Param::START,
                to: t(1, 2),
                weight: Rational::ONE,
            },
            WeightedEaseRegion {
                from: t(1, 2),
                to: Param::END,
                weight: Rational::new(2, 1).unwrap(),
            },
        ],
    };
    f.definition.spans = retained_children(&f, &marks);
    let graph = f.graph(&marks).unwrap();
    let saved = graph.clone();
    let limits = |entries| {
        DesignResourceLimits::new(
            NonZeroUsize::new(entries).unwrap(),
            NonZeroUsize::new(1).unwrap(),
            NonZeroU64::new(1).unwrap(),
        )
    };
    let mut exact = DesignResourceMeter::new(limits(7));
    charge_sewing_graph_resources(&graph, &mut exact).unwrap();
    assert_eq!(exact.finish().unwrap().collection_entries(), 7); // 2 spans +1 source +2 regions +2 original stops.
    let mut limited = DesignResourceMeter::new(limits(6));
    let expected = DesignResourceLimitError::Count {
        unit: DesignResourceCount::CollectionEntries,
        maximum: 6,
        consumed: 5,
        requested: 2,
        location: DesignResourceLocation {
            scope: DesignResourceScope::Sewing,
            owner: Some(id(301)),
            field: "sewing.ease_sources.stops",
            index: Some(0),
        },
    };
    assert_eq!(
        charge_sewing_graph_resources(&graph, &mut limited),
        Err(expected)
    );
    assert_eq!(limited.finish(), Err(expected));
    let mut snapshots = DesignResourceMeter::new(limits(14));
    charge_sewing_graph_resources(&graph, &mut snapshots).unwrap();
    charge_sewing_graph_resources(&saved, &mut snapshots).unwrap();
    assert_eq!(snapshots.finish().unwrap().collection_entries(), 14); // No dedup across logical snapshots.
    assert_eq!(graph, saved);
}

#[test]
fn duplicate_retained_interval_claims_cannot_spend_the_same_original_budget_twice() {
    use sc_core::ontology::SeamEaseBindingIssue;
    use std::error::Error as _;
    let mut f = fixture();
    f.span_mut().direction = SeamDirection::Same;
    let original = f.graph(&[]).unwrap();
    let root = original.spans().first().unwrap().retain_ease_source();
    let a = f.span().a.range.edge();
    let b = f.span().b.range.edge();
    let split = |portion: &sc_core::ontology::SeamEasePortion, a_cut, b_cut| {
        portion
            .split_at(
                EdgeAnchor {
                    edge: a,
                    param: a_cut,
                },
                EdgeAnchor {
                    edge: b,
                    param: b_cut,
                },
            )
            .unwrap()
    };
    let [lower, upper] = split(&root, t(1, 2), t(1, 2));
    let [_, remainder] = split(&root, t(1, 4), t(1, 4));
    let [middle, _] = split(&remainder, t(3, 4), t(3, 4));
    let [_, b_overlap] = split(&root, t(1, 2), t(1, 4));
    for (first, next, side) in [
        (lower.clone(), lower.clone(), SeamSideId::A),
        (lower.clone(), middle, SeamSideId::A),
        (upper, root.clone(), SeamSideId::A),
        (lower, b_overlap, SeamSideId::B),
    ] {
        let first_range = first.side(side).range;
        let interval = next.side(side).range;
        f.definition.spans = [first, next]
            .into_iter()
            .zip(302_u128..)
            .map(|(portion, identity)| SeamSpanDefinition {
                id: id(identity),
                a: portion.side(SeamSideId::A),
                b: portion.side(SeamSideId::B),
                direction: portion.source().direction(),
                ease: portion.into(),
                stops: vec![],
            })
            .collect();
        let issue = SeamEaseBindingIssue::Overlap {
            side,
            first_span: id(302),
            first: first_range,
            interval,
        };
        let error = f.graph(&[]).unwrap_err();
        assert_eq!(
            error,
            SewingError::EasePortionBinding {
                span: id(303),
                source_span: id(301),
                issue: Box::new(issue)
            }
        );
        assert_eq!(error.source().unwrap().to_string(), issue.to_string());
    }
}

#[test]
fn equal_independent_source_captures_are_folded_into_one_actual_canonical_body() {
    let mut f = fixture();
    f.span_mut().ease.declared_mut().unwrap().distribution = EaseDistribution::Weighted {
        side: SeamSideId::A,
        regions: vec![WeightedEaseRegion {
            from: Param::START,
            to: Param::END,
            weight: Rational::ONE,
        }],
    };
    let first_capture = retained_children(&f, &[]);
    let second_capture = retained_children(&f, &[]);
    let first_source = first_capture
        .first()
        .unwrap()
        .ease
        .portion()
        .unwrap()
        .source();
    let second_source = second_capture
        .get(1)
        .unwrap()
        .ease
        .portion()
        .unwrap()
        .source();
    assert_eq!(first_source, second_source);
    assert!(!std::ptr::eq(first_source, second_source));
    f.definition.spans = vec![
        first_capture.first().unwrap().clone(),
        second_capture.get(1).unwrap().clone(),
    ];
    let graph = f.graph(&[]).unwrap();
    let canonical = graph.ease_sources().next().unwrap();
    assert!(
        graph.spans().iter().all(|span| std::ptr::eq(
            span.definition().ease.portion().unwrap().source(),
            canonical
        )),
        "all portions must reference the actual canonical body charged by the graph"
    );
}

#[test]
fn distinct_retained_budgets_keep_first_consuming_order_and_separate_interval_claims() {
    let mut f = fixture();
    let original = retained_children(&f, &[]).first().unwrap().clone();
    f.span_mut().id = id(311);
    let mut other = retained_children(&f, &[]).first().unwrap().clone();
    other.id = id(312);
    assert_eq!(original.a, other.a);
    assert_eq!(original.b, other.b);
    f.definition.spans = vec![other, original];
    let graph = f.graph(&[]).unwrap();
    assert_eq!(
        graph
            .ease_sources()
            .map(|source| source.span())
            .collect::<Vec<_>>(),
        vec![id(311), id(301)]
    );
    for (span, source) in graph.spans().iter().zip(graph.ease_sources()) {
        assert!(std::ptr::eq(
            span.definition().ease.portion().unwrap().source(),
            source
        ));
        let mut editable = span.definition().clone();
        assert!(editable.ease.declared_mut().is_none());
    }
}
