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
fn nonuniform_recovery_oracle_inputs_are_valid_original_sources_without_invented_quantities() {
    use core::num::{NonZeroU64, NonZeroUsize};
    use sc_core::ontology::WeightedEaseLimits;
    fn fraction(value: &str) -> Rational {
        let (numerator, denominator) = value.split_once('/').unwrap_or((value, "1"));
        Rational::new(numerator.parse().unwrap(), denominator.parse().unwrap()).unwrap()
    }
    let mut rows = 0;
    let mut widest_denominator = 0;
    for row in include_str!("fixtures/nonuniform_ease.tsv").lines().skip(1) {
        let fields = row.split('\t').collect::<Vec<_>>();
        assert_eq!(fields.len(), 11);
        let field = |index| *fields.get(index).unwrap();
        let parent = field(1).parse::<i64>().unwrap();
        let side = match field(2) {
            "A" => SeamSideId::A,
            "B" => SeamSideId::B,
            value => panic!("unexpected fixture side {value}"),
        };
        let direction = match field(3) {
            "Same" => SeamDirection::Same,
            "Opposite" => SeamDirection::Opposite,
            value => panic!("unexpected fixture direction {value}"),
        };
        let at = t(field(4).parse().unwrap(), field(5).parse().unwrap());
        assert!((Param::START..=Param::END).contains(&at));
        let regions = field(6)
            .split(';')
            .map(|region| {
                let fields = region.split(':').collect::<Vec<_>>();
                assert_eq!(fields.len(), 3);
                WeightedEaseRegion {
                    from: Param::new(fraction(fields.first().unwrap())).unwrap(),
                    to: Param::new(fraction(fields.get(1).unwrap())).unwrap(),
                    weight: fraction(fields.get(2).unwrap()),
                }
            })
            .collect();
        let mut f = fixture();
        f.span_mut().direction = direction;
        let declaration = f.span_mut().ease.declared_mut().unwrap();
        declaration.amount = EaseAmount::Explicit(Length::from_micrometres(parent).unwrap());
        declaration.distribution = EaseDistribution::Weighted { side, regions };
        let graph = f.graph(&[]).unwrap();
        let source = graph.spans().first().unwrap().retain_ease_source();
        assert_eq!(source.source().declaration(), f.span().ease.declaration());
        assert_eq!(
            source.source().uniform_explicit_cumulative_at_a(at),
            Err(sc_core::ontology::SeamEaseAmountError::Weighted {
                source_span: id(301),
                side,
            })
        );
        // The uniform API retains its actual weighted refusal. The separately admitted weighted
        // query consumes the same original declaration and the independent wide Fraction result.
        let numerator = field(7).parse::<num_bigint::BigUint>().unwrap();
        let denominator = field(8).parse::<num_bigint::BigUint>().unwrap();
        assert!(numerator <= denominator);
        widest_denominator = widest_denominator.max(denominator.bits());
        let first = field(9).parse::<i64>().unwrap();
        let second = field(10).parse::<i64>().unwrap();
        assert_eq!(first + second, parent);
        let admitted = source
            .source()
            .admit_weighted_explicit(WeightedEaseLimits::new(
                NonZeroUsize::new(4).unwrap(),
                NonZeroU64::new(1000).unwrap(),
            ))
            .unwrap();
        let domain = admitted.source().side(side).range;
        assert_eq!(
            admitted
                .cumulative_at(EdgeAnchor {
                    edge: domain.edge(),
                    param: at
                })
                .unwrap()
                .as_micrometres(),
            first,
            "{row}"
        );
        assert_eq!(
            admitted.amount_in(domain).unwrap().as_micrometres(),
            parent,
            "{row}"
        );
        rows += 1;
    }
    assert_eq!(rows, 160);
    assert_eq!(widest_denominator, 188);
}

#[test]
fn nonuniform_recovery_notches_can_be_authored_in_later_frames_and_either_named_order() {
    let mut f = fixture();
    let first_definition = match f.notch(400, t(1, 4)) {
        SewingLandmark::Notch(notch) => notch.definition().clone(),
        SewingLandmark::Turn(_) => unreachable!("fixture constructs a notch"),
    };
    let second_definition = match f.notch(401, t(3, 4)) {
        SewingLandmark::Notch(notch) => notch.definition().clone(),
        SewingLandmark::Turn(_) => unreachable!("fixture constructs a notch"),
    };
    let original_edge = f.span().a.range.edge();
    let fragments = f.ledger.split(&mut f.ids, original_edge, t(1, 2)).unwrap();
    let marks = [
        (first_definition, fragments.first()),
        (second_definition, fragments.second()),
    ]
    .map(|(mut definition, edge)| {
        definition.anchor = EdgeAnchor {
            edge,
            param: t(1, 2),
        };
        SewingLandmark::Notch(Notch::new(definition, f.pieces.first().unwrap(), &f.ledger).unwrap())
    });
    for (mark, original_at) in marks.iter().zip([t(1, 4), t(3, 4)]) {
        assert_eq!(
            mark.resolve(&f.ledger),
            f.ledger.resolve(original_edge, original_at)
        );
    }
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
    for (start, end) in [(id(400), id(401)), (id(401), id(400))] {
        f.span_mut().ease.declared_mut().unwrap().distribution = EaseDistribution::BetweenNotches {
            side: SeamSideId::A,
            start,
            end,
        };
        let graph = f.graph(&marks).unwrap();
        let portion = graph.spans().first().unwrap().retain_ease_source();
        assert_eq!(portion.source().stops(), f.span().stops);
        assert_eq!(
            portion.source().uniform_explicit_cumulative_at_a(t(1, 2)),
            Err(sc_core::ontology::SeamEaseAmountError::BetweenNotches {
                source_span: id(301),
                side: SeamSideId::A,
                start,
                end,
            })
        );
    }
}

