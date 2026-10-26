//! Focused transformation intent contracts use the actual garment composite fixture.
use super::*;
use sc_core::value::ValueProvenanceValidation;
use sc_units::Count;
fn definition(state: SizeSetTransformationState) -> SizeSetTransformationDefinition {
    SizeSetTransformationDefinition {
        id: id(2000),
        reference: SizeSetReference {
            id: id(1),
            revision: Count::new(3),
        },
        source: id(3000),
        kind: SizeSetTransformationKind::LabelMapping,
        state,
    }
}
fn mapping(entries: &[(u128, &str)]) -> SizeSetTransformationValue {
    SizeSetTransformationValue::LabelMapping(
        entries
            .iter()
            .map(|(member, text)| SizeLabelMapping {
                member: id(*member),
                label: SizeLabel::new((*text).to_owned()).expect("authored label"),
            })
            .collect(),
    )
}
fn assumed(value: SizeSetTransformationValue) -> SizeSetTransformationState {
    SizeSetTransformationState::Assumed {
        value,
        assumption: id(3001),
    }
}
#[test]
fn exact_partial_and_simultaneous_label_mappings_preserve_state_order_and_source() {
    let f = Fixture::new(MeasurementKind::Garment);
    let before = f.membership.clone();
    with_parts(
        &f,
        Options::default(),
        |chart, chart_context, breaks, break_context| {
            let inputs = GarmentSizeSetInputs::new(chart, chart_context, breaks, break_context)
                .expect("actual inputs")
                .into();
            for value in [
                mapping(&[(3, " Atelier Étoile ")]),
                mapping(&[(4, "S"), (3, "M")]),
            ] {
                for state in [
                    SizeSetTransformationState::Known {
                        value: value.clone(),
                        evidence: vec![id(4001), id(4000)],
                    },
                    assumed(value.clone()),
                    SizeSetTransformationState::Preference {
                        value: value.clone(),
                        provenance: id(4002),
                    },
                ] {
                    let authored = definition(state);
                    let result = SizeSetTransformation::new(authored.clone(), inputs)
                        .expect("explicit mapping");
                    assert_eq!(result.definition(), &authored);
                    assert_eq!(result.authored_value(), Ok(&value));
                    assert_eq!(result.validate_current(inputs), Ok(()));
                    assert_eq!(
                        result.provenance_validation(),
                        ValueProvenanceValidation::DeferredToDesignAndG4
                    );
                }
            }
        },
    );
    assert_eq!(f.membership, before);
}
#[test]
fn unknown_and_derived_have_only_the_original_request_or_formula_without_fallback() {
    let f = Fixture::new(MeasurementKind::Garment);
    with_parts(
        &f,
        Options::default(),
        |chart, chart_context, breaks, break_context| {
            let inputs = GarmentSizeSetInputs::new(chart, chart_context, breaks, break_context)
                .expect("actual inputs")
                .into();
            for kind in [
                SizeSetTransformationKind::LabelMapping,
                SizeSetTransformationKind::BaseSubstitution,
                SizeSetTransformationKind::BreakScaling,
            ] {
                for (state, refusal) in [
                    (
                        SizeSetTransformationState::Unknown {
                            observation: id(4000),
                        },
                        SizeSetTransformationReadError::RequiresObservation {
                            transformation: id(2000),
                            observation: id(4000),
                        },
                    ),
                    (
                        SizeSetTransformationState::Derived { formula: id(4001) },
                        SizeSetTransformationReadError::RequiresEvaluation {
                            transformation: id(2000),
                            formula: id(4001),
                        },
                    ),
                ] {
                    let mut authored = definition(state);
                    authored.kind = kind;
                    let result = SizeSetTransformation::new(authored.clone(), inputs)
                        .expect("value-less intent");
                    assert_eq!(result.definition(), &authored);
                    assert_eq!(result.authored_value(), Err(refusal));
                }
            }
        },
    );
}
#[test]
fn malformed_evidence_and_stale_or_aliased_declarations_refuse_before_payload() {
    let f = Fixture::new(MeasurementKind::Garment);
    with_parts(
        &f,
        Options::default(),
        |chart, chart_context, breaks, break_context| {
            let inputs = GarmentSizeSetInputs::new(chart, chart_context, breaks, break_context)
                .expect("actual inputs")
                .into();
            for (evidence, error) in [
                (vec![], SizeSetTransformationError::NoEvidence(id(2000))),
                (
                    vec![id(4000), id(4000)],
                    SizeSetTransformationError::DuplicateEvidence(id(4000)),
                ),
            ] {
                assert_eq!(
                    SizeSetTransformation::new(
                        definition(SizeSetTransformationState::Known {
                            value: mapping(&[]),
                            evidence,
                        }),
                        inputs
                    ),
                    Err(error)
                );
            }
            let mut stale = definition(assumed(mapping(&[(3, "Small")])));
            stale.reference.revision = Count::new(2);
            assert_eq!(
                SizeSetTransformation::new(stale.clone(), inputs),
                Err(SizeSetTransformationError::SourceMismatch {
                    expected: stale.reference,
                    actual: f.membership.reference(),
                })
            );
            for identity in [1, 2, 10, 11, 100, 200, 300, 400, 500, 600, 800, 3000] {
                let mut alias = definition(assumed(mapping(&[(3, "Small")])));
                alias.id = id(identity);
                assert_eq!(
                    SizeSetTransformation::new(alias, inputs),
                    Err(SizeSetTransformationError::DuplicateIdentity(id(identity)))
                );
            }
        },
    );
}
#[test]
fn labels_refuse_empty_duplicate_missing_and_effectively_colliding_members() {
    let f = Fixture::new(MeasurementKind::Garment);
    with_parts(
        &f,
        Options::default(),
        |chart, chart_context, breaks, break_context| {
            let inputs = GarmentSizeSetInputs::new(chart, chart_context, breaks, break_context)
                .expect("actual inputs")
                .into();
            for (value, error) in [
                (mapping(&[]), SizeSetTransformationError::EmptyMapping),
                (
                    mapping(&[(3, "Small"), (3, "Second")]),
                    SizeSetTransformationError::DuplicateMember(id(3)),
                ),
                (
                    mapping(&[(99, "Small")]),
                    SizeSetTransformationError::MissingMember(id(99)),
                ),
                (
                    mapping(&[(3, "M")]),
                    SizeSetTransformationError::DuplicateLabel {
                        first: id(3),
                        second: id(4),
                        label: SizeLabel::new("M".to_owned()).expect("label"),
                    },
                ),
                (
                    mapping(&[(4, "Same"), (2, "Same")]),
                    SizeSetTransformationError::DuplicateLabel {
                        first: id(2),
                        second: id(4),
                        label: SizeLabel::new("Same".to_owned()).expect("label"),
                    },
                ),
            ] {
                assert_eq!(
                    SizeSetTransformation::new(definition(assumed(value)), inputs),
                    Err(error)
                );
            }
        },
    );
}
#[test]
fn base_is_an_explicit_existing_member_and_domain_never_comes_from_the_payload() {
    let f = Fixture::new(MeasurementKind::Garment);
    let before = f.membership.clone();
    with_parts(
        &f,
        Options::default(),
        |chart, chart_context, breaks, break_context| {
            let inputs = GarmentSizeSetInputs::new(chart, chart_context, breaks, break_context)
                .expect("actual inputs")
                .into();
            let mut authored =
                definition(assumed(SizeSetTransformationValue::BaseSubstitution(id(4))));
            assert_eq!(
                SizeSetTransformation::new(authored.clone(), inputs),
                Err(SizeSetTransformationError::KindMismatch {
                    expected: SizeSetTransformationKind::LabelMapping,
                    actual: SizeSetTransformationKind::BaseSubstitution,
                })
            );
            authored.kind = SizeSetTransformationKind::BaseSubstitution;
            let result =
                SizeSetTransformation::new(authored.clone(), inputs).expect("new base intent");
            assert_eq!(result.definition(), &authored);
            assert_eq!(
                result.authored_value(),
                Ok(&SizeSetTransformationValue::BaseSubstitution(id(4)))
            );
            authored.state = assumed(SizeSetTransformationValue::BaseSubstitution(id(99)));
            assert_eq!(
                SizeSetTransformation::new(authored, inputs),
                Err(SizeSetTransformationError::MissingMember(id(99)))
            );
        },
    );
    assert_eq!(f.membership, before);
}

