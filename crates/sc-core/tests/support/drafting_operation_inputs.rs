//! Original mixed/local input order with interleaved Root/Profile fields and a Profile button.
use super::{id, limits, scope, ParameterSourceView, Provider, TestResult};
use sc_core::{construction::*, design::*, ontology::*, recipe::*, value::*};
#[test]
fn complete_operation_inputs_keep_original_interleaving_sources_and_unchanged_cursor() -> TestResult
{
    let case = 8;
    let garment = super::super::root_numeric_suppliers::interleaved_fixture()?;
    let other = super::super::root_numeric_suppliers::interleaved_fixture()?;
    let source = Provider::new(&garment, other.inputs(), 0)?;
    let source_view: &ParameterSourceView<'_> = &source;
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
    let mut steps = vec![
        DraftingProgramStep::Semantic { ordinal: 1 },
        DraftingProgramStep::Drafting(DraftingOperationInput::PlaceGrainline(&grainline)),
    ];

    if case == 8 {
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
    let selected = SelectedProfilePin::new(SelectedProfilePinDefinition {
        profile_id: "original".into(),
        schema_version: "schema".into(),
        content_version: "content".into(),
    })?;

    let mut rows = vec![DraftingSemanticSelectionRow::Rule {
        ordinal: 1,
        origin: DraftingRuleSelection::Design,
    }];
    for index in [1, 4] {
        rows.push(DraftingSemanticSelectionRow::Profile {
            location: DraftingSemanticSelectionLocation::Semantic {
                ordinal: 1,
                field: DraftingSemanticSelectionField::LogicalInput { index },
            },
            selected: &selected,
        });
    }
    if case == 8 {
        rows.push(DraftingSemanticSelectionRow::Rule {
            ordinal: 2,
            origin: DraftingRuleSelection::Design,
        });
        rows.push(DraftingSemanticSelectionRow::Profile {
            location: DraftingSemanticSelectionLocation::Semantic {
                ordinal: 2,
                field: DraftingSemanticSelectionField::ButtonholeButton,
            },
            selected: &selected,
        });
    }
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

    let occurrences = 13;
    let row_count = 5;
    let passes = 19;
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
    let owner = checked
        .root_input_scope(&root_scope)
        .map_err(|e| e.to_string())?;

    let root = owner
        .bind_root_input_members()
        .map_err(|e| e.to_string())?
        .bind_root_logical_suppliers()
        .map_err(|e| e.to_string())?
        .bind_root_numeric_suppliers()
        .map_err(|e| e.to_string())?;
    assert_eq!(root.logical().members().records().len(), 6);
    assert_eq!(root.records().len(), 5);
    let mut headers = root
        .bind_profile_input_headers()
        .map_err(|e| e.to_string())?;
    let proof = headers
        .bind_profile_input_parameters()
        .map_err(|e| e.to_string())?;
    assert_eq!(proof.records().len(), 3);
    assert_eq!(proof.operation_input_count(), 9);
    assert_eq!(proof.operation_inputs().count(), 9);
    let before = (
        source.calls.get(),
        source.parameter_calls.get(),
        source.base.walks.get(),
        proof
            .root()
            .logical()
            .members()
            .owner()
            .selections()
            .reserved_child_passes(),
    );
    assert_eq!(before.3, 19);
    let expected = [
        (
            1,
            1,
            DraftingSemanticSelectionField::LogicalInput { index: 0 },
            false,
        ),
        (
            1,
            1,
            DraftingSemanticSelectionField::LogicalInput { index: 1 },
            true,
        ),
        (
            1,
            1,
            DraftingSemanticSelectionField::LogicalInput { index: 2 },
            false,
        ),
        (
            1,
            1,
            DraftingSemanticSelectionField::LogicalInput { index: 3 },
            false,
        ),
        (
            1,
            1,
            DraftingSemanticSelectionField::LogicalInput { index: 4 },
            true,
        ),
        (
            1,
            1,
            DraftingSemanticSelectionField::LogicalInput { index: 5 },
            false,
        ),
        (
            1,
            1,
            DraftingSemanticSelectionField::LogicalInput { index: 6 },
            false,
        ),
        (2, 3, DraftingSemanticSelectionField::ButtonholeButton, true),
        (
            2,
            3,
            DraftingSemanticSelectionField::ButtonholeInput { index: 0 },
            false,
        ),
    ];
    for (input, (ordinal, position, field, is_profile)) in proof.operation_inputs().zip(expected) {
        assert_eq!(input.ordinal(), ordinal);
        assert_eq!(input.position(), position);
        assert_eq!(
            input.location(),
            DraftingSemanticSelectionLocation::Semantic { ordinal, field }
        );
        assert!(core::ptr::eq(
            input.operation(),
            garment
                .inputs()
                .semantic_recipe()
                .operations()
                .get(ordinal - 1)
                .ok_or("actual original consumer")?
        ));
        assert!(same_field(
            input.field(),
            input_field(ordinal, field, &program)?
        ));
        match input {
            DraftingProgramOperationInput::Root(binding) if !is_profile => {
                assert!(same_field(
                    binding.member().field(),
                    input_field(ordinal, field, &program)?
                ));
                let original = proof
                    .root()
                    .logical()
                    .members()
                    .records()
                    .get(binding.member_index())
                    .ok_or("actual original Root record")?;
                assert!(core::ptr::eq(
                    original.operation(),
                    binding.member().operation()
                ));
                match binding.member().source() {
                    DraftingProgramRootInputSource::Numeric(member) => {
                        assert!(core::ptr::eq(member.scope(), &root_scope));
                        let Some(DraftingProgramRootInputSupplier::Numeric(supplier)) =
                            binding.supplier()
                        else {
                            return Err("actual Root numeric supplier".into());
                        };
                        assert_eq!(supplier.input().target(), member.binding().target());
                        assert!(core::ptr::eq(supplier.graph(), &graph));
                    }
                    DraftingProgramRootInputSource::Logical(member) => {
                        assert_eq!(
                            member.declaration().definition().state.tag(),
                            LogicalInputStateTag::Unknown
                        );
                        assert!(binding.supplier().is_none());
                    }
                }
            }
            DraftingProgramOperationInput::Profile(parameter) if is_profile => {
                let original = proof
                    .records()
                    .get(parameter.header_index())
                    .ok_or("actual original Profile record")?;
                assert!(core::ptr::eq(parameter, original));
                assert!(core::ptr::eq(
                    parameter.source().parent(),
                    parameter.header().source()
                ));
                assert!(core::ptr::eq(
                    parameter.source().binding().inputs(),
                    source.inputs.logicals()
                ));
                assert!(same_field(
                    parameter.header().request().field(),
                    input_field(ordinal, field, &program)?
                ));
            }
            _ => return Err("wrong original input source class".into()),
        }
    }
    assert!(matches!(
        proof.operation_inputs().nth(1),
        Some(DraftingProgramOperationInput::Profile(_))
    ));
    let button = proof
        .operation_inputs()
        .nth(7)
        .ok_or("original Profile button")?;
    assert_eq!(
        button.location(),
        DraftingSemanticSelectionLocation::Semantic {
            ordinal: 2,
            field: DraftingSemanticSelectionField::ButtonholeButton
        }
    );
    assert!(matches!(button, DraftingProgramOperationInput::Profile(_)));
    let mut tail = proof.operation_inputs().skip(8);
    let numeric = tail.next().ok_or("original numeric policy input")?;
    assert!(matches!(numeric, DraftingProgramOperationInput::Root(_)));
    let mut fused = proof.operation_inputs();
    for _ in 0..9 {
        assert!(fused.next().is_some());
    }
    assert!(fused.next().is_none());
    assert!(fused.next().is_none());
    assert_eq!(
        (
            source.calls.get(),
            source.parameter_calls.get(),
            source.base.walks.get(),
            proof
                .root()
                .logical()
                .members()
                .owner()
                .selections()
                .reserved_child_passes()
        ),
        before
    );
    Ok(())
}
fn input_field<'a, H: DesignMetadataHost, S: DraftingContextSource<H> + ?Sized>(
    ordinal: usize,
    field: DraftingSemanticSelectionField,
    program: &NormalizedDraftingProgram<'_, 'a, H, S>,
) -> TestResult<DraftingSemanticField<'a>> {
    program
        .semantic_dependencies()
        .find(|use_| {
            use_.selection_location()
                == DraftingSemanticSelectionLocation::Semantic { ordinal, field }
        })
        .map(|use_| use_.field())
        .ok_or_else(|| "actual original field".into())
}

fn same_field(left: DraftingSemanticField<'_>, right: DraftingSemanticField<'_>) -> bool {
    match (left, right) {
        (
            DraftingSemanticField::ButtonholeButton(a),
            DraftingSemanticField::ButtonholeButton(b),
        ) => core::ptr::eq(a, b),
        (
            DraftingSemanticField::ButtonholeInput { index: a, input: x },
            DraftingSemanticField::ButtonholeInput { index: b, input: y },
        ) => a == b && core::ptr::eq(x, y),
        (
            DraftingSemanticField::LogicalInput { index: a, input: x },
            DraftingSemanticField::LogicalInput { index: b, input: y },
        ) => a == b && core::ptr::eq(x, y),
        _ => false,
    }
}
