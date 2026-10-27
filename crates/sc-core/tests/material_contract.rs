//! Material metadata preserves canonical property state, explicit stage intent and refusal scope.
#![allow(clippy::expect_used)]
use sc_core::{
    ontology::{
        EntityId, Material, MaterialDefinition as Definition, MaterialError as Error,
        MaterialProperty as Property, MaterialPropertyDeclaration as Declaration,
        MaterialShrinkage, MaterialShrinkageStage as Stage, MaterialTextField as Field,
    },
    value::{
        CanonicalInputTarget as Target, CanonicalInputs, LengthDeclaration,
        LengthDeclarationDefinition, LengthState, ScalarInputDeclaration,
        ScalarInputDeclarationDefinition, ScalarInputKind as Kind, ScalarInputState as State,
        ScalarInputValue as Value, ValueProvenanceValidation,
    },
};
use sc_units::Length;

fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn length(value: i64) -> Length {
    Length::from_micrometres(value).expect("fixture length domain")
}
fn scalar(identity: u128, kind: Kind, state: State) -> ScalarInputDeclaration {
    ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
        id: id(identity),
        source: id(900),
        kind,
        state,
    })
    .expect("typed fixture scalar")
}
fn canonical(width: LengthState) -> CanonicalInputs {
    let lengths = vec![LengthDeclaration::new(LengthDeclarationDefinition {
        id: id(1),
        source: id(901),
        state: width,
    })
    .expect("explicit fixture width state")];
    let scalars = (2..=8)
        .map(|identity| {
            scalar(
                identity,
                match identity {
                    6 | 7 => Kind::Ratio,
                    8 => Kind::Length,
                    _ => Kind::Boolean,
                },
                State::Unknown {
                    observation: id(902),
                },
            )
        })
        .collect();
    CanonicalInputs::new(lengths, scalars).expect("unique canonical fixture properties")
}
fn unknown() -> CanonicalInputs {
    canonical(LengthState::Unknown {
        observation: id(903),
    })
}
fn target(identity: u128, kind: Kind) -> Target {
    Target::Scalar {
        declaration: id(identity),
        kind,
    }
}
fn definition() -> Definition {
    Definition {
        id: id(100),
        source: id(910),
        name: "  private_material_fixture  ".into(),
        composition: Some(" caller-authored composition claim ".into()),
        width: Target::Length(id(1)),
        nap: target(2, Kind::Boolean),
        directional_print: target(3, Kind::Boolean),
        striped: target(4, Kind::Boolean),
        checked: target(5, Kind::Boolean),
        shrinkage: Some(MaterialShrinkage {
            x: target(6, Kind::Ratio),
            y: target(7, Kind::Ratio),
            treatment: id(911),
            stage: Stage::PostGradePreExport,
        }),
    }
}
fn replace(inputs: &CanonicalInputs, identity: u128, kind: Kind, state: State) -> CanonicalInputs {
    let scalars = inputs
        .scalars()
        .iter()
        .map(|value| {
            if value.id() == id(identity) {
                scalar(identity, kind, state.clone())
            } else {
                value.clone()
            }
        })
        .collect();
    CanonicalInputs::new(inputs.lengths().to_vec(), scalars).expect("same-id current replacement")
}

#[test]
fn authored_metadata_and_every_property_keep_exact_canonical_borrows_without_defaults() {
    let canonical = unknown();
    let original = definition();
    let material = Material::new(original.clone(), &canonical).unwrap();
    assert_eq!(material.id(), id(100));
    assert_eq!(material.definition(), &original);
    for property in Property::ALL {
        match material.property_declaration(property, &canonical).unwrap() {
            Declaration::Length(actual) => {
                assert!(core::ptr::eq(actual, canonical.length(id(1)).unwrap()))
            }
            Declaration::Scalar(actual) => {
                assert_eq!(actual.kind(), property.expected_kind());
                assert!(core::ptr::eq(
                    actual,
                    canonical.scalar(actual.id()).unwrap()
                ));
                assert!(actual.authored_value().is_err());
            }
        }
    }
    assert_eq!(
        material.provenance_validation(),
        ValueProvenanceValidation::DeferredToDesignAndG4
    );
    assert_eq!(
        material.definition().shrinkage.unwrap().stage,
        Stage::PostGradePreExport
    );
    assert!(!format!("{material:?}").contains("private_material_fixture"));
    assert!(!format!("{material:?}").contains("composition claim"));
}

