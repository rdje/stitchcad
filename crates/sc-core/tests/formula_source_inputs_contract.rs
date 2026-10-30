//! Complete actual canonical/source joins without reading, copying or evaluating states.
#![allow(clippy::expect_used)]
use sc_core::{
    ontology::EntityId,
    recipe::{
        FormulaExpressionDocument, FormulaSource, FormulaSourceInputRefusal as Refusal,
        FormulaSourceInputs as Scope, FormulaSourceLimits, FormulaSources,
    },
    value::{
        CanonicalInputPosition as Position, CanonicalInputTarget as Target, CanonicalInputs,
        LengthDeclaration, LengthDeclarationDefinition, LengthState, LengthValueError,
        ScalarInputDeclaration, ScalarInputDeclarationDefinition, ScalarInputKind as Kind,
        ScalarInputState as State, ScalarInputValue,
    },
};
use sc_units::Length;

fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn length(identity: u128, source: u128, state: LengthState) -> LengthDeclaration {
    LengthDeclaration::new(LengthDeclarationDefinition {
        id: id(identity),
        source: id(source),
        state,
    })
    .expect("valid original dedicated state")
}
fn scalar(identity: u128, source: u128, kind: Kind, state: State) -> ScalarInputDeclaration {
    ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
        id: id(identity),
        source: id(source),
        kind,
        state,
    })
    .expect("valid original scalar state")
}
fn sources(ids: &[u128]) -> FormulaSources {
    let documents: Vec<_> = ids
        .iter()
        .map(|identity| {
            FormulaSource::new(
                id(*identity),
                FormulaExpressionDocument::new(" missing + 1 mm / 0 ".into()).expect("syntax only"),
            )
        })
        .collect();
    let limits = FormulaSourceLimits::new(
        documents.len(),
        documents
            .iter()
            .map(|s| s.expression().source().len())
            .sum(),
        documents
            .iter()
            .map(|s| s.expression().canonical().as_str().len())
            .sum(),
    );
    FormulaSources::new(documents, limits).expect("unique explicit source entities")
}

#[test]
fn all_actual_derived_families_and_kinds_keep_exact_original_targets_and_one_real_source() {
    let inputs = CanonicalInputs::new(
        vec![length(10, 4000, LengthState::Derived { formula: id(9001) })],
        Kind::ALL
            .iter()
            .enumerate()
            .map(|(index, kind)| {
                scalar(
                    100 + index as u128,
                    4001,
                    *kind,
                    State::Derived { formula: id(9001) },
                )
            })
            .collect(),
    )
    .expect("actual complete inventory");
    let before = inputs.clone();
    let originals = sources(&[9001]);
    let scope = Scope::new(&inputs, &originals).expect("all original formula references exist");
    assert!(std::ptr::eq(scope.inputs(), &inputs));
    assert!(std::ptr::eq(scope.sources(), &originals));
    let source = originals.source(id(9001)).expect("exact authored source");
    let mut joined = scope.derived_inputs();
    let dedicated = joined.next().expect("original dedicated target");
    assert_eq!(dedicated.position(), Position::Length { index: 0 });
    assert_eq!(dedicated.target(), Target::Length(id(10)));
    let record = dedicated.length().expect("dedicated family");
    assert!(std::ptr::eq(
        record,
        inputs.length(id(10)).expect("actual canonical owner")
    ));
    assert_eq!(
        record.authored_value(),
        Err(LengthValueError::RequiresEvaluation {
            declaration: id(10),
            formula: id(9001),
        })
    );
    assert!(dedicated.scalar().is_none());
    assert!(std::ptr::eq(dedicated.source(), source));
    for (index, (actual, record)) in joined.zip(inputs.scalars()).enumerate() {
        assert_eq!(actual.position(), Position::Scalar { index });
        assert_eq!(
            actual.target(),
            Target::Scalar {
                declaration: record.id(),
                kind: record.kind()
            }
        );
        assert!(std::ptr::eq(
            actual.scalar().expect("general family"),
            record
        ));
        assert!(actual.length().is_none());
        assert!(std::ptr::eq(actual.source(), source));
    }
    assert_eq!(scope.derived_inputs().count(), 7);
    let reverse: Vec<_> = scope
        .derived_inputs()
        .rev()
        .map(|input| input.position())
        .collect();
    assert_eq!(reverse.last(), Some(&Position::Length { index: 0 }));
    assert_eq!(inputs, before);
    assert!(!format!("{scope:?}").contains("missing"));
    // Missing names, bad operation and incompatible output kinds still need their real checks.
    assert_eq!(source.expression().source(), " missing + 1 mm / 0 ");
}

