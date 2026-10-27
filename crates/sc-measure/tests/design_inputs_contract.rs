//! Actual higher source schemas compose with core input ownership without a dependency cycle.
#![allow(clippy::expect_used)]
#[path = "support/owned_size_set_fixture.rs"]
mod support;
use sc_core::{
    command::DesignRevision,
    design::{
        DesignFormulaError, DesignFormulaHost, DesignHostEntity, DesignHostEntityKind,
        DesignInputs, DesignInputsDefinition, DesignInputsError, DesignInputsPosition,
        DesignMetadata, DesignMetadataError, DesignParameterDefinition, DesignParameterTarget,
        DesignParameters, DesignSizeSetReference, DesignSourceReferences, DesignStructure,
        DesignStructureDefinition, DesignStructurePosition,
    },
    name::MachineToken,
    ontology::{CutPlan, IdentityLedger, MaterialInventory, PieceInventory},
    recipe::{
        FormulaBindingKind, FormulaDeclarationSource, FormulaInputOrigin, FormulaNamespaceError,
        FormulaRecipeDocument,
    },
    value::CanonicalInputs,
};
use sc_measure::*;
use support::id;
fn fixture() -> DesignInputsDefinition<DesignSources> {
    let (canonical, measurements, mtm, garment) = support::inventory_fixture();
    let size_sets =
        SizeSetInventory::new(vec![mtm.into(), garment.into()], &canonical, &measurements)
            .expect("actual external sources");
    let host = DesignSources::new(
        DesignSourcesDefinition {
            measurements,
            size_sets,
        },
        &canonical,
    )
    .expect("actual metadata host");
    let sources = DesignMetadata::new(host, &canonical).expect("actual core host acceptance");
    let parameters = DesignParameters::new(
        vec![DesignParameterDefinition {
            id: id(5001),
            token: MachineToken::new("authored_body").expect("explicit name"),
            target: DesignParameterTarget::Length(id(10)),
        }],
        &canonical,
    )
    .expect("actual canonical parameter target");
    let materials =
        MaterialInventory::new(vec![], &canonical).expect("explicit empty material inventory");
    let structure = DesignStructure::new(DesignStructureDefinition {
        id: id(5000),
        revision: DesignRevision::initial(),
        inputs: canonical,
        pieces: PieceInventory::new(vec![]).expect("explicit empty Pieces"),
        cut_plan: CutPlan::new(vec![], &[]).expect("explicit empty plan"),
        topology: IdentityLedger::new(),
    })
    .expect("actual structural owner");
    DesignInputsDefinition {
        references: DesignSourceReferences {
            measurement_table: None,
            ease_set: None,
            size_set: None,
        },
        structure,
        parameters,
        materials,
        sources,
        recipe: FormulaRecipeDocument::new("let width:length=body_waist".to_owned())
            .expect("exact owned recipe syntax"),
    }
}
#[test]
fn actual_source_host_borrows_the_single_canonical_owner_inside_core_composition() {
    let state = DesignInputs::new(fixture()).expect("actual cross-crate composition");
    assert_eq!(state.id(), id(5000));
    assert!(core::ptr::eq(state.inputs(), state.structure().inputs()));
    let declaration = state
        .sources()
        .measurement_declaration(id(1), state.inputs())
        .expect("actual current body binding");
    assert!(core::ptr::eq(
        declaration,
        state
            .inputs()
            .length(id(10))
            .expect("single actual canonical record")
    ));
    assert_eq!(state.sources().host().size_sets().sets().len(), 2);
    assert_eq!(state.recipe().source(), "let width:length=body_waist");
}
#[test]
fn actual_external_set_metadata_cannot_reuse_the_containing_design_identity() {
    let mut definition = fixture();
    let mut structure = definition.structure.definition().clone();
    structure.id = id(40);
    definition.structure = DesignStructure::new(structure).expect("valid standalone structural id");
    assert_eq!(
        DesignInputs::new(definition).expect_err("actual whole registry collision"),
        DesignInputsError::DuplicateIdentity {
            identity: id(40),
            first: DesignInputsPosition::Structure(DesignStructurePosition::Design),
            duplicate: DesignInputsPosition::Source(DesignHostEntity {
                id: id(40),
                kind: DesignHostEntityKind::SizeSet,
                index: 0
            })
        }
    );
}
#[test]
fn sources_are_revalidated_against_owned_values_after_standalone_construction() {
    let mut definition = fixture();
    let mut structure = definition.structure.definition().clone();
    structure.inputs = CanonicalInputs::new(structure.inputs.lengths().to_vec(), vec![])
        .expect("removed actual garment coordinate");
    definition.structure = DesignStructure::new(structure).expect("actual replacement structure");
    assert!(matches!(DesignInputs::new(definition),
        Err(DesignInputsError::Sources(issue)) if matches!(issue.as_ref(),DesignMetadataError::Host(
            DesignSourcesError::SizeSets(issue)) if matches!(issue.as_ref(),SizeSetInventoryError::InvalidSet {set,..} if set.id==id(1000)))));
}

fn references(
    table: Option<u128>,
    ease: Option<u128>,
    size: Option<(u128, u32)>,
) -> DesignSourceReferences {
    DesignSourceReferences {
        measurement_table: table.map(id),
        ease_set: ease.map(id),
        size_set: size.map(|(identity, revision)| {
            DesignSizeSetReference::new(id(identity), sc_units::Count::new(revision))
        }),
    }
}
#[test]
fn provided_and_unprovided_reference_fields_are_owned_exactly_without_inferred_selection() {
    for authored in [
        references(None, None, None),
        references(None, None, Some((40, 2))),
        references(Some(51), Some(60), Some((40, 2))),
        references(Some(51), Some(60), Some((1000, 0))),
    ] {
        let mut definition = fixture();
        definition.references = authored;
        let state = DesignInputs::new(definition)
            .expect("actual supplied references with valid joint scope");
        assert_eq!(state.references(), authored);
        assert_eq!(state.validate_current(), Ok(()));
    }
}
#[test]
fn numeric_presence_and_wrong_revision_cannot_replace_provided_source_metadata() {
    let expected = DesignSizeSetReference::new(id(40), sc_units::Count::new(3));
    let actual = DesignSizeSetReference::new(id(40), sc_units::Count::new(2));
    for (authored, issue) in [
        (
            references(Some(2000), None, None),
            DesignSourcesError::MissingMeasurementTable { table: id(2000) },
        ),
        (
            references(None, Some(2000), None),
            DesignSourcesError::MissingEaseSet { set: id(2000) },
        ),
        (
            references(None, None, Some((2000, 0))),
            DesignSourcesError::SizeSets(Box::new(SizeSetInventoryError::MissingSet {
                expected: DesignSizeSetReference::new(id(2000), sc_units::Count::new(0)),
            })),
        ),
        (
            references(None, None, Some((40, 3))),
            DesignSourcesError::SizeSets(Box::new(SizeSetInventoryError::RevisionMismatch {
                expected,
                actual,
            })),
        ),
    ] {
        let mut definition = fixture();
        assert!(definition.structure.inputs().scalar(id(2000)).is_some());
        definition.references = authored;
        assert_eq!(
            DesignInputs::new(definition).expect_err("provided metadata must actually match"),
            DesignInputsError::References(Box::new(issue))
        );
    }
}
#[test]
fn provided_table_must_match_actual_selected_ease_and_size_sources() {
    for (authored, issue) in [
        (
            references(Some(50), Some(60), None),
            DesignSourcesError::EaseTableMismatch {
                ease_set: id(60),
                selected_table: id(50),
                actual_table: id(51),
            },
        ),
        (
            references(Some(50), None, Some((40, 2))),
            DesignSourcesError::SizeTableMismatch {
                size_set: DesignSizeSetReference::new(id(40), sc_units::Count::new(2)),
                selected_table: id(50),
                actual_table: id(51),
            },
        ),
    ] {
        let mut definition = fixture();
        definition.references = authored;
        assert_eq!(
            DesignInputs::new(definition).expect_err("individually present references disagree"),
            DesignInputsError::References(Box::new(issue))
        );
    }
}
#[test]
fn mtm_chart_keeps_its_exact_ease_set_even_when_another_set_has_matching_content() {
    let mut definition = fixture();
    let canonical = definition.structure.inputs();
    let original = definition.sources.host();
    let mut inputs = original.measurements().definition().clone();
    let peer = {
        let records = inputs
            .measurements
            .record_context(canonical)
            .expect("actual records");
        let tables =
            MeasurementTableContext::new(&inputs.measurements.definition().measurements, &records)
                .expect("actual metadata");
        let context = EaseSetContext::new(
            &inputs.measurements.definition().tables,
            &inputs.eases,
            &tables,
        )
        .expect("actual mapping context");
        let mut peer = inputs
            .ease_sets
            .first()
            .expect("actual selected set")
            .definition()
            .clone();
        peer.id = id(61);
        EaseSet::new(peer, &context).expect("distinct actual set with matching entries and tables")
    };
    inputs.ease_sets.push(peer);
    let measurements =
        MeasurementInputs::new(inputs, canonical).expect("actual complete replacement metadata");
    let sources = DesignSources::new(
        DesignSourcesDefinition {
            measurements,
            size_sets: original.size_sets().clone(),
        },
        canonical,
    )
    .expect("both actual sets valid in current source scope");
    definition.sources = DesignMetadata::new(sources, canonical).expect("actual current provider");
    definition.references = references(Some(51), Some(61), Some((40, 2)));
    assert_eq!(
        DesignInputs::new(definition).expect_err("peer cannot replace the chart's saved identity"),
        DesignInputsError::References(Box::new(DesignSourcesError::MtmEaseSetMismatch {
            size_set: DesignSizeSetReference::new(id(40), sc_units::Count::new(2)),
            selected_set: id(61),
            actual_set: id(60)
        }))
    );
}

