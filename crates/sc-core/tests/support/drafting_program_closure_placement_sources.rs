//! Original Closure placement sources borrow every exact stable role/member/current work parent.
use super::{id, limits, scope, Provider, SourceError, TestResult};
use sc_core::{construction::*, design::*, recipe::*};
use std::error::Error;
#[test]
fn closure_placement_sources_keep_stable_work_original_imports_and_first_late_causes() -> TestResult
{
    let mut baseline = None;
    for case in 0..10u8 {
        let fixture_case = match case {
            8 => 7,
            9 => 8,
            _ => 0,
        };
        let garment = super::closure_placement_work::fixture(fixture_case)?;
        let other = super::closure_placement_work::fixture(0)?;
        let mut source = Provider::new(&garment, other.inputs(), 0)?;
        source.component_import = Some(DraftingSourceRecord::new(
            b"original Closure placement import".to_vec(),
        ));
        source.rules = vec![super::super::RuleRow {
            declaration: SemanticRuleDeclaration::new(
                SemanticRuleDefinition {
                    id: id(882),
                    signature: SemanticRuleSignature::Buttonhole { inputs: vec![] },
                },
                SemanticRuleLimits::new(0, 0, 0),
            )?,
            body: DraftingSourceRecord::new([1, 2, 3, 4]),
        }];
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
        let steps = vec![
            DraftingProgramStep::Semantic { ordinal: 1 },
            DraftingProgramStep::Drafting(DraftingOperationInput::PlaceGrainline(&grainline)),
        ];

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

        let raw_selections =
            DraftingSemanticSelectionInputs::new(vec![DraftingSemanticSelectionRow::Rule {
                ordinal: 1,
                origin: DraftingRuleSelection::Design,
            }]);
        let selections = raw_selections
            .admit(DraftingSemanticSelectionLimits::new(content, 1000))
            .map_err(|e| e.to_string())?;
        let actual = geometry
            .semantic_inputs(&admitted_garment, &selections)
            .map_err(|e| e.to_string())?;
        let mut meter = DesignResourceMeter::new(content);
        actual.charge_resources(&mut meter)?;
        let demand = meter.finish()?;
        let occurrences = garment.pocket_closure_dependencies().count() + 4;
        let passes = if matches!(case, 0 | 8 | 9) {
            128
        } else {
            baseline.ok_or("successful placement-work reservation")? + if case == 1 { 6 } else { 7 }
        };
        let policy = DraftingProgramSemanticWorkLimits::new(
            DraftingProgramSemanticPopulationLimits::new(
                content,
                occurrences,
                2 * occurrences,
                occurrences + 1,
            ),
            passes,
            DraftingProgramSemanticChildContentLimits::new(
                passes as u128 * demand.collection_entries() as u128,
                passes as u128 * demand.text_bytes() as u128,
                passes as u128 * u128::from(demand.counter_bits()),
            ),
            passes as u128 * (demand.collection_entries() + 8).pow(2) as u128,
            occurrences + 1 + passes,
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
        let mut pocket_work = inputs
            .bind_pocket_component_work()
            .map_err(|e| e.to_string())?;
        let pockets = pocket_work
            .bind_pocket_sources()
            .map_err(|e| e.to_string())?;
        let relations = pockets
            .bind_closure_buttonholes()
            .map_err(|e| e.to_string())?;
        let mut placements = relations
            .bind_closure_placement_work()
            .map_err(|e| e.to_string())?;
        let prior = placements
            .parent()
            .parent()
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
        if case == 0 {
            baseline = Some(prior);
        }
        let original_work = placements.records().as_ptr();
        let original_operations = placements.parent().records().as_ptr();
        let count = placements.records().len();
        let stage_passes = if count == 0 { 0 } else { count + 1 };
        source.base.walks.set(0);
        assert_eq!(source.component_calls.get(), 0);
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
            4 => 1,
            5 => 2,
            6 => 3,
            7 => 4,
            _ => 0,
        });
        let result = placements.bind_closure_placement_sources();
        assert_eq!(
            (
                source.calls.get(),
                source.parameter_calls.get(),
                source.numeric_calls.get(),
                source.rule_calls.get()
            ),
            queries
        );
        if matches!(case, 0 | 8 | 9) {
            let proof = result.map_err(|e| e.to_string())?;
            assert_eq!(proof.records().len(), count);
            assert_eq!(proof.work().as_ptr(), original_work);
            assert_eq!(proof.parent().records().as_ptr(), original_operations);
            assert_eq!(proof.child_pass_count(), stage_passes);
            assert_eq!(proof.demand(), if count == 0 { None } else { Some(demand) });
            assert_eq!(source.base.walks.get(), stage_passes);
            assert_eq!(source.component_calls.get(), count);
            for (index, record) in proof.records().iter().enumerate() {
                let work = proof
                    .work()
                    .get(index)
                    .ok_or("stable actual placement work")?;
                assert_eq!(record.work_index(), index);
                assert!(std::ptr::eq(record.work(), work));
                assert!(std::ptr::eq(record.source().work(), work.work()));
                assert_eq!(record.source().creator_position(), None);
                let notion = work.member().notion();
                assert_eq!(work.work().target().copy, notion.copy());
                assert_eq!(work.work().target().piece, notion.piece());
                match record.source().source().origin {
                    DraftingComponentOrigin::Imported(original) => {
                        assert!(std::ptr::eq(original.piece, work.work().piece()));
                        assert!(std::ptr::eq(
                            original.source,
                            source
                                .component_import
                                .as_ref()
                                .ok_or("original import record")?
                        ));
                        assert_eq!(
                            original.source.data().as_slice(),
                            b"original Closure placement import"
                        );
                    }
                    _ => return Err("original imported source required".into()),
                }
            }
            drop(proof);
        } else {
            let error = result.expect_err("whole source class refuses without an accepted prefix");
            assert_eq!(error.source_count(), count);
            assert_eq!(error.work().as_ptr(), original_work);
            assert_eq!(error.parent().records().as_ptr(), original_operations);
            assert_eq!(error.demand().is_some(), case >= 3);
            match (case, error.refusal()) {
                (1, DraftingProgramClosurePlacementSourceRefusal::Work(cause)) => {
                    assert!(
                        matches!(cause.refusal(),DraftingProgramSemanticWorkRefusal::ChildPasses {reserved,requested,..} if *reserved==prior && *requested==stage_passes)
                    );
                    assert_eq!(
                        (source.base.walks.get(), source.component_calls.get()),
                        (0, 0)
                    );
                }
                (
                    2,
                    DraftingProgramClosurePlacementSourceRefusal::Resources(
                        DesignResourceLimitError::Count { location, .. },
                    ),
                ) => {
                    assert_eq!(location.field, "numeric.extra");
                    assert_eq!(
                        (source.base.walks.get(), source.component_calls.get()),
                        (1, 0)
                    );
                }
                (
                    3..=7,
                    DraftingProgramClosurePlacementSourceRefusal::Source { work_index, cause },
                ) => {
                    assert_eq!(*work_index, 1);
                    let work = error.work().get(1).ok_or("original second role work")?;
                    assert!(std::ptr::eq(cause.work(), work.work()));
                    assert_eq!(
                        work.member().owner().role(),
                        sc_core::ontology::ClosurePlacementRole::Second
                    );
                    assert_eq!(source.base.walks.get(), 3);
                    assert_eq!(source.component_calls.get(), if case == 3 { 1 } else { 2 });
                    match (case, cause.refusal()) {
                        (3, DraftingComponentSourceRefusal::Work(child)) => {
                            assert!(cause.returned().is_none());
                            assert!(
                                matches!(child.refusal(),DraftingComponentWorkRefusal::Resources(DesignResourceLimitError::Count {location,..}) if location.field=="rule.late-cache")
                            );
                        }
                        (4, DraftingComponentSourceRefusal::Source(SourceError::Missing)) => {
                            assert!(cause.returned().is_none());
                            assert!(error.source().is_some_and(|c| c.is::<SourceError>()));
                        }
                        (
                            5,
                            DraftingComponentSourceRefusal::Mismatch(
                                DraftingComponentSourceIssue::Piece,
                            ),
                        )
                        | (
                            6,
                            DraftingComponentSourceRefusal::Mismatch(
                                DraftingComponentSourceIssue::Frame,
                            ),
                        )
                        | (
                            7,
                            DraftingComponentSourceRefusal::Mismatch(
                                DraftingComponentSourceIssue::Snapshot,
                            ),
                        ) => assert!(cause.returned().is_some()),
                        _ => return Err(format!("unexpected source cause {cause:?}").into()),
                    }
                }
                _ => return Err(format!("unexpected aggregate cause {error:?}").into()),
            }
            drop(error);
        }
        let reserved = placements
            .parent()
            .parent()
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
        assert_eq!(reserved, prior + if case == 1 { 0 } else { stage_passes });
        if case != 0 && case != 9 {
            let walks = source.base.walks.get();
            let queries = source.component_calls.get();
            let repeated = placements.bind_closure_placement_sources();
            if case == 8 {
                assert!(repeated.map_err(|e| e.to_string())?.records().is_empty());
            } else {
                assert!(matches!(
                    repeated.expect_err("no refunds").refusal(),
                    DraftingProgramClosurePlacementSourceRefusal::Work(_)
                ));
            }
            assert_eq!(
                (source.base.walks.get(), source.component_calls.get()),
                (walks, queries)
            );
        }
    }
    Ok(())
}
