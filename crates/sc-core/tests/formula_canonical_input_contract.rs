//! Real inventory membership precedes authored reads and never changes canonical families.
#![allow(clippy::expect_used)]
use sc_core::construction::{DraftingInitialInputSource, DraftingSourceRecord};
use sc_core::{
    name::MachineToken,
    ontology::{EdgeRef, EntityId, LocalTag, PointRef},
    recipe::{
        FormulaBindingKind as B, FormulaCanonicalInput as Joined,
        FormulaCanonicalInputAdapter as Adapter, FormulaCanonicalInputRefusal as Refusal,
        FormulaCheckedNodeForm as Form, FormulaDeclaration as D,
        FormulaDeclarationSource as Source, FormulaExpression, FormulaInitialDeclaration,
        FormulaInputOrigin as Origin, FormulaNamespace, FormulaRecipe, FormulaReservedName,
        FormulaScalarInputAdapterRule as ScalarRule, FormulaScalarInputOrigin as ScalarOrigin,
    },
    value::{
        CanonicalInputs, LengthDeclaration, LengthDeclarationDefinition, LengthState,
        LengthValueError, ScalarInputDeclaration, ScalarInputDeclarationDefinition,
        ScalarInputKind as K, ScalarInputReadError, ScalarInputState as State,
        ScalarInputValue as Value,
    },
};
use sc_units::Length;

fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn length(identity: u128, state: LengthState) -> LengthDeclaration {
    LengthDeclaration::new(LengthDeclarationDefinition {
        id: id(identity),
        source: id(20),
        state,
    })
    .expect("canonical length")
}
fn scalar(identity: u128, kind: K, state: State) -> ScalarInputDeclaration {
    ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
        id: id(identity),
        source: id(21),
        kind,
        state,
    })
    .expect("canonical scalar")
}
fn unknown(identity: u128) -> LengthDeclaration {
    length(
        identity,
        LengthState::Unknown {
            observation: id(30),
        },
    )
}
fn inventory() -> CanonicalInputs {
    CanonicalInputs::new(
        vec![unknown(9), unknown(1)],
        vec![
            scalar(
                8,
                K::Count,
                State::Unknown {
                    observation: id(32),
                },
            ),
            scalar(
                2,
                K::Boolean,
                State::Preference {
                    value: Value::from_boolean(false),
                    provenance: id(31),
                },
            ),
        ],
    )
    .expect("unique families")
}

#[test]
fn initial_source_view_keeps_actual_inventory_metadata_and_opaque_snapshot_loans() {
    struct Metadata {
        name: MachineToken,
        id: EntityId,
    }
    struct Snapshot {
        data: Vec<u8>,
    }
    let inputs = inventory();
    let metadata = DraftingSourceRecord::new(Metadata {
        name: MachineToken::new("private_initial_source").expect("source token"),
        id: id(50),
    });
    let snapshot = DraftingSourceRecord::new(Snapshot { data: vec![7, 8] });
    let actual = inputs.length(id(1)).expect("actual dedicated record");
    let declarations = [
        D::length_input(
            &metadata.data().name,
            Origin::Measurement,
            metadata.data().id,
            actual,
        ),
        D::input(
            &metadata.data().name,
            ScalarOrigin::Profile,
            metadata.data().id,
            id(8),
            B::Count,
        ),
    ];
    for declaration in declarations {
        let joined = Joined::resolve(declaration, &inputs).expect("actual inventory/kind join");
        let source = DraftingInitialInputSource {
            input: joined,
            metadata: &metadata,
            snapshot: &snapshot,
        };
        let copied = source;
        assert!(std::ptr::eq(copied.input.inputs(), &inputs));
        assert!(copied.metadata.is_same_record(&metadata));
        assert!(source.snapshot.is_same_record(copied.snapshot));
        assert!(std::ptr::eq(
            copied.input.metadata().name(),
            metadata.data().name.as_str()
        ));
        assert_eq!(copied.snapshot.data().data.as_slice(), [7, 8]);
        assert!(!format!("{copied:?}").contains("private_initial_source"));
        match copied.input.adapter() {
            Adapter::Length(input) => {
                assert!(std::ptr::eq(input.canonical_declaration(), actual));
                assert!(matches!(input.state(), LengthState::Unknown { .. }));
            }
            Adapter::Scalar(input) => {
                assert!(std::ptr::eq(
                    input.canonical_declaration(),
                    inputs.scalar(id(8)).expect("actual scalar")
                ));
                assert!(matches!(input.state(), State::Unknown { .. }));
            }
        }
    }
}
#[test]
fn dedicated_records_keep_actual_inventory_borrows_and_all_original_read_requirements() {
    let name = MachineToken::new("body_width").expect("name");
    let inputs = CanonicalInputs::new(
        vec![
            length(
                1,
                LengthState::Known {
                    value: Length::ZERO,
                    evidence: vec![id(32)],
                },
            ),
            unknown(2),
            length(3, LengthState::Derived { formula: id(33) }),
        ],
        vec![],
    )
    .expect("inventory");
    let before = inputs.clone();
    for origin in [
        Origin::Measurement,
        Origin::Ease,
        Origin::Parameter,
        Origin::Profile,
        Origin::Material,
    ] {
        for record in inputs.lengths() {
            let metadata = D::length_input(&name, origin, id(40), record);
            let joined = Joined::resolve(metadata, &inputs).expect("exact inventory record");
            assert!(std::ptr::eq(joined.inputs(), &inputs));
            assert_eq!(joined.metadata().name(), metadata.name());
            let Adapter::Length(input) = joined.adapter() else {
                unreachable!("dedicated fixture");
            };
            assert!(std::ptr::eq(input.canonical_declaration(), record));
            assert!(std::ptr::eq(input.state(), record.state()));
            assert_eq!(input.origin(), origin);
            match record.state() {
                LengthState::Known { value, .. } => assert_eq!(
                    input.read_authored().expect("authored zero").value(),
                    (*value).into()
                ),
                LengthState::Unknown { observation } => assert_eq!(
                    input.read_authored().expect_err("still unknown").reason(),
                    LengthValueError::RequiresObservation {
                        declaration: record.id(),
                        observation: *observation
                    }
                ),
                LengthState::Derived { formula } => assert_eq!(
                    input.read_authored().expect_err("still derived").reason(),
                    LengthValueError::RequiresEvaluation {
                        declaration: record.id(),
                        formula: *formula
                    }
                ),
                _ => unreachable!("three actual fixture states"),
            }
        }
    }
    assert_eq!(inputs, before);
}

