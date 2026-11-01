//! Actual complete basic join source/bank parents for role/identity/output contracts.
use sc_core::{construction::*, ontology::*};
pub fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
#[path = "seam_edit_sources.rs"]
mod seams;
pub fn with_banks(run: impl FnOnce(BoundJoinCutBanks<'_>)) {
    let f = seams::fixture(false);
    let inventory = PieceInventory::new(f.pieces.clone()).expect("whole source inventory");
    let graph = SewingGraph::new(
        SewingGraphDefinition {
            id: id(300),
            spans: vec![seams::first_span(&f.graph).definition().clone()],
        },
        &f.plan,
        inventory.pieces(),
        &f.marks,
        &f.ledger,
    )
    .expect("actual source graph");
    let edit = JoinPiecesAtSeam::new(id(1000), id(301), id(100), id(101)).expect("join inputs");
    let source = edit
        .bind_source(&graph, &f.plan, inventory.pieces())
        .expect("actual source slice");
    let limits = JoinCopyInventoryLimits::new(2, 2, 8);
    let copies = source
        .bind_copy_inventory(&inventory, &limits)
        .expect("complete original inventory");
    let request = JoinExactSpliceRequirement::new(JoinSplicePlacement::KeepOriginal);
    let splice = copies.require_exact_splice(&request);
    let policy = JoinPieceMetadataPolicy::new(
        JoinPieceFieldIntent::FromSide(SeamSideId::A),
        JoinPieceFieldIntent::FromSide(SeamSideId::A),
        JoinPieceFieldIntent::FromSide(SeamSideId::A),
        JoinPieceFieldIntent::FromSide(SeamSideId::A),
    );
    let text_limits = JoinPieceMetadataLimits::new(28);
    let metadata = splice
        .bind_metadata(&policy, &text_limits)
        .expect("complete selected fields");
    let population = JoinCutPopulation::new(1, Mirroring::Single);
    let cut = metadata
        .bind_cut_population(&population)
        .expect("explicit cut population");
    let sources = cut
        .admit_source_pieces(JoinSourcePieceLimits::new(0, 2))
        .expect("whole original H/R");
    let banks = sources
        .bind_cut_banks()
        .expect("actual original A/B cut banks");
    run(banks);
}
