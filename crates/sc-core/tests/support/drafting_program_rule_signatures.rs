//! Complete original named signatures and Root outputs under stable actual source parents.
use super::{id, limits, scope, Provider, RuleSourceView, TestResult};
use sc_core::{construction::*, design::*, name::MachineToken, recipe::*, value::*};
#[test]
fn complete_rule_signatures_preserve_named_pairs_actual_outputs_and_first_late_causes() -> TestResult
{
    for case in 0..15u8 {
        let empty = case == 13;
        let button = !empty;
        let output_mode = if (9..=12).contains(&case) {
            case - 5
        } else {
            0
        };
        let garment = if empty {
            super::super::super::root_numeric_suppliers::without_semantics_fixture()?
        } else {
            super::super::super::root_numeric_suppliers::signature_fixture(output_mode)?
        };
        let other = if empty {
            super::super::super::root_numeric_suppliers::without_semantics_fixture()?
        } else {
            super::super::super::root_numeric_suppliers::signature_fixture(output_mode)?
        };
        let mut source = Provider::new(&garment, other.inputs(), 0)?;
        let mut requirements = [
            (
                "numeric_0",
                SemanticRuleInputKind::Numeric(CanonicalNumericInputKind::Length),
            ),
            (
                "numeric_1",
                SemanticRuleInputKind::Numeric(CanonicalNumericInputKind::Scalar(if case == 5 {
                    ScalarInputKind::Ratio
                } else {
                    ScalarInputKind::Length
                })),
            ),
            (
                "numeric_2",
                SemanticRuleInputKind::Numeric(CanonicalNumericInputKind::Scalar(
                    ScalarInputKind::Boolean,
                )),
            ),
            (
                "numeric_3",
                SemanticRuleInputKind::Numeric(CanonicalNumericInputKind::Length),
            ),
            (
                "logical",
                SemanticRuleInputKind::Logical(LogicalInputKind::PocketOpening),
            ),
            (
                "profile_0",
                SemanticRuleInputKind::Logical(LogicalInputKind::PocketOpening),
            ),
            (
                if case == 6 { "foreign" } else { "profile_1" },
                SemanticRuleInputKind::Logical(LogicalInputKind::PocketOpening),
            ),
        ]
        .into_iter()
        .map(|(role, kind)| {
            Ok(SemanticRuleInput {
                role: MachineToken::new(role)?,
                kind,
            })
        })
        .collect::<TestResult<Vec<_>>>()?;
        requirements.reverse();
        if case == 8 {
            requirements.pop();
        }
        let signatures = vec![
            (
                900,
                SemanticRuleSignature::Logical {
                    result: LogicalInputKind::PocketOpening,
                    inputs: vec![],
                },
            ),
            (
                901,
                SemanticRuleSignature::Logical {
                    result: if case == 7 {
                        LogicalInputKind::NotionSize
                    } else {
                        LogicalInputKind::PocketOpening
                    },
                    inputs: requirements,
                },
            ),
            (
                902,
                SemanticRuleSignature::Buttonhole {
                    inputs: if case == 14 {
                        vec![ButtonholePolicyRequirement {
                            role: MachineToken::new("policy_extra")?,
                            kind: CanonicalNumericInputKind::Length,
                        }]
                    } else {
                        vec![]
                    },
                },
            ),
        ];
        source.rules = signatures
            .into_iter()
            .map(|(n, signature)| {
                Ok(super::super::RuleRow {
                    declaration: SemanticRuleDeclaration::new(
                        SemanticRuleDefinition {
                            id: id(n),
                            signature,
                        },
                        SemanticRuleLimits::new(7, 100, 49),
                    )?,
                    body: DraftingSourceRecord::new([1, 2, 3, 4]),
                })
            })
            .collect::<TestResult<Vec<_>>>()?;
        let source_view: &RuleSourceView<'_> = &source;
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
        let mut steps = vec![];
        if !empty {
            steps.push(DraftingProgramStep::Semantic { ordinal: 1 });
        }
        steps.push(DraftingProgramStep::Drafting(
            DraftingOperationInput::PlaceGrainline(&grainline),
        ));

        if button {
            steps.push(DraftingProgramStep::Semantic { ordinal: 2 });
            steps.push(DraftingProgramStep::Semantic { ordinal: 3 });
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
        let selected = super::super::pin("original")?;
        let rows = if empty {
            vec![]
        } else {
            vec![
                DraftingSemanticSelectionRow::Rule {
                    ordinal: 1,
                    origin: DraftingRuleSelection::Design,
                },
                DraftingSemanticSelectionRow::Rule {
                    ordinal: 2,
                    origin: DraftingRuleSelection::Profile(&selected),
                },
                DraftingSemanticSelectionRow::Rule {
                    ordinal: 3,
                    origin: DraftingRuleSelection::Design,
                },
                DraftingSemanticSelectionRow::Profile {
                    location: DraftingSemanticSelectionLocation::Semantic {
                        ordinal: 2,
                        field: DraftingSemanticSelectionField::LogicalInput { index: 5 },
                    },
                    selected: &selected,
                },
                DraftingSemanticSelectionRow::Profile {
                    location: DraftingSemanticSelectionLocation::Semantic {
                        ordinal: 2,
                        field: DraftingSemanticSelectionField::LogicalInput { index: 6 },
                    },
                    selected: &selected,
                },
                DraftingSemanticSelectionRow::Profile {
                    location: DraftingSemanticSelectionLocation::Semantic {
                        ordinal: 3,
                        field: DraftingSemanticSelectionField::ButtonholeButton,
                    },
                    selected: &selected,
                },
            ]
        };
        let row_count = rows.len();
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
        let occurrences = if empty { 1 } else { 14 };
        let passes = if empty {
            0
        } else if case == 1 {
            30
        } else {
            31
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
        let root = checked
            .root_input_scope(&root_scope)
            .map_err(|e| e.to_string())?
            .bind_root_input_members()
            .map_err(|e| e.to_string())?
            .bind_root_logical_suppliers()
            .map_err(|e| e.to_string())?
            .bind_root_numeric_suppliers()
            .map_err(|e| e.to_string())?;
        let mut headers = root
            .bind_profile_input_headers()
            .map_err(|e| e.to_string())?;
        let inputs = headers
            .bind_profile_input_parameters()
            .map_err(|e| e.to_string())?;

        let mut work = inputs.admit_rule_work().map_err(|e| e.to_string())?;
        let mut bound = work.bind_rule_sources().map_err(|e| e.to_string())?;
        let original_sources = bound.records().as_ptr();
        assert_eq!(
            bound
                .inputs()
                .root()
                .logical()
                .members()
                .owner()
                .selections()
                .reserved_child_passes(),
            if empty { 0 } else { 27 }
        );
        let queries = (
            source.calls.get(),
            source.parameter_calls.get(),
            source.rule_calls.get(),
        );
        source.base.walks.set(0);
        if case == 2 {
            source.base.extra_charge.set(100001);
        }
        if case == 3 || case == 4 {
            source.extra_after.set(Some(3));
            source.extra_entries.set(if case == 3 {
                100001
            } else {
                demand.collection_entries()
            });
        }
        let result = bound.bind_rule_signatures();
        assert_eq!(
            (
                source.calls.get(),
                source.parameter_calls.get(),
                source.rule_calls.get()
            ),
            queries
        );
        if case != 0 && !empty {
            let error = result.expect_err(
                "complete signatures refuse first original cause without successful prefix",
            );
            assert_eq!(error.signature_count(), 3);
            assert_eq!(error.sources().as_ptr(), original_sources);
            assert_eq!(error.inputs().operation_input_count(), 8);
            let cursor = error
                .inputs()
                .root()
                .logical()
                .members()
                .owner()
                .selections();
            if case == 1 {
                assert!(
                    matches!(error.refusal(),DraftingProgramRuleSignatureRefusal::Work(e) if matches!(e.refusal(),DraftingProgramSemanticWorkRefusal::ChildPasses {allowed:30,reserved:27,requested:4}))
                );
                assert_eq!(cursor.reserved_child_passes(), 27);
                assert_eq!(source.base.walks.get(), 0);
                assert!(error.demand().is_none());
            } else if case == 2 {
                assert!(
                    matches!(error.refusal(),DraftingProgramRuleSignatureRefusal::Resources(DesignResourceLimitError::Count {location,..}) if location.field=="numeric.extra")
                );
                assert_eq!(cursor.reserved_child_passes(), 31);
                assert_eq!(source.base.walks.get(), 1);
                assert!(error.demand().is_none());
            } else {
                let DraftingProgramRuleSignatureRefusal::Signature {
                    source_index,
                    cause,
                } = error.refusal()
                else {
                    return Err("original late signature cause".into());
                };
                let index = if case == 14 { 2 } else { 1 };
                assert_eq!(*source_index, index);
                let parent = error
                    .sources()
                    .get(index)
                    .ok_or("original refused source")?;
                assert!(std::ptr::eq(cause.parent(), parent.source()));
                assert!(error.demand().is_some());
                assert_eq!(cursor.reserved_child_passes(), 31);
                assert_eq!(source.base.walks.get(), if case == 14 { 4 } else { 3 });
                match case {
                    3 => assert!(
                        matches!(cause.refusal(),DraftingRuleSignatureRefusal::Work(e) if matches!(e.refusal(),DraftingRuleWorkRefusal::Resources(DesignResourceLimitError::Count {location,..}) if location.field=="rule.late-cache"))
                    ),
                    4 => assert!(
                        matches!(cause.refusal(),DraftingRuleSignatureRefusal::Work(e) if matches!(e.refusal(),DraftingRuleWorkRefusal::MatchingInspections {allowed,required} if *allowed==per_pass && required>allowed))
                    ),
                    5 => assert!(matches!(
                        cause.refusal(),
                        DraftingRuleSignatureRefusal::Mismatch(
                            DraftingRuleSignatureIssue::InputKind {
                                input: 1,
                                requirement: 5,
                                ..
                            }
                        )
                    )),
                    6 => assert!(matches!(
                        cause.refusal(),
                        DraftingRuleSignatureRefusal::Mismatch(
                            DraftingRuleSignatureIssue::MissingRole { input: 6 }
                        )
                    )),
                    7 => assert!(matches!(
                        cause.refusal(),
                        DraftingRuleSignatureRefusal::Mismatch(
                            DraftingRuleSignatureIssue::ResultKind {
                                expected: LogicalInputKind::PocketOpening,
                                actual: LogicalInputKind::NotionSize
                            }
                        )
                    )),
                    8 => assert!(matches!(
                        cause.refusal(),
                        DraftingRuleSignatureRefusal::Mismatch(
                            DraftingRuleSignatureIssue::InputCount {
                                inputs: 7,
                                requirements: 6
                            }
                        )
                    )),
                    9 => {
                        assert!(matches!(
                            cause.refusal(),
                            DraftingRuleSignatureRefusal::Output(
                                LogicalInputLookupError::Missing { .. }
                            )
                        ));
                        assert!(cause.output().is_none());
                    }
                    10 => assert!(
                        matches!(cause.refusal(),DraftingRuleSignatureRefusal::Output(LogicalInputLookupError::Kind {expected:LogicalInputKind::PocketOpening,actual}) if actual.definition().kind==LogicalInputKind::NotionSize)
                    ),
                    11 => assert!(matches!(
                        cause.refusal(),
                        DraftingRuleSignatureRefusal::OutputNotDerived
                    )),
                    12 => assert!(
                        matches!(cause.refusal(),DraftingRuleSignatureRefusal::DerivedOperation {expected,actual} if *expected==id(80) && *actual==id(79))
                    ),
                    14 => assert!(matches!(
                        cause.refusal(),
                        DraftingRuleSignatureRefusal::Mismatch(
                            DraftingRuleSignatureIssue::InputCount {
                                inputs: 0,
                                requirements: 1
                            }
                        )
                    )),
                    _ => return Err("unhandled signature case".into()),
                }
                if (10..=12).contains(&case) {
                    assert!(std::ptr::eq(
                        cause.output().ok_or("actual refused output")?,
                        garment
                            .inputs()
                            .inputs()
                            .logicals()
                            .declaration(id(91))
                            .ok_or("original output")?
                    ));
                }
            }
            continue;
        }
        let proof = result.map_err(|e| e.to_string())?;
        assert_eq!(proof.sources().as_ptr(), original_sources);
        assert_eq!(proof.records().len(), if empty { 0 } else { 3 });
        assert_eq!(proof.child_pass_count(), if empty { 0 } else { 4 });
        assert_eq!(proof.demand().is_some(), !empty);
        assert_eq!(source.base.walks.get(), if empty { 0 } else { 4 });
        assert_eq!(
            proof
                .inputs()
                .root()
                .logical()
                .members()
                .owner()
                .selections()
                .reserved_child_passes(),
            passes
        );
        for (index, record) in proof.records().iter().enumerate() {
            let original = proof.sources().get(index).ok_or("original source")?;
            let signature = record.signature();
            assert_eq!(record.source_index(), index);
            assert!(std::ptr::eq(record.source(), original));
            assert!(std::ptr::eq(signature.parent(), original.source()));
            assert_eq!(
                (
                    original.work().request().ordinal(),
                    original.work().request().position()
                ),
                match index {
                    0 => (1, 1),
                    1 => (2, 3),
                    2 => (3, 4),
                    _ => return Err("unexpected signature".into()),
                }
            );
            assert_eq!(signature.input_count(), if index == 1 { 7 } else { 0 });
            assert!(signature.input_match(signature.input_count()).is_none());
            if index == 2 {
                assert!(signature.output().is_none());
            } else {
                let output = signature.output().ok_or("actual Root output")?;
                assert!(std::ptr::eq(
                    output.inputs(),
                    garment.inputs().inputs().logicals()
                ));
                assert_eq!(output.index(), index);
                assert_eq!(
                    output.declaration().id(),
                    id(if index == 0 { 90 } else { 91 })
                );
                assert!(
                    matches!(output.declaration().definition().state,LogicalInputState::Derived {operation} if operation==id(if index==0 {79} else {80}))
                );
            }
            for input_index in 0..signature.input_count() {
                let pair = signature
                    .input_match(input_index)
                    .ok_or("complete original match")?;
                assert_eq!(
                    (pair.input_index(), pair.requirement_index()),
                    (input_index, 6 - input_index)
                );
                assert!(std::ptr::eq(
                    pair.operation(),
                    original.work().request().operation()
                ));
                assert!(std::ptr::eq(
                    pair.rule(),
                    original.source().source().declaration
                ));
                let SemanticOperation::LogicalDerivation(operation) = pair.operation() else {
                    return Err("actual logical operation".into());
                };
                let SemanticRuleSignature::Logical {
                    inputs: requirements,
                    ..
                } = &pair.rule().definition().signature
                else {
                    return Err("actual rule signature".into());
                };
                let DraftingRuleInputPair::Logical { input, requirement } = pair.pair() else {
                    return Err("actual named logical pair".into());
                };
                assert!(std::ptr::eq(
                    input,
                    operation
                        .definition()
                        .inputs
                        .get(input_index)
                        .ok_or("original input")?
                ));
                assert!(std::ptr::eq(
                    requirement,
                    requirements
                        .get(6 - input_index)
                        .ok_or("original requirement")?
                ));
            }
        }
        if !empty {
            let profile = proof
                .inputs()
                .records()
                .first()
                .ok_or("Profile input")?
                .source()
                .binding();
            assert!(std::ptr::eq(profile.inputs(), source.inputs.logicals()));
            assert!(
                matches!(profile.declaration().definition().state,LogicalInputState::Derived {operation} if operation==id(80))
            );
            assert!(!std::ptr::eq(
                profile.declaration(),
                garment
                    .inputs()
                    .inputs()
                    .logicals()
                    .declaration(id(90))
                    .ok_or("Root90")?
            ));
        }
        drop(proof);
        if !empty {
            let refusal = bound
                .bind_rule_signatures()
                .expect_err("spent complete cursor refuses before fresh work");
            assert!(matches!(
                refusal.refusal(),
                DraftingProgramRuleSignatureRefusal::Work(_)
            ));
            assert_eq!(source.base.walks.get(), 4);
            assert_eq!(
                (
                    source.calls.get(),
                    source.parameter_calls.get(),
                    source.rule_calls.get()
                ),
                queries
            );
        }
    }
    Ok(())
}
