//! Actual immutable garment components and complete global ownership, without current-source claims.
#![allow(clippy::expect_used, clippy::panic)]
use super::{fixture, id, with_sewing_graph};
use sc_core::{
    design::{
        DesignGarment, DesignGarmentDefinition, DesignGarmentError as Error,
        DesignGarmentPosition as Position, DesignInputs, DesignInputsPosition, DesignSewing,
        DesignSewingDefinition, DesignSewingPosition, DesignStructure, DesignStructurePosition,
    },
    ontology::*,
};
use sc_measure::DesignSources;
use sc_units::Length;

pub(super) fn definition() -> DesignGarmentDefinition<DesignSources> {
    let (inputs, graph) = with_sewing_graph(fixture(), vec![EaseAmount::Explicit(Length::ZERO)]);
    let structure = inputs.structure();
    let pieces = structure.pieces().pieces();
    let piece = pieces.first().expect("served Piece");
    let edge = piece.edges().next().expect("owned edge").edge;
    let direction = DirectedRange {
        range: EdgeRange::whole(edge),
        direction: Direction::Reversed,
    };
    let placements = pieces
        .iter()
        .zip([200100, 200101])
        .map(|(piece, identity)| {
            let edge = piece.edges().next().expect("notion source").edge;
            let copy = structure
                .cut_plan()
                .copies_for_piece(piece.id())
                .next()
                .expect("explicit copy");
            NotionPlacement::new(
                NotionPlacementDefinition {
                    id: id(identity),
                    copy: copy.id(),
                    anchor: EdgeAnchor {
                        edge,
                        param: Param::START,
                    },
                    direction: DirectedRange {
                        range: EdgeRange::whole(edge),
                        direction: Direction::Original,
                    },
                },
                structure.cut_plan(),
                piece,
                structure.topology(),
            )
            .expect("actual notion")
        })
        .collect::<Vec<_>>();
    let notions = NotionPlacementInventory::new(placements).expect("ordered physical placements");
    let closures = ClosureInventory::new(vec![Closure::new(
        ClosureDefinition {
            id: id(200200),
            kind: ClosureKind::HookAndBar {
                hook: NotionSize::Declaration(id(2000)),
                bar: NotionSize::Profile(ProfileParameterRef::new(id(10))),
            },
            instances: vec![ClosureInstance {
                id: id(200201),
                first: id(200100),
                second: id(200101),
            }],
        },
        notions.placements(),
        structure.cut_plan(),
        pieces,
        structure.topology(),
    )
    .expect("actual Closure/instance")])
    .expect("ordered Closures");
    let served = PocketPieceRef {
        copy: id(96000),
        piece: piece.id(),
    };
    let pockets = PocketInventory::new(vec![Pocket::new(
        PocketDefinition {
            id: id(200000),
            served,
            position: EdgeAnchor {
                edge,
                param: Param::START,
            },
            orientation: direction,
            opening: PocketOpening::Declaration(id(95000)),
            components: vec![
                served,
                PocketPieceRef {
                    copy: id(96001),
                    piece: id(95001),
                },
            ],
        },
        structure.cut_plan(),
        pieces,
        structure.topology(),
    )
    .expect("actual Pocket")])
    .expect("ordered Pockets");
    let layer = |identity| LayerDefinition {
        id: id(identity),
        served_piece: piece.id(),
        offset: LayerOffsetRelationship {
            operation: id(90000),
            sources: vec![direction],
        },
        material: MaterialAssignment::Unresolved("selection pending".into()),
    };
    let layers = LayerInventory::new(
        vec![Facing::new(layer(200300), piece, structure.topology()).expect("Facing")],
        vec![Lining::new(layer(200301), piece, structure.topology()).expect("modelled Lining")],
        vec![Interfacing::new(layer(200302), piece, structure.topology()).expect("Interfacing")],
    )
    .expect("exact distinct layers");
    let hems = HemInventory::new(vec![Hem::new(
        HemDefinition {
            id: id(200400),
            edge,
            depth: HemDepth::Explicit {
                parameter: id(5001),
                value: Length::ZERO,
            },
            fold_type: HemFoldType::Declaration(id(2000)),
            method: HemMethod::Turned,
        },
        piece,
        structure.topology(),
        None,
    )
    .expect("actual Hem")])
    .expect("ordered Hems");
    let intake = IntakeAmount::Explicit {
        parameter: id(5001),
        value: Length::ZERO,
    };
    let fold = |identity| FoldDefinition {
        id: id(identity),
        intake,
        fold_lines: vec![direction],
        direction,
        closing_operation: id(90000),
    };
    let constructions = ConstructionInventory::new(
        vec![Dart::new(
            DartDefinition {
                id: id(200500),
                intake,
                apex: EdgeAnchor {
                    edge,
                    param: Param::END,
                },
                first_leg: direction,
                second_leg: DirectedRange {
                    range: EdgeRange::new(
                        edge,
                        Param::START,
                        Param::new(Rational::new(1, 2).expect("half")).expect("parameter"),
                    )
                    .expect("partial distinct leg"),
                    direction: Direction::Original,
                },
                direction,
                closing_operation: id(90000),
            },
            piece,
            structure.topology(),
        )
        .expect("actual Dart")],
        vec![Tuck::new(fold(200501), piece, structure.topology()).expect("Tuck")],
        vec![Pleat::new(fold(200502), piece, structure.topology()).expect("Pleat")],
        vec![Gather::new(
            GatherDefinition {
                id: id(200503),
                graph: graph.id(),
                span: id(97000),
                side: SeamSideId::A,
                copy: id(96000),
                direction,
                closing_operation: id(90000),
            },
            piece,
            structure.cut_plan(),
            &graph,
            structure.topology(),
        )
        .expect("Gather")],
        vec![Grainline::new(
            GrainlineDefinition {
                id: id(200504),
                arrow: direction,
                alignment: GrainAlignment::ParallelTo(direction),
                stripe: None,
                plaid: None,
            },
            piece,
            structure.topology(),
        )
        .expect("grain")],
        vec![SeamAllowance::new(
            SeamAllowanceDefinition {
                id: id(200505),
                edge,
                width: AllowanceWidth::Explicit {
                    parameter: id(5001),
                    value: Length::ZERO,
                },
                corner: CornerTreatment::Envelope,
                inclusion: ProfileParameterRef::new(id(2000)),
            },
            piece,
            structure.topology(),
        )
        .expect("allowance")],
    )
    .expect("all six original families");
    let sewing = DesignSewing::new(DesignSewingDefinition {
        inputs,
        graph,
        landmarks: vec![],
    })
    .expect("single sewing/input owner");
    DesignGarmentDefinition {
        sewing,
        pockets,
        notions,
        closures,
        layers,
        hems,
        constructions,
    }
}

