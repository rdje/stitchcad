//! Actual canonical source/state reads, separate from static metadata and whole evaluation.
#![allow(clippy::expect_used)]
use sc_core::{
    name::MachineToken,
    ontology::{EdgeRef, EntityId, LocalTag, PointRef},
    recipe::{
        FormulaBindingKind as B, FormulaDeclaration as D, FormulaDeclarationSource as S,
        FormulaInputOrigin as I, FormulaKind as K, FormulaLengthInput as Input,
        FormulaNumericValue as V, FormulaRational as R, FormulaRecipe, FormulaReservedName,
        FormulaScalarInputOrigin as P,
    },
    value::{
        LengthDeclaration, LengthDeclarationDefinition, LengthState, LengthValueError,
        ValueProvenanceValidation,
    },
};
use sc_units::Length;

fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}

fn canonical(state: LengthState) -> LengthDeclaration {
    LengthDeclaration::new(LengthDeclarationDefinition {
        id: id(11),
        source: id(12),
        state,
    })
    .expect("valid canonical state")
}

#[test]
fn validated_lengths_import_exactly_at_both_scalar_limits_and_zero() {
    for integer in [-1_000_000_000, -1, 0, 1, 1_000_000_000] {
        let length = Length::from_micrometres(integer).expect("admitted unit length");
        let value = V::from(length);
        assert_eq!(value.kind(), K::Length);
        assert_eq!(
            value.rational(),
            R::from_signed_integer(i128::from(integer))
        );
        assert_eq!(
            value,
            V::new(K::Length, value.rational()).expect("same domain")
        );
    }
}

#[test]
fn every_origin_and_state_retains_the_actual_source_and_unavailability_request() {
    let name = MachineToken::new("body_width").expect("machine name");
    let zero = Length::ZERO;
    let negative = Length::from_micrometres(-1_000_000_000).expect("negative scalar limit");
    let positive = Length::from_micrometres(1_000_000_000).expect("positive scalar limit");
    let states = [
        LengthState::Known {
            value: zero,
            evidence: vec![id(21), id(22)],
        },
        LengthState::Assumed {
            value: negative,
            assumption: id(23),
        },
        LengthState::Preference {
            value: positive,
            provenance: id(24),
        },
        LengthState::Unknown {
            observation: id(25),
        },
        LengthState::Derived { formula: id(26) },
    ];
    let mut combinations = 0;
    for origin in [
        I::Measurement,
        I::Ease,
        I::Parameter,
        I::Profile,
        I::Material,
    ] {
        for state in &states {
            let record = canonical(state.clone());
            let before = record.clone();
            let metadata = D::length_input(&name, origin, id(13), &record);
            let input = Input::from_declaration(metadata).expect("actual canonical input");
            assert!(std::ptr::eq(input.canonical_declaration(), &record));
            assert!(std::ptr::eq(input.state(), record.state()));
            assert_eq!(input.metadata().name(), name.as_str());
            assert_eq!(input.metadata().origin(), metadata.origin());
            let S::LengthInput {
                origin: actual,
                input: metadata_id,
                declaration,
            } = input.metadata().source()
            else {
                panic!("original source variant lost");
            };
            assert_eq!(actual, origin);
            assert_eq!(metadata_id, id(13));
            assert!(std::ptr::eq(declaration, &record));
            assert_eq!(declaration.id(), id(11));
            assert_eq!(declaration.source(), id(12));
            assert_eq!(
                input.provenance_validation(),
                ValueProvenanceValidation::DeferredToDesignAndG4
            );
            match state {
                LengthState::Known { value, .. }
                | LengthState::Assumed { value, .. }
                | LengthState::Preference { value, .. } => {
                    let read = input.read_authored().expect("present authored value");
                    assert!(std::ptr::eq(read.input().canonical_declaration(), &record));
                    assert_eq!(read.input().state(), state);
                    assert_eq!(read.value().kind(), K::Length);
                    assert_eq!(
                        read.value().rational(),
                        R::from_signed_integer(i128::from(value.as_micrometres()))
                    );
                }
                LengthState::Unknown { observation } => {
                    let error = input.read_authored().expect_err("unknown has no value");
                    assert!(std::ptr::eq(error.input().canonical_declaration(), &record));
                    assert_eq!(error.input().state(), state);
                    assert_eq!(
                        error.reason(),
                        LengthValueError::RequiresObservation {
                            declaration: id(11),
                            observation: *observation
                        }
                    );
                }
                LengthState::Derived { formula } => {
                    let error = input
                        .read_authored()
                        .expect_err("derived requires evaluation");
                    assert!(std::ptr::eq(error.input().canonical_declaration(), &record));
                    assert_eq!(error.input().state(), state);
                    assert_eq!(
                        error.reason(),
                        LengthValueError::RequiresEvaluation {
                            declaration: id(11),
                            formula: *formula
                        }
                    );
                }
            }
            assert_eq!(record, before, "read must not mutate the source");
            combinations += 1;
        }
    }
    assert_eq!(combinations, 25);
}

#[test]
fn a_length_kind_claim_or_other_metadata_cannot_impersonate_a_canonical_input() {
    let name = MachineToken::new("supplied").expect("machine name");
    for origin in [P::Parameter, P::Profile, P::Material] {
        for kind in [B::Length, B::Area, B::Angle, B::Ratio, B::Count, B::Boolean] {
            assert!(Input::from_declaration(D::input(&name, origin, id(1), id(2), kind)).is_none());
        }
    }
    for metadata in [
        D::point(&name, PointRef::new(id(3), LocalTag::FIRST)),
        D::edge(&name, EdgeRef::new(id(4), LocalTag::FIRST)),
    ] {
        assert!(Input::from_declaration(metadata).is_none());
    }
    for reserved in FormulaReservedName::ALL {
        assert!(Input::from_declaration(D::reserved(reserved)).is_none());
    }
    let recipe = FormulaRecipe::parse("let supplied:length=1 um")
        .expect("recipe syntax")
        .normalize_literals()
        .expect("literal input");
    let binding = D::recipe(&recipe, 1).expect("actual binding metadata");
    assert_eq!(binding.kind(), K::Length);
    assert!(Input::from_declaration(binding).is_none());
}