#[test]
fn selected_formula_names_borrow_actual_canonical_records_with_original_metadata_origins() {
    let mut definition = fixture();
    definition.references = references(Some(51), Some(60), None);
    definition.parameters = DesignParameters::new(
        vec![
            definition
                .parameters
                .parameters()
                .first()
                .expect("actual parameter")
                .definition()
                .clone(),
            DesignParameterDefinition {
                id: id(5002),
                token: MachineToken::new("fitting_count").expect("explicit name"),
                target: DesignParameterTarget::Scalar {
                    declaration: id(2000),
                    kind: sc_core::value::ScalarInputKind::Count,
                },
            },
        ],
        definition.structure.inputs(),
    )
    .expect("actual general scalar parameter");
    let state = DesignInputs::new(definition).expect("actual selected state");
    let original = state.inputs().clone();
    let namespace = state
        .formula_namespace()
        .expect("actual composed namespace");
    assert_eq!(namespace.declarations().len(), 14);
    for (name, origin, input, target) in [
        ("authored_body", FormulaInputOrigin::Parameter, 5001, 10),
        ("garment_waist", FormulaInputOrigin::Measurement, 2, 11),
        ("waistband", FormulaInputOrigin::Measurement, 3, 12),
        ("waist_ease", FormulaInputOrigin::Ease, 30, 13),
        ("waistband_ease", FormulaInputOrigin::Ease, 31, 14),
    ] {
        let token = MachineToken::new(name).expect("authored name");
        let declaration = namespace.resolve(&token).expect("selected declaration");
        let (actual, metadata, declaration) = match declaration.source() {
            FormulaDeclarationSource::LengthInput {
                origin,
                input,
                declaration,
            } => Some((origin, input, declaration)),
            _ => None,
        }
        .expect("actual dedicated length source");
        assert_eq!((actual, metadata), (origin, id(input)));
        assert!(core::ptr::eq(
            declaration,
            state
                .inputs()
                .length(id(target))
                .expect("single actual owner")
        ));
    }
    let count = namespace
        .resolve(&MachineToken::new("fitting_count").expect("name"))
        .expect("general parameter");
    assert!(
        matches!(count.source(), FormulaDeclarationSource::Input { input, declaration, kind: FormulaBindingKind::Count, .. } if input == id(5002) && declaration == id(2000))
    );
    assert!(namespace
        .resolve(&MachineToken::new("body_waist").expect("unselected name"))
        .is_err());
    assert_eq!(state.inputs(), &original);
}

#[test]
fn unprovided_selection_does_not_import_names_from_available_metadata_or_selected_size_set() {
    for authored in [
        references(None, None, None),
        references(None, None, Some((40, 2))),
    ] {
        let mut definition = fixture();
        definition.references = authored;
        let state = DesignInputs::new(definition).expect("explicit draft selection");
        let namespace = state
            .formula_namespace()
            .expect("parameter and reserved metadata only");
        assert_eq!(namespace.declarations().len(), 9);
        for name in ["body_waist", "garment_waist", "waist_ease"] {
            assert!(namespace
                .resolve(&MachineToken::new(name).expect("valid query"))
                .is_err());
        }
    }
    let mut definition = fixture();
    definition.references = references(Some(50), None, None);
    let state = DesignInputs::new(definition).expect("body table explicitly selected");
    let namespace = state.formula_namespace().expect("only selected body names");
    assert_eq!(namespace.declarations().len(), 10);
    assert!(namespace
        .resolve(&MachineToken::new("body_waist").expect("name"))
        .is_ok());
    assert!(namespace
        .resolve(&MachineToken::new("garment_waist").expect("name"))
        .is_err());
}

#[test]
fn cross_origin_collision_retains_both_actual_sources_without_shadowing() {
    let mut definition = fixture();
    definition.references = references(Some(51), Some(60), None);
    definition.parameters = DesignParameters::new(
        vec![DesignParameterDefinition {
            id: id(5001),
            token: MachineToken::new("garment_waist").expect("same authored spelling"),
            target: DesignParameterTarget::Length(id(10)),
        }],
        definition.structure.inputs(),
    )
    .expect("valid standalone parameter");
    let state = DesignInputs::new(definition)
        .expect("input admission remains distinct from namespace checking");
    let issue = match state
        .formula_namespace()
        .expect_err("neither origin shadows the other")
    {
        DesignFormulaError::Namespace(issue @ FormulaNamespaceError::AmbiguousName { .. }) => {
            Some(issue)
        }
        _ => None,
    }
    .expect("original ambiguity");
    assert_eq!(issue.token(), "formula_ambiguous_name");
    let sources = issue.binding_sources();
    assert_eq!(
        sources.map(|source| source.source().origin()),
        [
            sc_core::recipe::FormulaOrigin::Parameter,
            sc_core::recipe::FormulaOrigin::Measurement
        ]
    );
    for (source, metadata, target) in sources
        .into_iter()
        .zip([5001, 2])
        .zip([10, 11])
        .map(|((a, b), c)| (a, b, c))
    {
        assert!(
            matches!(source.source(), FormulaDeclarationSource::LengthInput { input, declaration, .. } if input == id(metadata) && core::ptr::eq(declaration, state.inputs().length(id(target)).expect("actual target")))
        );
    }
}

#[test]
fn selected_ease_reserved_name_keeps_original_attempted_mapping_and_borrowed_source() {
    let mut definition = fixture();
    let canonical = definition.structure.inputs();
    let original = definition.sources.host();
    let mut inputs = original.measurements().definition().clone();
    let peer = {
        let records = inputs
            .measurements
            .record_context(canonical)
            .expect("actual records");
        let tables =
            MeasurementTableContext::new(&inputs.measurements.definition().measurements, &records)
                .expect("actual metadata");
        let context = EaseSetContext::new(
            &inputs.measurements.definition().tables,
            &inputs.eases,
            &tables,
        )
        .expect("actual mapping context");
        let mut peer = inputs
            .ease_sets
            .first()
            .expect("actual set")
            .definition()
            .clone();
        peer.id = id(61);
        peer.entries.first_mut().expect("actual mapping").token =
            MachineToken::new("eps_num").expect("reserved token is a valid machine spelling");
        EaseSet::new(peer, &context).expect("local set shape remains separate from flat namespace")
    };
    inputs.ease_sets.push(peer);
    let measurements = MeasurementInputs::new(inputs, canonical).expect("actual current sources");
    let sources = DesignSources::new(
        DesignSourcesDefinition {
            measurements,
            size_sets: original.size_sets().clone(),
        },
        canonical,
    )
    .expect("actual provider");
    definition.sources = DesignMetadata::new(sources, canonical).expect("core current owner");
    definition.references = references(Some(51), Some(61), None);
    let state = DesignInputs::new(definition).expect("actual input scope");
    let issue = match state
        .formula_namespace()
        .expect_err("reserved name cannot be authored")
    {
        DesignFormulaError::Namespace(issue @ FormulaNamespaceError::ReservedBinding { .. }) => {
            Some(issue)
        }
        _ => None,
    }
    .expect("original reserved refusal");
    assert_eq!(issue.token(), "formula_rebinding");
    let [reserved, attempted] = issue.binding_sources();
    assert_eq!(reserved.name(), "eps_num");
    assert!(
        matches!(attempted.source(), FormulaDeclarationSource::LengthInput { origin: FormulaInputOrigin::Ease, input, declaration } if input == id(30) && core::ptr::eq(declaration, state.inputs().length(id(13)).expect("original canonical Ease")))
    );
}

#[test]
fn formula_source_query_rechecks_unselected_actual_sources_against_replacement_canonical_inputs() {
    let definition = fixture();
    let canonical = definition.structure.inputs();
    let host = definition.sources.host();
    assert!(host
        .formula_declarations(references(None, None, None), canonical)
        .expect("no selection, current inventory")
        .is_empty());
    let replacement =
        CanonicalInputs::new(canonical.lengths().to_vec(), vec![]).expect("explicit replacement");
    assert!(matches!(
        host.formula_declarations(references(None, None, None), &replacement),
        Err(DesignSourcesError::SizeSets(_))
    ));
}

fn with_sewing_graph(
    mut definition: DesignInputsDefinition<DesignSources>,
    amounts: Vec<sc_core::ontology::EaseAmount>,
) -> (DesignInputs<DesignSources>, sc_core::ontology::SewingGraph) {
    use sc_core::ontology::{
        CopyOrientation, CutCopyDefinition, CuttingSide, DeclaredEase, DeterministicIdGenerator,
        DirectedEdge, Direction, EaseDistribution, EdgeRange, LabelText, MaterialAssignment,
        Mirroring, Piece, PieceDefinition, SeamDirection, SeamSide, SeamSpanDefinition,
        SewingGraph, SewingGraphDefinition,
    };
    let mut topology = IdentityLedger::new();
    let edges = topology
        .declare_edges(&mut DeterministicIdGenerator::from_start(90000), 2)
        .expect("actual source edges");
    let pieces = edges
        .into_iter()
        .zip([95000, 95001])
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
                    material: MaterialAssignment::Unresolved("explicit selection pending".into()),
                    layer_index: 0,
                    label: LabelText {
                        name: "sewing source".into(),
                        size: "authored".into(),
                        fabric: "unassigned".into(),
                        colorway: "explicit".into(),
                    },
                },
                &topology,
            )
            .expect("actual born Piece")
        })
        .collect::<Vec<_>>();
    let plan = CutPlan::new(
        pieces
            .iter()
            .zip([96000, 96001])
            .map(|(piece, identity)| CutCopyDefinition {
                id: id(identity),
                piece: piece.id(),
                orientation: CopyOrientation::Authored,
            })
            .collect(),
        &pieces,
    )
    .expect("actual complete copies");
    let first = pieces
        .first()
        .expect("first source Piece")
        .edges()
        .next()
        .expect("held edge")
        .edge;
    let second = pieces
        .get(1)
        .expect("second source Piece")
        .edges()
        .next()
        .expect("held edge")
        .edge;
    let graph = SewingGraph::new(
        SewingGraphDefinition {
            id: id(94000),
            spans: amounts
                .into_iter()
                .enumerate()
                .map(|(index, amount)| SeamSpanDefinition {
                    id: id(97000 + u128::try_from(index).expect("authored span ordinal")),
                    a: SeamSide {
                        copy: id(96000),
                        range: EdgeRange::whole(first),
                    },
                    b: SeamSide {
                        copy: id(96001),
                        range: EdgeRange::whole(second),
                    },
                    direction: SeamDirection::Opposite,
                    ease: DeclaredEase {
                        amount,
                        distribution: EaseDistribution::Uniform,
                    },
                    stops: vec![],
                })
                .collect(),
        },
        &plan,
        &pieces,
        &[],
        &topology,
    )
    .expect("actual sewing graph");
    let mut structure = definition.structure.definition().clone();
    structure.pieces = PieceInventory::new(pieces).expect("actual complete Pieces");
    structure.cut_plan = plan;
    structure.topology = topology;
    definition.structure =
        DesignStructure::new(structure).expect("actual joined structural sources");
    (
        DesignInputs::new(definition).expect("actual current root"),
        graph,
    )
}

