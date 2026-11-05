//! Original Root garment numeric Derived targets retain exact checked-source/acyclic owners.
use super::{id, limits, scope, Provider, RuleSourceView, TestResult};
use sc_core::{construction::*, design::*, name::MachineToken, ontology::*, recipe::*, value::*};
use sc_units::Length;
fn fixture(case: u8) -> TestResult<DesignGarment<super::EmptyHost>> {
    let canonical = CanonicalInputs::with_logicals(
        vec![LengthDeclaration::new(LengthDeclarationDefinition {
            id: id(10),
            source: id(1010),
            state: if case == 1 {
                LengthState::Unknown {
                    observation: id(1110),
                }
            } else {
                LengthState::Derived { formula: id(500) }
            },
        })?],
        vec![ScalarInputDeclaration::new(
            ScalarInputDeclarationDefinition {
                id: id(20),
                source: id(1020),
                kind: ScalarInputKind::Length,
                state: if case == 1 {
                    ScalarInputState::Unknown {
                        observation: id(1120),
                    }
                } else {
                    ScalarInputState::Derived { formula: id(500) }
                },
            },
        )?],
        LogicalInputs::new(
            [
                (90, LogicalInputKind::PocketOpening),
                (91, LogicalInputKind::NotionSize),
            ]
            .into_iter()
            .map(|(n, kind)| {
                Ok(LogicalInputDeclaration::new(
                    LogicalInputDeclarationDefinition {
                        id: id(n),
                        source: id(1000 + n),
                        kind,
                        domain: vec![MachineToken::new("private_option")?],
                        state: LogicalInputState::Unknown {
                            observation: id(1100 + n),
                        },
                    },
                    LogicalInputLimits::new(1, 0, 14, 2),
                )?)
            })
            .collect::<TestResult<Vec<_>>>()?,
            LogicalInputInventoryLimits::new(2, 4),
        )?,
    )?;
    let parameters = [
        (7101, CanonicalInputTarget::Length(id(10))),
        (
            7102,
            if case == 4 {
                CanonicalInputTarget::Length(id(10))
            } else {
                CanonicalInputTarget::Scalar {
                    declaration: id(20),
                    kind: ScalarInputKind::Length,
                }
            },
        ),
    ]
    .into_iter()
    .enumerate()
    .map(|(index, (n, target))| {
        Ok(DesignParameterDefinition {
            id: id(n),
            token: MachineToken::new(format!("zipper_{index}"))?,
            target,
        })
    })
    .collect::<TestResult<Vec<_>>>()?;
    let kinds = vec![
        ClosureKind::HookAndBar {
            hook: NotionSize::Declaration(id(91)),
            bar: NotionSize::Declaration(id(91)),
        },
        ClosureKind::ButtonAndButtonhole {
            button: NotionSize::Declaration(id(91)),
            hole: ButtonholeDerivation { operation: id(880) },
        },
        ClosureKind::CentredZipper {
            length: ClosureLength::Explicit {
                parameter: id(7100),
                value: Length::from_micrometres(15000)?,
            },
        },
        ClosureKind::CentredZipper {
            length: ClosureLength::Formula(id(7101)),
        },
        ClosureKind::CentredZipper {
            length: ClosureLength::Formula(id(7102)),
        },
    ];
    let sources = FormulaSources::new(
        vec![FormulaSource::new(
            id(500),
            FormulaExpressionDocument::new(if case == 5 { "1==1" } else { "1 mm" }.into())?,
        )],
        FormulaSourceLimits::new(1, 1000, 1000),
    )?;
    super::super::super::super::garment_source::garment_with_formula_declarations(
        canonical,
        parameters,
        kinds,
        PocketOpening::Declaration(id(90)),
        sources,
    )
}
#[test]
fn garment_root_numeric_suppliers_retain_original_parameter_targets_checked_sources_and_graph(
) -> TestResult {
    for case in 0..6u8 {
        let garment = fixture(case)?;
        let other = fixture(case)?;
        let original_length = garment
            .inputs()
            .inputs()
            .length(id(10))
            .ok_or("actual original Length")?;
        let original_scalar = garment
            .inputs()
            .inputs()
            .scalar(id(20))
            .ok_or("actual original Scalar Length")?;
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
        if case == 5 {
            let error = namespace
                .check_root_sources(FormulaSourceCheckLimits::new(1, 1000, 2, 10000))
                .expect_err("original source kind refuses before numeric supplier owner exists");
            assert!(
                matches!(error.cause().refusal(),FormulaSourceCheckRefusal::TargetKind {source_index:0,target,expected:FormulaKind::Length,actual:FormulaKind::Boolean} if target.length().is_some_and(|original|std::ptr::eq(original,original_length)))
            );
            continue;
        }
        let sources = namespace
            .check_root_sources(FormulaSourceCheckLimits::new(1, 1000, 2, 10000))
            .map_err(|e| e.cause().to_string())?;
        let dependencies = sources
            .proof()
            .prepare_dependencies(FormulaSourceDependencyLimits::new(1, 0, 2))
            .map_err(|e| e.to_string())?;
        let graph = dependencies
            .admit_acyclic(FormulaSourceCycleLimits::new(1, 2))
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

        let raw_selections = DraftingSemanticSelectionInputs::new(vec![]);
        let selections = raw_selections
            .admit(DraftingSemanticSelectionLimits::new(content, 0))
            .map_err(|e| e.to_string())?;
        let actual = geometry
            .semantic_inputs(&admitted_garment, &selections)
            .map_err(|e| e.to_string())?;
        let mut meter = DesignResourceMeter::new(content);
        actual.charge_resources(&mut meter)?;
        let demand = meter.finish()?;
        let occurrences = garment.pocket_closure_dependencies().count() + 1;
        let passes = if case == 1 {
            7
        } else if case == 2 {
            8
        } else {
            9
        };
        let per_pass = (demand.collection_entries() + 8).pow(2);
        let policy = DraftingProgramSemanticWorkLimits::new(
            DraftingProgramSemanticPopulationLimits::new(content, occurrences, 0, occurrences),
            passes,
            DraftingProgramSemanticChildContentLimits::new(
                passes as u128 * demand.collection_entries() as u128,
                passes as u128 * demand.text_bytes() as u128,
                passes as u128 * u128::from(demand.counter_bits()),
            ),
            passes as u128 * per_pass as u128,
            occurrences + passes,
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
        let mut bound = work.bind_rule_sources().map_err(|e| e.to_string())?;
        let signatures = bound.bind_rule_signatures().map_err(|e| e.to_string())?;
        let members = signatures
            .bind_garment_root_input_members()
            .map_err(|e| e.to_string())?;
        let logical = members
            .bind_garment_root_logical_suppliers()
            .map_err(|e| e.to_string())?;
        assert_eq!(logical.members().records().len(), 7);
        assert!(logical.records().is_empty());
        assert_eq!(
            logical
                .members()
                .owner()
                .inputs()
                .root()
                .logical()
                .members()
                .owner()
                .selections()
                .reserved_child_passes(),
            7
        );
        source.base.walks.set(0);
        let queries = (
            source.calls.get(),
            source.parameter_calls.get(),
            source.rule_calls.get(),
        );
        if case == 3 {
            source.base.extra_charge.set(100001);
        }
        let result = logical.bind_garment_root_numeric_suppliers();
        assert_eq!(
            (
                source.calls.get(),
                source.parameter_calls.get(),
                source.rule_calls.get()
            ),
            queries
        );
        if case == 2 || case == 3 {
            let error = result.expect_err("complete numeric class refuses without supplier prefix");
            assert_eq!(error.supplier_count(), Some(2));
            assert_eq!(error.owner().members().records().len(), 7);
            let cursor = error
                .owner()
                .members()
                .owner()
                .inputs()
                .root()
                .logical()
                .members()
                .owner()
                .selections();
            assert!(error.demand().is_none());
            if case == 2 {
                assert!(
                    matches!(error.refusal(),DraftingProgramGarmentRootNumericSupplierRefusal::Work(cause) if matches!(cause.refusal(),DraftingProgramSemanticWorkRefusal::ChildPasses {allowed:8,reserved:7,requested:2}))
                );
                assert_eq!(cursor.reserved_child_passes(), 7);
                assert_eq!(source.base.walks.get(), 0);
            } else {
                assert!(
                    matches!(error.refusal(),DraftingProgramGarmentRootNumericSupplierRefusal::Resources(DesignResourceLimitError::Count {location,..}) if location.field=="numeric.extra")
                );
                assert_eq!(cursor.reserved_child_passes(), 9);
                assert_eq!(source.base.walks.get(), 1);
            }
            continue;
        }
        let proof = result.map_err(|e| e.to_string())?;
        let count = if case == 1 { 0 } else { 2 };
        assert_eq!(proof.records().len(), count);
        assert_eq!(proof.child_pass_count(), count);
        assert_eq!(proof.demand(), if case == 1 { None } else { Some(demand) });
        assert_eq!(source.base.walks.get(), usize::from(case != 1));
        assert_eq!(
            proof
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
        for (index, record) in proof.records().iter().copied().enumerate() {
            assert_eq!(record.member_index(), 5 + index);
            let member = proof
                .logical()
                .members()
                .records()
                .get(record.member_index())
                .ok_or("actual Root numeric consumer")?;
            let DraftingProgramGarmentRootInputSource::FormulaLength {
                parameter,
                source: actual,
                ..
            } = member.source()
            else {
                return Err("actual parameter-bound Root member".into());
            };
            assert_eq!(parameter.id(), id(7101 + index as u128));
            assert!(std::ptr::eq(record.graph(), &graph));
            assert!(std::ptr::eq(
                record.source(),
                sources
                    .proof()
                    .source(id(500))
                    .ok_or("original checked source")?
            ));
            assert!(std::ptr::eq(
                record.source().normalized().source(),
                record.input().source()
            ));
            assert_eq!(record.input().source().id(), id(500));
            assert_eq!(actual.binding().target(), parameter.definition().target);
            if index == 0 || case == 4 {
                assert!(record
                    .input()
                    .length()
                    .is_some_and(|original| std::ptr::eq(original, original_length)));
                assert!(record.input().scalar().is_none());
            } else {
                assert!(record
                    .input()
                    .scalar()
                    .is_some_and(|original| std::ptr::eq(original, original_scalar)));
                assert!(record.input().length().is_none());
            }
        }
        if count == 2 {
            let a = proof.records().first().ok_or("first relation")?;
            let b = proof.records().get(1).ok_or("second relation")?;
            assert!(std::ptr::eq(a.source(), b.source()));
            if case == 4 {
                assert!(std::ptr::eq(
                    a.input().length().ok_or("first actual Length")?,
                    b.input().length().ok_or("same actual Length")?
                ));
            }
        }
    }
    Ok(())
}
