//! Gather binds stable physical material and borrows the sewing span's canonical ease source.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
use sc_core::ontology::{
    CopyOrientation, CutCopyDefinition, CutPlan, CuttingSide, DeclaredEase,
    DeterministicIdGenerator, DirectedEdge, DirectedRange, Direction, EaseAmount, EaseDistribution,
    EdgeAnchor, EdgeRange, EdgeRef, EntityId, Gather, GatherDefinition, GatherError,
    GatherReferenceRole, GeometricValidation, IdentityLedger, IntakeValidation, LabelText,
    LocalTag, MaterialAssignment, Mirroring, Notch, NotchDefinition, NotchDimensionBindings,
    NotchProfileBindings, Param, Piece, PieceDefinition, ProfileBindingValidation,
    ProfileParameterRef, Rational, SeamDirection, SeamSide, SeamSideId, SeamSpanDefinition,
    SewingError, SewingGraph, SewingGraphDefinition, SewingLandmark, StopLandmark,
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

impl Fixture {
    fn gather_definition(&self, side: SeamSideId) -> GatherDefinition {
        let index = match side {
            SeamSideId::A => 0,
            SeamSideId::B => 1,
        };
        let selected = match side {
            SeamSideId::A => self.span().a,
            SeamSideId::B => self.span().b,
        };
        GatherDefinition {
            id: id(400),
            graph: self.definition.id,
            span: self.span().id,
            side,
            copy: selected.copy,
            direction: DirectedRange {
                range: EdgeRange::whole(
                    self.pieces
                        .get(index)
                        .unwrap()
                        .definition()
                        .construction_lines
                        .first()
                        .unwrap()
                        .edge,
                ),
                direction: Direction::Original,
            },
            closing_operation: id(450),
        }
    }
    fn gather(&self, input: GatherDefinition, graph: &SewingGraph) -> Result<Gather, GatherError> {
        let index = match input.side {
            SeamSideId::A => 0,
            SeamSideId::B => 1,
        };
        Gather::new(
            input,
            self.pieces.get(index).unwrap(),
            &self.plan,
            graph,
            &self.ledger,
        )
    }
}
#[test]
fn immutable_gather_retains_physical_target_and_borrows_existing_intake_and_allocation() {
    let f = fixture();
    let graph = f.graph(&[]).unwrap();
    let input = f.gather_definition(SeamSideId::A);
    let gather = f.gather(input.clone(), &graph).unwrap();
    assert_eq!(gather.id(), input.id);
    assert_eq!(gather.piece(), f.pieces.first().unwrap().id());
    assert_eq!(gather.definition(), &input);
    assert_eq!(gather.copy(&f.plan).unwrap().id(), input.copy);
    let source = gather.intake_source(&graph).unwrap();
    assert_eq!(source.gathered_side, SeamSideId::A);
    assert!(std::ptr::eq(
        source.declaration,
        &graph.spans().first().unwrap().definition().ease
    ));
    assert_eq!(
        gather.attachment_resolution(&graph, &f.ledger).unwrap(),
        graph
            .spans()
            .first()
            .unwrap()
            .resolve_side(SeamSideId::A, &f.ledger)
    );
    assert_eq!(
        gather.geometric_validation(),
        GeometricValidation::DeferredToG2
    );
    assert_eq!(
        gather.intake_validation(),
        IntakeValidation::DeferredToG2AndG3
    );
    assert_eq!(gather.profile_binding_validation(&graph).unwrap(), None);
}
#[test]
fn gathered_side_requires_the_correct_explicit_sign_with_authored_zero_legal_for_either_side() {
    let mut f = fixture();
    for (side, value, valid) in [
        (SeamSideId::A, 1, true),
        (SeamSideId::A, -1, false),
        (SeamSideId::B, -1, true),
        (SeamSideId::B, 1, false),
        (SeamSideId::A, 0, true),
        (SeamSideId::B, 0, true),
    ] {
        let differential = Length::from_micrometres(value).unwrap();
        f.span_mut().ease.amount = EaseAmount::Explicit(differential);
        let graph = f.graph(&[]).unwrap();
        let result = f.gather(f.gather_definition(side), &graph);
        if valid {
            let source = result.unwrap().intake_source(&graph).unwrap();
            assert_eq!(
                source.declaration.amount,
                EaseAmount::Explicit(differential)
            );
            assert_eq!(source.gathered_side, side);
        } else {
            assert_eq!(
                result,
                Err(GatherError::IncompatibleEaseSign { side, differential })
            );
        }
    }
}
#[test]
fn symbolic_sources_supply_no_value_or_default_and_profile_deferral_is_visible() {
    let mut f = fixture();
    for amount in [
        EaseAmount::Formula(id(600)),
        EaseAmount::Profile(ProfileParameterRef::new(id(601))),
    ] {
        f.span_mut().ease.amount = amount;
        let graph = f.graph(&[]).unwrap();
        for side in [SeamSideId::A, SeamSideId::B] {
            let gather = f.gather(f.gather_definition(side), &graph).unwrap();
            assert_eq!(
                gather.intake_source(&graph).unwrap().declaration.amount,
                amount
            );
            assert_eq!(
                gather.profile_binding_validation(&graph).unwrap(),
                if matches!(amount, EaseAmount::Profile(_)) {
                    Some(ProfileBindingValidation::DeferredToG4)
                } else {
                    None
                }
            );
            assert_eq!(
                gather.intake_validation(),
                IntakeValidation::DeferredToG2AndG3
            );
        }
    }
}
#[test]
fn uniform_weighted_and_between_notch_allocation_have_one_canonical_graph_source() {
    let mut f = fixture();
    let marks = vec![f.notch(700, t(1, 4)), f.notch(701, t(3, 4))];
    f.span_mut().stops = vec![
        StopLandmark {
            side: SeamSideId::A,
            landmark: id(700),
        },
        StopLandmark {
            side: SeamSideId::A,
            landmark: id(701),
        },
    ];
    for distribution in [
        EaseDistribution::Uniform,
        EaseDistribution::Weighted {
            side: SeamSideId::A,
            regions: vec![WeightedEaseRegion {
                from: Param::START,
                to: Param::END,
                weight: Rational::new(2, 1).unwrap(),
            }],
        },
        EaseDistribution::BetweenNotches {
            side: SeamSideId::A,
            start: id(700),
            end: id(701),
        },
    ] {
        f.span_mut().ease.distribution = distribution.clone();
        let graph = f.graph(&marks).unwrap();
        let gather = f
            .gather(f.gather_definition(SeamSideId::A), &graph)
            .unwrap();
        let source = gather.intake_source(&graph).unwrap();
        assert_eq!(source.declaration.distribution, distribution);
        assert!(std::ptr::eq(
            source.declaration,
            &graph.spans().first().unwrap().definition().ease
        ));
    }
}
#[test]
fn wrong_graph_missing_span_and_changed_side_copy_are_refused_without_retargeting() {
    let mut f = fixture();
    let graph = f.graph(&[]).unwrap();
    let input = f.gather_definition(SeamSideId::A);
    let gather = f.gather(input.clone(), &graph).unwrap();
    f.definition.id = id(399);
    let wrong = f.graph(&[]).unwrap();
    assert_eq!(
        gather.intake_source(&wrong),
        Err(GatherError::WrongGraph {
            held: input.graph,
            provided: id(399)
        })
    );
    f.definition.id = input.graph;
    f.span_mut().id = id(398);
    let missing = f.graph(&[]).unwrap();
    assert_eq!(
        gather.intake_source(&missing),
        Err(GatherError::MissingSpan(input.span))
    );
    f.span_mut().id = input.span;
    f.span_mut().a.copy = id(201);
    let changed = f.graph(&[]).unwrap();
    assert_eq!(
        gather.intake_source(&changed),
        Err(GatherError::CopyBindingChanged {
            held: id(200),
            current: id(201)
        })
    );
    assert_eq!(
        gather.attachment_resolution(&changed, &f.ledger),
        Err(GatherError::CopyBindingChanged {
            held: id(200),
            current: id(201)
        })
    );
    assert_eq!(
        f.gather(input.clone(), &changed),
        Err(GatherError::CopyBindingChanged {
            held: id(200),
            current: id(201)
        })
    );
    assert_eq!(gather.definition(), &input);
}
#[test]
fn missing_or_reassigned_physical_copy_stays_a_typed_target_failure() {
    let f = fixture();
    let graph = f.graph(&[]).unwrap();
    let input = f.gather_definition(SeamSideId::A);
    let gather = f.gather(input.clone(), &graph).unwrap();
    let changed = f
        .plan
        .copies()
        .iter()
        .map(|copy| {
            let mut d = *copy.definition();
            if d.id == id(200) {
                d.id = id(205);
            }
            d
        })
        .collect();
    let plan = CutPlan::new(changed, &f.pieces).unwrap();
    assert_eq!(gather.copy(&plan), Err(GatherError::MissingCopy(id(200))));
    let reassigned = CutPlan::new(
        vec![
            CutCopyDefinition {
                id: id(200),
                piece: id(101),
                orientation: CopyOrientation::Authored,
            },
            CutCopyDefinition {
                id: id(201),
                piece: id(100),
                orientation: CopyOrientation::Authored,
            },
            CutCopyDefinition {
                id: id(202),
                piece: id(100),
                orientation: CopyOrientation::Reflected,
            },
        ],
        &f.pieces,
    )
    .unwrap();
    let expected = GatherError::CopyOutsidePiece {
        copy: id(200),
        expected: id(100),
        actual: id(101),
    };
    assert_eq!(gather.copy(&reassigned), Err(expected.clone()));
    assert_eq!(
        Gather::new(
            input,
            f.pieces.first().unwrap(),
            &reassigned,
            &graph,
            &f.ledger
        ),
        Err(expected)
    );
}
#[test]
fn direction_unknown_foreign_and_hidden_merged_interiors_are_not_owned_by_endpoint_coincidence() {
    let mut f = fixture();
    let graph = f.graph(&[]).unwrap();
    let mut input = f.gather_definition(SeamSideId::A);
    let missing = EdgeRef::new(id(u128::MAX), LocalTag::FIRST);
    input.direction.range = EdgeRange::whole(missing);
    assert!(matches!(
        f.gather(input.clone(), &graph),
        Err(GatherError::UnresolvedReference {
            role: GatherReferenceRole::Direction,
            ..
        })
    ));
    let foreign = *f
        .ledger
        .declare_edges(&mut f.ids, 1)
        .unwrap()
        .first()
        .unwrap();
    input.direction.range = EdgeRange::whole(foreign);
    assert_eq!(
        f.gather(input.clone(), &graph),
        Err(GatherError::OutsidePiece {
            role: GatherReferenceRole::Direction,
            piece: id(100)
        })
    );
    let left = f.gather_definition(SeamSideId::A).direction.range.edge();
    let right = f.span().a.range.edge();
    let unit = Length::from_micrometres(1).unwrap();
    let first = f
        .ledger
        .merge(&mut f.ids, left, unit, foreign, unit)
        .unwrap();
    let merged = f
        .ledger
        .merge(
            &mut f.ids,
            first,
            Length::from_micrometres(2).unwrap(),
            right,
            unit,
        )
        .unwrap();
    input.direction.range = EdgeRange::whole(merged);
    assert_eq!(
        f.gather(input.clone(), &graph),
        Err(GatherError::OutsidePiece {
            role: GatherReferenceRole::Direction,
            piece: id(100)
        })
    );
    f.ledger.reverse(&mut f.ids, merged).unwrap();
    assert!(matches!(
        f.gather(input, &graph),
        Err(GatherError::OutsidePiece {
            role: GatherReferenceRole::Direction,
            ..
        })
    ));
}
#[test]
fn attachment_interior_deletion_remains_visible_despite_live_held_endpoints() {
    let mut f = fixture();
    let graph = f.graph(&[]).unwrap();
    let input = f.gather_definition(SeamSideId::A);
    let gather = f.gather(input.clone(), &graph).unwrap();
    let edge = f.span().a.range.edge();
    let first = f.ledger.split(&mut f.ids, edge, t(1, 3)).unwrap();
    let second = f.ledger.split(&mut f.ids, first.second(), t(1, 2)).unwrap();
    f.ledger.delete(&mut f.ids, second.first()).unwrap();
    let evidence = gather.attachment_resolution(&graph, &f.ledger).unwrap();
    assert!(evidence.start().resolved().is_some());
    assert!(evidence.end().resolved().is_some());
    assert_eq!(evidence.repairs().count(), 1);
    assert!(
        matches!(f.gather(input,&graph),Err(GatherError::UnresolvedReference{role:GatherReferenceRole::Attachment,evidence:e}) if *e==evidence)
    );
}
#[test]
fn attachment_split_endpoint_choice_is_not_selected_by_a_valid_span_identity() {
    let mut f = fixture();
    let edge = f.span().a.range.edge();
    f.span_mut().a.range = EdgeRange::new(edge, t(1, 2), Param::END).unwrap();
    let graph = f.graph(&[]).unwrap();
    let input = f.gather_definition(SeamSideId::A);
    f.ledger.split(&mut f.ids, edge, t(1, 2)).unwrap();
    assert!(
        matches!(f.gather(input,&graph),Err(GatherError::UnresolvedReference{role:GatherReferenceRole::Attachment,evidence}) if evidence.has_full_coverage() && evidence.start().resolved().is_none())
    );
}
#[test]
fn direction_queries_and_input_replacement_leave_original_definition_unchanged() {
    let mut f = fixture();
    let graph = f.graph(&[]).unwrap();
    let mut input = f.gather_definition(SeamSideId::A);
    input.direction.direction = Direction::Reversed;
    let gather = f.gather(input.clone(), &graph).unwrap();
    let edge = input.direction.range.edge();
    let parts = f.ledger.split(&mut f.ids, edge, t(1, 2)).unwrap();
    f.ledger.reverse(&mut f.ids, parts.second()).unwrap();
    let evidence = gather.direction_resolution(&f.ledger);
    assert_eq!(
        evidence.portions().next().unwrap().direction,
        Direction::Original
    );
    assert_eq!(evidence.evidence().repairs().count(), 0);
    let mut changed = input.clone();
    changed.direction.direction = Direction::Original;
    changed.closing_operation = id(451);
    let replacement = f.gather(changed, &graph).unwrap();
    assert_ne!(gather, replacement);
    assert_eq!(gather.definition(), &input);
}
#[test]
fn a_changed_explicit_ease_sign_is_detected_by_current_target_queries() {
    let mut f = fixture();
    let graph = f.graph(&[]).unwrap();
    let gather = f
        .gather(f.gather_definition(SeamSideId::A), &graph)
        .unwrap();
    let negative = Length::from_micrometres(-1).unwrap();
    f.span_mut().ease.amount = EaseAmount::Explicit(negative);
    let changed = f.graph(&[]).unwrap();
    let expected = GatherError::IncompatibleEaseSign {
        side: SeamSideId::A,
        differential: negative,
    };
    assert_eq!(gather.intake_source(&changed), Err(expected.clone()));
    assert_eq!(gather.profile_binding_validation(&changed), Err(expected));
}
