//! Original semantic ownership,kind loans and complete resource composition.
use core::num::{NonZeroU64, NonZeroUsize};
use sc_core::{
    design::*,
    name::MachineToken,
    ontology::{EntityId, NotionSize},
    recipe::*,
    value::*,
};
use std::error::Error;
type TestResult<T = ()> = Result<T, Box<dyn Error>>;
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn button(n: u128, populated: bool) -> TestResult<SemanticOperation> {
    let inputs = if populated {
        vec![
            ButtonholePolicyInput {
                role: MachineToken::new("policy_b")?,
                target: CanonicalInputTarget::Length(id(901)),
            },
            ButtonholePolicyInput {
                role: MachineToken::new("policy_a")?,
                target: CanonicalInputTarget::Scalar {
                    declaration: id(902),
                    kind: ScalarInputKind::Ratio,
                },
            },
        ]
    } else {
        vec![]
    };
    Ok(SemanticOperation::Buttonhole(ButtonholeOperation::new(
        ButtonholeOperationDefinition {
            id: id(n),
            button: NotionSize::Declaration(id(903)),
            policy: id(900),
            inputs,
        },
        ButtonholeOperationLimits::new(20, 160, 400),
    )?))
}
fn logical(n: u128) -> TestResult<SemanticOperation> {
    Ok(SemanticOperation::LogicalDerivation(
        LogicalDerivationOperation::new(
            LogicalDerivationDefinition {
                id: id(n),
                output: LogicalInputTarget {
                    declaration: id(700),
                    kind: LogicalInputKind::PocketOpening,
                },
                rule: id(900),
                inputs: vec![LogicalDerivationInput {
                    role: MachineToken::new("source_a")?,
                    source: LogicalDerivationInputSource::Logical(LogicalInputTarget {
                        declaration: id(701),
                        kind: LogicalInputKind::PocketOpening,
                    }),
                }],
            },
            LogicalDerivationLimits::new(10, 80, 100),
        )?,
    ))
}
fn operations() -> TestResult<Vec<SemanticOperation>> {
    Ok(vec![button(90, true)?, logical(70)?, button(80, false)?])
}
fn selected(entries: usize, text: usize) -> TestResult<DesignResourceLimits> {
    Ok(DesignResourceLimits::new(
        NonZeroUsize::new(entries).ok_or("positive entries")?,
        NonZeroUsize::new(text).ok_or("positive text")?,
        NonZeroU64::MAX,
    ))
}
fn at(field: &'static str, owner: Option<u128>, index: Option<usize>) -> DesignResourceLocation {
    DesignResourceLocation {
        scope: DesignResourceScope::Inputs,
        owner: owner.map(id),
        field,
        index,
    }
}
#[test]
fn original_order_and_kind_bindings_retain_actual_owner_member_and_authored_buffers() -> TestResult
{
    let original = operations()?;
    let expected = original.clone();
    let buffer = original.as_ptr();
    let recipe = SemanticRecipe::new(original, SemanticRecipeLimits::new(3, 9))?;
    assert_eq!(recipe.operations(), expected);
    assert_eq!(recipe.operations().as_ptr(), buffer);
    assert_eq!(
        (
            recipe.shape().operations(),
            recipe.shape().validation_inspections()
        ),
        (3, 9)
    );
    for ((index, operation), (n, kind)) in recipe.operations().iter().enumerate().zip([
        (90, SemanticOperationKind::Buttonhole),
        (70, SemanticOperationKind::LogicalDerivation),
        (80, SemanticOperationKind::Buttonhole),
    ]) {
        assert_eq!((operation.id(), operation.kind()), (id(n), kind));
        let bound = recipe.resolve(id(n), kind).map_err(|e| e.to_string())?;
        assert!(std::ptr::eq(bound.recipe(), &recipe));
        assert!(std::ptr::eq(bound.operation(), operation));
        assert_eq!(bound.index(), index);
        assert!(std::ptr::eq(
            recipe.operation(id(n)).ok_or("actual member")?,
            operation
        ));
    }
    assert!(recipe.operation(id(900)).is_none());
    assert!(
        matches!(recipe.resolve(id(900),SemanticOperationKind::Buttonhole),
        Err(SemanticOperationLookupError::Missing {operation,expected:SemanticOperationKind::Buttonhole}) if operation==id(900))
    );
    let actual = recipe.operations().first().ok_or("first member")?;
    assert!(
        matches!(recipe.resolve(id(90),SemanticOperationKind::LogicalDerivation),
        Err(SemanticOperationLookupError::Kind {expected:SemanticOperationKind::LogicalDerivation,actual:p}) if std::ptr::eq(p,actual))
    );
    let wider = SemanticRecipe::new(
        recipe.operations().to_vec(),
        SemanticRecipeLimits::new(30, 900),
    )?;
    assert_eq!(recipe, wider);
    assert_eq!(recipe.limits(), SemanticRecipeLimits::new(3, 9));
    let other = wider
        .resolve(id(90), SemanticOperationKind::Buttonhole)
        .map_err(|e| e.to_string())?;
    assert!(!std::ptr::eq(other.recipe(), &recipe));
    assert!(!std::ptr::eq(other.operation(), actual));
    assert!(!format!("{recipe:?} {:?}", recipe.operations()).contains("policy_b"));
    Ok(())
}
#[test]
fn complete_quotas_precede_cross_family_and_identical_duplicates_and_empty_is_explicit(
) -> TestResult {
    use SemanticRecipeRefusal as R;
    for (limits, refusal) in [
        (
            SemanticRecipeLimits::new(2, 0),
            R::Operations {
                allowed: 2,
                required: 3,
            },
        ),
        (
            SemanticRecipeLimits::new(3, 8),
            R::ValidationInspections {
                allowed: 8,
                required: 9,
            },
        ),
        (
            SemanticRecipeLimits::new(3, 9),
            R::DuplicateIdentity {
                identity: id(90),
                first: 0,
                duplicate: 1,
            },
        ),
    ] {
        assert_eq!(
            SemanticRecipe::new(
                vec![button(90, true)?, logical(90)?, button(80, false)?],
                limits
            ),
            Err(SemanticRecipeError { limits, refusal })
        );
    }
    let original = button(90, true)?;
    let limits = SemanticRecipeLimits::new(3, 9);
    assert_eq!(
        SemanticRecipe::new(vec![original.clone(), logical(70)?, original], limits),
        Err(SemanticRecipeError {
            limits,
            refusal: R::DuplicateIdentity {
                identity: id(90),
                first: 0,
                duplicate: 2
            }
        })
    );
    let empty = SemanticRecipe::new(vec![], SemanticRecipeLimits::new(0, 0))?;
    assert!(empty.operations().is_empty());
    assert!(empty.operation(id(90)).is_none());
    assert!(matches!(
        empty.resolve(id(90), SemanticOperationKind::Buttonhole),
        Err(SemanticOperationLookupError::Missing { .. })
    ));
    assert_eq!(
        (
            empty.shape().operations(),
            empty.shape().validation_inspections()
        ),
        (0, 0)
    );
    Ok(())
}
#[test]
fn complete_inventory_then_original_members_preserve_cumulative_demand_and_first_causes(
) -> TestResult {
    let recipe = SemanticRecipe::new(operations()?, SemanticRecipeLimits::new(30, 900))?;
    let mut meter = DesignResourceMeter::new(selected(8, 27)?);
    meter.charge_entries(2, at("prior", None, Some(7)))?;
    meter.charge_text("old", at("prior", None, Some(7)))?;
    meter.charge_counter_bits(1, at("prior", None, Some(7)))?;
    charge_semantic_recipe_resources(&recipe, &mut meter)?;
    let demand = meter.finish()?;
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (8, 27, 1)
    );
    for (entries, text, unit, maximum, consumed, requested, location) in [
        (
            2,
            24,
            DesignResourceCount::CollectionEntries,
            2,
            0,
            3,
            at("semantic.operations", None, None),
        ),
        (
            4,
            24,
            DesignResourceCount::CollectionEntries,
            4,
            3,
            2,
            at("semantic.buttonhole.inputs", Some(90), None),
        ),
        (
            5,
            24,
            DesignResourceCount::CollectionEntries,
            5,
            5,
            1,
            at("semantic.logical.inputs", Some(70), None),
        ),
        (
            6,
            23,
            DesignResourceCount::TextBytes,
            23,
            16,
            8,
            at("semantic.logical.inputs.role", Some(70), Some(0)),
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
            charge_semantic_recipe_resources(&recipe, &mut meter),
            Err(expected)
        );
        assert_eq!(meter.finish(), Err(expected));
    }
    let empty = SemanticRecipe::new(vec![], SemanticRecipeLimits::new(0, 0))?;
    let mut meter = DesignResourceMeter::new(selected(1, 1)?);
    charge_semantic_recipe_resources(&empty, &mut meter)?;
    assert_eq!(meter.finish()?.collection_entries(), 0);
    for recipe in [&recipe, &empty] {
        let mut meter = DesignResourceMeter::new(selected(1, 1)?);
        let original = meter
            .charge_entries(2, at("original.first", None, Some(7)))
            .err()
            .ok_or("original refusal")?;
        assert_eq!(
            charge_semantic_recipe_resources(recipe, &mut meter),
            Err(original)
        );
        assert_eq!(meter.finish(), Err(original));
    }
    Ok(())
}
