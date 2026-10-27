//! Complete actual garment source/assignment checks, without execution or resolved physical proof.
#![allow(clippy::expect_used, clippy::panic)]
use super::{garment_ownership, id};
use sc_core::{
    design::{
        DesignGarment, DesignGarmentDefinition, DesignGarmentError as Error, DesignInputs,
        DesignSewing, DesignStructure,
    },
    ontology::*,
};
use sc_measure::DesignSources;

fn clear_garments(definition: &mut DesignGarmentDefinition<DesignSources>) {
    definition.pockets = PocketInventory::new(vec![]).expect("empty Pockets");
    definition.notions = NotionPlacementInventory::new(vec![]).expect("empty notions");
    definition.closures = ClosureInventory::new(vec![]).expect("empty Closures");
    definition.layers = LayerInventory::new(vec![], vec![], vec![]).expect("empty layers");
    definition.hems = HemInventory::new(vec![]).expect("empty Hems");
    definition.constructions =
        ConstructionInventory::new(vec![], vec![], vec![], vec![], vec![], vec![])
            .expect("empty constructions");
}
fn empty_graph(definition: &mut DesignGarmentDefinition<DesignSources>) {
    let mut sewing = definition.sewing.definition().clone();
    let structure = sewing.inputs.structure();
    sewing.graph = SewingGraph::new(
        SewingGraphDefinition {
            id: sewing.graph.id(),
            spans: vec![],
        },
        structure.cut_plan(),
        structure.pieces().pieces(),
        &sewing.landmarks,
        structure.topology(),
    )
    .expect("actual empty graph");
    definition.sewing = DesignSewing::new(sewing).expect("exact current owner");
}
fn lose_edge(definition: &mut DesignGarmentDefinition<DesignSources>, piece_index: usize) {
    let mut sewing = definition.sewing.definition().clone();
    let mut inputs = sewing.inputs.definition().clone();
    let mut structure = inputs.structure.definition().clone();
    let edge = structure
        .pieces
        .pieces()
        .get(piece_index)
        .expect("actual Piece")
        .edges()
        .next()
        .expect("held edge")
        .edge;
    structure
        .topology
        .delete(&mut DeterministicIdGenerator::from_start(400000), edge)
        .expect("later source loss");
    inputs.structure = DesignStructure::new(structure).expect("held repair state");
    sewing.inputs = DesignInputs::new(inputs).expect("current actual root inputs");
    definition.sewing = DesignSewing::new(sewing).expect("held semantics, no reconstructed birth");
}

#[test]
fn whole_current_source_success_preserves_actual_content_and_modelled_execution_deferrals() {
    let definition = garment_ownership::definition();
    let original = definition.clone();
    let garment = DesignGarment::new(definition).expect("actual complete components");
    assert_eq!(garment.validate_current(), Ok(()));
    assert_eq!(garment.definition(), &original);
    assert!(garment.layers().require_in_scope().is_err());
    assert!(garment.layers().entries().all(|(_, layer)| matches!(
        layer.definition().material,
        MaterialAssignment::Unresolved(_)
    )));
    // Existing explicit/logical sources remain intent, not evaluated values or supported execution.
    assert_eq!(
        garment
            .constructions()
            .darts()
            .first()
            .expect("Dart")
            .definition()
            .closing_operation,
        id(90000)
    );
}

