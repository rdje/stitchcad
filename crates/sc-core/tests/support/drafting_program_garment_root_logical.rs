//! Genuine Root garment suppliers retain positions without fabricating a consumer order.
use super::{id, limits, scope, Provider, RuleSourceView, TestResult};
use sc_core::{construction::*, design::*, name::MachineToken, ontology::*, recipe::*, value::*};
use sc_units::Length;
fn fixture(case: u8) -> TestResult<DesignGarment<super::EmptyHost>> {
    let empty = case == 7;
    let other_output = case == 2 || case == 3 || case == 5 || case == 6;
    let logicals = [
        (
            90,
            LogicalInputKind::PocketOpening,
            if case == 2 || case == 3 { 999 } else { 80 },
        ),
        (
            91,
            if case == 6 {
                LogicalInputKind::NotionSize
            } else {
                LogicalInputKind::PocketOpening
            },
            80,
        ),
        (92, LogicalInputKind::NotionSize, 81),
    ]
    .into_iter()
    .map(|(n, kind, operation)| {
        Ok(LogicalInputDeclaration::new(
            LogicalInputDeclarationDefinition {
                id: id(n),
                source: id(1000 + n),
                kind,
                domain: vec![MachineToken::new("private_option")?],
                state: if empty || (n == 91 && !other_output) {
                    LogicalInputState::Unknown {
                        observation: id(1100 + n),
                    }
                } else {
                    LogicalInputState::Derived {
                        operation: id(operation),
                    }
                },
            },
            LogicalInputLimits::new(1, 0, 14, 2),
        )?)
    })
    .collect::<TestResult<Vec<_>>>()?;
    let canonical = CanonicalInputs::with_logicals(
        vec![LengthDeclaration::new(LengthDeclarationDefinition {
            id: id(10),
            source: id(1010),
            state: LengthState::Unknown {
                observation: id(1110),
            },
        })?],
        vec![ScalarInputDeclaration::new(
            ScalarInputDeclarationDefinition {
                id: id(20),
                source: id(1020),
                kind: ScalarInputKind::Length,
                state: ScalarInputState::Unknown {
                    observation: id(1120),
                },
            },
        )?],
        LogicalInputs::new(logicals, LogicalInputInventoryLimits::new(3, 9))?,
    )?;
    let parameters = [
        (7101, CanonicalInputTarget::Length(id(10))),
        (
            7102,
            CanonicalInputTarget::Scalar {
                declaration: id(20),
                kind: ScalarInputKind::Length,
            },
        ),
    ]
    .into_iter()
    .enumerate()
    .map(|(index, (n, target))| {
        Ok(DesignParameterDefinition {
            id: id(n),
            token: MachineToken::new(format!("zipper_{index}"))?,
            target,
        })
    })
    .collect::<TestResult<Vec<_>>>()?;
    let kinds = vec![
        ClosureKind::HookAndBar {
            hook: NotionSize::Declaration(id(92)),
            bar: NotionSize::Declaration(id(92)),
        },
        ClosureKind::ButtonAndButtonhole {
            button: NotionSize::Declaration(id(92)),
            hole: ButtonholeDerivation { operation: id(880) },
        },
        ClosureKind::CentredZipper {
            length: ClosureLength::Explicit {
                parameter: id(7100),
                value: Length::from_micrometres(15000)?,
            },
        },
        ClosureKind::CentredZipper {
            length: ClosureLength::Formula(id(7101)),
        },
        ClosureKind::CentredZipper {
            length: ClosureLength::Formula(id(7102)),
        },
    ];
    let semantic = if empty {
        SemanticRecipe::empty()
    } else {
        SemanticRecipe::new(
            vec![
                SemanticOperation::LogicalDerivation(LogicalDerivationOperation::new(
                    LogicalDerivationDefinition {
                        id: id(80),
                        output: LogicalInputTarget {
                            declaration: id(if other_output { 91 } else { 90 }),
                            kind: if case == 6 {
                                LogicalInputKind::NotionSize
                            } else {
                                LogicalInputKind::PocketOpening
                            },
                        },
                        rule: id(901),
                        inputs: vec![],
                    },
                    LogicalDerivationLimits::new(0, 0, 0),
                )?),
                if case == 4 {
                    SemanticOperation::Buttonhole(ButtonholeOperation::new(
                        ButtonholeOperationDefinition {
                            id: id(81),
                            button: NotionSize::Profile(ProfileParameterRef::new(id(703))),
                            policy: id(902),
                            inputs: vec![],
                        },
                        ButtonholeOperationLimits::new(0, 0, 0),
                    )?)
                } else {
                    SemanticOperation::LogicalDerivation(LogicalDerivationOperation::new(
                        LogicalDerivationDefinition {
                            id: id(81),
                            output: LogicalInputTarget {
                                declaration: id(92),
                                kind: LogicalInputKind::NotionSize,
                            },
                            rule: id(902),
                            inputs: vec![],
                        },
                        LogicalDerivationLimits::new(0, 0, 0),
                    )?)
                },
            ],
            SemanticRecipeLimits::new(2, 4),
        )?
    };
    super::super::super::super::garment_source::garment_with_semantic_declarations(
        canonical,
        parameters,
        kinds,
        PocketOpening::Declaration(id(90)),
        semantic,
    )
}
#[test]
fn garment_root_logical_suppliers_keep_actual_outputs_signatures_positions_and_unordered_consumers(
) -> TestResult {
    for case in 0..9u8 {
        let garment = fixture(case)?;
        let other = fixture(case)?;
        let mut source = Provider::new(&garment, other.inputs(), 0)?;
        if case != 7 {
            source.rules = [901, 902]
                .into_iter()
                .map(|n| {
                    Ok(super::super::RuleRow {
                        declaration: SemanticRuleDeclaration::new(
                            SemanticRuleDefinition {
                                id: id(n),
                                signature: if case == 4 && n == 902 {
                                    SemanticRuleSignature::Buttonhole { inputs: vec![] }
                                } else {
                                    SemanticRuleSignature::Logical {
                                        result: if n == 901 && case != 6 {
                                            LogicalInputKind::PocketOpening
                                        } else {
                                            LogicalInputKind::NotionSize
                                        },
                                        inputs: vec![],
                                    }
                                },
                            },
                            SemanticRuleLimits::new(0, 0, 0),
                        )?,
                        body: DraftingSourceRecord::new([1, 2, 3, 4]),
                    })
                })
                .collect::<TestResult<Vec<_>>>()?;
        }
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
        let intervening = GrainlinePlacement::new(id(80001), id(90000))?;
        let mut steps = vec![DraftingProgramStep::Drafting(
            DraftingOperationInput::PlaceGrainline(&grainline),
        )];
        if case != 7 {
            steps.push(DraftingProgramStep::Semantic { ordinal: 1 });
            if case == 8 {
                steps.push(DraftingProgramStep::Drafting(
                    DraftingOperationInput::PlaceGrainline(&intervening),
                ));
            }
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
        let sources = namespace
            .check_root_sources(FormulaSourceCheckLimits::new(0, 0, 0, 0))
            .map_err(|e| e.cause().to_string())?;
        let dependencies = sources
            .proof()
            .prepare_dependencies(FormulaSourceDependencyLimits::new(0, 0, 1))
            .map_err(|e| e.to_string())?;
        let graph = dependencies
            .admit_acyclic(FormulaSourceCycleLimits::new(0, 0))
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
        let mut rows = vec![];
        if case != 7 {
            for ordinal in [1, 2] {
                rows.push(DraftingSemanticSelectionRow::Rule {
                    ordinal,
                    origin: DraftingRuleSelection::Design,
                });
            }
        }
        if case == 4 {
            rows.push(DraftingSemanticSelectionRow::Profile {
                location: DraftingSemanticSelectionLocation::Semantic {
                    ordinal: 2,
                    field: DraftingSemanticSelectionField::ButtonholeButton,
                },
                selected: &selected,
            });
        }
        let row_count = rows.len();
        let raw_selections = DraftingSemanticSelectionInputs::new(rows);
        let selections = raw_selections
            .admit(DraftingSemanticSelectionLimits::new(content, 1000))
            .map_err(|e| e.to_string())?;
        let actual = geometry
            .semantic_inputs(&admitted_garment, &selections)
            .map_err(|e| e.to_string())?;
        let mut meter = DesignResourceMeter::new(content);
        actual.charge_resources(&mut meter)?;
        let demand = meter.finish()?;
        let occurrences = garment.pocket_closure_dependencies().count()
            + 1
            + usize::from(case == 8)
            + if case == 7 { 0 } else { 4 };
        let before = if case == 7 {
            7
        } else if case == 4 {
            20
        } else {
            16
        };
        let passes = if case == 1 {
            19
        } else {
            before + if case == 7 { 0 } else { 4 }
        };
        let per_pass = (demand.collection_entries() + 8).pow(2);
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
        let signatures = bound.bind_rule_signatures().map_err(|e| e.to_string())?;
        let members = signatures
            .bind_garment_root_input_members()
            .map_err(|e| e.to_string())?;
        assert_eq!(members.records().len(), 7);
        assert_eq!(
            members
                .owner()
                .inputs()
                .root()
                .logical()
                .members()
                .owner()
                .selections()
                .reserved_child_passes(),
            before
        );
        source.base.walks.set(0);
        let queries = (
            source.calls.get(),
            source.parameter_calls.get(),
            source.rule_calls.get(),
        );
        if case == 2 {
            source.base.extra_charge.set(100001);
        }
        let result = members.bind_garment_root_logical_suppliers();
        assert_eq!(
            (
                source.calls.get(),
                source.parameter_calls.get(),
                source.rule_calls.get()
            ),
            queries
        );
        if (1..=6).contains(&case) {
            let error = result
                .expect_err("complete original Root suppliers refuse without accepted prefix");
            assert_eq!(error.supplier_count(), Some(4));
            assert_eq!(error.owner().records().len(), 7);
            let cursor = error
                .owner()
                .owner()
                .inputs()
                .root()
                .logical()
                .members()
                .owner()
                .selections();
            if case == 1 {
                assert!(
                    matches!(error.refusal(),DraftingProgramGarmentRootLogicalSupplierRefusal::Work(cause) if matches!(cause.refusal(),DraftingProgramSemanticWorkRefusal::ChildPasses {allowed:19,reserved:16,requested:4}))
                );
                assert_eq!(cursor.reserved_child_passes(), 16);
                assert_eq!(source.base.walks.get(), 0);
                assert!(error.demand().is_none());
            } else if case == 2 {
                assert!(
                    matches!(error.refusal(),DraftingProgramGarmentRootLogicalSupplierRefusal::Resources(DesignResourceLimitError::Count {location,..}) if location.field=="numeric.extra")
                );
                assert_eq!(cursor.reserved_child_passes(), passes);
                assert_eq!(source.base.walks.get(), 1);
                assert!(error.demand().is_none());
            } else {
                assert_eq!(cursor.reserved_child_passes(), passes);
                assert_eq!(source.base.walks.get(), 1);
                assert!(error.demand().is_some());
                match error.refusal() {
                    DraftingProgramGarmentRootLogicalSupplierRefusal::Lookup {
                        member_index,
                        cause,
                    } => {
                        assert_eq!(*member_index, usize::from(case == 4));
                        assert!(std::ptr::eq(cause.scope(), &root_scope));
                        if case == 3 {
                            assert!(matches!(
                                cause.cause(),
                                DraftingDesignSemanticLookupCause::Operation(
                                    SemanticOperationLookupError::Missing { .. }
                                )
                            ));
                        } else {
                            assert_eq!(case, 4);
                            assert!(
                                matches!(cause.cause(),DraftingDesignSemanticLookupCause::Operation(SemanticOperationLookupError::Kind {actual,..}) if matches!(actual,SemanticOperation::Buttonhole(_)))
                            );
                        }
                    }
                    DraftingProgramGarmentRootLogicalSupplierRefusal::Output {
                        member_index,
                        source: actual,
                    } => {
                        assert_eq!(*member_index, 0);
                        assert!(case == 5 || case == 6);
                        assert!(std::ptr::eq(
                            actual.operation(),
                            garment
                                .inputs()
                                .semantic_recipe()
                                .operations()
                                .first()
                                .ok_or("actual wrong output supplier")?
                        ));
                        let SemanticOperation::LogicalDerivation(op) = actual.operation() else {
                            return Err("actual logical supplier".into());
                        };
                        assert_eq!(op.definition().output.declaration, id(91));
                        assert_eq!(
                            op.definition().output.kind,
                            if case == 6 {
                                LogicalInputKind::NotionSize
                            } else {
                                LogicalInputKind::PocketOpening
                            }
                        );
                    }
                    _ => return Err("unexpected original supplier cause".into()),
                }
            }
            continue;
        }
        let proof = result.map_err(|e| e.to_string())?;
        assert_eq!(proof.records().len(), if case == 7 { 0 } else { 4 });
        assert_eq!(proof.child_pass_count(), if case == 7 { 0 } else { 4 });
        assert_eq!(proof.demand(), if case == 7 { None } else { Some(demand) });
        assert_eq!(source.base.walks.get(), usize::from(case != 7));
        assert_eq!(
            proof
                .members()
                .owner()
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
            assert_eq!(record.member_index(), index);
            let member = proof
                .members()
                .records()
                .get(index)
                .ok_or("original Root consumer")?;
            let DraftingProgramGarmentRootInputSource::Logical(actual) = member.source() else {
                return Err("actual logical member".into());
            };
            let target = actual.binding().declaration();
            assert_eq!(record.output().declaration, target.id());
            assert_eq!(record.output().kind, target.definition().kind);
            assert!(std::ptr::eq(record.source().scope(), &root_scope));
            let original = garment
                .inputs()
                .semantic_recipe()
                .operations()
                .get(usize::from(index != 0))
                .ok_or("original Root supplier")?;
            assert!(std::ptr::eq(record.source().operation(), original));
            let SemanticOperation::LogicalDerivation(operation) = original else {
                return Err("actual logical operation".into());
            };
            assert!(std::ptr::eq(
                record.output(),
                &operation.definition().output
            ));
            let expected = if index == 0 {
                (1, 2, 0)
            } else if case == 8 {
                (2, 4, 1)
            } else {
                (2, 3, 1)
            };
            assert_eq!(
                (
                    record.ordinal(),
                    record.position(),
                    record.signature_index()
                ),
                expected
            );
            let signature = proof
                .members()
                .owner()
                .records()
                .get(record.signature_index())
                .ok_or("actual complete original signature")?;
            assert!(std::ptr::eq(
                signature.source().work().request().operation(),
                original
            ));
            assert!(std::ptr::eq(
                signature
                    .signature()
                    .output()
                    .ok_or("actual supplier output")?
                    .declaration(),
                target
            ));
            assert_eq!(
                member.occurrence().selection_location(),
                match index {
                    0 => DraftingSemanticSelectionLocation::Pocket {
                        pocket: 0,
                        field: DraftingPocketSelectionField::Opening
                    },
                    1 | 2 => DraftingSemanticSelectionLocation::Closure {
                        closure: 0,
                        field: DraftingClosureSelectionField::Size(if index == 1 {
                            DraftingClosureSizeRole::Hook
                        } else {
                            DraftingClosureSizeRole::Bar
                        })
                    },
                    3 => DraftingSemanticSelectionLocation::Closure {
                        closure: 1,
                        field: DraftingClosureSelectionField::Size(DraftingClosureSizeRole::Button)
                    },
                    _ => return Err("unexpected original consumer".into()),
                }
            );
        }
    }
    Ok(())
}
