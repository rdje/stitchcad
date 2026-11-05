//! Complete exact Design/Profile rule sources with original work parents and one shared cursor.
use super::{id, limits, scope, Provider, RuleSourceView, TestResult};
use sc_core::{construction::*, design::*, recipe::*, value::*};
#[test]
fn complete_rule_sources_preserve_original_parents_bodies_and_first_late_causes() -> TestResult {
    for case in 0..17u8 {
        let empty = case == 15;
        let button = !empty;
        let garment = if empty {
            super::super::super::root_numeric_suppliers::without_semantics_fixture()?
        } else {
            super::super::super::root_numeric_suppliers::fixture(false, false, 2, true)?
        };
        let other = if empty {
            super::super::super::root_numeric_suppliers::without_semantics_fixture()?
        } else {
            super::super::super::root_numeric_suppliers::fixture(false, false, 2, true)?
        };
        let mut source = Provider::new(&garment, other.inputs(), 0)?;
        if case == 9 {
            source.wrong_pin = super::super::pin("original")?;
        }
        source.rules = [(901, false), (902, true), (999, false), (902, false)]
            .into_iter()
            .map(|(n, button)| {
                Ok(super::super::RuleRow {
                    declaration: SemanticRuleDeclaration::new(
                        SemanticRuleDefinition {
                            id: id(n),
                            signature: if button {
                                SemanticRuleSignature::Buttonhole { inputs: vec![] }
                            } else {
                                SemanticRuleSignature::Logical {
                                    result: LogicalInputKind::PocketOpening,
                                    inputs: vec![],
                                }
                            },
                        },
                        SemanticRuleLimits::new(0, 0, 0),
                    )?,
                    body: DraftingSourceRecord::new([1, 2, 3, 4]),
                })
            })
            .collect::<TestResult<Vec<_>>>()?;
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
        let mut steps = vec![];
        if !empty {
            steps.push(DraftingProgramStep::Semantic { ordinal: 1 });
        }
        steps.push(DraftingProgramStep::Drafting(
            DraftingOperationInput::PlaceGrainline(&grainline),
        ));

        if button {
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
        let selected = super::super::pin("original")?;
        let wrong = super::super::pin("unaccepted")?;
        let rule_pin = if case == 99 { &wrong } else { &selected };
        // Deliberately scrambled request rows preserve actual row indices,not traversal order.
        let mut rows = vec![];
        if !empty {
            rows.push(DraftingSemanticSelectionRow::Rule {
                ordinal: 2,
                origin: DraftingRuleSelection::Profile(rule_pin),
            });
            rows.push(DraftingSemanticSelectionRow::Profile {
                location: DraftingSemanticSelectionLocation::Semantic {
                    ordinal: 1,
                    field: DraftingSemanticSelectionField::LogicalInput { index: 6 },
                },
                selected: &selected,
            });
            rows.push(DraftingSemanticSelectionRow::Rule {
                ordinal: 1,
                origin: if case == 14 {
                    DraftingRuleSelection::Profile(rule_pin)
                } else {
                    DraftingRuleSelection::Design
                },
            });
            rows.push(DraftingSemanticSelectionRow::Profile {
                location: DraftingSemanticSelectionLocation::Semantic {
                    ordinal: 1,
                    field: DraftingSemanticSelectionField::LogicalInput { index: 5 },
                },
                selected: &selected,
            });
            rows.push(DraftingSemanticSelectionRow::Profile {
                location: DraftingSemanticSelectionLocation::Semantic {
                    ordinal: 2,
                    field: DraftingSemanticSelectionField::ButtonholeButton,
                },
                selected: &selected,
            });
        }
        let row_count = rows.len();
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
        let occurrences = if empty { 1 } else { 12 };
        let passes = if empty {
            0
        } else if case == 1 {
            23
        } else if case == 14 {
            25
        } else {
            24
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
        let original_work_ptr = work.records().as_ptr();
        assert_eq!(
            work.inputs()
                .root()
                .logical()
                .members()
                .owner()
                .selections()
                .reserved_child_passes(),
            if empty { 0 } else { 20 }
        );
        source.rule_calls.set(0);
        source.calls.set(0);
        source.parameter_calls.set(0);
        source.base.walks.set(0);
        source.rule_mode.set(case);
        if case == 2 {
            source.base.extra_charge.set(100001);
        }
        if case == 3 || case == 16 {
            source.extra_after.set(Some(3));
            source.extra_entries.set(if case == 3 {
                100001
            } else {
                demand.collection_entries()
            });
        }
        let result = work.bind_rule_sources();
        assert_eq!(source.parameter_calls.get(), 0);
        if case != 0 && case != 14 && case != 15 {
            let error = result.expect_err(
                "complete class refuses original cause without successful source prefix",
            );
            assert_eq!(error.source_count(), 2);
            assert_eq!(error.works().as_ptr(), original_work_ptr);
            assert_eq!(error.inputs().operation_input_count(), 8);
            let cursor = error
                .inputs()
                .root()
                .logical()
                .members()
                .owner()
                .selections();
            if case == 1 {
                assert!(
                    matches!(error.refusal(),DraftingProgramRuleSourceRefusal::Work(cause) if matches!(cause.refusal(),DraftingProgramSemanticWorkRefusal::ChildPasses {allowed:23,reserved:20,requested:4}))
                );
                assert_eq!(cursor.reserved_child_passes(), 20);
                assert_eq!(
                    (
                        source.base.walks.get(),
                        source.rule_calls.get(),
                        source.calls.get()
                    ),
                    (0, 0, 0)
                );
                assert!(error.demand().is_none());
            } else if case == 2 {
                assert!(
                    matches!(error.refusal(),DraftingProgramRuleSourceRefusal::Resources(DesignResourceLimitError::Count {location,..}) if location.field=="numeric.extra")
                );
                assert_eq!(cursor.reserved_child_passes(), 24);
                assert_eq!(
                    (
                        source.base.walks.get(),
                        source.rule_calls.get(),
                        source.calls.get()
                    ),
                    (1, 0, 0)
                );
                assert!(error.demand().is_none());
            } else {
                let DraftingProgramRuleSourceRefusal::Source { work_index, cause } =
                    error.refusal()
                else {
                    return Err("expected original late source cause".into());
                };
                assert_eq!(*work_index, 1);
                let parent = error.works().get(1).ok_or("original refused work")?;
                assert!(std::ptr::eq(cause.work(), parent.work()));
                assert!(error.demand().is_some());
                assert_eq!(parent.request().row_index(), 0);
                assert_eq!(cursor.reserved_child_passes(), 24);
                if case == 3 {
                    assert!(
                        matches!(cause.refusal(),DraftingRuleSourceRefusal::Work(e) if matches!(e.refusal(),DraftingRuleWorkRefusal::Resources(DesignResourceLimitError::Count {location,..}) if location.field=="rule.late-cache"))
                    );
                } else if case == 16 {
                    assert!(
                        matches!(cause.refusal(),DraftingRuleSourceRefusal::Work(e) if matches!(e.refusal(),DraftingRuleWorkRefusal::MatchingInspections {allowed,required} if *allowed==per_pass && required>allowed))
                    );
                } else if case == 4 {
                    assert!(
                        matches!(cause.refusal(),DraftingRuleSourceRefusal::Profile(e) if matches!(e.refusal(),DraftingProfileSourceRefusal::Mismatch(DraftingProfileSourceIssue::SelectedPin(sc_core::ontology::SelectedProfilePinField::ProfileId))))
                    );
                } else if case == 13 {
                    assert!(matches!(
                        cause.refusal(),
                        DraftingRuleSourceRefusal::Source(super::super::SourceError::Missing)
                    ));
                } else {
                    let expected = match case {
                        5 => DraftingRuleSourceIssue::Root,
                        6 => DraftingRuleSourceIssue::Origin,
                        7 => DraftingRuleSourceIssue::Snapshot,
                        8 => DraftingRuleSourceIssue::Inventory,
                        9 => DraftingRuleSourceIssue::SelectedPin,
                        10 => DraftingRuleSourceIssue::Profile,
                        11 => DraftingRuleSourceIssue::Rule,
                        12 => DraftingRuleSourceIssue::Kind,
                        _ => return Err("unhandled mode".into()),
                    };
                    assert!(
                        matches!(cause.refusal(),DraftingRuleSourceRefusal::Mismatch(actual) if *actual==expected)
                    );
                    assert!(cause.returned().is_some());
                    if case == 9 {
                        let DraftingRuleOrigin::Profile(actual) =
                            cause.returned().ok_or("returned source")?.origin
                        else {
                            return Err("actual Profile origin".into());
                        };
                        assert_eq!(actual.selected, &source.selected);
                        assert!(!std::ptr::eq(actual.selected, &source.selected));
                    }
                }
                assert_eq!(
                    (
                        source.base.walks.get(),
                        source.rule_calls.get(),
                        source.calls.get()
                    ),
                    if case == 3 || case == 16 {
                        (3, 1, 0)
                    } else if case == 4 {
                        (4, 1, 1)
                    } else {
                        (4, 2, 1)
                    }
                );
            }
            continue;
        }
        let proof = result.map_err(|e| e.to_string())?;
        let profiles = if empty {
            0
        } else if case == 14 {
            2
        } else {
            1
        };
        let stage = if empty { 0 } else { 3 + profiles };
        assert_eq!(proof.child_pass_count(), stage);
        assert_eq!(proof.works().as_ptr(), original_work_ptr);
        assert_eq!(proof.records().len(), if empty { 0 } else { 2 });
        assert_eq!(proof.demand().is_some(), !empty);
        assert_eq!(
            (
                source.base.walks.get(),
                source.rule_calls.get(),
                source.calls.get()
            ),
            (stage, if empty { 0 } else { 2 }, profiles)
        );
        assert_eq!(
            proof
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
            let parent = proof.works().get(index).ok_or("original parent")?;
            assert_eq!(record.work_index(), index);
            assert!(std::ptr::eq(record.work(), parent));
            assert!(std::ptr::eq(record.source().work(), parent.work()));
            let original = source.rules.get(index).ok_or("original rule row")?;
            let actual = record.source().source();
            assert!(std::ptr::eq(actual.declaration, &original.declaration));
            let body: &DraftingSourceRecord<[u8]> = &original.body;
            assert!(actual.body.is_same_record(body));
            assert!(std::ptr::eq(actual.origin.root(), garment.inputs()));
            assert!(actual
                .origin
                .snapshot()
                .is_same_record(source.base.snapshot()));
            if let Some(profile) = record.source().profile_source() {
                let DraftingRuleOrigin::Profile(origin) = actual.origin else {
                    return Err("original Profile origin".into());
                };
                assert!(std::ptr::eq(profile.selected, &source.selected));
                assert!(std::ptr::eq(origin.selected, profile.selected));
                assert!(!std::ptr::eq(profile.selected, &selected));
                assert!(std::ptr::eq(origin.inputs, profile.inputs));
                assert!(origin.profile.is_same_record(profile.profile));
            }
        }
        drop(proof);
        if !empty {
            let refusal = work
                .bind_rule_sources()
                .expect_err("spent cursor cannot re-run complete source work");
            assert!(matches!(
                refusal.refusal(),
                DraftingProgramRuleSourceRefusal::Work(_)
            ));
            assert_eq!(
                (
                    source.base.walks.get(),
                    source.rule_calls.get(),
                    source.calls.get()
                ),
                (stage, 2, profiles)
            );
        }
    }
    Ok(())
}
