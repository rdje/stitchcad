//! Actual Pocket/notion/Closure content; no registry, geometry or physical substitutes.
use super::{limits, Error, Location, Meter, Scope, Unit};
use sc_core::{
    design::{
        charge_closure_inventory_resources, charge_notion_placement_inventory_resources,
        charge_pocket_inventory_resources, DesignResourceSink,
    },
    ontology::{
        ButtonholeDerivation, Closure, ClosureDefinition, ClosureInstance, ClosureInventory,
        ClosureKind, ClosureLength, CopyOrientation, CutCopyDefinition, CutPlan, CuttingSide,
        DeterministicIdGenerator, DirectedEdge, DirectedRange, Direction, EdgeAnchor, EdgeRange,
        EntityId, IdentityLedger, LabelText, MaterialAssignment, Mirroring, NotionPlacement,
        NotionPlacementDefinition, NotionPlacementInventory, NotionSize, Param, Piece,
        PieceDefinition, Pocket, PocketDefinition, PocketInventory, PocketOpening, PocketPieceRef,
        ProfileParameterRef,
    },
};
use sc_units::Length;
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
struct Fixture {
    pockets: PocketInventory,
    notions: NotionPlacementInventory,
    closures: ClosureInventory,
    pieces: Vec<Piece>,
    plan: CutPlan,
    ledger: IdentityLedger,
    ids: DeterministicIdGenerator,
}
fn fixture() -> Fixture {
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
                    quantity: 2,
                    mirroring: Mirroring::MirroredPairs,
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
            .expect("actual born Piece")
        })
        .collect();
    let copies = pieces
        .iter()
        .flat_map(|piece| {
            [CopyOrientation::Authored, CopyOrientation::Reflected]
                .into_iter()
                .map(move |orientation| (piece, orientation))
        })
        .zip(200_u128..)
        .map(|((piece, orientation), identity)| CutCopyDefinition {
            id: id(identity),
            piece: piece.id(),
            orientation,
        })
        .collect();
    let plan = CutPlan::new(copies, &pieces).expect("four actual cut copies");
    let served = pieces.first().expect("actual served Piece");
    let edge = *edges.first().expect("actual held source edge");
    let anchor = EdgeAnchor {
        edge,
        param: Param::START,
    };
    let direction = DirectedRange {
        range: EdgeRange::whole(edge),
        direction: Direction::Reversed,
    };
    let component = |copy| PocketPieceRef {
        copy: id(copy),
        piece: id(101),
    };
    let pockets = PocketInventory::new(
        [
            (
                300,
                PocketOpening::Declaration(id(900)),
                vec![component(202)],
            ),
            (
                301,
                PocketOpening::Profile(ProfileParameterRef::new(id(901))),
                vec![component(202), component(203)],
            ),
        ]
        .into_iter()
        .map(|(identity, opening, components)| {
            Pocket::new(
                PocketDefinition {
                    id: id(identity),
                    served: PocketPieceRef {
                        copy: id(200),
                        piece: served.id(),
                    },
                    position: anchor,
                    orientation: direction,
                    opening,
                    components,
                },
                &plan,
                &pieces,
                &ledger,
            )
            .expect("actual Pocket with original source/copy intent")
        })
        .collect(),
    )
    .expect("actual ordered Pockets");
    let notions = NotionPlacementInventory::new(
        (400_u128..413)
            .enumerate()
            .map(|(index, identity)| {
                NotionPlacement::new(
                    NotionPlacementDefinition {
                        id: id(identity),
                        copy: id(if index % 2 == 0 { 200 } else { 201 }),
                        anchor,
                        direction,
                    },
                    &plan,
                    served,
                    &ledger,
                )
                .expect("actual notion placement")
            })
            .collect(),
    )
    .expect("actual notions including unused placement");
    let kinds = [
        ClosureKind::CentredZipper {
            length: ClosureLength::Explicit {
                parameter: id(910),
                value: Length::from_micrometres(180_000).expect("positive zipper length"),
            },
        },
        ClosureKind::CentredZipper {
            length: ClosureLength::Formula(id(911)),
        },
        ClosureKind::CentredZipper {
            length: ClosureLength::Profile(ProfileParameterRef::new(id(912))),
        },
        ClosureKind::HookAndBar {
            hook: NotionSize::Declaration(id(913)),
            bar: NotionSize::Profile(ProfileParameterRef::new(id(914))),
        },
        ClosureKind::ButtonAndButtonhole {
            button: NotionSize::Profile(ProfileParameterRef::new(id(915))),
            hole: ButtonholeDerivation { operation: id(916) },
        },
    ];
    let mut next_placement = 400;
    let mut next_instance = 600;
    let closures = ClosureInventory::new(
        kinds
            .into_iter()
            .zip(500_u128..)
            .map(|(kind, identity)| {
                let instances = (0..if identity == 500 { 2 } else { 1 })
                    .map(|_| {
                        let instance = ClosureInstance {
                            id: id(next_instance),
                            first: id(next_placement),
                            second: id(next_placement + 1),
                        };
                        next_placement += 2;
                        next_instance += 1;
                        instance
                    })
                    .collect();
                Closure::new(
                    ClosureDefinition {
                        id: id(identity),
                        kind,
                        instances,
                    },
                    notions.placements(),
                    &plan,
                    &pieces,
                    &ledger,
                )
                .expect("actual supported Closure with canonical placement ids")
            })
            .collect(),
    )
    .expect("actual Closure/instance inventory");
    Fixture {
        pockets,
        notions,
        closures,
        pieces,
        plan,
        ledger,
        ids,
    }
}
fn charge(f: &Fixture, meter: &mut Meter) -> Result<(), Error> {
    charge_pocket_inventory_resources(&f.pockets, meter)?;
    charge_notion_placement_inventory_resources(&f.notions, meter)?;
    charge_closure_inventory_resources(&f.closures, meter)
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
fn actual_pockets_notions_and_all_supported_closure_intents_count_without_expansion_or_mutation() {
    let f = fixture();
    let original = (f.pockets.clone(), f.notions.clone(), f.closures.clone());
    let mut meter = Meter::new(limits(29, 1, 1));
    charge(&f, &mut meter).expect("2 Pockets,3 components,13 notions,5 Closures,6 instances");
    let demand = meter.finish().expect("exact represented scope");
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (29, 0, 0)
    );
    assert_eq!((f.pockets, f.notions, f.closures), original);
}
#[test]
fn garment_precharges_preserve_original_owner_index_and_cumulative_refusal() {
    let f = fixture();
    for (maximum, consumed, requested, field, owner, index) in [
        (1, 0, 2, "pockets", None, None),
        (4, 3, 2, "pockets.components", Some(301), Some(1)),
        (17, 5, 13, "notions", None, None),
        (22, 18, 5, "closures", None, None),
        (24, 23, 2, "closures.instances", Some(500), Some(0)),
        (25, 25, 1, "closures.instances", Some(501), Some(1)),
    ] {
        let original = (f.pockets.clone(), f.notions.clone(), f.closures.clone());
        let mut meter = Meter::new(limits(maximum, 1, 1));
        let expected = Error::Count {
            unit: Unit::CollectionEntries,
            maximum,
            consumed,
            requested,
            location: location(field, owner, index),
        };
        assert_eq!(charge(&f, &mut meter), Err(expected));
        assert_eq!(meter.finish(), Err(expected));
        assert_eq!(
            (&f.pockets, &f.notions, &f.closures),
            (&original.0, &original.1, &original.2)
        );
    }
}
#[test]
fn actual_deleted_source_does_not_rebirth_resolve_or_repair_held_garment_objects() {
    let mut f = fixture();
    let edge = f
        .pieces
        .first()
        .expect("served Piece")
        .definition()
        .boundary
        .first()
        .expect("actual held edge")
        .edge;
    f.ledger
        .delete(&mut f.ids, edge)
        .expect("real deletion retires held source");
    assert!(f
        .notions
        .validate_current(&f.plan, &f.pieces, &f.ledger)
        .is_err());
    let original = (
        f.pockets.clone(),
        f.notions.clone(),
        f.closures.clone(),
        f.ledger.clone(),
    );
    let mut meter = Meter::new(limits(29, 1, 1));
    charge(&f, &mut meter).expect("held accounting needs no current context validation");
    assert_eq!(
        meter.finish().expect("held demand").collection_entries(),
        29
    );
    assert_eq!((f.pockets, f.notions, f.closures, f.ledger), original);
}
#[test]
fn empty_garment_scopes_count_zero_and_all_producers_preserve_prior_error() {
    let pockets = PocketInventory::new(vec![]).expect("empty draft Pockets");
    let notions = NotionPlacementInventory::new(vec![]).expect("empty draft notions");
    let closures = ClosureInventory::new(vec![]).expect("empty draft Closures");
    let mut meter = Meter::new(limits(1, 1, 1));
    charge_pocket_inventory_resources(&pockets, &mut meter).expect("empty Pockets");
    charge_notion_placement_inventory_resources(&notions, &mut meter).expect("empty notions");
    charge_closure_inventory_resources(&closures, &mut meter).expect("empty Closures");
    assert_eq!(
        meter.finish().expect("empty demand").collection_entries(),
        0
    );
    let first = meter_error();
    let mut refused = Meter::new(limits(1, 1, 1));
    assert_eq!(
        refused.charge_entries(2, location("original", None, None)),
        Err(first)
    );
    let f = fixture();
    assert_eq!(
        charge_pocket_inventory_resources(&f.pockets, &mut refused),
        Err(first)
    );
    assert_eq!(
        charge_notion_placement_inventory_resources(&f.notions, &mut refused),
        Err(first)
    );
    assert_eq!(
        charge_closure_inventory_resources(&f.closures, &mut refused),
        Err(first)
    );
    assert_eq!(
        charge_closure_inventory_resources(&closures, &mut refused),
        Err(first)
    );
    assert_eq!(refused.finish(), Err(first));
}
fn meter_error() -> Error {
    Error::Count {
        unit: Unit::CollectionEntries,
        maximum: 1,
        consumed: 0,
        requested: 2,
        location: location("original", None, None),
    }
}