pub(super) fn pocket_with_id(identity: EntityId) -> PocketInventory {
    let definition = definition();
    let mut pocket = definition
        .pockets
        .pockets()
        .first()
        .expect("Pocket")
        .definition()
        .clone();
    pocket.id = identity;
    let structure = definition.sewing.inputs().structure();
    PocketInventory::new(vec![Pocket::new(
        pocket,
        structure.cut_plan(),
        structure.pieces().pieces(),
        structure.topology(),
    )
    .expect("independent valid Pocket")])
    .expect("single owner")
}
fn design_position() -> Position {
    Position::Sewing(DesignSewingPosition::Inputs(
        DesignInputsPosition::Structure(DesignStructurePosition::Design),
    ))
}
fn change_owned_id(
    definition: &mut DesignGarmentDefinition<DesignSources>,
    position: Position,
    identity: EntityId,
) {
    let structure = definition.sewing.inputs().structure();
    let pieces = structure.pieces().pieces();
    let piece = pieces.first().expect("actual Piece");
    let ledger = structure.topology();
    match position {
        Position::Pocket { .. } => definition.pockets = pocket_with_id(identity),
        Position::NotionPlacement { index } => {
            let mut records = definition.notions.placements().to_vec();
            let original = records.get(index).expect("notion");
            let served = structure
                .pieces()
                .piece(original.piece())
                .expect("actual held Piece");
            let mut input = *original.definition();
            input.id = identity;
            *records.get_mut(index).expect("notion") =
                NotionPlacement::new(input, structure.cut_plan(), served, ledger)
                    .expect("independent notion");
            definition.notions =
                NotionPlacementInventory::new(records).expect("independent inventory");
        }
        Position::Closure(location) => {
            let mut input = definition
                .closures
                .closures()
                .first()
                .expect("Closure")
                .definition()
                .clone();
            match location {
                ClosureInventoryPosition::Closure { .. } => input.id = identity,
                ClosureInventoryPosition::Instance { .. } => {
                    input.instances.first_mut().expect("instance").id = identity
                }
            }
            definition.closures = ClosureInventory::new(vec![Closure::new(
                input,
                definition.notions.placements(),
                structure.cut_plan(),
                pieces,
                ledger,
            )
            .expect("independent Closure")])
            .expect("independent inventory");
        }
        Position::Layer(location) => {
            let mut facings = definition.layers.facings().to_vec();
            let mut linings = definition.layers.linings().to_vec();
            let mut interfacings = definition.layers.interfacings().to_vec();
            match location.kind {
                LayerKind::Facing => {
                    let mut input = facings.first().expect("Facing").definition().clone();
                    input.id = identity;
                    *facings.first_mut().expect("Facing") =
                        Facing::new(input, piece, ledger).expect("independent Facing");
                }
                LayerKind::Lining => {
                    let mut input = linings.first().expect("Lining").definition().clone();
                    input.id = identity;
                    *linings.first_mut().expect("Lining") =
                        Lining::new(input, piece, ledger).expect("independent Lining");
                }
                LayerKind::Interfacing => {
                    let mut input = interfacings
                        .first()
                        .expect("Interfacing")
                        .definition()
                        .clone();
                    input.id = identity;
                    *interfacings.first_mut().expect("Interfacing") =
                        Interfacing::new(input, piece, ledger).expect("independent Interfacing");
                }
            }
            definition.layers =
                LayerInventory::new(facings, linings, interfacings).expect("independent layers");
        }
        Position::Hem { .. } => {
            let mut input = definition
                .hems
                .hems()
                .first()
                .expect("Hem")
                .definition()
                .clone();
            input.id = identity;
            definition.hems = HemInventory::new(vec![
                Hem::new(input, piece, ledger, None).expect("independent Hem")
            ])
            .expect("independent inventory");
        }
        Position::Construction(location) => {
            let old = &definition.constructions;
            let mut darts = old.darts().to_vec();
            let mut tucks = old.tucks().to_vec();
            let mut pleats = old.pleats().to_vec();
            let mut gathers = old.gathers().to_vec();
            let mut grains = old.grainlines().to_vec();
            let mut allowances = old.allowances().to_vec();
            macro_rules! replace {($values:ident,$ty:ident,$($context:expr),+)=>{{
                let mut input=$values.first().expect("construction").definition().clone();input.id=identity;
                *$values.first_mut().expect("construction")=$ty::new(input,$($context),+).expect("independently valid construction");
            }};}
            match location.kind {
                ConstructionKind::Dart => replace!(darts, Dart, piece, ledger),
                ConstructionKind::Tuck => replace!(tucks, Tuck, piece, ledger),
                ConstructionKind::Pleat => replace!(pleats, Pleat, piece, ledger),
                ConstructionKind::Gather => replace!(
                    gathers,
                    Gather,
                    piece,
                    structure.cut_plan(),
                    definition.sewing.graph(),
                    ledger
                ),
                ConstructionKind::Grainline => replace!(grains, Grainline, piece, ledger),
                ConstructionKind::SeamAllowance => {
                    replace!(allowances, SeamAllowance, piece, ledger)
                }
            }
            definition.constructions =
                ConstructionInventory::new(darts, tucks, pleats, gathers, grains, allowances)
                    .expect("independent constructions");
        }
        Position::Sewing(_) => panic!("fixture mutation only changes garment ownership"),
    }
}

