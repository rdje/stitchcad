//! Actual source metadata across the core/higher-source port; no geometry or Profile resolver.
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
use sc_units::{Angle, Length};

fn definition() -> DesignInputsDefinition<DesignSources> {
    let mut definition = fixture();
    let mut structure = definition.structure.definition().clone();
    let mut scalars = structure.inputs.scalars().to_vec();
    for (identity, kind) in [
        (2003, ScalarInputKind::Length),
        (2006, ScalarInputKind::Angle),
    ] {
        scalars.push(
            ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
                id: id(identity),
                source: id(2010),
                kind,
                state: ScalarInputState::Unknown {
                    observation: id(2011),
                },
            })
            .expect("actual unknown typed input"),
        );
    }
    structure.inputs = CanonicalInputs::new(structure.inputs.lengths().to_vec(), scalars)
        .expect("one canonical owner");
    definition.structure = DesignStructure::new(structure).expect("actual structure");
    definition.parameters = DesignParameters::new(
        vec![
            DesignParameterDefinition {
                id: id(5001),
                token: MachineToken::new("width_length").expect("token"),
                target: DesignParameterTarget::Length(id(12)),
            },
            DesignParameterDefinition {
                id: id(5002),
                token: MachineToken::new("width_general").expect("token"),
                target: DesignParameterTarget::Scalar {
                    declaration: id(2003),
                    kind: ScalarInputKind::Length,
                },
            },
            DesignParameterDefinition {
                id: id(5003),
                token: MachineToken::new("grain_angle").expect("token"),
                target: DesignParameterTarget::Scalar {
                    declaration: id(2006),
                    kind: ScalarInputKind::Angle,
                },
            },
        ],
        definition.structure.inputs(),
    )
    .expect("actual parameter bindings");
    definition
}
fn inventory(
    definition: DesignInputsDefinition<DesignSources>,
    angles: Vec<Option<GrainAngle>>,
    widths: Vec<AllowanceWidth>,
) -> (DesignInputs<DesignSources>, ConstructionInventory) {
    let (state, _) = with_sewing_graph(definition, vec![]);
    let piece = state
        .structure()
        .pieces()
        .pieces()
        .first()
        .expect("actual Piece");
    let edge = piece.edges().next().expect("actual edge").edge;
    let range = DirectedRange {
        range: EdgeRange::whole(edge),
        direction: Direction::Reversed,
    };
    let grains = angles
        .into_iter()
        .enumerate()
        .map(|(index, angle)| {
            Grainline::new(
                GrainlineDefinition {
                    id: id(112000 + u128::try_from(index).expect("ordinal")),
                    arrow: range,
                    alignment: angle.map_or(GrainAlignment::ParallelTo(range), |angle| {
                        GrainAlignment::AtAngle {
                            reference: range,
                            angle,
                        }
                    }),
                    stripe: Some(range),
                    plaid: Some(range),
                },
                piece,
                state.structure().topology(),
            )
            .expect("actual grain")
        })
        .collect();
    let allowances = widths
        .into_iter()
        .enumerate()
        .map(|(index, width)| {
            SeamAllowance::new(
                SeamAllowanceDefinition {
                    id: id(113000 + u128::try_from(index).expect("ordinal")),
                    edge,
                    width,
                    corner: CornerTreatment::Envelope,
                    inclusion: ProfileParameterRef::new(id(2000)),
                },
                piece,
                state.structure().topology(),
            )
            .expect("actual allowance")
        })
        .collect();
    let inventory = ConstructionInventory::new(vec![], vec![], vec![], vec![], grains, allowances)
        .expect("exact ordered families");
    (state, inventory)
}
fn explicit_width() -> AllowanceWidth {
    AllowanceWidth::Explicit {
        parameter: id(111000),
        value: Length::ZERO,
    }
}