#[test]
fn sewing_amount_views_borrow_actual_explicit_and_both_length_parameter_families_with_visible_profile_deferral(
) {
    use sc_core::{
        design::DesignSewingAmountSource,
        ontology::{EaseAmount, ProfileBindingValidation, ProfileParameterRef},
        value::{
            ScalarInputDeclaration, ScalarInputDeclarationDefinition, ScalarInputKind,
            ScalarInputState,
        },
    };
    let mut definition = fixture();
    let mut structure = definition.structure.definition().clone();
    let mut scalars = structure.inputs.scalars().to_vec();
    scalars.push(
        ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
            id: id(2003),
            source: id(2005),
            kind: ScalarInputKind::Length,
            state: ScalarInputState::Unknown {
                observation: id(2004),
            },
        })
        .expect("actual general Length input"),
    );
    structure.inputs = CanonicalInputs::new(structure.inputs.lengths().to_vec(), scalars)
        .expect("single combined canonical owner");
    definition.structure = DesignStructure::new(structure).expect("actual updated structure");
    let mut parameters = definition
        .parameters
        .parameters()
        .iter()
        .map(|parameter| parameter.definition().clone())
        .collect::<Vec<_>>();
    parameters.push(DesignParameterDefinition {
        id: id(5002),
        token: MachineToken::new("sewing_extra").expect("authored name"),
        target: DesignParameterTarget::Scalar {
            declaration: id(2003),
            kind: ScalarInputKind::Length,
        },
    });
    definition.parameters = DesignParameters::new(parameters, definition.structure.inputs())
        .expect("actual general parameter binding");
    let (state, graph) = with_sewing_graph(
        definition,
        vec![
            EaseAmount::Explicit(sc_units::Length::from_micrometres(-1500).expect("signed length")),
            EaseAmount::Formula(id(5001)),
            EaseAmount::Formula(id(5002)),
            EaseAmount::Profile(ProfileParameterRef::new(id(10))),
        ],
    );
    let original = (state.inputs().clone(), graph.clone());
    let views = state
        .sewing_amount_declarations(&graph)
        .expect("actual typed declarations");
    assert_eq!(views.len(), 4);
    for (view, span) in views.iter().zip(graph.spans()) {
        assert!(core::ptr::eq(view.span(), span));
        assert!(core::ptr::eq(view.canonical_inputs(), state.inputs()));
        match view.source() {
            DesignSewingAmountSource::Explicit(value) => {
                let actual = match &span.definition().ease.amount {
                    EaseAmount::Explicit(value) => Some(value),
                    _ => None,
                }
                .expect("original explicit source");
                assert!(core::ptr::eq(value, actual));
                assert_eq!(value.as_micrometres(), -1500);
            }
            DesignSewingAmountSource::Formula {
                parameter,
                declaration,
            } => {
                assert!(state
                    .parameters()
                    .parameters()
                    .iter()
                    .any(|actual| core::ptr::eq(actual, parameter)));
                assert_eq!(
                    declaration.source().kind(),
                    sc_core::recipe::FormulaKind::Length
                );
                let checked = match declaration.source() {
                    FormulaDeclarationSource::LengthInput {
                        input, declaration, ..
                    } => {
                        assert_eq!(input, id(5001));
                        assert!(core::ptr::eq(
                            declaration,
                            state
                                .inputs()
                                .length(id(10))
                                .expect("actual dedicated record")
                        ));
                        Some(())
                    }
                    FormulaDeclarationSource::Input {
                        input,
                        declaration,
                        kind: FormulaBindingKind::Length,
                        ..
                    } => {
                        assert_eq!((input, declaration), (id(5002), id(2003)));
                        Some(())
                    }
                    _ => None,
                };
                checked.expect("actual length parameter family");
            }
            DesignSewingAmountSource::Profile(reference) => {
                assert_eq!(reference, ProfileParameterRef::new(id(10)));
                assert_eq!(
                    view.profile_validation(),
                    Some(ProfileBindingValidation::DeferredToG4)
                );
                assert!(state.inputs().length(id(10)).is_some());
            }
        }
    }
    assert_eq!(state.inputs(), &original.0);
    assert_eq!(graph, original.1);
}

#[test]
fn sewing_formula_amount_cannot_infer_parameter_metadata_from_numeric_canonical_presence() {
    use sc_core::{design::DesignSewingDeclarationError, ontology::EaseAmount};
    for numeric_only in [10, 2000] {
        let (state, graph) =
            with_sewing_graph(fixture(), vec![EaseAmount::Formula(id(numeric_only))]);
        assert_eq!(
            state
                .sewing_amount_declarations(&graph)
                .expect_err("canonical identity alone is not parameter metadata"),
            DesignSewingDeclarationError::MissingFormulaParameter {
                span: id(97000),
                parameter: id(numeric_only)
            }
        );
    }
}

#[test]
fn sewing_formula_amount_requires_actual_length_kind_and_keeps_late_original_span_context() {
    use sc_core::{
        design::DesignSewingDeclarationError, ontology::EaseAmount, value::ScalarInputKind,
    };
    let mut definition = fixture();
    definition.parameters = DesignParameters::new(
        vec![DesignParameterDefinition {
            id: id(5002),
            token: MachineToken::new("sewing_count").expect("explicit name"),
            target: DesignParameterTarget::Scalar {
                declaration: id(2000),
                kind: ScalarInputKind::Count,
            },
        }],
        definition.structure.inputs(),
    )
    .expect("actual well-formed Count parameter");
    let (state, graph) = with_sewing_graph(
        definition,
        vec![
            EaseAmount::Explicit(sc_units::Length::ZERO),
            EaseAmount::Formula(id(5002)),
        ],
    );
    assert_eq!(
        state
            .sewing_amount_declarations(&graph)
            .expect_err("Count cannot become Length"),
        DesignSewingDeclarationError::FormulaKind {
            span: id(97001),
            parameter: id(5002),
            actual: sc_core::recipe::FormulaKind::Count
        }
    );
}

fn sewing_owner_definition() -> sc_core::design::DesignSewingDefinition<DesignSources> {
    use sc_core::{
        design::DesignSewingDefinition,
        ontology::{EaseAmount, EdgeAnchor, Param, Rational, SewingLandmark, TurnPoint},
    };
    let (mut inputs, graph) = with_sewing_graph(fixture(), vec![EaseAmount::Formula(id(5001))]);
    let structure = inputs.structure();
    let piece = structure.pieces().pieces().first().expect("actual Piece");
    let edge = piece.edges().next().expect("actual held edge").edge;
    let mark = SewingLandmark::Turn(
        TurnPoint::new(
            id(98000),
            EdgeAnchor {
                edge,
                param: Param::new(Rational::new(1, 2).expect("half")).expect("valid parameter"),
            },
            piece,
            structure.topology(),
        )
        .expect("actual born semantic turn"),
    );
    let mut root = inputs.definition().clone();
    let mut updated = root.structure.definition().clone();
    let mut edits = sc_core::ontology::DeterministicIdGenerator::from_start(110000);
    for held in updated
        .pieces
        .pieces()
        .iter()
        .flat_map(|piece| piece.edges())
    {
        updated
            .topology
            .split(
                &mut edits,
                held.edge,
                Param::new(Rational::new(1, 3).expect("third")).expect("valid parameter"),
            )
            .expect("retire every original output while keeping held graph/landmark references");
    }
    assert!(updated
        .topology
        .live_edges()
        .all(|edge| edge.creator() != id(90000)));
    root.structure = DesignStructure::new(updated).expect("current historical structural snapshot");
    inputs = DesignInputs::new(root).expect("same actual metadata over updated history");
    DesignSewingDefinition {
        inputs,
        graph,
        landmarks: vec![mark],
    }
}

#[test]
fn sewing_owner_preserves_exact_root_graph_landmarks_and_single_canonical_inventory() {
    use sc_core::design::DesignSewing;
    let definition = sewing_owner_definition();
    let original = definition.clone();
    let state = DesignSewing::new(definition).expect("complete actual ownership");
    assert_eq!(state.definition(), &original);
    assert!(core::ptr::eq(state.inputs(), &state.definition().inputs));
    assert!(core::ptr::eq(state.graph(), &state.definition().graph));
    assert!(core::ptr::eq(
        state.inputs().inputs(),
        state.inputs().structure().inputs()
    ));
    assert_eq!(state.landmarks(), original.landmarks);
    let declarations = state
        .inputs()
        .sewing_amount_declarations(state.graph())
        .expect("actual amount metadata");
    assert!(declarations
        .iter()
        .all(|view| core::ptr::eq(view.canonical_inputs(), state.inputs().inputs())));
    let structure = state.inputs().structure();
    assert_eq!(
        state.graph().validate_current(
            structure.cut_plan(),
            structure.pieces().pieces(),
            state.landmarks(),
            structure.topology()
        ),
        Ok(())
    );
}

