//! Explicit canonical cut intent against actual source fields,without output identities or geometry.
#![allow(clippy::expect_used)]
#[path = "support/mirror_source.rs"]
mod common;
use common::{axis, fixture, id, input};
use sc_core::{
    construction::{
        MirrorPieceCutPolicy as Policy, MirrorPieceCutPolicyRefusal as Refusal,
        MirrorPieceFoldIntent as Fold, MirrorPieceSourceLimits,
    },
    ontology::{CuttingSide, Handedness, Mirroring, Piece},
};

#[test]
fn explicit_result_fields_borrow_one_canonical_policy_instead_of_inheriting_source_cut_data() {
    let (_, piece) = fixture();
    let original = piece.clone();
    let input = input(10, 100, axis());
    let limits = MirrorPieceSourceLimits::new(2, 8);
    let source = input.bind_piece(&piece, &limits).expect("actual source");
    for (quantity, mirroring) in [
        (u32::MAX, Mirroring::Single),
        (6, Mirroring::MirroredPairs),
        (
            3,
            Mirroring::PairMember {
                handedness: Handedness::Left,
                companion: id(100),
            },
        ),
    ] {
        let policy = Policy::new(quantity, mirroring, Fold::NoFold, CuttingSide::Face);
        let cut = source
            .bind_cut_policy(&policy)
            .expect("local declared cut rules");
        assert!(std::ptr::eq(cut.source(), &source));
        assert!(std::ptr::eq(cut.policy(), &policy));
        assert!(std::ptr::eq(cut.quantity(), policy.quantity()));
        assert!(std::ptr::eq(cut.mirroring(), policy.mirroring()));
        assert!(std::ptr::eq(cut.cutting_side(), policy.cutting_side()));
        assert_eq!(*cut.quantity(), quantity);
        assert_eq!(*cut.mirroring(), mirroring);
        assert_eq!(*cut.cutting_side(), CuttingSide::Face);
        assert_eq!(*cut.policy().fold(), Fold::NoFold);
        assert!(!cut.cut_on_fold());
        assert!(cut.fold_source().is_none());
        assert_eq!(cut.source().piece().definition().quantity, 4);
        assert!(cut.source().piece().definition().cut_on_fold);
        assert_eq!(
            cut.source().piece().definition().cutting_side,
            CuttingSide::WrongSide
        );
    }
    assert_eq!(piece, original);
}

#[test]
fn reflected_fold_requirement_borrows_actual_original_fold_without_issuing_a_new_edge() {
    let (_, piece) = fixture();
    let input = input(10, 100, axis());
    let limits = MirrorPieceSourceLimits::new(2, 8);
    let source = input.bind_piece(&piece, &limits).expect("actual source");
    let policy = Policy::new(
        2,
        Mirroring::MirroredPairs,
        Fold::ReflectedSource,
        CuttingSide::WrongSide,
    );
    let cut = source
        .bind_cut_policy(&policy)
        .expect("actual declared source fold");
    let fold = piece
        .definition()
        .fold_edges
        .first()
        .expect("original fold");
    assert!(std::ptr::eq(
        cut.fold_source().expect("attributed source"),
        fold
    ));
    assert!(cut.cut_on_fold());
    assert_eq!(*cut.policy().fold(), Fold::ReflectedSource);
    assert_eq!(source.edge_count(), 8);
}

#[test]
fn zero_then_pair_quantity_precede_missing_fold_and_errors_retain_all_actual_context() {
    let (ledger, piece) = fixture();
    let mut definition = piece.definition().clone();
    definition.cut_on_fold = false;
    definition.fold_edges.clear();
    let piece = Piece::new(definition, &ledger).expect("actual source without fold");
    let original = piece.clone();
    let input = input(10, 100, axis());
    let limits = MirrorPieceSourceLimits::new(2, 7);
    let source = input
        .bind_piece(&piece, &limits)
        .expect("exact no-fold source population");
    for (quantity, refusal) in [
        (0, Refusal::ZeroQuantity),
        (3, Refusal::UnpairedQuantity { quantity: 3 }),
        (2, Refusal::MissingSourceFold),
    ] {
        let policy = Policy::new(
            quantity,
            Mirroring::MirroredPairs,
            Fold::ReflectedSource,
            CuttingSide::Face,
        );
        let original_policy = policy;
        let error = source
            .bind_cut_policy(&policy)
            .expect_err("first actual local cause");
        assert_eq!(error.refusal(), refusal);
        assert!(std::ptr::eq(error.source(), &source));
        assert!(std::ptr::eq(error.policy(), &policy));
        assert!(std::ptr::eq(error.source().input(), &input));
        assert_eq!(policy, original_policy);
    }
    assert_eq!(piece, original);
}

#[test]
fn no_fold_is_explicitly_legal_for_an_actual_source_without_fold() {
    let (ledger, piece) = fixture();
    let mut definition = piece.definition().clone();
    definition.cut_on_fold = false;
    definition.fold_edges.clear();
    let piece = Piece::new(definition, &ledger).expect("actual source without fold");
    let input = input(10, 100, axis());
    let limits = MirrorPieceSourceLimits::new(2, 7);
    let source = input.bind_piece(&piece, &limits).expect("actual source");
    let policy = Policy::new(1, Mirroring::Single, Fold::NoFold, CuttingSide::WrongSide);
    let cut = source
        .bind_cut_policy(&policy)
        .expect("no fabricated fold required");
    assert!(!cut.cut_on_fold());
    assert!(cut.fold_source().is_none());
}
