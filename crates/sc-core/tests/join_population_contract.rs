//! Canonical result population intent,without allocating or reconciling physical copies.
#![allow(clippy::expect_used)]
use sc_core::{construction::*, ontology::*};
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
#[allow(dead_code)]
#[path = "support/seam_edit_sources.rs"]
mod common;
fn with_metadata(run: impl FnOnce(BoundJoinPieceMetadata<'_>)) {
    let f = common::fixture(false);
    let inventory = PieceInventory::new(f.pieces.clone()).expect("whole source inventory");
    let input = JoinPiecesAtSeam::new(id(1000), id(301), id(100), id(101)).expect("source request");
    let source = input
        .bind_source(&f.graph, &f.plan, inventory.pieces())
        .expect("actual original slice");
    let limits = JoinCopyInventoryLimits::new(2, 2, 8);
    let copies = source
        .bind_copy_inventory(&inventory, &limits)
        .expect("canonical original population");
    let request = JoinExactSpliceRequirement::new(JoinSplicePlacement::KeepOriginal);
    let splice = copies.require_exact_splice(&request);
    let fields = JoinPieceMetadataPolicy::new(
        JoinPieceFieldIntent::FromSide(SeamSideId::A),
        JoinPieceFieldIntent::FromSide(SeamSideId::A),
        JoinPieceFieldIntent::FromSide(SeamSideId::A),
        JoinPieceFieldIntent::FromSide(SeamSideId::A),
    );
    let limits = JoinPieceMetadataLimits::new(28);
    run(splice
        .bind_metadata(&fields, &limits)
        .expect("actual complete selected metadata"));
}
#[test]
fn explicit_totals_and_modes_lend_original_fields_without_pairing_or_allocating_copies() {
    with_metadata(|metadata| {
        let rows = [
            (5, Mirroring::Single),
            (4, Mirroring::MirroredPairs),
            (
                3,
                Mirroring::PairMember {
                    handedness: Handedness::Left,
                    companion: id(500),
                },
            ),
            (
                3,
                Mirroring::PairMember {
                    handedness: Handedness::Right,
                    companion: id(500),
                },
            ),
            (u32::MAX, Mirroring::Single),
        ];
        for (quantity, mirroring) in rows {
            let raw = JoinCutPopulation::new(quantity, mirroring);
            let before = raw;
            let bound = metadata
                .bind_cut_population(&raw)
                .expect("locally valid explicit canonical population");
            assert!(std::ptr::eq(bound.metadata(), &metadata));
            assert!(std::ptr::eq(bound.population(), &raw));
            assert!(std::ptr::eq(bound.quantity(), raw.quantity()));
            assert!(std::ptr::eq(bound.mirroring(), raw.mirroring()));
            assert_eq!(
                (*bound.quantity(), *bound.mirroring()),
                (quantity, mirroring)
            );
            assert_eq!(raw, before);
            assert!(format!("{bound:?}").contains("BoundJoinCutPopulation"));
        }
        for side in [SeamSideId::A, SeamSideId::B] {
            assert_eq!(
                metadata
                    .splice()
                    .source()
                    .side(side)
                    .piece()
                    .definition()
                    .quantity,
                1
            );
        }
    });
}
#[test]
fn zero_precedes_pair_parity_and_refusals_keep_original_raw_values_and_context() {
    use JoinCutPopulationRefusal as R;
    with_metadata(|metadata| {
        for (quantity, mirroring, cause) in [
            (0, Mirroring::Single, R::ZeroQuantity),
            (0, Mirroring::MirroredPairs, R::ZeroQuantity),
            (
                0,
                Mirroring::PairMember {
                    handedness: Handedness::Left,
                    companion: id(500),
                },
                R::ZeroQuantity,
            ),
            (
                3,
                Mirroring::MirroredPairs,
                R::UnpairedQuantity { quantity: 3 },
            ),
            (
                u32::MAX,
                Mirroring::MirroredPairs,
                R::UnpairedQuantity { quantity: u32::MAX },
            ),
        ] {
            let raw = JoinCutPopulation::new(quantity, mirroring);
            let error = metadata
                .bind_cut_population(&raw)
                .expect_err("actual canonical population cause");
            assert!(std::ptr::eq(error.metadata(), &metadata));
            assert!(std::ptr::eq(error.population(), &raw));
            assert_eq!(error.refusal(), cause);
            assert_eq!(
                (
                    *error.population().quantity(),
                    *error.population().mirroring()
                ),
                (quantity, mirroring)
            );
            assert!(error.to_string().starts_with("join cut population:"));
            assert!(format!("{error:?}").contains("JoinCutPopulationError"));
        }
    });
}
