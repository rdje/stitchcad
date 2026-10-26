//! Structural ownership composes actual records without certifying a complete Design or release.
#![allow(clippy::expect_used)]
use sc_core::{
    command::DesignRevision,
    design::{
        DesignStructure, DesignStructureDefinition, DesignStructureError as Error,
        DesignStructurePosition as Position,
    },
    ontology::{
        CopyOrientation, CutCopyDefinition, CutPlan, CutPlanError, CuttingSide,
        DeterministicIdGenerator, DirectedEdge, Direction, EntityId, IdentityLedger, LabelText,
        MaterialAssignment, Mirroring, Param, Piece, PieceDefinition, PieceInventory, Rational,
        Resolution, SplitSide,
    },
    value::{
        CanonicalInputs, LengthDeclaration, LengthDeclarationDefinition, LengthState,
        ScalarInputDeclaration, ScalarInputDeclarationDefinition, ScalarInputKind,
        ScalarInputState,
    },
};

fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn inputs(length: u128, scalar: u128) -> CanonicalInputs {
    CanonicalInputs::new(
        vec![LengthDeclaration::new(LengthDeclarationDefinition {
            id: id(length),
            source: id(10),
            state: LengthState::Derived { formula: id(1_000) },
        })
        .expect("explicit derived length")],
        vec![
            ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
                id: id(scalar),
                source: id(30),
                kind: ScalarInputKind::Length,
                state: ScalarInputState::Unknown {
                    observation: id(10),
                },
            })
            .expect("explicit unknown scalar"),
        ],
    )
    .expect("distinct canonical fixture inputs")
}
fn fixture() -> DesignStructureDefinition {
    let mut topology = IdentityLedger::new();
    let mut ids = DeterministicIdGenerator::from_start(1_000);
    let edges = topology
        .declare_edges(&mut ids, 2)
        .expect("explicit fixture edges");
    let pieces: Vec<_> = [20, 10]
        .into_iter()
        .zip(edges)
        .map(|(identity, edge)| {
            Piece::new(
                PieceDefinition {
                    id: id(identity),
                    boundary: vec![DirectedEdge {
                        edge,
                        direction: Direction::Original,
                    }],
                    holes: vec![],
                    construction_lines: vec![],
                    quantity: 1,
                    mirroring: Mirroring::Single,
                    cut_on_fold: false,
                    fold_edges: vec![],
                    cutting_side: CuttingSide::WrongSide,
                    material: MaterialAssignment::Unresolved(
                        "awaiting explicit material selection".into(),
                    ),
                    layer_index: -2,
                    label: LabelText {
                        name: "fixture".into(),
                        size: "authored".into(),
                        fabric: "unassigned".into(),
                        colorway: "undetermined".into(),
                    },
                },
                &topology,
            )
            .expect("valid born Piece")
        })
        .collect();
    let cut_plan = CutPlan::new(
        vec![
            CutCopyDefinition {
                id: id(30),
                piece: id(10),
                orientation: CopyOrientation::Authored,
            },
            CutCopyDefinition {
                id: id(31),
                piece: id(20),
                orientation: CopyOrientation::Authored,
            },
        ],
        &pieces,
    )
    .expect("complete explicit copies");
    let edge = pieces
        .first()
        .expect("first Piece")
        .edges()
        .next()
        .expect("held edge")
        .edge;
    let halfway = Param::new(Rational::new(1, 2).expect("exact half")).expect("in interval");
    topology
        .register(id(20), edge, halfway)
        .expect("authored registration");
    topology
        .split(&mut ids, edge, halfway)
        .expect("historical held edge");
    topology
        .state_split_choice(id(20), edge, halfway, SplitSide::Second)
        .expect("explicit side");
    DesignStructureDefinition {
        id: id(1),
        revision: DesignRevision::from_counter(u128::MAX).successor(),
        inputs: inputs(40, 50),
        pieces: PieceInventory::new(pieces).expect("complete Pieces"),
        cut_plan,
        topology,
    }
}

#[test]
fn exact_components_revision_history_and_choices_survive_with_canonical_borrows() {
    let supplied = fixture();
    let original = supplied.clone();
    let state = DesignStructure::new(supplied).unwrap();
    assert_eq!(state.definition(), &original);
    assert_eq!(state.id(), id(1));
    assert_eq!(
        state.revision().to_string(),
        "340282366920938463463374607431768211456"
    );
    assert!(core::ptr::eq(state.inputs(), &state.definition().inputs));
    assert!(core::ptr::eq(state.pieces(), &state.definition().pieces));
    assert!(core::ptr::eq(
        state.cut_plan(),
        &state.definition().cut_plan
    ));
    assert!(core::ptr::eq(
        state.topology(),
        &state.definition().topology
    ));
    assert_eq!(state.topology().registrations().count(), 1);
    let registration = state.topology().registrations().next().unwrap();
    assert_eq!(registration.split_choice(), Some(SplitSide::Second));
    let mut detached = state.definition().clone();
    detached.topology = IdentityLedger::new();
    assert_eq!(state.definition(), &original);
    assert_ne!(detached.topology, *state.topology());
}