#[test]
fn ownership_keeps_actual_components_complete_nested_ids_and_references_as_uses() {
    let definition = definition();
    let original = definition.clone();
    let garment = DesignGarment::new(definition).expect("actual complete immutable ownership");
    assert_eq!(garment.definition(), &original);
    assert!(core::ptr::eq(
        garment.sewing(),
        &garment.definition().sewing
    ));
    assert!(core::ptr::eq(garment.inputs(), garment.sewing().inputs()));
    assert!(core::ptr::eq(
        garment.inputs().inputs(),
        garment.inputs().structure().inputs()
    ));
    assert!(core::ptr::eq(
        garment.pockets(),
        &garment.definition().pockets
    ));
    assert!(core::ptr::eq(
        garment.notions(),
        &garment.definition().notions
    ));
    assert!(core::ptr::eq(
        garment.closures(),
        &garment.definition().closures
    ));
    assert!(core::ptr::eq(
        garment.layers(),
        &garment.definition().layers
    ));
    assert!(core::ptr::eq(garment.hems(), &garment.definition().hems));
    assert!(core::ptr::eq(
        garment.constructions(),
        &garment.definition().constructions
    ));
    let sewing = garment
        .sewing()
        .identities()
        .into_iter()
        .map(|(identity, position)| (identity, Position::Sewing(position)))
        .collect::<Vec<_>>();
    let all = garment.identities();
    assert_eq!(all.get(..sewing.len()).expect("whole prefix"), sewing);
    assert_eq!(
        all.get(sewing.len()..).expect("actual garment suffix"),
        [
            (id(200000), Position::Pocket { index: 0 }),
            (id(200100), Position::NotionPlacement { index: 0 }),
            (id(200101), Position::NotionPlacement { index: 1 }),
            (
                id(200200),
                Position::Closure(ClosureInventoryPosition::Closure { index: 0 })
            ),
            (
                id(200201),
                Position::Closure(ClosureInventoryPosition::Instance {
                    closure: 0,
                    index: 0
                })
            ),
            (
                id(200300),
                Position::Layer(LayerInventoryPosition {
                    kind: LayerKind::Facing,
                    index: 0
                })
            ),
            (
                id(200301),
                Position::Layer(LayerInventoryPosition {
                    kind: LayerKind::Lining,
                    index: 0
                })
            ),
            (
                id(200302),
                Position::Layer(LayerInventoryPosition {
                    kind: LayerKind::Interfacing,
                    index: 0
                })
            ),
            (id(200400), Position::Hem { index: 0 }),
            (
                id(200500),
                Position::Construction(ConstructionInventoryPosition {
                    kind: ConstructionKind::Dart,
                    index: 0
                })
            ),
            (
                id(200501),
                Position::Construction(ConstructionInventoryPosition {
                    kind: ConstructionKind::Tuck,
                    index: 0
                })
            ),
            (
                id(200502),
                Position::Construction(ConstructionInventoryPosition {
                    kind: ConstructionKind::Pleat,
                    index: 0
                })
            ),
            (
                id(200503),
                Position::Construction(ConstructionInventoryPosition {
                    kind: ConstructionKind::Gather,
                    index: 0
                })
            ),
            (
                id(200504),
                Position::Construction(ConstructionInventoryPosition {
                    kind: ConstructionKind::Grainline,
                    index: 0
                })
            ),
            (
                id(200505),
                Position::Construction(ConstructionInventoryPosition {
                    kind: ConstructionKind::SeamAllowance,
                    index: 0
                })
            ),
        ]
    );
    for identity in [5001, 2000, 90000, 95000] {
        assert_eq!(
            all.iter()
                .filter(|(owned, _)| *owned == id(identity))
                .count(),
            1,
            "uses add no owner"
        );
    }
    assert_eq!(garment.layers().linings().len(), 1);
    assert!(garment.layers().require_in_scope().is_err()); // Modelled scope survives admission.
    let debug = format!("{garment:?}");
    assert!(
        debug.contains("construction_count: 6")
            && !debug.contains("EntityId")
            && !debug.contains("selection pending")
    );
}

