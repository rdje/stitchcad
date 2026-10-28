//! Actual immutable sewing contexts; logical/profile intent remains explicitly deferred.
use super::{limits, Error, Location, Meter, Scope, Unit};
use sc_core::{
    design::{charge_sewing_graph_resources, charge_sewing_landmark_resources, DesignResourceSink},
    ontology::{
        CopyOrientation, CutCopyDefinition, CutPlan, CuttingSide, DeclaredEase,
        DeterministicIdGenerator, DirectedEdge, Direction, EaseAmount, EaseDistribution,
        EdgeAnchor, EdgeRange, EntityId, IdentityLedger, LabelText, MaterialAssignment, Mirroring,
        Notch, NotchDefinition, NotchDimensionBindings, NotchProfileBindings, Param, Piece,
        PieceDefinition, ProfileParameterRef, Rational, SeamDirection, SeamSide, SeamSideId,
        SeamSpanDefinition, SewingGraph, SewingGraphDefinition, SewingLandmark, StopLandmark,
        TurnPoint, WeightedEaseRegion,
    },
};
use sc_units::Length;
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn t(n: i64, d: i64) -> Param {
    Param::new(Rational::new(n, d).expect("exact ratio")).expect("valid parameter")
}
fn fixture() -> (
    SewingGraph,
    Vec<SewingLandmark>,
    IdentityLedger,
    DeterministicIdGenerator,
) {
    let mut ledger = IdentityLedger::new();
    let mut ids = DeterministicIdGenerator::new();
    let edges = ledger
        .declare_edges(&mut ids, 2)
        .expect("actual source edges");
    let pieces: Vec<_> = edges
        .iter()
        .copied()
        .zip(100_u128..)
        .map(|(edge, identity)| {
            Piece::new(
                PieceDefinition {
                    id: id(identity),
                    boundary: vec![DirectedEdge {
                        edge,
                        direction: Direction::Original,
                    }],
                    holes: vec![],
                    construction_lines: vec![],
                    quantity: 1,
                    mirroring: Mirroring::Single,
                    cut_on_fold: false,
                    fold_edges: vec![],
                    cutting_side: CuttingSide::Face,
                    material: MaterialAssignment::Unresolved("selection pending".into()),
                    layer_index: 0,
                    label: LabelText {
                        name: format!("piece{identity}"),
                        size: "M".into(),
                        fabric: "woven".into(),
                        colorway: "plain".into(),
                    },
                },
                &ledger,
            )
            .expect("actual structurally born Piece")
        })
        .collect();
    let plan = CutPlan::new(
        pieces
            .iter()
            .zip(200_u128..)
            .map(|(piece, identity)| CutCopyDefinition {
                id: id(identity),
                piece: piece.id(),
                orientation: CopyOrientation::Authored,
            })
            .collect(),
        &pieces,
    )
    .expect("actual distinct physical copies");
    let first = pieces.first().expect("actual first Piece");
    let second = pieces.last().expect("actual second Piece");
    let a = *edges.first().expect("actual first edge");
    let b = *edges.last().expect("actual second edge");
    let bindings = NotchProfileBindings {
        style: ProfileParameterRef::new(id(5000)),
        sample: NotchDimensionBindings {
            depth: ProfileParameterRef::new(id(5001)),
            width: ProfileParameterRef::new(id(5002)),
        },
        production: NotchDimensionBindings {
            depth: ProfileParameterRef::new(id(5003)),
            width: ProfileParameterRef::new(id(5004)),
        },
        encoding: ProfileParameterRef::new(id(5005)),
    };
    let landmarks = vec![
        SewingLandmark::Notch(
            Notch::new(
                NotchDefinition {
                    id: id(400),
                    anchor: EdgeAnchor {
                        edge: a,
                        param: t(1, 4),
                    },
                    representation: bindings,
                },
                first,
                &ledger,
            )
            .expect("actual first semantic Notch"),
        ),
        SewingLandmark::Notch(
            Notch::new(
                NotchDefinition {
                    id: id(401),
                    anchor: EdgeAnchor {
                        edge: a,
                        param: t(3, 4),
                    },
                    representation: bindings,
                },
                first,
                &ledger,
            )
            .expect("actual second semantic Notch"),
        ),
        SewingLandmark::Turn(
            TurnPoint::new(
                id(402),
                EdgeAnchor {
                    edge: b,
                    param: t(1, 2),
                },
                second,
                &ledger,
            )
            .expect("actual semantic turn"),
        ),
    ];
    let side_a = SeamSide {
        copy: id(200),
        range: EdgeRange::whole(a),
    };
    let side_b = SeamSide {
        copy: id(201),
        range: EdgeRange::whole(b),
    };
    let notch_stops = vec![
        StopLandmark {
            side: SeamSideId::A,
            landmark: id(400),
        },
        StopLandmark {
            side: SeamSideId::A,
            landmark: id(401),
        },
    ];
    let spans = vec![
        SeamSpanDefinition {
            id: id(600),
            a: side_a,
            b: side_b,
            direction: SeamDirection::Same,
            ease: DeclaredEase {
                amount: EaseAmount::Explicit(Length::ZERO),
                distribution: EaseDistribution::Uniform,
            },
            stops: vec![StopLandmark {
                side: SeamSideId::B,
                landmark: id(402),
            }],
        },
        SeamSpanDefinition {
            id: id(601),
            a: side_a,
            b: side_b,
            direction: SeamDirection::Opposite,
            ease: DeclaredEase {
                amount: EaseAmount::Profile(ProfileParameterRef::new(id(5006))),
                distribution: EaseDistribution::Weighted {
                    side: SeamSideId::A,
                    regions: vec![
                        WeightedEaseRegion {
                            from: Param::START,
                            to: t(1, 2),
                            weight: Rational::new(1, 1).expect("positive density"),
                        },
                        WeightedEaseRegion {
                            from: t(1, 2),
                            to: Param::END,
                            weight: Rational::new(2, 1).expect("positive second density"),
                        },
                    ],
                },
            },
            stops: notch_stops.clone(),
        },
        SeamSpanDefinition {
            id: id(602),
            a: side_a,
            b: side_b,
            direction: SeamDirection::Same,
            ease: DeclaredEase {
                amount: EaseAmount::Formula(id(5007)),
                distribution: EaseDistribution::BetweenNotches {
                    side: SeamSideId::A,
                    start: id(400),
                    end: id(401),
                },
            },
            stops: notch_stops,
        },
    ];
    let graph = SewingGraph::new(
        SewingGraphDefinition { id: id(500), spans },
        &plan,
        &pieces,
        &landmarks,
        &ledger,
    )
    .expect("actual graph with all ease sources/distributions");
    (graph, landmarks, ledger, ids)
}
fn location(field: &'static str, owner: Option<u128>, index: Option<usize>) -> Location {
    Location {
        scope: Scope::Sewing,
        owner: owner.map(id),
        field,
        index,
    }
}
#[test]
fn actual_sewing_graph_and_landmarks_count_all_distributions_stops_and_symbolic_payloads_without_changes(
) {
    let (graph, landmarks, _, _) = fixture();
    let saved = (graph.clone(), landmarks.clone());
    let mut meter = Meter::new(limits(13, 1, 1));
    charge_sewing_graph_resources(&graph, &mut meter).expect("3 spans, 2 regions, 5 stops");
    charge_sewing_landmark_resources(&landmarks, &mut meter).expect("2 Notches and1 Turn");
    let demand = meter.finish().expect("exact actual component demand");
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (13, 0, 0)
    );
    assert_eq!((graph, landmarks), saved);
}
#[test]
fn sewing_precharge_stop_and_weighted_region_refusals_keep_actual_graph_span_and_index() {
    let (graph, _, _, _) = fixture();
    for (maximum, expected) in [
        (
            2,
            Error::Count {
                unit: Unit::CollectionEntries,
                maximum: 2,
                consumed: 0,
                requested: 3,
                location: location("sewing.spans", Some(500), None),
            },
        ),
        (
            3,
            Error::Count {
                unit: Unit::CollectionEntries,
                maximum: 3,
                consumed: 3,
                requested: 1,
                location: location("sewing.spans.stops", Some(600), Some(0)),
            },
        ),
        (
            5,
            Error::Count {
                unit: Unit::CollectionEntries,
                maximum: 5,
                consumed: 4,
                requested: 2,
                location: location("sewing.spans.ease.regions", Some(601), Some(1)),
            },
        ),
    ] {
        let saved = graph.clone();
        let mut meter = Meter::new(limits(maximum, 1, 1));
        assert_eq!(
            charge_sewing_graph_resources(&graph, &mut meter),
            Err(expected)
        );
        assert_eq!(meter.finish(), Err(expected));
        assert_eq!(graph, saved);
    }
}
#[test]
fn held_sewing_anchors_after_real_deletion_remain_inspectable_and_landmark_limits_stay_cumulative()
{
    let (graph, landmarks, mut ledger, mut ids) = fixture();
    let edge = graph
        .spans()
        .first()
        .expect("actual first span")
        .definition()
        .a
        .range
        .edge();
    ledger
        .delete(&mut ids, edge)
        .expect("actual later orphaning edit");
    assert!(!ledger.is_live(edge));
    let saved = (graph.clone(), landmarks.clone(), ledger.clone());
    let mut meter = Meter::new(limits(13, 1, 1));
    charge_sewing_graph_resources(&graph, &mut meter)
        .expect("held graph without current resolution");
    charge_sewing_landmark_resources(&landmarks, &mut meter)
        .expect("held Notch anchors without rebirth");
    assert_eq!(
        meter
            .finish()
            .expect("same held content")
            .collection_entries(),
        13
    );
    let mut limited = Meter::new(limits(12, 1, 1));
    charge_sewing_graph_resources(&graph, &mut limited).expect("10 graph entries");
    let expected = Error::Count {
        unit: Unit::CollectionEntries,
        maximum: 12,
        consumed: 10,
        requested: 3,
        location: location("sewing.landmarks", None, None),
    };
    assert_eq!(
        charge_sewing_landmark_resources(&landmarks, &mut limited),
        Err(expected)
    );
    assert_eq!(limited.finish(), Err(expected));
    assert_eq!((graph, landmarks, ledger), saved);
}
#[test]
fn actual_empty_sewing_scopes_keep_zero_demand_and_an_original_prior_refusal() {
    let ledger = IdentityLedger::new();
    let plan = CutPlan::new(vec![], &[]).expect("actual empty physical draft");
    let graph = SewingGraph::new(
        SewingGraphDefinition {
            id: id(500),
            spans: vec![],
        },
        &plan,
        &[],
        &[],
        &ledger,
    )
    .expect("actual empty sewing draft");
    let mut meter = Meter::new(limits(1, 1, 1));
    charge_sewing_graph_resources(&graph, &mut meter).expect("empty graph");
    charge_sewing_landmark_resources(&[], &mut meter).expect("empty landmarks");
    let demand = meter.finish().expect("empty actual scope");
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (0, 0, 0)
    );
    let mut meter = Meter::new(limits(1, 1, 1));
    let original = meter
        .charge_text("é", location("preceding", None, None))
        .expect_err("first prior refusal");
    assert_eq!(
        charge_sewing_graph_resources(&graph, &mut meter),
        Err(original)
    );
    assert_eq!(
        charge_sewing_landmark_resources(&[], &mut meter),
        Err(original)
    );
    assert_eq!(meter.finish(), Err(original));
}
