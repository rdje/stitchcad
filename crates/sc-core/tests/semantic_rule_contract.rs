//! Original typed signature families, whole constructor bounds and cumulative source accounting.
use core::num::{NonZeroU64, NonZeroUsize};
use sc_core::{design::*, name::MachineToken, ontology::EntityId, recipe::*, value::*};
use std::error::Error;
type TestResult<T = ()> = Result<T, Box<dyn Error>>;
fn id() -> EntityId {
    EntityId::from_bits(900)
}
fn definitions() -> TestResult<Vec<SemanticRuleDefinition>> {
    let kinds = core::iter::once(CanonicalNumericInputKind::Length).chain(
        ScalarInputKind::ALL
            .into_iter()
            .map(CanonicalNumericInputKind::Scalar),
    );
    let numeric = kinds
        .enumerate()
        .map(|(index, kind)| -> TestResult<_> {
            Ok(ButtonholePolicyRequirement {
                role: MachineToken::new(format!("source_{index}"))?,
                kind,
            })
        })
        .collect::<TestResult<Vec<_>>>()?;
    let mut definitions = vec![SemanticRuleDefinition {
        id: id(),
        signature: SemanticRuleSignature::Buttonhole {
            inputs: numeric.clone(),
        },
    }];
    for result in [
        LogicalInputKind::PocketOpening,
        LogicalInputKind::NotionSize,
    ] {
        let mut inputs = vec![
            SemanticRuleInput {
                role: MachineToken::new("source_a")?,
                kind: SemanticRuleInputKind::Logical(LogicalInputKind::PocketOpening),
            },
            SemanticRuleInput {
                role: MachineToken::new("source_b")?,
                kind: SemanticRuleInputKind::Logical(LogicalInputKind::NotionSize),
            },
        ];
        inputs.extend(numeric.iter().map(|input| SemanticRuleInput {
            role: input.role.clone(),
            kind: SemanticRuleInputKind::Numeric(input.kind),
        }));
        definitions.push(SemanticRuleDefinition {
            id: id(),
            signature: SemanticRuleSignature::Logical { result, inputs },
        });
    }
    Ok(definitions)
}
fn roles(definition: &SemanticRuleDefinition) -> Vec<&MachineToken> {
    match &definition.signature {
        SemanticRuleSignature::Logical { inputs, .. } => {
            inputs.iter().map(|input| &input.role).collect()
        }
        SemanticRuleSignature::Buttonhole { inputs } => {
            inputs.iter().map(|input| &input.role).collect()
        }
    }
}
fn buffer(definition: &SemanticRuleDefinition) -> *const () {
    match &definition.signature {
        SemanticRuleSignature::Logical { inputs, .. } => inputs.as_ptr().cast(),
        SemanticRuleSignature::Buttonhole { inputs } => inputs.as_ptr().cast(),
    }
}
fn clear(definition: &mut SemanticRuleDefinition) {
    match &mut definition.signature {
        SemanticRuleSignature::Logical { inputs, .. } => inputs.clear(),
        SemanticRuleSignature::Buttonhole { inputs } => inputs.clear(),
    }
}
fn repeat(definition: &mut SemanticRuleDefinition) -> TestResult {
    let role = roles(definition)
        .first()
        .copied()
        .ok_or("first role")?
        .clone();
    match &mut definition.signature {
        SemanticRuleSignature::Logical { inputs, .. } => {
            inputs.last_mut().ok_or("last role")?.role = role
        }
        SemanticRuleSignature::Buttonhole { inputs } => {
            inputs.last_mut().ok_or("last role")?.role = role
        }
    }
    Ok(())
}
fn selected(entries: usize, text: usize) -> TestResult<DesignResourceLimits> {
    Ok(DesignResourceLimits::new(
        NonZeroUsize::new(entries).ok_or("positive entries")?,
        NonZeroUsize::new(text).ok_or("positive text")?,
        NonZeroU64::MAX,
    ))
}
fn at(
    scope: DesignResourceScope,
    field: &'static str,
    index: Option<usize>,
) -> DesignResourceLocation {
    DesignResourceLocation {
        scope,
        owner: Some(id()),
        field,
        index,
    }
}
#[test]
fn original_families_all_kinds_and_buffers_survive_without_coercion_or_result_invention(
) -> TestResult {
    for definition in definitions()? {
        let expected = definition.clone();
        let count = roles(&definition).len();
        let original_buffer = buffer(&definition);
        let original_text = roles(&definition)
            .first()
            .ok_or("first role")?
            .as_str()
            .as_ptr();
        let limits = SemanticRuleLimits::new(count, 8 * count, count * count);
        let rule = SemanticRuleDeclaration::new(definition, limits)?;
        assert_eq!(rule.definition(), &expected);
        assert_eq!(buffer(rule.definition()), original_buffer);
        assert_eq!(
            roles(rule.definition())
                .first()
                .ok_or("original role")?
                .as_str()
                .as_ptr(),
            original_text
        );
        assert_eq!(rule.id(), id());
        assert_eq!(rule.limits(), limits);
        assert_eq!(
            (
                rule.shape().inputs(),
                rule.shape().role_text_bytes(),
                rule.shape().validation_inspections()
            ),
            (count, 8 * count, count * count)
        );
        let wider = SemanticRuleDeclaration::new(
            expected.clone(),
            SemanticRuleLimits::new(100, 1000, 10000),
        )?;
        assert_eq!(rule, wider);
        let expected_kind = match expected.signature {
            SemanticRuleSignature::Logical { .. } => SemanticOperationKind::LogicalDerivation,
            SemanticRuleSignature::Buttonhole { .. } => SemanticOperationKind::Buttonhole,
        };
        assert_eq!(rule.kind(), expected_kind);
        assert!(!format!("{rule:?} {expected:?}").contains("source_"));
        let mut detached = expected;
        detached.id = EntityId::from_bits(901);
        clear(&mut detached);
        assert_eq!(rule.id(), id());
        assert_eq!(roles(rule.definition()).len(), count);
    }
    assert_ne!(
        CanonicalNumericInputKind::Length,
        CanonicalNumericInputKind::Scalar(ScalarInputKind::Length)
    );
    Ok(())
}
#[test]
fn complete_original_quotas_precede_first_duplicate_and_empty_signatures_are_explicit() -> TestResult
{
    for mut definition in definitions()? {
        let count = roles(&definition).len();
        repeat(&mut definition)?;
        use SemanticRuleRefusal as R;
        for (limits, refusal) in [
            (
                SemanticRuleLimits::new(count - 1, 0, 0),
                R::Limit {
                    resource: SemanticRuleResource::Inputs,
                    allowed: count - 1,
                    required: count,
                },
            ),
            (
                SemanticRuleLimits::new(count, 8 * count - 1, 0),
                R::Limit {
                    resource: SemanticRuleResource::RoleTextBytes,
                    allowed: 8 * count - 1,
                    required: 8 * count,
                },
            ),
            (
                SemanticRuleLimits::new(count, 8 * count, count * count - 1),
                R::Limit {
                    resource: SemanticRuleResource::ValidationInspections,
                    allowed: count * count - 1,
                    required: count * count,
                },
            ),
            (
                SemanticRuleLimits::new(count, 8 * count, count * count),
                R::DuplicateRole {
                    first: 0,
                    duplicate: count - 1,
                },
            ),
        ] {
            assert_eq!(
                SemanticRuleDeclaration::new(definition.clone(), limits),
                Err(SemanticRuleDeclarationError {
                    rule: id(),
                    limits,
                    refusal
                })
            );
        }
        clear(&mut definition);
        let empty = SemanticRuleDeclaration::new(definition, SemanticRuleLimits::new(0, 0, 0))?;
        assert_eq!(
            (
                empty.shape().inputs(),
                empty.shape().role_text_bytes(),
                empty.shape().validation_inspections()
            ),
            (0, 0, 0)
        );
    }
    Ok(())
}
#[test]
fn actual_source_fields_precharge_vectors_and_retain_scope_prior_demand_and_first_refusal(
) -> TestResult {
    for definition in definitions()? {
        let count = roles(&definition).len();
        let rule =
            SemanticRuleDeclaration::new(definition, SemanticRuleLimits::new(100, 1000, 10000))?;
        for scope in [DesignResourceScope::Inputs, DesignResourceScope::SourceHost] {
            let prior = at(scope, "prior", Some(7));
            let mut meter = DesignResourceMeter::new(selected(count + 2, 8 * count + 3)?);
            meter.charge_entries(2, prior)?;
            meter.charge_text("old", prior)?;
            meter.charge_counter_bits(1, prior)?;
            charge_semantic_rule_resources(&rule, &mut meter, scope)?;
            let demand = meter.finish()?;
            assert_eq!(
                (
                    demand.collection_entries(),
                    demand.text_bytes(),
                    demand.counter_bits()
                ),
                (count + 2, 8 * count + 3, 1)
            );
            for (entries, text, unit, maximum, consumed, requested, location) in [
                (
                    count - 1,
                    8 * count,
                    DesignResourceCount::CollectionEntries,
                    count - 1,
                    0,
                    count,
                    at(scope, "semantic.rule.inputs", None),
                ),
                (
                    count,
                    7,
                    DesignResourceCount::TextBytes,
                    7,
                    0,
                    8,
                    at(scope, "semantic.rule.inputs.role", Some(0)),
                ),
                (
                    count,
                    8 * count - 1,
                    DesignResourceCount::TextBytes,
                    8 * count - 1,
                    8 * (count - 1),
                    8,
                    at(scope, "semantic.rule.inputs.role", Some(count - 1)),
                ),
            ] {
                let mut meter = DesignResourceMeter::new(selected(entries, text)?);
                let expected = DesignResourceLimitError::Count {
                    unit,
                    maximum,
                    consumed,
                    requested,
                    location,
                };
                assert_eq!(
                    charge_semantic_rule_resources(&rule, &mut meter, scope),
                    Err(expected)
                );
                assert_eq!(meter.finish(), Err(expected));
            }
            let mut definition = rule.definition().clone();
            clear(&mut definition);
            let empty = SemanticRuleDeclaration::new(definition, SemanticRuleLimits::new(0, 0, 0))?;
            let mut meter = DesignResourceMeter::new(selected(1, 1)?);
            charge_semantic_rule_resources(&empty, &mut meter, scope)?;
            let demand = meter.finish()?;
            assert_eq!(
                (
                    demand.collection_entries(),
                    demand.text_bytes(),
                    demand.counter_bits()
                ),
                (0, 0, 0)
            );
            for rule in [&empty, &rule] {
                let mut meter = DesignResourceMeter::new(selected(1, 1)?);
                let original = meter
                    .charge_entries(2, prior)
                    .err()
                    .ok_or("original refusal")?;
                assert_eq!(
                    charge_semantic_rule_resources(rule, &mut meter, scope),
                    Err(original)
                );
                assert_eq!(meter.finish(), Err(original));
            }
        }
    }
    Ok(())
}
