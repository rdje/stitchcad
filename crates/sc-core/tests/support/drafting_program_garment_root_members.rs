//! Actual Root-owned Pocket/Closure members retain original fields,metadata and canonical owners.
use super::{id, limits, scope, Provider, RuleSourceView, TestResult};
use sc_core::{construction::*, design::*, name::MachineToken, ontology::*, recipe::*, value::*};
use sc_units::Length;
fn fixture(case: u8) -> TestResult<DesignGarment<super::EmptyHost>> {
    let canonical = CanonicalInputs::with_logicals(
        vec![LengthDeclaration::new(LengthDeclarationDefinition {
            id: id(10),
            source: id(1010),
            state: LengthState::Unknown {
                observation: id(1110),
            },
        })?],
        [
            (20, ScalarInputKind::Length),
            (30, ScalarInputKind::Boolean),
        ]
        .into_iter()
        .map(|(n, kind)| {
            Ok(ScalarInputDeclaration::new(
                ScalarInputDeclarationDefinition {
                    id: id(n),
                    source: id(1000 + n),
                    kind,
                    state: ScalarInputState::Unknown {
                        observation: id(1100 + n),
                    },
                },
            )?)
        })
        .collect::<TestResult<Vec<_>>>()?,
        LogicalInputs::new(
            [
                (90, LogicalInputKind::PocketOpening, "private_option"),
                (91, LogicalInputKind::NotionSize, "hardware"),
            ]
            .into_iter()
            .map(|(n, kind, domain)| {
                Ok(LogicalInputDeclaration::new(
                    LogicalInputDeclarationDefinition {
                        id: id(n),
                        source: id(1000 + n),
                        kind,
                        domain: vec![MachineToken::new(domain)?],
                        state: if case == 9 && n == 90 {
                            LogicalInputState::Derived { operation: id(80) }
                        } else {
                            LogicalInputState::Unknown {
                                observation: id(1100 + n),
                            }
                        },
                    },
                    LogicalInputLimits::new(1, 0, 14, 2),
                )?)
            })
            .collect::<TestResult<Vec<_>>>()?,
            LogicalInputInventoryLimits::new(2, 4),
        )?,
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
        (
            7103,
            CanonicalInputTarget::Scalar {
                declaration: id(30),
                kind: ScalarInputKind::Boolean,
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
    let kinds = if case == 8 {
        vec![]
    } else {
        vec![
            ClosureKind::HookAndBar {
                hook: NotionSize::Declaration(id(91)),
                bar: NotionSize::Declaration(id(91)),
            },
            ClosureKind::ButtonAndButtonhole {
                button: NotionSize::Declaration(id(if case == 4 { 90 } else { 91 })),
                hole: ButtonholeDerivation { operation: id(80) },
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
                length: ClosureLength::Formula(id(if case == 2 || case == 5 {
                    999
                } else if case == 6 {
                    7103
                } else {
                    7102
                })),
            },
        ]
    };
    let opening = if case == 8 {
        PocketOpening::Profile(ProfileParameterRef::new(id(700)))
    } else {
        PocketOpening::Declaration(id(if case == 3 {
            999
        } else if case == 7 {
            91
        } else {
            90
        }))
    };
    super::super::super::super::garment_source::garment_with_declarations(
        canonical, parameters, kinds, opening,
    )
}
#[test]
fn garment_root_input_members_preserve_actual_root_fields_parameter_provenance_and_cursor(
) -> TestResult {
    for case in 0..10u8 {
        let garment = fixture(case)?;
        let other = fixture(case)?;
        let source = Provider::new(&garment, other.inputs(), 0)?;
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
        let steps = vec![DraftingProgramStep::Drafting(
            DraftingOperationInput::PlaceGrainline(&grainline),
        )];

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
        let rows = if case == 8 {
            vec![DraftingSemanticSelectionRow::Profile {
                location: DraftingSemanticSelectionLocation::Pocket {
                    pocket: 0,
                    field: DraftingPocketSelectionField::Opening,
                },
                selected: &selected,
            }]
        } else {
            vec![]
        };
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
        let occurrences = garment.pocket_closure_dependencies().count() + 1;
        let passes = if case == 8 {
            0
        } else if case == 1 {
            6
        } else {
            7
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
        assert_eq!(
            signatures
                .inputs()
                .root()
                .logical()
                .members()
                .owner()
                .selections()
                .reserved_child_passes(),
            0
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
        let result = signatures.bind_garment_root_input_members();
        assert_eq!(
            (
                source.calls.get(),
                source.parameter_calls.get(),
                source.rule_calls.get()
            ),
            queries
        );
        if (1..=7).contains(&case) {
            let error =
                result.expect_err("whole Root member class refuses without successful prefix");
            assert_eq!(error.member_count(), Some(7));
            assert!(std::ptr::eq(
                error
                    .owner()
                    .inputs()
                    .root()
                    .logical()
                    .members()
                    .owner()
                    .inputs(),
                &actual
            ));
            let cursor = error
                .owner()
                .inputs()
                .root()
                .logical()
                .members()
                .owner()
                .selections();
            if case == 1 {
                assert!(
                    matches!(error.refusal(),DraftingProgramGarmentRootInputMemberRefusal::Work(cause) if matches!(cause.refusal(),DraftingProgramSemanticWorkRefusal::ChildPasses {allowed:6,reserved:0,requested:7}))
                );
                assert_eq!(cursor.reserved_child_passes(), 0);
                assert_eq!(source.base.walks.get(), 0);
                assert!(error.demand().is_none());
            } else if case == 2 {
                assert!(
                    matches!(error.refusal(),DraftingProgramGarmentRootInputMemberRefusal::Resources(DesignResourceLimitError::Count {location,..}) if location.field=="numeric.extra")
                );
                assert_eq!(cursor.reserved_child_passes(), 7);
                assert_eq!(source.base.walks.get(), 1);
                assert!(error.demand().is_none());
            } else {
                assert_eq!(cursor.reserved_child_passes(), 7);
                assert_eq!(source.base.walks.get(), 1);
                assert!(error.demand().is_some());
                match error.refusal() {
                    DraftingProgramGarmentRootInputMemberRefusal::Lookup { use_, cause } => {
                        assert!(std::ptr::eq(use_.garment(), &garment));
                        assert!(std::ptr::eq(cause.scope(), &root_scope));
                        if case == 3 {
                            assert!(matches!(
                                cause.cause(),
                                DraftingDesignSemanticLookupCause::Logical(
                                    LogicalInputLookupError::Missing { .. }
                                )
                            ));
                        } else {
                            assert!(
                                matches!(cause.cause(),DraftingDesignSemanticLookupCause::Logical(LogicalInputLookupError::Kind {actual,..}) if std::ptr::eq(actual,garment.inputs().inputs().logicals().declaration(id(if case==4 {90} else {91})).ok_or("original wrong-kind member")?))
                            );
                        }
                        assert_eq!(
                            use_.selection_location(),
                            if case == 4 {
                                DraftingSemanticSelectionLocation::Closure {
                                    closure: 1,
                                    field: DraftingClosureSelectionField::Size(
                                        DraftingClosureSizeRole::Button,
                                    ),
                                }
                            } else {
                                DraftingSemanticSelectionLocation::Pocket {
                                    pocket: 0,
                                    field: DraftingPocketSelectionField::Opening,
                                }
                            }
                        );
                    }
                    DraftingProgramGarmentRootInputMemberRefusal::MissingParameter {
                        use_,
                        identity,
                    } => {
                        assert_eq!(case, 5);
                        assert_eq!(*identity, id(999));
                        assert_eq!(
                            use_.selection_location(),
                            DraftingSemanticSelectionLocation::Closure {
                                closure: 4,
                                field: DraftingClosureSelectionField::Length
                            }
                        );
                    }
                    DraftingProgramGarmentRootInputMemberRefusal::FormulaKind {
                        use_,
                        parameter,
                        metadata,
                    } => {
                        assert_eq!(case, 6);
                        assert!(std::ptr::eq(
                            *parameter,
                            garment
                                .inputs()
                                .parameters()
                                .parameter(id(7103))
                                .ok_or("actual bad Length metadata")?
                        ));
                        assert_eq!(metadata.kind(), FormulaKind::Boolean);
                        assert_eq!(
                            use_.selection_location(),
                            DraftingSemanticSelectionLocation::Closure {
                                closure: 4,
                                field: DraftingClosureSelectionField::Length
                            }
                        );
                    }
                    _ => return Err("unexpected original Root cause".into()),
                }
            }
            continue;
        }
        let proof = result.map_err(|e| e.to_string())?;
        assert_eq!(proof.records().len(), if case == 8 { 0 } else { 7 });
        assert_eq!(proof.child_pass_count(), passes);
        assert_eq!(proof.demand(), if case == 8 { None } else { Some(demand) });
        assert_eq!(source.base.walks.get(), usize::from(case != 8));
        assert_eq!(
            proof
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
        let originals: Vec<_> = garment
            .pocket_closure_dependencies()
            .filter(|u| {
                matches!(
                    u.field(),
                    DraftingGarmentSemanticField::PocketOpening(PocketOpening::Declaration(_))
                        | DraftingGarmentSemanticField::ClosureSize {
                            source: NotionSize::Declaration(_),
                            ..
                        }
                        | DraftingGarmentSemanticField::ClosureLength(_)
                )
            })
            .collect();
        assert_eq!(originals.len(), proof.records().len());
        for (index, record) in proof.records().iter().enumerate() {
            let occurrence = record.occurrence();
            let original = originals
                .get(index)
                .ok_or("complete original input occurrence")?;
            assert_eq!(
                occurrence.selection_location(),
                original.selection_location()
            );
            assert!(std::ptr::eq(occurrence.garment(), &garment));
            match (record.source(), original.field()) {
                (
                    DraftingProgramGarmentRootInputSource::Logical(member),
                    DraftingGarmentSemanticField::PocketOpening(PocketOpening::Declaration(
                        identity,
                    )),
                )
                | (
                    DraftingProgramGarmentRootInputSource::Logical(member),
                    DraftingGarmentSemanticField::ClosureSize {
                        source: NotionSize::Declaration(identity),
                        ..
                    },
                ) => {
                    assert!(std::ptr::eq(member.scope(), &root_scope));
                    assert!(std::ptr::eq(
                        member.binding().inputs(),
                        garment.inputs().inputs().logicals()
                    ));
                    assert!(std::ptr::eq(
                        member.binding().declaration(),
                        garment
                            .inputs()
                            .inputs()
                            .logicals()
                            .declaration(*identity)
                            .ok_or("actual original member")?
                    ));
                    if case == 9 && index == 0 {
                        assert!(
                            matches!(member.binding().declaration().definition().state,LogicalInputState::Derived {operation} if operation==id(80))
                        );
                    }
                }
                (
                    DraftingProgramGarmentRootInputSource::ExplicitLength { parameter, value },
                    DraftingGarmentSemanticField::ClosureLength(ClosureLength::Explicit {
                        parameter: actual,
                        value: actual_value,
                    }),
                ) => {
                    assert!(std::ptr::eq(*parameter, actual));
                    assert!(std::ptr::eq(*value, actual_value));
                    assert!(garment
                        .inputs()
                        .parameters()
                        .parameter(**parameter)
                        .is_none());
                }
                (
                    DraftingProgramGarmentRootInputSource::FormulaLength {
                        parameter,
                        metadata,
                        source: member,
                    },
                    DraftingGarmentSemanticField::ClosureLength(ClosureLength::Formula(identity)),
                ) => {
                    assert!(std::ptr::eq(
                        *parameter,
                        garment
                            .inputs()
                            .parameters()
                            .parameter(*identity)
                            .ok_or("original parameter")?
                    ));
                    assert_eq!(metadata.kind(), FormulaKind::Length);
                    assert!(std::ptr::eq(member.scope(), &root_scope));
                    assert!(std::ptr::eq(
                        member.binding().inputs(),
                        garment.inputs().inputs()
                    ));
                    assert_eq!(member.binding().target(), parameter.definition().target);
                    assert_eq!(
                        member.binding().target(),
                        if index == 5 {
                            CanonicalInputTarget::Length(id(10))
                        } else {
                            CanonicalInputTarget::Scalar {
                                declaration: id(20),
                                kind: ScalarInputKind::Length,
                            }
                        }
                    );
                }
                _ => return Err("original Root source class changed".into()),
            }
        }
        if case == 0 {
            let hook = proof.records().get(1).ok_or("hook")?;
            let bar = proof.records().get(2).ok_or("bar")?;
            let (
                DraftingProgramGarmentRootInputSource::Logical(a),
                DraftingProgramGarmentRootInputSource::Logical(b),
            ) = (hook.source(), bar.source())
            else {
                return Err("actual sizes".into());
            };
            assert!(std::ptr::eq(
                a.binding().declaration(),
                b.binding().declaration()
            ));
            assert_ne!(
                hook.occurrence().selection_location(),
                bar.occurrence().selection_location()
            );
            assert!(!format!("{:?}", proof.records()).contains("15000"));
        }
    }
    Ok(())
}
