//! Real higher-source integration; metadata borrowing is distinct from current garment sources.
#![allow(clippy::expect_used, clippy::panic)]
use super::{fixture, id, with_sewing_graph};
use sc_core::{
    design::{
        DesignConstructionDeclarationError as Error, DesignConstructionDeclarationRole as Role,
        DesignConstructionDeclarationSource as Source, DesignInputs, DesignInputsDefinition,
        DesignParameterDefinition, DesignParameterTarget, DesignParameters, DesignStructure,
    },
    name::MachineToken,
    ontology::*,
    recipe::{FormulaKind, FormulaLengthInput, FormulaScalarInput},
    value::{
        CanonicalInputs, ScalarInputDeclaration, ScalarInputDeclarationDefinition, ScalarInputKind,
        ScalarInputState,
    },
};
use sc_measure::DesignSources;
use sc_units::Length;

fn with_folds(
    definition: DesignInputsDefinition<DesignSources>,
    fields: Vec<(ConstructionKind, IntakeAmount)>,
) -> (DesignInputs<DesignSources>, ConstructionInventory) {
    let (state, _) = with_sewing_graph(definition, vec![]);
    let structure = state.structure();
    let piece = structure
        .pieces()
        .pieces()
        .first()
        .expect("actual held source");
    let edge = piece.edges().next().expect("owned source edge").edge;
    let direction = DirectedRange {
        range: EdgeRange::whole(edge),
        direction: Direction::Original,
    };
    let mut darts = Vec::new();
    let mut tucks = Vec::new();
    let mut pleats = Vec::new();
    for (index, (kind, intake)) in fields.into_iter().enumerate() {
        let identity = id(110000 + u128::try_from(index).expect("fixture ordinal"));
        if kind == ConstructionKind::Dart {
            darts.push(
                Dart::new(
                    DartDefinition {
                        id: identity,
                        intake,
                        apex: EdgeAnchor {
                            edge,
                            param: Param::END,
                        },
                        first_leg: direction,
                        second_leg: DirectedRange {
                            range: EdgeRange::new(
                                edge,
                                Param::START,
                                Param::new(Rational::new(1, 2).expect("rational"))
                                    .expect("parameter"),
                            )
                            .expect("distinct interval"),
                            direction: Direction::Reversed,
                        },
                        direction,
                        closing_operation: id(2000),
                    },
                    piece,
                    structure.topology(),
                )
                .expect("actual born Dart"),
            );
        } else {
            let definition = FoldDefinition {
                id: identity,
                intake,
                fold_lines: vec![direction],
                direction,
                closing_operation: id(2000),
            };
            match kind {
                ConstructionKind::Tuck => tucks
                    .push(Tuck::new(definition, piece, structure.topology()).expect("actual Tuck")),
                ConstructionKind::Pleat => pleats.push(
                    Pleat::new(definition, piece, structure.topology()).expect("actual Pleat"),
                ),
                _ => panic!("fixture only authors three intake families"),
            }
        }
    }
    let inventory = ConstructionInventory::new(darts, tucks, pleats, vec![], vec![], vec![])
        .expect("actual family orders");
    (state, inventory)
}
fn explicit() -> IntakeAmount {
    IntakeAmount::Explicit {
        parameter: id(111000),
        value: Length::ZERO,
    }
}
fn original_fields(source: ConstructionRef<'_>) -> (&IntakeAmount, &EntityId) {
    match source {
        ConstructionRef::Dart(value) => (
            &value.definition().intake,
            &value.definition().closing_operation,
        ),
        ConstructionRef::Tuck(value) => (
            &value.definition().intake,
            &value.definition().closing_operation,
        ),
        ConstructionRef::Pleat(value) => (
            &value.definition().intake,
            &value.definition().closing_operation,
        ),
        _ => panic!("actual intake family"),
    }
}

