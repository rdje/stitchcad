//! Complete original Pocket sources borrow stable work records and preserve the unique cursor.
use super::{id, limits, scope, Provider, TestResult};
use sc_core::{construction::*, design::*, recipe::*};
use std::error::Error;
#[test]
fn pocket_sources_keep_stable_parents_original_imports_whole_cursor_and_late_causes() -> TestResult
{
    for case in 0..10u8 {
        let empty = case == 4;
        let mode = match case {
            4 => 2,
            5 => 3,
            _ => 0,
        };
        let garment = super::garment_profile_headers::fixture_with_pockets(0, 1, mode)?;
        let other = super::garment_profile_headers::numeric_fixture(false)?;
        let mut source = Provider::new(&garment, other.inputs(), 0)?;
        super::super::numeric::prepare(&mut source, 0)?;
        source.component_import = Some(DraftingSourceRecord::new(
            b"original imported Piece100 payload".to_vec(),
        ));
        let source_view = &source;
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
        // One explicit opening row per actual Pocket; repeated targets remain distinct fields.
        for pocket in 1..garment.pockets().pockets().len() {
            rows.push(DraftingSemanticSelectionRow::Profile {
                location: L::Pocket {
                    pocket,
                    field: DraftingPocketSelectionField::Opening,
                },
                selected: &selected,
            });
        }
        rows.push(DraftingSemanticSelectionRow::Profile {
            location: L::Closure {
                closure: 6,
                field: C::Length,
            },
            selected: &selected,
        });
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
            if empty && matches!(location, L::Pocket { .. }) {
                continue;
            }
            rows.push(DraftingSemanticSelectionRow::Profile {
                location,
                selected: &selected,
            });
        }
        rows.push(DraftingSemanticSelectionRow::Profile {
            location: L::Closure {
                closure: 5,
                field: C::Length,
            },
            selected: &selected,
        });
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

        let prior = if empty {
            17
        } else if case == 5 {
            21
        } else {
            19
        };
        let count = if empty {
            0
        } else if case == 5 {
            4
        } else {
            2
        };
        let stage_passes = if count == 0 { 0 } else { count + 1 };
        let passes = prior
            + stage_passes
            + if case == 1 {
                stage_passes - 1
            } else {
                stage_passes
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
        let mut numeric = logical
            .bind_garment_profile_numeric_headers()
            .map_err(|e| e.to_string())?;
        let inputs = numeric
            .bind_garment_profile_numeric_parameters()
            .map_err(|e| e.to_string())?;
        let original_parameters = inputs.records().as_ptr();
        let mut pocket_work = inputs
            .bind_pocket_component_work()
            .map_err(|e| e.to_string())?;
        let original_work = pocket_work.records().as_ptr();
        let prior = prior + stage_passes;
        assert_eq!(source.component_calls.get(), 0);
        source.base.walks.set(0);
        let queries = (
            source.calls.get(),
            source.parameter_calls.get(),
            source.numeric_calls.get(),
            source.rule_calls.get(),
        );
        if case == 2 {
            source.base.extra_charge.set(100001);
        }
        if case == 3 {
            source.extra_after.set(Some(3));
            source.extra_entries.set(100001);
        }
        source.component_mode.set(match case {
            6 => 1,
            7 => 2,
            8 => 3,
            9 => 4,
            _ => 0,
        });
        let result = pocket_work.bind_pocket_sources();
        assert_eq!(
            (
                source.calls.get(),
                source.parameter_calls.get(),
                source.numeric_calls.get(),
                source.rule_calls.get()
            ),
            queries
        );
        if matches!(case, 1 | 2 | 3 | 6..=9) {
            let error = result.expect_err("whole source class refuses without an accepted prefix");
            assert_eq!(error.source_count(), count);
            assert_eq!(error.work().as_ptr(), original_work);
            assert_eq!(error.parent().records().as_ptr(), original_parameters);
            let reserved = error
                .parent()
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
                .reserved_child_passes();
            assert_eq!(reserved, if case == 1 { prior } else { passes });
            match (case, error.refusal()) {
                (1, DraftingProgramPocketSourceRefusal::Work(cause)) => {
                    assert!(matches!(
                        cause.refusal(),
                        DraftingProgramSemanticWorkRefusal::ChildPasses {
                            allowed: 24,
                            reserved: 22,
                            requested: 3
                        }
                    ));
                    assert!(error.demand().is_none());
                    assert_eq!(
                        (source.base.walks.get(), source.component_calls.get()),
                        (0, 0)
                    );
                }
                (
                    2,
                    DraftingProgramPocketSourceRefusal::Resources(
                        DesignResourceLimitError::Count { location, .. },
                    ),
                ) => {
                    assert_eq!(location.field, "numeric.extra");
                    assert!(error.demand().is_none());
                    assert_eq!(
                        (source.base.walks.get(), source.component_calls.get()),
                        (1, 0)
                    );
                }
                (3 | 6..=9, DraftingProgramPocketSourceRefusal::Source { work_index, cause }) => {
                    assert_eq!(*work_index, 1);
                    let parent = error.work().get(1).ok_or("original second work parent")?;
                    assert!(core::ptr::eq(cause.work(), parent.work()));
                    assert_eq!(parent.work().target().copy, id(10002));
                    assert_eq!(
                        parent.occurrence().selection_location(),
                        DraftingSemanticSelectionLocation::Pocket {
                            pocket: 0,
                            field: DraftingPocketSelectionField::Component { index: 0 }
                        }
                    );
                    assert!(error.demand().is_some());
                    assert_eq!(source.base.walks.get(), 3);
                    assert_eq!(source.component_calls.get(), if case == 3 { 1 } else { 2 });
                    match (case, cause.refusal()) {
                        (3, DraftingComponentSourceRefusal::Work(child)) => {
                            assert!(cause.returned().is_none());
                            assert!(
                                matches!(child.refusal(),DraftingComponentWorkRefusal::Resources(DesignResourceLimitError::Count{location,..}) if location.field=="rule.late-cache")
                            );
                        }
                        (
                            6,
                            DraftingComponentSourceRefusal::Source(super::SourceError::Missing),
                        ) => {
                            assert!(cause.returned().is_none());
                            assert!(error
                                .source()
                                .is_some_and(|c| c.downcast_ref::<super::SourceError>().is_some()));
                        }
                        (
                            7,
                            DraftingComponentSourceRefusal::Mismatch(
                                DraftingComponentSourceIssue::Piece,
                            ),
                        )
                        | (
                            8,
                            DraftingComponentSourceRefusal::Mismatch(
                                DraftingComponentSourceIssue::Frame,
                            ),
                        )
                        | (
                            9,
                            DraftingComponentSourceRefusal::Mismatch(
                                DraftingComponentSourceIssue::Snapshot,
                            ),
                        ) => {
                            assert!(cause.returned().is_some());
                        }
                        _ => {
                            return Err(
                                format!("original child refusal {:?}", cause.refusal()).into()
                            )
                        }
                    }
                }
                _ => {
                    return Err(
                        format!("original source-stage refusal {:?}", error.refusal()).into(),
                    )
                }
            }
            drop(error);
        } else {
            let proof = result.map_err(|e| e.to_string())?;
            assert_eq!(proof.records().len(), count);
            assert_eq!(proof.work().as_ptr(), original_work);
            assert_eq!(proof.parent().records().as_ptr(), original_parameters);
            assert_eq!(proof.child_pass_count(), stage_passes);
            assert_eq!(proof.demand().is_some(), !empty);
            assert_eq!(
                (source.base.walks.get(), source.component_calls.get()),
                (stage_passes, count)
            );
            for (index, record) in proof.records().iter().enumerate() {
                assert_eq!(record.work_index(), index);
                let parent = proof.work().get(index).ok_or("exact stable work parent")?;
                assert!(core::ptr::eq(record.work(), parent));
                assert!(core::ptr::eq(record.source().work(), parent.work()));
                assert_eq!(record.source().creator_position(), None);
                match record.source().source().origin {
                    DraftingComponentOrigin::Imported(original) => {
                        assert!(core::ptr::eq(original.piece, parent.work().piece()));
                        assert!(core::ptr::eq(
                            original.source,
                            source
                                .component_import
                                .as_ref()
                                .ok_or("actual imported record")?
                        ));
                        assert_eq!(
                            original.source.data().as_slice(),
                            b"original imported Piece100 payload"
                        );
                    }
                    _ => return Err("original imported source".into()),
                }
            }
            drop(proof);
        }
        let walks = source.base.walks.get();
        let queries = source.component_calls.get();
        let repeated = pocket_work.bind_pocket_sources();
        if empty {
            assert!(repeated.map_err(|e| e.to_string())?.records().is_empty());
        } else {
            assert!(matches!(
                repeated
                    .expect_err("same cursor cannot refund reserved work")
                    .refusal(),
                DraftingProgramPocketSourceRefusal::Work(_)
            ));
        }
        assert_eq!(
            (source.base.walks.get(), source.component_calls.get()),
            (walks, queries)
        );
    }
    Ok(())
}
