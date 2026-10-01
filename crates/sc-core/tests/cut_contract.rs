//! Physical-copy identity and complete cut-plan contracts, independent of geometric transforms.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

use sc_core::ontology::{
    CopyOrientation, CutCopyDefinition, CutPlan, CutPlanError, CuttingSide,
    DeterministicIdGenerator, DirectedEdge, Direction, EntityId, GeometricValidation, Handedness,
    IdGenerator, IdentityLedger, LabelText, MaterialAssignment, Mirroring, Piece, PieceDefinition,
};

fn piece(id: u128, quantity: u32, mirroring: Mirroring) -> Piece {
    let mut ledger = IdentityLedger::new();
    let mut ids = DeterministicIdGenerator::new();
    let edge = *ledger.declare_edges(&mut ids, 1).unwrap().first().unwrap();
    Piece::new(
        PieceDefinition {
            id: EntityId::from_bits(id),
            boundary: vec![DirectedEdge {
                edge,
                direction: Direction::Original,
            }],
            holes: vec![],
            construction_lines: vec![],
            quantity,
            mirroring,
            cut_on_fold: false,
            fold_edges: vec![],
            cutting_side: CuttingSide::Face,
            material: MaterialAssignment::Unresolved("selection pending".into()),
            layer_index: 0,
            label: LabelText {
                name: "test piece".into(),
                size: "M".into(),
                fabric: "woven".into(),
                colorway: "plain".into(),
            },
        },
        &ledger,
    )
    .unwrap()
}

fn copy(id: u128, piece: &Piece, orientation: CopyOrientation) -> CutCopyDefinition {
    CutCopyDefinition {
        id: EntityId::from_bits(id),
        piece: piece.id(),
        orientation,
    }
}

#[test]
fn copies_retain_explicit_identity_when_reordered_and_geometry_stays_deferred() {
    let piece = piece(1, 2, Mirroring::Single);
    let a = copy(10, &piece, CopyOrientation::Authored);
    let b = copy(11, &piece, CopyOrientation::Authored);
    let pieces = [piece];
    let first = CutPlan::new(vec![a, b], &pieces).unwrap();
    let reversed = CutPlan::new(vec![b, a], &pieces).unwrap();
    assert_ne!(first.copies(), reversed.copies());
    for input in [a, b] {
        let retained = first.copy(input.id).unwrap();
        assert_eq!(retained, reversed.copy(input.id).unwrap());
        assert_eq!(retained.definition(), &input);
        assert_eq!(retained.piece(), pieces.first().unwrap().id());
        assert_eq!(retained.orientation(), CopyOrientation::Authored);
    }
    assert_eq!(first.copies_for_piece(a.piece).count(), 2);
    assert!(first.copy(EntityId::from_bits(99)).is_none());
    assert_eq!(
        first.geometric_validation(),
        GeometricValidation::DeferredToG2
    );
    let mut editable = first
        .copies()
        .iter()
        .map(|copy| *copy.definition())
        .collect::<Vec<_>>();
    editable.clear();
    assert_eq!(first.copies().len(), 2);
}

#[test]
fn duplicate_pattern_and_copy_ids_and_cross_namespace_collisions_are_refused() {
    let piece = piece(1, 2, Mirroring::Single);
    let a = copy(10, &piece, CopyOrientation::Authored);
    let b = copy(11, &piece, CopyOrientation::Authored);
    assert_eq!(
        CutPlan::new(vec![a, b], &[piece.clone(), piece.clone()]),
        Err(CutPlanError::DuplicatePiece { piece: piece.id() })
    );
    assert_eq!(
        CutPlan::new(vec![a, a], std::slice::from_ref(&piece)),
        Err(CutPlanError::DuplicateCopy { copy: a.id })
    );
    let collision = CutCopyDefinition {
        id: piece.id(),
        ..a
    };
    assert_eq!(
        CutPlan::new(vec![collision, b], &[piece]),
        Err(CutPlanError::IdentityCollision { id: collision.id })
    );
}

#[test]
fn unknown_pattern_names_the_physical_copy_and_missing_target() {
    let piece = piece(1, 1, Mirroring::Single);
    let input = CutCopyDefinition {
        piece: EntityId::from_bits(999),
        ..copy(10, &piece, CopyOrientation::Authored)
    };
    assert_eq!(
        CutPlan::new(vec![input], &[piece]),
        Err(CutPlanError::MissingPiece {
            copy: input.id,
            piece: input.piece
        })
    );
}

