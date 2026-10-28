//! Actual complete core owner over real external sources, without a counting-host replacement.
use super::{id, Error, Limits, Location, Meter, Scope, Unit};
use core::num::{NonZeroU64, NonZeroUsize};
use sc_core::{
    command::DesignRevision,
    design::{
        charge_design_garment_resources, charge_design_input_resources,
        charge_design_sewing_resources, DesignGarment, DesignGarmentDefinition, DesignInputs,
        DesignInputsDefinition, DesignMetadata, DesignParameterDefinition, DesignParameters,
        DesignResourceSink, DesignSewing, DesignSewingDefinition, DesignSourceReferences,
        DesignStructure, DesignStructureDefinition,
    },
    name::MachineToken,
    ontology::*,
    recipe::FormulaRecipeDocument,
    value::CanonicalInputTarget,
};
use sc_measure::DesignSources;
fn limits(entries: usize, text: usize, bits: u64) -> Limits {
    Limits::new(
        NonZeroUsize::new(entries).expect("explicit positive entry maximum"),
        NonZeroUsize::new(text).expect("explicit positive text maximum"),
        NonZeroU64::new(bits).expect("explicit positive bit maximum"),
    )
}
fn fixture() -> DesignGarment<DesignSources> {
    let (host, canonical) = super::source_host::fixture();
    let mut ids = DeterministicIdGenerator::from_start(60000);
    let mut topology = IdentityLedger::new();
    let edge = *topology
        .declare_edges(&mut ids, 1)
        .expect("actual edge birth")
        .first()
        .expect("one actual edge");
    let piece = Piece::new(
        PieceDefinition {
            id: id(50010),
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
            material: MaterialAssignment::Unresolved(" é ".into()),
            layer_index: 0,
            label: LabelText {
                name: "P".into(),
                size: "M".into(),
                fabric: "猫".into(),
                colorway: "é".into(),
            },
        },
        &topology,
    )
    .expect("actual original Piece");
    let pieces = PieceInventory::new(vec![piece]).expect("one authored Piece");
    let cut_plan = CutPlan::new(
        vec![CutCopyDefinition {
            id: id(50011),
            piece: id(50010),
            orientation: CopyOrientation::Authored,
        }],
        pieces.pieces(),
    )
    .expect("one actual physical copy");
    let piece = pieces.pieces().first().expect("actual held Piece");
    let landmarks = vec![SewingLandmark::Notch(
        Notch::new(
            NotchDefinition {
                id: id(50030),
                anchor: EdgeAnchor {
                    edge,
                    param: Param::START,
                },
                representation: NotchProfileBindings {
                    style: ProfileParameterRef::new(id(50040)),
                    sample: NotchDimensionBindings {
                        depth: ProfileParameterRef::new(id(50041)),
                        width: ProfileParameterRef::new(id(50042)),
                    },
                    production: NotchDimensionBindings {
                        depth: ProfileParameterRef::new(id(50043)),
                        width: ProfileParameterRef::new(id(50044)),
                    },
                    encoding: ProfileParameterRef::new(id(50045)),
                },
            },
            piece,
            &topology,
        )
        .expect("actual semantic landmark"),
    )];
    let notion = NotionPlacement::new(
        NotionPlacementDefinition {
            id: id(50031),
            copy: id(50011),
            anchor: EdgeAnchor {
                edge,
                param: Param::START,
            },
            direction: DirectedRange {
                range: EdgeRange::whole(edge),
                direction: Direction::Original,
            },
        },
        &cut_plan,
        piece,
        &topology,
    )
    .expect("actual unused notion");
    let graph = SewingGraph::new(
        SewingGraphDefinition {
            id: id(50001),
            spans: vec![],
        },
        &cut_plan,
        pieces.pieces(),
        &landmarks,
        &topology,
    )
    .expect("actual graph");
    let parameters = DesignParameters::new(
        vec![DesignParameterDefinition {
            id: id(50020),
            token: MachineToken::new("p").expect("explicit token"),
            target: CanonicalInputTarget::Length(
                canonical
                    .lengths()
                    .first()
                    .expect("actual source length")
                    .id(),
            ),
        }],
        &canonical,
    )
    .expect("actual parameter over sole canonical owner");
    let materials = MaterialInventory::new(vec![], &canonical).expect("explicit empty Materials");
    let sources = DesignMetadata::new(host, &canonical).expect("actual host and retained census");
    assert_eq!(
        sources.entities().len(),
        26,
        "actual retained identity records"
    );
    let inputs = DesignInputs::new(DesignInputsDefinition {
        structure: DesignStructure::new(DesignStructureDefinition {
            id: id(50000),
            revision: DesignRevision::from_counter(u128::MAX).successor(),
            inputs: canonical,
            pieces,
            cut_plan,
            topology,
        })
        .expect("actual structural root"),
        parameters,
        recipe: FormulaRecipeDocument::new(" \n".into())
            .expect("exact source and empty canonical recipe"),
        materials,
        sources,
        references: DesignSourceReferences {
            measurement_table: None,
            ease_set: None,
            size_set: None,
        },
    })
    .expect("actual complete input owner");
    let sewing = DesignSewing::new(DesignSewingDefinition {
        inputs,
        graph,
        landmarks,
    })
    .expect("actual complete sewing owner");
    DesignGarment::new(DesignGarmentDefinition {
        sewing,
        pockets: PocketInventory::new(vec![]).expect("explicit empty Pockets"),
        notions: NotionPlacementInventory::new(vec![notion])
            .expect("actual retained unused notion"),
        closures: ClosureInventory::new(vec![]).expect("explicit empty Closures"),
        layers: LayerInventory::new(vec![], vec![], vec![]).expect("explicit empty layers"),
        hems: HemInventory::new(vec![]).expect("explicit empty Hems"),
        constructions: ConstructionInventory::new(vec![], vec![], vec![], vec![], vec![], vec![])
            .expect("explicit empty constructions"),
    })
    .expect("actual complete garment owner")
}
fn location(scope: Scope, field: &'static str, owner: Option<u128>) -> Location {
    Location {
        scope,
        owner: owner.map(id),
        field,
        index: None,
    }
}
#[test]
fn complete_actual_roots_compose_the_retained_census_host_and_each_later_owner() {
    let garment = fixture();
    let original = garment.clone();
    let mut input_meter = Meter::new(limits(78, 650, 129));
    charge_design_input_resources(garment.inputs(), &mut input_meter)
        .expect("complete actual inputs");
    assert_eq!(
        input_meter
            .finish()
            .expect("complete input demand")
            .collection_entries(),
        78
    );
    let mut sewing_meter = Meter::new(limits(79, 650, 129));
    charge_design_sewing_resources(garment.sewing(), &mut sewing_meter)
        .expect("complete actual sewing");
    assert_eq!(
        sewing_meter
            .finish()
            .expect("complete sewing demand")
            .collection_entries(),
        79
    );
    let mut meter = Meter::new(limits(80, 650, 129));
    charge_design_garment_resources(&garment, &mut meter).expect("every actual owned root");
    let demand = meter.finish().expect("complete logical model content");
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (80, 650, 129)
    );
    assert_eq!(garment, original);
}
#[test]
fn whole_model_refusals_preserve_cross_root_original_locations_and_exact_first_error() {
    let garment = fixture();
    for (policy, expected) in [
        (
            limits(80, 650, 128),
            Error::CounterBits {
                maximum: 128,
                consumed: 0,
                requested: 129,
                location: location(Scope::Structure, "design.revision", Some(50000)),
            },
        ),
        (
            limits(77, 650, 129),
            Error::Count {
                unit: Unit::CollectionEntries,
                maximum: 77,
                consumed: 52,
                requested: 26,
                location: location(Scope::Inputs, "sources.entities", Some(50000)),
            },
        ),
        (
            limits(78, 650, 129),
            Error::Count {
                unit: Unit::CollectionEntries,
                maximum: 78,
                consumed: 78,
                requested: 1,
                location: location(Scope::Sewing, "sewing.landmarks", None),
            },
        ),
        (
            limits(79, 650, 129),
            Error::Count {
                unit: Unit::CollectionEntries,
                maximum: 79,
                consumed: 79,
                requested: 1,
                location: location(Scope::Garment, "notions", None),
            },
        ),
        (
            limits(80, 649, 129),
            Error::Count {
                unit: Unit::TextBytes,
                maximum: 649,
                consumed: 641,
                requested: 9,
                location: Location {
                    index: Some(1),
                    ..location(
                        Scope::SourceHost,
                        "garment_sets.chart.observations.measurement.token",
                        Some(1006),
                    )
                },
            },
        ),
    ] {
        let original = garment.clone();
        let mut meter = Meter::new(policy);
        assert_eq!(
            charge_design_garment_resources(&garment, &mut meter),
            Err(expected)
        );
        assert_eq!(meter.finish(), Err(expected));
        assert_eq!(garment, original);
    }
}
#[test]
fn complete_model_keeps_prior_units_and_cannot_clear_a_prior_refusal() {
    let garment = fixture();
    let original = garment.clone();
    let prior = location(Scope::Inputs, "earlier", None);
    let mut meter = Meter::new(limits(82, 652, 136));
    meter.charge_entries(2, prior).expect("prior entries");
    meter.charge_text("é", prior).expect("prior text");
    meter.charge_counter_bits(7, prior).expect("prior bits");
    charge_design_garment_resources(&garment, &mut meter).expect("one cumulative meter");
    let demand = meter.finish().expect("all actual content plus prior units");
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (82, 652, 136)
    );
    let mut refused = Meter::new(limits(1, 1, 1));
    let first = refused
        .charge_entries(2, prior)
        .expect_err("original prior refusal");
    assert_eq!(
        charge_design_garment_resources(&garment, &mut refused),
        Err(first)
    );
    assert_eq!(refused.finish(), Err(first));
    assert_eq!(garment, original);
}
#[test]
fn complete_held_model_accounting_does_not_run_semantic_repair_or_current_validation() {
    let garment = fixture();
    let mut structure = garment.inputs().structure().definition().clone();
    let edge = structure
        .pieces
        .pieces()
        .first()
        .expect("actual Piece")
        .edges()
        .next()
        .expect("actual held edge")
        .edge;
    structure
        .topology
        .delete(&mut DeterministicIdGenerator::from_start(60002), edge)
        .expect("real later deletion");
    let mut inputs = garment.inputs().definition().clone();
    inputs.structure = DesignStructure::new(structure).expect("inspectable held structural root");
    let mut sewing = garment.sewing().definition().clone();
    sewing.inputs = DesignInputs::new(inputs).expect("unchanged current canonical/source metadata");
    let mut definition = garment.definition().clone();
    definition.sewing = DesignSewing::new(sewing).expect("held sewing metadata");
    let held = DesignGarment::new(definition).expect("held garment owner");
    held.validate_current()
        .expect_err("deleted actual landmark/notion source needs repair");
    let original = held.clone();
    let mut meter = Meter::new(limits(80, 650, 129));
    charge_design_garment_resources(&held, &mut meter)
        .expect("held data without repair/current lookup");
    let demand = meter
        .finish()
        .expect("journal +1/live index -1 preserves total");
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (80, 650, 129)
    );
    assert_eq!(held, original);
}
#[test]
fn actual_empty_draft_keeps_its_retained_canonical_recipe_and_zero_other_dynamic_content() {
    use sc_core::value::CanonicalInputs;
    use sc_measure::{
        DesignSourcesDefinition, MeasurementInputs, MeasurementInputsDefinition,
        MeasurementInventory, MeasurementInventoryDefinition, SizeSetInventory,
    };
    let canonical = CanonicalInputs::new(vec![], vec![]).expect("actual empty canonical draft");
    let measurements = MeasurementInventory::new(
        MeasurementInventoryDefinition {
            landmarks: vec![],
            procedures: vec![],
            measurements: vec![],
            tables: vec![],
        },
        &canonical,
    )
    .expect("actual empty measurement metadata");
    let measurements = MeasurementInputs::new(
        MeasurementInputsDefinition {
            measurements,
            eases: vec![],
            ease_sets: vec![],
        },
        &canonical,
    )
    .expect("actual empty source metadata");
    let size_sets =
        SizeSetInventory::new(vec![], &canonical, &measurements).expect("actual empty sets");
    let host = DesignSources::new(
        DesignSourcesDefinition {
            measurements,
            size_sets,
        },
        &canonical,
    )
    .expect("actual explicit empty host");
    let sources = DesignMetadata::new(host, &canonical).expect("actual empty captured census");
    let parameters = DesignParameters::new(vec![], &canonical).expect("empty parameter registry");
    let materials = MaterialInventory::new(vec![], &canonical).expect("empty Materials");
    let topology = IdentityLedger::new();
    let pieces = PieceInventory::new(vec![]).expect("empty Pieces");
    let cut_plan = CutPlan::new(vec![], pieces.pieces()).expect("empty copies");
    let graph = SewingGraph::new(
        SewingGraphDefinition {
            id: id(50001),
            spans: vec![],
        },
        &cut_plan,
        pieces.pieces(),
        &[],
        &topology,
    )
    .expect("actual empty sewing graph");
    let inputs = DesignInputs::new(DesignInputsDefinition {
        structure: DesignStructure::new(DesignStructureDefinition {
            id: id(50000),
            revision: DesignRevision::initial(),
            inputs: canonical,
            pieces,
            cut_plan,
            topology,
        })
        .expect("actual empty structure"),
        parameters,
        recipe: FormulaRecipeDocument::new(String::new()).expect("empty authored recipe"),
        materials,
        sources,
        references: DesignSourceReferences {
            measurement_table: None,
            ease_set: None,
            size_set: None,
        },
    })
    .expect("actual empty input draft");
    let garment = DesignGarment::new(DesignGarmentDefinition {
        sewing: DesignSewing::new(DesignSewingDefinition {
            inputs,
            graph,
            landmarks: vec![],
        })
        .expect("actual empty sewing draft"),
        pockets: PocketInventory::new(vec![]).expect("empty Pockets"),
        notions: NotionPlacementInventory::new(vec![]).expect("empty notions"),
        closures: ClosureInventory::new(vec![]).expect("empty Closures"),
        layers: LayerInventory::new(vec![], vec![], vec![]).expect("empty layers"),
        hems: HemInventory::new(vec![]).expect("empty Hems"),
        constructions: ConstructionInventory::new(vec![], vec![], vec![], vec![], vec![], vec![])
            .expect("empty constructions"),
    })
    .expect("actual complete empty draft");
    let original = garment.clone();
    let mut meter = Meter::new(limits(1, 8, 1));
    charge_design_garment_resources(&garment, &mut meter).expect("retained canonical recipe only");
    let demand = meter.finish().expect("actual empty draft demand");
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (0, 8, 0)
    );
    let mut short = Meter::new(limits(1, 7, 1));
    let expected = Error::Count {
        unit: Unit::TextBytes,
        maximum: 7,
        consumed: 0,
        requested: 8,
        location: location(Scope::Inputs, "recipe.canonical", None),
    };
    assert_eq!(
        charge_design_garment_resources(&garment, &mut short),
        Err(expected)
    );
    assert_eq!(short.finish(), Err(expected));
    assert_eq!(garment, original);
}
