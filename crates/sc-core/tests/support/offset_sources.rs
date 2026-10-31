//! Actual allowance/source fixtures shared by focused input and candidate-admission contracts.
use super::{id, parameter};
use sc_core::{
    construction::{OffsetConstruction, OffsetSide},
    ontology::*,
};
pub struct Fixture {
    pub ledger: IdentityLedger,
    pub piece: Piece,
    pub definition: SeamAllowanceDefinition,
}
pub fn fixture() -> Fixture {
    let mut ledger = IdentityLedger::new();
    let mut ids = DeterministicIdGenerator::new();
    let edges = ledger
        .declare_edges(&mut ids, 2)
        .expect("source identities");
    let directed = |edge| DirectedEdge {
        edge,
        direction: Direction::Original,
    };
    let piece = Piece::new(
        PieceDefinition {
            id: id(100),
            boundary: vec![directed(*edges.first().expect("first source"))],
            holes: vec![],
            construction_lines: vec![directed(*edges.get(1).expect("second source"))],
            quantity: 1,
            mirroring: Mirroring::Single,
            cut_on_fold: false,
            fold_edges: vec![],
            cutting_side: CuttingSide::Face,
            material: MaterialAssignment::Unresolved("test selection".into()),
            layer_index: 0,
            label: LabelText {
                name: "test".into(),
                size: "M".into(),
                fabric: "woven".into(),
                colorway: "plain".into(),
            },
        },
        &ledger,
    )
    .expect("structural Piece");
    Fixture {
        ledger,
        piece,
        definition: SeamAllowanceDefinition {
            id: id(80),
            edge: *edges.first().expect("first source"),
            width: AllowanceWidth::Formula(id(70)),
            corner: CornerTreatment::Miter,
            inclusion: ProfileParameterRef::new(id(60)),
        },
    }
}
impl Fixture {
    pub fn allowance(&self) -> SeamAllowance {
        SeamAllowance::new(self.definition.clone(), &self.piece, &self.ledger)
            .expect("actual source descriptor")
    }
    pub fn input(&self) -> OffsetConstruction {
        OffsetConstruction::new(
            id(10),
            DirectedRange {
                range: EdgeRange::new(self.definition.edge, parameter(1, 4), parameter(3, 4))
                    .expect("positive exact range"),
                direction: Direction::Reversed,
            },
            OffsetSide::Left,
            self.definition.id,
        )
        .expect("distinct source and descriptor")
    }
}
