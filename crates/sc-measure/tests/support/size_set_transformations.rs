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
