//! Real Root operation fields under the full original owner chain and one finite child cursor.
use super::{garment_source, limits, scope, EmptyHost, Provider, SourceView, TestResult};
use sc_core::{construction::*, design::*, name::MachineToken, ontology::*, recipe::*, value::*};
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn fixture(mode: u8) -> TestResult<DesignGarment<EmptyHost>> {
    let original = super::super::semantic_root::canonical()?;
    let mut logicals = original.logicals().declarations().to_vec();
    if mode != 2 {
        logicals.push(LogicalInputDeclaration::new(
            LogicalInputDeclarationDefinition {
                id: id(1100),
                source: id(9200),
                kind: LogicalInputKind::NotionSize,
                domain: vec![MachineToken::new("hardware_symbol")?],
                state: LogicalInputState::Unknown {
                    observation: id(9201),
                },
            },
            LogicalInputLimits::new(1, 0, 15, 2),
        )?);
    }
    let canonical = CanonicalInputs::with_logicals(
        original.lengths().to_vec(),
        original.scalars().to_vec(),
        LogicalInputs::new(logicals, LogicalInputInventoryLimits::new(2, 4))?,
    )?;
    let button = ButtonholeOperation::new(
        ButtonholeOperationDefinition {
            id: id(70),
            button: if mode >= 4 {
                NotionSize::Profile(ProfileParameterRef::new(id(701)))
            } else {
                NotionSize::Declaration(id(1100))
            },
            policy: id(900),
            inputs: if mode == 5 {
                vec![]
            } else {
                ["policy_a", "policy_b"]
                    .into_iter()
                    .map(|role| {
                        Ok(ButtonholePolicyInput {
                            role: MachineToken::new(role)?,
                            target: CanonicalInputTarget::Length(id(10)),
                        })
                    })
                    .collect::<TestResult<Vec<_>>>()?
            },
        },
        ButtonholeOperationLimits::new(10, 1000, 1000),
    )?;
    let input = match mode {
        1 => LogicalDerivationInputSource::Logical(LogicalInputTarget {
            declaration: id(90),
            kind: LogicalInputKind::PocketOpening,
        }),
        3 => LogicalDerivationInputSource::Numeric(CanonicalInputTarget::Scalar {
            declaration: id(20),
            kind: ScalarInputKind::Count,
        }),
        4 | 5 => LogicalDerivationInputSource::Profile {
            parameter: ProfileParameterRef::new(id(702)),
            kind: LogicalInputKind::NotionSize,
        },
        _ => LogicalDerivationInputSource::Numeric(CanonicalInputTarget::Scalar {
            declaration: id(20),
            kind: ScalarInputKind::Boolean,
        }),
    };
    let logical = LogicalDerivationOperation::new(
        LogicalDerivationDefinition {
            id: id(80),
            output: LogicalInputTarget {
                declaration: id(90),
                kind: LogicalInputKind::PocketOpening,
            },
            rule: id(901),
            inputs: vec![LogicalDerivationInput {
                role: MachineToken::new("source_a")?,
                source: input,
            }],
        },
        LogicalDerivationLimits::new(10, 1000, 1000),
    )?;
    garment_source::garment_with_semantics(
        canonical,
        SemanticRecipe::new(
            vec![
                SemanticOperation::Buttonhole(button),
                SemanticOperation::LogicalDerivation(logical),
            ],
            SemanticRecipeLimits::new(2, 4),
        )?,
    )
}
#[test]
fn complete_root_operation_members_keep_original_fields_sources_and_bounded_stage_order(
) -> TestResult {
    // 0 numeric,1 actual Derived logical (order not yet accepted),2 missing first button,
    // 3 wrong last numeric kind,4 Profile fields skipped,5 no Root fields,6 short capacity.
    for case in 0..7u8 {
        let mode = if case == 6 { 0 } else { case };
        let garment = fixture(mode)?;
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
        if mode >= 4 {
            rows.push(DraftingSemanticSelectionRow::Profile {
                location: DraftingSemanticSelectionLocation::Semantic {
                    ordinal: 1,
                    field: DraftingSemanticSelectionField::ButtonholeButton,
                },
                selected: &selected,
            });
            rows.push(DraftingSemanticSelectionRow::Profile {
                location: DraftingSemanticSelectionLocation::Semantic {
                    ordinal: 2,
                    field: DraftingSemanticSelectionField::LogicalInput { index: 0 },
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
        let occurrences = if mode == 5 { 11 } else { 13 };
        let row_count = if mode >= 4 { 5 } else { 3 };
        let expected_count = if mode == 5 {
            0
        } else if mode == 4 {
            2
        } else {
            4
        };
        let passes = if case == 6 { 2 } else { expected_count };
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
        source.walks.set(0);
        source.base.trace.set(0);
        source.alternate.trace.set(0);
        let result = owner.bind_root_input_members();
        if matches!(case, 2 | 3 | 6) {
            let error = result.expect_err("first original Root member or complete child quota");
            assert!(core::ptr::eq(error.owner().inputs(), &actual));
            assert!(core::ptr::eq(error.owner().scope(), &root_scope));
            assert!(core::ptr::eq(error.owner().selections().limits(), &policy));
            assert_eq!(error.member_count(), Some(4));
            match error.refusal() {
                DraftingProgramRootInputMemberRefusal::Work(cause) if case == 6 => {
                    assert!(matches!(
                        cause.refusal(),
                        DraftingProgramSemanticWorkRefusal::ChildPasses {
                            allowed: 2,
                            reserved: 0,
                            requested: 4
                        }
                    ));
                    assert_eq!(error.demand(), None);
                    assert_eq!(source.walks.get(), 0);
                    assert_eq!(error.owner().selections().reserved_child_passes(), 0);
                }
                DraftingProgramRootInputMemberRefusal::Lookup { location, cause } => {
                    assert!(core::ptr::eq(cause.scope(), &root_scope));
                    assert!(error.demand().is_some());
                    assert_eq!(source.walks.get(), 1);
                    assert_eq!(error.owner().selections().reserved_child_passes(), 4);
                    if case == 2 {
                        assert_eq!(
                            *location,
                            DraftingSemanticSelectionLocation::Semantic {
                                ordinal: 1,
                                field: DraftingSemanticSelectionField::ButtonholeButton
                            }
                        );
                        assert!(
                            matches!(cause.cause(),DraftingDesignSemanticLookupCause::Logical(LogicalInputLookupError::Missing {declaration,expected:LogicalInputKind::NotionSize}) if declaration==id(1100))
                        );
                    } else {
                        assert_eq!(
                            *location,
                            DraftingSemanticSelectionLocation::Semantic {
                                ordinal: 2,
                                field: DraftingSemanticSelectionField::LogicalInput { index: 0 }
                            }
                        );
                        let DraftingDesignSemanticLookupCause::Numeric(cause) = cause.cause()
                        else {
                            return Err("missing original scalar cause".into());
                        };
                        assert!(core::ptr::eq(cause.inputs(), garment.inputs().inputs()));
                        assert!(
                            matches!(cause.refusal(),CanonicalNumericInputRefusal::ScalarKind {expected:ScalarInputKind::Count,index:0,actual:member} if core::ptr::eq(member,garment.inputs().inputs().scalar(id(20)).ok_or("original scalar")?))
                        );
                    }
                }
                _ => return Err(format!("wrong first Root member refusal: {case}: {error}").into()),
            }
            assert!(!format!("{error:?} {error}").contains("hardware_symbol"));
        } else {
            let proof = result.map_err(|e| e.to_string())?;
            assert!(core::ptr::eq(proof.owner().inputs(), &actual));
            assert!(core::ptr::eq(proof.owner().scope(), &root_scope));
            assert_eq!(proof.records().len(), expected_count);
            assert_eq!(proof.owner().selections().remaining_child_passes(), 0);
            assert_eq!(
                proof.owner().selections().reserved_child_passes(),
                expected_count
            );
            assert_eq!(source.walks.get(), usize::from(expected_count != 0));
            assert_eq!(proof.demand().is_some(), expected_count != 0);
            for (index, record) in proof.records().iter().copied().enumerate() {
                let (ordinal, position) = if mode == 4 || (index < 3) {
                    (1, 1)
                } else {
                    (2, 3)
                };
                assert_eq!((record.ordinal(), record.position()), (ordinal, position));
                assert!(core::ptr::eq(
                    record.operation(),
                    garment
                        .inputs()
                        .semantic_recipe()
                        .operations()
                        .get(ordinal - 1)
                        .ok_or("actual operation")?
                ));
                match record.source() {
                    DraftingProgramRootInputSource::Logical(member) => {
                        assert!(core::ptr::eq(member.scope(), &root_scope));
                        assert!(core::ptr::eq(
                            member.binding().inputs(),
                            garment.inputs().inputs().logicals()
                        ));
                        if ordinal == 2 {
                            assert!(
                                matches!(member.declaration().definition().state,LogicalInputState::Derived {operation} if operation==id(80))
                            );
                        } else {
                            assert!(
                                matches!(member.declaration().definition().state,LogicalInputState::Unknown {observation} if observation==id(9201))
                            );
                        }
                    }
                    DraftingProgramRootInputSource::Numeric(member) => {
                        assert!(core::ptr::eq(member.scope(), &root_scope));
                        assert!(core::ptr::eq(
                            member.binding().inputs(),
                            garment.inputs().inputs()
                        ));
                        match member.member() {
                            CanonicalNumericInputMember::Length { declaration, .. } => {
                                assert!(core::ptr::eq(
                                    declaration,
                                    garment
                                        .inputs()
                                        .inputs()
                                        .length(id(10))
                                        .ok_or("original length")?
                                ))
                            }
                            CanonicalNumericInputMember::Scalar { declaration, .. } => {
                                assert!(core::ptr::eq(
                                    declaration,
                                    garment
                                        .inputs()
                                        .inputs()
                                        .scalar(id(20))
                                        .ok_or("original scalar")?
                                ))
                            }
                        }
                    }
                }
                assert!(!format!("{record:?}").contains("hardware_symbol"));
            }
            if expected_count >= 2 {
                let first = proof
                    .records()
                    .get(if mode == 4 { 0 } else { 1 })
                    .ok_or("first repeated original policy input")?;
                let second = proof
                    .records()
                    .get(if mode == 4 { 1 } else { 2 })
                    .ok_or("second repeated original policy input")?;
                assert!(matches!(
                    first.field(),
                    DraftingSemanticField::ButtonholeInput { index: 0, .. }
                ));
                assert!(matches!(
                    second.field(),
                    DraftingSemanticField::ButtonholeInput { index: 1, .. }
                ));
                let (
                    DraftingProgramRootInputSource::Numeric(a),
                    DraftingProgramRootInputSource::Numeric(b),
                ) = (first.source(), second.source())
                else {
                    return Err("original repeated numeric sources".into());
                };
                assert_eq!(a.binding().target(), b.binding().target());
                assert_eq!(first.position(), second.position());
                assert_ne!(first.location(), second.location());
            }
            assert_eq!(proof.owner().selections().row_kinds().len(), row_count);
            assert!(!format!("{proof:?}").contains("hardware_symbol"));
        }
    }
    Ok(())
}