#[test]
fn exact_roles_borrow_real_objects_and_both_unread_length_families_without_coercion() {
    let mut definition = fixture();
    let mut structure = definition.structure.definition().clone();
    let mut scalars = structure.inputs.scalars().to_vec();
    scalars.push(
        ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
            id: id(2003),
            source: id(2005),
            kind: ScalarInputKind::Length,
            state: ScalarInputState::Unknown {
                observation: id(2004),
            },
        })
        .expect("actual unknown general Length"),
    );
    structure.inputs = CanonicalInputs::new(structure.inputs.lengths().to_vec(), scalars)
        .expect("single canonical owner");
    definition.structure = DesignStructure::new(structure).expect("actual current structure");
    definition.parameters = DesignParameters::new(
        vec![
            DesignParameterDefinition {
                id: id(5001),
                token: MachineToken::new("intake_length").expect("token"),
                target: DesignParameterTarget::Length(id(12)),
            },
            DesignParameterDefinition {
                id: id(5002),
                token: MachineToken::new("intake_general").expect("token"),
                target: DesignParameterTarget::Scalar {
                    declaration: id(2003),
                    kind: ScalarInputKind::Length,
                },
            },
        ],
        definition.structure.inputs(),
    )
    .expect("actual Derived/Unknown metadata");
    let (state, inventory) = with_folds(
        definition,
        vec![
            (ConstructionKind::Dart, explicit()),
            (ConstructionKind::Dart, IntakeAmount::Formula(id(5001))),
            (ConstructionKind::Tuck, IntakeAmount::Formula(id(5002))),
            (
                ConstructionKind::Pleat,
                IntakeAmount::Profile(ProfileParameterRef::new(id(10))),
            ),
        ],
    );
    let saved = (state.inputs().clone(), inventory.clone());
    let views = state
        .construction_intake_declarations(&inventory)
        .expect("actual typed metadata");
    assert_eq!(
        views
            .iter()
            .map(|view| (view.position().kind, view.position().index, view.role()))
            .collect::<Vec<_>>(),
        vec![
            (ConstructionKind::Dart, 0, Role::Intake),
            (ConstructionKind::Dart, 0, Role::ClosingOperation),
            (ConstructionKind::Dart, 1, Role::Intake),
            (ConstructionKind::Dart, 1, Role::ClosingOperation),
            (ConstructionKind::Tuck, 0, Role::Intake),
            (ConstructionKind::Tuck, 0, Role::ClosingOperation),
            (ConstructionKind::Pleat, 0, Role::Intake),
            (ConstructionKind::Pleat, 0, Role::ClosingOperation),
        ]
    );
    for view in views {
        let (_, original) = inventory
            .entries()
            .find(|(position, _)| *position == view.position())
            .expect("original position");
        let (intake, closing) = original_fields(original);
        let (actual_intake, actual_closing) = original_fields(view.construction());
        assert!(core::ptr::eq(intake, actual_intake) && core::ptr::eq(closing, actual_closing));
        assert!(core::ptr::eq(view.canonical_inputs(), state.inputs()));
        match view.source() {
            Source::ExplicitIntake { parameter, value } => {
                let IntakeAmount::Explicit {
                    parameter: actual,
                    value: actual_value,
                } = intake
                else {
                    panic!("explicit")
                };
                assert!(core::ptr::eq(parameter, actual) && core::ptr::eq(value, actual_value));
                assert_eq!(*value, Length::ZERO);
                assert!(state.parameters().parameter(*parameter).is_none());
                assert_eq!(view.profile_validation(), None);
            }
            Source::FormulaIntake {
                parameter,
                declaration,
            } => {
                assert!(core::ptr::eq(
                    parameter,
                    state
                        .parameters()
                        .parameter(parameter.id())
                        .expect("metadata")
                ));
                if let Some(length) = FormulaLengthInput::from_declaration(declaration) {
                    assert!(core::ptr::eq(
                        length.canonical_declaration(),
                        state.inputs().length(id(12)).expect("Derived")
                    ));
                    assert!(length.read_authored().is_err());
                } else {
                    let actual = state.inputs().scalar(id(2003)).expect("Unknown");
                    let scalar = FormulaScalarInput::from_declaration(declaration, actual)
                        .expect("general Length");
                    assert!(core::ptr::eq(scalar.canonical_declaration(), actual));
                    assert!(scalar.read_authored().is_err());
                }
                assert_eq!(view.profile_validation(), None);
            }
            Source::ProfileIntake(reference) => {
                let IntakeAmount::Profile(actual) = intake else {
                    panic!("profile")
                };
                assert!(core::ptr::eq(reference, actual));
                assert_eq!(
                    view.profile_validation(),
                    Some(ProfileBindingValidation::DeferredToG4)
                );
            }
            Source::ClosingOperation(operation) => {
                assert!(core::ptr::eq(operation, closing));
                assert!(state.inputs().scalar(*operation).is_some()); // Numeric alias never changes its role.
                assert_eq!(view.profile_validation(), None);
            }
            _ => panic!("only intake and closing roles belong to this query"),
        }
        let debug = format!("{view:?}");
        assert!(
            !debug.contains("EntityId") && !debug.contains("Length(") && !debug.contains("Unknown")
        );
    }
    assert_eq!((state.inputs(), &inventory), (&saved.0, &saved.1));
}

