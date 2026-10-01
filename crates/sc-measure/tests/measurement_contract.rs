//! Metadata borrows canonical values and validates current body/POM references without retargeting.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use sc_core::{
    name::MachineToken,
    ontology::EntityId,
    value::{LengthDeclaration, LengthDeclarationDefinition, LengthState, LengthValueError},
};
use sc_measure::{
    Landmark, LandmarkDefinition, Measurement, MeasurementContext, MeasurementDefinition,
    MeasurementError, MeasurementKind, MeasurementProcedure, MeasurementProcedureDefinition,
};
use sc_units::{Length, Unit};

fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn token(name: &str) -> MachineToken {
    MachineToken::new(name).unwrap()
}
fn declaration(state: LengthState) -> LengthDeclaration {
    LengthDeclaration::new(LengthDeclarationDefinition {
        id: id(10),
        source: id(90),
        state,
    })
    .unwrap()
}
fn metadata(kind: MeasurementKind) -> MeasurementDefinition {
    MeasurementDefinition {
        id: id(1),
        name: "Taille / waist".to_owned(),
        token: token("waist_girth"),
        entered_unit: Unit::Centimetre,
        kind,
        landmarks: [id(20), id(21)],
        procedure: id(30),
        declaration: id(10),
    }
}
struct Fixture {
    declarations: Vec<LengthDeclaration>,
    landmarks: Vec<Landmark>,
    procedures: Vec<MeasurementProcedure>,
}
impl Fixture {
    fn new(kind: MeasurementKind) -> Self {
        Self {
            declarations: vec![declaration(LengthState::Assumed {
                value: Length::from_micrometres(740_000).unwrap(), assumption: id(91),
            })],
            landmarks: [20, 21].into_iter().map(|n| Landmark::new(LandmarkDefinition {
                id: id(n), name: format!("Fixture landmark {n}"), kind, source: id(92),
            }).unwrap()).collect(),
            procedures: vec![MeasurementProcedure::new(MeasurementProcedureDefinition {
                id: id(30), name: "Fixture procedure".to_owned(), kind,
                documentation: "Fixture only: take the declared dimension between its named landmarks, recording the entered unit and this source assumption.".to_owned(),
                source: id(93),
            }).unwrap()],
        }
    }
    fn context(&self) -> MeasurementContext<'_> {
        MeasurementContext::new(&self.declarations, &self.landmarks, &self.procedures).unwrap()
    }
}

#[test]
fn body_and_garment_metadata_retain_all_fields_and_borrow_the_canonical_records() {
    for kind in [MeasurementKind::Body, MeasurementKind::Garment] {
        let fixture = Fixture::new(kind);
        let context = fixture.context();
        let definition = metadata(kind);
        let measurement = Measurement::new(definition.clone(), &context).unwrap();
        assert_eq!(measurement.id(), id(1));
        assert_eq!(measurement.definition(), &definition);
        assert_eq!(measurement.validate_current(&context), Ok(()));
        assert!(std::ptr::eq(
            measurement.declaration(&context).unwrap(),
            fixture.declarations.first().unwrap()
        ));
        let [a, b] = measurement.landmarks(&context).unwrap();
        assert_eq!([a.id(), b.id()], definition.landmarks);
        assert_eq!(a.definition().source, id(92));
        assert!(std::ptr::eq(
            measurement.procedure(&context).unwrap(),
            fixture.procedures.first().unwrap()
        ));
        assert_eq!(
            measurement.procedure(&context).unwrap().definition().source,
            id(93)
        );
    }
}

#[test]
fn canonical_metadata_requires_names_and_real_nonblank_procedure_documentation() {
    let fixture = Fixture::new(MeasurementKind::Body);
    for blank in ["", " \t\n", "\u{2003}"] {
        let mut landmark = fixture.landmarks.first().unwrap().definition().clone();
        landmark.name = blank.to_owned();
        assert_eq!(
            Landmark::new(landmark),
            Err(MeasurementError::EmptyName(id(20)))
        );
        let mut procedure = fixture.procedures.first().unwrap().definition().clone();
        procedure.name = blank.to_owned();
        assert_eq!(
            MeasurementProcedure::new(procedure),
            Err(MeasurementError::EmptyName(id(30)))
        );
        let mut procedure = fixture.procedures.first().unwrap().definition().clone();
        procedure.documentation = blank.to_owned();
        assert_eq!(
            MeasurementProcedure::new(procedure),
            Err(MeasurementError::EmptyProcedureDocumentation(id(30)))
        );
        let mut definition = metadata(MeasurementKind::Body);
        definition.name = blank.to_owned();
        assert_eq!(
            Measurement::new(definition, &fixture.context()),
            Err(MeasurementError::EmptyName(id(1)))
        );
    }
    let mut definition = metadata(MeasurementKind::Body);
    definition.name = "  Tour de taille — taille naturelle  ".to_owned();
    let measurement = Measurement::new(definition.clone(), &fixture.context()).unwrap();
    assert_eq!(measurement.definition().name, definition.name);
    assert_eq!(measurement.definition().token.as_str(), "waist_girth");
}

