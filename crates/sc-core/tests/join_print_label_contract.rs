//! Canonical joined printed indicators derive from actual matched fields,without duplicate inputs.
#![allow(clippy::expect_used)]
#[allow(dead_code)]
#[path = "support/join_result.rs"]
mod common;
use common::id;
use sc_core::{construction::*, ontology::*};
fn check(
    inventory: &MatchedJoinPieceInventory<'_>,
    quantity: u32,
    mirroring: Mirroring,
    fold: bool,
) {
    let sources = JoinConsumerSources::new(id(0), id(0), id(0), id(0), id(0));
    let required = inventory.require_consumers(&sources);
    let label = required.printed_label();
    assert!(std::ptr::eq(
        label.text,
        &inventory.result().definition().label
    ));
    assert_eq!(
        (label.quantity, label.mirroring, label.cut_on_fold),
        (quantity, mirroring, fold)
    );
    assert_eq!(required.requirements().len(), 5); // Preview discharges no outstanding consumer scope.
    assert_eq!(
        inventory.result().geometric_validation(),
        GeometricValidation::DeferredToG2
    );
}
#[test]
fn actual_matched_single_fold_mirrored_total_and_separate_hand_supply_canonical_print_fields() {
    common::with_result(
        |_| {},
        |fields| {
            let raw = JoinCompanionRepairManifest::new(id(1000), vec![]);
            let repairs = raw
                .admit(&fields, JoinCompanionRepairLimits::new(0, 0))
                .expect("explicit no companion exceptions");
            let originals = repairs
                .match_original_pieces()
                .expect("original fields reconciled");
            let inventory = originals
                .match_result_piece()
                .expect("actual2-hole/8-construction/explicit-fold result");
            check(&inventory, 1, Mirroring::Single, true);
        },
    );
    common::candidate::with_bound_plan(|plan| {
        let fields = plan
            .admit_piece_fields(JoinInventoryFieldLimits::new(3, 2, 0, 24, 114))
            .expect("complete original/candidate resources");
        let raw = JoinCompanionRepairManifest::new(id(1000), vec![]);
        let repairs = raw
            .admit(&fields, JoinCompanionRepairLimits::new(0, 0))
            .expect("explicit no companion exceptions");
        let originals = repairs
            .match_original_pieces()
            .expect("original fields reconciled");
        let inventory = originals
            .match_result_piece()
            .expect("actual2-copy mirrored result");
        check(&inventory, 2, Mirroring::MirroredPairs, false); // Total includes both hands,not per-hand1.
        assert_eq!(inventory.plan().copy_count(), 3); // Unrelated copy202 is not a joined print quantity.
    });
    let mode = Mirroring::PairMember {
        handedness: Handedness::Left,
        companion: id(102),
    };
    common::candidate::with_companion_fields(mode, true, |fields| {
        let raw = JoinCompanionRepairManifest::new(
            id(1000),
            vec![JoinCompanionRepairRequest::new(
                id(102),
                Mirroring::PairMember {
                    handedness: Handedness::Right,
                    companion: id(100),
                },
                id(5000),
                id(0),
            )],
        );
        let repairs = raw
            .admit(&fields, JoinCompanionRepairLimits::new(1, 2))
            .expect("explicit retained counterpart reference requirement");
        let originals = repairs
            .match_original_pieces()
            .expect("original fields except admitted companion");
        let inventory = originals
            .match_result_piece()
            .expect("actual explicit separate-member result");
        check(&inventory, 1, mode, false);
        assert_eq!(
            *inventory
                .originals()
                .original(id(102))
                .expect("actual retained member")
                .companion_repair()
                .expect("still explicit unaccepted request")
                .evidence_source(),
            id(0)
        );
    });
}
