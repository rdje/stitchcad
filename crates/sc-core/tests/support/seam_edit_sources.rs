//! Actual structural seam sources shared by focused editing-association contracts.
use super::id;
use sc_core::ontology::*;
pub struct Fixture {
    pub ledger: IdentityLedger,
    pub pieces: Vec<Piece>,
    pub plan: CutPlan,
    pub graph: SewingGraph,
    pub marks: Vec<SewingLandmark>,
}
pub fn first_span(graph: &SewingGraph) -> &SeamSpan {
    graph.spans().first().expect("canonical source span")
}
pub fn fixture(shared: bool) -> Fixture {
    let mut ledger = IdentityLedger::new();
    let mut ids = DeterministicIdGenerator::new();
    let [a, b]: [EdgeRef; 2] = ledger
        .declare_edges(&mut ids, 2)
        .expect("source edges")
        .try_into()
        .expect("two declared sources");
    let directed = |edge| DirectedEdge {
        edge,
        direction: Direction::Original,
    };
    let make_piece = |identity, boundary, quantity, mirroring| {
        Piece::new(
            PieceDefinition {
                id: identity,
                boundary,
                holes: vec![],
                construction_lines: vec![],
                quantity,
                mirroring,
                cut_on_fold: false,
                fold_edges: vec![],
                cutting_side: CuttingSide::Face,
                material: MaterialAssignment::Unresolved("test material".into()),
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
        .expect("structural Piece")
    };
    let pieces = if shared {
        vec![make_piece(
            id(100),
            vec![directed(a), directed(b)],
            2,
            Mirroring::MirroredPairs,
        )]
    } else {
        vec![
            make_piece(id(100), vec![directed(a)], 1, Mirroring::Single),
            make_piece(id(101), vec![directed(b)], 1, Mirroring::Single),
        ]
    };
    let plan = CutPlan::new(
        vec![
            CutCopyDefinition {
                id: id(201),
                piece: if shared { id(100) } else { id(101) },
                orientation: if shared {
                    CopyOrientation::Reflected
                } else {
                    CopyOrientation::Authored
                },
            },
            CutCopyDefinition {
                id: id(200),
                piece: id(100),
                orientation: CopyOrientation::Authored,
            },
        ],
        &pieces,
    )
    .expect("actual supplied copy plan");
    let marks = vec![SewingLandmark::Turn(
        TurnPoint::new(
            id(401),
            EdgeAnchor {
                edge: a,
                param: Param::START,
            },
            pieces.first().expect("source Piece"),
            &ledger,
        )
        .expect("actual semantic stop"),
    )];
    let graph = SewingGraph::new(
        SewingGraphDefinition {
            id: id(300),
            spans: vec![SeamSpanDefinition {
                id: id(301),
                a: SeamSide {
                    copy: id(200),
                    range: EdgeRange::whole(a),
                },
                b: SeamSide {
                    copy: id(201),
                    range: EdgeRange::whole(b),
                },
                direction: SeamDirection::Opposite,
                ease: DeclaredEase {
                    amount: EaseAmount::Profile(ProfileParameterRef::new(id(8000))),
                    distribution: EaseDistribution::Uniform,
                }
                .into(),
                stops: vec![StopLandmark {
                    side: SeamSideId::A,
                    landmark: id(401),
                }],
            }],
        },
        &plan,
        &pieces,
        &marks,
        &ledger,
    )
    .expect("actual canonical graph");
    Fixture {
        ledger,
        pieces,
        plan,
        graph,
        marks,
    }
}
