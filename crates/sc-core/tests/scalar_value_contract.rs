//! Canonical scalar state, raw angle/storage domains and computation-only Area.
#![allow(clippy::expect_used)]
use sc_core::{
    ontology::EntityId,
    value::{
        ScalarInputDeclaration as Declaration, ScalarInputDeclarationDefinition as Definition,
        ScalarInputDeclarationError as Error, ScalarInputKind as Kind,
        ScalarInputReadError as ReadError, ScalarInputState as State, ScalarInputStateTag as Tag,
        ScalarInputValue as Value, ScalarInputValueError as ValueError, ValueProvenanceValidation,
    },
};
use sc_units::Length;

fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}

fn definition(kind: Kind, state: State) -> Definition {
    Definition {
        id: id(1),
        source: id(2),
        kind,
        state,
    }
}

fn values() -> [Value; 5] {
    [
        Length::from_micrometres(-1_000_000_000)
            .expect("negative scalar limit")
            .into(),
        Value::from_angle_microdegrees(-720_000_000),
        Value::from_ratio_ppm(-3_000_000),
        Value::from_count(0).expect("zero is a count"),
        Value::from_boolean(false),
    ]
}

#[test]
fn scalar_values_keep_signed_storage_raw_turns_and_boolean_presence() {
    for integer in [i64::MIN, -720_000_000, -1, 0, 720_000_000, i64::MAX] {
        let angle = Value::from_angle_microdegrees(integer);
        assert_eq!(angle.kind(), Kind::Angle);
        assert_eq!(angle.numeric_integer(), Some(integer));
        assert_eq!(angle.boolean(), None);
        let ratio = Value::from_ratio_ppm(integer);
        assert_eq!(ratio.kind(), Kind::Ratio);
        assert_eq!(ratio.numeric_integer(), Some(integer));
    }
    for integer in [i64::MIN, -1] {
        assert_eq!(
            Value::from_count(integer),
            Err(ValueError::NegativeCount(integer))
        );
    }
    for integer in [0, i64::MAX] {
        let count = Value::from_count(integer).expect("nonnegative signed storage");
        assert_eq!(count.kind(), Kind::Count);
        assert_eq!(count.numeric_integer(), Some(integer));
    }
    for integer in [-1_000_000_000, 0, 1_000_000_000] {
        let length: Value = Length::from_micrometres(integer)
            .expect("valid Length")
            .into();
        assert_eq!(length.kind(), Kind::Length);
        assert_eq!(length.numeric_integer(), Some(integer));
    }
    for boolean in [false, true] {
        let value = Value::from_boolean(boolean);
        assert_eq!(value.kind(), Kind::Boolean);
        assert_eq!(value.boolean(), Some(boolean));
        assert_eq!(value.numeric_integer(), None);
    }
}

#[test]
fn canonical_states_retain_identity_provenance_and_the_actual_value_request() {
    let mut states_checked = 0;
    for value in values() {
        for state in [
            State::Known {
                value,
                evidence: vec![id(12), id(11)],
            },
            State::Assumed {
                value,
                assumption: id(13),
            },
            State::Preference {
                value,
                provenance: id(14),
            },
            State::Unknown {
                observation: id(15),
            },
            State::Derived { formula: id(16) },
        ] {
            let authored = definition(value.kind(), state.clone());
            let canonical = Declaration::new(authored.clone()).expect("valid canonical state");
            assert_eq!(canonical.id(), id(1));
            assert_eq!(canonical.source(), id(2));
            assert_eq!(canonical.kind(), value.kind());
            assert_eq!(canonical.definition(), &authored);
            assert!(std::ptr::eq(
                canonical.state(),
                &canonical.definition().state
            ));
            assert_eq!(
                canonical.provenance_validation(),
                ValueProvenanceValidation::DeferredToDesignAndG4
            );
            match state {
                State::Known { .. } | State::Assumed { .. } | State::Preference { .. } => {
                    assert_eq!(canonical.authored_value(), Ok(value));
                }
                State::Unknown { observation } => {
                    assert_eq!(
                        canonical.authored_value(),
                        Err(ReadError::RequiresObservation {
                            declaration: id(1),
                            observation,
                        })
                    );
                }
                State::Derived { formula } => {
                    assert_eq!(
                        canonical.authored_value(),
                        Err(ReadError::RequiresEvaluation {
                            declaration: id(1),
                            formula,
                        })
                    );
                }
            }
            assert_eq!(canonical.definition(), &authored);
            states_checked += 1;
        }
    }
    let area = Declaration::new(definition(Kind::Area, State::Derived { formula: id(17) }))
        .expect("Area is computation-only");
    assert_eq!(area.kind(), Kind::Area);
    assert_eq!(
        area.authored_value(),
        Err(ReadError::RequiresEvaluation {
            declaration: id(1),
            formula: id(17),
        })
    );
    assert_eq!(states_checked + 1, 26);
}