#[test]
fn every_owned_garment_family_and_nested_instance_refuses_the_actual_root_design_id() {
    let original = definition();
    let owner = DesignGarment::new(original.clone()).expect("valid baseline");
    let root_count = owner.sewing().identities().len();
    let positions = owner
        .identities()
        .get(root_count..)
        .expect("all garment owners")
        .iter()
        .map(|(_, position)| *position)
        .collect::<Vec<_>>();
    for duplicate in positions {
        let mut definition = original.clone();
        change_owned_id(&mut definition, duplicate, id(5000));
        assert_eq!(
            DesignGarment::new(definition).expect_err("global owner collision"),
            Error::DuplicateIdentity {
                identity: id(5000),
                first: design_position(),
                duplicate
            }
        );
    }
}

#[test]
fn cross_family_collisions_keep_first_semantic_positions_and_instances_are_first_class_owners() {
    for (identity, first, duplicate) in [
        (
            200000,
            Position::Pocket { index: 0 },
            Position::Closure(ClosureInventoryPosition::Instance {
                closure: 0,
                index: 0,
            }),
        ),
        (
            200100,
            Position::NotionPlacement { index: 0 },
            Position::Construction(ConstructionInventoryPosition {
                kind: ConstructionKind::SeamAllowance,
                index: 0,
            }),
        ),
        (
            200300,
            Position::Layer(LayerInventoryPosition {
                kind: LayerKind::Facing,
                index: 0,
            }),
            Position::Hem { index: 0 },
        ),
    ] {
        let mut definition = definition();
        change_owned_id(&mut definition, duplicate, id(identity));
        assert_eq!(
            DesignGarment::new(definition).expect_err("actual independent components alias"),
            Error::DuplicateIdentity {
                identity: id(identity),
                first,
                duplicate
            }
        );
    }
}

