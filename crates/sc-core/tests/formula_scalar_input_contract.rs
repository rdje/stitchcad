//! General scalar adapters verify the actual canonical target without authorizing a registry.
#![allow(clippy::expect_used)]
use sc_core::{
    name::MachineToken,
    ontology::{EdgeRef, EntityId, LocalTag, PointRef},
    recipe::{
        FormulaBindingKind as B, FormulaDeclaration as D, FormulaDeclarationSource as S,
        FormulaInputOrigin, FormulaRecipe, FormulaReservedName, FormulaScalarInput as Input,
        FormulaScalarInputAdapterRule as Rule, FormulaScalarInputOrigin as Origin,
    },
    value::{
        LengthDeclaration, LengthDeclarationDefinition, LengthState,
        ScalarInputDeclaration as Canonical, ScalarInputDeclarationDefinition as Definition,
        ScalarInputKind as K, ScalarInputReadError as ReadError, ScalarInputState as State,
        ScalarInputValue as Value, ValueProvenanceValidation,
    },
};
use sc_units::Length;
use std::error::Error as _;

const KINDS: [(K, B); 6] = [
    (K::Length, B::Length),
    (K::Angle, B::Angle),
    (K::Area, B::Area),
    (K::Ratio, B::Ratio),
    (K::Count, B::Count),
    (K::Boolean, B::Boolean),
];
const ORIGINS: [Origin; 3] = [Origin::Parameter, Origin::Profile, Origin::Material];

fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn canonical(kind: K, state: State) -> Canonical {
    Canonical::new(Definition {
        id: id(11),
        source: id(12),
        kind,
        state,
    })
    .expect("valid canonical scalar")
}
fn unknown(kind: K) -> Canonical {
    canonical(
        kind,
        if kind == K::Area {
            State::Derived { formula: id(16) }
        } else {
            State::Unknown {
                observation: id(15),
            }
        },
    )
}
fn values() -> [(Value, B); 5] {
    [
        (Length::ZERO.into(), B::Length),
        (Value::from_angle_microdegrees(-720_000_000), B::Angle),
        (Value::from_ratio_ppm(-1_000_000), B::Ratio),
        (Value::from_count(0).expect("zero Count"), B::Count),
        (Value::from_boolean(false), B::Boolean),
    ]
}

#[test]
fn matching_all_kinds_and_origins_retains_actual_metadata_without_reading_a_value() {
    let name = MachineToken::new("authored_scalar").expect("name");
    for origin in ORIGINS {
        for (kind, binding) in KINDS {
            let record = unknown(kind);
            let metadata = D::input(&name, origin, id(13), id(11), binding);
            let input = Input::from_declaration(metadata, &record).expect("matched id/kind");
            assert_eq!(B::from(kind), binding, "independent kind table");
            assert_eq!(input.name(), name.as_str());
            assert_eq!(input.origin(), origin);
            assert_eq!(input.input_id(), id(13));
            assert!(std::ptr::eq(input.canonical_declaration(), &record));
            assert!(std::ptr::eq(input.state(), record.state()));
            let expected = format!("{:?}", metadata);
            assert_eq!(format!("{:?}", input.metadata()), expected);
            assert!(matches!(input.metadata().source(), S::Input { .. }));
            if let S::Input {
                origin: actual,
                input: metadata_id,
                declaration,
                kind: declared,
            } = input.metadata().source()
            {
                assert_eq!(
                    (actual, metadata_id, declaration, declared),
                    (origin, id(13), id(11), binding)
                );
            }
            assert_eq!(
                input.provenance_validation(),
                ValueProvenanceValidation::DeferredToDesignAndG4
            );
        }
    }
}

#[test]
fn identity_precedes_kind_and_refusals_retain_real_candidate_and_name() {
    let name = MachineToken::new("private_customer_input").expect("name");
    let record = unknown(K::Boolean);
    let metadata = D::input(&name, Origin::Parameter, id(13), id(99), B::Length);
    let error = Input::from_declaration(metadata, &record).expect_err("wrong id and kind");
    assert_eq!(
        error.rule(),
        Rule::CanonicalIdentity {
            declared: id(99),
            actual: id(11)
        }
    );
    assert_eq!(error.name(), name.as_str());
    assert!(std::ptr::eq(error.canonical_declaration(), &record));
    assert!(
        !format!("{error:?}").contains(name.as_str()),
        "Debug does not publish customer names"
    );
    assert!(
        error.to_string().contains(name.as_str()),
        "explicit Display retains actual name"
    );
}