#[test]
fn missing_declaration_landmarks_or_procedure_are_refused_by_identity_without_peer_selection() {
    let fixture = Fixture::new(MeasurementKind::Body);
    let context = fixture.context();
    for missing in [id(100), id(101)] {
        let mut definition = metadata(MeasurementKind::Body);
        definition.declaration = missing;
        assert_eq!(
            Measurement::new(definition, &context),
            Err(MeasurementError::MissingDeclaration(missing))
        );
        for landmarks in [[missing, id(21)], [id(20), missing]] {
            let mut definition = metadata(MeasurementKind::Body);
            definition.landmarks = landmarks;
            assert_eq!(
                Measurement::new(definition, &context),
                Err(MeasurementError::MissingLandmark(missing))
            );
        }
        let mut definition = metadata(MeasurementKind::Body);
        definition.procedure = missing;
        assert_eq!(
            Measurement::new(definition, &context),
            Err(MeasurementError::MissingProcedure(missing))
        );
    }
}

#[test]
fn a_current_landmark_cannot_silently_change_body_measurement_into_garment_pom() {
    for (expected, actual) in [
        (MeasurementKind::Body, MeasurementKind::Garment),
        (MeasurementKind::Garment, MeasurementKind::Body),
    ] {
        let mut fixture = Fixture::new(expected);
        let measurement = Measurement::new(metadata(expected), &fixture.context()).unwrap();
        let mut changed = fixture.landmarks.first().unwrap().definition().clone();
        changed.kind = actual;
        *fixture.landmarks.first_mut().unwrap() = Landmark::new(changed).unwrap();
        let error = MeasurementError::LandmarkKindMismatch {
            measurement: id(1),
            landmark: id(20),
            expected,
            actual,
        };
        assert_eq!(
            measurement.validate_current(&fixture.context()),
            Err(error.clone())
        );
        assert_eq!(
            Measurement::new(metadata(expected), &fixture.context()),
            Err(error)
        );
        assert_eq!(measurement.definition().kind, expected);
    }
}

#[test]
fn a_current_procedure_must_match_the_original_measurement_domain() {
    for (expected, actual) in [
        (MeasurementKind::Body, MeasurementKind::Garment),
        (MeasurementKind::Garment, MeasurementKind::Body),
    ] {
        let mut fixture = Fixture::new(expected);
        let measurement = Measurement::new(metadata(expected), &fixture.context()).unwrap();
        let mut changed = fixture.procedures.first().unwrap().definition().clone();
        changed.kind = actual;
        *fixture.procedures.first_mut().unwrap() = MeasurementProcedure::new(changed).unwrap();
        let error = MeasurementError::ProcedureKindMismatch {
            measurement: id(1),
            procedure: id(30),
            expected,
            actual,
        };
        assert_eq!(
            measurement.validate_current(&fixture.context()),
            Err(error.clone())
        );
        assert_eq!(measurement.procedure(&fixture.context()), Err(error));
    }
}

#[test]
fn duplicate_within_kind_and_cross_kind_context_identities_refuse_before_lookup() {
    for duplicate_kind in 0..4 {
        let mut fixture = Fixture::new(MeasurementKind::Body);
        let duplicate = match duplicate_kind {
            0 => {
                fixture
                    .declarations
                    .push(fixture.declarations.first().unwrap().clone());
                id(10)
            }
            1 => {
                fixture
                    .landmarks
                    .push(fixture.landmarks.first().unwrap().clone());
                id(20)
            }
            2 => {
                fixture
                    .procedures
                    .push(fixture.procedures.first().unwrap().clone());
                id(30)
            }
            _ => {
                let mut changed = fixture.landmarks.first().unwrap().definition().clone();
                changed.id = id(10);
                *fixture.landmarks.first_mut().unwrap() = Landmark::new(changed).unwrap();
                id(10)
            }
        };
        assert_eq!(
            MeasurementContext::new(
                &fixture.declarations,
                &fixture.landmarks,
                &fixture.procedures
            )
            .unwrap_err(),
            MeasurementError::DuplicateIdentity(duplicate)
        );
    }
}

