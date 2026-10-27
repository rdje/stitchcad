//! Unified adapters retain original objects/roles and one canonical owner after full current checks.
#![allow(clippy::expect_used, clippy::panic)]
use super::{garment_current, garment_ownership, id};
use sc_core::{
    design::{
        DesignClosureDeclarationRole, DesignConstructionDeclarationError,
        DesignConstructionDeclarationRole, DesignGarment, DesignGarmentDeclarationError as Error,
        DesignGarmentDeclarationSource as Source, DesignGarmentDefinition,
        DesignHemDeclarationError, DesignHemDeclarationRole, DesignInputs, DesignStructure,
    },
    ontology::*,
    recipe::FormulaKind,
    value::{
        CanonicalInputTarget, CanonicalInputs, ScalarInputDeclaration,
        ScalarInputDeclarationDefinition, ScalarInputKind, ScalarInputState,
    },
};
use sc_measure::DesignSources;

fn assigned_definition() -> DesignGarmentDefinition<DesignSources> {
    let mut definition = garment_ownership::definition();
    let mut sewing = definition.sewing.definition().clone();
    let mut inputs = sewing.inputs.definition().clone();
    let mut structure = inputs.structure.definition().clone();
    let mut scalars = structure.inputs.scalars().to_vec();
    scalars.push(
        ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
            id: id(220001),
            source: id(220002),
            kind: ScalarInputKind::Boolean,
            state: ScalarInputState::Unknown {
                observation: id(220003),
            },
        })
        .expect("actual unknown Boolean"),
    );
    structure.inputs = CanonicalInputs::new(structure.inputs.lengths().to_vec(), scalars)
        .expect("same owner plus explicit flag source");
    inputs.structure = DesignStructure::new(structure).expect("actual canonical replacement");
    let flag = CanonicalInputTarget::Scalar {
        declaration: id(220001),
        kind: ScalarInputKind::Boolean,
    };
    let material = Material::new(
        MaterialDefinition {
            id: id(220000),
            source: id(220004),
            name: "private assigned material".into(),
            composition: None,
            width: CanonicalInputTarget::Length(id(10)),
            nap: flag,
            directional_print: flag,
            striped: flag,
            checked: flag,
            shrinkage: None,
        },
        inputs.structure.inputs(),
    )
    .expect("actual material, unknown flags remain unknown");
    inputs.materials = MaterialInventory::new(vec![material], inputs.structure.inputs())
        .expect("complete actual Material inventory");
    sewing.inputs = DesignInputs::new(inputs).expect("actual input root");
    definition.sewing = sc_core::design::DesignSewing::new(sewing).expect("same held sewing");
    let root = definition.sewing.inputs().structure();
    let mut facing = definition
        .layers
        .facings()
        .first()
        .expect("Facing")
        .definition()
        .clone();
    facing.material = MaterialAssignment::Assigned(id(220000));
    definition.layers = LayerInventory::new(
        vec![Facing::new(
            facing,
            root.pieces().pieces().first().expect("served Piece"),
            root.topology(),
        )
        .expect("same current Facing sources")],
        definition.layers.linings().to_vec(),
        definition.layers.interfacings().to_vec(),
    )
    .expect("original family order");
    definition
}