#[test]
fn exact_alignment_width_and_policy_sources_borrow_original_objects_without_values_or_defaults() {
    let (state, inventory) = inventory(
        definition(),
        vec![
            None,
            Some(GrainAngle::Explicit(
                Angle::from_degrees_rational(45, 1).expect("exact bias"),
            )),
            Some(GrainAngle::Formula(id(5003))),
            Some(GrainAngle::Profile(ProfileParameterRef::new(id(10)))),
        ],
        vec![
            explicit_width(),
            AllowanceWidth::Formula(id(5001)),
            AllowanceWidth::Formula(id(5002)),
            AllowanceWidth::Profile(ProfileParameterRef::new(id(10))),
        ],
    );
    let saved = (state.inputs().clone(), inventory.clone());
    let views = state
        .grain_allowance_declarations(&inventory)
        .expect("actual kind-checked metadata");
    assert_eq!(
        views
            .iter()
            .map(|view| (view.position().kind, view.position().index, view.role()))
            .collect::<Vec<_>>(),
        vec![
            (ConstructionKind::Grainline, 0, Role::GrainAlignment),
            (ConstructionKind::Grainline, 1, Role::GrainAlignment),
            (ConstructionKind::Grainline, 2, Role::GrainAlignment),
            (ConstructionKind::Grainline, 3, Role::GrainAlignment),
            (ConstructionKind::SeamAllowance, 0, Role::AllowanceWidth),
            (ConstructionKind::SeamAllowance, 0, Role::AllowanceInclusion),
            (ConstructionKind::SeamAllowance, 1, Role::AllowanceWidth),
            (ConstructionKind::SeamAllowance, 1, Role::AllowanceInclusion),
            (ConstructionKind::SeamAllowance, 2, Role::AllowanceWidth),
            (ConstructionKind::SeamAllowance, 2, Role::AllowanceInclusion),
            (ConstructionKind::SeamAllowance, 3, Role::AllowanceWidth),
            (ConstructionKind::SeamAllowance, 3, Role::AllowanceInclusion),
        ]
    );
    for view in views {
        assert!(core::ptr::eq(view.canonical_inputs(), state.inputs()));
        match view.construction() {
            ConstructionRef::Grainline(actual) => assert!(core::ptr::eq(
                actual,
                inventory
                    .grainlines()
                    .get(view.position().index)
                    .expect("original grain")
            )),
            ConstructionRef::SeamAllowance(actual) => {
                assert!(core::ptr::eq(
                    actual,
                    inventory
                        .allowances()
                        .get(view.position().index)
                        .expect("original allowance")
                ));
                assert_eq!(actual.definition().corner, CornerTreatment::Envelope);
            }
            _ => panic!("only grain/allowance"),
        }
        match view.source() {
            Source::ParallelAlignment(axis) => {
                let ConstructionRef::Grainline(grain) = view.construction() else {
                    panic!("grain")
                };
                let GrainAlignment::ParallelTo(actual) = &grain.definition().alignment else {
                    panic!("parallel")
                };
                assert!(core::ptr::eq(axis, actual));
                assert_eq!(axis.direction, Direction::Reversed);
            }
            Source::ExplicitAngle(value) => {
                let ConstructionRef::Grainline(grain) = view.construction() else {
                    panic!("grain")
                };
                let GrainAlignment::AtAngle {
                    angle: GrainAngle::Explicit(actual),
                    ..
                } = &grain.definition().alignment
                else {
                    panic!("explicit")
                };
                assert!(core::ptr::eq(value, actual));
                assert_eq!(value.as_microdegrees(), 45_000_000);
            }
            Source::FormulaAngle {
                parameter,
                declaration,
            } => {
                assert!(core::ptr::eq(
                    parameter,
                    state
                        .parameters()
                        .parameter(id(5003))
                        .expect("Angle metadata")
                ));
                assert_eq!(declaration.kind(), FormulaKind::Angle);
                let actual = state.inputs().scalar(id(2006)).expect("Unknown Angle");
                let scalar = FormulaScalarInput::from_declaration(declaration, actual)
                    .expect("actual Angle declaration");
                assert!(core::ptr::eq(scalar.canonical_declaration(), actual));
                assert!(scalar.read_authored().is_err());
            }
            Source::ProfileAngle(reference) => {
                let ConstructionRef::Grainline(grain) = view.construction() else {
                    panic!("grain")
                };
                let GrainAlignment::AtAngle {
                    angle: GrainAngle::Profile(actual),
                    ..
                } = &grain.definition().alignment
                else {
                    panic!("profile angle")
                };
                assert!(core::ptr::eq(reference, actual));
            }
            Source::ExplicitWidth { parameter, value } => {
                let ConstructionRef::SeamAllowance(allowance) = view.construction() else {
                    panic!("allowance")
                };
                let AllowanceWidth::Explicit {
                    parameter: actual,
                    value: actual_value,
                } = &allowance.definition().width
                else {
                    panic!("explicit width")
                };
                assert!(core::ptr::eq(parameter, actual) && core::ptr::eq(value, actual_value));
                assert_eq!(*value, Length::ZERO);
                assert!(state.parameters().parameter(*parameter).is_none());
            }
            Source::FormulaWidth {
                parameter,
                declaration,
            } => {
                assert!(core::ptr::eq(
                    parameter,
                    state
                        .parameters()
                        .parameter(parameter.id())
                        .expect("Length metadata")
                ));
                if let Some(length) = FormulaLengthInput::from_declaration(declaration) {
                    assert!(core::ptr::eq(
                        length.canonical_declaration(),
                        state.inputs().length(id(12)).expect("Derived Length")
                    ));
                    assert!(length.read_authored().is_err());
                } else {
                    let actual = state.inputs().scalar(id(2003)).expect("Unknown Length");
                    let scalar = FormulaScalarInput::from_declaration(declaration, actual)
                        .expect("general Length");
                    assert!(core::ptr::eq(scalar.canonical_declaration(), actual));
                    assert!(scalar.read_authored().is_err());
                }
            }
            Source::ProfileWidth(reference) => {
                let ConstructionRef::SeamAllowance(allowance) = view.construction() else {
                    panic!("allowance")
                };
                let AllowanceWidth::Profile(actual) = &allowance.definition().width else {
                    panic!("profile width")
                };
                assert!(core::ptr::eq(reference, actual));
            }
            Source::AllowanceInclusion(reference) => {
                let ConstructionRef::SeamAllowance(allowance) = view.construction() else {
                    panic!("allowance")
                };
                assert!(core::ptr::eq(reference, &allowance.definition().inclusion));
                assert!(state.inputs().scalar(reference.id()).is_some()); // Count id remains logical policy.
            }
            _ => panic!("only grain/allowance sources"),
        }
        let profile = matches!(
            view.source(),
            Source::ProfileAngle(_) | Source::ProfileWidth(_) | Source::AllowanceInclusion(_)
        );
        assert_eq!(
            view.profile_validation(),
            profile.then_some(ProfileBindingValidation::DeferredToG4)
        );
        let debug = format!("{view:?}");
        assert!(
            !debug.contains("EntityId")
                && !debug.contains("Unknown")
                && !debug.contains("45000000")
        );
    }
    assert_eq!((state.inputs(), &inventory), (&saved.0, &saved.1));
}