#[test]
fn sewing_owner_refuses_graph_span_and_landmark_aliases_with_actual_root_positions() {
    use sc_core::{
        design::{DesignSewing, DesignSewingError, DesignSewingPosition},
        ontology::{SewingGraph, SewingGraphDefinition, SewingLandmark, TurnPoint},
    };
    for (identity, first, duplicate) in [
        (
            5000,
            DesignInputsPosition::Structure(DesignStructurePosition::Design),
            DesignSewingPosition::Graph,
        ),
        (
            5001,
            DesignInputsPosition::Parameter { index: 0 },
            DesignSewingPosition::Span { index: 0 },
        ),
        (
            90000,
            DesignInputsPosition::Structure(DesignStructurePosition::Operation { index: 0 }),
            DesignSewingPosition::Landmark { index: 0 },
        ),
    ] {
        let mut definition = sewing_owner_definition();
        let structure = definition.inputs.structure();
        if identity == 90000 {
            let original = definition.landmarks.first().expect("actual turn");
            let resolved = original
                .resolve(structure.topology())
                .resolved()
                .expect("current unique landmark position");
            let anchor = sc_core::ontology::EdgeAnchor {
                edge: resolved.edge(),
                param: resolved.param(),
            };
            definition.landmarks = vec![SewingLandmark::Turn(
                TurnPoint::new(
                    id(identity),
                    anchor,
                    structure
                        .pieces()
                        .piece(original.piece())
                        .expect("actual Piece"),
                    structure.topology(),
                )
                .expect("independently valid landmark"),
            )];
        } else {
            let mut spans = definition
                .graph
                .spans()
                .iter()
                .map(|span| span.definition().clone())
                .collect::<Vec<_>>();
            if identity == 5001 {
                spans.first_mut().expect("actual span").id = id(identity);
            }
            definition.graph = SewingGraph::new(
                SewingGraphDefinition {
                    id: if identity == 5000 {
                        id(identity)
                    } else {
                        definition.graph.id()
                    },
                    spans,
                },
                structure.cut_plan(),
                structure.pieces().pieces(),
                &definition.landmarks,
                structure.topology(),
            )
            .expect("independently valid sewing graph");
        }
        assert_eq!(
            DesignSewing::new(definition).expect_err("cross-component actual identity collision"),
            DesignSewingError::DuplicateIdentity {
                identity: id(identity),
                first: DesignSewingPosition::Inputs(first),
                duplicate
            }
        );
    }
}

#[test]
fn sewing_owner_preserves_held_graph_for_inspection_after_an_explicit_current_copy_replacement() {
    use sc_core::design::DesignSewing;
    let mut definition = sewing_owner_definition();
    let original_graph = definition.graph.clone();
    let mut root = definition.inputs.definition().clone();
    let mut structure = root.structure.definition().clone();
    structure.cut_plan = CutPlan::new(
        structure
            .cut_plan
            .copies()
            .iter()
            .map(|copy| {
                let mut copy = *copy.definition();
                if copy.id == id(96000) {
                    copy.id = id(96010);
                }
                copy
            })
            .collect(),
        structure.pieces.pieces(),
    )
    .expect("complete explicit replacement copies");
    root.structure = DesignStructure::new(structure).expect("actual current structure");
    definition.inputs = DesignInputs::new(root).expect("actual current metadata/root");
    let state = DesignSewing::new(definition).expect("held sewing draft remains inspectable");
    assert_eq!(state.graph(), &original_graph);
    let structure = state.inputs().structure();
    assert_eq!(
        state
            .graph()
            .missing_copies(structure.cut_plan())
            .collect::<Vec<_>>(),
        [id(96000)]
    );
    assert_eq!(
        state.graph().validate_current(
            structure.cut_plan(),
            structure.pieces().pieces(),
            state.landmarks(),
            structure.topology()
        ),
        Err(sc_core::ontology::SewingError::MissingCopy {
            span: id(97000),
            side: sc_core::ontology::SeamSideId::A,
            copy: id(96000)
        })
    );
    assert!(structure
        .topology_readiness()
        .expect("independent structural scope")
        .is_ready());
}

#[test]
fn sewing_current_sources_return_actual_borrowed_amounts_and_preserve_profile_deferral() {
    use sc_core::{
        design::{DesignSewing, DesignSewingDefinition},
        ontology::{EaseAmount, ProfileBindingValidation, ProfileParameterRef},
    };
    let definition = sewing_owner_definition();
    let original = definition.clone();
    let state = DesignSewing::new(definition).expect("owned actual state");
    assert_eq!(state.validate_current(), Ok(()));
    let declarations = state
        .amount_declarations()
        .expect("complete current sources");
    assert!(declarations
        .iter()
        .zip(state.graph().spans())
        .all(|(view, span)| core::ptr::eq(view.span(), span)
            && core::ptr::eq(view.canonical_inputs(), state.inputs().inputs())));
    assert_eq!(state.definition(), &original);
    let (inputs, graph) = with_sewing_graph(
        fixture(),
        vec![EaseAmount::Profile(ProfileParameterRef::new(id(10)))],
    );
    let profile = DesignSewing::new(DesignSewingDefinition {
        inputs,
        graph,
        landmarks: vec![],
    })
    .expect("logical Profile draft");
    assert_eq!(profile.validate_current(), Ok(()));
    assert_eq!(
        profile
            .amount_declarations()
            .expect("metadata current, physical role deferred")
            .first()
            .expect("actual span")
            .profile_validation(),
        Some(ProfileBindingValidation::DeferredToG4)
    );
}

#[test]
fn sewing_current_sources_validate_unused_landmark_anchors_and_keep_the_concrete_original_cause() {
    use sc_core::{
        design::{DesignSewing, DesignSewingError},
        ontology::{AnchorError, DeterministicIdGenerator},
    };
    use std::error::Error;
    let mut definition = sewing_owner_definition();
    assert!(definition
        .graph
        .spans()
        .iter()
        .all(|span| span.definition().stops.is_empty()));
    let point = definition
        .landmarks
        .first()
        .expect("unused actual landmark")
        .resolve(definition.inputs.structure().topology())
        .resolved()
        .expect("original live point");
    let mut root = definition.inputs.definition().clone();
    let mut structure = root.structure.definition().clone();
    structure
        .topology
        .delete(
            &mut DeterministicIdGenerator::from_start(120000),
            point.edge(),
        )
        .expect("explicit anchor material deletion");
    root.structure = DesignStructure::new(structure).expect("inspectable held structural damage");
    definition.inputs = DesignInputs::new(root).expect("current input metadata");
    let original_graph = definition.graph.clone();
    let state = DesignSewing::new(definition).expect("inspectable held sewing state");
    let issue = state
        .validate_current()
        .expect_err("unused metadata is still an owned current obligation");
    assert!(
        matches!(&issue, DesignSewingError::Landmark { landmark, issue } if *landmark == id(98000) && matches!(issue.as_ref(), AnchorError::CurrentUnresolved { .. }))
    );
    assert!(issue
        .source()
        .expect("original current anchor cause")
        .is::<AnchorError>());
    assert!(!issue
        .source()
        .expect("concrete cause")
        .is::<Box<AnchorError>>());
    assert_eq!(state.graph(), &original_graph);
}

#[test]
fn sewing_current_sources_reject_stale_actual_graph_copies_with_the_original_graph_error() {
    use sc_core::{
        design::{DesignSewing, DesignSewingError},
        ontology::{SeamSideId, SewingError},
    };
    use std::error::Error;
    let mut definition = sewing_owner_definition();
    let mut root = definition.inputs.definition().clone();
    let mut structure = root.structure.definition().clone();
    structure.cut_plan = CutPlan::new(
        structure
            .cut_plan
            .copies()
            .iter()
            .map(|copy| {
                let mut copy = *copy.definition();
                if copy.id == id(96000) {
                    copy.id = id(96010);
                }
                copy
            })
            .collect(),
        structure.pieces.pieces(),
    )
    .expect("actual explicit replacement plan");
    root.structure = DesignStructure::new(structure).expect("actual structure");
    definition.inputs = DesignInputs::new(root).expect("actual current input metadata");
    let state = DesignSewing::new(definition).expect("old sewing refs remain inspectable");
    assert!(state
        .inputs()
        .sewing_amount_declarations(state.graph())
        .is_ok());
    let expected = SewingError::MissingCopy {
        span: id(97000),
        side: SeamSideId::A,
        copy: id(96000),
    };
    let issue = state
        .validate_current()
        .expect_err("metadata-only success cannot certify graph sources");
    assert_eq!(issue, DesignSewingError::Graph(Box::new(expected.clone())));
    assert_eq!(
        issue
            .source()
            .expect("original graph cause")
            .downcast_ref::<SewingError>(),
        Some(&expected)
    );
    assert!(!issue
        .source()
        .expect("concrete graph cause")
        .is::<Box<SewingError>>());
}

#[test]
fn sewing_current_sources_refuse_an_unused_landmarks_missing_actual_piece_before_graph_lookup() {
    use sc_core::design::{DesignSewing, DesignSewingError};
    let mut definition = sewing_owner_definition();
    let mut root = definition.inputs.definition().clone();
    let mut structure = root.structure.definition().clone();
    structure.pieces = PieceInventory::new(
        structure
            .pieces
            .pieces()
            .iter()
            .filter(|piece| piece.id() != id(95000))
            .cloned()
            .collect(),
    )
    .expect("explicit remaining Pieces");
    structure.cut_plan = CutPlan::new(
        structure
            .cut_plan
            .copies()
            .iter()
            .filter(|copy| copy.piece() != id(95000))
            .map(|copy| *copy.definition())
            .collect(),
        structure.pieces.pieces(),
    )
    .expect("complete remaining copies");
    root.structure = DesignStructure::new(structure).expect("actual reduced structure");
    definition.inputs = DesignInputs::new(root).expect("actual remaining input scope");
    let state = DesignSewing::new(definition).expect("inspectable old semantic references");
    assert_eq!(
        state.validate_current(),
        Err(DesignSewingError::MissingLandmarkPiece {
            landmark: id(98000),
            piece: id(95000)
        })
    );
}