#[test]
fn exact_quantity_is_required_for_every_piece_including_unmentioned_pieces() {
    let piece = piece(1, 2, Mirroring::Single);
    let a = copy(10, &piece, CopyOrientation::Authored);
    let b = copy(11, &piece, CopyOrientation::Authored);
    let c = copy(12, &piece, CopyOrientation::Authored);
    for (inputs, actual) in [(vec![], 0), (vec![a], 1), (vec![a, b, c], 3)] {
        assert_eq!(
            CutPlan::new(inputs, std::slice::from_ref(&piece)),
            Err(CutPlanError::QuantityMismatch {
                piece: piece.id(),
                expected: 2,
                actual
            })
        );
    }
    let other = piece_with_one_copy();
    assert_eq!(
        CutPlan::new(vec![a, b], &[piece, other.clone()]),
        Err(CutPlanError::QuantityMismatch {
            piece: other.id(),
            expected: 1,
            actual: 0
        })
    );
}

fn piece_with_one_copy() -> Piece {
    piece(2, 1, Mirroring::Single)
}

#[test]
fn mirrored_pairs_require_equal_orientations_without_inventing_left_right_geometry() {
    let piece = piece(1, 4, Mirroring::MirroredPairs);
    let copies = (10..14)
        .map(|id| copy(id, &piece, CopyOrientation::Authored))
        .collect::<Vec<_>>();
    assert_eq!(
        CutPlan::new(copies.clone(), std::slice::from_ref(&piece)),
        Err(CutPlanError::UnbalancedPairs {
            piece: piece.id(),
            authored: 4,
            reflected: 0
        })
    );
    let mut balanced = copies;
    for copy in balanced.iter_mut().skip(2) {
        copy.orientation = CopyOrientation::Reflected;
    }
    let plan = CutPlan::new(balanced, &[piece]).unwrap();
    assert_eq!(
        plan.copies()
            .iter()
            .filter(|copy| copy.orientation() == CopyOrientation::Reflected)
            .count(),
        2
    );
}

#[test]
fn single_and_separate_handed_members_never_request_an_additional_reflection() {
    for mirroring in [
        Mirroring::Single,
        Mirroring::PairMember {
            handedness: Handedness::Left,
            companion: EntityId::from_bits(2),
        },
    ] {
        let piece = piece(1, 1, mirroring);
        let input = copy(10, &piece, CopyOrientation::Reflected);
        assert_eq!(
            CutPlan::new(vec![input], std::slice::from_ref(&piece)),
            Err(CutPlanError::UnexpectedReflection {
                copy: input.id,
                piece: piece.id()
            })
        );
        assert!(CutPlan::new(vec![copy(10, &piece, CopyOrientation::Authored)], &[piece]).is_ok());
    }
}

#[test]
fn explicit_new_copy_replaces_a_removed_one_without_transferring_its_identity() {
    let piece = piece(1, 2, Mirroring::Single);
    let a = copy(10, &piece, CopyOrientation::Authored);
    let b = copy(11, &piece, CopyOrientation::Authored);
    let c = copy(12, &piece, CopyOrientation::Authored);
    let before = CutPlan::new(vec![a, b], std::slice::from_ref(&piece)).unwrap();
    let after = CutPlan::new(vec![a, c], &[piece]).unwrap();
    assert_eq!(before.copy(a.id), after.copy(a.id));
    assert!(after.copy(b.id).is_none());
    assert!(after.copy(c.id).is_some());
    // Sewing graph validation must name the missing b, rather than silently replacing b with c.
}

#[test]
fn empty_piece_collection_has_exactly_one_complete_empty_plan() {
    assert!(CutPlan::new(vec![], &[]).unwrap().copies().is_empty());
    let input = CutCopyDefinition {
        id: EntityId::from_bits(1),
        piece: EntityId::from_bits(2),
        orientation: CopyOrientation::Authored,
    };
    assert_eq!(
        CutPlan::new(vec![input], &[]),
        Err(CutPlanError::MissingPiece {
            copy: input.id,
            piece: input.piece
        })
    );
}

#[test]
fn injected_identity_sequences_replay_copy_definitions_byte_for_byte() {
    let piece = piece(100, 2, Mirroring::Single);
    let generate = || {
        let mut ids = DeterministicIdGenerator::new();
        vec![
            CutCopyDefinition {
                id: ids.next_id(),
                piece: piece.id(),
                orientation: CopyOrientation::Authored,
            },
            CutCopyDefinition {
                id: ids.next_id(),
                piece: piece.id(),
                orientation: CopyOrientation::Authored,
            },
        ]
    };
    let a = CutPlan::new(generate(), std::slice::from_ref(&piece)).unwrap();
    let b = CutPlan::new(generate(), &[piece]).unwrap();
    assert_eq!(a, b);
    assert_eq!(format!("{a:?}"), format!("{b:?}"));
}