#[test]
fn angular_and_length_formula_roles_refuse_each_others_actual_metadata_at_original_positions() {
    for (angles, widths, kind, index, identity, role, expected, actual, construction) in [
        (
            vec![None, Some(GrainAngle::Formula(id(5001)))],
            vec![],
            ConstructionKind::Grainline,
            1,
            5001,
            Role::GrainAlignment,
            FormulaKind::Angle,
            FormulaKind::Length,
            112001,
        ),
        (
            vec![None],
            vec![explicit_width(), AllowanceWidth::Formula(id(5003))],
            ConstructionKind::SeamAllowance,
            1,
            5003,
            Role::AllowanceWidth,
            FormulaKind::Length,
            FormulaKind::Angle,
            113001,
        ),
    ] {
        let (state, inventory) = inventory(definition(), angles, widths);
        assert_eq!(
            state
                .grain_allowance_declarations(&inventory)
                .expect_err("exact kind required"),
            Error::FormulaKind {
                construction: id(construction),
                position: ConstructionInventoryPosition { kind, index },
                role,
                parameter: id(identity),
                expected,
                actual,
            }
        );
    }
}

#[test]
fn bare_numeric_records_never_become_angle_or_width_parameter_metadata() {
    for (angles, widths, kind, identity, role, construction) in [
        (
            vec![Some(GrainAngle::Formula(id(2006)))],
            vec![],
            ConstructionKind::Grainline,
            2006,
            Role::GrainAlignment,
            112000,
        ),
        (
            vec![],
            vec![AllowanceWidth::Formula(id(10))],
            ConstructionKind::SeamAllowance,
            10,
            Role::AllowanceWidth,
            113000,
        ),
    ] {
        let (state, inventory) = inventory(definition(), angles, widths);
        assert_eq!(
            state
                .grain_allowance_declarations(&inventory)
                .expect_err("actual parameter required"),
            Error::MissingFormulaParameter {
                construction: id(construction),
                position: ConstructionInventoryPosition { kind, index: 0 },
                role,
                parameter: id(identity),
            }
        );
    }
}

#[test]
fn metadata_borrowing_leaves_missing_current_piece_sources_and_empty_content_explicit() {
    let (state, inventory) = inventory(definition(), vec![None], vec![explicit_width()]);
    let mut definition = state.definition().clone();
    let mut structure = definition.structure.definition().clone();
    structure.pieces = PieceInventory::new(vec![]).expect("explicit removal");
    structure.cut_plan = CutPlan::new(vec![], &[]).expect("empty copies");
    definition.structure = DesignStructure::new(structure).expect("retained topology");
    let current = DesignInputs::new(definition).expect("valid input scope without held Piece");
    assert_eq!(
        current
            .grain_allowance_declarations(&inventory)
            .expect("metadata only")
            .len(),
        3
    );
    let empty = ConstructionInventory::new(vec![], vec![], vec![], vec![], vec![], vec![])
        .expect("empty scope");
    assert!(current
        .grain_allowance_declarations(&empty)
        .expect("explicit empty")
        .is_empty());
}