#[test]
fn each_unused_family_source_keeps_the_exact_original_context_and_standard_cause() {
    for family in [
        "sewing",
        "pockets",
        "notions",
        "closures",
        "layers",
        "hems",
        "constructions",
    ] {
        let original = garment_ownership::definition();
        let mut definition = original.clone();
        clear_garments(&mut definition);
        if family != "sewing" {
            empty_graph(&mut definition);
        }
        match family {
            "sewing" => lose_edge(&mut definition, 1),
            "pockets" => {
                definition.pockets = original.pockets;
                lose_edge(&mut definition, 1);
            }
            "notions" => {
                definition.notions = original.notions;
                lose_edge(&mut definition, 1);
            }
            "closures" => {
                definition.notions = original.notions;
                definition.closures = original.closures;
                let root = definition.sewing.inputs().structure();
                let mut placements = definition.notions.placements().to_vec();
                let held = placements.first().expect("held notion");
                let mut input = *held.definition();
                input.id = id(200110);
                *placements.first_mut().expect("first notion") = NotionPlacement::new(
                    input,
                    root.cut_plan(),
                    root.pieces().piece(held.piece()).expect("Piece"),
                    root.topology(),
                )
                .expect("valid renamed notion");
                definition.notions =
                    NotionPlacementInventory::new(placements).expect("actual current notions");
            }
            "layers" => {
                definition.layers =
                    LayerInventory::new(vec![], vec![], original.layers.interfacings().to_vec())
                        .expect("unused original Interfacing");
                lose_edge(&mut definition, 0);
            }
            "hems" => {
                definition.hems = original.hems;
                lose_edge(&mut definition, 0);
            }
            "constructions" => {
                definition.constructions = original.constructions;
                lose_edge(&mut definition, 0);
            }
            _ => panic!("declared finite families"),
        }
        let saved = definition.clone();
        let garment = DesignGarment::new(definition).expect("held independently valid ownership");
        let error = garment
            .validate_current()
            .expect_err("unused source is still an obligation");
        let root = garment.inputs().structure();
        let plan = root.cut_plan();
        let pieces = root.pieces().pieces();
        let ledger = root.topology();
        macro_rules! exact_cause {
            ($variant:ident,$ty:ty,$query:expr) => {{
                let original = $query.expect_err("original complete family refusal");
                assert_eq!(
                    error,
                    Error::$variant(Box::new(original.clone())),
                    "{family}"
                );
                assert_eq!(
                    std::error::Error::source(&error)
                        .expect("original concrete family")
                        .downcast_ref::<$ty>(),
                    Some(&original)
                );
            }};
        }
        match family {
            "sewing" => exact_cause!(
                Sewing,
                sc_core::design::DesignSewingError<sc_measure::DesignSourcesError>,
                garment.sewing().validate_current()
            ),
            "pockets" => {
                let original = garment
                    .pockets()
                    .validate_current(plan, pieces, ledger)
                    .expect_err("full component interval");
                assert!(matches!(
                    original,
                    PocketInventoryError::ComponentRange { .. }
                ));
                exact_cause!(
                    Pockets,
                    PocketInventoryError,
                    garment.pockets().validate_current(plan, pieces, ledger)
                );
            }
            "notions" => {
                exact_cause!(
                    Notions,
                    NotionPlacementInventoryError,
                    garment.notions().validate_current(plan, pieces, ledger)
                );
                assert!(std::error::Error::source(&error)
                    .expect("inventory")
                    .source()
                    .expect("placement")
                    .source()
                    .expect("anchor")
                    .is::<AnchorError>());
            }
            "closures" => exact_cause!(
                Closures,
                ClosureInventoryError,
                garment
                    .closures()
                    .validate_current(garment.notions(), plan, pieces, ledger)
            ),
            "layers" => exact_cause!(
                Layers,
                LayerInventoryError,
                garment.layers().validate_current(pieces, ledger)
            ),
            "hems" => exact_cause!(
                Hems,
                HemInventoryError,
                garment
                    .hems()
                    .validate_current(garment.layers(), pieces, ledger)
            ),
            "constructions" => exact_cause!(
                Constructions,
                ConstructionInventoryError,
                garment.constructions().validate_current(
                    pieces,
                    plan,
                    garment.sewing().graph(),
                    garment.sewing().landmarks(),
                    ledger
                )
            ),
            _ => panic!("declared finite families"),
        }
        assert_eq!(garment.definition(), &saved);
    }
}

#[test]
fn layer_assignments_require_actual_material_targets_and_source_damage_keeps_original_priority() {
    let original = garment_ownership::definition();
    let mut definition = original.clone();
    clear_garments(&mut definition);
    empty_graph(&mut definition);
    let mut layer = original
        .layers
        .interfacings()
        .first()
        .expect("actual Interfacing")
        .definition()
        .clone();
    layer.material = MaterialAssignment::Assigned(id(5001)); // Actual parameter id is no Material.
    let root = definition.sewing.inputs().structure();
    let interfacing = Interfacing::new(
        layer,
        root.pieces().pieces().first().expect("Piece"),
        root.topology(),
    )
    .expect("born source, assignment registry separate");
    definition.layers =
        LayerInventory::new(vec![], vec![], vec![interfacing]).expect("actual unused layer");
    let garment =
        DesignGarment::new(definition.clone()).expect("global ownership, target remains a use");
    let expected = LayerInventoryError::MissingMaterial {
        layer: id(200302),
        position: LayerInventoryPosition {
            kind: LayerKind::Interfacing,
            index: 0,
        },
        material: id(5001),
    };
    assert_eq!(
        garment.validate_current(),
        Err(Error::Layers(Box::new(expected)))
    );
    lose_edge(&mut definition, 0);
    let damaged = DesignGarment::new(definition).expect("both held problems inspectable");
    let root = damaged.inputs().structure();
    let source = damaged
        .layers()
        .validate_current(root.pieces().pieces(), root.topology())
        .expect_err("source precedes assignment");
    assert!(matches!(source, LayerInventoryError::Layer { .. }));
    assert_eq!(
        damaged.validate_current(),
        Err(Error::Layers(Box::new(source)))
    );
}

#[test]
fn a_later_hem_failure_returns_its_authored_index_after_valid_earlier_sources_without_cached_success(
) {
    let mut definition = garment_ownership::definition();
    clear_garments(&mut definition);
    empty_graph(&mut definition);
    let root = definition.sewing.inputs().structure();
    let pieces = root.pieces().pieces();
    let make = |index: usize, identity: u128| {
        let piece = pieces.get(index).expect("actual held Piece");
        let edge = piece.edges().next().expect("finish edge").edge;
        Hem::new(
            HemDefinition {
                id: id(identity),
                edge,
                depth: HemDepth::Explicit {
                    parameter: id(5001),
                    value: sc_units::Length::ZERO,
                },
                fold_type: HemFoldType::Declaration(id(2000)),
                method: HemMethod::Turned,
            },
            piece,
            root.topology(),
            None,
        )
        .expect("born Hem")
    };
    definition.hems =
        HemInventory::new(vec![make(0, 201001), make(1, 201000)]).expect("authored descending ids");
    assert!(DesignGarment::new(definition.clone())
        .expect("valid current owner")
        .validate_current()
        .is_ok());
    lose_edge(&mut definition, 1);
    let garment = DesignGarment::new(definition).expect("held damage");
    assert!(
        matches!(garment.validate_current(),Err(Error::Hems(issue)) if matches!(issue.as_ref(),HemInventoryError::Hem {index:1,hem,..} if *hem==id(201000)))
    );
}
