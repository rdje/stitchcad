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
