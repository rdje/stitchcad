//! Explicit authored axes retain canonical content and reject ambiguous current sources.
#![allow(clippy::expect_used, clippy::indexing_slicing, clippy::panic)]
use sc_core::{
    ontology::EntityId,
    value::{
        LengthDeclaration, LengthDeclarationDefinition, LengthState, ScalarInputDeclaration,
        ScalarInputDeclarationDefinition, ScalarInputKind, ScalarInputReadError, ScalarInputState,
        ScalarInputValue,
    },
};
use sc_measure::{
    SizeAxis, SizeAxisDefinition, SizeAxisError, SizeAxisInputRole, SizeAxisKind,
    SizeAxisSourceContext, SizeAxisValueContent, SizeAxisValueDefinition, SizeAxisValueSource,
    SizeLabel, SizeSetReference,
};
use sc_units::{Count, Length};

fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn label(s: &str) -> SizeLabel {
    SizeLabel::new(s.to_owned()).expect("nonblank fixture label")
}
fn definition() -> SizeAxisDefinition {
    SizeAxisDefinition {
        reference: SizeSetReference {
            id: id(100),
            revision: Count::new(3),
        },
        id: id(200),
        name: " Body sizes 尺碼 ".to_owned(),
        kind: SizeAxisKind::Label,
        values: vec![
            SizeAxisValueDefinition {
                id: id(300),
                source: SizeAxisValueSource::Label(label("2XL")),
            },
            SizeAxisValueDefinition {
                id: id(301),
                source: SizeAxisValueSource::Label(label(" S ")),
            },
        ],
    }
}
fn scalar(kind: ScalarInputKind, state: ScalarInputState) -> ScalarInputDeclaration {
    ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
        id: id(501),
        source: id(601),
        kind,
        state,
    })
    .expect("canonical fixture scalar")
}
fn numeric_definition(kind: ScalarInputKind) -> SizeAxisDefinition {
    let mut d = definition();
    d.kind = SizeAxisKind::Scalar(kind);
    d.values = vec![SizeAxisValueDefinition {
        id: id(300),
        source: SizeAxisValueSource::Scalar(id(501)),
    }];
    d
}

#[test]
fn exact_names_labels_and_authored_order_are_not_inferred_or_sorted() {
    let context = SizeAxisSourceContext::new(&[], &[]).expect("empty canonical inventory");
    let d = definition();
    let axis = SizeAxis::new(d.clone(), &context).expect("authored labelled axis");
    assert_eq!(axis.definition(), &d);
    let first = axis.value(id(300), &context).expect("first coordinate");
    assert_eq!(first.axis(), id(200));
    assert!(std::ptr::eq(
        first.definition(),
        &axis.definition().values[0]
    ));
    assert_eq!(first.content(), SizeAxisValueContent::Label(&label("2XL")));
    assert_eq!(
        axis.value(id(301), &context)
            .expect("second coordinate")
            .content(),
        SizeAxisValueContent::Label(&label(" S "))
    );
    assert_eq!(
        axis.value(id(999), &context).err(),
        Some(SizeAxisError::MissingValue {
            axis: id(200),
            value: id(999)
        })
    );
}

#[test]
fn dedicated_and_general_lengths_borrow_the_actual_records_without_reading_unknowns() {
    let length = LengthDeclaration::new(LengthDeclarationDefinition {
        id: id(500),
        source: id(600),
        state: LengthState::Known {
            value: Length::from_micrometres(1_700_000).expect("length"),
            evidence: vec![id(700)],
        },
    })
    .expect("canonical length");
    let unknown = scalar(
        ScalarInputKind::Length,
        ScalarInputState::Unknown {
            observation: id(701),
        },
    );
    let context = SizeAxisSourceContext::new(&[&length], &[&unknown]).expect("actual records");
    let mut d = numeric_definition(ScalarInputKind::Length);
    d.values.insert(
        0,
        SizeAxisValueDefinition {
            id: id(301),
            source: SizeAxisValueSource::Length(id(500)),
        },
    );
    let axis = SizeAxis::new(d, &context).expect("one quantity kind, explicit source roles");
    let SizeAxisValueContent::Length(actual) = axis
        .value(id(301), &context)
        .expect("length coordinate")
        .content()
    else {
        panic!("wrong canonical role")
    };
    assert!(std::ptr::eq(actual, &length));
    assert_eq!(actual.source(), id(600));
    assert_eq!(
        actual
            .authored_value()
            .expect("known value")
            .as_micrometres(),
        1_700_000
    );
    let SizeAxisValueContent::Scalar(actual) = axis
        .value(id(300), &context)
        .expect("unknown coordinate")
        .content()
    else {
        panic!("wrong canonical role")
    };
    assert!(std::ptr::eq(actual, &unknown));
    assert_eq!(actual.source(), id(601));
    assert_eq!(
        actual.authored_value(),
        Err(ScalarInputReadError::RequiresObservation {
            declaration: id(501),
            observation: id(701)
        })
    );
}

