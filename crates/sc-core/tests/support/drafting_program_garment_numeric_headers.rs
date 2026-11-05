//! Complete original numeric garment header parents after logical Profile parameters.
use super::{id, limits, scope, EmptyHost, Frame, Provider, Snapshot, SourceError, TestResult};
use sc_core::{construction::*, design::*, ontology::*, recipe::*, value::*};
pub(super) trait NumericProvider:
    super::RuleProvider
    + DraftingProfileNumericProvider<
        EmptyHost,
        NumericMetadata = super::super::numeric::NumericMapping,
    >
{
}
impl NumericProvider for Provider<'_> {}
pub(super) type NumericSourceView<'a> = dyn NumericProvider<
        Frame = Frame,
        Snapshot = Snapshot,
        Geometry = [u8],
        Error = SourceError,
        Profile = [u8],
    > + 'a;
#[test]
fn garment_profile_numeric_headers_keep_original_fields_rows_parents_and_complete_work(
) -> TestResult {
    for case in 0..10u8 {
        let empty = case == 9;
        let garment = super::garment_profile_headers::numeric_fixture(empty)?;
        let other = super::garment_profile_headers::numeric_fixture(empty)?;
        let mut source = Provider::new(&garment, other.inputs(), 0)?;
        if case == 8 {
            source.wrong_pin = SelectedProfilePin::new(SelectedProfilePinDefinition {
                profile_id: "original".into(),
                schema_version: "wrong".into(),
                content_version: "content".into(),
            })?;
        }
        let source_view: &NumericSourceView<'_> = &source;
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
        let wrong_request = super::super::pin("other")?;
        use DraftingClosureSelectionField as C;
        use DraftingSemanticSelectionLocation as L;
        let mut rows = Vec::new();
        if !empty {
            rows.push(DraftingSemanticSelectionRow::Profile {
                location: L::Closure {
                    closure: 6,
                    field: C::Length,
                },
                selected: if case == 7 { &wrong_request } else { &selected },
            });
        }
        for location in [
            L::Closure {
                closure: 0,
                field: C::Size(DraftingClosureSizeRole::Bar),
            },
            L::Pocket {
                pocket: 0,
                field: DraftingPocketSelectionField::Opening,
            },
            L::Closure {
                closure: 1,
                field: C::Size(DraftingClosureSizeRole::Button),
            },
            L::Closure {
                closure: 0,
                field: C::Size(DraftingClosureSizeRole::Hook),
            },
        ] {
            rows.push(DraftingSemanticSelectionRow::Profile {
                location,
                selected: &selected,
            });
        }
        if !empty {
            rows.push(DraftingSemanticSelectionRow::Profile {
                location: L::Closure {
                    closure: 5,
                    field: C::Length,
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
        let occurrences = garment.pocket_closure_dependencies().count() + 1;

        let before = if empty { 14 } else { 13 };
        let passes = if empty {
            before
        } else if case == 1 {
            15
        } else {
            16
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
        let logical = headers
            .bind_garment_profile_logical_parameters()
            .map_err(|e| e.to_string())?;
        let original_logical_headers = logical.headers().as_ptr();
        let original_logical_parameters = logical.records().as_ptr();
        assert_eq!(logical.records().len(), 4);
        let parameter_queries = source.parameter_calls.get();
        source.calls.set(0);
        source.base.walks.set(0);
        source.header_override.set(Some(match case {
            3 | 8 => 3,
            4 => 5,
            5 => 6,
            6 => 7,
            _ => 0,
        }));
        if case == 2 {
            source.base.extra_charge.set(100001);
        }
        let result = logical.bind_garment_profile_numeric_headers();
        assert_eq!(source.parameter_calls.get(), parameter_queries);
        assert_eq!(source.numeric_calls.get(), 0);
        assert_eq!(source.rule_calls.get(), 0);
        if !matches!(case, 0 | 9) {
            let error =
                result.expect_err("complete numeric headers refuse without accepted prefix");
            assert_eq!(error.header_count(), Some(2));
            assert_eq!(error.owner().headers().as_ptr(), original_logical_headers);
            assert_eq!(
                error.owner().records().as_ptr(),
                original_logical_parameters
            );
            let cursor = error
                .owner()
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
            assert_eq!(
                cursor.reserved_child_passes(),
                if case == 1 { before } else { 16 }
            );
            match (case, error.refusal()) {
                (1, DraftingProgramGarmentProfileNumericHeaderRefusal::Work(cause)) => {
                    assert!(matches!(
                        cause.refusal(),
                        DraftingProgramSemanticWorkRefusal::ChildPasses {
                            allowed: 15,
                            reserved: 13,
                            requested: 3
                        }
                    ));
                    assert!(error.demand().is_none());
                    assert_eq!((source.base.walks.get(), source.calls.get()), (0, 0));
                }
                (
                    2,
                    DraftingProgramGarmentProfileNumericHeaderRefusal::Resources(
                        DesignResourceLimitError::Count { location, .. },
                    ),
                ) => {
                    assert_eq!(location.field, "numeric.extra");
                    assert!(error.demand().is_none());
                    assert_eq!((source.base.walks.get(), source.calls.get()), (1, 0));
                }
                (
                    3..=8,
                    DraftingProgramGarmentProfileNumericHeaderRefusal::Source { request, cause },
                ) => {
                    let late = matches!(case, 3 | 7 | 8);
                    assert_eq!(request.row_index(), if late { 0 } else { 5 });
                    assert_eq!(
                        request.parameter(),
                        ProfileParameterRef::new(id(if late { 705 } else { 704 }))
                    );
                    assert!(core::ptr::eq(request.occurrence().garment(), &garment));
                    assert!(core::ptr::eq(cause.selected(), request.selected()));
                    assert_eq!(error.demand(), Some(demand));
                    match (case, cause.refusal()) {
                        (
                            3 | 7,
                            DraftingProfileSourceRefusal::Mismatch(
                                DraftingProfileSourceIssue::SelectedPin(
                                    SelectedProfilePinField::ProfileId,
                                ),
                            ),
                        )
                        | (
                            4,
                            DraftingProfileSourceRefusal::Mismatch(
                                DraftingProfileSourceIssue::Root,
                            ),
                        )
                        | (
                            5,
                            DraftingProfileSourceRefusal::Mismatch(
                                DraftingProfileSourceIssue::Snapshot,
                            ),
                        )
                        | (6, DraftingProfileSourceRefusal::Source(SourceError::Missing))
                        | (
                            8,
                            DraftingProfileSourceRefusal::Mismatch(
                                DraftingProfileSourceIssue::SelectedPin(
                                    SelectedProfilePinField::SchemaVersion,
                                ),
                            ),
                        ) => {}
                        _ => {
                            return Err(format!(
                                "wrong numeric header cause: {:?}",
                                cause.refusal()
                            )
                            .into())
                        }
                    }
                    let queries = if late { 2 } else { 1 };
                    assert_eq!(
                        (source.base.walks.get(), source.calls.get()),
                        (queries + 1, queries)
                    );
                }
                _ => return Err("wrong numeric header stage refusal".into()),
            }
            continue;
        }
        let proof = result.map_err(|e| e.to_string())?;
        assert_eq!(proof.logical().headers().as_ptr(), original_logical_headers);
        assert_eq!(
            proof.logical().records().as_ptr(),
            original_logical_parameters
        );
        let count = if empty { 0 } else { 2 };
        assert_eq!(proof.records().len(), count);
        assert_eq!(proof.child_pass_count(), if empty { 0 } else { 3 });
        assert_eq!(proof.demand(), if empty { None } else { Some(demand) });
        assert_eq!(
            (source.base.walks.get(), source.calls.get()),
            if empty { (0, 0) } else { (3, 2) }
        );
        assert_eq!(
            proof
                .logical()
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
            assert_eq!(request.row_index(), if index == 0 { 5 } else { 0 });
            assert_eq!(
                request.parameter(),
                ProfileParameterRef::new(id(if index == 0 { 704 } else { 705 }))
            );
            assert_eq!(request.kind(), ScalarInputKind::Length);
            assert!(core::ptr::eq(request.occurrence().garment(), &garment));
            assert!(core::ptr::eq(request.selected(), &selected));
            let parent = header.source();
            assert!(core::ptr::eq(parent.selected(), request.selected()));
            assert!(core::ptr::eq(parent.context().source(), source_view));
            let raw = parent.source();
            assert!(core::ptr::eq(raw.root, garment.inputs()));
            assert!(core::ptr::eq(raw.selected, &source.selected));
            assert!(core::ptr::eq(raw.inputs, &source.inputs));
            assert!(raw.profile.is_same_record(&source.body));
            assert!(raw.snapshot.is_same_record(source.base.snapshot()));
        }
    }
    Ok(())
}
