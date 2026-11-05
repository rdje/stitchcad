//! Public-factory logical Derived joins under actual Root owners and genuine mixed order.
use super::{garment_source, limits, scope, EmptyHost, Provider, SourceView, TestResult};
use sc_core::{construction::*, design::*, name::MachineToken, ontology::*, recipe::*, value::*};
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn target(declaration: u128, kind: LogicalInputKind) -> LogicalInputTarget {
    LogicalInputTarget {
        declaration: id(declaration),
        kind,
    }
}
fn fixture(case: u8) -> TestResult<DesignGarment<EmptyHost>> {
    let original = super::super::semantic_root::canonical()?;
    let state = match case {
        1 => LogicalInputState::Derived { operation: id(80) },
        3 => LogicalInputState::Derived { operation: id(999) },
        8 => LogicalInputState::Unknown {
            observation: id(9300),
        },
        _ => LogicalInputState::Derived { operation: id(70) },
    };
    let mut definition = original
        .logicals()
        .declarations()
        .first()
        .ok_or("original logical")?
        .definition()
        .clone();
    definition.state = state;
    let logical = LogicalInputDeclaration::new(definition, LogicalInputLimits::new(1, 0, 8, 2))?;
    let second = LogicalInputDeclaration::new(
        LogicalInputDeclarationDefinition {
            id: id(1100),
            source: id(9200),
            kind: LogicalInputKind::NotionSize,
            domain: vec![MachineToken::new("hidden_symbol")?],
            state: LogicalInputState::Derived { operation: id(80) },
        },
        LogicalInputLimits::new(1, 0, 13, 2),
    )?;
    let canonical = CanonicalInputs::with_logicals(
        original.lengths().to_vec(),
        original.scalars().to_vec(),
        LogicalInputs::new(
            vec![logical, second],
            LogicalInputInventoryLimits::new(2, 4),
        )?,
    )?;
    let supplier = if case == 4 {
        SemanticOperation::Buttonhole(ButtonholeOperation::new(
            ButtonholeOperationDefinition {
                id: id(70),
                button: NotionSize::Profile(ProfileParameterRef::new(id(701))),
                policy: id(900),
                inputs: vec![],
            },
            ButtonholeOperationLimits::new(0, 0, 0),
        )?)
    } else {
        SemanticOperation::LogicalDerivation(LogicalDerivationOperation::new(
            LogicalDerivationDefinition {
                id: id(70),
                output: match case {
                    5 => target(1100, LogicalInputKind::NotionSize),
                    6 => target(90, LogicalInputKind::NotionSize),
                    _ => target(90, LogicalInputKind::PocketOpening),
                },
                rule: id(900),
                inputs: vec![],
            },
            LogicalDerivationLimits::new(0, 0, 0),
        )?)
    };
    let consumer = SemanticOperation::LogicalDerivation(LogicalDerivationOperation::new(
        LogicalDerivationDefinition {
            id: id(80),
            output: if case == 9 {
                target(1100, LogicalInputKind::NotionSize)
            } else {
                target(90, LogicalInputKind::PocketOpening)
            },
            rule: id(901),
            inputs: vec![
                LogicalDerivationInput {
                    role: MachineToken::new("first")?,
                    source: LogicalDerivationInputSource::Logical(target(
                        90,
                        LogicalInputKind::PocketOpening,
                    )),
                },
                LogicalDerivationInput {
                    role: MachineToken::new("second")?,
                    source: LogicalDerivationInputSource::Logical(if case == 9 {
                        target(1100, LogicalInputKind::NotionSize)
                    } else {
                        target(90, LogicalInputKind::PocketOpening)
                    }),
                },
                LogicalDerivationInput {
                    role: MachineToken::new("numeric")?,
                    source: LogicalDerivationInputSource::Numeric(CanonicalInputTarget::Length(
                        id(10),
                    )),
                },
            ],
        },
        LogicalDerivationLimits::new(3, 18, 9),
    )?);
    garment_source::garment_with_semantics(
        canonical,
        SemanticRecipe::new(
            if case == 2 {
                vec![consumer, supplier]
            } else {
                vec![supplier, consumer]
            },
            SemanticRecipeLimits::new(2, 4),
        )?,
    )
}
#[test]
fn root_logical_suppliers_require_actual_output_and_strict_genuine_mixed_order() -> TestResult {
    // Earlier,repeated,self,future,missing,wrong family/output ID/output kind,atomic budget,
    // no logical Derived work,and a late self failure after one valid private supplier.
    for case in 0..10u8 {
        let garment = fixture(case)?;
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
                DraftingProgramStep::Semantic { ordinal: 2 },
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
        let sources = namespace
            .check_root_sources(FormulaSourceCheckLimits::new(1, 1000, 0, 10000))
            .map_err(|e| e.to_string())?;
        let dependencies = sources
            .proof()
            .prepare_dependencies(FormulaSourceDependencyLimits::new(1, 0, 2))
            .map_err(|e| e.to_string())?;
        let graph = dependencies
            .admit_acyclic(FormulaSourceCycleLimits::new(1, 2))
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
            DraftingSemanticSelectionRow::Profile {
                location: DraftingSemanticSelectionLocation::Pocket {
                    pocket: 0,
                    field: DraftingPocketSelectionField::Opening,
                },
                selected: &selected,
            },
            DraftingSemanticSelectionRow::Rule {
                ordinal: 1,
                origin: DraftingRuleSelection::Design,
            },
            DraftingSemanticSelectionRow::Rule {
                ordinal: 2,
                origin: DraftingRuleSelection::Design,
            },
        ];
        if case == 4 {
            rows.push(DraftingSemanticSelectionRow::Profile {
                location: DraftingSemanticSelectionLocation::Semantic {
                    ordinal: 1,
                    field: DraftingSemanticSelectionField::ButtonholeButton,
                },
                selected: &selected,
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

        let occurrences = 13;
        let row_count = rows_count(case);
        let passes = if case == 7 { 4 } else { 5 };
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

        let members = owner.bind_root_input_members().map_err(|e| e.to_string())?;
        assert_eq!(members.records().len(), 3);
        assert_eq!(members.owner().selections().reserved_child_passes(), 3);
        source.walks.set(0);
        source.base.trace.set(0);
        source.alternate.trace.set(0);
        let result = members.bind_root_logical_suppliers();
        if case == 0 || case == 8 {
            let proof = result.map_err(|e| e.to_string())?;
            let count = if case == 8 { 0 } else { 2 };
            assert_eq!(proof.records().len(), count);
            assert_eq!(proof.demand().is_some(), count > 0);
            assert_eq!(source.walks.get(), usize::from(count > 0));
            assert!(core::ptr::eq(proof.members().owner().inputs(), &actual));
            assert!(core::ptr::eq(proof.members().owner().scope(), &root_scope));
            assert_eq!(
                proof.members().owner().selections().reserved_child_passes(),
                3 + count
            );
            for (index, record) in proof.records().iter().enumerate() {
                assert_eq!(record.member_index(), index);
                assert_eq!((record.ordinal(), record.position()), (1, 1));
                let member = *proof
                    .members()
                    .records()
                    .get(index)
                    .ok_or("original member")?;
                assert_eq!((member.ordinal(), member.position()), (2, 3));
                assert!(core::ptr::eq(record.source().scope(), &root_scope));
                assert!(core::ptr::eq(
                    record.source().operation(),
                    garment
                        .inputs()
                        .semantic_recipe()
                        .operations()
                        .first()
                        .ok_or("original supplier")?
                ));
                let SemanticOperation::LogicalDerivation(supplier) = record.source().operation()
                else {
                    return Err("actual supplier".into());
                };
                assert!(core::ptr::eq(
                    record.output(),
                    &supplier.definition().output
                ));
                let DraftingProgramRootInputSource::Logical(member) = member.source() else {
                    return Err("logical consumer".into());
                };
                assert!(core::ptr::eq(
                    member.declaration(),
                    garment
                        .inputs()
                        .inputs()
                        .logicals()
                        .declarations()
                        .first()
                        .ok_or("original logical member")?
                ));
                assert_eq!(
                    member.declaration().definition().state.tag(),
                    LogicalInputStateTag::Derived
                );
            }
            assert!(!format!("{proof:?}").contains("hidden_symbol"));
        } else {
            let error = result.expect_err("first original supplier refusal");
            assert_eq!(error.supplier_count(), Some(2));
            assert!(core::ptr::eq(error.owner().owner().inputs(), &actual));
            assert!(core::ptr::eq(error.owner().owner().scope(), &root_scope));
            assert_eq!(error.owner().records().len(), 3);
            assert_eq!(error.demand().is_some(), case != 7);
            assert_eq!(source.walks.get(), usize::from(case != 7));
            assert_eq!(
                error.owner().owner().selections().reserved_child_passes(),
                if case == 7 { 3 } else { 5 }
            );
            match error.refusal() {
                DraftingProgramRootLogicalSupplierRefusal::Work(cause) if case == 7 => {
                    assert!(matches!(
                        cause.refusal(),
                        DraftingProgramSemanticWorkRefusal::ChildPasses {
                            allowed: 4,
                            reserved: 3,
                            requested: 2
                        }
                    ))
                }
                DraftingProgramRootLogicalSupplierRefusal::Lookup {
                    member_index,
                    cause,
                } if case == 3 || case == 4 => {
                    assert_eq!(*member_index, 0);
                    assert!(core::ptr::eq(cause.scope(), &root_scope));
                    assert!(
                        matches!(cause.request(),DraftingDesignSemanticRequest::Operation {operation,kind:SemanticOperationKind::LogicalDerivation} if operation==id(if case==3 {999} else {70}))
                    );
                    let DraftingDesignSemanticLookupCause::Operation(cause) = cause.cause() else {
                        return Err("typed supplier cause".into());
                    };
                    if case == 3 {
                        assert!(matches!(
                            cause,
                            SemanticOperationLookupError::Missing { .. }
                        ));
                    } else {
                        assert!(matches!(cause, SemanticOperationLookupError::Kind { .. }));
                    }
                }
                DraftingProgramRootLogicalSupplierRefusal::Output {
                    member_index,
                    source,
                } if case == 5 || case == 6 => {
                    assert_eq!(*member_index, 0);
                    assert!(core::ptr::eq(
                        source.operation(),
                        garment
                            .inputs()
                            .semantic_recipe()
                            .operations()
                            .first()
                            .ok_or("original supplier")?
                    ));
                }
                DraftingProgramRootLogicalSupplierRefusal::Order {
                    member_index,
                    source,
                    ordinal,
                    position,
                } if matches!(case, 1 | 2 | 9) => {
                    assert_eq!(*member_index, usize::from(case == 9));
                    assert_eq!((*ordinal, *position), (2, 3));
                    assert!(core::ptr::eq(
                        source.operation(),
                        garment
                            .inputs()
                            .semantic_recipe()
                            .operations()
                            .get(1)
                            .ok_or("original later supplier")?
                    ));
                    assert_eq!(
                        error
                            .owner()
                            .records()
                            .get(*member_index)
                            .ok_or("original refused consumer")?
                            .position(),
                        if case == 2 { 1 } else { 3 }
                    );
                }
                _ => return Err(format!("unexpected case{case}: {error}").into()),
            }
            assert!(!format!("{error:?} {error}").contains("hidden_symbol"));
        }
        let trace = if case == 7 || case == 8 { 0 } else { 2 };
        assert_eq!(
            (source.base.trace.get(), source.alternate.trace.get()),
            (trace, trace)
        );
    }
    Ok(())
}
fn rows_count(case: u8) -> usize {
    if case == 4 {
        4
    } else {
        3
    }
}