#[test]
fn sewing_current_sources_preserve_the_actual_amount_declaration_error_after_graph_validation() {
    use sc_core::{
        design::{
            DesignSewing, DesignSewingDeclarationError, DesignSewingDefinition, DesignSewingError,
        },
        ontology::EaseAmount,
        value::ScalarInputKind,
    };
    use std::error::Error;
    let mut definition = fixture();
    definition.parameters = DesignParameters::new(
        vec![DesignParameterDefinition {
            id: id(5002),
            token: MachineToken::new("sewing_count").expect("actual authored name"),
            target: DesignParameterTarget::Scalar {
                declaration: id(2000),
                kind: ScalarInputKind::Count,
            },
        }],
        definition.structure.inputs(),
    )
    .expect("valid Count parameter metadata");
    let (inputs, graph) = with_sewing_graph(definition, vec![EaseAmount::Formula(id(5002))]);
    let state = DesignSewing::new(DesignSewingDefinition {
        inputs,
        graph,
        landmarks: vec![],
    })
    .expect("owned original sources");
    let expected = DesignSewingDeclarationError::<DesignSourcesError>::FormulaKind {
        span: id(97000),
        parameter: id(5002),
        actual: sc_core::recipe::FormulaKind::Count,
    };
    let issue = state
        .validate_current()
        .expect_err("current graph does not turn Count into Length");
    assert_eq!(issue, DesignSewingError::Amount(Box::new(expected.clone())));
    assert_eq!(
        issue
            .source()
            .expect("original amount metadata cause")
            .downcast_ref::<DesignSewingDeclarationError<DesignSourcesError>>(),
        Some(&expected)
    );
}

fn with_closures(
    definition: DesignInputsDefinition<DesignSources>,
    kinds: Vec<sc_core::ontology::ClosureKind>,
) -> (
    DesignInputs<DesignSources>,
    sc_core::ontology::ClosureInventory,
) {
    use sc_core::ontology::{
        Closure, ClosureDefinition, ClosureInstance, ClosureInventory, DirectedRange, Direction,
        EdgeAnchor, EdgeRange, NotionPlacement, NotionPlacementDefinition, Param,
    };
    let (state, _) = with_sewing_graph(definition, vec![]);
    let structure = state.structure();
    let pieces = structure.pieces().pieces();
    let placements = pieces
        .iter()
        .zip([99000, 99001])
        .map(|(piece, identity)| {
            let edge = piece.edges().next().expect("actual held boundary").edge;
            let copy = structure
                .cut_plan()
                .copies_for_piece(piece.id())
                .next()
                .expect("actual physical copy");
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
            .expect("actual born placement")
        })
        .collect::<Vec<_>>();
    let closures = kinds
        .into_iter()
        .enumerate()
        .map(|(index, kind)| {
            let ordinal = u128::try_from(index).expect("authored fixture ordinal");
            Closure::new(
                ClosureDefinition {
                    id: id(100000 + ordinal),
                    kind,
                    instances: vec![ClosureInstance {
                        id: id(101000 + ordinal),
                        first: id(99000),
                        second: id(99001),
                    }],
                },
                &placements,
                structure.cut_plan(),
                pieces,
                structure.topology(),
            )
            .expect("actual Closure")
        })
        .collect();
    (
        state,
        ClosureInventory::new(closures).expect("actual ordered Closure inventory"),
    )
}

#[test]
fn closure_declarations_borrow_original_roles_and_both_length_families_without_value_reads() {
    use sc_core::{
        design::{DesignClosureDeclarationRole as Role, DesignClosureDeclarationSource as Source},
        ontology::{
            ButtonholeDerivation, ButtonholeValidation, ClosureKind, ClosureLength, NotionSize,
            ProfileBindingValidation, ProfileParameterRef,
        },
        recipe::{FormulaLengthInput, FormulaScalarInput},
        value::{
            ScalarInputDeclaration, ScalarInputDeclarationDefinition, ScalarInputKind,
            ScalarInputState,
        },
    };
    let mut definition = fixture();
    let mut structure = definition.structure.definition().clone();
    let mut scalars = structure.inputs.scalars().to_vec();
    scalars.push(
        ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
            id: id(2003),
            source: id(2005),
            kind: ScalarInputKind::Length,
            state: ScalarInputState::Unknown {
                observation: id(2004),
            },
        })
        .expect("actual unknown general Length"),
    );
    structure.inputs = CanonicalInputs::new(structure.inputs.lengths().to_vec(), scalars)
        .expect("single canonical owner");
    definition.structure = DesignStructure::new(structure).expect("actual replacement structure");
    definition.parameters = DesignParameters::new(
        vec![
            DesignParameterDefinition {
                id: id(5001),
                token: MachineToken::new("zip_length").expect("authored token"),
                target: DesignParameterTarget::Length(id(12)),
            },
            DesignParameterDefinition {
                id: id(5002),
                token: MachineToken::new("zip_general").expect("authored token"),
                target: DesignParameterTarget::Scalar {
                    declaration: id(2003),
                    kind: ScalarInputKind::Length,
                },
            },
        ],
        definition.structure.inputs(),
    )
    .expect("actual derived/unknown parameter metadata");
    let (state, closures) = with_closures(
        definition,
        vec![
            ClosureKind::CentredZipper {
                length: ClosureLength::Explicit {
                    parameter: id(102000),
                    value: sc_units::Length::from_micrometres(180000).expect("positive intent"),
                },
            },
            ClosureKind::CentredZipper {
                length: ClosureLength::Formula(id(5001)),
            },
            ClosureKind::CentredZipper {
                length: ClosureLength::Formula(id(5002)),
            },
            ClosureKind::CentredZipper {
                length: ClosureLength::Profile(ProfileParameterRef::new(id(10))),
            },
            ClosureKind::HookAndBar {
                hook: NotionSize::Declaration(id(2000)),
                bar: NotionSize::Profile(ProfileParameterRef::new(id(10))),
            },
            ClosureKind::ButtonAndButtonhole {
                button: NotionSize::Profile(ProfileParameterRef::new(id(10))),
                hole: ButtonholeDerivation {
                    operation: id(2000),
                },
            },
        ],
    );
    let original = (state.inputs().clone(), closures.clone());
    let views = state
        .closure_declarations(&closures)
        .expect("all actual declaration roles");
    assert_eq!(
        views.iter().map(|view| view.role()).collect::<Vec<_>>(),
        vec![
            Role::ZipperLength,
            Role::ZipperLength,
            Role::ZipperLength,
            Role::ZipperLength,
            Role::HookSize,
            Role::BarSize,
            Role::ButtonSize,
            Role::Buttonhole,
        ]
    );
    for view in &views {
        assert!(core::ptr::eq(view.canonical_inputs(), state.inputs()));
        assert!(core::ptr::eq(
            view.closure(),
            closures
                .closure(view.closure().id())
                .expect("original owner")
        ));
        match view.source() {
            Source::ExplicitLength { parameter, value } => {
                let ClosureKind::CentredZipper {
                    length:
                        ClosureLength::Explicit {
                            parameter: actual_id,
                            value: actual_value,
                        },
                } = &view.closure().definition().kind
                else {
                    unreachable!()
                };
                assert!(core::ptr::eq(parameter, actual_id));
                assert!(core::ptr::eq(value, actual_value));
                assert_eq!(value.as_micrometres(), 180000);
                assert!(state.parameters().parameter(*parameter).is_none());
            }
            Source::FormulaLength {
                parameter,
                declaration,
            } => {
                assert!(core::ptr::eq(
                    parameter,
                    state
                        .parameters()
                        .parameter(parameter.id())
                        .expect("actual metadata")
                ));
                if let Some(length) = FormulaLengthInput::from_declaration(declaration) {
                    assert!(core::ptr::eq(
                        length.canonical_declaration(),
                        state
                            .inputs()
                            .length(id(12))
                            .expect("actual derived source")
                    ));
                    assert!(length.read_authored().is_err());
                } else {
                    let canonical = state
                        .inputs()
                        .scalar(id(2003))
                        .expect("actual unknown general source");
                    let scalar = FormulaScalarInput::from_declaration(declaration, canonical)
                        .expect("actual general Length");
                    assert!(core::ptr::eq(scalar.canonical_declaration(), canonical));
                    assert!(scalar.read_authored().is_err());
                }
            }
            Source::ProfileLength(reference) => {
                let ClosureKind::CentredZipper {
                    length: ClosureLength::Profile(actual),
                } = &view.closure().definition().kind
                else {
                    unreachable!()
                };
                assert!(core::ptr::eq(reference, actual));
                assert_eq!(
                    view.profile_validation(),
                    Some(ProfileBindingValidation::DeferredToG4)
                );
            }
            Source::Size(size) => {
                let actual = match (&view.closure().definition().kind, view.role()) {
                    (ClosureKind::HookAndBar { hook, .. }, Role::HookSize) => hook,
                    (ClosureKind::HookAndBar { bar, .. }, Role::BarSize) => bar,
                    (ClosureKind::ButtonAndButtonhole { button, .. }, Role::ButtonSize) => button,
                    _ => unreachable!(),
                };
                assert!(core::ptr::eq(size, actual));
                assert_eq!(
                    view.profile_validation(),
                    if matches!(size, NotionSize::Profile(_)) {
                        Some(ProfileBindingValidation::DeferredToG4)
                    } else {
                        None
                    }
                );
            }
            Source::Buttonhole(source) => {
                let ClosureKind::ButtonAndButtonhole { button, hole } =
                    &view.closure().definition().kind
                else {
                    unreachable!()
                };
                assert!(core::ptr::eq(source.button_size(), button));
                assert!(core::ptr::eq(source.derivation(), hole));
                assert_eq!(source.derivation().operation, id(2000));
                assert_eq!(
                    view.buttonhole_validation(),
                    Some(ButtonholeValidation::DeferredToG3)
                );
                assert_eq!(
                    view.profile_validation(),
                    Some(ProfileBindingValidation::DeferredToG4)
                );
            }
        }
    }
    assert_eq!(state.inputs(), &original.0);
    assert_eq!(closures, original.1);
}

