//! Actual logical canonical membership, all-family ownership and complete Root content admission.
use super::{limits, root_with_host, EmptyHost, TestResult};
use sc_core::{design::*, name::MachineToken, ontology::*, recipe::*, value::*};
use sc_units::Length;
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn logicals() -> TestResult<LogicalInputs> {
    let records = [
        (
            80,
            LogicalInputState::Unknown {
                observation: id(900),
            },
        ),
        (70, LogicalInputState::Derived { operation: id(901) }),
        (
            90,
            LogicalInputState::Known {
                value: MachineToken::new("small")?,
                evidence: vec![id(903), id(902)],
            },
        ),
    ]
    .into_iter()
    .map(|(n, state)| -> TestResult<LogicalInputDeclaration> {
        Ok(LogicalInputDeclaration::new(
            LogicalInputDeclarationDefinition {
                id: id(n),
                source: id(999),
                kind: LogicalInputKind::PocketOpening,
                domain: vec![MachineToken::new("small")?, MachineToken::new("large")?],
                state,
            },
            LogicalInputLimits::new(2, 2, 15, 10),
        )?)
    })
    .collect::<TestResult<Vec<_>>>()?;
    Ok(LogicalInputs::new(
        records,
        LogicalInputInventoryLimits::new(3, 9),
    )?)
}
fn length(n: u128) -> TestResult<LengthDeclaration> {
    Ok(LengthDeclaration::new(LengthDeclarationDefinition {
        id: id(n),
        source: id(998),
        state: LengthState::Known {
            value: Length::ZERO,
            evidence: vec![id(904), id(905)],
        },
    })?)
}
fn canonical() -> TestResult<CanonicalInputs> {
    Ok(CanonicalInputs::with_logicals(
        vec![length(10)?],
        vec![ScalarInputDeclaration::new(
            ScalarInputDeclarationDefinition {
                id: id(20),
                source: id(997),
                kind: ScalarInputKind::Boolean,
                state: ScalarInputState::Known {
                    value: ScalarInputValue::from_boolean(true),
                    evidence: vec![id(906)],
                },
            },
        )?],
        logicals()?,
    )?)
}
fn root_with(
    canonical: CanonicalInputs,
    sources: Vec<FormulaSource>,
) -> TestResult<DesignInputs<EmptyHost>> {
    root_with_host(
        "",
        sources,
        EmptyHost,
        canonical,
        vec![],
        DesignSourceReferences {
            measurement_table: None,
            ease_set: None,
            size_set: None,
        },
    )
}
#[test]
fn actual_original_logical_storage_survives_attachment_and_numeric_replacement() -> TestResult {
    let original = logicals()?;
    let buffer = original.declarations().as_ptr();
    let inputs = CanonicalInputs::with_logicals(vec![length(10)?], vec![], original)?;
    assert_eq!(inputs.logicals().declarations().as_ptr(), buffer);
    let binding = inputs
        .logicals()
        .resolve(id(70), LogicalInputKind::PocketOpening)
        .map_err(|e| e.to_string())?;
    assert!(std::ptr::eq(binding.inputs(), inputs.logicals()));
    assert!(std::ptr::eq(
        binding.declaration(),
        inputs
            .logicals()
            .declarations()
            .get(1)
            .ok_or("original second declaration")?
    ));
    assert_eq!(
        binding.declaration().definition().state,
        LogicalInputState::Derived { operation: id(901) }
    );
    let replacement = LengthDeclaration::new(LengthDeclarationDefinition {
        id: id(10),
        source: id(998),
        state: LengthState::Unknown {
            observation: id(907),
        },
    })?;
    let changed = inputs.with_length_declaration(replacement)?;
    assert_eq!(changed.logicals(), inputs.logicals());
    assert_eq!(
        inputs.length(id(10)).map(LengthDeclaration::state),
        Some(&LengthState::Known {
            value: Length::ZERO,
            evidence: vec![id(904), id(905)],
        })
    );
    assert!(inputs.length(id(70)).is_none() && inputs.scalar(id(70)).is_none());
    assert!(inputs.contains(id(70)) && !inputs.contains(id(901)));
    let numeric_only = CanonicalInputs::new(vec![], vec![])?;
    assert!(numeric_only.logicals().declarations().is_empty());
    assert_eq!(
        numeric_only.logicals().limits(),
        LogicalInputInventoryLimits::new(0, 0)
    );
    Ok(())
}
#[test]
fn cross_family_and_containing_structure_collisions_keep_actual_first_positions() -> TestResult {
    assert_eq!(
        CanonicalInputs::with_logicals(vec![length(70)?], vec![], logicals()?),
        Err(CanonicalInputsError::DuplicateIdentity {
            identity: id(70),
            first: CanonicalInputPosition::Length { index: 0 },
            duplicate: CanonicalInputPosition::Logical { index: 1 },
        })
    );
    let root = root_with(canonical()?, vec![])?;
    let mut definition = root.structure().definition().clone();
    definition.id = id(70);
    assert_eq!(
        DesignStructure::new(definition),
        Err(DesignStructureError::DuplicateIdentity {
            identity: id(70),
            first: DesignStructurePosition::Design,
            duplicate: DesignStructurePosition::Logical { index: 1 },
        })
    );
    let census = root.identities();
    assert_eq!(
        census
            .iter()
            .filter_map(|(id, p)| matches!(
                p,
                DesignInputsPosition::Structure(DesignStructurePosition::Logical { .. })
            )
            .then_some(*id))
            .collect::<Vec<_>>(),
        [id(80), id(70), id(90)]
    );
    Ok(())
}
#[test]
fn logical_operation_references_do_not_enter_numeric_source_graph_but_owned_ids_are_disjoint(
) -> TestResult {
    let root = root_with(canonical()?, vec![])?;
    let joined = FormulaSourceInputs::new(root.inputs(), root.formula_sources())
        .map_err(|e| e.to_string())?;
    assert_eq!(joined.derived_inputs().count(), 0);
    let source = FormulaSource::new(id(70), FormulaExpressionDocument::new("1 mm".into())?);
    let error = root_with(canonical()?, vec![source])
        .err()
        .ok_or("logical/source identity collision")?;
    let typed = error
        .downcast_ref::<DesignInputsError<super::SourceError>>()
        .ok_or("original typed error")?;
    assert!(matches!(typed,DesignInputsError::DuplicateIdentity {
        identity,first:DesignInputsPosition::Structure(DesignStructurePosition::Logical {index:1}),
        duplicate:DesignInputsPosition::FormulaSource {index:0},
    } if *identity==id(70)));
    Ok(())
}
#[test]
fn numeric_parameter_guards_include_logical_ownership_without_accepting_a_logical_operand(
) -> TestResult {
    let inputs = canonical()?;
    let parameter = |identity, target| -> TestResult<DesignParameterDefinition> {
        Ok(DesignParameterDefinition {
            id: id(identity),
            token: MachineToken::new("opening")?,
            target,
        })
    };
    assert_eq!(
        DesignParameters::new(
            vec![parameter(70, CanonicalInputTarget::Length(id(10)))?],
            &inputs
        ),
        Err(DesignParameterError::CanonicalIdentityCollision {
            identity: id(70),
            index: 0
        })
    );
    assert_eq!(
        DesignParameters::new(
            vec![parameter(
                40,
                CanonicalInputTarget::Scalar {
                    declaration: id(70),
                    kind: ScalarInputKind::Boolean,
                }
            )?],
            &inputs
        ),
        Err(DesignParameterError::MissingScalar {
            parameter: id(40),
            declaration: id(70)
        })
    );
    Ok(())
}
#[test]
fn actual_root_content_and_first_logical_refusal_use_the_complete_cumulative_walk() -> TestResult {
    let root = root_with(canonical()?, vec![])?;
    let mut meter = DesignResourceMeter::new(limits(16, 43, 3)?);
    charge_design_input_resources(&root, &mut meter)?;
    let demand = meter.finish()?;
    // Numeric entries5 + logical inventory3/domain6/evidence2;domain30/selection5 + recipe8 bytes.
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (16, 43, 3)
    );
    let mut meter = DesignResourceMeter::new(limits(16, 34, 3)?);
    let expected = DesignResourceLimitError::Count {
        unit: DesignResourceCount::TextBytes,
        maximum: 34,
        consumed: 30,
        requested: 5,
        location: DesignResourceLocation {
            scope: DesignResourceScope::Structure,
            owner: Some(id(90)),
            field: "canonical.logicals.value",
            index: Some(2),
        },
    };
    assert_eq!(
        charge_design_input_resources(&root, &mut meter),
        Err(expected)
    );
    assert_eq!(meter.finish(), Err(expected));
    let error = root
        .prepare_formula_scope(limits(16, 34, 3)?)
        .err()
        .ok_or("resource first")?;
    assert!(
        matches!(error.refusal(),DesignFormulaScopeRefusal::Resources(actual) if *actual==expected)
    );
    Ok(())
}