#[test]
fn all_general_kinds_and_origins_select_actual_unknown_or_derived_registry_records() {
    let name = MachineToken::new("private_scalar").expect("name");
    for kind in [K::Length, K::Angle, K::Area, K::Ratio, K::Count, K::Boolean] {
        let state = if kind == K::Area {
            State::Derived { formula: id(33) }
        } else {
            State::Unknown {
                observation: id(30),
            }
        };
        let inputs =
            CanonicalInputs::new(vec![], vec![scalar(1, kind, state)]).expect("actual inventory");
        let record = inputs.scalar(id(1)).expect("actual scalar");
        for origin in [
            ScalarOrigin::Parameter,
            ScalarOrigin::Profile,
            ScalarOrigin::Material,
        ] {
            let metadata = D::input(&name, origin, id(40), record.id(), B::from(kind));
            let joined = Joined::resolve(metadata, &inputs).expect("identity/kind before read");
            let Adapter::Scalar(input) = joined.adapter() else {
                unreachable!("scalar fixture");
            };
            assert!(std::ptr::eq(input.canonical_declaration(), record));
            assert!(std::ptr::eq(input.state(), record.state()));
            assert_eq!(input.origin(), origin);
            assert_eq!(input.metadata().name(), metadata.name());
            assert_eq!(
                input.read_authored().expect_err("no fallback").reason(),
                if kind == K::Area {
                    ScalarInputReadError::RequiresEvaluation {
                        declaration: id(1),
                        formula: id(33),
                    }
                } else {
                    ScalarInputReadError::RequiresObservation {
                        declaration: id(1),
                        observation: id(30),
                    }
                }
            );
            assert!(!format!("{joined:?}").contains("private_scalar"));
        }
    }
}

#[test]
fn missing_and_wrong_family_targets_precede_kind_with_actual_original_positions_and_causes() {
    let name = MachineToken::new("private_customer_value").expect("name");
    let inputs = inventory();
    for (target, other_position) in [(2, Some(1)), (99, None)] {
        let external = unknown(target);
        let metadata = D::length_input(&name, Origin::Measurement, id(40), &external);
        let error = Joined::resolve(metadata, &inputs).expect_err("no dedicated target");
        assert!(
            matches!(error.refusal(), Refusal::MissingLength { declaration, scalar_index }
            if declaration == id(target) && scalar_index == other_position)
        );
        assert!(std::ptr::eq(error.inputs(), &inputs));
        assert!(
            matches!(error.metadata().source(), Source::LengthInput { declaration, .. }
            if std::ptr::eq(declaration, &external))
        );
        assert!(!format!("{error:?}").contains(name.as_str()));
    }
    for (target, other_position) in [(1, Some(1)), (99, None)] {
        let metadata = D::input(&name, ScalarOrigin::Parameter, id(40), id(target), B::Area);
        let error = Joined::resolve(metadata, &inputs).expect_err("no scalar target");
        assert!(
            matches!(error.refusal(), Refusal::MissingScalar { declaration, length_index }
            if declaration == id(target) && length_index == other_position)
        );
        assert!(std::ptr::eq(error.inputs(), &inputs));
    }
    let wrong_kind = D::input(&name, ScalarOrigin::Profile, id(40), id(2), B::Length);
    let error = Joined::resolve(wrong_kind, &inputs).expect_err("actual Boolean wins");
    let Refusal::Scalar(cause) = error.refusal() else {
        unreachable!("original scalar cause");
    };
    assert_eq!(cause.name(), name.as_str());
    assert!(std::ptr::eq(
        cause.canonical_declaration(),
        inputs.scalar(id(2)).expect("actual candidate")
    ));
    assert_eq!(
        cause.rule(),
        ScalarRule::CanonicalKind {
            declared: B::Length,
            actual: K::Boolean
        }
    );
    assert!(!format!("{error:?}").contains(name.as_str()));
}