#[test]
fn all_scalar_kinds_keep_their_original_state_and_false_is_present() {
    for kind in ScalarInputKind::ALL {
        let state = if kind == ScalarInputKind::Area {
            ScalarInputState::Derived { formula: id(702) }
        } else {
            ScalarInputState::Unknown {
                observation: id(701),
            }
        };
        let record = scalar(kind, state.clone());
        let context = SizeAxisSourceContext::new(&[], &[&record]).expect("actual scalar");
        let axis = SizeAxis::new(numeric_definition(kind), &context).expect("kind-preserving axis");
        let view = axis.value(id(300), &context).expect("coordinate");
        assert_eq!(view.kind(), SizeAxisKind::Scalar(kind));
        let SizeAxisValueContent::Scalar(actual) = view.content() else {
            panic!("wrong role")
        };
        assert!(std::ptr::eq(actual, &record));
        assert_eq!(actual.state(), &state);
        assert!(actual.authored_value().is_err());
    }
    for state in [
        ScalarInputState::Known {
            value: ScalarInputValue::from_boolean(false),
            evidence: vec![id(700)],
        },
        ScalarInputState::Assumed {
            value: ScalarInputValue::from_boolean(false),
            assumption: id(703),
        },
        ScalarInputState::Preference {
            value: ScalarInputValue::from_boolean(false),
            provenance: id(704),
        },
    ] {
        let record = scalar(ScalarInputKind::Boolean, state.clone());
        let context = SizeAxisSourceContext::new(&[], &[&record]).expect("actual scalar");
        let axis = SizeAxis::new(numeric_definition(ScalarInputKind::Boolean), &context)
            .expect("boolean axis");
        let SizeAxisValueContent::Scalar(actual) =
            axis.value(id(300), &context).expect("coordinate").content()
        else {
            panic!("wrong role")
        };
        assert_eq!(actual.state(), &state);
        assert_eq!(
            actual.authored_value().expect("authored false").boolean(),
            Some(false)
        );
    }
}

#[test]
fn ambiguous_names_identities_and_labels_refuse_without_defaults() {
    let context = SizeAxisSourceContext::new(&[], &[]).expect("empty inventory");
    let mut d = definition();
    d.name = "\u{2003}\t".to_owned();
    assert_eq!(
        SizeAxis::new(d.clone(), &context).err(),
        Some(SizeAxisError::BlankName {
            axis: id(200),
            name: d.name
        })
    );
    let mut d = definition();
    d.values.clear();
    assert_eq!(
        SizeAxis::new(d, &context).err(),
        Some(SizeAxisError::EmptyValues(id(200)))
    );
    for identity in [id(100), id(200), id(300)] {
        let mut d = definition();
        d.values[1].id = identity;
        assert_eq!(
            SizeAxis::new(d, &context).err(),
            Some(SizeAxisError::DuplicateIdentity {
                axis: id(200),
                identity
            })
        );
    }
    let mut d = definition();
    d.id = id(100);
    assert_eq!(
        SizeAxis::new(d, &context).err(),
        Some(SizeAxisError::DuplicateIdentity {
            axis: id(100),
            identity: id(100)
        })
    );
    let mut d = definition();
    d.values[1].source = SizeAxisValueSource::Label(label("2XL"));
    assert_eq!(
        SizeAxis::new(d, &context).err(),
        Some(SizeAxisError::DuplicateLabel {
            axis: id(200),
            label: label("2XL")
        })
    );
}

#[test]
fn current_inventory_must_keep_source_role_kind_and_distinct_metadata_identity() {
    let record = scalar(
        ScalarInputKind::Length,
        ScalarInputState::Unknown {
            observation: id(701),
        },
    );
    let context = SizeAxisSourceContext::new(&[], &[&record]).expect("actual scalar");
    assert_eq!(
        SizeAxisSourceContext::new(&[], &[&record, &record]).err(),
        Some(SizeAxisError::DuplicateSource(id(501)))
    );
    let length = LengthDeclaration::new(LengthDeclarationDefinition {
        id: id(501),
        source: id(600),
        state: LengthState::Derived { formula: id(702) },
    })
    .expect("length alias fixture");
    assert_eq!(
        SizeAxisSourceContext::new(&[&length], &[&record]).err(),
        Some(SizeAxisError::DuplicateSource(id(501)))
    );
    let empty = SizeAxisSourceContext::new(&[], &[]).expect("empty inventory");
    let d = numeric_definition(ScalarInputKind::Length);
    assert_eq!(
        SizeAxis::new(d.clone(), &empty).err(),
        Some(SizeAxisError::MissingSource {
            axis: id(200),
            value: id(300),
            declaration: id(501),
            expected: SizeAxisInputRole::Scalar
        })
    );
    let mut wrong_role = d.clone();
    wrong_role.values[0].source = SizeAxisValueSource::Length(id(501));
    assert_eq!(
        SizeAxis::new(wrong_role, &context).err(),
        Some(SizeAxisError::SourceRole {
            axis: id(200),
            value: id(300),
            declaration: id(501),
            expected: SizeAxisInputRole::Length,
            actual: SizeAxisInputRole::Scalar
        })
    );
    for position in 0..3 {
        let mut alias = d.clone();
        match position {
            0 => alias.reference.id = id(501),
            1 => alias.id = id(501),
            _ => alias.values[0].id = id(501),
        }
        assert_eq!(
            SizeAxis::new(alias.clone(), &context).err(),
            Some(SizeAxisError::DuplicateIdentity {
                axis: alias.id,
                identity: id(501)
            })
        );
    }
    let axis = SizeAxis::new(d, &context).expect("original sources valid");
    let changed = scalar(
        ScalarInputKind::Ratio,
        ScalarInputState::Derived { formula: id(702) },
    );
    let changed_context =
        SizeAxisSourceContext::new(&[], &[&changed]).expect("replacement inventory");
    let refusal = SizeAxisError::Kind {
        axis: id(200),
        value: id(300),
        wanted: SizeAxisKind::Scalar(ScalarInputKind::Length),
        actual: SizeAxisKind::Scalar(ScalarInputKind::Ratio),
    };
    assert_eq!(
        axis.validate_current(&changed_context),
        Err(refusal.clone())
    );
    assert_eq!(axis.value(id(300), &changed_context).err(), Some(refusal));
    assert!(axis.validate_current(&context).is_ok());
    assert!(matches!(
        axis.value(id(300), &empty),
        Err(SizeAxisError::MissingSource { .. })
    ));
}