fn scalar_factor(
    kind: sc_core::value::ScalarInputKind,
    state: sc_core::value::ScalarInputState,
) -> sc_core::value::ScalarInputDeclaration {
    sc_core::value::ScalarInputDeclaration::new(sc_core::value::ScalarInputDeclarationDefinition {
        id: id(160),
        source: id(1160),
        kind,
        state,
    })
    .expect("canonical factor fixture")
}
#[test]
fn break_scaling_borrows_original_signed_delta_and_all_factor_states_without_multiplication() {
    use sc_core::value::{
        ScalarInputKind as Kind, ScalarInputReadError as ReadError, ScalarInputState as State,
        ScalarInputValue as Value,
    };
    let cases = [
        (
            State::Known {
                value: Value::from_ratio_ppm(0),
                evidence: vec![id(4100)],
            },
            LengthState::Known {
                value: support::length(0),
                evidence: vec![id(4101)],
            },
        ),
        (
            State::Known {
                value: Value::from_ratio_ppm(-1_500_000),
                evidence: vec![id(4100)],
            },
            LengthState::Preference {
                value: support::length(-10_000),
                provenance: id(4101),
            },
        ),
        (
            State::Assumed {
                value: Value::from_ratio_ppm(1_100_000),
                assumption: id(4100),
            },
            LengthState::Assumed {
                value: support::length(40_000),
                assumption: id(4101),
            },
        ),
        (
            State::Preference {
                value: Value::from_ratio_ppm(2_000_000),
                provenance: id(4100),
            },
            LengthState::Unknown {
                observation: id(4101),
            },
        ),
        (
            State::Unknown {
                observation: id(4100),
            },
            LengthState::Derived { formula: id(4101) },
        ),
        (
            State::Derived { formula: id(4100) },
            LengthState::Unknown {
                observation: id(4101),
            },
        ),
    ];
    for (state, delta_state) in cases {
        let mut f = Fixture::new(MeasurementKind::Garment);
        f.scalars.push(scalar_factor(Kind::Ratio, state.clone()));
        f.lengths[0] = declaration(100, delta_state.clone());
        with_parts(
            &f,
            Options {
                complete_breaks: true,
                ..Options::default()
            },
            |chart, chart_context, breaks, break_context| {
                let inputs = GarmentSizeSetInputs::new(chart, chart_context, breaks, break_context)
                    .expect("actual inputs")
                    .into();
                let binding = SizeBreakScaling {
                    size_break: id(700),
                    factor: id(160),
                };
                let mut authored =
                    definition(assumed(SizeSetTransformationValue::BreakScaling(binding)));
                authored.kind = SizeSetTransformationKind::BreakScaling;
                let intent = SizeSetTransformation::new(authored.clone(), inputs)
                    .expect("canonical scaling intent");
                assert_eq!(intent.definition(), &authored);
                let actual = binding.inputs(inputs).expect("actual borrowed inputs");
                assert!(std::ptr::eq(
                    actual.size_break,
                    &breaks.definition().breaks[0]
                ));
                assert!(std::ptr::eq(actual.delta, &f.lengths[0]));
                assert!(std::ptr::eq(actual.factor, &f.scalars[0]));
                assert_eq!(actual.factor.definition().state, state);
                assert_eq!(actual.delta.definition().state, delta_state);
                match state {
                    State::Unknown { observation } => assert_eq!(
                        actual.factor.authored_value(),
                        Err(ReadError::RequiresObservation {
                            declaration: id(160),
                            observation
                        })
                    ),
                    State::Derived { formula } => assert_eq!(
                        actual.factor.authored_value(),
                        Err(ReadError::RequiresEvaluation {
                            declaration: id(160),
                            formula
                        })
                    ),
                    State::Known { value, .. }
                    | State::Assumed { value, .. }
                    | State::Preference { value, .. } => {
                        assert_eq!(actual.factor.authored_value(), Ok(value))
                    }
                }
            },
        );
    }
}
#[test]
fn scaling_requires_available_break_and_actual_general_ratio_role_and_kind() {
    use sc_core::value::{ScalarInputKind as Kind, ScalarInputState as State};
    for kind in [
        Kind::Length,
        Kind::Angle,
        Kind::Ratio,
        Kind::Count,
        Kind::Boolean,
        Kind::Area,
    ] {
        let mut f = Fixture::new(MeasurementKind::Garment);
        let state = if kind == Kind::Area {
            State::Derived { formula: id(4100) }
        } else {
            State::Unknown {
                observation: id(4100),
            }
        };
        f.scalars.push(scalar_factor(kind, state));
        with_parts(
            &f,
            Options {
                complete_breaks: true,
                ..Options::default()
            },
            |chart, chart_context, breaks, break_context| {
                let inputs = GarmentSizeSetInputs::new(chart, chart_context, breaks, break_context)
                    .expect("actual inputs")
                    .into();
                let binding = SizeBreakScaling {
                    size_break: id(700),
                    factor: id(160),
                };
                if kind == Kind::Ratio {
                    assert!(binding.inputs(inputs).is_ok());
                } else {
                    assert_eq!(
                        binding.inputs(inputs),
                        Err(SizeSetTransformationError::FactorKind {
                            factor: id(160),
                            actual: kind
                        })
                    );
                }
                for (candidate, error) in [
                    (
                        SizeBreakScaling {
                            size_break: id(9999),
                            ..binding
                        },
                        SizeSetTransformationError::MissingBreak(id(9999)),
                    ),
                    (
                        SizeBreakScaling {
                            factor: id(9999),
                            ..binding
                        },
                        SizeSetTransformationError::MissingFactor(id(9999)),
                    ),
                    (
                        SizeBreakScaling {
                            factor: id(100),
                            ..binding
                        },
                        SizeSetTransformationError::FactorRole(id(100)),
                    ),
                    (
                        SizeBreakScaling {
                            factor: id(600),
                            ..binding
                        },
                        SizeSetTransformationError::FactorRole(id(600)),
                    ),
                ] {
                    let mut authored =
                        definition(assumed(SizeSetTransformationValue::BreakScaling(candidate)));
                    authored.kind = SizeSetTransformationKind::BreakScaling;
                    assert_eq!(SizeSetTransformation::new(authored, inputs), Err(error));
                }
            },
        );
    }
}
