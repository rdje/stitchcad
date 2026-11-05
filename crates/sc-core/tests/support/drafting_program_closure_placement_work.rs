//! Complete actual Closure placements retain original roles and freshly bounded copy/Piece work.
use super::{id, limits, scope, Provider, TestResult};
use sc_core::{construction::*, design::*, name::MachineToken, ontology::*, recipe::*, value::*};
pub(super) fn fixture(case: u8) -> TestResult<DesignGarment<super::EmptyHost>> {
    let logicals = [
        (90, LogicalInputKind::PocketOpening),
        (91, LogicalInputKind::NotionSize),
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
        LogicalInputs::new(logicals, LogicalInputInventoryLimits::new(2, 4))?,
    )?;
    let button = NotionSize::Declaration(id(91));
    let kinds = if case == 7 {
        vec![]
    } else {
        vec![
            ClosureKind::HookAndBar {
                hook: button,
                bar: button,
            },
            ClosureKind::ButtonAndButtonhole {
                button,
                hole: ButtonholeDerivation { operation: id(881) },
            },
            ClosureKind::ButtonAndButtonhole {
                button,
                hole: ButtonholeDerivation { operation: id(881) },
            },
        ]
    };
    let operation = SemanticOperation::Buttonhole(ButtonholeOperation::new(
        ButtonholeOperationDefinition {
            id: id(881),
            button,
            policy: id(882),
            inputs: vec![],
        },
        ButtonholeOperationLimits::new(0, 0, 0),
    )?);
    let mode = match case {
        5 => 1,
        6 => 4,
        8 => 3,
        _ => 0,
    };
    super::super::super::super::garment_source::garment_with_placement_work(
        canonical,
        SemanticRecipe::new(vec![operation], SemanticRecipeLimits::new(1, 1))?,
        kinds,
        mode,
    )
}
#[test]
fn closure_placement_work_keeps_complete_roles_current_members_and_first_causes() -> TestResult {
    let mut baseline = None;
    for case in 0..9u8 {
        let garment = fixture(case)?;
        let other = fixture(0)?;
        let mut source = Provider::new(&garment, other.inputs(), 0)?;
        source.component_import = Some(DraftingSourceRecord::new(
            b"unused original import".to_vec(),
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
        let passes = if matches!(case, 0 | 7 | 8) {
            128
        } else {
            baseline.ok_or("successful prior passes")? + if case == 1 { 12 } else { 13 }
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
        let prior = relations
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
        let original_relations = relations.records().as_ptr();
        let original_pockets = relations.parent().records().as_ptr();
        let count = if case == 7 {
            0
        } else if case == 8 {
            12
        } else {
            6
        };
        let stage_passes = if count == 0 { 0 } else { 2 * count + 1 };
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
        if case == 3 || case == 4 {
            source.extra_after.set(Some(if case == 3 { 4 } else { 5 }));
            source.extra_entries.set(100001);
        }
        let result = relations.bind_closure_placement_work();
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
        let parent = if matches!(case, 0 | 7 | 8) {
            let proof = result.map_err(|e| e.to_string())?;
            assert_eq!(proof.records().len(), count);
            assert_eq!(proof.child_pass_count(), stage_passes);
            assert_eq!(proof.demand(), if count == 0 { None } else { Some(demand) });
            let expected = garment
                .closures()
                .closures()
                .iter()
                .enumerate()
                .flat_map(|(ci, c)| {
                    c.definition()
                        .instances
                        .iter()
                        .enumerate()
                        .flat_map(move |(ii, i)| {
                            [ClosurePlacementRole::First, ClosurePlacementRole::Second]
                                .into_iter()
                                .map(move |role| (ci, c, ii, i, role))
                        })
                });
            for (record, (ci, c, ii, i, role)) in proof.records().iter().zip(expected) {
                assert!(std::ptr::eq(record.member().parent(), &admitted_garment));
                assert!(std::ptr::eq(record.work().parent(), &admitted_garment));
                let owner = record.member().owner();
                assert!(std::ptr::eq(owner.closure(), c));
                assert_eq!(owner.closure_index(), ci);
                assert!(std::ptr::eq(owner.instance(), i));
                assert_eq!(owner.instance_index(), ii);
                assert_eq!(owner.role(), role);
                let notion = garment
                    .notions()
                    .placement(i.placement(role))
                    .ok_or("actual notion")?;
                assert!(std::ptr::eq(record.member().notion(), notion));
                assert_eq!(
                    record.work().target(),
                    PocketPieceRef {
                        copy: notion.copy(),
                        piece: notion.piece()
                    }
                );
                assert!(std::ptr::eq(
                    record.work().copy(),
                    garment
                        .inputs()
                        .structure()
                        .cut_plan()
                        .copy(notion.copy())
                        .ok_or("actual current copy")?
                ));
                assert!(std::ptr::eq(
                    record.work().piece(),
                    garment
                        .inputs()
                        .structure()
                        .pieces()
                        .piece(notion.piece())
                        .ok_or("actual current Piece")?
                ));
                assert_eq!(
                    record.occurrence().selection_location(),
                    DraftingSemanticSelectionLocation::Closure {
                        closure: ci,
                        field: DraftingClosureSelectionField::Instance { index: ii },
                    }
                );
            }
            assert_eq!(source.base.walks.get(), stage_passes);
            proof.into_parent()
        } else {
            let error = result.expect_err("complete placement work must refuse without a prefix");
            assert_eq!(error.component_count(), Some(count));
            assert_eq!(error.demand().is_some(), case >= 3);
            match (case, error.refusal()) {
                (1, DraftingProgramClosurePlacementWorkRefusal::Work(c)) => {
                    assert!(
                        matches!(c.refusal(),DraftingProgramSemanticWorkRefusal::ChildPasses {reserved,requested,..} if *reserved==prior && *requested==13)
                    );
                    assert_eq!(source.base.walks.get(), 0);
                }
                (
                    2,
                    DraftingProgramClosurePlacementWorkRefusal::Resources(
                        DesignResourceLimitError::Count { location, .. },
                    ),
                ) => {
                    assert_eq!(location.field, "numeric.extra");
                    assert_eq!(source.base.walks.get(), 1);
                }
                (
                    3 | 5,
                    DraftingProgramClosurePlacementWorkRefusal::Member {
                        occurrence,
                        request,
                        cause,
                    },
                ) => {
                    let ci = if case == 3 { 0 } else { 1 };
                    assert_eq!(
                        occurrence.selection_location(),
                        DraftingSemanticSelectionLocation::Closure {
                            closure: ci,
                            field: DraftingClosureSelectionField::Instance { index: 0 },
                        }
                    );
                    assert_eq!(request.role, ClosurePlacementRole::Second);
                    assert_eq!(cause.request(), *request);
                    assert!(std::ptr::eq(cause.parent(), &admitted_garment));
                    match (case, cause.refusal()) {
                        (
                            3,
                            DraftingClosurePlacementMemberRefusal::Resources(
                                DesignResourceLimitError::Count { location, .. },
                            ),
                        ) => assert_eq!(location.field, "rule.late-cache"),
                        (5, DraftingClosurePlacementMemberRefusal::MissingNotion { owner }) => {
                            assert_eq!(owner.target(), id(20011))
                        }
                        _ => return Err(format!("unexpected member cause {cause:?}").into()),
                    }
                    assert_eq!(source.base.walks.get(), if case == 3 { 4 } else { 8 });
                }
                (
                    4 | 6,
                    DraftingProgramClosurePlacementWorkRefusal::Component {
                        occurrence,
                        member,
                        cause,
                    },
                ) => {
                    assert_eq!(
                        occurrence.selection_location(),
                        DraftingSemanticSelectionLocation::Closure {
                            closure: 0,
                            field: DraftingClosureSelectionField::Instance { index: 0 },
                        }
                    );
                    assert_eq!(member.owner().role(), ClosurePlacementRole::Second);
                    assert!(std::ptr::eq(member.parent(), &admitted_garment));
                    assert!(std::ptr::eq(cause.parent(), &admitted_garment));
                    assert_eq!(
                        cause.target(),
                        PocketPieceRef {
                            copy: member.notion().copy(),
                            piece: member.notion().piece()
                        }
                    );
                    match (case, cause.refusal()) {
                        (
                            4,
                            DraftingComponentWorkRefusal::Resources(
                                DesignResourceLimitError::Count { location, .. },
                            ),
                        ) => assert_eq!(location.field, "rule.late-cache"),
                        (6, DraftingComponentWorkRefusal::MissingCopy) => {
                            assert_eq!(cause.target().copy, id(10002))
                        }
                        _ => return Err(format!("unexpected component cause {cause:?}").into()),
                    }
                    assert_eq!(source.base.walks.get(), 5);
                }
                _ => return Err(format!("unexpected work error {error:?}").into()),
            }
            error.into_parent()
        };
        assert_eq!(parent.records().as_ptr(), original_relations);
        assert_eq!(parent.parent().records().as_ptr(), original_pockets);
        let reserved = parent
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
        if !matches!(case, 0 | 8) {
            let walks = source.base.walks.get();
            let repeated = parent.bind_closure_placement_work();
            if case == 7 {
                assert!(repeated.map_err(|e| e.to_string())?.records().is_empty());
            } else {
                assert!(matches!(
                    repeated.expect_err("no refund").refusal(),
                    DraftingProgramClosurePlacementWorkRefusal::Work(_)
                ));
            }
            assert_eq!(source.base.walks.get(), walks);
        }
    }
    Ok(())
}