#[test]
fn required_text_is_checked_without_trimming_or_inventing_composition() {
    let canonical = unknown();
    for (field, text) in [(Field::Name, " \n"), (Field::Composition, "\t")] {
        let mut definition = definition();
        match field {
            Field::Name => definition.name = text.into(),
            Field::Composition => definition.composition = Some(text.into()),
        }
        assert_eq!(
            Material::new(definition, &canonical),
            Err(Error::EmptyText {
                material: id(100),
                field
            })
        );
    }
    let mut definition = definition();
    definition.composition = None;
    assert_eq!(
        Material::new(definition.clone(), &canonical)
            .unwrap()
            .definition(),
        &definition
    );
}

#[test]
fn property_kind_family_and_missing_identity_are_exact_without_conversion_or_peer_selection() {
    let canonical = unknown();
    let mut definition = definition();
    definition.nap = Target::Length(id(1));
    assert_eq!(
        Material::new(definition, &canonical),
        Err(Error::TargetKind {
            material: id(100),
            property: Property::Nap,
            expected: Kind::Boolean,
            target: Target::Length(id(1))
        })
    );
    for width in [
        Target::Length(id(8)),
        target(1, Kind::Length),
        Target::Length(id(999)),
    ] {
        let mut definition = self::definition();
        definition.width = width;
        assert_eq!(
            Material::new(definition, &canonical),
            Err(Error::MissingDeclaration {
                material: id(100),
                property: Property::Width,
                target: width
            })
        );
    }
    let material = Material::new(self::definition(), &canonical).unwrap();
    let current = replace(
        &canonical,
        2,
        Kind::Angle,
        State::Unknown {
            observation: id(904),
        },
    );
    assert_eq!(
        material.validate_current(&current),
        Err(Error::CanonicalKind {
            material: id(100),
            property: Property::Nap,
            declaration: id(2),
            expected: Kind::Boolean,
            actual: Kind::Angle
        })
    );
}

#[test]
fn present_widths_are_positive_for_every_authored_value_state_and_both_representations() {
    for value in [-1, 0, 1] {
        for (length_state, scalar_state) in [
            (
                LengthState::Known {
                    value: length(value),
                    evidence: vec![id(920)],
                },
                State::Known {
                    value: length(value).into(),
                    evidence: vec![id(920)],
                },
            ),
            (
                LengthState::Assumed {
                    value: length(value),
                    assumption: id(921),
                },
                State::Assumed {
                    value: length(value).into(),
                    assumption: id(921),
                },
            ),
            (
                LengthState::Preference {
                    value: length(value),
                    provenance: id(922),
                },
                State::Preference {
                    value: length(value).into(),
                    provenance: id(922),
                },
            ),
        ] {
            let dedicated = canonical(length_state);
            let general = replace(&dedicated, 8, Kind::Length, scalar_state);
            for (inputs, width) in [
                (&dedicated, Target::Length(id(1))),
                (&general, target(8, Kind::Length)),
            ] {
                let mut definition = definition();
                definition.width = width;
                let result = Material::new(definition, inputs);
                if value > 0 {
                    assert!(result.is_ok());
                } else {
                    assert_eq!(
                        result,
                        Err(Error::NonPositiveWidth {
                            material: id(100),
                            target: width,
                            actual_micrometres: value
                        })
                    );
                }
            }
        }
    }
    for state in [
        LengthState::Unknown {
            observation: id(923),
        },
        LengthState::Derived { formula: id(924) },
    ] {
        let canonical = canonical(state);
        let material = Material::new(definition(), &canonical).unwrap();
        assert!(material.validate_width_result(length(1)).is_ok());
        assert_eq!(
            material.validate_width_result(length(0)),
            Err(Error::NonPositiveWidth {
                material: id(100),
                target: Target::Length(id(1)),
                actual_micrometres: 0
            })
        );
        assert_eq!(
            material.validate_width_result(length(-1)),
            Err(Error::NonPositiveWidth {
                material: id(100),
                target: Target::Length(id(1)),
                actual_micrometres: -1
            })
        );
    }
}