#[test]
fn measurement_identity_cannot_reuse_a_canonical_target_identity() {
    let fixture = Fixture::new(MeasurementKind::Body);
    for target in [id(10), id(20), id(30)] {
        let mut definition = metadata(MeasurementKind::Body);
        definition.id = target;
        assert_eq!(
            Measurement::new(definition, &fixture.context()),
            Err(MeasurementError::DuplicateIdentity(target))
        );
    }
}

#[test]
fn metadata_cannot_rebind_any_reserved_formula_input() {
    let fixture = Fixture::new(MeasurementKind::Body);
    for name in [
        "eps_num",
        "eps_geo",
        "eps_fmt",
        "eps_imp",
        "eps_phys",
        "size_index",
        "size_count",
        "is_base_size",
    ] {
        let mut definition = metadata(MeasurementKind::Body);
        definition.token = token(name);
        assert_eq!(
            Measurement::new(definition, &fixture.context()),
            Err(MeasurementError::ReservedInputToken {
                measurement: id(1),
                token: token(name)
            })
        );
    }
    let mut definition = metadata(MeasurementKind::Body);
    definition.token = token("eps_geo_input");
    assert_eq!(
        Measurement::new(definition, &fixture.context())
            .unwrap()
            .definition()
            .token
            .as_str(),
        "eps_geo_input"
    );
}

#[test]
fn all_authored_states_are_borrowed_without_numeric_or_source_duplication() {
    let value = Length::from_micrometres(740_000).unwrap();
    for state in [
        LengthState::Known {
            value,
            evidence: vec![id(91)],
        },
        LengthState::Assumed {
            value,
            assumption: id(91),
        },
        LengthState::Unknown {
            observation: id(91),
        },
        LengthState::Preference {
            value,
            provenance: id(91),
        },
        LengthState::Derived { formula: id(91) },
    ] {
        let mut fixture = Fixture::new(MeasurementKind::Body);
        fixture.declarations = vec![declaration(state.clone())];
        let context = fixture.context();
        let measurement = Measurement::new(metadata(MeasurementKind::Body), &context).unwrap();
        let borrowed = measurement.declaration(&context).unwrap();
        assert_eq!(borrowed.state(), &state);
        assert_eq!(borrowed.source(), id(90));
        assert!(std::ptr::eq(
            borrowed,
            fixture.declarations.first().unwrap()
        ));
        match state {
            LengthState::Unknown { observation } => assert_eq!(
                borrowed.authored_value(),
                Err(LengthValueError::RequiresObservation {
                    declaration: id(10),
                    observation
                })
            ),
            LengthState::Derived { formula } => assert_eq!(
                borrowed.authored_value(),
                Err(LengthValueError::RequiresEvaluation {
                    declaration: id(10),
                    formula
                })
            ),
            _ => assert_eq!(borrowed.authored_value(), Ok(value)),
        }
    }
}

#[test]
fn entered_unit_metadata_never_reconverts_or_defaults_the_canonical_length() {
    let mut fixture = Fixture::new(MeasurementKind::Body);
    let value = Length::from_rational(3, 4, Unit::Inch).unwrap();
    fixture.declarations = vec![declaration(LengthState::Assumed {
        value,
        assumption: id(91),
    })];
    for entered_unit in [
        Unit::Micrometre,
        Unit::Millimetre,
        Unit::Centimetre,
        Unit::Metre,
        Unit::Inch,
        Unit::HpglPlotterUnit,
        Unit::PdfPoint,
    ] {
        let mut definition = metadata(MeasurementKind::Body);
        definition.entered_unit = entered_unit;
        let context = fixture.context();
        let measurement = Measurement::new(definition, &context).unwrap();
        assert_eq!(measurement.definition().entered_unit, entered_unit);
        assert_eq!(
            measurement
                .declaration(&context)
                .unwrap()
                .authored_value()
                .unwrap()
                .as_micrometres(),
            19_050
        );
    }
}

#[test]
fn current_same_id_declaration_replacement_exposes_its_new_state_and_source() {
    let mut fixture = Fixture::new(MeasurementKind::Body);
    let measurement =
        Measurement::new(metadata(MeasurementKind::Body), &fixture.context()).unwrap();
    let old = fixture.declarations.first().unwrap().clone();
    let mut replacement = old.definition().clone();
    replacement.source = id(95);
    replacement.state = LengthState::Unknown {
        observation: id(96),
    };
    fixture.declarations = vec![LengthDeclaration::new(replacement).unwrap()];
    let context = fixture.context();
    assert_eq!(measurement.validate_current(&context), Ok(()));
    assert_eq!(measurement.declaration(&context).unwrap().source(), id(95));
    assert_eq!(
        measurement.declaration(&context).unwrap().authored_value(),
        Err(LengthValueError::RequiresObservation {
            declaration: id(10),
            observation: id(96)
        })
    );
    assert_eq!(old.source(), id(90));
    assert_eq!(old.authored_value().unwrap().as_micrometres(), 740_000);
}

