//! Complete original Closure relation class preserves the prior source proof and finite cursor.
use super::{id, limits, scope, Provider, TestResult};
use sc_core::{construction::*, design::*, name::MachineToken, ontology::*, recipe::*, value::*};
fn fixture(case: u8) -> TestResult<DesignGarment<super::EmptyHost>> {
    let logicals = [
        (90, LogicalInputKind::PocketOpening),
        (91, LogicalInputKind::NotionSize),
        (92, LogicalInputKind::NotionSize),
    ]
    .into_iter()
    .map(|(n, kind)| {
        Ok(LogicalInputDeclaration::new(
            LogicalInputDeclarationDefinition {
                id: id(n),
                source: id(n + 1000),
                kind,
                domain: vec![MachineToken::new("option")?],
                state: LogicalInputState::Unknown {
                    observation: id(n + 1100),
                },
            },
            LogicalInputLimits::new(1, 0, 6, 2),
        )?)
    })
    .collect::<TestResult<Vec<_>>>()?;
    let canonical = CanonicalInputs::with_logicals(
        vec![],
        vec![],
        LogicalInputs::new(logicals, LogicalInputInventoryLimits::new(3, 9))?,
    )?;
    let button = NotionSize::Declaration(id(91));
    let mut kinds = vec![ClosureKind::HookAndBar {
        hook: button,
        bar: button,
    }];
    if case != 6 {
        kinds.push(ClosureKind::ButtonAndButtonhole {
            button,
            hole: ButtonholeDerivation { operation: id(881) },
        });
        kinds.push(ClosureKind::ButtonAndButtonhole {
            button: if case == 5 {
                NotionSize::Declaration(id(92))
            } else {
                button
            },
            hole: ButtonholeDerivation {
                operation: id(if case == 4 { 999 } else { 881 }),
            },
        });
    }
    let operation = SemanticOperation::Buttonhole(ButtonholeOperation::new(
        ButtonholeOperationDefinition {
            id: id(881),
            button,
            policy: id(882),
            inputs: vec![],
        },
        ButtonholeOperationLimits::new(0, 0, 0),
    )?);
    super::super::super::super::garment_source::garment_with_semantic_declarations(
        canonical,
        vec![],
        kinds,
        PocketOpening::Declaration(id(90)),
        SemanticRecipe::new(vec![operation], SemanticRecipeLimits::new(1, 1))?,
    )
}
#[test]
fn closure_cohort_keeps_complete_original_relations_first_late_causes_and_same_cursor() -> TestResult
{
    let mut baseline = None;
    for case in 0..7u8 {
        let garment = fixture(case)?;
        let other = fixture(0)?;
        let mut source = Provider::new(&garment, other.inputs(), 0)?;
        source.component_import = Some(DraftingSourceRecord::new(b"original import".to_vec()));
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
        let passes = if case == 0 || case == 6 {
            128
        } else {
            baseline.ok_or("first successful prior reservation")? + if case == 1 { 2 } else { 3 }
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
        let original_work = pockets.work().as_ptr();
        let original_sources = pockets.records().as_ptr();
        let prior = pockets
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
        source.base.walks.set(0);
        let queries = (
            source.calls.get(),
            source.parameter_calls.get(),
            source.numeric_calls.get(),
            source.rule_calls.get(),
            source.component_calls.get(),
        );
        if case == 2 {
            source.base.extra_charge.set(100001);
        }
        if case == 3 {
            source.extra_after.set(Some(3));
            source.extra_entries.set(100001);
        }
        let result = pockets.bind_closure_buttonholes();
        assert_eq!(
            (
                source.calls.get(),
                source.parameter_calls.get(),
                source.numeric_calls.get(),
                source.rule_calls.get(),
                source.component_calls.get()
            ),
            queries
        );
        let parent = if case == 0 || case == 6 {
            let proof = result.map_err(|e| e.to_string())?;
            let count = if case == 6 { 0 } else { 2 };
            assert_eq!(proof.records().len(), count);
            assert_eq!(proof.child_pass_count(), if count == 0 { 0 } else { 3 });
            assert_eq!(proof.demand(), if count == 0 { None } else { Some(demand) });
            for (index, record) in proof.records().iter().enumerate() {
                let relation = record.relation();
                assert!(std::ptr::eq(relation.parent(), &admitted_garment));
                assert_eq!(relation.closure_index(), index + 1);
                assert!(std::ptr::eq(
                    relation.closure(),
                    &garment.closures().closures()[index + 1]
                ));
                assert!(std::ptr::eq(
                    relation.operation().operation(),
                    &garment.inputs().semantic_recipe().operations()[0]
                ));
                assert_eq!(
                    record.occurrence().selection_location(),
                    DraftingSemanticSelectionLocation::Closure {
                        closure: index + 1,
                        field: DraftingClosureSelectionField::Buttonhole,
                    }
                );
                assert!(
                    matches!(record.occurrence().field(),DraftingGarmentSemanticField::ClosureButtonhole(d)
                    if std::ptr::eq(d,relation.source().derivation()))
                );
            }
            assert_eq!(source.base.walks.get(), if count == 0 { 0 } else { 3 });
            proof.into_parent()
        } else {
            let error =
                result.expect_err("whole relation class refuses without an accepted prefix");
            assert_eq!(error.relation_count(), Some(2));
            assert_eq!(error.demand().is_some(), case >= 3);
            match (case, error.refusal()) {
                (1, DraftingProgramClosureButtonholeRefusal::Work(c)) => {
                    assert!(
                        matches!(c.refusal(),DraftingProgramSemanticWorkRefusal::ChildPasses {reserved,requested,..}
                        if *reserved==prior && *requested==3)
                    );
                    assert_eq!(source.base.walks.get(), 0);
                }
                (
                    2,
                    DraftingProgramClosureButtonholeRefusal::Resources(
                        DesignResourceLimitError::Count { location, .. },
                    ),
                ) => {
                    assert_eq!(location.field, "numeric.extra");
                    assert_eq!(source.base.walks.get(), 1);
                }
                (
                    3..=5,
                    DraftingProgramClosureButtonholeRefusal::Relation { occurrence, cause },
                ) => {
                    assert_eq!(
                        occurrence.selection_location(),
                        DraftingSemanticSelectionLocation::Closure {
                            closure: 2,
                            field: DraftingClosureSelectionField::Buttonhole,
                        }
                    );
                    assert!(std::ptr::eq(cause.parent(), &admitted_garment));
                    assert_eq!(cause.closure(), id(20022));
                    assert_eq!(source.base.walks.get(), 3);
                    match (case, cause.refusal()) {
                        (
                            3,
                            DraftingClosureButtonholeRefusal::Resources(
                                DesignResourceLimitError::Count { location, .. },
                            ),
                        ) => assert_eq!(location.field, "rule.late-cache"),
                        (
                            4,
                            DraftingClosureButtonholeRefusal::Operation(
                                SemanticOperationLookupError::Missing { operation, .. },
                            ),
                        ) => assert_eq!(*operation, id(999)),
                        (5, DraftingClosureButtonholeRefusal::ButtonOrigin { source, .. }) => {
                            assert_eq!(*source.button_size(), NotionSize::Declaration(id(92)))
                        }
                        _ => return Err(format!("unexpected child {cause:?}").into()),
                    }
                }
                _ => return Err(format!("unexpected aggregate {error:?}").into()),
            }
            error.into_parent()
        };
        assert_eq!(parent.work().as_ptr(), original_work);
        assert_eq!(parent.records().as_ptr(), original_sources);
        let reserved = parent
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
        assert_eq!(reserved, prior + if case == 1 || case == 6 { 0 } else { 3 });
        if case != 0 {
            let walks = source.base.walks.get();
            let repeated = parent.bind_closure_buttonholes();
            if case == 6 {
                assert!(repeated.map_err(|e| e.to_string())?.records().is_empty());
            } else {
                assert!(matches!(
                    repeated.expect_err("no refunds").refusal(),
                    DraftingProgramClosureButtonholeRefusal::Work(_)
                ));
            }
            assert_eq!(source.base.walks.get(), walks);
        }
    }
    Ok(())
}