#[test]
fn equal_inventory_and_equal_external_length_copies_cannot_replace_the_actual_borrowed_owner() {
    let name = MachineToken::new("body_width").expect("name");
    let inputs = inventory();
    let other = inputs.clone();
    assert_eq!(inputs, other);
    let record = inputs.length(id(1)).expect("actual original");
    let external = record.clone();
    for borrowed in [record, &external] {
        let metadata = D::length_input(&name, Origin::Measurement, id(40), borrowed);
        let error = Joined::resolve(metadata, &other).expect_err("actual owner differs");
        assert!(
            matches!(error.refusal(), Refusal::LengthOwner { declaration }
            if declaration == id(1))
        );
        assert!(std::ptr::eq(error.inputs(), &other));
    }
    let metadata = D::length_input(&name, Origin::Measurement, id(40), &external);
    assert!(
        matches!(Joined::resolve(metadata, &inputs).expect_err("equal external copy").refusal(),
        Refusal::LengthOwner { declaration } if declaration == id(1))
    );
    let metadata = D::length_input(&name, Origin::Measurement, id(40), record);
    assert!(Joined::resolve(metadata, &inputs).is_ok());

    // General metadata carries an identity, not a canonical borrow; it selects this actual owner.
    let metadata = D::input(&name, ScalarOrigin::Material, id(40), id(2), B::Boolean);
    let joined =
        Joined::resolve(metadata, &other).expect("actual scalar selected in supplied owner");
    let Adapter::Scalar(input) = joined.adapter() else {
        unreachable!("general Boolean fixture");
    };
    assert!(std::ptr::eq(
        input.canonical_declaration(),
        other.scalar(id(2)).expect("actual supplied record")
    ));
    assert!(!std::ptr::eq(
        input.canonical_declaration(),
        inputs.scalar(id(2)).expect("original record")
    ));
    assert_eq!(
        input
            .read_authored()
            .expect("present false")
            .value()
            .boolean(),
        Some(false)
    );
}

#[test]
fn actual_checked_name_metadata_joins_but_reserved_geometry_and_prior_let_sources_do_not() {
    let name = MachineToken::new("body_width").expect("name");
    let inputs = inventory();
    let record = inputs.length(id(1)).expect("actual record");
    let metadata = D::length_input(&name, Origin::Measurement, id(40), record);
    let expression = FormulaExpression::parse("(body_width)")
        .expect("syntax")
        .normalize_literals()
        .expect("inputs");
    let namespace =
        FormulaNamespace::new([FormulaInitialDeclaration::try_from(metadata).expect("initial")])
            .expect("namespace");
    let proof = expression
        .check_kinds(&namespace)
        .expect("actual static proof");
    let Form::Name { declaration, .. } = proof.root().form() else {
        unreachable!("actual name fixture");
    };
    let joined = Joined::resolve(declaration, &inputs).expect("source resolved during checking");
    assert!(
        matches!(joined.metadata().source(), Source::LengthInput { declaration, .. }
        if std::ptr::eq(declaration, record))
    );
    let recipe = FormulaRecipe::parse("let value:length=1 mm")
        .expect("recipe")
        .normalize_literals()
        .expect("inputs");
    for metadata in [
        D::reserved(FormulaReservedName::IsBaseSize),
        D::point(&name, PointRef::new(id(50), LocalTag::FIRST)),
        D::edge(&name, EdgeRef::new(id(51), LocalTag::FIRST)),
        D::recipe(&recipe, 1).expect("actual prior let"),
    ] {
        let error = Joined::resolve(metadata, &inputs).expect_err("not a canonical input source");
        assert!(matches!(error.refusal(), Refusal::NotCanonicalInput));
        assert_eq!(error.metadata().kind(), metadata.kind());
        assert_eq!(error.metadata().origin(), metadata.origin());
    }
}