#[test]
fn unified_views_cover_every_actual_family_in_role_order_with_paired_layer_material_and_offset() {
    let definition = assigned_definition();
    let saved = definition.clone();
    let garment = DesignGarment::new(definition).expect("actual complete owner");
    let canonical = garment.inputs().inputs();
    let views = garment
        .declarations()
        .expect("complete source and role preflight");
    let mut actual = Vec::new();
    let mut assigned = 0;
    let mut unresolved = 0;
    for view in views {
        assert!(core::ptr::eq(view.canonical_inputs(), canonical));
        match view.source() {
            Source::SewingAmount(value) => {
                assert!(core::ptr::eq(value.canonical_inputs(), canonical));
                assert!(core::ptr::eq(
                    value.span(),
                    garment.sewing().graph().spans().first().expect("span")
                ));
                actual.push(("sewing", value.span().id()));
            }
            Source::PocketOpening(value) => {
                assert!(core::ptr::eq(value.canonical_inputs(), canonical));
                assert!(core::ptr::eq(
                    value.pocket(),
                    garment
                        .pockets()
                        .pockets()
                        .get(value.index())
                        .expect("original Pocket")
                ));
                actual.push(("pocket", value.pocket().id()));
            }
            Source::Closure(value) => {
                assert!(core::ptr::eq(value.canonical_inputs(), canonical));
                assert!(core::ptr::eq(
                    value.closure(),
                    garment
                        .closures()
                        .closures()
                        .first()
                        .expect("actual Closure")
                ));
                let role = match value.role() {
                    DesignClosureDeclarationRole::HookSize => "hook",
                    DesignClosureDeclarationRole::BarSize => "bar",
                    other => panic!("unexpected actual Closure role: {other:?}"),
                };
                actual.push((role, value.closure().id()));
            }
            Source::Layer { assignment, offset } => {
                assert!(core::ptr::eq(offset.canonical_inputs(), canonical));
                let original = garment
                    .layers()
                    .entries()
                    .find(|(_, layer)| layer.id() == offset.layer().id())
                    .expect("actual original layer")
                    .1;
                assert!(core::ptr::eq(
                    offset.layer().definition(),
                    original.definition()
                ));
                match assignment {
                    LayerMaterialView::Assigned { layer, material } => {
                        assigned += 1;
                        assert!(core::ptr::eq(layer.definition(), original.definition()));
                        assert!(core::ptr::eq(
                            material,
                            garment
                                .inputs()
                                .materials()
                                .material(id(220000))
                                .expect("actual Material")
                        ));
                    }
                    LayerMaterialView::Unresolved { layer, reason } => {
                        unresolved += 1;
                        assert!(core::ptr::eq(layer.definition(), original.definition()));
                        let MaterialAssignment::Unresolved(held) = &original.definition().material
                        else {
                            panic!("original unresolved state")
                        };
                        assert_eq!(reason.len(), held.len());
                        assert!(core::ptr::eq(reason.as_ptr(), held.as_ptr()));
                    }
                }
                actual.push(("layer", offset.layer().id()));
            }
            Source::Hem(value) => {
                assert!(core::ptr::eq(value.canonical_inputs(), canonical));
                assert!(core::ptr::eq(
                    value.hem(),
                    garment.hems().hems().first().expect("actual Hem")
                ));
                let role = match value.role() {
                    DesignHemDeclarationRole::Depth => "depth",
                    DesignHemDeclarationRole::FoldType => "fold",
                };
                actual.push((role, value.hem().id()));
            }
            Source::Construction(value) => {
                assert!(core::ptr::eq(value.canonical_inputs(), canonical));
                macro_rules! same {
                    ($held:expr,$values:expr) => {
                        assert!(core::ptr::eq(
                            $held,
                            $values
                                .get(value.position().index)
                                .expect("actual original object")
                        ))
                    };
                }
                match value.construction() {
                    ConstructionRef::Dart(held) => same!(held, garment.constructions().darts()),
                    ConstructionRef::Tuck(held) => same!(held, garment.constructions().tucks()),
                    ConstructionRef::Pleat(held) => same!(held, garment.constructions().pleats()),
                    ConstructionRef::Grainline(held) => {
                        same!(held, garment.constructions().grainlines())
                    }
                    ConstructionRef::SeamAllowance(held) => {
                        same!(held, garment.constructions().allowances())
                    }
                    ConstructionRef::Gather(_) => {
                        panic!("Gather keeps its distinct joined adapter")
                    }
                }
                let role = match value.role() {
                    DesignConstructionDeclarationRole::Intake => "intake",
                    DesignConstructionDeclarationRole::ClosingOperation => "closing",
                    DesignConstructionDeclarationRole::GrainAlignment => "grain",
                    DesignConstructionDeclarationRole::AllowanceWidth => "width",
                    DesignConstructionDeclarationRole::AllowanceInclusion => "inclusion",
                };
                actual.push((role, value.construction().id()));
            }
            Source::Gather(value) => {
                assert!(core::ptr::eq(value.canonical_inputs(), canonical));
                assert!(core::ptr::eq(
                    value.gather(),
                    garment
                        .constructions()
                        .gathers()
                        .get(value.position().index)
                        .expect("actual Gather")
                ));
                assert!(core::ptr::eq(
                    value.intake_source().declaration,
                    &garment
                        .sewing()
                        .graph()
                        .spans()
                        .first()
                        .expect("span")
                        .definition()
                        .ease
                ));
                actual.push(("gather", value.gather().id()));
            }
        }
        let debug = format!("{view:?}");
        assert!(
            !debug.contains("EntityId") && !debug.contains("private") && !debug.contains("Unknown")
        );
    }
    assert_eq!((assigned, unresolved), (1, 2));
    assert_eq!(
        actual,
        vec![
            ("sewing", id(97000)),
            ("pocket", id(200000)),
            ("hook", id(200200)),
            ("bar", id(200200)),
            ("layer", id(200300)),
            ("layer", id(200301)),
            ("layer", id(200302)),
            ("depth", id(200400)),
            ("fold", id(200400)),
            ("intake", id(200500)),
            ("closing", id(200500)),
            ("intake", id(200501)),
            ("closing", id(200501)),
            ("intake", id(200502)),
            ("closing", id(200502)),
            ("gather", id(200503)),
            ("grain", id(200504)),
            ("width", id(200505)),
            ("inclusion", id(200505)),
        ]
    );
    assert!(garment.layers().require_in_scope().is_err());
    assert_eq!(garment.definition(), &saved);
}