#[test]
fn authored_kind_mismatches_and_non_derived_area_are_structural_refusals() {
    for declared in Kind::ALL {
        for value in values() {
            for state in [
                State::Known {
                    value,
                    evidence: vec![id(11)],
                },
                State::Assumed {
                    value,
                    assumption: id(12),
                },
                State::Preference {
                    value,
                    provenance: id(13),
                },
            ] {
                let tag = state.tag();
                let actual = Declaration::new(definition(declared, state));
                if declared == Kind::Area {
                    assert_eq!(
                        actual,
                        Err(Error::AreaMustBeDerived {
                            declaration: id(1),
                            state: tag
                        })
                    );
                } else if declared != value.kind() {
                    assert_eq!(
                        actual,
                        Err(Error::KindMismatch {
                            declaration: id(1),
                            declared,
                            actual: value.kind(),
                        })
                    );
                } else {
                    assert_eq!(actual.expect("matching kind").authored_value(), Ok(value));
                }
            }
        }
    }
    assert_eq!(
        Declaration::new(definition(
            Kind::Area,
            State::Unknown {
                observation: id(18)
            }
        )),
        Err(Error::AreaMustBeDerived {
            declaration: id(1),
            state: Tag::Unknown
        })
    );
}

#[test]
fn known_evidence_keeps_authored_order_and_the_first_actual_duplicate() {
    for value in values() {
        assert_eq!(
            Declaration::new(definition(
                value.kind(),
                State::Known {
                    value,
                    evidence: vec![]
                }
            )),
            Err(Error::NoEvidence(id(1)))
        );
        assert_eq!(
            Declaration::new(definition(
                value.kind(),
                State::Known {
                    value,
                    evidence: vec![id(11), id(12), id(12), id(11)],
                }
            )),
            Err(Error::DuplicateEvidence {
                declaration: id(1),
                evidence: id(12)
            })
        );
        let authored = definition(
            value.kind(),
            State::Known {
                value,
                evidence: vec![id(12), id(11)],
            },
        );
        assert_eq!(
            Declaration::new(authored.clone())
                .expect("distinct references")
                .definition(),
            &authored
        );
    }
}

#[test]
fn editing_a_replacement_preserves_the_prior_canonical_record_and_value() {
    let value = Value::from_boolean(false);
    let authored = definition(
        Kind::Boolean,
        State::Known {
            value,
            evidence: vec![id(11)],
        },
    );
    let original = Declaration::new(authored.clone()).expect("Known false");
    let mut replacement = original.definition().clone();
    if let State::Known { value, evidence } = &mut replacement.state {
        *value = Value::from_boolean(true);
        evidence.push(id(12));
    }
    replacement.source = id(3);
    let replaced = Declaration::new(replacement).expect("new true claim");
    assert_eq!(original.definition(), &authored);
    assert_eq!(original.authored_value(), Ok(Value::from_boolean(false)));
    assert_eq!(replaced.authored_value(), Ok(Value::from_boolean(true)));
    assert_eq!(original.source(), id(2));
    assert_eq!(replaced.source(), id(3));
}
