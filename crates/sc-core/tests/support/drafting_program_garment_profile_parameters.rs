//! Original logical garment parameter proofs borrow stable actual headers and one unique cursor.
use super::{id, limits, scope, Provider, RuleSourceView, TestResult};
use sc_core::{construction::*, design::*, recipe::*, value::*};
#[test]
fn garment_profile_logical_parameters_keep_original_parents_metadata_states_and_first_causes(
) -> TestResult {
    for case in 0..10u8 {
        let empty = case == 8;
        let fixture_case = if empty {
            1
        } else if case == 9 {
            10
        } else {
            0
        };
        let garment = super::garment_profile_headers::fixture(fixture_case)?;
        let other = super::garment_profile_headers::fixture(fixture_case)?;
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
        let requested = &selected;
        use DraftingClosureSelectionField as C;
        use DraftingSemanticSelectionLocation as L;
        let mut rows = vec![DraftingSemanticSelectionRow::Profile {
            location: L::Closure {
                closure: 5,
                field: C::Length,
            },
            selected: requested,
        }];
        if !empty {
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
                    requested,
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

        let before = if empty { 7 } else { 8 };
        let passes = if empty {
            7
        } else if case == 1 {
            12
        } else {
            13
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
        let mut op_headers = root
            .bind_profile_input_headers()
            .map_err(|e| e.to_string())?;
        let inputs = op_headers
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
        let mut headers = root
            .bind_garment_profile_logical_headers()
            .map_err(|e| e.to_string())?;
        let original_headers = headers.records().as_ptr();
        assert_eq!(
            headers
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
            before
        );
        let header_queries = source.calls.get();
        source.parameter_calls.set(0);
        source.base.walks.set(0);
        source
            .parameter_mode
            .set(if empty || case == 9 { 0 } else { case });
        if case == 2 {
            source.base.extra_charge.set(100001);
        }
        let result = headers.bind_garment_profile_logical_parameters();
        assert_eq!(
            (source.calls.get(), source.rule_calls.get()),
            (header_queries, 0)
        );
        if case != 0 && !empty {
            let error =
                result.expect_err("complete parameters refuse first original cause without prefix");
            assert_eq!(error.parameter_count(), 4);
            assert_eq!(error.headers().as_ptr(), original_headers);
            let cursor = error
                .root()
                .logical()
                .members()
                .owner()
                .inputs()
                .root()
                .logical()
                .members()
                .owner()
                .selections();
            if case == 1 {
                assert!(
                    matches!(error.refusal(),DraftingProgramGarmentProfileLogicalParameterRefusal::Work(cause) if matches!(cause.refusal(),DraftingProgramSemanticWorkRefusal::ChildPasses {allowed:12,reserved:8,requested:5}))
                );
                assert_eq!(cursor.reserved_child_passes(), 8);
                assert_eq!(
                    (source.base.walks.get(), source.parameter_calls.get()),
                    (0, 0)
                );
                assert!(error.demand().is_none());
            } else if case == 2 {
                assert!(
                    matches!(error.refusal(),DraftingProgramGarmentProfileLogicalParameterRefusal::Resources(DesignResourceLimitError::Count {location,..}) if location.field=="numeric.extra")
                );
                assert_eq!(cursor.reserved_child_passes(), 13);
                assert_eq!(
                    (source.base.walks.get(), source.parameter_calls.get()),
                    (1, 0)
                );
                assert!(error.demand().is_none());
            } else {
                let DraftingProgramGarmentProfileLogicalParameterRefusal::Parameter {
                    header_index,
                    cause,
                } = error.refusal()
                else {
                    return Err("original parameter cause".into());
                };
                let index = if case == 5 {
                    0
                } else if case == 9 {
                    3
                } else {
                    1
                };
                assert_eq!(*header_index, index);
                let parent = error
                    .headers()
                    .get(index)
                    .ok_or("original refused header")?;
                assert!(std::ptr::eq(cause.parent(), parent.source()));
                assert_eq!(cause.parameter(), parent.request().parameter());
                assert!(error.demand().is_some());
                assert_eq!(cursor.reserved_child_passes(), 13);
                match case {
                    3 => assert!(
                        matches!(cause.refusal(),DraftingProfileLogicalRefusal::Canonical(LogicalInputLookupError::Kind {expected:LogicalInputKind::NotionSize,actual}) if std::ptr::eq(*actual,source.inputs.logicals().declaration(id(90)).ok_or("actual wrong kind Profile member")?))
                    ),
                    4 => assert!(matches!(
                        cause.refusal(),
                        DraftingProfileLogicalRefusal::Mismatch(
                            DraftingProfileLogicalIssue::Parameter
                        )
                    )),
                    5 | 9 => assert!(matches!(
                        cause.refusal(),
                        DraftingProfileLogicalRefusal::Source(super::SourceError::Missing)
                    )),
                    6 => assert!(matches!(
                        cause.refusal(),
                        DraftingProfileLogicalRefusal::Mismatch(
                            DraftingProfileLogicalIssue::SelectedPin
                        )
                    )),
                    7 => assert!(matches!(
                        cause.refusal(),
                        DraftingProfileLogicalRefusal::Mismatch(
                            DraftingProfileLogicalIssue::Inventory
                        )
                    )),
                    _ => return Err("unhandled mode".into()),
                }
                let queries = if case == 5 {
                    1
                } else if case == 9 {
                    4
                } else {
                    2
                };
                assert_eq!(
                    (source.base.walks.get(), source.parameter_calls.get()),
                    (queries + 1, queries)
                );
            }
            continue;
        }
        let proof = result.map_err(|e| e.to_string())?;
        let count = if empty { 0 } else { 4 };
        assert_eq!(proof.records().len(), count);
        assert_eq!(proof.headers().as_ptr(), original_headers);
        assert_eq!(proof.child_pass_count(), if empty { 0 } else { 5 });
        assert_eq!(proof.demand(), if empty { None } else { Some(demand) });
        assert_eq!(
            (source.base.walks.get(), source.parameter_calls.get()),
            if empty { (0, 0) } else { (5, 4) }
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
        for (index, record) in proof.records().iter().enumerate() {
            assert_eq!(record.header_index(), index);
            let header = proof.headers().get(index).ok_or("actual original header")?;
            assert!(std::ptr::eq(record.header(), header));
            assert!(std::ptr::eq(record.source().parent(), header.source()));
            let request = header.request();
            let raw = record.source().source();
            let expected = source
                .metadata
                .iter()
                .find(|r| r.data().parameter == request.parameter())
                .ok_or("actual source metadata")?;
            assert!(raw.metadata.is_same_record(expected));
            assert!(std::ptr::eq(raw.parameter, &expected.data().parameter));
            let member = record.source().binding();
            assert!(std::ptr::eq(member.inputs(), source.inputs.logicals()));
            assert!(std::ptr::eq(
                member.declaration(),
                source
                    .inputs
                    .logicals()
                    .declaration(if index == 0 { id(90) } else { id(91) })
                    .ok_or("actual Profile member")?
            ));
            assert_eq!(record.source().kind(), request.kind());
            assert_ne!(request.parameter().id(), id(704));
            if index == 0 {
                assert!(
                    matches!(member.declaration().definition().state,LogicalInputState::Derived {operation} if operation==id(80))
                );
                let root = garment
                    .inputs()
                    .inputs()
                    .logicals()
                    .declaration(id(90))
                    .ok_or("actual Root90")?;
                assert!(matches!(
                    root.definition().state,
                    LogicalInputState::Unknown { .. }
                ));
                assert!(!std::ptr::eq(member.declaration(), root));
                assert!(garment.inputs().semantic_recipe().operations().is_empty());
            } else {
                assert!(matches!(
                    member.declaration().definition().state,
                    LogicalInputState::Unknown { .. }
                ));
            }
        }
        if !empty {
            let hook = proof.records().get(1).ok_or("hook")?;
            let bar = proof.records().get(2).ok_or("bar")?;
            assert!(hook
                .source()
                .source()
                .metadata
                .is_same_record(bar.source().source().metadata));
            assert!(std::ptr::eq(
                hook.source().declaration(),
                bar.source().declaration()
            ));
            assert!(!std::ptr::eq(hook.source().parent(), bar.source().parent()));
        }
        drop(proof);
        if !empty {
            let error = headers
                .bind_garment_profile_logical_parameters()
                .expect_err("spent cursor refuses before another walk/query");
            assert!(matches!(
                error.refusal(),
                DraftingProgramGarmentProfileLogicalParameterRefusal::Work(_)
            ));
            assert_eq!(
                (
                    source.base.walks.get(),
                    source.parameter_calls.get(),
                    source.calls.get()
                ),
                (5, 4, header_queries)
            );
        }
    }
    Ok(())
}