#[test]
fn retired_creator_ownership_and_held_damage_survive_without_source_reconstruction() {
    let mut definition = definition();
    let mut sewing = definition.sewing.definition().clone();
    let mut inputs = sewing.inputs.definition().clone();
    let mut structure = inputs.structure.definition().clone();
    let mut ids = DeterministicIdGenerator::from_start(300000);
    for held in structure
        .pieces
        .pieces()
        .iter()
        .flat_map(|piece| piece.edges())
    {
        structure
            .topology
            .split(
                &mut ids,
                held.edge,
                Param::new(Rational::new(1, 3).expect("third")).expect("inside"),
            )
            .expect("retire original creator outputs");
    }
    assert!(structure
        .topology
        .live_edges()
        .all(|edge| edge.creator() != id(90000)));
    inputs.structure = DesignStructure::new(structure).expect("historical source frame retained");
    sewing.inputs = DesignInputs::new(inputs).expect("same canonical/input state");
    definition.sewing =
        DesignSewing::new(sewing).expect("held semantic objects remain inspectable");
    let owner = DesignGarment::new(definition.clone()).expect("historical ownership");
    let first = owner
        .identities()
        .into_iter()
        .find(|(identity, _)| *identity == id(90000))
        .expect("retired creator remains owned")
        .1;
    let duplicate = Position::Hem { index: 0 };
    let mut collision = definition.clone();
    change_owned_id(&mut collision, duplicate, id(90000));
    assert_eq!(
        DesignGarment::new(collision).expect_err("retirement does not free identity"),
        Error::DuplicateIdentity {
            identity: id(90000),
            first,
            duplicate
        }
    );
    let mut sewing = definition.sewing.definition().clone();
    let mut inputs = sewing.inputs.definition().clone();
    let mut structure = inputs.structure.definition().clone();
    let live = structure
        .topology
        .live_edges()
        .next()
        .expect("current fragment");
    structure
        .topology
        .delete(&mut ids, live)
        .expect("later actual damage");
    inputs.structure = DesignStructure::new(structure).expect("held repair retained");
    sewing.inputs = DesignInputs::new(inputs).expect("current input scope");
    definition.sewing = DesignSewing::new(sewing).expect("held graph preserved");
    let original = definition.clone();
    let damaged =
        DesignGarment::new(definition).expect("ownership does not certify current sources");
    assert_eq!(damaged.definition(), &original);
    assert!(damaged.sewing().validate_current().is_err());
}
