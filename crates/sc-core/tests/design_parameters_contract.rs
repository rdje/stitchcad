//! Owned parameter metadata borrows current canonical input state, without value/source duplication.
#![allow(clippy::expect_used)]
use sc_core::{
    design::{
        DesignParameterDefinition as Definition, DesignParameterError as Error,
        DesignParameterTarget as Target, DesignParameters as Parameters,
    },
    name::MachineToken,
    ontology::EntityId,
    recipe::{
        FormulaBindingKind, FormulaDeclarationSource, FormulaLengthInput, FormulaScalarInput,
        FormulaScalarInputAdapterRule,
    },
    value::{
        CanonicalInputs, LengthDeclaration, LengthDeclarationDefinition, LengthState,
        ScalarInputDeclaration, ScalarInputDeclarationDefinition, ScalarInputKind as Kind,
        ScalarInputState as State, ScalarInputValue as Value,
    },
};

fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn inputs(kind: Kind, state: State) -> CanonicalInputs {
    CanonicalInputs::new(
        vec![LengthDeclaration::new(LengthDeclarationDefinition {
            id: id(1),
            source: id(900),
            state: LengthState::Derived { formula: id(901) },
        })
        .expect("explicit derived length")],
        vec![
            ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
                id: id(2),
                source: id(902),
                kind,
                state,
            })
            .expect("explicit general scalar"),
            ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
                id: id(3),
                source: id(903),
                kind: Kind::Length,
                state: State::Unknown {
                    observation: id(904),
                },
            })
            .expect("distinct general Length scalar"),
        ],
    )
    .expect("unique canonical fixture inputs")
}
fn unknown() -> CanonicalInputs {
    inputs(
        Kind::Boolean,
        State::Unknown {
            observation: id(905),
        },
    )
}
fn parameter(identity: u128, name: &str, target: Target) -> Definition {
    Definition {
        id: id(identity),
        token: MachineToken::new(name).expect("valid token"),
        target,
    }
}
fn scalar() -> Target {
    Target::Scalar {
        declaration: id(2),
        kind: Kind::Boolean,
    }
}

#[test]
fn mixed_families_order_and_current_canonical_borrows_remain_exact() {
    let canonical = unknown();
    let definitions = vec![
        parameter(
            12,
            "raw_length",
            Target::Scalar {
                declaration: id(3),
                kind: Kind::Length,
            },
        ),
        parameter(10, "width", Target::Length(id(1))),
        parameter(11, "enabled", scalar()),
        parameter(13, "other_width", Target::Length(id(1))),
    ];
    let registry = Parameters::new(definitions.clone(), &canonical).unwrap();
    for (actual, original) in registry.parameters().iter().zip(&definitions) {
        assert_eq!(actual.definition(), original);
        assert!(core::ptr::eq(
            registry.parameter(actual.id()).unwrap(),
            actual
        ));
    }
    assert!(registry.parameter(id(99)).is_none());
    let length = registry
        .parameter(id(10))
        .unwrap()
        .declaration(&canonical)
        .unwrap();
    let view = FormulaLengthInput::from_declaration(length).unwrap();
    assert!(core::ptr::eq(
        view.canonical_declaration(),
        canonical.length(id(1)).unwrap()
    ));
    assert!(view.read_authored().is_err());
    let general = registry
        .parameter(id(12))
        .unwrap()
        .declaration(&canonical)
        .unwrap();
    assert!(FormulaLengthInput::from_declaration(general).is_none());
    let view =
        FormulaScalarInput::from_declaration(general, canonical.scalar(id(3)).unwrap()).unwrap();
    assert!(core::ptr::eq(
        view.canonical_declaration(),
        canonical.scalar(id(3)).unwrap()
    ));
    assert!(view.read_authored().is_err());
}

#[test]
fn metadata_id_duplicates_and_canonical_identity_aliases_are_refused_before_targets() {
    let canonical = unknown();
    assert_eq!(
        Parameters::new(
            vec![
                parameter(10, "first", Target::Length(id(99))),
                parameter(10, "second", scalar())
            ],
            &canonical
        ),
        Err(Error::DuplicateIdentity {
            identity: id(10),
            first: 0,
            duplicate: 1,
        })
    );
    for identity in [1, 2, 3] {
        assert_eq!(
            Parameters::new(vec![parameter(identity, "alias", scalar())], &canonical),
            Err(Error::CanonicalIdentityCollision {
                identity: id(identity),
                index: 0
            })
        );
    }
}

#[test]
fn equal_names_refuse_even_for_shared_targets_and_language_inputs_remain_reserved() {
    let canonical = unknown();
    assert_eq!(
        Parameters::new(
            vec![
                parameter(10, "width", Target::Length(id(1))),
                parameter(11, "width", Target::Length(id(1)))
            ],
            &canonical
        ),
        Err(Error::AmbiguousName {
            first: 0,
            duplicate: 1
        })
    );
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
        assert_eq!(
            Parameters::new(vec![parameter(10, name, scalar())], &canonical),
            Err(Error::ReservedName { parameter: id(10) })
        );
    }
}