#[test]
fn weighted_workspace_admission_keeps_complete_original_source_and_exact_selected_limits() {
    use core::num::{NonZeroU64, NonZeroUsize};
    use sc_core::ontology::{WeightedEaseLimits, WeightedEaseWorkspaceError as Error};
    let limits = |regions, bits| {
        WeightedEaseLimits::new(
            NonZeroUsize::new(regions).unwrap(),
            NonZeroU64::new(bits).unwrap(),
        )
    };
    for parent in [-1, 0, 1] {
        let mut f = fixture();
        let declaration = f.span_mut().ease.declared_mut().unwrap();
        declaration.amount = EaseAmount::Explicit(Length::from_micrometres(parent).unwrap());
        declaration.distribution = EaseDistribution::Weighted {
            side: SeamSideId::B,
            regions: (0_i64..4)
                .map(|index| WeightedEaseRegion {
                    from: t(index, 4),
                    to: t(index + 1, 4),
                    weight: Rational::new(1, i64::MAX - 2 * (index + 1)).unwrap(),
                })
                .collect(),
        };
        let graph = f.graph(&[]).unwrap();
        let original = graph.spans().first().unwrap().retain_ease_source();
        let [first, second] = original
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
        for portion in [&original, &first, &second] {
            let source = portion.source();
            let admitted = source.admit_weighted_explicit(limits(4, 977)).unwrap();
            assert!(std::ptr::eq(admitted.source(), source));
            assert_eq!(admitted.distribution_side(), SeamSideId::B);
            assert_eq!(admitted.region_count(), 4);
            assert_eq!(admitted.parent_amount().as_micrometres(), parent);
            assert_eq!(admitted.integer_bit_bound(), 977);
            assert_eq!(admitted.limits(), limits(4, 977));
            assert_eq!(admitted.limits().regions().get(), 4);
            assert_eq!(admitted.limits().integer_bits().get(), 977);
            let count_error = source.admit_weighted_explicit(limits(3, 1)).unwrap_err();
            assert_eq!(
                count_error,
                Error::Regions {
                    source_span: id(301),
                    side: SeamSideId::B,
                    required: 4,
                    limit: 3,
                }
            );
            assert_eq!(
                source.admit_weighted_explicit(limits(4, 976)).unwrap_err(),
                Error::IntegerBits {
                    source_span: id(301),
                    side: SeamSideId::B,
                    regions: 4,
                    required: 977,
                    limit: 976,
                }
            );
            assert!(std::error::Error::source(&count_error).is_none());
            let debug = format!("{admitted:?}");
            assert!(debug.contains("regions: 4") && !debug.contains("WeightedEaseRegion"));
        }
    }
}

#[test]
fn weighted_workspace_admission_preserves_actual_eligibility_before_budget_refusals() {
    use core::num::{NonZeroU64, NonZeroUsize};
    use sc_core::ontology::{WeightedEaseLimits, WeightedEaseWorkspaceError as Error};
    let tiny = WeightedEaseLimits::new(NonZeroUsize::new(1).unwrap(), NonZeroU64::new(1).unwrap());
    let mut f = fixture();
    f.span_mut().ease.declared_mut().unwrap().amount = EaseAmount::Formula(id(500));
    let graph = f.graph(&[]).unwrap();
    let source = graph.spans().first().unwrap().retain_ease_source();
    assert_eq!(
        source.source().admit_weighted_explicit(tiny).unwrap_err(),
        Error::Uniform {
            source_span: id(301)
        }
    );
    f.span_mut().ease.declared_mut().unwrap().distribution = EaseDistribution::Weighted {
        side: SeamSideId::A,
        regions: vec![WeightedEaseRegion {
            from: Param::START,
            to: Param::END,
            weight: Rational::ONE,
        }],
    };
    for amount in [
        EaseAmount::Formula(id(500)),
        EaseAmount::Profile(ProfileParameterRef::new(id(501))),
    ] {
        f.span_mut().ease.declared_mut().unwrap().amount = amount;
        let graph = f.graph(&[]).unwrap();
        let source = graph.spans().first().unwrap().retain_ease_source();
        assert_eq!(
            source.source().admit_weighted_explicit(tiny).unwrap_err(),
            Error::Amount {
                source_span: id(301),
                amount
            }
        );
    }
    let marks = [f.notch(400, t(1, 4)), f.notch(401, t(3, 4))];
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
        start: id(401),
        end: id(400),
    };
    let graph = f.graph(&marks).unwrap();
    let source = graph.spans().first().unwrap().retain_ease_source();
    assert_eq!(
        source.source().admit_weighted_explicit(tiny).unwrap_err(),
        Error::BetweenNotches {
            source_span: id(301),
            side: SeamSideId::A,
            start: id(401),
            end: id(400),
        }
    );
}

