//! Shared original walk-input fixture;canonical structural objects claim no geometric correspondence.
use sc_core::{
    construction::{WalkBalanceNotch as Mark, WalkBalancePair as Pair},
    ontology::*,
};
pub fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
pub fn t(n: i64, d: i64) -> Param {
    Param::new(Rational::new(n, d).expect("exact rational")).expect("unit interval")
}
pub fn profiles(seed: u128) -> NotchProfileBindings {
    let p = |n| ProfileParameterRef::new(id(seed + n));
    NotchProfileBindings {
        style: p(0),
        sample: NotchDimensionBindings {
            depth: p(1),
            width: p(2),
        },
        production: NotchDimensionBindings {
            depth: p(3),
            width: p(4),
        },
        encoding: p(5),
    }
}
pub struct Fixture {
    pub ledger: IdentityLedger,
    pub pieces: Vec<Piece>,
    pub plan: CutPlan,
    pub graph: SewingGraph,
    pub extra: EdgeRef,
}
pub fn fixture() -> Fixture {
    let mut ledger = IdentityLedger::new();
    let mut ids = DeterministicIdGenerator::new();
    let [a, b, extra] = std::array::from_fn(|_| {
        *ledger
            .declare_edges(&mut ids, 1)
            .expect("source allocation")
            .first()
            .expect("one source")
    });
    let pieces = [(id(100), a, vec![extra]), (id(101), b, vec![])]
        .into_iter()
        .map(|(identity, boundary, lines)| {
            let directed = |edge| DirectedEdge {
                edge,
                direction: Direction::Original,
            };
            Piece::new(
                PieceDefinition {
                    id: identity,
                    boundary: vec![directed(boundary)],
                    holes: vec![],
                    construction_lines: lines.into_iter().map(directed).collect(),
                    quantity: 1,
                    mirroring: Mirroring::Single,
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
        })
        .collect::<Vec<_>>();
    let plan = CutPlan::new(
        vec![
            CutCopyDefinition {
                id: id(200),
                piece: id(100),
                orientation: CopyOrientation::Authored,
            },
            CutCopyDefinition {
                id: id(201),
                piece: id(101),
                orientation: CopyOrientation::Authored,
            },
        ],
        &pieces,
    )
    .expect("actual copy plan");
    let graph = SewingGraph::new(
        SewingGraphDefinition {
            id: id(300),
            spans: vec![SeamSpanDefinition {
                id: id(301),
                a: SeamSide {
                    copy: id(200),
                    range: EdgeRange::new(a, t(1, 7), t(6, 7)).expect("A source interval"),
                },
                b: SeamSide {
                    copy: id(201),
                    range: EdgeRange::new(b, t(1, 5), t(4, 5)).expect("B source interval"),
                },
                direction: SeamDirection::Opposite,
                ease: DeclaredEase {
                    amount: EaseAmount::Profile(ProfileParameterRef::new(id(9000))),
                    distribution: EaseDistribution::Uniform,
                }
                .into(),
                stops: vec![],
            }],
        },
        &plan,
        &pieces,
        &[],
        &ledger,
    )
    .expect("actual structurally valid graph");
    Fixture {
        ledger,
        pieces,
        plan,
        graph,
        extra,
    }
}
impl Fixture {
    pub fn span(&self) -> &SeamSpan {
        self.graph.spans().first().expect("canonical source")
    }
    pub fn pair(&self, a: Param, b: Param) -> Pair {
        Pair::new(
            Mark::new(
                EdgeAnchor {
                    edge: self.span().definition().a.range.edge(),
                    param: a,
                },
                profiles(1000),
            ),
            Mark::new(
                EdgeAnchor {
                    edge: self.span().definition().b.range.edge(),
                    param: b,
                },
                profiles(2000),
            ),
        )
    }
}