#[test]
fn metadata_identity_disjointness_is_rechecked_after_canonical_replacement() {
    let canonical = unknown();
    let mut definition = definition();
    definition.id = id(1);
    assert_eq!(
        Material::new(definition, &canonical),
        Err(Error::IdentityCollision { material: id(1) })
    );
    let material = Material::new(self::definition(), &canonical).unwrap();
    let mut scalars = canonical.scalars().to_vec();
    scalars.push(scalar(
        100,
        Kind::Boolean,
        State::Unknown {
            observation: id(925),
        },
    ));
    let current = CanonicalInputs::new(canonical.lengths().to_vec(), scalars).unwrap();
    assert_eq!(
        material.validate_current(&current),
        Err(Error::IdentityCollision { material: id(100) })
    );
}

#[test]
fn current_flags_keep_explicit_false_or_derived_state_and_never_cache_an_old_fact() {
    let canonical = unknown();
    let material = Material::new(definition(), &canonical).unwrap();
    for state in [
        State::Known {
            value: Value::from_boolean(false),
            evidence: vec![id(926)],
        },
        State::Derived { formula: id(927) },
    ] {
        let current = replace(&canonical, 2, Kind::Boolean, state.clone());
        material.validate_current(&current).unwrap();
        let view = material
            .property_declaration(Property::Nap, &current)
            .unwrap();
        assert!(matches!(view, Declaration::Scalar(_)));
        if let Declaration::Scalar(actual) = view {
            assert!(core::ptr::eq(actual, current.scalar(id(2)).unwrap()));
            assert_eq!(actual.state(), &state);
            match state {
                State::Known { .. } => {
                    assert_eq!(actual.authored_value().unwrap().boolean(), Some(false))
                }
                _ => assert!(actual.authored_value().is_err()),
            }
            let debug = format!(
                "{:?}",
                material
                    .property_declaration(Property::Nap, &current)
                    .unwrap()
            );
            assert!(!debug.contains("evidence"));
            assert!(!debug.contains("926"));
        }
    }
    assert!(canonical.scalar(id(2)).unwrap().authored_value().is_err());
}

#[test]
fn optional_shrinkage_preserves_signed_ratio_state_treatment_and_explicit_unapplied_stage() {
    let canonical = unknown();
    let mut definition = definition();
    definition.shrinkage = None;
    let material = Material::new(definition, &canonical).unwrap();
    assert_eq!(material.target(Property::ShrinkageX), None);
    assert_eq!(
        material.property_declaration(Property::ShrinkageY, &canonical),
        Err(Error::NoShrinkage {
            material: id(100),
            property: Property::ShrinkageY
        })
    );
    let current = replace(
        &canonical,
        6,
        Kind::Ratio,
        State::Preference {
            value: Value::from_ratio_ppm(-20_000),
            provenance: id(928),
        },
    );
    let material = Material::new(self::definition(), &current).unwrap();
    let view = material
        .property_declaration(Property::ShrinkageX, &current)
        .unwrap();
    assert!(matches!(view, Declaration::Scalar(_)));
    if let Declaration::Scalar(actual) = view {
        assert_eq!(
            actual.authored_value().unwrap().numeric_integer(),
            Some(-20_000)
        );
    }
    assert_eq!(material.definition().shrinkage.unwrap().treatment, id(911));
    assert_eq!(
        material.definition().shrinkage.unwrap().stage,
        Stage::PostGradePreExport
    );
    let current = replace(
        &current,
        7,
        Kind::Count,
        State::Unknown {
            observation: id(929),
        },
    );
    assert_eq!(
        material.validate_current(&current),
        Err(Error::CanonicalKind {
            material: id(100),
            property: Property::ShrinkageY,
            declaration: id(7),
            expected: Kind::Ratio,
            actual: Kind::Count
        })
    );
}

#[test]
fn detached_edits_do_not_mutate_material_and_property_views_borrow_only_canonical_storage() {
    let canonical = unknown();
    let view = {
        let material = Material::new(definition(), &canonical).unwrap();
        material
            .property_declaration(Property::Width, &canonical)
            .unwrap()
    };
    assert_eq!(view, Declaration::Length(canonical.length(id(1)).unwrap()));
    let material = Material::new(definition(), &canonical).unwrap();
    let original = material.definition().clone();
    let mut detached = original.clone();
    detached.name.clear();
    assert!(Material::new(detached, &canonical).is_err());
    assert_eq!(material.definition(), &original);
}
