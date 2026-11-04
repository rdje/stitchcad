//! Required actual Root semantic cohort,global ownership and complete original resource fields.
use super::{limits, root_with_host_and_semantic, EmptyHost, SourceError, TestResult};
use sc_core::{design::*, name::MachineToken, ontology::*, recipe::*, value::*};
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn canonical() -> TestResult<CanonicalInputs> {
    let logical = LogicalInputDeclaration::new(
        LogicalInputDeclarationDefinition {
            id: id(90),
            source: id(1004),
            kind: LogicalInputKind::PocketOpening,
            domain: vec![MachineToken::new("option_a")?],
            state: LogicalInputState::Derived { operation: id(80) },
        },
        LogicalInputLimits::new(1, 0, 8, 2),
    )?;
    Ok(CanonicalInputs::with_logicals(
        vec![LengthDeclaration::new(LengthDeclarationDefinition {
            id: id(10),
            source: id(1000),
            state: LengthState::Unknown {
                observation: id(1001),
            },
        })?],
        vec![ScalarInputDeclaration::new(
            ScalarInputDeclarationDefinition {
                id: id(20),
                source: id(1002),
                kind: ScalarInputKind::Boolean,
                state: ScalarInputState::Unknown {
                    observation: id(1003),
                },
            },
        )?],
        LogicalInputs::new(vec![logical], LogicalInputInventoryLimits::new(1, 1))?,
    )?)
}
fn recipe(button_id: u128) -> TestResult<SemanticRecipe> {
    let button = ButtonholeOperation::new(
        ButtonholeOperationDefinition {
            id: id(button_id),
            button: NotionSize::Declaration(id(1100)),
            policy: id(900),
            inputs: vec![ButtonholePolicyInput {
                role: MachineToken::new("policy_a")?,
                target: CanonicalInputTarget::Length(id(10)),
            }],
        },
        ButtonholeOperationLimits::new(1, 8, 1),
    )?;
    let logical = LogicalDerivationOperation::new(
        LogicalDerivationDefinition {
            id: id(80),
            output: LogicalInputTarget {
                declaration: id(90),
                kind: LogicalInputKind::PocketOpening,
            },
            rule: id(901),
            inputs: vec![LogicalDerivationInput {
                role: MachineToken::new("source_a")?,
                source: LogicalDerivationInputSource::Numeric(CanonicalInputTarget::Scalar {
                    declaration: id(20),
                    kind: ScalarInputKind::Boolean,
                }),
            }],
        },
        LogicalDerivationLimits::new(1, 8, 1),
    )?;
    Ok(SemanticRecipe::new(
        vec![
            SemanticOperation::Buttonhole(button),
            SemanticOperation::LogicalDerivation(logical),
        ],
        SemanticRecipeLimits::new(2, 4),
    )?)
}
fn root_with(
    recipe: SemanticRecipe,
    sources: Vec<FormulaSource>,
    parameters: Vec<DesignParameterDefinition>,
) -> TestResult<DesignInputs<EmptyHost>> {
    root_with_host_and_semantic(
        "",
        sources,
        EmptyHost,
        canonical()?,
        parameters,
        DesignSourceReferences {
            measurement_table: None,
            ease_set: None,
            size_set: None,
        },
        recipe,
    )
}
#[test]
fn original_semantic_owner_and_member_buffers_move_into_required_root_without_a_default(
) -> TestResult {
    let original = recipe(70)?;
    let expected = original.clone();
    let buffer = original.operations().as_ptr();
    let root = root_with(original, vec![], vec![])?;
    assert_eq!(root.semantic_recipe(), &expected);
    assert_eq!(root.semantic_recipe().operations().as_ptr(), buffer);
    assert!(std::ptr::eq(
        root.semantic_recipe(),
        &root.definition().semantic_recipe
    ));
    let member = root
        .semantic_recipe()
        .resolve(id(80), SemanticOperationKind::LogicalDerivation)
        .map_err(|e| e.to_string())?;
    assert!(std::ptr::eq(member.recipe(), root.semantic_recipe()));
    assert_eq!(member.index(), 1);
    assert_eq!(
        root.identities()
            .into_iter()
            .filter_map(|(id, p)| match p {
                DesignInputsPosition::SemanticOperation { index } => Some((id, index)),
                _ => None,
            })
            .collect::<Vec<_>>(),
        [(id(70), 0), (id(80), 1)]
    );
    let explicit = super::root()?;
    assert!(explicit.semantic_recipe().operations().is_empty());
    assert_eq!(
        explicit.semantic_recipe().limits(),
        SemanticRecipeLimits::new(0, 0)
    );
    Ok(())
}
#[test]
fn all_family_ownership_collisions_keep_original_root_positions_before_source_binding() -> TestResult
{
    for (recipe, sources, parameters, identity, first, duplicate) in [
        (
            recipe(10)?,
            vec![],
            vec![],
            id(10),
            DesignInputsPosition::Structure(DesignStructurePosition::Length { index: 0 }),
            DesignInputsPosition::SemanticOperation { index: 0 },
        ),
        (
            recipe(90)?,
            vec![],
            vec![],
            id(90),
            DesignInputsPosition::Structure(DesignStructurePosition::Logical { index: 0 }),
            DesignInputsPosition::SemanticOperation { index: 0 },
        ),
        (
            recipe(70)?,
            vec![FormulaSource::new(
                id(70),
                FormulaExpressionDocument::new("1 mm".into())?,
            )],
            vec![],
            id(70),
            DesignInputsPosition::FormulaSource { index: 0 },
            DesignInputsPosition::SemanticOperation { index: 0 },
        ),
        (
            recipe(70)?,
            vec![],
            vec![DesignParameterDefinition {
                id: id(70),
                token: MachineToken::new("limit_a")?,
                target: CanonicalInputTarget::Length(id(10)),
            }],
            id(70),
            DesignInputsPosition::Parameter { index: 0 },
            DesignInputsPosition::SemanticOperation { index: 0 },
        ),
    ] {
        let error = root_with(recipe, sources, parameters)
            .err()
            .ok_or("whole Root identity refusal")?;
        let original = error
            .downcast_ref::<DesignInputsError<SourceError>>()
            .ok_or("original typed Root cause")?;
        assert!(
            matches!(original,DesignInputsError::DuplicateIdentity {identity:i,first:a,duplicate:b}
            if *i==identity && *a==first && *b==duplicate)
        );
    }
    Ok(())
}
#[test]
fn whole_actual_root_resource_walk_charges_original_semantic_fields_before_queries() -> TestResult {
    let root = root_with(recipe(70)?, vec![], vec![])?;
    let mut meter = DesignResourceMeter::new(limits(8, 32, 3)?);
    charge_design_input_resources(&root, &mut meter)?;
    let demand = meter.finish()?;
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (8, 32, 3)
    );
    for (entries, text, unit, maximum, consumed, requested, field, owner, index) in [
        (
            6,
            32,
            DesignResourceCount::CollectionEntries,
            6,
            6,
            1,
            "semantic.buttonhole.inputs",
            70,
            None,
        ),
        (
            8,
            23,
            DesignResourceCount::TextBytes,
            23,
            16,
            8,
            "semantic.buttonhole.inputs.role",
            70,
            Some(0),
        ),
        (
            8,
            31,
            DesignResourceCount::TextBytes,
            31,
            24,
            8,
            "semantic.logical.inputs.role",
            80,
            Some(0),
        ),
    ] {
        let selected = limits(entries, text, 3)?;
        let mut meter = DesignResourceMeter::new(selected);
        let expected = DesignResourceLimitError::Count {
            unit,
            maximum,
            consumed,
            requested,
            location: DesignResourceLocation {
                scope: DesignResourceScope::Inputs,
                owner: Some(id(owner)),
                field,
                index,
            },
        };
        assert_eq!(
            charge_design_input_resources(&root, &mut meter),
            Err(expected)
        );
        assert_eq!(meter.finish(), Err(expected));
        let error = root
            .prepare_formula_scope(selected)
            .err()
            .ok_or("Root semantic content refusal")?;
        assert!(
            matches!(error.refusal(),DesignFormulaScopeRefusal::Resources(actual) if *actual==expected)
        );
    }
    assert!(
        matches!(root.inputs().logicals().declaration(id(90)).map(|r|&r.definition().state),
        Some(LogicalInputState::Derived {operation}) if *operation==id(80))
    );
    Ok(())
}
