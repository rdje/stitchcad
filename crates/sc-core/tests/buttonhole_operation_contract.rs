//! Original typed Buttonhole recipe inputs; no independent physical length or fabricated source.
use core::num::{NonZeroU64, NonZeroUsize};
use sc_core::{
    design::*,
    name::MachineToken,
    ontology::{EntityId, NotionSize, ProfileParameterRef},
    recipe::*,
    value::*,
};
use std::error::Error;
type TestResult<T = ()> = Result<T, Box<dyn Error>>;
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn definition(button: NotionSize) -> TestResult<ButtonholeOperationDefinition> {
    Ok(ButtonholeOperationDefinition {
        id: id(10),
        button,
        policy: id(900),
        inputs: vec![
            ButtonholePolicyInput {
                role: MachineToken::new("policy_b")?,
                target: CanonicalInputTarget::Scalar {
                    declaration: id(20),
                    kind: ScalarInputKind::Ratio,
                },
            },
            ButtonholePolicyInput {
                role: MachineToken::new("policy_a")?,
                target: CanonicalInputTarget::Length(id(21)),
            },
        ],
    })
}
fn selected(entries: usize, text: usize) -> TestResult<DesignResourceLimits> {
    Ok(DesignResourceLimits::new(
        NonZeroUsize::new(entries).ok_or("positive entries")?,
        NonZeroUsize::new(text).ok_or("positive text")?,
        NonZeroU64::MAX,
    ))
}
fn at(field: &'static str, index: Option<usize>) -> DesignResourceLocation {
    DesignResourceLocation {
        scope: DesignResourceScope::Inputs,
        owner: Some(id(10)),
        field,
        index,
    }
}
#[test]
fn both_original_button_origins_and_ordered_policy_buffers_move_without_value_or_history_invention(
) -> TestResult {
    for button in [
        NotionSize::Declaration(id(30)),
        NotionSize::Profile(ProfileParameterRef::new(id(31))),
    ] {
        let original = definition(button)?;
        let expected = original.clone();
        let buffer = original.inputs.as_ptr();
        let operation =
            ButtonholeOperation::new(original, ButtonholeOperationLimits::new(2, 16, 4))?;
        assert_eq!(operation.definition(), &expected);
        assert_eq!(operation.definition().inputs.as_ptr(), buffer);
        assert_eq!(operation.definition().button, button);
        assert_eq!(operation.id(), id(10));
        assert_eq!(
            (
                operation.shape().policy_inputs(),
                operation.shape().role_text_bytes(),
                operation.shape().validation_inspections()
            ),
            (2, 16, 4)
        );
        let wider = ButtonholeOperation::new(
            expected.clone(),
            ButtonholeOperationLimits::new(20, 160, 400),
        )?;
        assert_eq!(operation, wider);
        assert_eq!(operation.limits(), ButtonholeOperationLimits::new(2, 16, 4));
        assert!(!format!("{operation:?} {expected:?}").contains("policy_b"));
        let mut detached = expected;
        detached.button = NotionSize::Declaration(id(99));
        detached.inputs.clear();
        assert_eq!(operation.definition().button, button);
        assert_eq!(operation.definition().inputs.len(), 2);
    }
    Ok(())
}
#[test]
fn complete_original_quotas_precede_duplicate_role_validation_and_references_are_not_deduplicated(
) -> TestResult {
    let mut repeated = definition(NotionSize::Declaration(id(30)))?;
    let first = repeated.inputs.first().ok_or("original role")?.role.clone();
    repeated.inputs.get_mut(1).ok_or("second role")?.role = first;
    use ButtonholeOperationRefusal as R;
    for (limits, refusal) in [
        (
            ButtonholeOperationLimits::new(1, 0, 0),
            R::Limit {
                resource: ButtonholeOperationResource::PolicyInputs,
                allowed: 1,
                required: 2,
            },
        ),
        (
            ButtonholeOperationLimits::new(2, 15, 0),
            R::Limit {
                resource: ButtonholeOperationResource::RoleTextBytes,
                allowed: 15,
                required: 16,
            },
        ),
        (
            ButtonholeOperationLimits::new(2, 16, 3),
            R::Limit {
                resource: ButtonholeOperationResource::ValidationInspections,
                allowed: 3,
                required: 4,
            },
        ),
        (
            ButtonholeOperationLimits::new(2, 16, 4),
            R::DuplicateRole {
                first: 0,
                duplicate: 1,
            },
        ),
    ] {
        assert_eq!(
            ButtonholeOperation::new(repeated.clone(), limits),
            Err(ButtonholeOperationError {
                operation: id(10),
                limits,
                refusal,
            })
        );
    }
    let mut shared = definition(NotionSize::Declaration(id(30)))?;
    let target = shared.inputs.first().ok_or("first original target")?.target;
    shared
        .inputs
        .get_mut(1)
        .ok_or("second original target")?
        .target = target;
    let shared = ButtonholeOperation::new(shared, ButtonholeOperationLimits::new(2, 16, 4))?;
    assert_eq!(shared.definition().inputs.len(), 2);
    let mut empty = definition(NotionSize::Declaration(id(30)))?;
    empty.inputs.clear();
    let empty = ButtonholeOperation::new(empty, ButtonholeOperationLimits::new(0, 0, 0))?;
    assert_eq!(
        (
            empty.shape().policy_inputs(),
            empty.shape().role_text_bytes(),
            empty.shape().validation_inspections()
        ),
        (0, 0, 0)
    );
    assert_eq!(empty.definition().button, NotionSize::Declaration(id(30)));
    assert_eq!(empty.definition().policy, id(900));
    Ok(())
}
#[test]
fn standalone_content_uses_actual_roles_and_cumulative_original_first_refusals() -> TestResult {
    let operation = ButtonholeOperation::new(
        definition(NotionSize::Declaration(id(30)))?,
        ButtonholeOperationLimits::new(200, 1600, 40000),
    )?;
    let mut meter = DesignResourceMeter::new(selected(3, 19)?);
    meter.charge_entries(1, at("prior", Some(7)))?;
    meter.charge_text("old", at("prior", Some(7)))?;
    meter.charge_counter_bits(2, at("prior", Some(7)))?;
    charge_buttonhole_operation_resources(&operation, &mut meter)?;
    let demand = meter.finish()?;
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (3, 19, 2)
    );
    for (entries, text, unit, maximum, consumed, requested, location) in [
        (
            1,
            16,
            DesignResourceCount::CollectionEntries,
            1,
            0,
            2,
            at("semantic.buttonhole.inputs", None),
        ),
        (
            2,
            7,
            DesignResourceCount::TextBytes,
            7,
            0,
            8,
            at("semantic.buttonhole.inputs.role", Some(0)),
        ),
        (
            2,
            15,
            DesignResourceCount::TextBytes,
            15,
            8,
            8,
            at("semantic.buttonhole.inputs.role", Some(1)),
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
            charge_buttonhole_operation_resources(&operation, &mut meter),
            Err(expected)
        );
        assert_eq!(meter.finish(), Err(expected));
    }
    let mut definition = definition(NotionSize::Profile(ProfileParameterRef::new(id(31))))?;
    definition.inputs.clear();
    let empty = ButtonholeOperation::new(definition, ButtonholeOperationLimits::new(0, 0, 0))?;
    let mut meter = DesignResourceMeter::new(selected(1, 1)?);
    charge_buttonhole_operation_resources(&empty, &mut meter)?;
    assert_eq!(meter.finish()?.collection_entries(), 0);
    for operation in [&empty, &operation] {
        let mut meter = DesignResourceMeter::new(selected(1, 1)?);
        let original = meter
            .charge_entries(2, at("original.first", Some(7)))
            .err()
            .ok_or("original refusal")?;
        assert_eq!(
            charge_buttonhole_operation_resources(operation, &mut meter),
            Err(original)
        );
        assert_eq!(meter.finish(), Err(original));
    }
    Ok(())
}
