//! Exact original Profile garment header parents under checked original direct row/pin loans.
use super::{id, limits, scope, Provider, RuleSourceView, TestResult};
use sc_core::{construction::*, design::*, name::MachineToken, ontology::*, recipe::*, value::*};
use sc_units::Length;
pub(super) fn fixture(case: u8) -> TestResult<DesignGarment<super::EmptyHost>> {
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
        LogicalInputs::new(
            [
                (90, LogicalInputKind::PocketOpening),
                (91, LogicalInputKind::NotionSize),
            ]
            .into_iter()
            .map(|(n, kind)| {
                Ok(LogicalInputDeclaration::new(
                    LogicalInputDeclarationDefinition {
                        id: id(n),
                        source: id(1000 + n),
                        kind,
                        domain: vec![MachineToken::new("private_option")?],
                        state: LogicalInputState::Unknown {
                            observation: id(1100 + n),
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
    let size = if case == 1 {
        NotionSize::Declaration(id(91))
    } else {
        NotionSize::Profile(ProfileParameterRef::new(id(703)))
    };
    let kinds = vec![
        ClosureKind::HookAndBar {
            hook: size,
            bar: size,
        },
        ClosureKind::ButtonAndButtonhole {
            button: if case == 10 {
                NotionSize::Profile(ProfileParameterRef::new(id(999)))
            } else {
                size
            },
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
        ClosureKind::CentredZipper {
            length: ClosureLength::Profile(ProfileParameterRef::new(id(704))),
        },
    ];
    let opening = if case == 1 {
        PocketOpening::Declaration(id(90))
    } else {
        PocketOpening::Profile(ProfileParameterRef::new(id(702)))
    };
    super::super::super::super::garment_source::garment_with_declarations(
        canonical, parameters, kinds, opening,
    )
}
#[test]
fn garment_profile_logical_headers_keep_actual_fields_direct_rows_pins_sources_and_cursor(
) -> TestResult {
    for case in 0..11u8 {
        let garment = fixture(case)?;
        let other = fixture(case)?;
        let source = Provider::new(
            &garment,
            other.inputs(),
            match case {
                4 => 3,
                5 => 5,
                6 => 6,
                7 => 7,
                _ => 0,
            },
        )?;
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
        let wrong = super::super::pin("other")?;
        let wrong_schema = SelectedProfilePin::new(SelectedProfilePinDefinition {
            profile_id: "original".into(),
            schema_version: "altered".into(),
            content_version: "content".into(),
        })?;
        let requested = if case == 8 { &wrong } else { &selected };
        use DraftingClosureSelectionField as C;
        use DraftingSemanticSelectionLocation as L;
        let mut rows = vec![DraftingSemanticSelectionRow::Profile {
            location: L::Closure {
                closure: 5,
                field: C::Length,
            },
            selected: requested,
        }];
        if case != 1 {
            for (location, selected) in [
                (
                    L::Closure {
                        closure: 0,
                        field: C::Size(DraftingClosureSizeRole::Bar),
                    },
                    requested,
                ),
                (
                    L::Pocket {
                        pocket: 0,
                        field: DraftingPocketSelectionField::Opening,
                    },
                    requested,
                ),
                (
                    L::Closure {
                        closure: 1,
                        field: C::Size(DraftingClosureSizeRole::Button),
                    },
                    if case == 9 { &wrong_schema } else { requested },
                ),
                (
                    L::Closure {
                        closure: 0,
                        field: C::Size(DraftingClosureSizeRole::Hook),
                    },
                    requested,
                ),
            ] {
                rows.push(DraftingSemanticSelectionRow::Profile { location, selected });
            }
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
        let occurrences = garment.pocket_closure_dependencies().count() + 1;
        let before = if case == 1 { 7 } else { 3 };
        let passes = if case == 1 || case == 2 { 7 } else { 8 };
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
        let root = signatures
            .bind_garment_root_input_members()
            .map_err(|e| e.to_string())?
            .bind_garment_root_logical_suppliers()
            .map_err(|e| e.to_string())?
            .bind_garment_root_numeric_suppliers()
            .map_err(|e| e.to_string())?;
        assert_eq!(root.logical().members().records().len(), before);
        assert_eq!(
            root.logical()
                .members()
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
        source.calls.set(0);
        source.parameter_calls.set(0);
        source.base.walks.set(0);
        if case == 3 {
            source.base.extra_charge.set(100001);
        }
        let result = root.bind_garment_profile_logical_headers();
        assert_eq!(
            (source.parameter_calls.get(), source.rule_calls.get()),
            (0, 0)
        );
        if (2..=9).contains(&case) {
            let error = result.expect_err(
                "complete logical Profile header class refuses without successful prefix",
            );
            assert_eq!(error.header_count(), Some(4));
            let cursor = error
                .owner()
                .logical()
                .members()
                .owner()
                .inputs()
                .root()
                .logical()
                .members()
                .owner()
                .selections();
            if case == 2 {
                assert!(
                    matches!(error.refusal(),DraftingProgramGarmentProfileLogicalHeaderRefusal::Work(cause) if matches!(cause.refusal(),DraftingProgramSemanticWorkRefusal::ChildPasses {allowed:7,reserved:3,requested:5}))
                );
                assert_eq!(cursor.reserved_child_passes(), 3);
                assert_eq!((source.base.walks.get(), source.calls.get()), (0, 0));
                assert!(error.demand().is_none());
            } else if case == 3 {
                assert!(
                    matches!(error.refusal(),DraftingProgramGarmentProfileLogicalHeaderRefusal::Resources(DesignResourceLimitError::Count {location,..}) if location.field=="numeric.extra")
                );
                assert_eq!(cursor.reserved_child_passes(), 8);
                assert_eq!((source.base.walks.get(), source.calls.get()), (1, 0));
                assert!(error.demand().is_none());
            } else {
                let DraftingProgramGarmentProfileLogicalHeaderRefusal::Source { request, cause } =
                    error.refusal()
                else {
                    return Err("expected original Profile source cause".into());
                };
                assert_eq!(
                    request.row_index(),
                    if case == 4 {
                        4
                    } else if case == 9 {
                        3
                    } else {
                        2
                    }
                );
                assert!(std::ptr::eq(request.occurrence().garment(), &garment));
                assert!(std::ptr::eq(cause.selected(), request.selected()));
                assert!(error.demand().is_some());
                assert_eq!(cursor.reserved_child_passes(), 8);
                let expected = match case {
                    4 | 8 => Some(DraftingProfileSourceIssue::SelectedPin(
                        SelectedProfilePinField::ProfileId,
                    )),
                    5 => Some(DraftingProfileSourceIssue::Root),
                    6 => Some(DraftingProfileSourceIssue::Snapshot),
                    9 => Some(DraftingProfileSourceIssue::SelectedPin(
                        SelectedProfilePinField::SchemaVersion,
                    )),
                    7 => None,
                    _ => return Err("unhandled mode".into()),
                };
                if let Some(expected) = expected {
                    assert!(
                        matches!(cause.refusal(),DraftingProfileSourceRefusal::Mismatch(actual) if *actual==expected)
                    );
                    assert!(cause.returned().is_some());
                } else {
                    assert!(matches!(
                        cause.refusal(),
                        DraftingProfileSourceRefusal::Source(super::SourceError::Missing)
                    ));
                }
                let queries = if case == 4 {
                    2
                } else if case == 9 {
                    4
                } else {
                    1
                };
                assert_eq!(
                    (source.base.walks.get(), source.calls.get()),
                    (queries + 1, queries)
                );
            }
            continue;
        }
        let proof = result.map_err(|e| e.to_string())?;
        let count = if case == 1 { 0 } else { 4 };
        assert_eq!(proof.records().len(), count);
        assert_eq!(proof.child_pass_count(), if count == 0 { 0 } else { 5 });
        assert_eq!(proof.demand(), if count == 0 { None } else { Some(demand) });
        assert_eq!(
            (source.base.walks.get(), source.calls.get()),
            if count == 0 { (0, 0) } else { (5, 4) }
        );
        assert_eq!(
            proof
                .root()
                .logical()
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
        for (index, header) in proof.records().iter().enumerate() {
            let request = header.request();
            assert_eq!(
                request.row_index(),
                match index {
                    0 => 2,
                    1 => 4,
                    2 => 1,
                    3 => 3,
                    _ => return Err("unexpected header".into()),
                }
            );
            assert!(std::ptr::eq(request.selected(), requested));
            assert!(std::ptr::eq(request.occurrence().garment(), &garment));
            assert_eq!(
                request.kind(),
                if index == 0 {
                    LogicalInputKind::PocketOpening
                } else {
                    LogicalInputKind::NotionSize
                }
            );
            assert_eq!(
                request.parameter().id(),
                id(if index == 0 {
                    702
                } else if case == 10 && index == 3 {
                    999
                } else {
                    703
                })
            );
            let raw = header.source().source();
            assert!(std::ptr::eq(raw.root, garment.inputs()));
            assert!(std::ptr::eq(raw.selected, &source.selected));
            assert!(!std::ptr::eq(raw.selected, request.selected()));
            assert!(std::ptr::eq(raw.inputs, &source.inputs));
            let body: &DraftingSourceRecord<[u8]> = &source.body;
            assert!(raw.profile.is_same_record(body));
            assert!(raw.snapshot.is_same_record(source.base.snapshot()));
            assert_ne!(request.parameter().id(), id(704));
        }
        if count > 0 {
            let hook = proof.records().get(1).ok_or("actual hook parent")?;
            let bar = proof.records().get(2).ok_or("actual bar parent")?;
            assert!(!std::ptr::eq(hook.source(), bar.source()));
            assert_eq!(hook.request().parameter(), bar.request().parameter());
            assert_ne!(
                hook.request().occurrence().selection_location(),
                bar.request().occurrence().selection_location()
            );
        }
    }
    Ok(())
}