#[test]
fn closure_declarations_require_actual_formula_metadata_despite_numeric_canonical_presence() {
    use sc_core::{
        design::DesignClosureDeclarationError as Error,
        ontology::{ClosureKind, ClosureLength},
    };
    for numeric_only in [10, 2000] {
        let (state, closures) = with_closures(
            fixture(),
            vec![ClosureKind::CentredZipper {
                length: ClosureLength::Formula(id(numeric_only)),
            }],
        );
        assert_eq!(
            state
                .closure_declarations(&closures)
                .expect_err("canonical id is no parameter metadata"),
            Error::MissingFormulaParameter {
                closure: id(100000),
                parameter: id(numeric_only)
            }
        );
    }
}

#[test]
fn closure_declarations_keep_the_later_actual_closure_when_formula_kind_is_wrong() {
    use sc_core::{
        design::DesignClosureDeclarationError as Error,
        ontology::{ClosureKind, ClosureLength},
        value::ScalarInputKind,
    };
    let mut definition = fixture();
    let mut parameters = definition
        .parameters
        .parameters()
        .iter()
        .map(|parameter| parameter.definition().clone())
        .collect::<Vec<_>>();
    parameters.push(DesignParameterDefinition {
        id: id(5002),
        token: MachineToken::new("count_intent").expect("authored token"),
        target: DesignParameterTarget::Scalar {
            declaration: id(2000),
            kind: ScalarInputKind::Count,
        },
    });
    definition.parameters = DesignParameters::new(parameters, definition.structure.inputs())
        .expect("actual Count metadata");
    let (state, closures) = with_closures(
        definition,
        vec![
            ClosureKind::CentredZipper {
                length: ClosureLength::Formula(id(5001)),
            },
            ClosureKind::CentredZipper {
                length: ClosureLength::Formula(id(5002)),
            },
        ],
    );
    assert_eq!(
        state
            .closure_declarations(&closures)
            .expect_err("Count cannot satisfy zipper Length"),
        Error::FormulaKind {
            closure: id(100001),
            parameter: id(5002),
            actual: sc_core::recipe::FormulaKind::Count
        }
    );
}

#[test]
fn closure_declarations_are_metadata_scope_and_do_not_approve_missing_current_placements() {
    use sc_core::ontology::{
        ClosureInventoryError, ClosureKind, ClosureLength, NotionPlacementInventory,
    };
    let (state, closures) = with_closures(
        fixture(),
        vec![ClosureKind::CentredZipper {
            length: ClosureLength::Formula(id(5001)),
        }],
    );
    let mut definition = state.definition().clone();
    let mut structure = definition.structure.definition().clone();
    structure.pieces = PieceInventory::new(vec![]).expect("explicit current empty Pieces");
    structure.cut_plan = CutPlan::new(vec![], &[]).expect("explicit current empty copies");
    definition.structure =
        DesignStructure::new(structure).expect("actual current structural owner");
    let current = DesignInputs::new(definition).expect("actual unchanged canonical metadata");
    let views = current
        .closure_declarations(&closures)
        .expect("metadata remains inspectable");
    assert_eq!(views.len(), 1);
    assert_eq!(
        views.first().expect("original Closure role").closure().id(),
        id(100000)
    );
    assert!(matches!(
        closures.validate_current(
            &NotionPlacementInventory::new(vec![]).expect("empty notions"),
            current.structure().cut_plan(),
            current.structure().pieces().pieces(),
            current.structure().topology()
        ),
        Err(ClosureInventoryError::Closure { .. })
    ));
}

fn with_hems(
    definition: DesignInputsDefinition<DesignSources>,
    fields: Vec<(sc_core::ontology::HemDepth, sc_core::ontology::HemFoldType)>,
) -> (DesignInputs<DesignSources>, sc_core::ontology::HemInventory) {
    use sc_core::ontology::{Hem, HemDefinition, HemInventory, HemMethod};
    let (state, _) = with_sewing_graph(definition, vec![]);
    let structure = state.structure();
    let piece = structure
        .pieces()
        .pieces()
        .first()
        .expect("actual served Piece");
    let edge = piece.edges().next().expect("actual finish edge").edge;
    let hems = fields
        .into_iter()
        .enumerate()
        .map(|(index, (depth, fold_type))| {
            Hem::new(
                HemDefinition {
                    id: id(103000 + u128::try_from(index).expect("authored fixture ordinal")),
                    edge,
                    depth,
                    fold_type,
                    method: HemMethod::Turned,
                },
                piece,
                structure.topology(),
                None,
            )
            .expect("actual born Hem")
        })
        .collect();
    (state, HemInventory::new(hems).expect("actual ordered Hems"))
}

#[test]
fn hem_declarations_borrow_original_depth_fold_roles_and_unread_length_sources() {
    use sc_core::{
        design::{DesignHemDeclarationRole as Role, DesignHemDeclarationSource as Source},
        ontology::{HemDepth, HemFoldType, ProfileBindingValidation, ProfileParameterRef},
        recipe::{FormulaLengthInput, FormulaScalarInput},
        value::{
            ScalarInputDeclaration, ScalarInputDeclarationDefinition, ScalarInputKind,
            ScalarInputState,
        },
    };
    let mut definition = fixture();
    let mut structure = definition.structure.definition().clone();
    let mut scalars = structure.inputs.scalars().to_vec();
    scalars.push(
        ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
            id: id(2003),
            source: id(2005),
            kind: ScalarInputKind::Length,
            state: ScalarInputState::Unknown {
                observation: id(2004),
            },
        })
        .expect("actual unknown general Length"),
    );
    structure.inputs = CanonicalInputs::new(structure.inputs.lengths().to_vec(), scalars)
        .expect("single canonical owner");
    definition.structure = DesignStructure::new(structure).expect("actual current structure");
    definition.parameters = DesignParameters::new(
        vec![
            DesignParameterDefinition {
                id: id(5001),
                token: MachineToken::new("hem_length").expect("authored token"),
                target: DesignParameterTarget::Length(id(12)),
            },
            DesignParameterDefinition {
                id: id(5002),
                token: MachineToken::new("hem_general").expect("authored token"),
                target: DesignParameterTarget::Scalar {
                    declaration: id(2003),
                    kind: ScalarInputKind::Length,
                },
            },
        ],
        definition.structure.inputs(),
    )
    .expect("actual derived/unknown parameter metadata");
    let (state, hems) = with_hems(
        definition,
        vec![
            (
                HemDepth::Explicit {
                    parameter: id(104000),
                    value: sc_units::Length::ZERO,
                },
                HemFoldType::Declaration(id(2000)),
            ),
            (
                HemDepth::Formula(id(5001)),
                HemFoldType::Declaration(id(2000)),
            ),
            (
                HemDepth::Formula(id(5002)),
                HemFoldType::Profile(ProfileParameterRef::new(id(10))),
            ),
            (
                HemDepth::Profile(ProfileParameterRef::new(id(10))),
                HemFoldType::Profile(ProfileParameterRef::new(id(10))),
            ),
        ],
    );
    let original = (state.inputs().clone(), hems.clone());
    let views = state
        .hem_declarations(&hems)
        .expect("actual field roles without value reads");
    assert_eq!(
        views.iter().map(|view| view.role()).collect::<Vec<_>>(),
        vec![
            Role::Depth,
            Role::FoldType,
            Role::Depth,
            Role::FoldType,
            Role::Depth,
            Role::FoldType,
            Role::Depth,
            Role::FoldType
        ]
    );
    for view in views {
        assert!(core::ptr::eq(
            view.hem(),
            hems.hem(view.hem().id()).expect("original Hem")
        ));
        assert!(core::ptr::eq(view.canonical_inputs(), state.inputs()));
        match view.source() {
            Source::ExplicitDepth { parameter, value } => {
                let HemDepth::Explicit {
                    parameter: actual_id,
                    value: actual_value,
                } = &view.hem().definition().depth
                else {
                    unreachable!()
                };
                assert!(core::ptr::eq(parameter, actual_id));
                assert!(core::ptr::eq(value, actual_value));
                assert_eq!(*value, sc_units::Length::ZERO);
                assert!(state.parameters().parameter(*parameter).is_none());
                assert_eq!(view.profile_validation(), None);
            }
            Source::FormulaDepth {
                parameter,
                declaration,
            } => {
                assert!(core::ptr::eq(
                    parameter,
                    state
                        .parameters()
                        .parameter(parameter.id())
                        .expect("actual metadata")
                ));
                if let Some(length) = FormulaLengthInput::from_declaration(declaration) {
                    assert!(core::ptr::eq(
                        length.canonical_declaration(),
                        state.inputs().length(id(12)).expect("actual derived input")
                    ));
                    assert!(length.read_authored().is_err());
                } else {
                    let canonical = state
                        .inputs()
                        .scalar(id(2003))
                        .expect("actual unknown general input");
                    let scalar = FormulaScalarInput::from_declaration(declaration, canonical)
                        .expect("current general Length");
                    assert!(core::ptr::eq(scalar.canonical_declaration(), canonical));
                    assert!(scalar.read_authored().is_err());
                }
                assert_eq!(view.profile_validation(), None);
            }
            Source::ProfileDepth(reference) => {
                let HemDepth::Profile(actual) = &view.hem().definition().depth else {
                    unreachable!()
                };
                assert!(core::ptr::eq(reference, actual));
                assert_eq!(
                    view.profile_validation(),
                    Some(ProfileBindingValidation::DeferredToG4)
                );
            }
            Source::FoldType(fold) => {
                assert!(core::ptr::eq(fold, &view.hem().definition().fold_type));
                assert_eq!(
                    view.profile_validation(),
                    if matches!(fold, HemFoldType::Profile(_)) {
                        Some(ProfileBindingValidation::DeferredToG4)
                    } else {
                        None
                    }
                );
            }
        }
    }
    assert_eq!((state.inputs(), &hems), (&original.0, &original.1));
}