#[test]
fn weighted_original_domain_queries_use_complete_fractions_beyond_reference_progress_width() {
    use core::num::{NonZeroU64, NonZeroUsize};
    use sc_core::ontology::{WeightedEaseAmountError as Error, WeightedEaseLimits};
    let limits =
        WeightedEaseLimits::new(NonZeroUsize::new(1).unwrap(), NonZeroU64::new(408).unwrap());
    let mut rows = 0;
    for row in include_str!("fixtures/weighted_domain_ease.tsv")
        .lines()
        .skip(1)
    {
        let fields = row.split('\t').collect::<Vec<_>>();
        assert_eq!(fields.len(), 11);
        let field = |index| *fields.get(index).unwrap();
        let parent = field(1).parse().unwrap();
        let from = t(field(2).parse().unwrap(), field(3).parse().unwrap());
        let at = t(field(4).parse().unwrap(), field(5).parse().unwrap());
        let to = t(field(6).parse().unwrap(), field(7).parse().unwrap());
        let mut f = fixture();
        let edge = f.span().a.range.edge();
        let domain = EdgeRange::new(edge, from, to).unwrap();
        f.span_mut().a.range = domain;
        f.span_mut().ease = DeclaredEase {
            amount: EaseAmount::Explicit(Length::from_micrometres(parent).unwrap()),
            distribution: EaseDistribution::Weighted {
                side: SeamSideId::A,
                regions: vec![WeightedEaseRegion {
                    from: Param::START,
                    to: Param::END,
                    weight: Rational::ONE,
                }],
            },
        }
        .into();
        let graph = f.graph(&[]).unwrap();
        let original = graph.spans().first().unwrap().retain_ease_source();
        let admitted = original.source().admit_weighted_explicit(limits).unwrap();
        let actual = admitted.cumulative_at(EdgeAnchor { edge, param: at });
        if field(10) == "outside" {
            assert_eq!(
                actual,
                Err(Error::Outside {
                    source_span: id(301),
                    side: SeamSideId::A,
                    range: domain,
                    at,
                }),
                "{row}"
            );
        } else {
            assert_eq!(
                actual.unwrap().as_micrometres(),
                field(10).parse::<i64>().unwrap(),
                "{row}"
            );
            if field(0).starts_with("final-") || field(0).starts_with("wide-") {
                assert!(matches!(
                    domain.relative_progress(at),
                    Err(sc_core::ontology::RangeProgressError::Arithmetic { .. })
                ));
            }
        }
        rows += 1;
    }
    assert_eq!(rows, 50);
}

#[test]
fn weighted_interval_quantities_conserve_original_boundaries_with_pre_rounding_b_orientation() {
    use core::num::{NonZeroU64, NonZeroUsize};
    use sc_core::ontology::WeightedEaseLimits;
    let limits =
        WeightedEaseLimits::new(NonZeroUsize::new(2).unwrap(), NonZeroU64::new(598).unwrap());
    for side in [SeamSideId::A, SeamSideId::B] {
        for parent in [-1, 1] {
            let mut f = fixture();
            f.span_mut().ease = DeclaredEase {
                amount: EaseAmount::Explicit(Length::from_micrometres(parent).unwrap()),
                distribution: EaseDistribution::Weighted {
                    side,
                    regions: vec![
                        WeightedEaseRegion {
                            from: Param::START,
                            to: t(1, 2),
                            weight: Rational::ONE,
                        },
                        WeightedEaseRegion {
                            from: t(1, 2),
                            to: Param::END,
                            weight: Rational::from_integer(3),
                        },
                    ],
                },
            }
            .into();
            let graph = f.graph(&[]).unwrap();
            let original = graph.spans().first().unwrap().retain_ease_source();
            let admitted = original.source().admit_weighted_explicit(limits).unwrap();
            let domain = original.source().side(side).range;
            let boundaries = [Param::START, t(1, 3), t(2, 3), Param::END];
            let mut quantities = boundaries
                .windows(2)
                .map(|ends| {
                    admitted
                        .amount_in(
                            EdgeRange::new(
                                domain.edge(),
                                *ends.first().unwrap(),
                                *ends.get(1).unwrap(),
                            )
                            .unwrap(),
                        )
                        .unwrap()
                        .as_micrometres()
                })
                .collect::<Vec<_>>();
            if side == SeamSideId::B {
                quantities.reverse();
                assert_eq!(quantities, [parent, 0, 0]);
            } else {
                assert_eq!(quantities, [0, parent, 0]);
            }
            assert_eq!(quantities.iter().sum::<i64>(), parent);
            assert_eq!(admitted.amount_in(domain).unwrap().as_micrometres(), parent);
        }
    }
}

#[test]
fn weighted_queries_preserve_actual_frames_and_bounds_even_for_zero_source() {
    use core::num::{NonZeroU64, NonZeroUsize};
    use sc_core::ontology::{WeightedEaseAmountError as Error, WeightedEaseLimits};
    let mut f = fixture();
    let edge = f.span().a.range.edge();
    let foreign = f.span().b.range.edge();
    let domain = EdgeRange::new(edge, t(1, 4), t(3, 4)).unwrap();
    f.span_mut().a.range = domain;
    f.span_mut().ease.declared_mut().unwrap().distribution = EaseDistribution::Weighted {
        side: SeamSideId::A,
        regions: vec![WeightedEaseRegion {
            from: Param::START,
            to: Param::END,
            weight: Rational::ONE,
        }],
    };
    let graph = f.graph(&[]).unwrap();
    let original = graph.spans().first().unwrap().retain_ease_source();
    let admitted = original
        .source()
        .admit_weighted_explicit(WeightedEaseLimits::new(
            NonZeroUsize::new(1).unwrap(),
            NonZeroU64::new(408).unwrap(),
        ))
        .unwrap();
    let at = EdgeAnchor {
        edge: foreign,
        param: Param::START,
    };
    let error = admitted.cumulative_at(at).unwrap_err();
    assert_eq!(
        error,
        Error::SourceFrame {
            source_span: id(301),
            side: SeamSideId::A,
            expected: edge,
            at
        }
    );
    assert!(std::error::Error::source(&error).is_none());
    assert_eq!(
        admitted.cumulative_at(EdgeAnchor {
            edge,
            param: Param::START
        }),
        Err(Error::Outside {
            source_span: id(301),
            side: SeamSideId::A,
            range: domain,
            at: Param::START
        })
    );
    let interval = EdgeRange::whole(foreign);
    assert_eq!(
        admitted.amount_in(interval),
        Err(Error::IntervalSource {
            source_span: id(301),
            side: SeamSideId::A,
            expected: edge,
            interval,
        })
    );
    assert_eq!(
        admitted.amount_in(EdgeRange::new(edge, t(1, 4), Param::END).unwrap()),
        Err(Error::Outside {
            source_span: id(301),
            side: SeamSideId::A,
            range: domain,
            at: Param::END
        })
    );
    assert_eq!(admitted.amount_in(domain), Ok(Length::ZERO));
}