#[test]
fn every_kind_mismatch_is_refused_before_any_unknown_or_derived_read() {
    let name = MachineToken::new("input_value").expect("name");
    for origin in ORIGINS {
        for (actual, correct) in KINDS {
            let record = unknown(actual);
            for (_, declared) in KINDS {
                let metadata = D::input(&name, origin, id(13), id(11), declared);
                let result = Input::from_declaration(metadata, &record);
                if declared == correct {
                    assert_eq!(result.expect("matching kind").state(), record.state());
                } else {
                    let error = result.expect_err("actual canonical kind differs");
                    assert_eq!(error.rule(), Rule::CanonicalKind { declared, actual });
                    assert!(std::ptr::eq(error.canonical_declaration(), &record));
                }
            }
        }
    }
}

#[test]
fn all_authored_states_keep_typed_zero_false_raw_turns_and_actual_required_actions() {
    let name = MachineToken::new("input_value").expect("name");
    let mut reads = 0;
    for origin in ORIGINS {
        for (value, binding) in values() {
            for state in [
                State::Known {
                    value,
                    evidence: vec![id(21), id(22)],
                },
                State::Assumed {
                    value,
                    assumption: id(23),
                },
                State::Preference {
                    value,
                    provenance: id(24),
                },
                State::Unknown {
                    observation: id(25),
                },
                State::Derived { formula: id(26) },
            ] {
                let record = canonical(value.kind(), state.clone());
                let before = record.clone();
                let input = Input::from_declaration(
                    D::input(&name, origin, id(13), id(11), binding),
                    &record,
                )
                .expect("valid scalar source");
                match state {
                    State::Known { .. } | State::Assumed { .. } | State::Preference { .. } => {
                        let result = input.read_authored().expect("present canonical value");
                        assert_eq!(result.value(), value);
                        assert!(std::ptr::eq(
                            result.input().canonical_declaration(),
                            &record
                        ));
                        assert_eq!(result.input().state(), &state);
                    }
                    State::Unknown { observation }
                    | State::Derived {
                        formula: observation,
                    } => {
                        let expected = if matches!(state, State::Unknown { .. }) {
                            ReadError::RequiresObservation {
                                declaration: id(11),
                                observation,
                            }
                        } else {
                            ReadError::RequiresEvaluation {
                                declaration: id(11),
                                formula: observation,
                            }
                        };
                        let error = input.read_authored().expect_err("required action retained");
                        assert_eq!(error.reason(), expected);
                        assert!(std::ptr::eq(error.input().canonical_declaration(), &record));
                        assert_eq!(error.input().state(), &state);
                        assert_eq!(
                            error
                                .source()
                                .and_then(|source| source.downcast_ref::<ReadError>()),
                            Some(&expected)
                        );
                    }
                }
                assert_eq!(record, before);
                reads += 1;
            }
        }
        let area = canonical(K::Area, State::Derived { formula: id(27) });
        let input =
            Input::from_declaration(D::input(&name, origin, id(13), id(11), B::Area), &area)
                .expect("actual derived Area metadata");
        assert_eq!(
            input
                .read_authored()
                .expect_err("Area is computed")
                .reason(),
            ReadError::RequiresEvaluation {
                declaration: id(11),
                formula: id(27)
            }
        );
        reads += 1;
    }
    assert_eq!(reads, 78);
}

#[test]
fn other_source_variants_cannot_be_admitted_by_matching_kind_or_identity() {
    let name = MachineToken::new("other_source").expect("name");
    let record = unknown(K::Length);
    let length = LengthDeclaration::new(LengthDeclarationDefinition {
        id: id(11),
        source: id(12),
        state: LengthState::Unknown {
            observation: id(15),
        },
    })
    .expect("canonical Length");
    let recipe = FormulaRecipe::parse("let other_source:length=1 um")
        .expect("syntax")
        .normalize_literals()
        .expect("literal");
    let mut metadata = vec![
        D::length_input(&name, FormulaInputOrigin::Parameter, id(13), &length),
        D::point(&name, PointRef::new(id(14), LocalTag::FIRST)),
        D::edge(&name, EdgeRef::new(id(14), LocalTag::FIRST)),
        D::recipe(&recipe, 1).expect("prior binding"),
    ];
    for reserved in FormulaReservedName::ALL {
        metadata.push(D::reserved(reserved));
    }
    for declaration in metadata {
        let expected = Rule::NotScalarInput {
            kind: declaration.kind(),
            origin: declaration.origin(),
        };
        let error =
            Input::from_declaration(declaration, &record).expect_err("source is not general Input");
        assert_eq!(error.rule(), expected);
        assert_eq!(error.name(), declaration.name());
    }
}
