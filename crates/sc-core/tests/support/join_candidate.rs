//! Born canonical candidate Pieces/copies under actual native source/result correspondence.
#[path = "join_pairing.rs"]
pub mod pairing;
pub use pairing::id;
use sc_core::{construction::*, ontology::*};
pub fn ledger() -> IdentityLedger {
    let mut ledger = IdentityLedger::new();
    for (operation, count) in [(1, 2), (2, 2), (1000, 14)] {
        ledger
            .declare_edges(&mut DeterministicIdGenerator::from_start(operation), count)
            .expect("explicit fixture operations and logical roles");
    }
    ledger
}
pub fn inventory(
    pairs: &RequiredJoinCopyCorrespondence<'_>,
    quantity: u32,
    mirroring: Mirroring,
    retained_id: u128,
    extra: bool,
) -> PieceInventory {
    let results = pairs.sources().results();
    let mut joined = results
        .source_piece(id(100))
        .expect("actual original A")
        .definition()
        .clone();
    joined.id = id(5000);
    joined.quantity = quantity;
    joined.mirroring = mirroring;
    joined.cut_on_fold = false;
    joined.fold_edges.clear();
    joined.construction_lines.clear();
    let contours = results
        .folds()
        .fold()
        .construction()
        .targets()
        .coverage()
        .groups()
        .sources()
        .uses()
        .connectivity()
        .contours();
    joined.boundary = contours
        .boundary()
        .map(|u| DirectedEdge {
            edge: u.logical_output(),
            direction: *u.direction(),
        })
        .collect();
    joined.holes = vec![];
    let ledger = ledger();
    let joined =
        Piece::new(joined, &ledger).expect("born candidate joined Piece;geometry deferred");
    let mut unrelated = results
        .source_piece(id(102))
        .expect("actual unrelated original")
        .definition()
        .clone();
    unrelated.id = id(retained_id);
    let mut pieces = vec![
        joined,
        Piece::new(unrelated, &ledger).expect("born unrelated candidate"),
    ];
    if extra {
        let mut d = results
            .source_piece(id(102))
            .expect("actual unrelated template")
            .definition()
            .clone();
        d.id = id(104);
        pieces.push(Piece::new(d, &ledger).expect("born extra candidate"));
    }
    PieceInventory::new(pieces).expect("complete companion-consistent candidate inventory")
}
pub fn definitions(pairs: &RequiredJoinCopyCorrespondence<'_>) -> Vec<CutCopyDefinition> {
    let results = pairs.sources().results();
    vec![
        *results
            .copy(id(6000))
            .expect("explicit result6000")
            .definition(),
        *results
            .source_copy(id(202))
            .expect("actual unrelated copy")
            .definition(),
        *results
            .copy(id(6001))
            .expect("explicit result6001")
            .definition(),
    ]
}
pub fn baseline_inventory(pairs: &RequiredJoinCopyCorrespondence<'_>) -> PieceInventory {
    inventory(pairs, 2, Mirroring::MirroredPairs, 102, false)
}
pub fn baseline_plan(
    pairs: &RequiredJoinCopyCorrespondence<'_>,
    inventory: &PieceInventory,
) -> CutPlan {
    CutPlan::new(definitions(pairs), inventory.pieces()).expect("complete canonical candidate plan")
}
