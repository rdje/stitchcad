//! Collection-level counterpart to the reference skirt's explicit cut-once L/R members.
use sc_core::ontology::{
    CuttingSide, DeterministicIdGenerator, DirectedEdge, Direction, EntityId, GeometricValidation,
    Handedness, IdentityLedger, LabelText, MaterialAssignment, Mirroring, Piece, PieceDefinition,
    PieceInventory, PieceInventoryError,
};

fn id(value: u128) -> EntityId {
    EntityId::from_bits(value)
}

fn member(handedness: Handedness, companion: u128) -> Mirroring {
    Mirroring::PairMember {
        handedness,
        companion: id(companion),
    }
}

#[allow(clippy::expect_used)]
fn pieces(rows: &[(u128, u32, Mirroring)]) -> Vec<Piece> {
    let mut ledger = IdentityLedger::new();
    let mut ids = DeterministicIdGenerator::from_start(1_000);
    rows.iter()
        .map(|&(identity, quantity, mirroring)| {
            let edge = ledger
                .declare_edges(&mut ids, 1)
                .expect("one fixture edge declaration")
                .into_iter()
                .next()
                .expect("one explicitly requested fixture edge");
            Piece::new(
                PieceDefinition {
                    id: id(identity),
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
                    cutting_side: CuttingSide::WrongSide,
                    material: MaterialAssignment::Unresolved(
                        "retain authored material intent".into(),
                    ),
                    layer_index: -3,
                    label: LabelText {
                        name: "a label supplies neither hand nor partner".into(),
                        size: "authored size".into(),
                        fabric: "awaiting selection".into(),
                        colorway: "undetermined".into(),
                    },
                },
                &ledger,
            )
            .expect("valid local fixture Piece")
        })
        .collect()
}

#[test]
fn valid_members_keep_every_piece_in_authored_order_with_canonical_borrows() {
    for rows in [
        vec![
            (20, 1, member(Handedness::Left, 10)),
            (30, 3, Mirroring::Single),
            (10, 1, member(Handedness::Right, 20)),
        ],
        vec![
            (10, 1, member(Handedness::Right, 20)),
            (20, 1, member(Handedness::Left, 10)),
        ],
    ] {
        let supplied = pieces(&rows);
        let original = supplied.clone();
        let inventory = PieceInventory::new(supplied).unwrap();
        assert_eq!(inventory.pieces(), original);
        for piece in inventory.pieces() {
            assert!(core::ptr::eq(inventory.piece(piece.id()).unwrap(), piece));
            assert_eq!(
                piece.geometric_validation(),
                GeometricValidation::DeferredToG2
            );
        }
        assert_eq!(inventory.piece(id(999)), None);
    }
}

#[test]
fn empty_and_nonmember_inventories_invent_no_companions_or_copies() {
    assert!(PieceInventory::new(vec![]).unwrap().pieces().is_empty());
    let supplied = pieces(&[(3, 3, Mirroring::Single), (2, 4, Mirroring::MirroredPairs)]);
    let original = supplied.clone();
    assert_eq!(PieceInventory::new(supplied).unwrap().pieces(), original);
}

#[test]
fn duplicate_identity_reports_original_positions_before_companion_checks() {
    let supplied = pieces(&[(10, 1, member(Handedness::Right, 20))]);
    let repeated = vec![
        supplied.first().unwrap().clone(),
        pieces(&[(30, 1, Mirroring::Single)])
            .into_iter()
            .next()
            .unwrap(),
        supplied.first().unwrap().clone(),
    ];
    assert_eq!(
        PieceInventory::new(repeated),
        Err(PieceInventoryError::DuplicatePiece {
            piece: id(10),
            first: 0,
            duplicate: 2,
        })
    );
}

#[test]
fn missing_companion_preserves_the_actual_authored_target() {
    assert_eq!(
        PieceInventory::new(pieces(&[(10, 1, member(Handedness::Right, 20))])),
        Err(PieceInventoryError::MissingCompanion {
            piece: id(10),
            companion: id(20)
        })
    );
}

#[test]
fn companion_cut_mode_must_be_explicit_separate_member_metadata() {
    for (quantity, actual) in [(1, Mirroring::Single), (2, Mirroring::MirroredPairs)] {
        assert_eq!(
            PieceInventory::new(pieces(&[
                (10, 1, member(Handedness::Right, 20)),
                (20, quantity, actual),
            ])),
            Err(PieceInventoryError::CompanionNotPairMember {
                piece: id(10),
                companion: id(20),
                actual,
            })
        );
    }
}

#[test]
fn opposite_hands_and_equal_quantities_cannot_supply_a_missing_reciprocal_link() {
    assert_eq!(
        PieceInventory::new(pieces(&[
            (10, 1, member(Handedness::Right, 20)),
            (20, 1, member(Handedness::Left, 30)),
            (30, 1, Mirroring::Single),
        ])),
        Err(PieceInventoryError::NonReciprocalCompanion {
            piece: id(10),
            companion: id(20),
            actual: id(30),
        })
    );
}

#[test]
fn explicit_equal_hands_are_refused_without_reading_print_labels() {
    for handedness in [Handedness::Left, Handedness::Right] {
        assert_eq!(
            PieceInventory::new(pieces(&[
                (20, 1, member(handedness, 10)),
                (10, 1, member(handedness, 20)),
            ])),
            Err(PieceInventoryError::SameHandedness {
                piece: id(20),
                companion: id(10),
                handedness,
            })
        );
    }
}

#[test]
fn equal_quantities_are_per_member_and_may_be_odd() {
    let mut pair = [
        (10, 3, member(Handedness::Right, 20)),
        (20, 3, member(Handedness::Left, 10)),
    ];
    assert!(PieceInventory::new(pieces(&pair)).is_ok());
    for (quantity, companion_quantity) in [(1, 3), (3, 1)] {
        pair[0].1 = quantity;
        pair[1].1 = companion_quantity;
        assert_eq!(
            PieceInventory::new(pieces(&pair)),
            Err(PieceInventoryError::UnequalQuantities {
                piece: id(10),
                companion: id(20),
                quantity,
                companion_quantity,
            })
        );
    }
}
