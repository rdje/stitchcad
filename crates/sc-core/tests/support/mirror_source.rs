//! Actual mirror source setup;original0719 helper bodies retained.
use sc_core::{
    construction::{MirrorAxis, MirrorPieceCopy},
    ontology::*,
};
pub fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
pub fn input(operation: u128, piece: u128, axis: MirrorAxis) -> MirrorPieceCopy {
    MirrorPieceCopy::new(id(operation), id(piece), axis).expect("independent original inputs")
}
pub fn axis() -> MirrorAxis {
    MirrorAxis::ThroughPoints {
        start: PointRef::new(id(90), LocalTag::new(7)),
        end: PointRef::new(id(91), LocalTag::new(3)),
    }
}
pub fn fixture() -> (IdentityLedger, Piece) {
    let mut ledger = IdentityLedger::new();
    let mut ids = DeterministicIdGenerator::from_start(40);
    let mut edge = |direction| DirectedEdge {
        edge: *ledger
            .declare_edges(&mut ids, 1)
            .expect("live source")
            .first()
            .expect("declared edge"),
        direction,
    };
    let first = edge(Direction::Reversed);
    let second = edge(Direction::Original);
    let third = edge(Direction::Reversed);
    let fourth = edge(Direction::Original);
    let piece = Piece::new(
        PieceDefinition {
            id: id(100),
            boundary: vec![first, second],
            holes: vec![vec![third], vec![fourth]],
            construction_lines: vec![
                DirectedEdge {
                    direction: Direction::Reversed,
                    ..second
                },
                second,
                DirectedEdge {
                    direction: Direction::Reversed,
                    ..fourth
                },
            ],
            quantity: 4,
            mirroring: Mirroring::MirroredPairs,
            cut_on_fold: true,
            fold_edges: vec![first.edge],
            cutting_side: CuttingSide::WrongSide,
            material: MaterialAssignment::Unresolved("  original material reason  ".into()),
            layer_index: -3,
            label: LabelText {
                name: "Asymmetric source".into(),
                size: "M".into(),
                fabric: "woven".into(),
                colorway: "blue".into(),
            },
        },
        &ledger,
    )
    .expect("structural original; no reflection geometry");
    (ledger, piece)
}
