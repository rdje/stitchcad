//! Original complete rule/policy work with explicit origins and one existing input/cursor owner.
use super::{
    id, limits, scope, EmptyHost, Frame, ParameterProvider, Provider, Snapshot, SourceError,
    TestResult,
};
use sc_core::{construction::*, design::*, recipe::*};
#[path = "drafting_program_garment_root_logical.rs"]
mod garment_root_logical;
#[path = "drafting_program_garment_root_members.rs"]
mod garment_root_members;
#[path = "drafting_program_garment_root_numeric.rs"]
mod garment_root_numeric;
#[path = "drafting_program_rule_signatures.rs"]
mod signatures;
#[path = "drafting_program_rule_sources.rs"]
mod sources;
impl DraftingRuleProvider<EmptyHost> for Provider<'_> {
    type RuleBody = [u8];
    fn rule_source<'s>(
        &'s self,
        request: DraftingRuleRequest<'_>,
    ) -> Result<DraftingRuleResponse<'s, EmptyHost, Self>, SourceError> {
        self.rule_calls.set(self.rule_calls.get() + 1);
        let mode = self.rule_mode.get();
        let late = self.rule_calls.get() == 2;
        if mode == 13 && late {
            return Err(SourceError::Missing);
        }
        let row = if late && mode == 11 {
            self.rules.get(2)
        } else if late && mode == 12 {
            self.rules.get(3)
        } else {
            self.rules
                .iter()
                .find(|r| r.declaration.id() == request.rule)
        }
        .ok_or(SourceError::Missing)?;
        let root = if late && mode == 5 {
            self.other
        } else {
            self.base.garment.inputs()
        };
        let snapshot = if late && mode == 7 {
            self.base.alternate.snapshot()
        } else {
            self.base.snapshot()
        };
        let origin: DraftingRuleOrigin<'s, EmptyHost, Snapshot, [u8]> =
            if matches!(request.origin, DraftingRuleSelection::Design) || (late && mode == 6) {
                DraftingRuleOrigin::Design { root, snapshot }
            } else {
                DraftingRuleOrigin::Profile(DraftingProfileSource {
                    root,
                    snapshot,
                    selected: if late && mode == 9 {
                        &self.wrong_pin
                    } else {
                        &self.selected
                    },
                    inputs: if late && mode == 8 {
                        self.other.inputs()
                    } else {
                        &self.inputs
                    },
                    profile: if late && mode == 10 {
                        &row.body
                    } else {
                        &self.body
                    },
                })
            };
        Ok(DraftingRuleSource {
            declaration: &row.declaration,
            body: &row.body,
            origin,
        })
    }
}
trait RuleProvider: ParameterProvider + DraftingRuleProvider<EmptyHost, RuleBody = [u8]> {}
impl RuleProvider for Provider<'_> {}
type RuleSourceView<'a> = dyn RuleProvider<
        Frame = Frame,
        Snapshot = Snapshot,
        Geometry = [u8],
        Error = SourceError,
        Profile = [u8],
    > + 'a;