fn project_source_notch<'a>(
    mark: &SewingLandmark,
    source: &sc_core::ontology::SeamEaseSource,
    ledger: &'a IdentityLedger,
    candidates: usize,
) -> sc_core::ontology::OriginalPointProjection<'a> {
    use core::num::{NonZeroU64, NonZeroUsize};
    use sc_core::ontology::{OriginalPointLimits, UniquePointLimits};
    let SewingLandmark::Notch(notch) = mark else {
        unreachable!("actual matching Notch fixture")
    };
    let EaseDistribution::BetweenNotches { side, start, end } = &source.declaration().distribution
    else {
        unreachable!("actual original notch-bounded source")
    };
    assert!(mark.id() == *start || mark.id() == *end);
    assert!(source
        .stops()
        .iter()
        .any(|stop| stop.side == *side && stop.landmark == mark.id()));
    assert_eq!(
        notch.profile_binding_validation(),
        sc_core::ontology::ProfileBindingValidation::DeferredToG4
    );
    let original_definition = notch.definition().clone();
    let anchor = notch.definition().anchor;
    let input = UniquePointLimits::new(
        NonZeroUsize::new(32).unwrap(),
        NonZeroUsize::new(32).unwrap(),
    );
    let current = ledger
        .resolve_unique_point(anchor.edge, anchor.param, input)
        .resolved()
        .unwrap();
    let coverage = ledger
        .resolve_range_coverage(
            source.side(*side).range,
            sc_core::ontology::RangeCoverageLimits::new(input, NonZeroUsize::new(32).unwrap()),
        )
        .unwrap();
    assert!(coverage.has_full_coverage());
    assert_eq!(coverage.held(), source.side(*side).range);
    let limits = OriginalPointLimits::new(
        input,
        NonZeroUsize::new(candidates).unwrap(),
        NonZeroU64::new(10000).unwrap(),
    );
    let admitted = ledger
        .admit_original_point(
            anchor.edge,
            anchor.param,
            source.side(*side).range.edge(),
            limits,
        )
        .unwrap();
    assert_eq!(admitted.current(), current);
    let projected = admitted.project().unwrap();
    assert_eq!(
        (
            projected.admission().reference(),
            projected.admission().param()
        ),
        (anchor.edge, anchor.param)
    );
    // Available coordinates cannot turn the old Uniform-only API into a notch-context fallback.
    assert_eq!(
        source.uniform_explicit_cumulative_at_a(t(1, 2)),
        Err(sc_core::ontology::SeamEaseAmountError::BetweenNotches {
            source_span: source.span(),
            side: *side,
            start: *start,
            end: *end,
        })
    );
    assert_eq!(notch.definition(), &original_definition);
    projected
}

#[test]
fn notch_context_recovery_accepts_a_late_anchor_with_an_unrepresentable_original_param() {
    let mut f = fixture();
    let definitions = [400, 401].map(|identity| match f.notch(identity, t(1, 4)) {
        SewingLandmark::Notch(notch) => notch.definition().clone(),
        SewingLandmark::Turn(_) => unreachable!("fixture constructs a notch"),
    });
    let source_edge = f.span().a.range.edge();
    let fragments = f.ledger.split(&mut f.ids, source_edge, t(1, 2)).unwrap();
    let row = include_str!("fixtures/notch_context_recovery.tsv")
        .lines()
        .find(|row| row.starts_with("wide-split\t"))
        .unwrap()
        .split('\t')
        .collect::<Vec<_>>();
    let current_n = row.get(1).unwrap().parse::<i64>().unwrap();
    let current_d = row.get(2).unwrap().parse::<i64>().unwrap();
    let original_d = row.get(4).unwrap().parse::<u128>().unwrap();
    assert_eq!(original_d, 2 * u128::from(current_d.unsigned_abs()));
    assert!(original_d > u128::from(i64::MAX.unsigned_abs()));
    let marks = definitions
        .into_iter()
        .zip([t(current_n, current_d), t(1, 2)])
        .map(|(mut definition, param)| {
            definition.anchor = EdgeAnchor {
                edge: fragments.first(),
                param,
            };
            SewingLandmark::Notch(
                Notch::new(definition, f.pieces.first().unwrap(), &f.ledger).unwrap(),
            )
        })
        .collect::<Vec<_>>();
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
    f.span_mut().ease = DeclaredEase {
        amount: EaseAmount::Explicit(Length::from_micrometres(1).unwrap()),
        distribution: EaseDistribution::BetweenNotches {
            side: SeamSideId::A,
            start: id(400),
            end: id(401),
        },
    }
    .into();
    let graph = f.graph(&marks).unwrap();
    assert_eq!(graph.ease_validation(), EaseValidation::DeferredToG2AndG3);
    assert!(f.ledger.resolve_range(f.span().a.range).has_full_coverage());
    let current = marks
        .first()
        .unwrap()
        .resolve(&f.ledger)
        .resolved()
        .unwrap();
    assert_eq!(current.edge(), fragments.first());
    assert_eq!(current.param(), t(current_n, current_d));
    // The bounded reference arithmetic refusal is correct;the admitted inverse below retains
    // the valid original coordinate exactly without forcing it into Param.
    assert_eq!(
        t(current_n, current_d)
            .as_rational()
            .checked_mul(t(1, 2).as_rational()),
        Err(sc_units::UnitError::Overflow {
            operation: "Rational::checked_mul"
        })
    );
    let original = graph.spans().first().unwrap().retain_ease_source();
    assert_eq!(original.source().span(), id(301));
    assert_eq!(original.source().side(SeamSideId::A), f.span().a);
    assert_eq!(original.source().stops(), f.span().stops);
    assert_eq!(original.source().declaration(), f.span().ease.declaration());
    let before = f.ledger.clone();
    let projected = project_source_notch(marks.first().unwrap(), original.source(), &f.ledger, 1);
    assert_eq!(
        projected
            .unique()
            .unwrap()
            .coordinate()
            .as_u128_ratio()
            .unwrap(),
        (1, original_d)
    );
    assert!(projected.unique().unwrap().coordinate().as_param().is_err());
    let upper = project_source_notch(marks.get(1).unwrap(), original.source(), &f.ledger, 1);
    assert_eq!(
        upper.unique().unwrap().coordinate().as_param().unwrap(),
        t(1, 4)
    );
    assert_eq!(f.ledger, before);
}