#[test]
fn hem_declarations_require_actual_parameter_metadata_despite_numeric_presence() {
    use sc_core::{
        design::DesignHemDeclarationError as Error,
        ontology::{HemDepth, HemFoldType},
    };
    for numeric_only in [10, 2000] {
        let (state, hems) = with_hems(
            fixture(),
            vec![(
                HemDepth::Formula(id(numeric_only)),
                HemFoldType::Declaration(id(2000)),
            )],
        );
        assert_eq!(
            state
                .hem_declarations(&hems)
                .expect_err("canonical id supplies no parameter metadata"),
            Error::MissingFormulaParameter {
                hem: id(103000),
                parameter: id(numeric_only)
            }
        );
    }
}

#[test]
fn hem_declarations_keep_the_later_actual_hem_when_formula_kind_is_wrong() {
    use sc_core::{
        design::DesignHemDeclarationError as Error,
        ontology::{HemDepth, HemFoldType},
        value::ScalarInputKind,
    };
    let mut definition = fixture();
    let mut parameters = definition
        .parameters
        .parameters()
        .iter()
        .map(|parameter| parameter.definition().clone())
        .collect::<Vec<_>>();
    parameters.push(DesignParameterDefinition {
        id: id(5002),
        token: MachineToken::new("hem_count").expect("authored token"),
        target: DesignParameterTarget::Scalar {
            declaration: id(2000),
            kind: ScalarInputKind::Count,
        },
    });
    definition.parameters = DesignParameters::new(parameters, definition.structure.inputs())
        .expect("actual Count parameter");
    let (state, hems) = with_hems(
        definition,
        vec![
            (
                HemDepth::Formula(id(5001)),
                HemFoldType::Declaration(id(2000)),
            ),
            (
                HemDepth::Formula(id(5002)),
                HemFoldType::Declaration(id(2000)),
            ),
        ],
    );
    assert_eq!(
        state
            .hem_declarations(&hems)
            .expect_err("Count cannot satisfy depth Length"),
        Error::FormulaKind {
            hem: id(103001),
            parameter: id(5002),
            actual: sc_core::recipe::FormulaKind::Count
        }
    );
}

#[test]
fn hem_declarations_are_metadata_scope_and_do_not_approve_missing_current_pieces() {
    use sc_core::ontology::{
        HemDepth, HemFoldType, HemInventory, HemInventoryError, LayerInventory,
    };
    let (state, hems) = with_hems(
        fixture(),
        vec![(
            HemDepth::Formula(id(5001)),
            HemFoldType::Declaration(id(2000)),
        )],
    );
    let mut definition = state.definition().clone();
    let mut structure = definition.structure.definition().clone();
    structure.pieces = PieceInventory::new(vec![]).expect("explicit current empty Pieces");
    structure.cut_plan = CutPlan::new(vec![], &[]).expect("explicit current empty copies");
    definition.structure = DesignStructure::new(structure).expect("actual current structure");
    let current = DesignInputs::new(definition).expect("actual unchanged canonical metadata");
    let views = current
        .hem_declarations(&hems)
        .expect("metadata stays inspectable");
    assert_eq!(views.len(), 2);
    assert_eq!(views.first().expect("original role").hem().id(), id(103000));
    assert!(matches!(
        hems.validate_current(
            &LayerInventory::new(vec![], vec![], vec![]).expect("empty layers"),
            current.structure().pieces().pieces(),
            current.structure().topology()
        ),
        Err(HemInventoryError::MissingPiece { .. })
    ));
    assert!(current
        .hem_declarations(&HemInventory::new(vec![]).expect("empty Hems"))
        .expect("explicit empty metadata scope")
        .is_empty());
}

fn with_layers() -> (
    DesignInputs<DesignSources>,
    sc_core::ontology::LayerInventory,
) {
    use sc_core::ontology::{
        DirectedRange, Direction, EdgeRange, Facing, Interfacing, LayerDefinition, LayerInventory,
        LayerOffsetRelationship, Lining, MaterialAssignment, Param, Rational,
    };
    let (state, _) = with_sewing_graph(fixture(), vec![]);
    let structure = state.structure();
    let piece = structure
        .pieces()
        .pieces()
        .first()
        .expect("actual served Piece");
    let edge = piece.edges().next().expect("actual held edge").edge;
    let middle = Param::new(Rational::new(1, 2).expect("exact midpoint")).expect("inside edge");
    let layer = |identity, operation| LayerDefinition {
        id: id(identity),
        served_piece: piece.id(),
        offset: LayerOffsetRelationship {
            operation: id(operation),
            sources: vec![
                DirectedRange {
                    range: EdgeRange::new(edge, middle, Param::END).expect("authored second half"),
                    direction: Direction::Reversed,
                },
                DirectedRange {
                    range: EdgeRange::new(edge, Param::START, middle).expect("authored first half"),
                    direction: Direction::Original,
                },
            ],
        },
        material: MaterialAssignment::Unresolved("private layer selection".into()),
    };
    let layers = LayerInventory::new(
        vec![
            Facing::new(layer(104003, 10), piece, structure.topology()).expect("actual Facing"),
            Facing::new(layer(104001, 2000), piece, structure.topology()).expect("later Facing"),
        ],
        vec![
            Lining::new(layer(104004, 5001), piece, structure.topology()).expect("modelled Lining"),
        ],
        vec![
            Interfacing::new(layer(104000, 108000), piece, structure.topology())
                .expect("actual Interfacing"),
        ],
    )
    .expect("distinct actual layer owners");
    (state, layers)
}

#[test]
fn layer_offset_declarations_keep_original_types_order_and_relationships_without_numeric_coercion()
{
    use sc_core::ontology::{Direction, LayerKind, LayerRef};
    let (state, layers) = with_layers();
    let views = state
        .layer_offset_declarations(&layers)
        .expect("current root metadata");
    assert_eq!(views.len(), 4);
    assert!(
        matches!(views.first().expect("first").layer(), LayerRef::Facing(value)
        if core::ptr::eq(value, layers.facings().first().expect("actual first Facing")))
    );
    assert!(
        matches!(views.get(2).expect("third").layer(), LayerRef::Lining(value)
        if core::ptr::eq(value, layers.linings().first().expect("actual Lining")))
    );
    assert!(
        matches!(views.last().expect("last").layer(), LayerRef::Interfacing(value)
        if core::ptr::eq(value, layers.interfacings().first().expect("actual Interfacing")))
    );
    assert_eq!(
        views
            .iter()
            .map(|view| view.layer().kind())
            .collect::<Vec<_>>(),
        vec![
            LayerKind::Facing,
            LayerKind::Facing,
            LayerKind::Lining,
            LayerKind::Interfacing
        ]
    );
    for ((_, layer), view) in layers.entries().zip(&views) {
        assert!(core::ptr::eq(view.canonical_inputs(), state.inputs()));
        assert!(core::ptr::eq(
            view.relationship(),
            &layer.definition().offset
        ));
        assert_eq!(view.layer(), layer);
        assert_eq!(
            view.relationship()
                .sources
                .iter()
                .map(|source| source.direction)
                .collect::<Vec<_>>(),
            vec![Direction::Reversed, Direction::Original]
        );
        let debug = format!("{view:?}");
        assert!(!debug.contains("private layer selection"));
        assert!(!debug.contains("operation"));
    }
    assert_eq!(
        views
            .iter()
            .map(|view| view.relationship().operation)
            .collect::<Vec<_>>(),
        [10, 2000, 5001, 108000].map(id)
    );
    assert!(state.inputs().length(id(10)).is_some());
    assert!(state.inputs().scalar(id(2000)).is_some());
    assert!(state.parameters().parameter(id(5001)).is_some());
    // Those same-number input/metadata records cannot supply an operation kind or execution result.
    assert_eq!(state.inputs(), fixture().structure.inputs());
}

#[test]
fn layer_offset_declarations_leave_modelled_lining_execution_to_its_named_scope_check() {
    use sc_core::ontology::LayerEnvelopeError;
    let (state, layers) = with_layers();
    let views = state
        .layer_offset_declarations(&layers)
        .expect("Lining remains inspectable");
    for (index, view) in views.iter().enumerate() {
        if index == 2 {
            let issue = view
                .layer()
                .require_in_scope()
                .expect_err("modelled Lining is outside v1 execution");
            assert_eq!(
                issue,
                LayerEnvelopeError::LiningDeferred {
                    served_piece: id(95000)
                }
            );
            assert_eq!(issue.diagnostic(), "env_lining");
            assert_eq!(issue.proving_gate(), "G7");
        } else {
            assert_eq!(view.layer().require_in_scope(), Ok(()));
        }
    }
}