#[test]
fn bare_numeric_ids_never_substitute_for_actual_formula_parameter_metadata() {
    for kind in [
        ConstructionKind::Dart,
        ConstructionKind::Tuck,
        ConstructionKind::Pleat,
    ] {
        for identity in [10, 2000] {
            let (state, inventory) =
                with_folds(fixture(), vec![(kind, IntakeAmount::Formula(id(identity)))]);
            assert_eq!(
                state
                    .construction_intake_declarations(&inventory)
                    .expect_err("actual metadata required"),
                Error::MissingFormulaParameter {
                    construction: id(110000),
                    position: ConstructionInventoryPosition { kind, index: 0 },
                    role: Role::Intake,
                    parameter: id(identity)
                }
            );
        }
    }
}

#[test]
fn later_wrong_kind_retains_original_semantic_position_without_partial_views() {
    let mut definition = fixture();
    definition.parameters = DesignParameters::new(
        vec![DesignParameterDefinition {
            id: id(5001),
            token: MachineToken::new("intake_count").expect("token"),
            target: DesignParameterTarget::Scalar {
                declaration: id(2000),
                kind: ScalarInputKind::Count,
            },
        }],
        definition.structure.inputs(),
    )
    .expect("actual Count metadata");
    let (state, inventory) = with_folds(
        definition,
        vec![
            (ConstructionKind::Dart, explicit()),
            (ConstructionKind::Pleat, explicit()),
            (ConstructionKind::Pleat, IntakeAmount::Formula(id(5001))),
        ],
    );
    let error = state
        .construction_intake_declarations(&inventory)
        .expect_err("Length required");
    assert_eq!(
        error,
        Error::FormulaKind {
            construction: id(110002),
            position: ConstructionInventoryPosition {
                kind: ConstructionKind::Pleat,
                index: 1
            },
            role: Role::Intake,
            parameter: id(5001),
            expected: FormulaKind::Length,
            actual: FormulaKind::Count
        }
    );
    assert!(std::error::Error::source(&error).is_none());
}

#[test]
fn metadata_borrows_do_not_certify_current_piece_sources_or_infer_empty_intake() {
    let (state, inventory) = with_folds(fixture(), vec![(ConstructionKind::Dart, explicit())]);
    let mut definition = state.definition().clone();
    let mut structure = definition.structure.definition().clone();
    structure.pieces = PieceInventory::new(vec![]).expect("explicit current removal");
    structure.cut_plan = CutPlan::new(vec![], &[]).expect("empty current copies");
    definition.structure = DesignStructure::new(structure).expect("held topology preserved");
    let current =
        DesignInputs::new(definition).expect("current input scope without original Piece");
    assert_eq!(
        current
            .construction_intake_declarations(&inventory)
            .expect("metadata scope")
            .len(),
        2
    );
    let empty = ConstructionInventory::new(vec![], vec![], vec![], vec![], vec![], vec![])
        .expect("empty draft scope");
    assert!(current
        .construction_intake_declarations(&empty)
        .expect("empty role list")
        .is_empty());
    let graph = SewingGraph::new(
        SewingGraphDefinition {
            id: id(999000),
            spans: vec![],
        },
        current.structure().cut_plan(),
        &[],
        &[],
        current.structure().topology(),
    )
    .expect("empty graph context");
    assert!(
        matches!(inventory.validate_current(&[],current.structure().cut_plan(),&graph,&[],current.structure().topology()),Err(ConstructionInventoryError::MissingPiece {piece,..}) if piece==id(95000))
    );
}