#[test]
fn notch_context_recovery_has_two_original_coordinates_for_one_unique_current_notch() {
    use sc_core::ontology::SplitSide;
    let mut f = fixture();
    let definitions = [400, 401].map(|identity| match f.notch(identity, t(1, 4)) {
        SewingLandmark::Notch(notch) => notch.definition().clone(),
        SewingLandmark::Turn(_) => unreachable!("fixture constructs a notch"),
    });
    let source_edge = f.span().a.range.edge();
    let fragments = f.ledger.split(&mut f.ids, source_edge, t(1, 2)).unwrap();
    f.ledger.reverse(&mut f.ids, fragments.second()).unwrap();
    // Equal positive lengths are declared protocol inputs here,not measured geometry evidence.
    let declared = Length::from_micrometres(1).unwrap();
    let merged = f
        .ledger
        .merge(
            &mut f.ids,
            fragments.first(),
            declared,
            fragments.second(),
            declared,
        )
        .unwrap();
    let marks = definitions
        .into_iter()
        .zip([t(1, 2), t(1, 4)])
        .map(|(mut definition, param)| {
            definition.anchor = EdgeAnchor {
                edge: merged,
                param,
            };
            SewingLandmark::Notch(
                Notch::new(definition, f.pieces.first().unwrap(), &f.ledger).unwrap(),
            )
        })
        .collect::<Vec<_>>();
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
    f.span_mut().ease = DeclaredEase {
        amount: EaseAmount::Explicit(Length::from_micrometres(1).unwrap()),
        distribution: EaseDistribution::BetweenNotches {
            side: SeamSideId::A,
            start: id(400),
            end: id(401),
        },
    }
    .into();
    let graph = f.graph(&marks).unwrap();
    assert_eq!(graph.ease_validation(), EaseValidation::DeferredToG2AndG3);
    assert!(f.ledger.resolve_range(f.span().a.range).has_full_coverage());
    let current = marks
        .first()
        .unwrap()
        .resolve(&f.ledger)
        .resolved()
        .unwrap();
    assert_eq!(current.edge(), merged);
    assert_eq!(current.param(), t(1, 2));
    let row = include_str!("fixtures/notch_context_recovery.tsv")
        .lines()
        .find(|row| row.starts_with("reversed-merge\t"))
        .unwrap()
        .split('\t')
        .collect::<Vec<_>>();
    let first = t(
        row.get(3).unwrap().parse().unwrap(),
        row.get(4).unwrap().parse().unwrap(),
    );
    let second = t(
        row.get(5).unwrap().parse().unwrap(),
        row.get(6).unwrap().parse().unwrap(),
    );
    assert_ne!(first, second);
    assert!(f.ledger.resolve(source_edge, first).resolved().is_none());
    // Explicit First is only a forward witness for this counterexample;it is not a repair or a
    // selection made on behalf of an original-source context consumer.
    let first_forward = f
        .ledger
        .resolve(source_edge, first)
        .choose(SplitSide::First)
        .resolved()
        .unwrap();
    let second_forward = f.ledger.resolve(source_edge, second).resolved().unwrap();
    assert_eq!(
        (first_forward.edge(), first_forward.param()),
        (current.edge(), current.param())
    );
    assert_eq!(
        (second_forward.edge(), second_forward.param()),
        (current.edge(), current.param())
    );
    // Directions are relative to the holder's original frame,not absolute location identities.
    // The two source paths retain different directions;the newer notch uses the merged frame.
    assert_eq!(first_forward.direction(), Direction::Original);
    assert_eq!(second_forward.direction(), Direction::Reversed);
    assert_eq!(current.direction(), Direction::Original);
    let original = graph.spans().first().unwrap().retain_ease_source();
    assert_eq!(original.source().side(SeamSideId::A), f.span().a);
    assert_eq!(original.source().stops(), f.span().stops);
    let before = f.ledger.clone();
    let projected = project_source_notch(marks.first().unwrap(), original.source(), &f.ledger, 2);
    assert!(projected.unique().is_none());
    let candidates = projected
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
        vec![(first, Direction::Original), (second, Direction::Reversed)]
    );
    assert_eq!(
        projected.admission().current().direction(),
        current.direction()
    );
    let other = project_source_notch(marks.get(1).unwrap(), original.source(), &f.ledger, 2);
    assert_eq!(
        other.unique().unwrap().coordinate().as_param().unwrap(),
        t(1, 4)
    );
    assert_eq!(f.ledger, before);
}