#[test]
fn cross_role_identity_checks_preserve_actual_roles_and_original_positions() {
    for (length, scalar, first, duplicate) in [
        (1, 50, Position::Design, Position::Length { index: 0 }),
        (
            10,
            50,
            Position::Length { index: 0 },
            Position::Piece { index: 1 },
        ),
        (
            40,
            30,
            Position::Scalar { index: 0 },
            Position::CutCopy { index: 0 },
        ),
        (
            1_000,
            50,
            Position::Length { index: 0 },
            Position::Operation { index: 0 },
        ),
    ] {
        let mut definition = fixture();
        definition.inputs = inputs(length, scalar);
        let identity = match duplicate {
            Position::CutCopy { .. } => id(scalar),
            _ => id(length),
        };
        assert_eq!(
            DesignStructure::new(definition),
            Err(Error::DuplicateIdentity {
                identity,
                first,
                duplicate,
            })
        );
    }
}

#[test]
fn design_identity_cannot_alias_a_retired_creating_operation() {
    let mut definition = fixture();
    definition.id = id(1_000);
    assert_eq!(
        DesignStructure::new(definition),
        Err(Error::DuplicateIdentity {
            identity: id(1_000),
            first: Position::Design,
            duplicate: Position::Operation { index: 0 },
        })
    );
}

#[test]
fn copies_validated_for_another_quantity_are_refused_against_actual_pieces() {
    let mut definition = fixture();
    let mut pieces = definition.pieces.pieces().to_vec();
    let mut changed = pieces.last().unwrap().definition().clone();
    changed.quantity = 2;
    pieces.pop();
    pieces.push(Piece::new(changed, &definition.topology).expect("other Piece edge still live"));
    definition.pieces = PieceInventory::new(pieces).unwrap();
    let error = DesignStructure::new(definition).unwrap_err();
    assert_eq!(
        error,
        Error::CutPlan(CutPlanError::QuantityMismatch {
            piece: id(10),
            expected: 2,
            actual: 1
        })
    );
    assert!(std::error::Error::source(&error).is_some());
}

#[test]
fn missing_current_piece_does_not_inherit_the_old_plans_construction_proof() {
    let mut definition = fixture();
    definition.pieces = PieceInventory::new(vec![]).unwrap();
    assert_eq!(
        DesignStructure::new(definition),
        Err(Error::CutPlan(CutPlanError::MissingPiece {
            copy: id(30),
            piece: id(10),
        }))
    );
}

#[test]
fn reflected_copies_cannot_keep_a_previous_mirrored_piece_policy() {
    let mut definition = fixture();
    let mut old_pieces = definition.pieces.pieces().to_vec();
    let mut mirrored = old_pieces.last().unwrap().definition().clone();
    mirrored.quantity = 2;
    mirrored.mirroring = Mirroring::MirroredPairs;
    old_pieces.pop();
    old_pieces.push(Piece::new(mirrored, &definition.topology).unwrap());
    definition.cut_plan = CutPlan::new(
        vec![
            CutCopyDefinition {
                id: id(30),
                piece: id(10),
                orientation: CopyOrientation::Authored,
            },
            CutCopyDefinition {
                id: id(32),
                piece: id(10),
                orientation: CopyOrientation::Reflected,
            },
            CutCopyDefinition {
                id: id(31),
                piece: id(20),
                orientation: CopyOrientation::Authored,
            },
        ],
        &old_pieces,
    )
    .unwrap();
    assert_eq!(
        DesignStructure::new(definition),
        Err(Error::CutPlan(CutPlanError::UnexpectedReflection {
            copy: id(32),
            piece: id(10),
        }))
    );
}

#[test]
fn deleted_held_references_remain_inspectable_without_registration_or_reattachment() {
    let mut definition = fixture();
    let piece = definition.pieces.piece(id(10)).unwrap();
    let held = piece.edges().next().unwrap().edge;
    let original = piece.clone();
    let mut ids = DeterministicIdGenerator::from_start(2_000);
    definition.topology.delete(&mut ids, held).unwrap();
    let ledger = definition.topology.clone();
    let state = DesignStructure::new(definition).unwrap();
    assert_eq!(state.topology(), &ledger);
    assert_eq!(state.pieces().piece(id(10)), Some(&original));
    assert!(matches!(
        state.topology().resolve(held, Param::START),
        Resolution::Unresolved(_)
    ));
    assert!(state
        .pieces()
        .piece(id(10))
        .unwrap()
        .range_resolutions(state.topology())
        .all(|(_, result)| !result.has_full_coverage()));
    assert_eq!(state.topology().registrations().count(), 1);
}

#[test]
fn explicitly_empty_structural_drafts_mint_no_content_and_advance_no_revision() {
    let definition = DesignStructureDefinition {
        id: id(0),
        revision: DesignRevision::initial(),
        inputs: CanonicalInputs::new(vec![], vec![]).unwrap(),
        pieces: PieceInventory::new(vec![]).unwrap(),
        cut_plan: CutPlan::new(vec![], &[]).unwrap(),
        topology: IdentityLedger::new(),
    };
    let state = DesignStructure::new(definition.clone()).unwrap();
    assert_eq!(state.definition(), &definition);
    assert_eq!(state.revision(), &DesignRevision::initial());
    assert_eq!(state.topology().registrations().count(), 0);
}