#[test]
fn layer_offset_declarations_do_not_certify_missing_current_served_pieces_or_infer_empty_content() {
    use sc_core::ontology::{LayerInventory, LayerInventoryError};
    let (state, layers) = with_layers();
    let mut definition = state.definition().clone();
    let mut structure = definition.structure.definition().clone();
    structure.pieces = PieceInventory::new(vec![]).expect("explicit current empty Pieces");
    structure.cut_plan = CutPlan::new(vec![], &[]).expect("explicit current empty copies");
    definition.structure = DesignStructure::new(structure).expect("actual current structure");
    let current = DesignInputs::new(definition).expect("actual unchanged canonical metadata");
    let views = current
        .layer_offset_declarations(&layers)
        .expect("original relationship stays inspectable");
    assert_eq!(views.len(), 4);
    assert!(core::ptr::eq(
        views.first().expect("original first").relationship(),
        &layers
            .facings()
            .first()
            .expect("actual original Facing")
            .definition()
            .offset
    ));
    assert!(
        matches!(layers.validate_current(current.structure().pieces().pieces(), current.structure().topology()),
        Err(LayerInventoryError::MissingServedPiece { piece, .. }) if piece == id(95000))
    );
    assert!(current
        .layer_offset_declarations(
            &LayerInventory::new(vec![], vec![], vec![]).expect("empty layers")
        )
        .expect("explicit empty metadata scope")
        .is_empty());
}

#[test]
fn construction_and_garment_ownership_rechecks_whole_root_before_empty_scope() {
    use core::cell::Cell;
    use sc_core::{
        design::{DesignMetadataHost, DesignReferenceHost, DesignSizeSetHost},
        ontology::LayerInventory,
        value::LengthDeclaration,
    };
    // A controlled port-contract violation exercises the public core's current-snapshot refusal.
    #[derive(Clone)]
    struct ChangingHost {
        actual: DesignSources,
        changed: Cell<bool>,
    }
    impl DesignMetadataHost for ChangingHost {
        type Error = DesignSourcesError;
        fn entities(&self) -> Vec<DesignHostEntity> {
            let mut entities = self.actual.entities();
            if self.changed.get() {
                entities.push(DesignHostEntity {
                    id: id(109000),
                    kind: DesignHostEntityKind::Ease,
                    index: 0,
                });
            }
            entities
        }
        fn validate_current(&self, canonical: &CanonicalInputs) -> Result<(), Self::Error> {
            self.actual.validate_current(canonical)
        }
        fn measurement_declaration<'a>(
            &'a self,
            measurement: sc_core::ontology::EntityId,
            canonical: &'a CanonicalInputs,
        ) -> Result<&'a LengthDeclaration, Self::Error> {
            self.actual.measurement_declaration(measurement, canonical)
        }
    }
    impl DesignSizeSetHost for ChangingHost {
        fn validate_size_set(
            &self,
            reference: DesignSizeSetReference,
            canonical: &CanonicalInputs,
        ) -> Result<(), Self::Error> {
            self.actual.validate_size_set(reference, canonical)
        }
    }
    impl DesignReferenceHost for ChangingHost {
        fn validate_references(
            &self,
            references: DesignSourceReferences,
            canonical: &CanonicalInputs,
        ) -> Result<(), Self::Error> {
            self.actual.validate_references(references, canonical)
        }
    }
    let definition = fixture();
    let sources = DesignMetadata::new(
        ChangingHost {
            actual: definition.sources.host().clone(),
            changed: Cell::new(false),
        },
        definition.structure.inputs(),
    )
    .expect("actual stable metadata snapshot");
    let state = DesignInputs::new(DesignInputsDefinition {
        structure: definition.structure,
        parameters: definition.parameters,
        recipe: definition.recipe,
        materials: definition.materials,
        sources,
        references: definition.references,
    })
    .expect("actual valid root before controlled port drift");
    let structure = state.structure();
    let graph = sc_core::ontology::SewingGraph::new(
        sc_core::ontology::SewingGraphDefinition {
            id: id(990000),
            spans: vec![],
        },
        structure.cut_plan(),
        structure.pieces().pieces(),
        &[],
        structure.topology(),
    )
    .expect("actual empty current sewing context");
    let sewing = sc_core::design::DesignSewing::new(sc_core::design::DesignSewingDefinition {
        inputs: state,
        graph,
        landmarks: vec![],
    })
    .expect("actual current sewing/root owner");
    let garment = sc_core::design::DesignGarment::new(sc_core::design::DesignGarmentDefinition {
        sewing,
        pockets: sc_core::ontology::PocketInventory::new(vec![]).expect("empty pockets"),
        notions: sc_core::ontology::NotionPlacementInventory::new(vec![]).expect("empty notions"),
        closures: sc_core::ontology::ClosureInventory::new(vec![]).expect("empty closures"),
        layers: LayerInventory::new(vec![], vec![], vec![]).expect("empty layers"),
        hems: sc_core::ontology::HemInventory::new(vec![]).expect("empty Hems"),
        constructions: sc_core::ontology::ConstructionInventory::new(
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        )
        .expect("empty constructions"),
    })
    .expect("whole immutable owner before controlled drift");
    assert!(garment.validate_current().is_ok());
    assert_eq!(garment.require_in_scope(), Ok(()));
    assert!(garment
        .declarations()
        .expect("whole current empty garment declarations")
        .is_empty());
    let sewing = garment.sewing();
    let state = sewing.inputs();
    let layers = LayerInventory::new(vec![], vec![], vec![]).expect("explicit empty layers");
    assert!(state
        .layer_offset_declarations(&layers)
        .expect("stable snapshot")
        .is_empty());
    let constructions = sc_core::ontology::ConstructionInventory::new(
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    )
    .expect("explicit empty constructions");
    assert!(state
        .construction_intake_declarations(&constructions)
        .expect("stable root")
        .is_empty());
    assert!(state
        .grain_allowance_declarations(&constructions)
        .expect("stable grain/allowance root")
        .is_empty());
    assert!(sewing
        .gather_declarations(&constructions)
        .expect("stable whole context")
        .is_empty());
    assert!(state
        .pocket_opening_declarations(garment.pockets())
        .expect("stable whole root")
        .is_empty());
    state.sources().host().changed.set(true);
    let pocket_error = state
        .pocket_opening_declarations(garment.pockets())
        .expect_err("whole root before empty opening list");
    assert!(
        matches!(&pocket_error, DesignInputsError::Sources(source) if matches!(source.as_ref(), DesignMetadataError::ChangedEntities))
    );
    assert!(std::error::Error::source(&pocket_error)
        .expect("original metadata cause")
        .is::<DesignMetadataError<DesignSourcesError>>());
    let admission = sc_core::design::DesignGarment::new(garment.definition().clone())
        .expect_err("current whole root admission");
    assert!(
        matches!(&admission, sc_core::design::DesignGarmentError::Inputs(issue)
        if matches!(issue.as_ref(), DesignInputsError::Sources(source)
            if matches!(source.as_ref(), DesignMetadataError::ChangedEntities)))
    );
    assert!(std::error::Error::source(&admission)
        .expect("original current inputs cause")
        .is::<DesignInputsError<DesignSourcesError>>());
    assert_eq!(
        garment
            .validate_current()
            .expect_err("recheck whole current root"),
        admission
    );
    // Explicit envelope scope is independent of host drift/current-source validation.
    assert_eq!(garment.require_in_scope(), Ok(()));
    let declaration_error = garment
        .declarations()
        .expect_err("whole garment preflight before empty views");
    assert_eq!(
        declaration_error,
        sc_core::design::DesignGarmentDeclarationError::Garment(Box::new(admission.clone()))
    );
    let mut collision = garment.definition().clone();
    collision.pockets = garment_ownership::pocket_with_id(id(5000));
    assert_eq!(
        sc_core::design::DesignGarment::new(collision)
            .expect_err("ownership precedes current root drift"),
        sc_core::design::DesignGarmentError::DuplicateIdentity {
            identity: id(5000),
            first: sc_core::design::DesignGarmentPosition::Sewing(
                sc_core::design::DesignSewingPosition::Inputs(DesignInputsPosition::Structure(
                    DesignStructurePosition::Design
                ))
            ),
            duplicate: sc_core::design::DesignGarmentPosition::Pocket { index: 0 },
        }
    );
    let gather_error = sewing
        .gather_declarations(&constructions)
        .expect_err("whole root before empty Gather");
    assert!(
        matches!(&gather_error, sc_core::design::DesignGatherDeclarationError::Sewing(issue)
        if matches!(issue.as_ref(), sc_core::design::DesignSewingError::Inputs(inputs)
            if matches!(inputs.as_ref(), DesignInputsError::Sources(source)
                if matches!(source.as_ref(), DesignMetadataError::ChangedEntities))))
    );
    let grain_error = state
        .grain_allowance_declarations(&constructions)
        .expect_err("whole root before empty grain/allowance");
    assert!(
        matches!(&grain_error, sc_core::design::DesignConstructionDeclarationError::Inputs(issue)
        if matches!(issue.as_ref(), DesignInputsError::Sources(source) if matches!(source.as_ref(), DesignMetadataError::ChangedEntities)))
    );
    let error = state
        .construction_intake_declarations(&constructions)
        .expect_err("whole current root first");
    assert!(
        matches!(&error, sc_core::design::DesignConstructionDeclarationError::Inputs(issue)
        if matches!(issue.as_ref(), DesignInputsError::Sources(source) if matches!(source.as_ref(), DesignMetadataError::ChangedEntities)))
    );
    assert!(std::error::Error::source(&error)
        .expect("original Inputs cause")
        .is::<DesignInputsError<DesignSourcesError>>());
    assert!(matches!(state.layer_offset_declarations(&layers),
        Err(DesignInputsError::Sources(issue)) if matches!(issue.as_ref(), DesignMetadataError::ChangedEntities)));
}

#[path = "support/construction_intake_declarations.rs"]
mod construction_intake_declarations;

#[path = "support/construction_grain_allowance_declarations.rs"]
mod construction_grain_allowance_declarations;

#[path = "support/construction_gather_declarations.rs"]
mod construction_gather_declarations;

#[path = "support/garment_ownership.rs"]
mod garment_ownership;

#[path = "support/garment_current.rs"]
mod garment_current;

#[path = "support/pocket_opening_declarations.rs"]
mod pocket_opening_declarations;

#[path = "support/garment_declarations.rs"]
mod garment_declarations;

#[path = "support/garment_scope.rs"]
mod garment_scope;