#[test]
fn notch_context_recovery_bounds_must_cover_derived_reference_choices_before_resolution() {
    use core::num::{NonZeroU64, NonZeroUsize};
    use sc_core::{
        design::{charge_topology_resources, DesignResourceLimits, DesignResourceMeter},
        ontology::{Resolution, ResolvedRef},
    };
    fn leaves(value: &Resolution, expected: ResolvedRef) -> usize {
        match value {
            Resolution::Resolved(actual) => {
                assert_eq!(*actual, expected);
                1
            }
            Resolution::SplitPoint { first, second } => {
                leaves(first, expected) + leaves(second, expected)
            }
            Resolution::Unresolved(task) => {
                panic!("unexpected orphan in this complete fixture: {task}")
            }
        }
    }
    let mut f = fixture();
    let definitions = [400, 401].map(|identity| match f.notch(identity, t(1, 4)) {
        SewingLandmark::Notch(notch) => notch.definition().clone(),
        SewingLandmark::Turn(_) => unreachable!("actual Notch definition"),
    });
    let original = f.span().a.range.edge();
    let mut current_edge = original;
    let declared = Length::from_micrometres(1).unwrap();
    for _ in 0..8 {
        let parts = f.ledger.split(&mut f.ids, current_edge, t(1, 2)).unwrap();
        current_edge = f
            .ledger
            .merge(
                &mut f.ids,
                parts.first(),
                declared,
                parts.second(),
                declared,
            )
            .unwrap();
    }
    let limits = DesignResourceLimits::new(
        NonZeroUsize::new(25).unwrap(),
        NonZeroUsize::new(1).unwrap(),
        NonZeroU64::new(1).unwrap(),
    );
    let mut meter = DesignResourceMeter::new(limits);
    charge_topology_resources(&f.ledger, &mut meter).unwrap();
    assert_eq!(meter.finish().unwrap().collection_entries(), 25);
    assert_eq!(f.ledger.journal().len(), 17);
    let expected = f.ledger.resolve(current_edge, t(1, 2)).resolved().unwrap();
    let actual = f.ledger.resolve(original, t(1, 2));
    assert!(actual.resolved().is_none()); // Choice requirements survive reunification.
    assert_eq!(leaves(&actual, expected), 256);
    // This is the public logical input census,not a promised bound on derived workspace or CPU.
    // A new context must admit/reference-limit derived choices before calling this full resolver.
    let marks = definitions
        .into_iter()
        .zip([t(1, 2), t(1, 4)])
        .map(|(mut definition, param)| {
            definition.anchor = EdgeAnchor {
                edge: current_edge,
                param,
            };
            SewingLandmark::Notch(
                Notch::new(definition, f.pieces.first().unwrap(), &f.ledger).unwrap(),
            )
        })
        .collect::<Vec<_>>();
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
    f.span_mut().ease = DeclaredEase {
        amount: EaseAmount::Explicit(Length::from_micrometres(1).unwrap()),
        distribution: EaseDistribution::BetweenNotches {
            side: SeamSideId::A,
            start: id(400),
            end: id(401),
        },
    }
    .into();
    let graph = f.graph(&marks).unwrap();
    assert_eq!(graph.ease_validation(), EaseValidation::DeferredToG2AndG3);
    let source = graph.spans().first().unwrap().retain_ease_source();
    assert_eq!(source.source().side(SeamSideId::A).range.edge(), original);
    let projected = project_source_notch(marks.first().unwrap(), source.source(), &f.ledger, 2);
    assert_eq!(projected.candidates().len(), 1);
    assert_eq!(
        projected.unique().unwrap().coordinate().as_param().unwrap(),
        t(1, 2)
    );
    assert_eq!(projected.unique().unwrap().direction(), Direction::Original);
    let other = project_source_notch(marks.get(1).unwrap(), source.source(), &f.ledger, 2);
    assert_eq!(
        other.unique().unwrap().coordinate().as_param().unwrap(),
        t(1, 4)
    );
}

