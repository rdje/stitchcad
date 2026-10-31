//! Complete actual walk/copy/pattern context;no fixture certifies geometric correspondence.
#[path = "walk_input.rs"]
pub mod input;
use input::{fixture, id, t};
use sc_core::{construction::*, ontology::*};
pub fn with_copies(run: impl FnOnce(BoundWalkCopyInventory<'_>)) {
    with_pair_count(1, run);
}
pub fn with_pair_count(pairs: usize, run: impl FnOnce(BoundWalkCopyInventory<'_>)) {
    let f = fixture();
    let edit = WalkSeamEdit::new(id(10), f.span().id(), vec![f.pair(t(1, 3), t(2, 5)); pairs]);
    let admitted = edit
        .admit(WalkEditLimits::new(pairs))
        .expect("complete requests");
    let source = admitted.bind_source(&f.graph).expect("actual graph/span");
    let pieces = f
        .pieces
        .iter()
        .map(|piece| {
            Piece::new(piece.definition().clone(), &f.ledger)
                .expect("actual independently born Piece fields")
        })
        .collect();
    let inventory = PieceInventory::new(pieces).expect("complete actual inventory");
    assert_eq!(
        inventory
            .piece(id(100))
            .expect("source pattern")
            .definition()
            .construction_lines
            .first()
            .expect("complete construction fields")
            .edge,
        f.extra
    );
    let limits = WalkCopyInventoryLimits::new(2, 2, 8);
    let copies = source
        .bind_copy_inventory(&f.plan, &inventory, &limits)
        .expect("both actual copy/pattern owners");
    run(copies);
}
