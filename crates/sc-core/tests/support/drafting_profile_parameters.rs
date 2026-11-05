//! Actual parameter/header/member loans borrow stable parent storage and one finite cursor.
use super::{id, limits, scope, ParameterSourceView, Provider, TestResult};
use sc_core::{construction::*, design::*, ontology::*, recipe::*, value::*};
#[test]
fn profile_input_parameters_retain_actual_parent_metadata_members_states_and_cursor() -> TestResult
{
    // Repeated external Derived parameters,short remaining cursor,fresh resource refusal,
    // late canonical kind,parameter,source pin,inventory refusals,missing source,Unknown Notion,N0.
    for case in 0..10u8 {
        let count = if case == 9 { 0 } else { 2 };
        let garment =
            super::super::root_numeric_suppliers::fixture(false, false, count, case == 8)?;
        let other = super::super::root_numeric_suppliers::fixture(false, false, count, case == 8)?;
        let source = Provider::new(&garment, other.inputs(), 0)?;
        let source_view: &ParameterSourceView<'_> = &source;
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

        if case == 8 {
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

        let mut rows = vec![DraftingSemanticSelectionRow::Rule {
            ordinal: 1,
            origin: DraftingRuleSelection::Design,
        }];
        for index in 0..count {
            rows.push(DraftingSemanticSelectionRow::Profile {
                location: DraftingSemanticSelectionLocation::Semantic {
                    ordinal: 1,
                    field: DraftingSemanticSelectionField::LogicalInput { index: 5 + index },
                },
                selected: &selected,
            });
        }
        if case == 8 {
            rows.push(DraftingSemanticSelectionRow::Rule {
                ordinal: 2,
                origin: DraftingRuleSelection::Design,
            });
            rows.push(DraftingSemanticSelectionRow::Profile {
                location: DraftingSemanticSelectionLocation::Semantic {
                    ordinal: 2,
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

        let occurrences = 8 + count + if case == 8 { 2 } else { 0 };
        let row_count = 1 + count + if case == 8 { 2 } else { 0 };
        let passes = if case == 9 {
            9
        } else if case == 8 {
            17
        } else if case == 1 {
            14
        } else {
            15
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

        let root = owner
            .bind_root_input_members()
            .map_err(|e| e.to_string())?
            .bind_root_logical_suppliers()
            .map_err(|e| e.to_string())?
            .bind_root_numeric_suppliers()
            .map_err(|e| e.to_string())?;
        let mut headers = root
            .bind_profile_input_headers()
            .map_err(|e| e.to_string())?;
        let header_count = count + usize::from(case == 8);
        let before = 9 + if header_count == 0 {
            0
        } else {
            header_count + 1
        };
        assert_eq!(headers.records().len(), header_count);
        assert_eq!(
            headers
                .root()
                .logical()
                .members()
                .owner()
                .selections()
                .reserved_child_passes(),
            before
        );
        let header_calls = source.calls.get();
        source.parameter_calls.set(0);
        source.base.walks.set(0);
        source
            .parameter_mode
            .set(if case == 8 || case == 9 { 0 } else { case });
        if case == 2 {
            source.base.extra_charge.set(100001);
        }
        let result = headers.bind_profile_input_parameters();
        if matches!(case, 0 | 8 | 9) {
            let proof = result.map_err(|e| e.to_string())?;
            assert_eq!(proof.records().len(), header_count);
            assert_eq!(proof.operation_input_count(), 5 + header_count);
            assert_eq!(proof.operation_inputs().count(), 5 + header_count);
            assert_eq!(
                proof
                    .operation_inputs()
                    .filter(|input| matches!(input, DraftingProgramOperationInput::Profile(_)))
                    .count(),
                header_count
            );
            assert_eq!(proof.headers().len(), header_count);
            assert_eq!(
                proof.demand(),
                if header_count == 0 {
                    None
                } else {
                    Some(demand)
                }
            );
            assert_eq!(
                proof.child_pass_count(),
                if header_count == 0 {
                    0
                } else {
                    header_count + 1
                }
            );
            assert_eq!(
                proof
                    .root()
                    .logical()
                    .members()
                    .owner()
                    .selections()
                    .reserved_child_passes(),
                passes
            );
            assert!(core::ptr::eq(
                proof.root().logical().members().owner().inputs(),
                &actual
            ));
            for (index, record) in proof.records().iter().enumerate() {
                let header = proof.headers().get(index).ok_or("original header")?;
                assert_eq!(record.header_index(), index);
                assert!(core::ptr::eq(record.header(), header));
                assert!(core::ptr::eq(record.source().parent(), header.source()));
                let request = header.request();
                assert_eq!(record.source().parameter(), request.parameter());
                assert_eq!(record.source().kind(), request.kind());
                let raw = record.source().source();
                let metadata = source
                    .metadata
                    .iter()
                    .find(|record| record.data().parameter == request.parameter())
                    .ok_or("original metadata")?;
                assert!(core::ptr::eq(raw.parameter, &metadata.data().parameter));
                assert!(raw.metadata.is_same_record(metadata));
                assert!(core::ptr::eq(
                    record.source().binding().inputs(),
                    source.inputs.logicals()
                ));
                let original = source
                    .inputs
                    .logicals()
                    .declaration(metadata.data().target.declaration)
                    .ok_or("actual Profile canonical member")?;
                assert!(core::ptr::eq(record.source().declaration(), original));
                assert!(core::ptr::eq(raw.declaration, original));
                assert!(core::ptr::eq(raw.profile.selected, &source.selected));
                assert!(core::ptr::eq(raw.profile.inputs, &source.inputs));
                if request.kind() == LogicalInputKind::NotionSize {
                    assert!(
                        matches!(original.definition().state,LogicalInputState::Unknown {observation} if observation==id(1191))
                    );
                    assert_eq!((request.ordinal(), request.position()), (2, 3));
                } else {
                    assert!(
                        matches!(original.definition().state,LogicalInputState::Derived {operation} if operation==id(80))
                    );
                    assert_eq!((request.ordinal(), request.position()), (1, 1));
                    let root = garment
                        .inputs()
                        .inputs()
                        .logicals()
                        .declaration(id(90))
                        .ok_or("original Root member")?;
                    assert!(!core::ptr::eq(original, root));
                    assert_eq!(root.definition().state.tag(), LogicalInputStateTag::Unknown);
                }
                assert!(!format!("{record:?} {:?}", record.source()).contains("private mapping"));
            }
            if header_count > 0 {
                let first = proof.records().first().ok_or("first original parameter")?;
                let second = proof.records().get(1).ok_or("second original parameter")?;
                assert!(core::ptr::eq(
                    first.source().declaration(),
                    second.source().declaration()
                ));
                assert!(first
                    .source()
                    .source()
                    .metadata
                    .is_same_record(second.source().source().metadata));
                assert!(!core::ptr::eq(
                    first.source().parent(),
                    second.source().parent()
                ));
            }
            assert_eq!(source.parameter_calls.get(), header_count);
            assert_eq!(
                source.base.walks.get(),
                if header_count == 0 {
                    0
                } else {
                    header_count + 1
                }
            );
        } else {
            let error = result.expect_err("complete stage or original child parameter refusal");
            assert_eq!(error.parameter_count(), header_count);
            assert_eq!(error.demand().is_some(), !matches!(case, 1 | 2));
            assert!(core::ptr::eq(
                error.root().logical().members().owner().inputs(),
                &actual
            ));
            assert_eq!(
                error
                    .root()
                    .logical()
                    .members()
                    .owner()
                    .selections()
                    .reserved_child_passes(),
                if case == 1 { before } else { passes }
            );
            match error.refusal() {
                DraftingProgramProfileParameterRefusal::Work(cause) if case == 1 => {
                    assert!(matches!(
                        cause.refusal(),
                        DraftingProgramSemanticWorkRefusal::ChildPasses {
                            allowed: 14,
                            reserved: 12,
                            requested: 3
                        }
                    ));
                    assert_eq!(
                        (source.parameter_calls.get(), source.base.walks.get()),
                        (0, 0)
                    );
                }
                DraftingProgramProfileParameterRefusal::Resources(cause) if case == 2 => {
                    assert!(
                        matches!(cause,DesignResourceLimitError::Count {location,..} if location.field=="numeric.extra")
                    );
                    assert_eq!(
                        (source.parameter_calls.get(), source.base.walks.get()),
                        (0, 1)
                    );
                }
                DraftingProgramProfileParameterRefusal::Parameter {
                    header_index,
                    cause,
                } => {
                    let index = if case == 5 { 0 } else { 1 };
                    assert_eq!(*header_index, index);
                    let header = error
                        .headers()
                        .get(index)
                        .ok_or("original refused header")?;
                    assert!(core::ptr::eq(cause.parent(), header.source()));
                    assert_eq!(cause.parameter(), header.request().parameter());
                    assert_eq!(source.parameter_calls.get(), index + 1);
                    assert_eq!(source.base.walks.get(), index + 2);
                    match cause.refusal() {
                        DraftingProfileLogicalRefusal::Canonical(
                            LogicalInputLookupError::Kind {
                                expected: LogicalInputKind::PocketOpening,
                                actual,
                            },
                        ) if case == 3 => assert!(core::ptr::eq(
                            *actual,
                            source
                                .inputs
                                .logicals()
                                .declaration(id(91))
                                .ok_or("wrong-kind original member")?
                        )),
                        DraftingProfileLogicalRefusal::Mismatch(
                            DraftingProfileLogicalIssue::Parameter,
                        ) if case == 4 => {}
                        DraftingProfileLogicalRefusal::Source(super::SourceError::Missing)
                            if case == 5 => {}
                        DraftingProfileLogicalRefusal::Mismatch(
                            DraftingProfileLogicalIssue::SelectedPin,
                        ) if case == 6 => {}
                        DraftingProfileLogicalRefusal::Mismatch(
                            DraftingProfileLogicalIssue::Inventory,
                        ) if case == 7 => {}
                        _ => return Err(format!("wrong original parameter cause: {cause}").into()),
                    }
                }
                _ => return Err(format!("wrong parameter stage cause: {error}").into()),
            }
        }
        // The result released its unique borrow without resetting the parent's original cursor.
        assert_eq!(
            headers
                .root()
                .logical()
                .members()
                .owner()
                .selections()
                .reserved_child_passes(),
            if case == 1 { before } else { passes }
        );
        assert_eq!(source.calls.get(), header_calls);
        if case == 0 {
            let calls = source.parameter_calls.get();
            let walks = source.base.walks.get();
            let again = headers
                .bind_profile_input_parameters()
                .expect_err("spent cursor cannot be reset or cloned");
            assert!(matches!(
                again.refusal(),
                DraftingProgramProfileParameterRefusal::Work(_)
            ));
            assert_eq!(source.parameter_calls.get(), calls);
            assert_eq!(source.base.walks.get(), walks);
        }
    }
    Ok(())
}
