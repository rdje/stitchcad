//! Genuine normalized owner/position/ordinal and every original typed semantic field.
use super::{
    limits, root, root_with_host_and_semantic, scope, semantic_root, EmptyHost, Source, TestResult,
};
use sc_core::{construction::*, design::*, name::MachineToken, ontology::*, recipe::*, value::*};
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn recipe() -> TestResult<SemanticRecipe> {
    let button = ButtonholeOperation::new(
        ButtonholeOperationDefinition {
            id: id(70),
            button: NotionSize::Profile(ProfileParameterRef::new(id(700))),
            policy: id(900),
            inputs: ["policy_a", "policy_b"]
                .into_iter()
                .map(|role| {
                    Ok(ButtonholePolicyInput {
                        role: MachineToken::new(role)?,
                        target: CanonicalInputTarget::Length(id(10)),
                    })
                })
                .collect::<TestResult<Vec<_>>>()?,
        },
        ButtonholeOperationLimits::new(2, 16, 4),
    )?;
    let sources = [
        LogicalDerivationInputSource::Logical(LogicalInputTarget {
            declaration: id(90),
            kind: LogicalInputKind::PocketOpening,
        }),
        LogicalDerivationInputSource::Profile {
            parameter: ProfileParameterRef::new(id(701)),
            kind: LogicalInputKind::NotionSize,
        },
        LogicalDerivationInputSource::Numeric(CanonicalInputTarget::Scalar {
            declaration: id(20),
            kind: ScalarInputKind::Boolean,
        }),
    ];
    let logical = LogicalDerivationOperation::new(
        LogicalDerivationDefinition {
            id: id(80),
            output: LogicalInputTarget {
                declaration: id(90),
                kind: LogicalInputKind::PocketOpening,
            },
            rule: id(901),
            inputs: ["source_a", "source_b", "source_c"]
                .into_iter()
                .zip(sources)
                .map(|(role, source)| {
                    Ok(LogicalDerivationInput {
                        role: MachineToken::new(role)?,
                        source,
                    })
                })
                .collect::<TestResult<Vec<_>>>()?,
        },
        LogicalDerivationLimits::new(3, 24, 9),
    )?;
    Ok(SemanticRecipe::new(
        vec![
            SemanticOperation::Buttonhole(button),
            SemanticOperation::LogicalDerivation(logical),
        ],
        SemanticRecipeLimits::new(2, 4),
    )?)
}
fn policy() -> TestResult<DraftingProgramLimits> {
    Ok(DraftingProgramLimits::new(
        limits(10000, 10000, 10000)?,
        100,
        100,
        1000,
    ))
}
#[test]
fn original_semantic_fields_pin_exact_program_owners_positions_ordinals_and_named_buffers(
) -> TestResult {
    let root = root_with_host_and_semantic(
        "let private_width:length=1 mm",
        vec![],
        EmptyHost,
        semantic_root::canonical()?,
        vec![],
        DesignSourceReferences {
            measurement_table: None,
            ease_set: None,
            size_set: None,
        },
        recipe()?,
    )?;
    let source = Source::new(&root);
    let context = scope(&root)?
        .bind_drafting_context(&source, limits(10000, 10000, 10000)?)
        .map_err(|e| e.to_string())?;
    let point = PointConstruction::new(
        id(100),
        FormulaExpressionDocument::new("1 mm".into())?,
        FormulaExpressionDocument::new("2 mm".into())?,
    );
    let steps = vec![
        DraftingProgramStep::Semantic { ordinal: 1 },
        DraftingProgramStep::Scalar { ordinal: 1 },
        DraftingProgramStep::Drafting(DraftingOperationInput::ConstructPoint(&point)),
        DraftingProgramStep::Semantic { ordinal: 2 },
    ];
    let raw = DraftingProgramInputs::new(context, steps.clone());
    let other = DraftingProgramInputs::new(context, steps);
    let policy = policy()?;
    let admitted = raw.admit(&policy).map_err(|e| e.to_string())?;
    let normalized = admitted.normalized().map_err(|e| e.to_string())?;
    let other_admitted = other.admit(&policy).map_err(|e| e.to_string())?;
    let other_program = other_admitted.normalized().map_err(|e| e.to_string())?;
    source.trace.set(0);
    let mut fields = normalized.semantic_dependencies();
    for (ordinal, position, count) in [(1, 1, 4), (2, 4, 5)] {
        let original = root
            .semantic_recipe()
            .operations()
            .get(ordinal - 1)
            .ok_or("actual Root operation")?;
        for index in 0..count {
            let use_ = fields.next().ok_or("complete original semantic fields")?;
            let copied = use_;
            assert!(core::ptr::eq(copied.program(), &normalized));
            assert!(!core::ptr::eq(copied.program(), &other_program));
            assert!(core::ptr::eq(use_.operation(), original));
            assert_eq!(use_.position(), position);
            assert_eq!(use_.step().position(), position);
            assert_eq!(use_.ordinal(), ordinal);
            assert!(
                matches!(use_.step().kind(),DraftingProgramStepKind::Semantic{ordinal:actual,operation} if actual==ordinal && core::ptr::eq(operation,original))
            );
            let exact = match (original, index, use_.field()) {
                (
                    SemanticOperation::Buttonhole(op),
                    0,
                    DraftingSemanticField::ButtonholeButton(actual),
                ) => core::ptr::eq(actual, &op.definition().button),
                (
                    SemanticOperation::Buttonhole(op),
                    1,
                    DraftingSemanticField::ButtonholePolicy(actual),
                ) => core::ptr::eq(actual, &op.definition().policy),
                (
                    SemanticOperation::Buttonhole(op),
                    i,
                    DraftingSemanticField::ButtonholeInput { index, input },
                ) => {
                    index == i - 2
                        && core::ptr::eq(
                            input,
                            op.definition()
                                .inputs
                                .get(index)
                                .ok_or("actual policy input")?,
                        )
                        && core::ptr::eq(
                            &input.role,
                            &op.definition().inputs.get(index).ok_or("actual role")?.role,
                        )
                }
                (
                    SemanticOperation::LogicalDerivation(op),
                    0,
                    DraftingSemanticField::LogicalOutput(actual),
                ) => core::ptr::eq(actual, &op.definition().output),
                (
                    SemanticOperation::LogicalDerivation(op),
                    1,
                    DraftingSemanticField::LogicalRule(actual),
                ) => core::ptr::eq(actual, &op.definition().rule),
                (
                    SemanticOperation::LogicalDerivation(op),
                    i,
                    DraftingSemanticField::LogicalInput { index, input },
                ) => {
                    index == i - 2
                        && core::ptr::eq(
                            input,
                            op.definition()
                                .inputs
                                .get(index)
                                .ok_or("actual logical input")?,
                        )
                        && core::ptr::eq(
                            &input.source,
                            &op.definition()
                                .inputs
                                .get(index)
                                .ok_or("actual typed source")?
                                .source,
                        )
                }
                _ => false,
            };
            assert!(exact);
            assert!(!format!("{use_:?}").contains("policy_a"));
        }
    }
    assert!(fields.next().is_none());
    assert!(fields.next().is_none());
    assert_eq!(other_program.semantic_dependencies().count(), 9);
    assert_eq!(source.trace.get(), 0);
    Ok(())
}
#[test]
fn actual_empty_program_has_no_semantic_fields_and_makes_no_provider_calls() -> TestResult {
    let root = root()?;
    let source = Source::new(&root);
    let context = scope(&root)?
        .bind_drafting_context(&source, limits(10000, 10000, 10000)?)
        .map_err(|e| e.to_string())?;
    let raw = DraftingProgramInputs::new(context, vec![]);
    let policy = policy()?;
    let admitted = raw.admit(&policy).map_err(|e| e.to_string())?;
    let program = admitted.normalized().map_err(|e| e.to_string())?;
    source.trace.set(0);
    let mut fields = program.semantic_dependencies();
    assert!(fields.next().is_none());
    assert!(fields.next().is_none());
    assert_eq!(source.trace.get(), 0);
    Ok(())
}
