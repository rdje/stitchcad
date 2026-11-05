//! Original numeric garment metadata/member proofs borrow exact stable parents and one cursor.
use super::garment_numeric_headers::NumericSourceView;
use super::{id, limits, scope, Provider, TestResult};
use sc_core::{construction::*, design::*, ontology::*, recipe::*, value::*};
#[test]
fn garment_profile_numeric_parameters_keep_original_parents_members_states_and_whole_work(
) -> TestResult {
    for case in 0..16u8 {
        let empty = case == 10;
        let garment = super::garment_profile_headers::numeric_fixture(empty)?;
        let other = super::garment_profile_headers::numeric_fixture(empty)?;
        let mut source = Provider::new(&garment, other.inputs(), 0)?;
        super::super::numeric::prepare(&mut source, 0)?;
        source.numeric_mode = match case {
            2 | 5 | 15 => 1,
            4 => 7,
            6 => 3,
            7 => 4,
            8 => 5,
            9 => 6,
            _ => 0,
        };
        source.numeric_late = matches!(case, 4..=9);
        if matches!(case, 3 | 12 | 13 | 14) {
            let target = match case {
                3 => CanonicalInputTarget::Scalar {
                    declaration: id(30),
                    kind: ScalarInputKind::Boolean,
                },
                12 => CanonicalInputTarget::Scalar {
                    declaration: id(20),
                    kind: ScalarInputKind::Boolean,
                },
                13 => CanonicalInputTarget::Length(id(20)),
                _ => CanonicalInputTarget::Length(id(10)),
            };
            *source
                .numeric_metadata
                .get_mut(1)
                .ok_or("second original numeric metadata")? =
                DraftingSourceRecord::new(super::super::numeric::NumericMapping {
                    parameter: ProfileParameterRef::new(id(705)),
                    target,
                    text: "original numeric metadata705".into(),
                });
        }
        if case == 11 {
            source
                .numeric_metadata
                .retain(|m| m.data().parameter.id() != id(705));
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
        use DraftingClosureSelectionField as C;
        use DraftingSemanticSelectionLocation as L;
        let mut rows = Vec::new();
        if !empty {
            rows.push(DraftingSemanticSelectionRow::Profile {
                location: L::Closure {
                    closure: 6,
                    field: C::Length,
                },
                selected: &selected,
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

        let before = if empty { 14 } else { 16 };
        let passes = if empty {
            before
        } else if case == 1 {
            18
        } else {
            19
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
        let original_logical = logical.records().as_ptr();
        let mut numeric = logical
            .bind_garment_profile_numeric_headers()
            .map_err(|e| e.to_string())?;
        let original_headers = numeric.records().as_ptr();
        let header_queries = source.calls.get();
        let logical_queries = source.parameter_calls.get();
        source.base.walks.set(0);
        assert_eq!(source.numeric_calls.get(), 0);
        if case == 2 {
            source.base.extra_charge.set(100001);
        }
        let result = numeric.bind_garment_profile_numeric_parameters();
        assert_eq!(source.calls.get(), header_queries);
        assert_eq!(source.parameter_calls.get(), logical_queries);
        assert_eq!(source.rule_calls.get(), 0);
        if !matches!(case, 0 | 10 | 14) {
            let error = result.expect_err("complete numeric parameters refuse without a prefix");
            assert_eq!(error.headers().as_ptr(), original_headers);
            assert_eq!(error.parameter_count(), 2);
            assert_eq!(error.logical().records().as_ptr(), original_logical);
            let cursor = error
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
                .selections();
            assert_eq!(
                cursor.reserved_child_passes(),
                if case == 1 { before } else { 19 }
            );
            match (case, error.refusal()) {
                (1, DraftingProgramGarmentProfileNumericParameterRefusal::Work(cause)) => {
                    assert!(matches!(
                        cause.refusal(),
                        DraftingProgramSemanticWorkRefusal::ChildPasses {
                            allowed: 18,
                            reserved: 16,
                            requested: 3
                        }
                    ));
                    assert!(error.demand().is_none());
                    assert_eq!(
                        (source.base.walks.get(), source.numeric_calls.get()),
                        (0, 0)
                    );
                }
                (
                    2,
                    DraftingProgramGarmentProfileNumericParameterRefusal::Resources(
                        DesignResourceLimitError::Count { location, .. },
                    ),
                ) => {
                    assert_eq!(location.field, "numeric.extra");
                    assert!(error.demand().is_none());
                    assert_eq!(
                        (source.base.walks.get(), source.numeric_calls.get()),
                        (1, 0)
                    );
                }
                (
                    3..=9 | 11..=13 | 15,
                    DraftingProgramGarmentProfileNumericParameterRefusal::Parameter {
                        header_index,
                        cause,
                    },
                ) => {
                    let index = if case == 15 { 0 } else { 1 };
                    assert_eq!(*header_index, index);
                    let header = error
                        .headers()
                        .get(index)
                        .ok_or("original refused numeric header")?;
                    assert!(core::ptr::eq(cause.parent(), header.source()));
                    assert_eq!(cause.parameter(), header.request().parameter());
                    assert_eq!(cause.kind(), ScalarInputKind::Length);
                    assert_eq!(error.demand(), Some(demand));
                    let queries = index + 1;
                    assert_eq!(
                        (source.base.walks.get(), source.numeric_calls.get()),
                        (queries + 1, queries)
                    );
                    match (case, cause.refusal()) {
                        (
                            3,
                            DraftingProfileNumericRefusal::Kind {
                                actual: ScalarInputKind::Boolean,
                            },
                        ) => {}
                        (
                            4,
                            DraftingProfileNumericRefusal::Mismatch(
                                DraftingProfileNumericIssue::Parameter,
                            ),
                        )
                        | (
                            6,
                            DraftingProfileNumericRefusal::Mismatch(
                                DraftingProfileNumericIssue::SelectedPin,
                            ),
                        )
                        | (
                            7,
                            DraftingProfileNumericRefusal::Mismatch(
                                DraftingProfileNumericIssue::Inventory,
                            ),
                        )
                        | (
                            8,
                            DraftingProfileNumericRefusal::Mismatch(
                                DraftingProfileNumericIssue::Profile,
                            ),
                        )
                        | (
                            9,
                            DraftingProfileNumericRefusal::Mismatch(
                                DraftingProfileNumericIssue::Snapshot,
                            ),
                        )
                        | (
                            5 | 11 | 15,
                            DraftingProfileNumericRefusal::Source(super::SourceError::Missing),
                        ) => {}
                        (12 | 13, DraftingProfileNumericRefusal::Canonical(canonical)) => {
                            assert!(core::ptr::eq(canonical.inputs(), &source.inputs));
                            match (case, canonical.refusal()) {
                                (
                                    12,
                                    CanonicalNumericInputRefusal::ScalarKind {
                                        expected: ScalarInputKind::Boolean,
                                        index: 0,
                                        actual,
                                    },
                                ) => {
                                    assert!(core::ptr::eq(
                                        actual,
                                        source
                                            .inputs
                                            .scalars()
                                            .first()
                                            .ok_or("actual original scalar")?
                                    ));
                                }
                                (
                                    13,
                                    CanonicalNumericInputRefusal::MissingLength {
                                        other: Some(CanonicalInputPosition::Scalar { index: 0 }),
                                    },
                                ) => {}
                                _ => return Err("wrong original canonical numeric refusal".into()),
                            }
                        }
                        _ => {
                            return Err(format!(
                                "wrong numeric parameter cause: {:?}",
                                cause.refusal()
                            )
                            .into())
                        }
                    }
                }
                _ => return Err("wrong original numeric parameter stage cause".into()),
            }
            continue;
        }
        let proof = result.map_err(|e| e.to_string())?;
        let count = if empty { 0 } else { 2 };
        assert_eq!(proof.records().len(), count);
        assert_eq!(proof.headers().as_ptr(), original_headers);
        assert_eq!(proof.logical().records().as_ptr(), original_logical);
        assert_eq!(proof.logical().records().len(), 4);
        assert_eq!(proof.child_pass_count(), if empty { 0 } else { 3 });
        assert_eq!(proof.demand(), if empty { None } else { Some(demand) });
        assert_eq!(
            (source.base.walks.get(), source.numeric_calls.get()),
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
        for (index, record) in proof.records().iter().enumerate() {
            assert_eq!(record.header_index(), index);
            let header = proof
                .headers()
                .get(index)
                .ok_or("original numeric header parent")?;
            assert!(core::ptr::eq(record.header(), header));
            assert!(core::ptr::eq(record.source().parent(), header.source()));
            let original = source
                .numeric_metadata
                .iter()
                .find(|m| m.data().parameter == header.request().parameter())
                .ok_or("original numeric metadata")?;
            let raw = record.source().source();
            assert!(raw.metadata.is_same_record(original));
            assert!(core::ptr::eq(raw.parameter, &original.data().parameter));
            assert!(core::ptr::eq(raw.target, &original.data().target));
            let binding = record.source().binding();
            assert!(core::ptr::eq(binding.inputs(), &source.inputs));
            assert_eq!(binding.target(), original.data().target);
            if index == 0 || case == 14 {
                assert!(
                    matches!(binding.member(),CanonicalNumericInputMember::Length {index:0,declaration} if core::ptr::eq(declaration,source.inputs.lengths().first().ok_or("original Profile length")?) && matches!(declaration.state(),LengthState::Derived {formula} if *formula==id(500)))
                );
            } else {
                assert!(
                    matches!(binding.member(),CanonicalNumericInputMember::Scalar {index:0,declaration} if core::ptr::eq(declaration,source.inputs.scalars().first().ok_or("original Profile scalar")?) && matches!(declaration.state(),ScalarInputState::Unknown {..}))
                );
            }
            assert!(matches!(
                garment
                    .inputs()
                    .inputs()
                    .lengths()
                    .first()
                    .ok_or("original Root length")?
                    .state(),
                LengthState::Unknown { .. }
            ));
        }
        drop(proof);
        if !empty {
            let walks = source.base.walks.get();
            let queries = source.numeric_calls.get();
            let repeated = numeric
                .bind_garment_profile_numeric_parameters()
                .expect_err("spent complete work cannot reset after drop");
            assert!(
                matches!(repeated.refusal(),DraftingProgramGarmentProfileNumericParameterRefusal::Work(cause) if matches!(cause.refusal(),DraftingProgramSemanticWorkRefusal::ChildPasses {allowed:19,reserved:19,requested:3}))
            );
            assert_eq!(
                (source.base.walks.get(), source.numeric_calls.get()),
                (walks, queries)
            );
        }
    }
    Ok(())
}
