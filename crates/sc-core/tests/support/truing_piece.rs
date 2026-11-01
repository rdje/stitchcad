//! Actual rich held Pieces with repeated construction uses and separate folds.
#[path = "truing_outside.rs"]
pub mod common;
use common::domain::roles::copies::{id, sources};
use sc_core::ontology::*;
pub fn rich(
    f: &mut sources::Fixture,
    side: SeamSideId,
    exact: bool,
    target_in_hole: bool,
) -> Vec<Piece> {
    let mut ids = DeterministicIdGenerator::from_start(10);
    let [outer, hole_a, hole_b, replacement]: [EdgeRef; 4] = f
        .ledger
        .declare_edges(&mut ids, 4)
        .expect("new actual source edges")
        .try_into()
        .expect("four exact source edges");
    let original = match side {
        SeamSideId::A => sources::first_span(&f.graph).definition().a.range.edge(),
        SeamSideId::B => sources::first_span(&f.graph).definition().b.range.edge(),
    };
    let target = if exact { original } else { replacement };
    let directed = |edge, direction| DirectedEdge { edge, direction };
    f.pieces
        .iter()
        .map(|p| {
            if p.id()
                != if side == SeamSideId::A {
                    id(100)
                } else {
                    id(101)
                }
            {
                return p.clone();
            }
            let mut d = p.definition().clone();
            d.boundary = vec![
                directed(outer, Direction::Original),
                directed(
                    if target_in_hole { replacement } else { target },
                    Direction::Reversed,
                ),
            ];
            d.holes = vec![
                vec![directed(
                    if target_in_hole { target } else { hole_a },
                    Direction::Reversed,
                )],
                vec![directed(hole_b, Direction::Original)],
            ];
            d.construction_lines = vec![
                directed(target, Direction::Original),
                directed(outer, Direction::Reversed),
                directed(target, Direction::Reversed),
            ];
            d.cut_on_fold = true;
            d.fold_edges = vec![if target_in_hole { replacement } else { target }];
            Piece::new(d, &f.ledger).expect("structural rich independently supplied Piece")
        })
        .collect()
}