#[test]
fn removing_each_current_target_leaves_old_metadata_inspectable_and_refuses_the_missing_id() {
    for removed_kind in 0..3 {
        let mut fixture = Fixture::new(MeasurementKind::Body);
        let measurement =
            Measurement::new(metadata(MeasurementKind::Body), &fixture.context()).unwrap();
        let expected = match removed_kind {
            0 => {
                fixture.declarations.clear();
                MeasurementError::MissingDeclaration(id(10))
            }
            1 => {
                fixture.landmarks.remove(0);
                MeasurementError::MissingLandmark(id(20))
            }
            _ => {
                fixture.procedures.clear();
                MeasurementError::MissingProcedure(id(30))
            }
        };
        assert_eq!(
            measurement.validate_current(&fixture.context()),
            Err(expected.clone())
        );
        assert_eq!(measurement.definition(), &metadata(MeasurementKind::Body));
        assert!(expected.to_string().contains(
            &match removed_kind {
                0 => id(10),
                1 => id(20),
                _ => id(30),
            }
            .to_string()
        ));
    }
}

#[test]
fn target_queries_validate_only_their_named_target_without_certifying_all_current_metadata() {
    let mut fixture = Fixture::new(MeasurementKind::Body);
    let measurement =
        Measurement::new(metadata(MeasurementKind::Body), &fixture.context()).unwrap();
    fixture.landmarks.clear();
    let context = fixture.context();
    assert!(measurement.declaration(&context).is_ok());
    assert!(measurement.procedure(&context).is_ok());
    assert_eq!(
        measurement.validate_current(&context),
        Err(MeasurementError::MissingLandmark(id(20)))
    );
    assert_eq!(measurement.definition().landmarks, [id(20), id(21)]);
}

#[test]
fn validated_replacement_preserves_original_metadata_and_shared_canonical_value_identity() {
    let fixture = Fixture::new(MeasurementKind::Body);
    let context = fixture.context();
    let original = Measurement::new(metadata(MeasurementKind::Body), &context).unwrap();
    let mut replacement = original.definition().clone();
    replacement.id = id(2);
    replacement.name = "Alternate display label".to_owned();
    replacement.token = token("waist_girth_alternative");
    replacement.entered_unit = Unit::Inch;
    replacement.landmarks = [id(21), id(20)];
    let new = Measurement::new(replacement.clone(), &context).unwrap();
    assert_eq!(original.definition(), &metadata(MeasurementKind::Body));
    assert_eq!(new.definition(), &replacement);
    assert!(std::ptr::eq(
        original.declaration(&context).unwrap(),
        new.declaration(&context).unwrap()
    ));
}

#[test]
fn ordered_or_repeated_girth_landmarks_retain_authored_reference_intent() {
    let fixture = Fixture::new(MeasurementKind::Body);
    for landmarks in [[id(21), id(20)], [id(20), id(20)]] {
        let mut definition = metadata(MeasurementKind::Body);
        definition.landmarks = landmarks;
        let context = fixture.context();
        let measurement = Measurement::new(definition, &context).unwrap();
        let [a, b] = measurement.landmarks(&context).unwrap();
        assert_eq!([a.id(), b.id()], landmarks);
    }
}

#[test]
fn current_procedure_documentation_and_landmark_names_are_read_from_the_canonical_records() {
    let mut fixture = Fixture::new(MeasurementKind::Body);
    let measurement =
        Measurement::new(metadata(MeasurementKind::Body), &fixture.context()).unwrap();
    let old_procedure = fixture.procedures.first().unwrap().clone();
    let mut replacement = old_procedure.definition().clone();
    replacement.documentation =
        "Replacement fixture-only instructions, pending physical review.".to_owned();
    replacement.source = id(94);
    *fixture.procedures.first_mut().unwrap() =
        MeasurementProcedure::new(replacement.clone()).unwrap();
    let mut landmark = fixture.landmarks.first().unwrap().definition().clone();
    landmark.name = "Revised localized landmark name".to_owned();
    *fixture.landmarks.first_mut().unwrap() = Landmark::new(landmark.clone()).unwrap();
    let context = fixture.context();
    assert_eq!(measurement.validate_current(&context), Ok(()));
    assert_eq!(
        measurement.procedure(&context).unwrap().definition(),
        &replacement
    );
    let [current, _] = measurement.landmarks(&context).unwrap();
    assert_eq!(current.definition(), &landmark);
    assert_ne!(
        old_procedure.definition().documentation,
        replacement.documentation
    );
}
