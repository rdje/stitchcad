//! Typed logical recipe sources preserve original data and complete refusal locations.
use core::num::{NonZeroU64, NonZeroUsize};
use sc_core::{
    design::*,
    name::MachineToken,
    ontology::{EntityId, ProfileParameterRef},
    recipe::*,
    value::*,
};
use std::error::Error;
type TestResult<T = ()> = Result<T, Box<dyn Error>>;
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn target(n: u128, kind: LogicalInputKind) -> LogicalInputTarget {
    LogicalInputTarget {
        declaration: id(n),
        kind,
    }
}
fn definition(kind: LogicalInputKind) -> TestResult<LogicalDerivationDefinition> {
    let inputs = [
        (
            "source_d",
            LogicalDerivationInputSource::Logical(target(20, LogicalInputKind::PocketOpening)),
        ),
        (
            "source_c",
            LogicalDerivationInputSource::Profile {
                parameter: ProfileParameterRef::new(id(31)),
                kind: LogicalInputKind::NotionSize,
            },
        ),
        (
            "source_b",
            LogicalDerivationInputSource::Numeric(CanonicalInputTarget::Scalar {
                declaration: id(40),
                kind: ScalarInputKind::Boolean,
            }),
        ),
        (
            "source_a",
            LogicalDerivationInputSource::Numeric(CanonicalInputTarget::Length(id(41))),
        ),
    ]
    .into_iter()
    .map(|(role, source)| -> TestResult<LogicalDerivationInput> {
        Ok(LogicalDerivationInput {
            role: MachineToken::new(role)?,
            source,
        })
    })
    .collect::<TestResult<Vec<_>>>()?;
    Ok(LogicalDerivationDefinition {
        id: id(10),
        output: target(70, kind),
        rule: id(900),
        inputs,
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
fn output_roles_and_all_typed_sources_retain_original_order_buffers_and_authored_content(
) -> TestResult {
    for kind in [
        LogicalInputKind::PocketOpening,
        LogicalInputKind::NotionSize,
    ] {
        let original = definition(kind)?;
        let expected = original.clone();
        let buffer = original.inputs.as_ptr();
        let operation =
            LogicalDerivationOperation::new(original, LogicalDerivationLimits::new(4, 32, 16))?;
        assert_eq!(operation.definition(), &expected);
        assert_eq!(operation.definition().inputs.as_ptr(), buffer);
        assert_eq!(operation.id(), id(10));
        assert_eq!(operation.definition().output, target(70, kind));
        assert_eq!(
            (
                operation.shape().inputs(),
                operation.shape().role_text_bytes(),
                operation.shape().validation_inspections()
            ),
            (4, 32, 16)
        );
        let wider = LogicalDerivationOperation::new(
            expected.clone(),
            LogicalDerivationLimits::new(40, 320, 1600),
        )?;
        assert_eq!(operation, wider);
        assert_eq!(operation.limits(), LogicalDerivationLimits::new(4, 32, 16));
        assert!(!format!("{operation:?} {expected:?}").contains("source_d"));
        let mut detached = expected;
        detached.output = target(99, kind);
        detached.inputs.clear();
        assert_eq!(operation.definition().output, target(70, kind));
        assert_eq!(operation.definition().inputs.len(), 4);
    }
    Ok(())
}
#[test]
fn whole_quotas_precede_duplicate_roles_and_shared_source_references_are_preserved() -> TestResult {
    let mut repeated = definition(LogicalInputKind::PocketOpening)?;
    let role = repeated.inputs.first().ok_or("first role")?.role.clone();
    repeated.inputs.last_mut().ok_or("last role")?.role = role;
    use LogicalDerivationRefusal as R;
    for (limits, refusal) in [
        (
            LogicalDerivationLimits::new(3, 0, 0),
            R::Limit {
                resource: LogicalDerivationResource::Inputs,
                allowed: 3,
                required: 4,
            },
        ),
        (
            LogicalDerivationLimits::new(4, 31, 0),
            R::Limit {
                resource: LogicalDerivationResource::RoleTextBytes,
                allowed: 31,
                required: 32,
            },
        ),
        (
            LogicalDerivationLimits::new(4, 32, 15),
            R::Limit {
                resource: LogicalDerivationResource::ValidationInspections,
                allowed: 15,
                required: 16,
            },
        ),
        (
            LogicalDerivationLimits::new(4, 32, 16),
            R::DuplicateRole {
                first: 0,
                duplicate: 3,
            },
        ),
    ] {
        assert_eq!(
            LogicalDerivationOperation::new(repeated.clone(), limits),
            Err(LogicalDerivationError {
                operation: id(10),
                limits,
                refusal
            })
        );
    }
    let mut shared = definition(LogicalInputKind::NotionSize)?;
    let source = shared.inputs.first().ok_or("first source")?.source;
    shared.inputs.last_mut().ok_or("last source")?.source = source;
    let shared = LogicalDerivationOperation::new(shared, LogicalDerivationLimits::new(4, 32, 16))?;
    assert_eq!(shared.definition().inputs.len(), 4);
    let mut empty = definition(LogicalInputKind::PocketOpening)?;
    empty.inputs.clear();
    let empty = LogicalDerivationOperation::new(empty, LogicalDerivationLimits::new(0, 0, 0))?;
    assert_eq!(
        (
            empty.shape().inputs(),
            empty.shape().role_text_bytes(),
            empty.shape().validation_inspections()
        ),
        (0, 0, 0)
    );
    assert_eq!(
        empty.definition().output,
        target(70, LogicalInputKind::PocketOpening)
    );
    assert_eq!(empty.definition().rule, id(900));
    Ok(())
}
#[test]
fn actual_complete_content_preserves_cumulative_demand_and_original_first_refusals() -> TestResult {
    let operation = LogicalDerivationOperation::new(
        definition(LogicalInputKind::NotionSize)?,
        LogicalDerivationLimits::new(40, 320, 1600),
    )?;
    let mut meter = DesignResourceMeter::new(selected(6, 35)?);
    meter.charge_entries(2, at("prior", Some(7)))?;
    meter.charge_text("old", at("prior", Some(7)))?;
    meter.charge_counter_bits(1, at("prior", Some(7)))?;
    charge_logical_derivation_resources(&operation, &mut meter)?;
    let demand = meter.finish()?;
    assert_eq!(
        (
            demand.collection_entries(),
            demand.text_bytes(),
            demand.counter_bits()
        ),
        (6, 35, 1)
    );
    for (entries, text, unit, maximum, consumed, requested, location) in [
        (
            3,
            32,
            DesignResourceCount::CollectionEntries,
            3,
            0,
            4,
            at("semantic.logical.inputs", None),
        ),
        (
            4,
            7,
            DesignResourceCount::TextBytes,
            7,
            0,
            8,
            at("semantic.logical.inputs.role", Some(0)),
        ),
        (
            4,
            31,
            DesignResourceCount::TextBytes,
            31,
            24,
            8,
            at("semantic.logical.inputs.role", Some(3)),
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
            charge_logical_derivation_resources(&operation, &mut meter),
            Err(expected)
        );
        assert_eq!(meter.finish(), Err(expected));
    }
    let mut definition = definition(LogicalInputKind::PocketOpening)?;
    definition.inputs.clear();
    let empty = LogicalDerivationOperation::new(definition, LogicalDerivationLimits::new(0, 0, 0))?;
    let mut meter = DesignResourceMeter::new(selected(1, 1)?);
    charge_logical_derivation_resources(&empty, &mut meter)?;
    assert_eq!(meter.finish()?.collection_entries(), 0);
    for operation in [&empty, &operation] {
        let mut meter = DesignResourceMeter::new(selected(1, 1)?);
        let original = meter
            .charge_entries(2, at("original.first", Some(7)))
            .err()
            .ok_or("original refusal")?;
        assert_eq!(
            charge_logical_derivation_resources(operation, &mut meter),
            Err(original)
        );
        assert_eq!(meter.finish(), Err(original));
    }
    Ok(())
}
