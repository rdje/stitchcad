//! Actual six-family owned content; no source, operation, sewing or physical expansion.
use super::{limits, Error, Location, Meter, Scope, Unit};
use sc_core::{
    design::{
        charge_construction_inventory_resources, charge_sewing_graph_resources, DesignResourceSink,
    },
    ontology::{
        AllowanceWidth, ConstructionInventory, CopyOrientation, CornerTreatment, CutCopyDefinition,
        CutPlan, CuttingSide, Dart, DartDefinition, DeclaredEase, DeterministicIdGenerator,
        DirectedEdge, DirectedRange, Direction, EaseAmount, EaseDistribution, EdgeAnchor,
        EdgeRange, EntityId, FoldDefinition, Gather, GatherDefinition, GrainAlignment, GrainAngle,
        Grainline, GrainlineDefinition, IdentityLedger, IntakeAmount, LabelText,
        MaterialAssignment, Mirroring, Param, Piece, PieceDefinition, Pleat, ProfileParameterRef,
        Rational, SeamAllowance, SeamAllowanceDefinition, SeamDirection, SeamSide, SeamSideId,
        SeamSpanDefinition, SewingGraph, SewingGraphDefinition, Tuck, WeightedEaseRegion,
    },
};
use sc_units::{Angle, Length};
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
struct Fixture {
    inventory: ConstructionInventory,
    graph: SewingGraph,
    piece: Piece,
    ledger: IdentityLedger,
    ids: DeterministicIdGenerator,
}
fn fixture() -> Fixture {
    let mut ledger = IdentityLedger::new();
    let mut ids = DeterministicIdGenerator::new();
    let edges = ledger
        .declare_edges(&mut ids, 4)
        .expect("actual declared sources");
    let directed = |index| DirectedEdge {
        edge: *edges.get(index).expect("actual source edge"),
        direction: Direction::Original,
    };
    let pieces: Vec<_> = (0..2)
        .zip(100_u128..)
        .map(|(index, identity)| {
            Piece::new(
                PieceDefinition {
                    id: id(identity),
                    boundary: vec![directed(index)],
                    holes: vec![],
                    construction_lines: vec![directed(index + 2)],
                    quantity: 1,
                    mirroring: Mirroring::Single,
                    cut_on_fold: false,
                    fold_edges: vec![],
                    cutting_side: CuttingSide::Face,
                    material: MaterialAssignment::Unresolved("selection pending".into()),
                    layer_index: 0,
                    label: LabelText {
                        name: format!("source{identity}"),
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
    .expect("actual independent copies");
    let half = Param::new(Rational::new(1, 2).expect("half ratio")).expect("half parameter");
    let graph = SewingGraph::new(
        SewingGraphDefinition {
            id: id(300),
            spans: vec![SeamSpanDefinition {
                id: id(301),
                a: SeamSide {
                    copy: id(200),
                    range: EdgeRange::whole(directed(0).edge),
                },
                b: SeamSide {
                    copy: id(201),
                    range: EdgeRange::whole(directed(1).edge),
                },
                direction: SeamDirection::Opposite,
                ease: DeclaredEase {
                    amount: EaseAmount::Explicit(Length::ZERO),
                    distribution: EaseDistribution::Weighted {
                        side: SeamSideId::A,
                        regions: vec![
                            WeightedEaseRegion {
                                from: Param::START,
                                to: half,
                                weight: Rational::new(1, 1).expect("positive weight"),
                            },
                            WeightedEaseRegion {
                                from: half,
                                to: Param::END,
                                weight: Rational::new(2, 1).expect("positive weight"),
                            },
                        ],
                    },
                },
                stops: vec![],
            }],
        },
        &plan,
        &pieces,
        &[],
        &ledger,
    )
    .expect("actual graph with canonical weighted ease owner");
    let piece = pieces.first().expect("actual construction Piece");
    let boundary = directed(0).edge;
    let internal = directed(2).edge;
    let range = |edge| DirectedRange {
        range: EdgeRange::whole(edge),
        direction: Direction::Reversed,
    };
    let intakes = [
        IntakeAmount::Explicit {
            parameter: id(900),
            value: Length::ZERO,
        },
        IntakeAmount::Formula(id(901)),
        IntakeAmount::Profile(ProfileParameterRef::new(id(902))),
    ];
    let darts = intakes
        .into_iter()
        .zip(400_u128..)
        .map(|(intake, identity)| {
            Dart::new(
                DartDefinition {
                    id: id(identity),
                    intake,
                    apex: EdgeAnchor {
                        edge: internal,
                        param: Param::END,
                    },
                    first_leg: range(internal),
                    second_leg: range(boundary),
                    direction: range(internal),
                    closing_operation: id(903),
                },
                piece,
                &ledger,
            )
            .expect("actual Dart with symbolic/explicit intake")
        })
        .collect();
    let fold = |identity, intake, fold_lines| FoldDefinition {
        id: id(identity),
        intake,
        fold_lines,
        direction: range(boundary),
        closing_operation: id(904),
    };
    let tucks = vec![
        Tuck::new(fold(410, intakes[0], vec![range(internal)]), piece, &ledger)
            .expect("actual one-line Tuck"),
        Tuck::new(
            fold(411, intakes[1], vec![range(internal), range(boundary)]),
            piece,
            &ledger,
        )
        .expect("actual two-line Tuck"),
    ];
    let pleats = vec![Pleat::new(
        fold(420, intakes[2], vec![range(boundary), range(internal)]),
        piece,
        &ledger,
    )
    .expect("actual two-line Pleat")];
    let gathers = vec![Gather::new(
        GatherDefinition {
            id: id(430),
            graph: graph.id(),
            span: id(301),
            side: SeamSideId::A,
            copy: id(200),
            direction: range(internal),
            closing_operation: id(905),
        },
        piece,
        &plan,
        &graph,
        &ledger,
    )
    .expect("actual Gather borrows existing ease")];
    let alignments = [
        GrainAlignment::ParallelTo(range(boundary)),
        GrainAlignment::AtAngle {
            reference: range(boundary),
            angle: GrainAngle::Explicit(Angle::ZERO),
        },
        GrainAlignment::AtAngle {
            reference: range(boundary),
            angle: GrainAngle::Formula(id(906)),
        },
        GrainAlignment::AtAngle {
            reference: range(boundary),
            angle: GrainAngle::Profile(ProfileParameterRef::new(id(907))),
        },
    ];
    let grainlines = alignments
        .into_iter()
        .zip(440_u128..)
        .map(|(alignment, identity)| {
            Grainline::new(
                GrainlineDefinition {
                    id: id(identity),
                    arrow: range(internal),
                    alignment,
                    stripe: if identity % 2 == 0 {
                        Some(range(boundary))
                    } else {
                        None
                    },
                    plaid: if identity % 2 == 1 {
                        Some(range(internal))
                    } else {
                        None
                    },
                },
                piece,
                &ledger,
            )
            .expect("actual grain and optional print references")
        })
        .collect();
    let widths = [
        AllowanceWidth::Explicit {
            parameter: id(908),
            value: Length::ZERO,
        },
        AllowanceWidth::Formula(id(909)),
        AllowanceWidth::Profile(ProfileParameterRef::new(id(910))),
    ];
    let allowances = widths
        .into_iter()
        .zip([
            CornerTreatment::Miter,
            CornerTreatment::Envelope,
            CornerTreatment::Trim,
        ])
        .zip(450_u128..)
        .map(|((width, corner), identity)| {
            SeamAllowance::new(
                SeamAllowanceDefinition {
                    id: id(identity),
                    edge: boundary,
                    width,
                    corner,
                    inclusion: ProfileParameterRef::new(id(911)),
                },
                piece,
                &ledger,
            )
            .expect("actual allowance and logical inclusion intent")
        })
        .collect();
    let inventory =
        ConstructionInventory::new(darts, tucks, pleats, gathers, grainlines, allowances)
            .expect("actual six-family construction owner");
    Fixture {
        inventory,
        graph,
        piece: piece.clone(),
        ledger,
        ids,
    }
}
fn location(field: &'static str, owner: Option<u128>, index: Option<usize>) -> Location {
    Location {
        scope: Scope::Garment,
        owner: owner.map(id),
        field,
        index,
    }
}
#[test]
fn actual_six_families_and_fold_vectors_count_exact_content_without_changing_any_intent() {
    let f = fixture();
    let original = f.inventory.clone();
    let mut meter = Meter::new(limits(19, 1, 1));
    charge_construction_inventory_resources(&f.inventory, &mut meter)
        .expect("14 constructions and5 fold lines");
    let demand = meter.finish().expect("exact actual scope");
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (19, 0, 0)
    );
    assert_eq!(f.inventory, original);
}
#[test]
fn construction_family_and_nested_fold_limits_keep_original_cumulative_owner_and_index() {
    let f = fixture();
    for (maximum, consumed, requested, field, owner, index) in [
        (2, 0, 3, "constructions.darts", None, None),
        (4, 3, 2, "constructions.tucks", None, None),
        (
            5,
            5,
            1,
            "constructions.tucks.fold_lines",
            Some(410),
            Some(0),
        ),
        (
            7,
            6,
            2,
            "constructions.tucks.fold_lines",
            Some(411),
            Some(1),
        ),
        (8, 8, 1, "constructions.pleats", None, None),
        (
            10,
            9,
            2,
            "constructions.pleats.fold_lines",
            Some(420),
            Some(0),
        ),
        (11, 11, 1, "constructions.gathers", None, None),
        (15, 12, 4, "constructions.grainlines", None, None),
        (18, 16, 3, "constructions.allowances", None, None),
    ] {
        let original = f.inventory.clone();
        let mut meter = Meter::new(limits(maximum, 1, 1));
        let expected = Error::Count {
            unit: Unit::CollectionEntries,
            maximum,
            consumed,
            requested,
            location: location(field, owner, index),
        };
        assert_eq!(
            charge_construction_inventory_resources(&f.inventory, &mut meter),
            Err(expected)
        );
        assert_eq!(meter.finish(), Err(expected));
        assert_eq!(f.inventory, original);
    }
}
#[test]
fn gather_sewing_reference_does_not_recharge_the_canonical_weighted_ease_owner() {
    let f = fixture();
    let mut meter = Meter::new(limits(22, 1, 1));
    charge_construction_inventory_resources(&f.inventory, &mut meter)
        .expect("construction19 excludes referenced graph");
    charge_sewing_graph_resources(&f.graph, &mut meter)
        .expect("actual graph1 span+2 weighted regions counted once");
    assert_eq!(
        meter
            .finish()
            .expect("composed actual owners")
            .collection_entries(),
        22
    );
}
#[test]
fn real_deleted_source_keeps_exact_held_construction_data_without_rebirth_or_resolution() {
    let mut f = fixture();
    let edge = f
        .piece
        .definition()
        .construction_lines
        .first()
        .expect("actual held source")
        .edge;
    f.ledger
        .delete(&mut f.ids, edge)
        .expect("actual deletion retires held construction source");
    assert!(f
        .inventory
        .darts()
        .first()
        .expect("actual held Dart")
        .validate_current(&f.piece, &f.ledger)
        .is_err());
    let original = (f.inventory.clone(), f.graph.clone(), f.ledger.clone());
    let mut meter = Meter::new(limits(19, 1, 1));
    charge_construction_inventory_resources(&f.inventory, &mut meter)
        .expect("held accounting has no live source prerequisite");
    assert_eq!(meter.finish().expect("held scope").collection_entries(), 19);
    assert_eq!((f.inventory, f.graph, f.ledger), original);
}
#[test]
fn empty_construction_families_count_zero_and_preserve_original_prior_failure() {
    let empty = ConstructionInventory::new(vec![], vec![], vec![], vec![], vec![], vec![])
        .expect("empty draft families");
    let mut meter = Meter::new(limits(1, 1, 1));
    charge_construction_inventory_resources(&empty, &mut meter).expect("six empty vectors");
    let demand = meter.finish().expect("empty actual scope");
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (0, 0, 0)
    );
    let mut refused = Meter::new(limits(1, 1, 1));
    let first = Error::Count {
        unit: Unit::CollectionEntries,
        maximum: 1,
        consumed: 0,
        requested: 2,
        location: location("original", None, None),
    };
    assert_eq!(
        refused.charge_entries(2, location("original", None, None)),
        Err(first)
    );
    assert_eq!(
        charge_construction_inventory_resources(&fixture().inventory, &mut refused),
        Err(first)
    );
    assert_eq!(
        charge_construction_inventory_resources(&empty, &mut refused),
        Err(first)
    );
    assert_eq!(refused.finish(), Err(first));
}