#[test]
fn complete_rule_work_retains_original_operations_origins_and_fresh_finite_cursor() -> TestResult {
    // Mixed Design/Profile,short cursor,fresh whole/late child resources,late child matching,
    // still-unaccepted mismatched Profile origin,empty class,repeated explicit Profile origins.
    for case in 0..8u8 {
        let empty = case == 6;
        let button = !empty;
        let garment = if empty {
            super::super::root_numeric_suppliers::without_semantics_fixture()?
        } else {
            super::super::root_numeric_suppliers::fixture(false, false, 2, true)?
        };
        let other = if empty {
            super::super::root_numeric_suppliers::without_semantics_fixture()?
        } else {
            super::super::root_numeric_suppliers::fixture(false, false, 2, true)?
        };
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
        let selected = super::pin("original")?;
        let wrong = super::pin("unaccepted")?;
        let rule_pin = if case == 5 { &wrong } else { &selected };
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
                origin: if case == 7 {
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
            19
        } else {
            20
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
        let before = if empty { 0 } else { 17 };
        assert_eq!(
            inputs
                .root()
                .logical()
                .members()
                .owner()
                .selections()
                .reserved_child_passes(),
            before
        );
        let queries = (source.calls.get(), source.parameter_calls.get());
        source.base.walks.set(0);
        if case == 2 {
            source.base.extra_charge.set(100001);
        }
        if case == 3 || case == 4 {
            source.extra_after.set(Some(3));
            source.extra_entries.set(if case == 3 {
                100001
            } else {
                demand.collection_entries()
            });
        }
        let result = inputs.admit_rule_work();
        assert_eq!(source.rule_calls.get(), 0);
        assert_eq!((source.calls.get(), source.parameter_calls.get()), queries);
        if case == 1 || case == 2 || case == 3 || case == 4 {
            let error = result
                .expect_err("complete original rule work refuses without a successful prefix");
            assert_eq!(error.rule_count(), Some(2));
            assert_eq!(error.owner().operation_input_count(), 8);
            assert!(std::ptr::eq(
                error.owner().root().logical().members().owner().inputs(),
                &actual
            ));
            let cursor = error
                .owner()
                .root()
                .logical()
                .members()
                .owner()
                .selections();
            if case == 1 {
                assert!(
                    matches!(error.refusal(),DraftingProgramRuleWorkRefusal::Work(cause) if matches!(cause.refusal(),DraftingProgramSemanticWorkRefusal::ChildPasses {allowed:19,reserved:17,requested:3}))
                );
                assert_eq!(cursor.reserved_child_passes(), before);
                assert_eq!(source.base.walks.get(), 0);
                assert!(error.demand().is_none());
            } else if case == 2 {
                assert!(
                    matches!(error.refusal(),DraftingProgramRuleWorkRefusal::Resources(DesignResourceLimitError::Count {location,..}) if location.field=="numeric.extra")
                );
                assert_eq!(cursor.reserved_child_passes(), 20);
                assert_eq!(source.base.walks.get(), 1);
                assert!(error.demand().is_none());
            } else {
                let DraftingProgramRuleWorkRefusal::Child { request, cause } = error.refusal()
                else {
                    return Err("expected original late child work cause".into());
                };
                assert_eq!(
                    (request.ordinal(), request.position(), request.row_index()),
                    (2, 3, 0)
                );
                assert_eq!(cause.operation(), id(81));
                assert!(error.demand().is_some());
                assert_eq!(cursor.reserved_child_passes(), 20);
                assert_eq!(source.base.walks.get(), 3);
                if case == 3 {
                    assert!(
                        matches!(cause.refusal(),DraftingRuleWorkRefusal::Resources(DesignResourceLimitError::Count {location,..}) if location.field=="rule.late-cache")
                    );
                } else {
                    assert!(
                        matches!(cause.refusal(),DraftingRuleWorkRefusal::MatchingInspections {allowed,required} if *allowed==per_pass && required>allowed)
                    );
                }
            }
            continue;
        }
        let proof = result.map_err(|e| e.to_string())?;
        assert_eq!(proof.child_pass_count(), if empty { 0 } else { 3 });
        assert_eq!(proof.records().len(), if empty { 0 } else { 2 });
        assert_eq!(
            proof.inputs().operation_input_count(),
            if empty { 0 } else { 8 }
        );
        assert_eq!(proof.demand().is_some(), !empty);
        assert_eq!(source.base.walks.get(), if empty { 0 } else { 3 });
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
            let request = record.request();
            let work = record.work();
            assert_eq!(
                (request.ordinal(), request.position(), request.row_index()),
                if index == 0 { (1, 1, 2) } else { (2, 3, 0) }
            );
            let original = garment
                .inputs()
                .semantic_recipe()
                .operations()
                .get(index)
                .ok_or("actual original operation")?;
            assert!(std::ptr::eq(request.operation(), original));
            assert!(std::ptr::eq(work.operation().operation(), original));
            assert_eq!(work.operation().index(), index);
            assert!(std::ptr::eq(work.context().source(), source_view));
            assert_eq!(work.limits().content(), content);
            assert_eq!(work.limits().matching_inspections(), per_pass);
            assert_eq!(
                work.shape().matching_inspection_bound(),
                (work.demand().collection_entries() + 8).pow(2)
            );
            assert_eq!(work.request().rule, id(if index == 0 { 901 } else { 902 }));
            match (request.origin(), work.selection()) {
                (DraftingRuleSelection::Profile(a), DraftingRuleSelection::Profile(b)) => {
                    assert!(std::ptr::eq(a, rule_pin));
                    assert!(std::ptr::eq(a, b));
                }
                (DraftingRuleSelection::Design, DraftingRuleSelection::Design) => {
                    assert!(index == 0 && case != 7)
                }
                _ => return Err("original explicit origin replaced".into()),
            }
            match (request.field(), original) {
                (
                    DraftingSemanticField::LogicalRule(reference),
                    SemanticOperation::LogicalDerivation(operation),
                ) => assert!(std::ptr::eq(reference, &operation.definition().rule)),
                (
                    DraftingSemanticField::ButtonholePolicy(reference),
                    SemanticOperation::Buttonhole(operation),
                ) => assert!(std::ptr::eq(reference, &operation.definition().policy)),
                _ => return Err("original rule field replaced".into()),
            }
        }
    }
    Ok(())
}