#[test]
fn missing_formula_keeps_actual_position_family_kind_and_never_uses_provenance_as_source() {
    let originals = sources(&[9001]);
    for general in [false, true] {
        let inputs = CanonicalInputs::new(
            vec![
                length(50, 4000, LengthState::Derived { formula: id(9001) }),
                length(
                    3,
                    9001,
                    if general {
                        LengthState::Unknown {
                            observation: id(80),
                        }
                    } else {
                        LengthState::Derived { formula: id(9002) }
                    },
                ),
            ],
            vec![scalar(
                1,
                9001,
                Kind::Area,
                State::Derived { formula: id(9002) },
            )],
        )
        .expect("actual unchanged families");
        let error =
            Scope::new(&inputs, &originals).expect_err("late missing source refuses whole scope");
        assert!(std::ptr::eq(error.inputs(), &inputs));
        assert!(std::ptr::eq(error.sources(), &originals));
        assert_eq!(
            error.refusal(),
            Refusal::MissingSource { formula: id(9002) }
        );
        if general {
            assert_eq!(error.position(), Position::Scalar { index: 0 });
            assert_eq!(
                error.target(),
                Target::Scalar {
                    declaration: id(1),
                    kind: Kind::Area
                }
            );
        } else {
            assert_eq!(error.position(), Position::Length { index: 1 });
            assert_eq!(error.target(), Target::Length(id(3)));
        }
        assert!(!format!("{error}").contains("missing +"));
    }
}

#[test]
fn complete_owned_id_census_precedes_missing_references_in_canonical_family_order() {
    let inputs = CanonicalInputs::new(
        vec![
            length(90, 4000, LengthState::Derived { formula: id(9999) }),
            length(
                20,
                4000,
                LengthState::Unknown {
                    observation: id(4001),
                },
            ),
        ],
        vec![scalar(
            10,
            4000,
            Kind::Boolean,
            State::Preference {
                value: ScalarInputValue::from_boolean(false),
                provenance: id(4002),
            },
        )],
    )
    .expect("unique canonical owners");
    // Opposite source/id order; neither colliding entity is referenced by a Derived state.
    let originals = sources(&[10, 20]);
    let error = Scope::new(&inputs, &originals).expect_err("ownership before source availability");
    assert_eq!(error.position(), Position::Length { index: 1 });
    assert_eq!(error.target(), Target::Length(id(20)));
    assert_eq!(
        error.refusal(),
        Refusal::IdentityCollision { source_index: 1 }
    );
    let scalar_only_collision = sources(&[10]);
    let error =
        Scope::new(&inputs, &scalar_only_collision).expect_err("full census includes scalar owner");
    assert_eq!(error.position(), Position::Scalar { index: 0 });
    assert_eq!(
        error.target(),
        Target::Scalar {
            declaration: id(10),
            kind: Kind::Boolean
        }
    );
    assert_eq!(
        error.refusal(),
        Refusal::IdentityCollision { source_index: 0 }
    );
}

#[test]
fn known_assumed_preference_and_unknown_keep_original_values_and_referenced_provenance() {
    let inputs = CanonicalInputs::new(
        vec![
            length(
                1,
                9001,
                LengthState::Known {
                    value: Length::ZERO,
                    evidence: vec![id(9001)],
                },
            ),
            length(
                2,
                9001,
                LengthState::Assumed {
                    value: Length::ZERO,
                    assumption: id(9001),
                },
            ),
            length(
                3,
                9001,
                LengthState::Preference {
                    value: Length::ZERO,
                    provenance: id(9001),
                },
            ),
            length(
                4,
                9001,
                LengthState::Unknown {
                    observation: id(9001),
                },
            ),
        ],
        vec![scalar(
            5,
            9001,
            Kind::Boolean,
            State::Preference {
                value: ScalarInputValue::from_boolean(false),
                provenance: id(9001),
            },
        )],
    )
    .expect("references are not additional owned declarations");
    let before = inputs.clone();
    let originals = sources(&[9001]);
    let scope =
        Scope::new(&inputs, &originals).expect("only actual ownership and Derived references join");
    assert_eq!(scope.derived_inputs().count(), 0);
    assert_eq!(inputs, before);
    let empty = CanonicalInputs::new(Vec::new(), Vec::new()).expect("empty actual owner");
    assert_eq!(
        Scope::new(&empty, &originals)
            .expect("unused source stays inspectable")
            .derived_inputs()
            .count(),
        0
    );
    let no_sources = sources(&[]);
    assert!(Scope::new(&inputs, &no_sources).is_ok());
}