#[test]
fn target_representation_is_pinned_without_cross_family_length_conversion() {
    let canonical = unknown();
    assert_eq!(
        Parameters::new(
            vec![parameter(10, "length", Target::Length(id(3)))],
            &canonical
        ),
        Err(Error::MissingLength {
            parameter: id(10),
            declaration: id(3)
        })
    );
    assert_eq!(
        Parameters::new(
            vec![parameter(
                10,
                "length",
                Target::Scalar {
                    declaration: id(1),
                    kind: Kind::Length
                }
            )],
            &canonical
        ),
        Err(Error::MissingScalar {
            parameter: id(10),
            declaration: id(1)
        })
    );
}

#[test]
fn current_kind_changes_refuse_with_the_original_adapter_rule() {
    let canonical = unknown();
    let registry = Parameters::new(vec![parameter(10, "enabled", scalar())], &canonical).unwrap();
    let changed = inputs(
        Kind::Angle,
        State::Unknown {
            observation: id(905),
        },
    );
    let expected = Error::ScalarTarget {
        parameter: id(10),
        rule: FormulaScalarInputAdapterRule::CanonicalKind {
            declared: FormulaBindingKind::Boolean,
            actual: Kind::Angle,
        },
    };
    assert_eq!(registry.validate_current(&changed), Err(expected.clone()));
    assert_eq!(
        registry
            .parameter(id(10))
            .unwrap()
            .declaration(&changed)
            .unwrap_err(),
        expected
    );
}

#[test]
fn missing_current_targets_keep_the_original_binding_without_peer_reassignment() {
    let canonical = unknown();
    let empty = CanonicalInputs::new(vec![], vec![]).unwrap();
    for (target, expected) in [
        (
            Target::Length(id(1)),
            Error::MissingLength {
                parameter: id(10),
                declaration: id(1),
            },
        ),
        (
            scalar(),
            Error::MissingScalar {
                parameter: id(10),
                declaration: id(2),
            },
        ),
    ] {
        let registry = Parameters::new(vec![parameter(10, "held", target)], &canonical).unwrap();
        assert_eq!(registry.validate_current(&empty), Err(expected));
        assert_eq!(
            registry.parameter(id(10)).unwrap().definition().target,
            target
        );
    }
}

#[test]
fn same_id_current_state_is_borrowed_without_cached_unknowns_or_values() {
    let canonical = unknown();
    let registry = Parameters::new(vec![parameter(10, "enabled", scalar())], &canonical).unwrap();
    let current = inputs(
        Kind::Boolean,
        State::Preference {
            value: Value::from_boolean(false),
            provenance: id(906),
        },
    );
    registry.validate_current(&current).unwrap();
    let metadata = registry
        .parameter(id(10))
        .unwrap()
        .declaration(&current)
        .unwrap();
    assert!(
        matches!(metadata.source(), FormulaDeclarationSource::Input { input, declaration, .. }
        if input == id(10) && declaration == id(2))
    );
    let view =
        FormulaScalarInput::from_declaration(metadata, current.scalar(id(2)).unwrap()).unwrap();
    assert!(core::ptr::eq(
        view.canonical_declaration(),
        current.scalar(id(2)).unwrap()
    ));
    assert_eq!(view.read_authored().unwrap().value().boolean(), Some(false));
    assert_eq!(view.canonical_declaration().definition().source, id(902));
}

#[test]
fn empty_and_detached_metadata_edits_do_not_change_owned_parameters() {
    let canonical = unknown();
    assert!(Parameters::new(vec![], &canonical)
        .unwrap()
        .parameters()
        .is_empty());
    let registry = Parameters::new(
        vec![parameter(10, "width", Target::Length(id(1)))],
        &canonical,
    )
    .unwrap();
    let mut detached = registry.parameter(id(10)).unwrap().definition().clone();
    detached.target = scalar();
    assert_eq!(
        registry.parameter(id(10)).unwrap().definition().target,
        Target::Length(id(1))
    );
    assert_ne!(&detached, registry.parameter(id(10)).unwrap().definition());
}

#[test]
fn whole_current_validation_detects_new_metadata_id_aliases_in_canonical_inputs() {
    let canonical = unknown();
    let registry = Parameters::new(vec![parameter(10, "enabled", scalar())], &canonical).unwrap();
    let mut lengths = canonical.lengths().to_vec();
    lengths.push(
        LengthDeclaration::new(LengthDeclarationDefinition {
            id: id(10),
            source: id(907),
            state: LengthState::Unknown {
                observation: id(908),
            },
        })
        .unwrap(),
    );
    let current = CanonicalInputs::new(lengths, canonical.scalars().to_vec()).unwrap();
    assert_eq!(
        registry.validate_current(&current),
        Err(Error::CanonicalIdentityCollision {
            identity: id(10),
            index: 0,
        })
    );
}