fn late_missing_width_parameter() -> DesignGarmentDefinition<DesignSources> {
    let mut definition = garment_ownership::definition();
    let root = definition.sewing.inputs().structure();
    let original = &definition.constructions;
    let mut allowance = original
        .allowances()
        .first()
        .expect("actual allowance")
        .definition()
        .clone();
    allowance.width = AllowanceWidth::Formula(id(2000));
    let allowance = SeamAllowance::new(
        allowance,
        root.pieces().pieces().first().expect("Piece"),
        root.topology(),
    )
    .expect("symbolic width source remains held intent");
    definition.constructions = ConstructionInventory::new(
        original.darts().to_vec(),
        original.tucks().to_vec(),
        original.pleats().to_vec(),
        original.gathers().to_vec(),
        original.grainlines().to_vec(),
        vec![allowance],
    )
    .expect("same ordered owners");
    definition
}

#[test]
fn late_metadata_failures_preserve_original_role_causes_without_exposing_earlier_views() {
    let definition = late_missing_width_parameter();
    let saved = definition.clone();
    let garment = DesignGarment::new(definition).expect("current source scope");
    assert!(garment.validate_current().is_ok());
    let original = DesignConstructionDeclarationError::MissingFormulaParameter {
        construction: id(200505),
        position: ConstructionInventoryPosition {
            kind: ConstructionKind::SeamAllowance,
            index: 0,
        },
        role: DesignConstructionDeclarationRole::AllowanceWidth,
        parameter: id(2000),
    };
    let error = garment
        .declarations()
        .expect_err("late width role refuses whole collection");
    assert_eq!(error, Error::Constructions(Box::new(original.clone())));
    assert_eq!(
        std::error::Error::source(&error)
            .expect("original role cause")
            .downcast_ref::<DesignConstructionDeclarationError<sc_measure::DesignSourcesError>>(),
        Some(&original)
    );
    assert_eq!(garment.definition(), &saved);
    let mut definition = garment_ownership::definition();
    let root = definition.sewing.inputs().structure();
    let mut hem = definition
        .hems
        .hems()
        .first()
        .expect("actual Hem")
        .definition()
        .clone();
    hem.depth = HemDepth::Formula(id(5001));
    definition.hems = HemInventory::new(vec![Hem::new(
        hem,
        root.pieces().pieces().first().expect("Piece"),
        root.topology(),
        None,
    )
    .expect("formula depth intent")])
    .expect("same Hem owner");
    let mut sewing = definition.sewing.definition().clone();
    let mut inputs = sewing.inputs.definition().clone();
    inputs.parameters = sc_core::design::DesignParameters::new(
        vec![sc_core::design::DesignParameterDefinition {
            id: id(5001),
            token: sc_core::name::MachineToken::new("actual_count").expect("token"),
            target: sc_core::design::DesignParameterTarget::Scalar {
                declaration: id(2000),
                kind: ScalarInputKind::Count,
            },
        }],
        inputs.structure.inputs(),
    )
    .expect("actual Count metadata");
    sewing.inputs = DesignInputs::new(inputs).expect("valid current root metadata");
    definition.sewing = sc_core::design::DesignSewing::new(sewing).expect("same sewing sources");
    let garment = DesignGarment::new(definition).expect("owned current source scope");
    assert!(garment.validate_current().is_ok());
    let original = DesignHemDeclarationError::FormulaKind {
        hem: id(200400),
        parameter: id(5001),
        actual: FormulaKind::Count,
    };
    assert_eq!(
        garment.declarations().expect_err("actual Hem Length role"),
        Error::Hems(Box::new(original))
    );
}

#[test]
fn complete_current_sources_precede_late_metadata_and_keep_the_original_family_chain() {
    let mut definition = late_missing_width_parameter();
    garment_current::lose_edge(&mut definition, 1);
    let garment =
        DesignGarment::new(definition).expect("held source and formula issues inspectable");
    let original = garment
        .validate_current()
        .expect_err("current sewing source before metadata");
    assert!(matches!(
        original,
        sc_core::design::DesignGarmentError::Sewing(_)
    ));
    let error = garment
        .declarations()
        .expect_err("whole source preflight, no views");
    assert_eq!(error, Error::Garment(Box::new(original.clone())));
    assert_eq!(
        std::error::Error::source(&error)
            .expect("original current garment cause")
            .downcast_ref::<sc_core::design::DesignGarmentError<sc_measure::DesignSourcesError>>(),
        Some(&original)
    );
}
