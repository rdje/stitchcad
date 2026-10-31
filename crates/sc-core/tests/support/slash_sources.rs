//! Actual source Piece and slash fixtures shared by focused contour/reference contracts.
use sc_core::{
    construction::*, name::MachineToken, ontology::*, recipe::FormulaExpressionDocument,
};
pub fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
pub struct Fixture {
    pub ledger: IdentityLedger,
    pub piece: Piece,
    pub first: DirectedEdge,
    pub second: DirectedEdge,
}
pub fn fixture() -> Fixture {
    let mut ledger = IdentityLedger::new();
    let mut ids = DeterministicIdGenerator::new();
    let edges = ledger
        .declare_edges(&mut ids, 2)
        .expect("actual source identities");
    let first = DirectedEdge {
        edge: *edges.first().expect("boundary"),
        direction: Direction::Original,
    };
    let second = DirectedEdge {
        edge: *edges.get(1).expect("construction"),
        direction: Direction::Original,
    };
    let piece = Piece::new(
        PieceDefinition {
            id: id(100),
            boundary: vec![first],
            holes: vec![],
            construction_lines: vec![second],
            quantity: 1,
            mirroring: Mirroring::Single,
            cut_on_fold: false,
            fold_edges: vec![],
            cutting_side: CuttingSide::Face,
            material: MaterialAssignment::Unresolved("explicit pending material".into()),
            layer_index: 0,
            label: LabelText {
                name: "source".into(),
                size: "M".into(),
                fabric: "woven".into(),
                colorway: "plain".into(),
            },
        },
        &ledger,
    )
    .expect("structural source Piece");
    Fixture {
        ledger,
        piece,
        first,
        second,
    }
}
pub fn intent(f: &Fixture) -> SlashSpreadOutputIntent {
    let input = SlashSpreadConstruction::rotation(
        id(10),
        id(100),
        DirectedRange {
            range: EdgeRange::whole(f.first.edge),
            direction: Direction::Original,
        },
        SlashSpreadSide::Left,
        PointRef::new(id(60), LocalTag::FIRST),
        FormulaExpressionDocument::new("10 deg".into()).expect("angle"),
    )
    .expect("spatial inputs");
    SlashSpreadOutputIntent::new(
        input,
        SlashSpreadBoundaryPolicy::StraightBridges,
        SlashSpreadFullnessIntent::Added,
    )
}
pub fn catalog() -> EdgeOutputRoleCatalog {
    EdgeOutputRoleCatalog::new(
        id(10),
        [(9, "main"), (2, "aux")]
            .into_iter()
            .map(|(tag, key)| {
                EdgeOutputRoleDefinition::new(
                    LocalTag::new(tag),
                    MachineToken::new(key).expect("key"),
                )
            })
            .collect(),
        vec![LocalTag::new(2), LocalTag::new(9)],
    )
}
pub fn catalogue_limits() -> EdgeOutputRoleCatalogLimits {
    EdgeOutputRoleCatalogLimits::new(2, 2, 7)
}
pub fn manifest(f: &Fixture) -> SlashSpreadEdgeManifest {
    SlashSpreadEdgeManifest::new(
        id(10),
        vec![
            SlashSpreadEdgeCandidate::new(
                LocalTag::new(9),
                SlashSpreadEdgeOrigin::Source {
                    range: DirectedRange {
                        range: EdgeRange::whole(f.first.edge),
                        direction: Direction::Original,
                    },
                    motion: SlashSpreadSourceMotion::Moving,
                },
            ),
            SlashSpreadEdgeCandidate::new(
                LocalTag::new(2),
                SlashSpreadEdgeOrigin::StraightBridge {
                    end: SlashSpreadCutEnd::Upper,
                    direction: SlashSpreadBridgeDirection::MovingToStationary,
                },
            ),
        ],
    )
}
pub fn output(tag: u32, direction: Direction) -> SlashSpreadContourEdge {
    SlashSpreadContourEdge::Output {
        tag: LocalTag::new(tag),
        direction,
    }
}
pub fn contours(f: &Fixture) -> SlashSpreadContours {
    SlashSpreadContours::new(
        vec![
            SlashSpreadContourEdge::Retained(f.first),
            output(9, Direction::Reversed),
        ],
        vec![vec![output(2, Direction::Original)]],
        vec![
            output(9, Direction::Original),
            output(9, Direction::Reversed),
            SlashSpreadContourEdge::Retained(f.second),
        ],
    )
}
pub fn limits() -> SlashSpreadContourLimits {
    SlashSpreadContourLimits::new(2, 2, 0, 1, 6)
}