#[test]
fn notch_context_recovery_resolves_a_valid_split_after_complete_fraction_recomposition() {
    let mut f = fixture();
    let source = f.span().a.range.edge();
    let row = include_str!("fixtures/notch_context_recovery.tsv")
        .lines()
        .find(|row| row.starts_with("split-final-fits\t"))
        .unwrap()
        .split('\t')
        .collect::<Vec<_>>();
    let split = t(
        row.get(5).unwrap().parse().unwrap(),
        row.get(6).unwrap().parse().unwrap(),
    );
    let held = t(
        row.get(3).unwrap().parse().unwrap(),
        row.get(4).unwrap().parse().unwrap(),
    );
    let expected = t(
        row.get(1).unwrap().parse().unwrap(),
        row.get(2).unwrap().parse().unwrap(),
    );
    let complete = EdgeRange::new(source, split, Param::END)
        .unwrap()
        .relative_progress(held)
        .unwrap();
    assert_eq!(complete, expected.as_rational());
    let second = f.ledger.split(&mut f.ids, source, split).unwrap().second();
    let actual = f.ledger.resolve(source, held).resolved().unwrap();
    assert_eq!(actual.edge(), second);
    assert_eq!(actual.param(), expected);
    assert_eq!(actual.direction(), Direction::Original);
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
            let root = binding.span().retain_ease_source();
            let [first_portion, second_portion] = root
                .split_at(request.cut(SeamSideId::A), request.cut(SeamSideId::B))
                .unwrap();
            assert_eq!(
                first_portion
                    .uniform_explicit_amount()
                    .unwrap()
                    .as_micrometres(),
                *first
            );
            assert_eq!(
                second_portion
                    .uniform_explicit_amount()
                    .unwrap()
                    .as_micrometres(),
                *second
            );
            assert_eq!(
                root.source()
                    .uniform_explicit_cumulative_at_a(request.cut(SeamSideId::A).param)
                    .unwrap()
                    .as_micrometres(),
                *first
            );
            assert_eq!(
                root.source()
                    .uniform_explicit_amount_in_a(first_portion.side(SeamSideId::A).range),
                first_portion.uniform_explicit_amount()
            );
            assert!(allocation.original_source().is_none());

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
        let root = graph.spans().first().unwrap().retain_ease_source();
        assert_eq!(
            root.uniform_explicit_amount(),
            Err(sc_core::ontology::SeamEaseAmountError::Amount {
                source_span: id(301),
                amount
            })
        );
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
    let root = graph.spans().first().unwrap().retain_ease_source();
    assert_eq!(
        root.uniform_explicit_amount(),
        Err(sc_core::ontology::SeamEaseAmountError::Weighted {
            source_span: id(301),
            side: SeamSideId::B
        })
    );
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
    let root = graph.spans().first().unwrap().retain_ease_source();
    assert_eq!(
        root.uniform_explicit_amount(),
        Err(sc_core::ontology::SeamEaseAmountError::BetweenNotches {
            source_span: id(301),
            side: SeamSideId::A,
            start: id(720),
            end: id(721)
        })
    );
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
    let allocation = request
        .bind_source_cuts(&graph)
        .unwrap()
        .uniform_explicit_ease()
        .unwrap();
    assert_eq!(
        (
            allocation.parent_amount().as_micrometres(),
            allocation.first_amount().as_micrometres(),
            allocation.second_amount().as_micrometres()
        ),
        (1, 0, 1)
    );
    assert_eq!(allocation.cut_progress(), Rational::new(1, 4).unwrap());
    assert!(std::ptr::eq(allocation.original_source().unwrap(), root));
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

#[test]
fn retained_uniform_quantities_and_command_views_keep_original_rounding_in_both_nested_orders() {
    for original in [1, -1] {
        for direction in [SeamDirection::Same, SeamDirection::Opposite] {
            let mut f = fixture();
            let a = f.span().a.range.edge();
            let b = f.span().b.range.edge();
            f.span_mut().a.range = EdgeRange::new(a, t(1, 7), t(6, 7)).unwrap();
            f.span_mut().b.range = EdgeRange::new(b, t(1, 5), t(4, 5)).unwrap();
            f.span_mut().direction = direction;
            f.span_mut().ease.declared_mut().unwrap().amount =
                EaseAmount::Explicit(Length::from_micrometres(original).unwrap());
            let graph = f.graph(&[]).unwrap();
            let root = graph.spans().first().unwrap().retain_ease_source();
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
            for (initial_a, initial_b, next_a, next_b, target, progress) in [
                (a1, b1, a2, b2, 1_usize, Rational::new(2, 3).unwrap()),
                (a2, b2, a1, b1, 0_usize, Rational::new(1, 3).unwrap()),
            ] {
                let initial = root.split_at(initial_a, initial_b).unwrap();
                let nested = initial
                    .get(target)
                    .unwrap()
                    .split_at(next_a, next_b)
                    .unwrap();
                let amounts = if target == 1 {
                    [
                        initial.first().unwrap().uniform_explicit_amount().unwrap(),
                        nested.first().unwrap().uniform_explicit_amount().unwrap(),
                        nested.get(1).unwrap().uniform_explicit_amount().unwrap(),
                    ]
                } else {
                    [
                        nested.first().unwrap().uniform_explicit_amount().unwrap(),
                        nested.get(1).unwrap().uniform_explicit_amount().unwrap(),
                        initial.get(1).unwrap().uniform_explicit_amount().unwrap(),
                    ]
                };
                assert_eq!(amounts.map(Length::as_micrometres), [0, original, 0]);
                let intermediate = SewingGraph::new(
                    SewingGraphDefinition {
                        id: id(300),
                        spans: initial
                            .into_iter()
                            .zip(302_u128..)
                            .map(|(portion, identity)| SeamSpanDefinition {
                                id: id(identity),
                                a: portion.side(SeamSideId::A),
                                b: portion.side(SeamSideId::B),
                                direction,
                                ease: portion.into(),
                                stops: vec![],
                            })
                            .collect(),
                    },
                    &f.plan,
                    &f.pieces,
                    &[],
                    &f.ledger,
                )
                .unwrap();
                let actual = intermediate.spans().get(target).unwrap();
                let request = SplitSeamSpan::new(actual.id(), next_a, next_b);
                let view = request
                    .bind_source_cuts(&intermediate)
                    .unwrap()
                    .uniform_explicit_ease()
                    .unwrap();
                assert!(std::ptr::eq(view.span(), actual));
                assert!(std::ptr::eq(view.request(), &request));
                assert!(std::ptr::eq(
                    view.original_source().unwrap(),
                    intermediate.ease_sources().next().unwrap()
                ));
                assert_eq!(view.cut_progress(), progress); // Original-root progress,not the child's1/2.
                assert_eq!(view.parent_amount().as_micrometres(), original);
                assert_eq!(
                    view.first_amount().checked_add(view.second_amount()),
                    Ok(view.parent_amount())
                );
                assert_eq!(
                    (
                        view.first_amount().as_micrometres(),
                        view.second_amount().as_micrometres()
                    ),
                    if target == 1 {
                        (original, 0)
                    } else {
                        (0, original)
                    }
                );
                assert_eq!(view.cumulative_at_a(t(1, 7)), Ok(Length::ZERO));
                assert_eq!(
                    view.cumulative_at_a(t(6, 7)).unwrap().as_micrometres(),
                    original
                );
                assert_eq!(
                    view.amount_in_a(EdgeRange::new(a, t(8, 21), t(13, 21)).unwrap())
                        .unwrap()
                        .as_micrometres(),
                    original
                );
            }
        }
    }
}

#[test]
fn retained_uniform_quantity_refusals_keep_original_eligibility_correspondence_and_arithmetic() {
    use sc_core::command::CommandUniformEaseError;
    use sc_core::ontology::SeamEaseAmountError as Error;
    use std::error::Error as _;
    for direction in [SeamDirection::Same, SeamDirection::Opposite] {
        let mut f = fixture();
        f.span_mut().direction = direction;
        let graph = f.graph(&[]).unwrap();
        let root = graph.spans().first().unwrap().retain_ease_source();
        let [first, second] = root
            .split_at(
                EdgeAnchor {
                    edge: f.span().a.range.edge(),
                    param: t(1, 3),
                },
                EdgeAnchor {
                    edge: f.span().b.range.edge(),
                    param: t(1, 2),
                },
            )
            .unwrap();
        for portion in [&first, &second] {
            assert_eq!(
                portion.uniform_explicit_amount().unwrap_err(),
                Error::Correspondence {
                    source_span: id(301),
                    at_a: t(1, 3),
                    at_b: t(1, 2),
                    a: Rational::new(1, 3).unwrap(),
                    b: Rational::new(1, 2).unwrap(),
                    direction
                }
            );
        }
        let a_range = first.side(SeamSideId::A).range;
        let b_range = first.side(SeamSideId::B).range;
        let next_b = match direction {
            SeamDirection::Same => t(1, 4),
            SeamDirection::Opposite => t(3, 4),
        };
        f.definition.spans = vec![SeamSpanDefinition {
            id: id(302),
            a: first.side(SeamSideId::A),
            b: first.side(SeamSideId::B),
            direction,
            ease: first.clone().into(),
            stops: vec![],
        }];
        let intermediate = f.graph(&[]).unwrap();
        let request = SplitSeamSpan::new(
            id(302),
            EdgeAnchor {
                edge: a_range.edge(),
                param: t(1, 6),
            },
            EdgeAnchor {
                edge: b_range.edge(),
                param: next_b,
            },
        );
        let error = request
            .bind_source_cuts(&intermediate)
            .unwrap()
            .uniform_explicit_ease()
            .unwrap_err();
        let CommandUniformEaseError::OriginalSource { span, issue } = &error else {
            panic!("actual source refusal")
        };
        assert_eq!(*span, id(302));
        assert_eq!(error.source().unwrap().to_string(), issue.to_string());
    }
    let mut f = fixture();
    for amount in [
        EaseAmount::Formula(id(830)),
        EaseAmount::Profile(ProfileParameterRef::new(id(831))),
    ] {
        f.span_mut().ease.declared_mut().unwrap().amount = amount;
        let graph = f.graph(&[]).unwrap();
        let root = graph.spans().first().unwrap().retain_ease_source();
        assert_eq!(
            root.uniform_explicit_amount(),
            Err(Error::Amount {
                source_span: id(301),
                amount
            })
        );
    }
    f.span_mut().ease.declared_mut().unwrap().amount = EaseAmount::Explicit(Length::ZERO);
    let maximum = i64::MAX;
    let original =
        EdgeRange::new(f.span().a.range.edge(), t(1, maximum), t(2, maximum - 1)).unwrap();
    f.span_mut().a.range = original;
    let graph = f.graph(&[]).unwrap();
    let root = graph.spans().first().unwrap().retain_ease_source();
    let [portion, _] = root
        .split_at(
            EdgeAnchor {
                edge: original.edge(),
                param: t(1, maximum - 1),
            },
            EdgeAnchor {
                edge: f.span().b.range.edge(),
                param: t(1, 2),
            },
        )
        .unwrap();
    let error = portion.uniform_explicit_amount().unwrap_err();
    let Error::Progress {
        source_span,
        side,
        issue,
    } = &error
    else {
        panic!("actual complete normalization cause")
    };
    assert_eq!((*source_span, *side), (id(301), SeamSideId::A));
    assert_eq!(error.source().unwrap().to_string(), issue.to_string());
    assert!(matches!(
        error
            .source()
            .unwrap()
            .source()
            .unwrap()
            .downcast_ref::<sc_units::UnitError>(),
        Some(sc_units::UnitError::Overflow {
            operation: "Rational::checked_relative_progress"
        })
    ));
}

#[test]
fn retained_uniform_source_queries_preserve_original_frame_and_closed_domain_refusals_even_at_zero()
{
    use sc_core::ontology::{RangeProgressError, SeamEaseAmountError as Error};
    use std::error::Error as _;
    let mut f = fixture();
    let edge = f.span().a.range.edge();
    let domain = EdgeRange::new(edge, t(1, 7), t(6, 7)).unwrap();
    f.span_mut().a.range = domain;
    let graph = f.graph(&[]).unwrap();
    let root = graph.spans().first().unwrap().retain_ease_source();
    let source = root.source();
    assert_eq!(
        source.uniform_explicit_cumulative_at_a(domain.from()),
        Ok(Length::ZERO)
    );
    assert_eq!(
        source.uniform_explicit_cumulative_at_a(domain.to()),
        Ok(Length::ZERO)
    );
    assert_eq!(
        source.uniform_explicit_amount_in_a(domain),
        Ok(Length::ZERO)
    );
    let wrong = EdgeRange::whole(f.span().b.range.edge());
    assert_eq!(
        source.uniform_explicit_amount_in_a(wrong),
        Err(Error::IntervalSource {
            source_span: id(301),
            expected: edge,
            interval: wrong
        })
    );
    for (from, to, at) in [
        (Param::START, Param::END, Param::START),
        (t(1, 2), Param::END, Param::END),
    ] {
        let issue = RangeProgressError::Outside { range: domain, at };
        let error = source
            .uniform_explicit_amount_in_a(EdgeRange::new(edge, from, to).unwrap())
            .unwrap_err();
        assert_eq!(
            error,
            Error::Progress {
                source_span: id(301),
                side: SeamSideId::A,
                issue: Box::new(issue.clone())
            }
        );
        assert_eq!(error.source().unwrap().to_string(), issue.to_string());
    }
}
