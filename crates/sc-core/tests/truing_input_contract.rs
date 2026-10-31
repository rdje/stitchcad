//! Actual seam/copy/provider association;no test fixture supplies a fitted curve or geometry proof.
#![allow(clippy::expect_used)]
use sc_core::{construction::*, ontology::*};
use sc_units::{Length, Tolerance, ToleranceClass};
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn half() -> Param {
    Param::new(Rational::new(1, 2).expect("half")).expect("unit interval")
}
fn tolerance(class: ToleranceClass, limit: i64) -> Tolerance {
    Tolerance::new(
        class,
        Length::from_micrometres(limit).expect("length domain"),
        "test-supplied length-fit requirement;not geometry or approval evidence",
    )
    .expect("explicit derivation")
}
fn request(span: EntityId, side: SeamSideId, piece: EntityId) -> ConstrainedSeamTruing {
    ConstrainedSeamTruing::new(id(1000), span, side, piece, id(9000)).expect("external sources")
}
struct Fixture {
    ledger: IdentityLedger,
    pieces: Vec<Piece>,
    plan: CutPlan,
    graph: SewingGraph,
    marks: Vec<SewingLandmark>,
}
fn first_span(graph: &SewingGraph) -> &SeamSpan {
    graph.spans().first().expect("canonical source span")
}
fn fixture(shared: bool) -> Fixture {
    let mut ledger = IdentityLedger::new();
    let mut ids = DeterministicIdGenerator::new();
    let [a, b]: [EdgeRef; 2] = ledger
        .declare_edges(&mut ids, 2)
        .expect("source edges")
        .try_into()
        .expect("two declared sources");
    let directed = |edge| DirectedEdge {
        edge,
        direction: Direction::Original,
    };
    let make_piece = |identity, boundary, quantity, mirroring| {
        Piece::new(
            PieceDefinition {
                id: identity,
                boundary,
                holes: vec![],
                construction_lines: vec![],
                quantity,
                mirroring,
                cut_on_fold: false,
                fold_edges: vec![],
                cutting_side: CuttingSide::Face,
                material: MaterialAssignment::Unresolved("test material".into()),
                layer_index: 0,
                label: LabelText {
                    name: "test".into(),
                    size: "M".into(),
                    fabric: "woven".into(),
                    colorway: "plain".into(),
                },
            },
            &ledger,
        )
        .expect("structural Piece")
    };
    let pieces = if shared {
        vec![make_piece(
            id(100),
            vec![directed(a), directed(b)],
            2,
            Mirroring::MirroredPairs,
        )]
    } else {
        vec![
            make_piece(id(100), vec![directed(a)], 1, Mirroring::Single),
            make_piece(id(101), vec![directed(b)], 1, Mirroring::Single),
        ]
    };
    let plan = CutPlan::new(
        vec![
            CutCopyDefinition {
                id: id(201),
                piece: if shared { id(100) } else { id(101) },
                orientation: if shared {
                    CopyOrientation::Reflected
                } else {
                    CopyOrientation::Authored
                },
            },
            CutCopyDefinition {
                id: id(200),
                piece: id(100),
                orientation: CopyOrientation::Authored,
            },
        ],
        &pieces,
    )
    .expect("actual supplied copy plan");
    let marks = vec![SewingLandmark::Turn(
        TurnPoint::new(
            id(401),
            EdgeAnchor {
                edge: a,
                param: Param::START,
            },
            pieces.first().expect("source Piece"),
            &ledger,
        )
        .expect("actual semantic stop"),
    )];
    let graph = SewingGraph::new(
        SewingGraphDefinition {
            id: id(300),
            spans: vec![SeamSpanDefinition {
                id: id(301),
                a: SeamSide {
                    copy: id(200),
                    range: EdgeRange::whole(a),
                },
                b: SeamSide {
                    copy: id(201),
                    range: EdgeRange::whole(b),
                },
                direction: SeamDirection::Opposite,
                ease: DeclaredEase {
                    amount: EaseAmount::Profile(ProfileParameterRef::new(id(8000))),
                    distribution: EaseDistribution::Uniform,
                }
                .into(),
                stops: vec![StopLandmark {
                    side: SeamSideId::A,
                    landmark: id(401),
                }],
            }],
        },
        &plan,
        &pieces,
        &marks,
        &ledger,
    )
    .expect("actual canonical graph");
    Fixture {
        ledger,
        pieces,
        plan,
        graph,
        marks,
    }
}
#[test]
fn both_adjusted_sides_borrow_actual_request_span_copy_fields_and_quantitative_requirement() {
    let f = fixture(false);
    let fit = tolerance(ToleranceClass::GeometricApproximation, 7);
    let original = f.graph.clone();
    let plan = f.plan.clone();
    let source = first_span(&f.graph);
    for (side, piece, copy) in [
        (SeamSideId::A, id(100), id(200)),
        (SeamSideId::B, id(101), id(201)),
    ] {
        let edit = request(id(301), side, piece);
        let before = edit;
        let provider = TruingFitProvider {
            source: id(9000),
            tolerance: &fit,
        };
        let bound = edit
            .bind_source(&f.graph, &f.plan, provider)
            .expect("actual associated sources");
        assert!(std::ptr::eq(bound.edit(), &edit));
        assert!(std::ptr::eq(bound.graph(), &f.graph));
        assert!(std::ptr::eq(bound.plan(), &f.plan));
        assert!(std::ptr::eq(bound.span(), source));
        assert!(std::ptr::eq(
            bound.selected_copy(),
            f.plan.copy(copy).expect("real selected copy")
        ));
        let (selected, other) = match side {
            SeamSideId::A => (&source.definition().a, &source.definition().b),
            SeamSideId::B => (&source.definition().b, &source.definition().a),
        };
        assert!(std::ptr::eq(bound.source_side(), selected));
        assert!(std::ptr::eq(bound.counterpart_side(), other));
        assert!(std::ptr::eq(bound.fit_provider().tolerance, &fit));
        assert_eq!(bound.fit_provider(), provider);
        assert_eq!(
            bound.fit_provider().tolerance.derivation(),
            fit.derivation()
        );
        assert_eq!(
            bound.affected_copies().collect::<Vec<_>>(),
            vec![bound.selected_copy()]
        );
        assert_eq!(edit.operation(), id(1000));
        assert_eq!(edit.span_id(), id(301));
        assert_eq!(edit.piece_id(), piece);
        assert_eq!(edit.adjusted_side(), side);
        assert_eq!(edit.fit_tolerance_source(), id(9000));
        assert_eq!(edit, before);
        assert_eq!(f.graph, original);
        assert_eq!(f.plan, plan);
        assert_eq!(
            bound.span().definition().stops,
            vec![StopLandmark {
                side: SeamSideId::A,
                landmark: id(401)
            }]
        );
        assert_eq!(
            bound.span().definition().ease.declaration().amount,
            EaseAmount::Profile(ProfileParameterRef::new(id(8000)))
        );
    }
}
#[test]
fn shared_piece_keeps_both_physical_copies_in_original_plan_order_without_freezing_counterpart() {
    let f = fixture(true);
    let fit = tolerance(ToleranceClass::GeometricApproximation, 0);
    for side in [SeamSideId::A, SeamSideId::B] {
        let edit = request(id(301), side, id(100));
        let bound = edit
            .bind_source(
                &f.graph,
                &f.plan,
                TruingFitProvider {
                    source: id(9000),
                    tolerance: &fit,
                },
            )
            .expect("explicit zero/shared Piece input");
        let copies = bound.affected_copies().collect::<Vec<_>>();
        assert_eq!(copies.len(), 2);
        for (actual, original) in copies.iter().zip(f.plan.copies()) {
            assert!(std::ptr::eq(*actual, original));
        }
        let [reflected, authored]: [&CutCopy; 2] = copies.try_into().expect("both actual copies");
        assert_eq!(reflected.id(), id(201));
        assert_eq!(reflected.orientation(), CopyOrientation::Reflected);
        assert_eq!(authored.id(), id(200));
        assert_eq!(authored.orientation(), CopyOrientation::Authored);
        assert_eq!(
            f.plan
                .copy(bound.counterpart_side().copy)
                .expect("counterpart copy")
                .piece(),
            id(100)
        );
    }
}
#[test]
fn actual_binding_refusals_preserve_all_owners_and_source_class_domain_precedence() {
    use TruingBindingRefusal as R;
    let f = fixture(false);
    let empty = CutPlan::new(vec![], &[]).expect("empty structural plan");
    let wrong = tolerance(ToleranceClass::PhysicalAcceptance, -1);
    let valid = tolerance(ToleranceClass::GeometricApproximation, 1);
    let negative = tolerance(ToleranceClass::GeometricApproximation, -1);
    let rows = [
        (
            request(id(999), SeamSideId::A, id(999)),
            &empty,
            id(999),
            &wrong,
            R::MissingSpan,
        ),
        (
            request(id(301), SeamSideId::B, id(999)),
            &empty,
            id(999),
            &wrong,
            R::MissingCopy,
        ),
        (
            request(id(301), SeamSideId::B, id(100)),
            &f.plan,
            id(999),
            &wrong,
            R::Piece,
        ),
        (
            request(id(301), SeamSideId::B, id(101)),
            &f.plan,
            id(999),
            &wrong,
            R::ToleranceSource,
        ),
        (
            request(id(301), SeamSideId::B, id(101)),
            &f.plan,
            id(9000),
            &wrong,
            R::ToleranceClass,
        ),
        (
            request(id(301), SeamSideId::B, id(101)),
            &f.plan,
            id(9000),
            &negative,
            R::NegativeTolerance,
        ),
    ];
    for (edit, plan, source, tolerance, expected) in rows {
        let provider = TruingFitProvider { source, tolerance };
        let before = f.graph.clone();
        let error = edit
            .bind_source(&f.graph, plan, provider)
            .expect_err("actual refusal");
        assert!(std::ptr::eq(error.edit(), &edit));
        assert!(std::ptr::eq(error.graph(), &f.graph));
        assert!(std::ptr::eq(error.plan(), plan));
        assert!(std::ptr::eq(error.fit_provider().tolerance, tolerance));
        assert_eq!(error.fit_provider(), provider);
        assert_eq!(error.refusal(), expected);
        assert_eq!(f.graph, before);
    }
    let edit = request(id(301), SeamSideId::A, id(100));
    for class in [
        ToleranceClass::Numerical,
        ToleranceClass::GeometricChordal,
        ToleranceClass::FormatQuantization,
        ToleranceClass::ImporterComparison,
        ToleranceClass::PhysicalAcceptance,
    ] {
        let fit = tolerance(class, 0);
        let error = edit
            .bind_source(
                &f.graph,
                &f.plan,
                TruingFitProvider {
                    source: id(9000),
                    tolerance: &fit,
                },
            )
            .expect_err("wrong quantitative role");
        assert_eq!(error.refusal(), R::ToleranceClass);
    }
    for fit in [
        &valid,
        &tolerance(ToleranceClass::GeometricApproximation, 1_000_000_000),
    ] {
        edit.bind_source(
            &f.graph,
            &f.plan,
            TruingFitProvider {
                source: id(9000),
                tolerance: fit,
            },
        )
        .expect("admitted domain;not approved requirement");
    }
}
#[test]
fn local_sources_refuse_in_declared_role_order_without_inventing_global_namespace_checks() {
    for (span, piece, fit, role) in [
        (1000, 1000, 1000, TruingInputRole::Span),
        (301, 1000, 1000, TruingInputRole::Piece),
        (301, 100, 1000, TruingInputRole::FitTolerance),
    ] {
        let error =
            ConstrainedSeamTruing::new(id(1000), id(span), SeamSideId::B, id(piece), id(fit))
                .expect_err("local self-source");
        assert_eq!(
            error,
            TruingInputError {
                operation: id(1000),
                role
            }
        );
    }
    let retained = ConstrainedSeamTruing::new(id(1000), id(301), SeamSideId::A, id(301), id(301))
        .expect("global role collision is not a local cycle");
    assert_eq!(retained.span_id(), retained.piece_id());
    assert_eq!(retained.piece_id(), retained.fit_tolerance_source());
}
#[test]
fn retained_partial_ease_and_stale_geometry_are_not_replaced_by_full_parent_or_binding_success() {
    let mut f = fixture(false);
    let source = first_span(&f.graph);
    let d = source.definition();
    let [portion, _] = source
        .retain_ease_source()
        .split_at(
            EdgeAnchor {
                edge: d.a.range.edge(),
                param: half(),
            },
            EdgeAnchor {
                edge: d.b.range.edge(),
                param: half(),
            },
        )
        .expect("actual original source partition");
    let graph = SewingGraph::new(
        SewingGraphDefinition {
            id: id(302),
            spans: vec![SeamSpanDefinition {
                id: id(303),
                a: portion.side(SeamSideId::A),
                b: portion.side(SeamSideId::B),
                direction: d.direction,
                ease: portion.into(),
                stops: d.stops.clone(),
            }],
        },
        &f.plan,
        &f.pieces,
        &f.marks,
        &f.ledger,
    )
    .expect("actual retained partial span");
    let edit = request(id(303), SeamSideId::A, id(100));
    let fit = tolerance(ToleranceClass::GeometricApproximation, 3);
    let provider = TruingFitProvider {
        source: id(9000),
        tolerance: &fit,
    };
    let bound = edit
        .bind_source(&graph, &f.plan, provider)
        .expect("actual retained ease binding");
    let actual = bound
        .span()
        .definition()
        .ease
        .portion()
        .expect("still a portion");
    assert!(std::ptr::eq(
        actual,
        first_span(&graph)
            .definition()
            .ease
            .portion()
            .expect("original portion field")
    ));
    assert_eq!(actual.side(SeamSideId::A).range.to(), half());
    assert_eq!(actual.source().side(SeamSideId::A).range.to(), Param::END);
    assert_eq!(
        graph.validate_current(&f.plan, &f.pieces, &f.marks, &f.ledger),
        Ok(())
    );
    f.ledger
        .delete(
            &mut DeterministicIdGenerator::from_start(10000),
            actual.side(SeamSideId::A).range.edge(),
        )
        .expect("delete current source");
    assert!(matches!(
        graph.validate_current(&f.plan, &f.pieces, &f.marks, &f.ledger),
        Err(SewingError::UnresolvedRange {
            side: SeamSideId::A,
            ..
        })
    ));
    assert!(std::ptr::eq(
        edit.bind_source(&graph, &f.plan, provider)
            .expect("structural association only")
            .span(),
        first_span(&graph)
    ));
    assert_eq!(
        graph.geometric_validation(),
        GeometricValidation::DeferredToG2
    );
}
