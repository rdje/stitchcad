//! Public finite logical records retain authored state and admit all bounds before validation.
use sc_core::{name::MachineToken, ontology::EntityId, value::*};
use std::error::Error;
type TestResult<T = ()> = Result<T, Box<dyn Error>>;
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn token(s: &str) -> TestResult<MachineToken> {
    Ok(MachineToken::new(s)?)
}
fn definition(
    kind: LogicalInputKind,
    state: LogicalInputState,
) -> TestResult<LogicalInputDeclarationDefinition> {
    Ok(LogicalInputDeclarationDefinition {
        id: id(10),
        source: id(20),
        kind,
        domain: vec![token("small")?, token("large")?],
        state,
    })
}
fn unlimited() -> LogicalInputLimits {
    LogicalInputLimits::new(usize::MAX, usize::MAX, usize::MAX, usize::MAX)
}
#[test]
fn every_logical_kind_and_state_keeps_original_domain_selection_and_provenance() -> TestResult {
    for kind in [
        LogicalInputKind::PocketOpening,
        LogicalInputKind::NotionSize,
    ] {
        let states = vec![
            (
                LogicalInputState::Known {
                    value: token("large")?,
                    evidence: vec![id(31), id(30)],
                },
                LogicalInputStateTag::Known,
            ),
            (
                LogicalInputState::Assumed {
                    value: token("small")?,
                    assumption: id(40),
                },
                LogicalInputStateTag::Assumed,
            ),
            (
                LogicalInputState::Unknown {
                    observation: id(50),
                },
                LogicalInputStateTag::Unknown,
            ),
            (
                LogicalInputState::Choice {
                    value: token("large")?,
                    decision: id(60),
                },
                LogicalInputStateTag::Choice,
            ),
            (
                LogicalInputState::Preference {
                    value: token("small")?,
                    provenance: id(70),
                },
                LogicalInputStateTag::Preference,
            ),
            (
                LogicalInputState::Derived { operation: id(80) },
                LogicalInputStateTag::Derived,
            ),
        ];
        for (state, tag) in states {
            let input = definition(kind, state)?;
            assert_eq!(input.state.tag(), tag);
            let expected = input.clone();
            let domain_pointer = input.domain.as_ptr();
            let evidence_pointer = match &input.state {
                LogicalInputState::Known { evidence, .. } => Some(evidence.as_ptr()),
                _ => None,
            };
            let record = LogicalInputDeclaration::new(input, unlimited())?;
            assert_eq!(record.id(), id(10));
            assert_eq!(record.kind(), kind);
            assert_eq!(record.limits(), unlimited());
            assert_eq!(record.definition(), &expected);
            assert_eq!(record.definition().state.tag(), tag);
            assert_eq!(
                record.definition().domain.as_ptr(),
                domain_pointer,
                "original vector is moved,not copied/reordered"
            );
            if let LogicalInputState::Known { evidence, .. } = &record.definition().state {
                assert_eq!(Some(evidence.as_ptr()), evidence_pointer);
            }
            let shape = record.shape();
            assert_eq!(shape.domain_members(), 2);
            assert_eq!(
                shape.evidence_records(),
                if tag == LogicalInputStateTag::Known {
                    2
                } else {
                    0
                }
            );
            assert_eq!(
                shape.text_bytes(),
                if matches!(
                    tag,
                    LogicalInputStateTag::Unknown | LogicalInputStateTag::Derived
                ) {
                    10
                } else {
                    15
                }
            );
            assert_eq!(
                shape.validation_inspections(),
                if tag == LogicalInputStateTag::Known {
                    10
                } else {
                    6
                }
            );
            assert!(!format!(
                "{record:?} {:?} {:?}",
                record.definition(),
                record.definition().state
            )
            .contains("small"));
            assert!(!format!(
                "{record:?} {:?} {:?}",
                record.definition(),
                record.definition().state
            )
            .contains("large"));
        }
    }
    Ok(())
}
#[test]
fn complete_original_quotas_are_inclusive_and_precede_invalid_domain_or_state() -> TestResult {
    let make = || {
        definition(
            LogicalInputKind::PocketOpening,
            LogicalInputState::Known {
                value: token("small")?,
                evidence: vec![id(31), id(30)],
            },
        )
    };
    let exact = LogicalInputLimits::new(2, 2, 15, 10);
    let proof = LogicalInputDeclaration::new(make()?, exact)?;
    assert_eq!(proof.limits(), exact);
    let differently_admitted = LogicalInputDeclaration::new(make()?, unlimited())?;
    assert_eq!(
        proof, differently_admitted,
        "authored equality is independent of constructor policy"
    );
    assert_ne!(proof.limits(), differently_admitted.limits());
    assert_eq!(
        (
            exact.domain_members(),
            exact.evidence_records(),
            exact.text_bytes(),
            exact.validation_inspections()
        ),
        (2, 2, 15, 10)
    );
    for (limits, expected) in [
        (
            LogicalInputLimits::new(1, 0, 0, 0),
            LogicalInputRefusal::DomainMembers {
                allowed: 1,
                required: 2,
            },
        ),
        (
            LogicalInputLimits::new(2, 1, 0, 0),
            LogicalInputRefusal::EvidenceRecords {
                allowed: 1,
                required: 2,
            },
        ),
        (
            LogicalInputLimits::new(2, 2, 14, 0),
            LogicalInputRefusal::TextBytes {
                allowed: 14,
                required: 15,
            },
        ),
        (
            LogicalInputLimits::new(2, 2, 15, 9),
            LogicalInputRefusal::ValidationInspections {
                allowed: 9,
                required: 10,
            },
        ),
    ] {
        let error = LogicalInputDeclaration::new(make()?, limits)
            .err()
            .ok_or("complete allowance refusal")?;
        assert_eq!(
            error,
            LogicalInputDeclarationError {
                declaration: id(10),
                refusal: expected
            }
        );
    }
    // Invalid evidence is still counted before any validation,without a successful prefix credit.
    let input = definition(
        LogicalInputKind::NotionSize,
        LogicalInputState::Known {
            value: token("small")?,
            evidence: vec![id(31), id(31), id(31)],
        },
    )?;
    let error =
        LogicalInputDeclaration::new(input, LogicalInputLimits::new(2, 2, usize::MAX, usize::MAX))
            .err()
            .ok_or("all repeated evidence counted")?;
    assert_eq!(
        error.refusal,
        LogicalInputRefusal::EvidenceRecords {
            allowed: 2,
            required: 3
        }
    );
    let mut input = definition(
        LogicalInputKind::PocketOpening,
        LogicalInputState::Unknown { observation: id(1) },
    )?;
    input.domain.push(token("large")?);
    let error =
        LogicalInputDeclaration::new(input, LogicalInputLimits::new(2, 0, usize::MAX, usize::MAX))
            .err()
            .ok_or("all repeated domain members counted")?;
    assert_eq!(
        error.refusal,
        LogicalInputRefusal::DomainMembers {
            allowed: 2,
            required: 3
        }
    );
    Ok(())
}
#[test]
fn original_domain_and_state_refusals_keep_first_authored_indices_without_defaulting() -> TestResult
{
    let state = || LogicalInputState::Unknown {
        observation: id(50),
    };
    let mut empty = definition(LogicalInputKind::PocketOpening, state())?;
    empty.domain.clear();
    let error = LogicalInputDeclaration::new(empty, LogicalInputLimits::new(0, 0, 0, 0))
        .err()
        .ok_or("nonempty actual domain required")?;
    assert_eq!(error.refusal, LogicalInputRefusal::EmptyDomain);
    let mut repeated = definition(
        LogicalInputKind::PocketOpening,
        LogicalInputState::Known {
            value: token("foreign")?,
            evidence: vec![],
        },
    )?;
    repeated.domain.extend([token("small")?, token("large")?]);
    let error = LogicalInputDeclaration::new(repeated, unlimited())
        .err()
        .ok_or("domain duplicates precede state")?;
    assert_eq!(
        error.refusal,
        LogicalInputRefusal::DuplicateDomain {
            first: 0,
            duplicate: 2
        }
    );
    for state in [
        LogicalInputState::Known {
            value: token("foreign")?,
            evidence: vec![],
        },
        LogicalInputState::Assumed {
            value: token("foreign")?,
            assumption: id(40),
        },
        LogicalInputState::Choice {
            value: token("foreign")?,
            decision: id(60),
        },
        LogicalInputState::Preference {
            value: token("foreign")?,
            provenance: id(70),
        },
    ] {
        let tag = state.tag();
        let input = definition(LogicalInputKind::NotionSize, state)?;
        let error = LogicalInputDeclaration::new(input, unlimited())
            .err()
            .ok_or("selection in actual domain")?;
        assert_eq!(
            error.refusal,
            LogicalInputRefusal::SelectionOutsideDomain { state: tag }
        );
        assert!(!error.to_string().contains("foreign"));
    }
    let input = definition(
        LogicalInputKind::NotionSize,
        LogicalInputState::Known {
            value: token("small")?,
            evidence: vec![],
        },
    )?;
    let error = LogicalInputDeclaration::new(input, unlimited())
        .err()
        .ok_or("Known evidence required")?;
    assert_eq!(error.refusal, LogicalInputRefusal::NoEvidence);
    let input = definition(
        LogicalInputKind::NotionSize,
        LogicalInputState::Known {
            value: token("small")?,
            evidence: vec![id(31), id(30), id(31), id(30)],
        },
    )?;
    let error = LogicalInputDeclaration::new(input, unlimited())
        .err()
        .ok_or("distinct original evidence required")?;
    assert_eq!(
        error.refusal,
        LogicalInputRefusal::DuplicateEvidence {
            first: 0,
            duplicate: 2,
            evidence: id(31)
        }
    );
    Ok(())
}
