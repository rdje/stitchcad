//! Actual numeric Derived association to original checked-source and acyclic owners.
use super::{limits, scope, EmptyHost, Provider, SourceView, TestResult};
use sc_core::{construction::*, design::*, name::MachineToken, ontology::*, recipe::*, value::*};
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn fixture(empty: bool) -> TestResult<DesignGarment<EmptyHost>> {
    let canonical = CanonicalInputs::with_logicals(
        vec![LengthDeclaration::new(LengthDeclarationDefinition {
            id: id(10),
            source: id(1000),
            state: if empty {
                LengthState::Unknown {
                    observation: id(1100),
                }
            } else {
                LengthState::Derived { formula: id(500) }
            },
        })?],
        [
            (20, ScalarInputKind::Length, 500),
            (30, ScalarInputKind::Boolean, 501),
        ]
        .into_iter()
        .map(|(identity, kind, formula)| {
            Ok(ScalarInputDeclaration::new(
                ScalarInputDeclarationDefinition {
                    id: id(identity),
                    source: id(1000 + identity),
                    kind,
                    state: if empty {
                        ScalarInputState::Unknown {
                            observation: id(1100 + identity),
                        }
                    } else {
                        ScalarInputState::Derived {
                            formula: id(formula),
                        }
                    },
                },
            )?)
        })
        .collect::<TestResult<Vec<_>>>()?,
        LogicalInputs::new(
            vec![LogicalInputDeclaration::new(
                LogicalInputDeclarationDefinition {
                    id: id(90),
                    source: id(1090),
                    kind: LogicalInputKind::PocketOpening,
                    domain: vec![MachineToken::new("private_option")?],
                    state: LogicalInputState::Unknown {
                        observation: id(1190),
                    },
                },
                LogicalInputLimits::new(1, 0, 14, 2),
            )?],
            LogicalInputInventoryLimits::new(1, 1),
        )?,
    )?;
    let mut inputs = [
        CanonicalInputTarget::Length(id(10)),
        CanonicalInputTarget::Scalar {
            declaration: id(20),
            kind: ScalarInputKind::Length,
        },
        CanonicalInputTarget::Scalar {
            declaration: id(30),
            kind: ScalarInputKind::Boolean,
        },
        CanonicalInputTarget::Length(id(10)),
    ]
    .into_iter()
    .enumerate()
    .map(|(index, target)| {
        Ok(LogicalDerivationInput {
            role: MachineToken::new(format!("numeric_{index}"))?,
            source: LogicalDerivationInputSource::Numeric(target),
        })
    })
    .collect::<TestResult<Vec<_>>>()?;
    inputs.push(LogicalDerivationInput {
        role: MachineToken::new("logical")?,
        source: LogicalDerivationInputSource::Logical(LogicalInputTarget {
            declaration: id(90),
            kind: LogicalInputKind::PocketOpening,
        }),
    });
    inputs.push(LogicalDerivationInput {
        role: MachineToken::new("profile")?,
        source: LogicalDerivationInputSource::Profile {
            parameter: ProfileParameterRef::new(id(702)),
            kind: LogicalInputKind::PocketOpening,
        },
    });
    let semantic = LogicalDerivationOperation::new(
        LogicalDerivationDefinition {
            id: id(80),
            output: LogicalInputTarget {
                declaration: id(90),
                kind: LogicalInputKind::PocketOpening,
            },
            rule: id(901),
            inputs,
        },
        LogicalDerivationLimits::new(6, 100, 36),
    )?;
    let sources = [(500, "1 mm"), (501, "1==1")]
        .into_iter()
        .map(|(identity, text)| {
            Ok(FormulaSource::new(
                id(identity),
                FormulaExpressionDocument::new(text.into())?,
            ))
        })
        .collect::<TestResult<Vec<_>>>()?;
    let inputs = super::super::root_with_host_and_semantic(
        "",
        sources,
        EmptyHost,
        canonical,
        vec![],
        DesignSourceReferences {
            measurement_table: None,
            ease_set: None,
            size_set: None,
        },
        SemanticRecipe::new(
            vec![SemanticOperation::LogicalDerivation(semantic)],
            SemanticRecipeLimits::new(1, 1),
        )?,
    )?;
    let structure = inputs.structure();
    let graph = SewingGraph::new(
        SewingGraphDefinition {
            id: id(10004),
            spans: vec![],
        },
        structure.cut_plan(),
        structure.pieces().pieces(),
        &[],
        structure.topology(),
    )?;
    let sewing = DesignSewing::new(DesignSewingDefinition {
        inputs,
        graph,
        landmarks: vec![],
    })
    .map_err(|e| e.to_string())?;
    Ok(DesignGarment::new(DesignGarmentDefinition {
        sewing,
        pockets: PocketInventory::new(vec![])?,
        notions: NotionPlacementInventory::new(vec![])?,
        closures: ClosureInventory::new(vec![])?,
        layers: LayerInventory::new(vec![], vec![], vec![])?,
        hems: HemInventory::new(vec![])?,
        constructions: ConstructionInventory::new(vec![], vec![], vec![], vec![], vec![], vec![])?,
    })
    .map_err(|e| e.to_string())?)
}
#[test]
fn root_numeric_suppliers_retain_actual_source_graph_and_representation_without_mixed_positions(
) -> TestResult {
    // Complete repeated/different families,empty numeric Derived,short remaining cursor,
    // and an actual fresh provider resource refusal after prior successful content admission.
    for case in 0..4u8 {
        let garment = fixture(case == 1)?;
        let source = Provider::new(&garment);
        let source_view: &SourceView<'_> = &source;
        let content = limits(100000, 100000, 100000)?;
        let context = scope(garment.inputs())?
            .bind_drafting_context(source_view, content)
            .map_err(|e| e.to_string())?;
        let admitted_garment = context
            .admit_garment_source(content)
            .map_err(|e| e.to_string())?;
        let root_scope = context
            .prepare_design_semantic_sources(content)
            .map_err(|e| e.to_string())?;
        let grainline = GrainlinePlacement::new(id(80000), id(90000))?;
        let raw = DraftingProgramInputs::new(
            context,
            vec![
                DraftingProgramStep::Semantic { ordinal: 1 },
                DraftingProgramStep::Drafting(DraftingOperationInput::PlaceGrainline(&grainline)),
            ],
        );
        let pp = DraftingProgramLimits::new(content, 1000, 1000, 10000);
        let program = raw
            .admit(&pp)
            .map_err(|e| e.to_string())?
            .normalized()
            .map_err(|e| e.to_string())?;
        let names = DraftingProgramNameInputs::new(&program, vec![], vec![]);
        let np = DraftingProgramNameLimits::new(content, 1000, 10000);
        let work = names.admit_name_work(&np).map_err(|e| e.to_string())?;
        let positions = work.bind_output_positions().map_err(|e| e.to_string())?;
        let roles = positions.bind_role_sources().map_err(|e| e.to_string())?;
        let initials = roles.bind_initial_sources().map_err(|e| e.to_string())?;
        let namespace = initials
            .bind_initial_namespace()
            .map_err(|e| e.to_string())?;
        if case == 0 {
            let refused = namespace
                .check_root_sources(FormulaSourceCheckLimits::new(2, 1000, 0, 10000))
                .expect_err("whole Derived target quota precedes source acceptance");
            assert!(matches!(
                refused.cause().refusal(),
                FormulaSourceCheckRefusal::Targets {
                    allowed: 0,
                    required: 3
                }
            ));
        }
        let sources = namespace
            .check_root_sources(FormulaSourceCheckLimits::new(2, 1000, 3, 10000))
            .map_err(|e| e.cause().to_string())?;
        let dependencies = sources
            .proof()
            .prepare_dependencies(FormulaSourceDependencyLimits::new(2, 0, 3))
            .map_err(|e| e.to_string())?;
        let graph = dependencies
            .admit_acyclic(FormulaSourceCycleLimits::new(2, 4))
            .map_err(|e| e.to_string())?;
        let op = DraftingProgramOrderLimits::new(1000, 10000, 100000);
        let order = namespace.check_order(&op).map_err(|e| e.to_string())?;
        let inputs = sources
            .dependency_inputs(&order, &graph)
            .map_err(|e| e.to_string())?;
        let cp = DraftingProgramCanonicalLimits::new(10000, 100000);
        let canonical = inputs
            .bind_canonical_inputs(&cp)
            .map_err(|e| e.to_string())?;
        let gp = DraftingProgramGeometryLimits::new(content, 10000, 100000, 100000);
        let geometry_work = canonical
            .admit_geometry_work(&gp)
            .map_err(|e| e.to_string())?;
        let geometry = geometry_work.bind_geometry().map_err(|e| e.to_string())?;
        let selected = SelectedProfilePin::new(SelectedProfilePinDefinition {
            profile_id: "original".into(),
            schema_version: "schema".into(),
            content_version: "content".into(),
        })?;

        let rows = vec![
            DraftingSemanticSelectionRow::Rule {
                ordinal: 1,
                origin: DraftingRuleSelection::Design,
            },
            DraftingSemanticSelectionRow::Profile {
                location: DraftingSemanticSelectionLocation::Semantic {
                    ordinal: 1,
                    field: DraftingSemanticSelectionField::LogicalInput { index: 5 },
                },
                selected: &selected,
            },
        ];
        let raw_selections = DraftingSemanticSelectionInputs::new(rows);
        let selections = raw_selections
            .admit(DraftingSemanticSelectionLimits::new(content, 10000))
            .map_err(|e| e.to_string())?;
        let actual = geometry
            .semantic_inputs(&admitted_garment, &selections)
            .map_err(|e| e.to_string())?;
        let mut meter = DesignResourceMeter::new(content);
        actual.charge_resources(&mut meter)?;
        let demand = meter.finish()?;

        let occurrences = 9;
        let row_count = 2;
        let passes = if case == 2 { 8 } else { 9 };
        let per_pass = (demand.collection_entries() + 8) * (demand.collection_entries() + 8);
        let policy = DraftingProgramSemanticWorkLimits::new(
            DraftingProgramSemanticPopulationLimits::new(
                content,
                occurrences,
                2 * occurrences * row_count,
                occurrences + row_count,
            ),
            passes,
            DraftingProgramSemanticChildContentLimits::new(
                passes as u128 * demand.collection_entries() as u128,
                passes as u128 * demand.text_bytes() as u128,
                passes as u128 * u128::from(demand.counter_bits()),
            ),
            passes as u128 * per_pass as u128,
            occurrences + row_count + passes,
        );
        let checked = actual
            .check_direct_selections(&policy)
            .map_err(|e| e.to_string())?;
        let owner = checked
            .root_input_scope(&root_scope)
            .map_err(|e| e.to_string())?;

        let logical = owner
            .bind_root_input_members()
            .map_err(|e| e.to_string())?
            .bind_root_logical_suppliers()
            .map_err(|e| e.to_string())?;
        assert_eq!(logical.members().records().len(), 5);
        assert!(logical.records().is_empty());
        source.walks.set(0);
        source.base.trace.set(0);
        source.alternate.trace.set(0);
        if case == 3 {
            source.extra_charge.set(100001);
        }
        let result = logical.bind_root_numeric_suppliers();
        if case == 0 || case == 1 {
            let proof = result.map_err(|e| e.to_string())?;
            let count = if case == 1 { 0 } else { 4 };
            assert_eq!(proof.records().len(), count);
            assert_eq!(proof.demand(), if count == 0 { None } else { Some(demand) });
            assert_eq!(source.walks.get(), usize::from(count > 0));
            assert!(core::ptr::eq(
                proof.logical().members().owner().inputs(),
                &actual
            ));
            assert_eq!(
                proof
                    .logical()
                    .members()
                    .owner()
                    .selections()
                    .reserved_child_passes(),
                5 + count
            );
            for (index, record) in proof.records().iter().copied().enumerate() {
                assert_eq!(record.member_index(), index);
                let original = proof
                    .logical()
                    .members()
                    .records()
                    .get(index)
                    .ok_or("actual input member")?;
                assert_eq!((original.ordinal(), original.position()), (1, 1));
                let DraftingProgramRootInputSource::Numeric(member) = original.source() else {
                    return Err("actual numeric member".into());
                };
                assert!(core::ptr::eq(member.scope(), &root_scope));
                assert_eq!(record.input().target(), member.binding().target());
                assert_eq!(record.input().position(), member.binding().position());
                match member.member() {
                    CanonicalNumericInputMember::Length { declaration, .. } => {
                        assert!(core::ptr::eq(
                            record.input().length().ok_or("original length")?,
                            declaration
                        ))
                    }
                    CanonicalNumericInputMember::Scalar { declaration, .. } => {
                        assert!(core::ptr::eq(
                            record.input().scalar().ok_or("original scalar")?,
                            declaration
                        ))
                    }
                }
                assert!(core::ptr::eq(
                    record.input().source(),
                    garment
                        .inputs()
                        .formula_sources()
                        .source(id(if index == 2 { 501 } else { 500 }))
                        .ok_or("actual source")?
                ));
                assert!(core::ptr::eq(
                    record.source(),
                    sources
                        .proof()
                        .source(record.input().source().id())
                        .ok_or("actual checked source")?
                ));
                assert!(core::ptr::eq(
                    record.source().normalized().source(),
                    record.input().source()
                ));
                assert!(core::ptr::eq(record.graph(), &graph));
                assert!(core::ptr::eq(
                    record.graph().dependencies().checked(),
                    sources.proof()
                ));
            }
            if count > 0 {
                let first = proof.records().first().ok_or("first relation")?;
                let repeated = proof.records().get(3).ok_or("repeated relation")?;
                assert_eq!(first.input().target(), repeated.input().target());
                assert!(core::ptr::eq(first.source(), repeated.source()));
                assert_ne!(first.member_index(), repeated.member_index());
                assert_ne!(
                    first.input().target(),
                    proof
                        .records()
                        .get(1)
                        .ok_or("general length")?
                        .input()
                        .target()
                );
            }
        } else {
            let error = result.expect_err("atomic budget or actual fresh resource refusal");
            assert_eq!(error.supplier_count(), Some(4));
            assert_eq!(error.demand(), None);
            assert!(core::ptr::eq(
                error.owner().members().owner().inputs(),
                &actual
            ));
            assert!(core::ptr::eq(
                error.owner().members().owner().scope(),
                &root_scope
            ));
            assert_eq!(
                error
                    .owner()
                    .members()
                    .owner()
                    .selections()
                    .reserved_child_passes(),
                if case == 2 { 5 } else { 9 }
            );
            match error.refusal() {
                DraftingProgramRootNumericSupplierRefusal::Work(cause) if case == 2 => {
                    assert!(matches!(
                        cause.refusal(),
                        DraftingProgramSemanticWorkRefusal::ChildPasses {
                            allowed: 8,
                            reserved: 5,
                            requested: 4
                        }
                    ));
                    assert_eq!(source.walks.get(), 0);
                }
                DraftingProgramRootNumericSupplierRefusal::Resources(cause) if case == 3 => {
                    assert!(
                        matches!(cause,DesignResourceLimitError::Count {unit:DesignResourceCount::CollectionEntries,maximum:100000,requested:100001,location,..} if location.field=="numeric.extra" && location.owner==Some(garment.inputs().id()))
                    );
                    assert_eq!(source.walks.get(), 1);
                }
                _ => return Err(format!("unexpected numeric refusal: {error}").into()),
            }
        }
        assert_eq!(
            (source.base.trace.get(), source.alternate.trace.get()),
            if case == 1 || case == 2 {
                (0, 0)
            } else {
                (2, 2)
            }
        );
    }
    Ok(())
}
