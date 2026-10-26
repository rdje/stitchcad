//! Canonical ownership is distinct from metadata binding, source truth and numeric execution.
#![allow(clippy::expect_used)]
use sc_core::{
    ontology::EntityId,
    value::{
        CanonicalInputPosition as Position, CanonicalInputs, CanonicalInputsError as Error,
        LengthDeclaration, LengthDeclarationDefinition, LengthState, LengthValueError,
        ScalarInputDeclaration, ScalarInputDeclarationDefinition, ScalarInputKind as Kind,
        ScalarInputReadError, ScalarInputState as State, ScalarInputValue as Value,
    },
};
use sc_units::Length;

fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn length(identity: u128, state: LengthState) -> LengthDeclaration {
    LengthDeclaration::new(LengthDeclarationDefinition {
        id: id(identity),
        source: id(900),
        state,
    })
    .expect("valid canonical fixture length")
}
fn scalar(identity: u128, kind: Kind, state: State) -> ScalarInputDeclaration {
    ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
        id: id(identity),
        source: id(800),
        kind,
        state,
    })
    .expect("valid canonical fixture scalar")
}
fn unknown(identity: u128) -> LengthDeclaration {
    length(
        identity,
        LengthState::Unknown {
            observation: id(901),
        },
    )
}
fn boolean(identity: u128) -> ScalarInputDeclaration {
    scalar(
        identity,
        Kind::Boolean,
        State::Preference {
            value: Value::from_boolean(false),
            provenance: id(801),
        },
    )
}

#[test]
fn authored_families_and_all_states_survive_with_canonical_borrow_identity() {
    let value = Length::from_micrometres(-17).expect("signed fixture length");
    let lengths = vec![
        length(
            5,
            LengthState::Known {
                value,
                evidence: vec![id(903), id(902)],
            },
        ),
        length(
            4,
            LengthState::Assumed {
                value,
                assumption: id(904),
            },
        ),
        unknown(3),
        length(
            2,
            LengthState::Preference {
                value,
                provenance: id(905),
            },
        ),
        length(1, LengthState::Derived { formula: id(906) }),
    ];
    let scalars = vec![
        scalar(
            16,
            Kind::Count,
            State::Preference {
                value: Value::from_count(0).expect("explicit zero Count"),
                provenance: id(807),
            },
        ),
        scalar(
            15,
            Kind::Boolean,
            State::Known {
                value: Value::from_boolean(true),
                evidence: vec![id(803), id(802)],
            },
        ),
        scalar(
            14,
            Kind::Ratio,
            State::Assumed {
                value: Value::from_ratio_ppm(-3_000_000),
                assumption: id(804),
            },
        ),
        scalar(
            13,
            Kind::Angle,
            State::Unknown {
                observation: id(805),
            },
        ),
        boolean(12),
        scalar(11, Kind::Area, State::Derived { formula: id(806) }),
    ];
    let expected_lengths = lengths.clone();
    let expected_scalars = scalars.clone();
    let inputs = CanonicalInputs::new(lengths, scalars).unwrap();
    assert_eq!(inputs.lengths(), expected_lengths);
    assert_eq!(inputs.scalars(), expected_scalars);
    for record in inputs.lengths() {
        assert!(core::ptr::eq(inputs.length(record.id()).unwrap(), record));
    }
    for record in inputs.scalars() {
        assert!(core::ptr::eq(inputs.scalar(record.id()).unwrap(), record));
    }
    assert_eq!(inputs.length(id(999)), None);
    assert_eq!(inputs.scalar(id(999)), None);
}

#[test]
fn unknown_and_derived_records_never_acquire_a_numeric_fallback() {
    let inputs = CanonicalInputs::new(
        vec![
            unknown(1),
            length(2, LengthState::Derived { formula: id(7) }),
        ],
        vec![
            scalar(3, Kind::Boolean, State::Unknown { observation: id(8) }),
            scalar(4, Kind::Area, State::Derived { formula: id(9) }),
        ],
    )
    .unwrap();
    assert_eq!(
        inputs.length(id(1)).unwrap().authored_value(),
        Err(LengthValueError::RequiresObservation {
            declaration: id(1),
            observation: id(901)
        })
    );
    assert_eq!(
        inputs.length(id(2)).unwrap().authored_value(),
        Err(LengthValueError::RequiresEvaluation {
            declaration: id(2),
            formula: id(7)
        })
    );
    assert_eq!(
        inputs.scalar(id(3)).unwrap().authored_value(),
        Err(ScalarInputReadError::RequiresObservation {
            declaration: id(3),
            observation: id(8)
        })
    );
    assert_eq!(
        inputs.scalar(id(4)).unwrap().authored_value(),
        Err(ScalarInputReadError::RequiresEvaluation {
            declaration: id(4),
            formula: id(9)
        })
    );
}

#[test]
fn dedicated_length_and_general_length_kind_remain_distinct_representations() {
    let scalar_length = scalar(
        2,
        Kind::Length,
        State::Preference {
            value: Length::ZERO.into(),
            provenance: id(3),
        },
    );
    let inputs = CanonicalInputs::new(vec![unknown(1)], vec![scalar_length.clone()]).unwrap();
    assert_eq!(inputs.length(id(2)), None);
    assert_eq!(inputs.scalar(id(1)), None);
    assert_eq!(inputs.scalar(id(2)), Some(&scalar_length));
}

#[test]
fn identical_length_duplicates_are_refused_at_their_original_positions() {
    let original = unknown(5);
    assert_eq!(
        CanonicalInputs::new(vec![original.clone(), unknown(3), original], vec![]),
        Err(Error::DuplicateIdentity {
            identity: id(5),
            first: Position::Length { index: 0 },
            duplicate: Position::Length { index: 2 }
        })
    );
}

#[test]
fn identical_scalar_duplicates_are_refused_at_their_original_positions() {
    let original = boolean(5);
    assert_eq!(
        CanonicalInputs::new(vec![], vec![original.clone(), original]),
        Err(Error::DuplicateIdentity {
            identity: id(5),
            first: Position::Scalar { index: 0 },
            duplicate: Position::Scalar { index: 1 }
        })
    );
}

#[test]
fn cross_family_identity_cannot_select_or_convert_one_of_two_records() {
    assert_eq!(
        CanonicalInputs::new(
            vec![unknown(7)],
            vec![
                boolean(8),
                scalar(7, Kind::Length, State::Unknown { observation: id(3) })
            ]
        ),
        Err(Error::DuplicateIdentity {
            identity: id(7),
            first: Position::Length { index: 0 },
            duplicate: Position::Scalar { index: 1 }
        })
    );
}

#[test]
fn empty_drafts_are_legal_and_referenced_ids_are_not_claimed_as_owned_records() {
    let empty = CanonicalInputs::new(vec![], vec![]).unwrap();
    assert!(empty.lengths().is_empty() && empty.scalars().is_empty());
    // Provenance references are retained data, not proof of ownership, existence or kind here.
    let inputs = CanonicalInputs::new(
        vec![length(1, LengthState::Unknown { observation: id(2) })],
        vec![boolean(2)],
    )
    .unwrap();
    assert_eq!(
        inputs.length(id(1)).unwrap().state(),
        &LengthState::Unknown { observation: id(2) }
    );
}
