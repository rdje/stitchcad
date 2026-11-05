//! Actual numeric Derived association to original checked-source and acyclic owners.
use super::{limits, scope, EmptyHost, Provider, SourceView, TestResult};
use sc_core::{construction::*, design::*, name::MachineToken, ontology::*, recipe::*, value::*};
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
pub(super) fn fixture(
    empty: bool,
    combined: bool,
    profile_inputs: usize,
    button: bool,
) -> TestResult<DesignGarment<EmptyHost>> {
    build_fixture(empty, combined, profile_inputs, button, false, true, 0)
}
pub(super) fn interleaved_fixture() -> TestResult<DesignGarment<EmptyHost>> {
    build_fixture(false, false, 2, true, true, true, 0)
}
pub(super) fn without_semantics_fixture() -> TestResult<DesignGarment<EmptyHost>> {
    build_fixture(true, false, 0, false, false, false, 0)
}
pub(super) fn signature_fixture(output_mode: u8) -> TestResult<DesignGarment<EmptyHost>> {
    build_fixture(false, true, 2, true, false, true, output_mode)
}
fn build_fixture(
    empty: bool,
    combined: bool,
    profile_inputs: usize,
    button: bool,
    interleaved: bool,
    include_semantics: bool,
    output_mode: u8,
) -> TestResult<DesignGarment<EmptyHost>> {
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
        {
            let first = LogicalInputs::new(
                vec![LogicalInputDeclaration::new(
                    LogicalInputDeclarationDefinition {
                        id: id(90),
                        source: id(1090),
                        kind: LogicalInputKind::PocketOpening,
                        domain: vec![MachineToken::new("private_option")?],
                        state: if combined {
                            LogicalInputState::Derived { operation: id(79) }
                        } else {
                            LogicalInputState::Unknown {
                                observation: id(1190),
                            }
                        },
                    },
                    LogicalInputLimits::new(1, 0, 14, 2),
                )?],
                LogicalInputInventoryLimits::new(1, 1),
            )?;
            if combined {
                let mut declarations = first.declarations().to_vec();
                if output_mode != 4 {
                    declarations.push(LogicalInputDeclaration::new(
                        LogicalInputDeclarationDefinition {
                            id: id(91),
                            source: id(1091),
                            kind: if output_mode == 5 {
                                LogicalInputKind::NotionSize
                            } else {
                                LogicalInputKind::PocketOpening
                            },
                            domain: vec![MachineToken::new("other_option")?],
                            state: if output_mode == 6 {
                                LogicalInputState::Unknown {
                                    observation: id(1191),
                                }
                            } else {
                                LogicalInputState::Derived {
                                    operation: id(if output_mode == 7 { 79 } else { 80 }),
                                }
                            },
                        },
                        LogicalInputLimits::new(1, 0, 12, 2),
                    )?);
                }
                LogicalInputs::new(declarations, LogicalInputInventoryLimits::new(2, 4))?
            } else {
                first
            }
        },
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
    for index in 0..profile_inputs {
        inputs.push(LogicalDerivationInput {
            role: MachineToken::new(format!("profile_{index}"))?,
            source: LogicalDerivationInputSource::Profile {
                parameter: ProfileParameterRef::new(id(702)),
                kind: LogicalInputKind::PocketOpening,
            },
        });
    }
    if interleaved {
        let last = inputs.pop().ok_or("original last Profile input")?;
        inputs.insert(1, last);
        let previous = inputs.pop().ok_or("original previous Profile input")?;
        inputs.insert(4, previous);
    }
    let semantic = LogicalDerivationOperation::new(
        LogicalDerivationDefinition {
            id: id(80),
            output: LogicalInputTarget {
                declaration: id(if combined { 91 } else { 90 }),
                kind: LogicalInputKind::PocketOpening,
            },
            rule: id(901),
            inputs,
        },
        LogicalDerivationLimits::new(7, 100, 49),
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
            {
                let mut operations = vec![];
                if combined {
                    operations.push(SemanticOperation::LogicalDerivation(
                        LogicalDerivationOperation::new(
                            LogicalDerivationDefinition {
                                id: id(79),
                                output: LogicalInputTarget {
                                    declaration: id(90),
                                    kind: LogicalInputKind::PocketOpening,
                                },
                                rule: id(900),
                                inputs: vec![],
                            },
                            LogicalDerivationLimits::new(0, 0, 0),
                        )?,
                    ));
                }
                if include_semantics {
                    operations.push(SemanticOperation::LogicalDerivation(semantic));
                }
                if button {
                    operations.push(SemanticOperation::Buttonhole(ButtonholeOperation::new(
                        ButtonholeOperationDefinition {
                            id: id(81),
                            button: NotionSize::Profile(ProfileParameterRef::new(id(703))),
                            policy: id(902),
                            inputs: if interleaved {
                                vec![ButtonholePolicyInput {
                                    role: MachineToken::new("button_numeric")?,
                                    target: CanonicalInputTarget::Length(id(10)),
                                }]
                            } else {
                                vec![]
                            },
                        },
                        ButtonholeOperationLimits::new(1, 14, 1),
                    )?));
                }
                operations
            },
            SemanticRecipeLimits::new(
                if combined && button { 3 } else { 2 },
                if combined && button { 9 } else { 4 },
            ),
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
    for case in 0..5u8 {
        let garment = fixture(case == 1, case == 4, 1, false)?;
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
        let mut steps = vec![
            DraftingProgramStep::Semantic { ordinal: 1 },
            DraftingProgramStep::Drafting(DraftingOperationInput::PlaceGrainline(&grainline)),
        ];
        if case == 4 {
            steps.push(DraftingProgramStep::Semantic { ordinal: 2 });
        }
        let raw = DraftingProgramInputs::new(context, steps);
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

        let mut rows = vec![
            DraftingSemanticSelectionRow::Rule {
                ordinal: 1,
                origin: DraftingRuleSelection::Design,
            },
            DraftingSemanticSelectionRow::Profile {
                location: DraftingSemanticSelectionLocation::Semantic {
                    ordinal: if case == 4 { 2 } else { 1 },
                    field: DraftingSemanticSelectionField::LogicalInput { index: 5 },
                },
                selected: &selected,
            },
        ];
        if case == 4 {
            rows.push(DraftingSemanticSelectionRow::Rule {
                ordinal: 2,
                origin: DraftingRuleSelection::Design,
            });
        }
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

        let occurrences = if case == 4 { 11 } else { 9 };
        let row_count = if case == 4 { 3 } else { 2 };
        let passes = if case == 2 {
            8
        } else if case == 4 {
            10
        } else {
            9
        };
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
        assert_eq!(logical.records().len(), usize::from(case == 4));
        source.walks.set(0);
        source.base.trace.set(0);
        source.alternate.trace.set(0);
        if case == 3 {
            source.extra_charge.set(100001);
        }
        let result = logical.bind_root_numeric_suppliers();
        if case == 0 || case == 1 || case == 4 {
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
                5 + count + usize::from(case == 4)
            );
            for (index, record) in proof.records().iter().copied().enumerate() {
                assert_eq!(record.member_index(), index);
                let original = proof
                    .logical()
                    .members()
                    .records()
                    .get(index)
                    .ok_or("actual input member")?;
                assert_eq!(
                    (original.ordinal(), original.position()),
                    if case == 4 { (2, 3) } else { (1, 1) }
                );
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
            assert_eq!(proof.bindings().len(), 5);
            let passes_before = proof
                .logical()
                .members()
                .owner()
                .selections()
                .reserved_child_passes();
            let walks_before = source.walks.get();
            for binding in proof.bindings() {
                let index = binding.member_index();
                let original = proof
                    .logical()
                    .members()
                    .records()
                    .get(index)
                    .ok_or("complete original input")?;
                assert_eq!(binding.member().location(), original.location());
                assert!(core::ptr::eq(
                    binding.member().operation(),
                    original.operation()
                ));
                match binding.supplier() {
                    Some(DraftingProgramRootInputSupplier::Numeric(record))
                        if case != 1 && index < 4 =>
                    {
                        assert!(core::ptr::eq(
                            record,
                            proof
                                .records()
                                .get(index)
                                .ok_or("original numeric relation")?
                        ))
                    }
                    Some(DraftingProgramRootInputSupplier::Logical(record))
                        if case == 4 && index == 4 =>
                    {
                        assert!(core::ptr::eq(
                            record,
                            proof
                                .logical()
                                .records()
                                .first()
                                .ok_or("original logical relation")?
                        ));
                        assert_eq!((record.ordinal(), record.position()), (1, 1));
                        assert_eq!(
                            (binding.member().ordinal(), binding.member().position()),
                            (2, 3)
                        );
                    }
                    None if case == 1 || index == 4 => {}
                    _ => return Err("incorrect complete Root source class".into()),
                }
            }
            let mut tail = proof.bindings().skip(4);
            assert_eq!(tail.next().ok_or("last binding")?.member_index(), 4);
            assert!(tail.next().is_none());
            assert!(tail.next().is_none());
            assert_eq!(source.walks.get(), walks_before);
            assert_eq!(
                proof
                    .logical()
                    .members()
                    .owner()
                    .selections()
                    .reserved_child_passes(),
                passes_before
            );
            let skipped = proof.bindings().nth(3).ok_or("fourth original member")?;
            assert_eq!(skipped.member_index(), 3);
            assert_eq!(skipped.supplier().is_some(), count > 0);
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
