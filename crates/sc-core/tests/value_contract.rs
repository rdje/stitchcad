//! Length input state/provenance is canonical; observation and evaluation never default to zero.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use sc_core::ontology::EntityId;
use sc_core::value::{
    LengthDeclaration, LengthDeclarationDefinition, LengthDeclarationError, LengthState,
    LengthValueError, ValueProvenanceValidation,
};
use sc_units::{Length, Unit};
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn length(n: i64) -> Length {
    Length::from_micrometres(n).unwrap()
}
fn input(state: LengthState) -> LengthDeclarationDefinition {
    LengthDeclarationDefinition {
        id: id(1),
        source: id(2),
        state,
    }
}
#[test]
fn all_five_states_retain_one_canonical_source_and_their_distinct_required_provenance() {
    let states = [
        LengthState::Known {
            value: length(760_000),
            evidence: vec![id(10), id(11)],
        },
        LengthState::Assumed {
            value: length(770_000),
            assumption: id(12),
        },
        LengthState::Unknown {
            observation: id(13),
        },
        LengthState::Preference {
            value: length(780_000),
            provenance: id(14),
        },
        LengthState::Derived { formula: id(15) },
    ];
    for state in states {
        let definition = input(state);
        let declaration = LengthDeclaration::new(definition.clone()).unwrap();
        assert_eq!(declaration.id(), definition.id);
        assert_eq!(declaration.source(), definition.source);
        assert_eq!(declaration.definition(), &definition);
        assert!(std::ptr::eq(
            declaration.state(),
            &declaration.definition().state
        ));
        assert_eq!(
            declaration.provenance_validation(),
            ValueProvenanceValidation::DeferredToDesignAndG4
        );
    }
}
#[test]
fn known_values_require_nonempty_distinct_evidence_and_do_not_reorder_authored_inventory() {
    assert_eq!(
        LengthDeclaration::new(input(LengthState::Known {
            value: length(1),
            evidence: vec![]
        })),
        Err(LengthDeclarationError::NoEvidence(id(1)))
    );
    assert_eq!(
        LengthDeclaration::new(input(LengthState::Known {
            value: length(1),
            evidence: vec![id(10), id(11), id(10)]
        })),
        Err(LengthDeclarationError::DuplicateEvidence {
            declaration: id(1),
            evidence: id(10)
        })
    );
    let definition = input(LengthState::Known {
        value: length(1),
        evidence: vec![id(11), id(10)],
    });
    assert_eq!(
        LengthDeclaration::new(definition.clone())
            .unwrap()
            .definition(),
        &definition
    );
}
#[test]
fn authored_known_assumed_and_preference_values_preserve_explicit_signed_lengths_and_zero() {
    for value in [length(-30_000), Length::ZERO, length(30_000)] {
        for state in [
            LengthState::Known {
                value,
                evidence: vec![id(10)],
            },
            LengthState::Assumed {
                value,
                assumption: id(11),
            },
            LengthState::Preference {
                value,
                provenance: id(12),
            },
        ] {
            let declaration = LengthDeclaration::new(input(state)).unwrap();
            assert_eq!(declaration.authored_value(), Ok(value));
            assert_eq!(
                declaration.provenance_validation(),
                ValueProvenanceValidation::DeferredToDesignAndG4
            );
        }
    }
}
#[test]
fn unknown_fact_names_required_observation_and_never_becomes_zero_or_separate_preference() {
    let unknown = LengthDeclaration::new(input(LengthState::Unknown {
        observation: id(13),
    }))
    .unwrap();
    let preference = LengthDeclaration::new(LengthDeclarationDefinition {
        id: id(3),
        ..input(LengthState::Preference {
            value: Length::ZERO,
            provenance: id(14),
        })
    })
    .unwrap();
    assert_eq!(preference.authored_value(), Ok(Length::ZERO));
    assert_eq!(
        unknown.authored_value(),
        Err(LengthValueError::RequiresObservation {
            declaration: id(1),
            observation: id(13)
        })
    );
    assert_eq!(
        unknown.state(),
        &LengthState::Unknown {
            observation: id(13)
        }
    );
    assert!(unknown
        .authored_value()
        .unwrap_err()
        .to_string()
        .contains(&id(13).to_string()));
}
#[test]
fn derived_input_names_sole_formula_without_storing_a_separately_authored_result() {
    let derived = LengthDeclaration::new(input(LengthState::Derived { formula: id(15) })).unwrap();
    assert_eq!(
        derived.authored_value(),
        Err(LengthValueError::RequiresEvaluation {
            declaration: id(1),
            formula: id(15)
        })
    );
    assert_eq!(derived.state(), &LengthState::Derived { formula: id(15) });
    assert!(derived
        .authored_value()
        .unwrap_err()
        .to_string()
        .contains(&id(15).to_string()));
    assert_eq!(
        derived.provenance_validation(),
        ValueProvenanceValidation::DeferredToDesignAndG4
    );
}
#[test]
fn exact_unit_conversion_is_authored_content_without_loss_of_its_state_or_source() {
    let value = Length::from_rational(3, 4, Unit::Inch).unwrap();
    let declaration = LengthDeclaration::new(input(LengthState::Assumed {
        value,
        assumption: id(12),
    }))
    .unwrap();
    assert_eq!(
        declaration.authored_value().unwrap().as_micrometres(),
        19_050
    );
    assert_eq!(declaration.source(), id(2));
    assert_eq!(
        declaration.state(),
        &LengthState::Assumed {
            value,
            assumption: id(12)
        }
    );
    // Entered unit belongs to Measurement metadata; the canonical declaration stores internal Length.
}
#[test]
fn replacing_input_or_mutating_cloned_evidence_cannot_change_the_old_canonical_declaration() {
    let definition = input(LengthState::Known {
        value: length(760_000),
        evidence: vec![id(10)],
    });
    let original = LengthDeclaration::new(definition.clone()).unwrap();
    let mut changed = original.definition().clone();
    if let LengthState::Known { value, evidence } = &mut changed.state {
        *value = length(770_000);
        evidence.push(id(11));
    }
    let replacement = LengthDeclaration::new(changed.clone()).unwrap();
    assert_eq!(original.definition(), &definition);
    assert_eq!(replacement.definition(), &changed);
    assert_eq!(original.authored_value(), Ok(length(760_000)));
    assert_eq!(replacement.authored_value(), Ok(length(770_000)));
    changed.state = LengthState::Unknown {
        observation: id(16),
    };
    changed.source = id(20);
    let unknown = LengthDeclaration::new(changed).unwrap();
    assert_eq!(original.definition(), &definition);
    assert_eq!(unknown.source(), id(20));
    assert_eq!(
        unknown.authored_value(),
        Err(LengthValueError::RequiresObservation {
            declaration: id(1),
            observation: id(16)
        })
    );
}
#[test]
fn replacing_unknown_with_derived_changes_the_required_source_without_mutating_old_view() {
    let original = LengthDeclaration::new(input(LengthState::Unknown {
        observation: id(13),
    }))
    .unwrap();
    let mut definition = original.definition().clone();
    definition.state = LengthState::Derived { formula: id(15) };
    let replacement = LengthDeclaration::new(definition).unwrap();
    assert_eq!(
        original.authored_value(),
        Err(LengthValueError::RequiresObservation {
            declaration: id(1),
            observation: id(13)
        })
    );
    assert_eq!(
        replacement.authored_value(),
        Err(LengthValueError::RequiresEvaluation {
            declaration: id(1),
            formula: id(15)
        })
    );
}
