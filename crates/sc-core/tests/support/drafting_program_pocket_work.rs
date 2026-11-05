//! Original Pocket served/component work retains the whole input proof and one finite cursor.
use super::{id, limits, scope, EmptyHost, Provider, SourceError, TestResult};
use sc_core::{construction::*, design::*, ontology::*, recipe::*};
impl DraftingComponentProvider<EmptyHost> for Provider<'_> {
    type Imported = [u8];
    fn component_source(
        &self,
        _: PocketPieceRef,
    ) -> Result<DraftingComponentResponse<'_, EmptyHost, Self>, SourceError> {
        panic!("current-member work must not query a source");
    }
}
#[test]
fn pocket_component_work_keeps_original_members_complete_cursor_and_first_typed_cause() -> TestResult
{
    for case in 0..7u8 {
        let empty = case == 4;
        let mode = match case {
            4 => 2,
            5 => 3,
            6 => 1,
            _ => 0,
        };
        let garment = super::garment_profile_headers::fixture_with_pockets(0, 1, mode)?;
        let other = super::garment_profile_headers::numeric_fixture(false)?;
        let mut source = Provider::new(&garment, other.inputs(), 0)?;
        super::super::numeric::prepare(&mut source, 0)?;
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
        let original_headers = inputs.headers().as_ptr();
        let original_parameters = inputs.records().as_ptr();
        let queries = (
            source.calls.get(),
            source.parameter_calls.get(),
            source.numeric_calls.get(),
            source.rule_calls.get(),
        );
        source.base.walks.set(0);
        if case == 2 {
            source.base.extra_charge.set(100001);
        }
        if case == 3 {
            source.extra_after.set(Some(3));
            source.extra_entries.set(100001);
        }
        let result = inputs.bind_pocket_component_work();
        assert_eq!(
            (
                source.calls.get(),
                source.parameter_calls.get(),
                source.numeric_calls.get(),
                source.rule_calls.get()
            ),
            queries
        );
        if matches!(case, 1 | 2 | 3 | 6) {
            let error =
                result.expect_err("complete Pocket stage rejects without exposing a prefix");
            assert_eq!(error.component_count(), Some(count));
            assert_eq!(error.parent().headers().as_ptr(), original_headers);
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
                (1, DraftingProgramPocketWorkRefusal::Work(cause)) => {
                    assert!(matches!(
                        cause.refusal(),
                        DraftingProgramSemanticWorkRefusal::ChildPasses {
                            allowed: 21,
                            reserved: 19,
                            requested: 3
                        }
                    ));
                    assert_eq!(source.base.walks.get(), 0);
                    assert!(error.demand().is_none());
                }
                (
                    2,
                    DraftingProgramPocketWorkRefusal::Resources(DesignResourceLimitError::Count {
                        location,
                        ..
                    }),
                ) => {
                    assert_eq!(location.field, "numeric.extra");
                    assert_eq!(source.base.walks.get(), 1);
                    assert!(error.demand().is_none());
                }
                (3 | 6, DraftingProgramPocketWorkRefusal::Component { occurrence, cause }) => {
                    assert!(
                        matches!(occurrence.owner(),DraftingGarmentSemanticOwner::Pocket {index:0,pocket} if core::ptr::eq(pocket,garment.pockets().pockets().first().ok_or("actual Pocket")?))
                    );
                    assert!(
                        matches!(occurrence.field(),DraftingGarmentSemanticField::PocketComponent {index:0,reference} if reference.copy==id(10002))
                    );
                    assert!(core::ptr::eq(cause.parent(), &admitted_garment));
                    assert_eq!(cause.target().copy, id(10002));
                    assert_eq!(source.base.walks.get(), 3);
                    assert!(error.demand().is_some());
                    if case == 3 {
                        assert!(
                            matches!(cause.refusal(),DraftingComponentWorkRefusal::Resources(DesignResourceLimitError::Count {location,..}) if location.field=="rule.late-cache")
                        );
                    } else {
                        assert!(matches!(
                            cause.refusal(),
                            DraftingComponentWorkRefusal::MissingCopy
                        ));
                    }
                }
                _ => panic!("original first typed cause: {:?}", error.refusal()),
            }
            let parent = error.into_parent();
            assert_eq!(parent.records().as_ptr(), original_parameters);
            continue;
        }
        let proof = result.map_err(|e| e.to_string())?;
        assert_eq!(proof.records().len(), count);
        assert_eq!(proof.child_pass_count(), stage_passes);
        assert_eq!(proof.demand().is_some(), !empty);
        assert_eq!(source.base.walks.get(), stage_passes);
        assert_eq!(proof.parent().headers().as_ptr(), original_headers);
        assert_eq!(proof.parent().records().as_ptr(), original_parameters);
        let expected = if empty {
            vec![]
        } else if case == 5 {
            vec![
                (0, false, 10001),
                (0, true, 10002),
                (1, false, 10001),
                (1, true, 10002),
            ]
        } else {
            vec![(0, false, 10001), (0, true, 10002)]
        };
        for (record, (pocket, component, copy)) in proof.records().iter().zip(expected) {
            let occurrence = record.occurrence();
            assert!(core::ptr::eq(occurrence.garment(), &garment));
            assert!(
                matches!(occurrence.owner(),DraftingGarmentSemanticOwner::Pocket {index,..} if index==pocket)
            );
            assert_eq!(
                occurrence.selection_location(),
                DraftingSemanticSelectionLocation::Pocket {
                    pocket,
                    field: if component {
                        DraftingPocketSelectionField::Component { index: 0 }
                    } else {
                        DraftingPocketSelectionField::Served
                    }
                }
            );
            let original = match occurrence.field() {
                DraftingGarmentSemanticField::PocketServed(reference) => reference,
                DraftingGarmentSemanticField::PocketComponent { reference, .. } => reference,
                _ => return Err("actual original Pocket field".into()),
            };
            let work = record.work();
            assert!(core::ptr::eq(work.parent(), &admitted_garment));
            assert_eq!(work.target(), *original);
            assert_eq!(work.copy().id(), id(copy));
            assert!(core::ptr::eq(
                work.copy(),
                garment
                    .inputs()
                    .structure()
                    .cut_plan()
                    .copy(id(copy))
                    .ok_or("current copy")?
            ));
            assert!(core::ptr::eq(
                work.piece(),
                garment
                    .inputs()
                    .structure()
                    .pieces()
                    .piece(id(100))
                    .ok_or("current Piece")?
            ));
        }
        let parent = proof.into_parent();
        let walks = source.base.walks.get();
        let repeated = parent.bind_pocket_component_work();
        if empty {
            assert!(repeated.map_err(|e| e.to_string())?.records().is_empty());
        } else {
            let error = repeated.expect_err("recovering parent cannot refund spent work");
            assert!(
                matches!(error.refusal(),DraftingProgramPocketWorkRefusal::Work(cause) if matches!(cause.refusal(),DraftingProgramSemanticWorkRefusal::ChildPasses {reserved,requested,..} if *reserved==passes && *requested==stage_passes))
            );
        }
        assert_eq!(source.base.walks.get(), walks);
    }
    Ok(())
}
