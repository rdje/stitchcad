//! Standalone original membership and kind loans never substitute an equal-data owner.
use sc_core::{name::MachineToken, ontology::EntityId, value::*};
use std::error::Error;
type TestResult<T = ()> = Result<T, Box<dyn Error>>;
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn record(n: u128, kind: LogicalInputKind, derived: bool) -> TestResult<LogicalInputDeclaration> {
    Ok(LogicalInputDeclaration::new(
        LogicalInputDeclarationDefinition {
            id: id(n),
            source: id(90),
            kind,
            domain: vec![MachineToken::new("private_member")?],
            state: if derived {
                LogicalInputState::Derived { operation: id(91) }
            } else {
                LogicalInputState::Unknown {
                    observation: id(92),
                }
            },
        },
        LogicalInputLimits::new(1, 0, 14, 2),
    )?)
}
fn records() -> TestResult<Vec<LogicalInputDeclaration>> {
    Ok(vec![
        record(30, LogicalInputKind::PocketOpening, false)?,
        record(10, LogicalInputKind::NotionSize, true)?,
        record(20, LogicalInputKind::NotionSize, false)?,
    ])
}
#[test]
fn original_membership_keeps_unsorted_storage_kind_and_actual_owner_loans() -> TestResult {
    let original = records()?;
    let expected = original.clone();
    let pointer = original.as_ptr();
    let policy = LogicalInputInventoryLimits::new(3, 9);
    let inputs = LogicalInputs::new(original, policy)?;
    assert_eq!(inputs.declarations(), expected.as_slice());
    assert_eq!(inputs.declarations().as_ptr(), pointer);
    assert_eq!(inputs.limits(), policy);
    assert_eq!(
        (policy.declarations(), policy.validation_inspections()),
        (3, 9)
    );
    assert_eq!(
        (
            inputs.shape().declarations(),
            inputs.shape().validation_inspections()
        ),
        (3, 9)
    );
    let other = LogicalInputs::new(expected, LogicalInputInventoryLimits::new(100, 10000))?;
    assert_eq!(
        inputs, other,
        "constructor policies do not change authored equality"
    );
    for (index, actual) in inputs.declarations().iter().enumerate() {
        let binding = inputs
            .resolve(actual.id(), actual.kind())
            .map_err(|e| e.to_string())?;
        let copied = binding;
        assert_eq!(copied.index(), index);
        assert!(std::ptr::eq(copied.inputs(), &inputs));
        assert!(std::ptr::eq(copied.declaration(), actual));
        assert!(std::ptr::eq(
            inputs.declaration(actual.id()).ok_or("owned original id")?,
            actual
        ));
        let foreign = other
            .resolve(actual.id(), actual.kind())
            .map_err(|e| e.to_string())?;
        assert_eq!(foreign.declaration(), actual);
        assert!(!std::ptr::eq(foreign.inputs(), binding.inputs()));
        assert!(!std::ptr::eq(foreign.declaration(), binding.declaration()));
        assert!(matches!(
            actual.definition().state.tag(),
            LogicalInputStateTag::Unknown | LogicalInputStateTag::Derived
        ));
        assert!(!format!("{copied:?}").contains("private_member"));
    }
    assert!(
        inputs.declaration(id(90)).is_none(),
        "source references are not owned declarations"
    );
    assert!(
        inputs.declaration(id(91)).is_none(),
        "operation references are not owned declarations"
    );
    assert!(
        inputs.declaration(id(92)).is_none(),
        "observation references are not owned declarations"
    );
    Ok(())
}
#[test]
fn complete_population_and_comparisons_refuse_before_duplicate_identity_checks() -> TestResult {
    let make = || -> TestResult<Vec<LogicalInputDeclaration>> {
        Ok(vec![
            record(40, LogicalInputKind::PocketOpening, false)?,
            record(50, LogicalInputKind::NotionSize, true)?,
            record(40, LogicalInputKind::NotionSize, false)?,
        ])
    };
    for (limits, expected) in [
        (
            LogicalInputInventoryLimits::new(2, 0),
            LogicalInputInventoryRefusal::Declarations {
                allowed: 2,
                required: 3,
            },
        ),
        (
            LogicalInputInventoryLimits::new(3, 8),
            LogicalInputInventoryRefusal::ValidationInspections {
                allowed: 8,
                required: 9,
            },
        ),
        (
            LogicalInputInventoryLimits::new(3, 9),
            LogicalInputInventoryRefusal::DuplicateIdentity {
                identity: id(40),
                first: 0,
                duplicate: 2,
            },
        ),
    ] {
        let error = LogicalInputs::new(make()?, limits)
            .err()
            .ok_or("complete count/work or duplicate refusal")?;
        assert_eq!(
            error,
            LogicalInputInventoryError {
                limits,
                refusal: expected
            }
        );
    }
    let original = record(70, LogicalInputKind::PocketOpening, false)?;
    let error = LogicalInputs::new(
        vec![original.clone(), original],
        LogicalInputInventoryLimits::new(2, 4),
    )
    .err()
    .ok_or("identical duplicate refused")?;
    assert_eq!(
        error.refusal,
        LogicalInputInventoryRefusal::DuplicateIdentity {
            identity: id(70),
            first: 0,
            duplicate: 1
        }
    );
    Ok(())
}
#[test]
fn missing_and_wrong_kind_keep_original_requests_and_real_records_without_fallback() -> TestResult {
    let inputs = LogicalInputs::new(records()?, LogicalInputInventoryLimits::new(3, 9))?;
    let expected = inputs
        .declaration(id(30))
        .ok_or("actual original opening")?;
    let error = inputs
        .resolve(id(30), LogicalInputKind::NotionSize)
        .err()
        .ok_or("wrong logical kind")?;
    let LogicalInputLookupError::Kind {
        expected: kind,
        actual,
    } = error
    else {
        return Err("actual kind cause".into());
    };
    assert_eq!(kind, LogicalInputKind::NotionSize);
    assert!(std::ptr::eq(actual, expected));
    assert_eq!(actual.kind(), LogicalInputKind::PocketOpening);
    assert!(!format!("{error:?} {error}").contains("private_member"));
    for id in [id(0), id(90), id(91), id(92)] {
        let error = inputs
            .resolve(id, LogicalInputKind::PocketOpening)
            .err()
            .ok_or("missing logical input")?;
        assert!(
            matches!(error,LogicalInputLookupError::Missing {declaration,expected} if declaration==id && expected==LogicalInputKind::PocketOpening)
        );
    }
    let empty = LogicalInputs::new(vec![], LogicalInputInventoryLimits::new(0, 0))?;
    assert!(empty.declarations().is_empty());
    assert!(empty.declaration(id(1)).is_none());
    assert_eq!(
        (
            empty.shape().declarations(),
            empty.shape().validation_inspections()
        ),
        (0, 0)
    );
    assert!(
        matches!(empty.resolve(id(1),LogicalInputKind::NotionSize),Err(LogicalInputLookupError::Missing {declaration,expected}) if declaration==id(1)&&expected==LogicalInputKind::NotionSize)
    );
    Ok(())
}
