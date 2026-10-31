//! Actual complete truing copy/Piece association;no fitted curve or independent-counterpart proof.
#![allow(clippy::expect_used)]
use sc_core::{construction::*, ontology::*};
use sc_units::{Length, Tolerance, ToleranceClass};
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
#[path = "support/seam_edit_sources.rs"]
mod common;
use common::{first_span, fixture};
fn tolerance() -> Tolerance {
    Tolerance::new(
        ToleranceClass::GeometricApproximation,
        Length::from_micrometres(7).expect("length"),
        "explicit fixture length-fit source,not geometric evidence",
    )
    .expect("requirement")
}
fn request(side: SeamSideId, piece: EntityId) -> ConstrainedSeamTruing {
    ConstrainedSeamTruing::new(id(1000), id(301), side, piece, id(9000)).expect("explicit input")
}
#[test]
fn both_adjusted_sides_lend_actual_complete_source_plan_inventory_policy_and_original_provider() {
    let f = fixture(false);
    let fit = tolerance();
    let before = f.pieces.clone();
    assert_eq!(f.marks.len(), 1);
    let inventory = PieceInventory::new(
        f.pieces
            .iter()
            .map(|p| {
                Piece::new(p.definition().clone(), &f.ledger).expect("independent actual Piece")
            })
            .collect(),
    )
    .expect("whole inventory");
    for (side, piece, other) in [
        (SeamSideId::A, id(100), id(101)),
        (SeamSideId::B, id(101), id(100)),
    ] {
        let edit = request(side, piece);
        let source = edit
            .bind_source(
                &f.graph,
                &f.plan,
                TruingFitProvider {
                    source: id(9000),
                    tolerance: &fit,
                },
            )
            .expect("actual original source");
        let limits = TruingCopyInventoryLimits::new(2, 2, 8);
        let bound = source
            .bind_copy_inventory(&inventory, &limits)
            .expect("whole actual context");
        assert!(std::ptr::eq(bound.source(), &source));
        assert!(std::ptr::eq(bound.plan(), &f.plan));
        assert!(std::ptr::eq(bound.inventory(), &inventory));
        assert!(std::ptr::eq(bound.limits(), &limits));
        assert!(std::ptr::eq(bound.source().span(), first_span(&f.graph)));
        assert!(std::ptr::eq(bound.source().fit_provider().tolerance, &fit));
        assert_eq!(
            (
                limits.copies(),
                limits.pieces(),
                limits.workspace_entries(),
                bound.workspace_bound()
            ),
            (2, 2, 8, 8)
        );
        assert_eq!(bound.selected_piece().id(), piece);
        assert_eq!(bound.counterpart_piece().id(), other);
        assert!(!bound.same_pattern_piece());
        for side in [SeamSideId::A, SeamSideId::B] {
            let original = match side {
                SeamSideId::A => &first_span(&f.graph).definition().a,
                SeamSideId::B => &first_span(&f.graph).definition().b,
            };
            let copy = f.plan.copy(original.copy).expect("actual copy");
            assert!(std::ptr::eq(bound.copy(side), copy));
            assert!(std::ptr::eq(
                bound.piece(side),
                inventory.piece(copy.piece()).expect("actual Piece")
            ));
        }
        assert!(std::ptr::eq(
            bound.selected_piece(),
            inventory.piece(piece).expect("actual selected Piece")
        ));
        assert!(format!("{bound:?}").contains("BoundTruingCopyInventory"));
    }
    assert_eq!(f.pieces, before);
}
#[test]
fn complete_c_i_workspace_quotas_precede_canonical_quantity_mismatch_and_keep_all_actual_parents() {
    let f = fixture(false);
    let fit = tolerance();
    let edit = request(SeamSideId::A, id(100));
    let source = edit
        .bind_source(
            &f.graph,
            &f.plan,
            TruingFitProvider {
                source: id(9000),
                tolerance: &fit,
            },
        )
        .expect("source");
    let mut definition = f.pieces.first().expect("A").definition().clone();
    definition.quantity = 2;
    let inventory = PieceInventory::new(vec![
        Piece::new(definition, &f.ledger).expect("changed actual quantity"),
        f.pieces.get(1).expect("B").clone(),
    ])
    .expect("actual inventory");
    use TruingCopyInventoryRefusal as R;
    for (limits, cause) in [
        (
            TruingCopyInventoryLimits::new(1, 0, 0),
            R::Copies {
                required: 2,
                allowed: 1,
            },
        ),
        (
            TruingCopyInventoryLimits::new(2, 1, 0),
            R::Pieces {
                required: 2,
                allowed: 1,
            },
        ),
        (
            TruingCopyInventoryLimits::new(2, 2, 7),
            R::Workspace {
                required: 8,
                allowed: 7,
            },
        ),
    ] {
        let error = source
            .bind_copy_inventory(&inventory, &limits)
            .expect_err("full resource first");
        assert!(std::ptr::eq(error.source(), &source));
        assert!(std::ptr::eq(error.plan(), &f.plan));
        assert!(std::ptr::eq(error.inventory(), &inventory));
        assert!(std::ptr::eq(error.limits(), &limits));
        assert_eq!(error.refusal(), &cause);
        assert!(error.to_string().starts_with("truing copy inventory:"));
        assert!(format!("{error:?}").contains("TruingCopyInventoryError"));
    }
    let limits = TruingCopyInventoryLimits::new(2, 2, 8);
    let expected = CutPlan::new(
        f.plan.copies().iter().map(|c| *c.definition()).collect(),
        inventory.pieces(),
    )
    .expect_err("canonical quantity mismatch");
    assert_eq!(
        source
            .bind_copy_inventory(&inventory, &limits)
            .expect_err("actual canonical refusal")
            .refusal(),
        &R::Inventory(expected)
    );
}
#[test]
fn missing_counterpart_in_retained_plan_refuses_in_original_a_b_order_without_a_partial_context() {
    let f = fixture(false);
    let fit = tolerance();
    for (selected, piece, copy, missing_side, missing_copy) in [
        (SeamSideId::A, id(100), id(200), SeamSideId::B, id(201)),
        (SeamSideId::B, id(101), id(201), SeamSideId::A, id(200)),
    ] {
        let inventory = PieceInventory::new(
            f.pieces
                .iter()
                .filter(|p| p.id() == piece)
                .cloned()
                .collect(),
        )
        .expect("actual selected inventory");
        let plan = CutPlan::new(
            vec![*f.plan.copy(copy).expect("selected copy").definition()],
            inventory.pieces(),
        )
        .expect("actual one-Piece plan");
        let edit = request(selected, piece);
        let source = edit
            .bind_source(
                &f.graph,
                &plan,
                TruingFitProvider {
                    source: id(9000),
                    tolerance: &fit,
                },
            )
            .expect("input binding checks selected copy only");
        let limits = TruingCopyInventoryLimits::new(1, 1, 4);
        let error = source
            .bind_copy_inventory(&inventory, &limits)
            .expect_err("counterpart stays mandatory");
        assert_eq!(
            error.refusal(),
            &TruingCopyInventoryRefusal::MissingCopy {
                side: missing_side,
                copy: missing_copy
            }
        );
        assert!(std::ptr::eq(error.plan(), &plan));
        assert!(std::ptr::eq(error.source(), &source));
    }
}
#[test]
fn shared_pattern_preserves_reflected_and_authored_copies_without_certifying_a_fixed_counterpart() {
    let f = fixture(true);
    let fit = tolerance();
    let inventory = PieceInventory::new(f.pieces.clone()).expect("actual shared pattern");
    for side in [SeamSideId::A, SeamSideId::B] {
        let edit = request(side, id(100));
        let source = edit
            .bind_source(
                &f.graph,
                &f.plan,
                TruingFitProvider {
                    source: id(9000),
                    tolerance: &fit,
                },
            )
            .expect("shared source");
        let limits = TruingCopyInventoryLimits::new(2, 1, 7);
        let bound = source
            .bind_copy_inventory(&inventory, &limits)
            .expect("actual coupled owners");
        assert!(bound.same_pattern_piece());
        assert!(std::ptr::eq(
            bound.selected_piece(),
            bound.counterpart_piece()
        ));
        assert_eq!(
            bound.copy(SeamSideId::A).orientation(),
            CopyOrientation::Authored
        );
        assert_eq!(
            bound.copy(SeamSideId::B).orientation(),
            CopyOrientation::Reflected
        );
        assert_eq!(
            bound
                .plan()
                .copies()
                .iter()
                .map(|c| c.id())
                .collect::<Vec<_>>(),
            vec![id(201), id(200)]
        );
        assert_eq!(bound.source().affected_copies().count(), 2);
        assert_eq!(
            bound.source().span().definition().stops,
            first_span(&f.graph).definition().stops
        );
    }
}
#[test]
fn unrelated_supplied_piece_is_counted_and_canonical_complete_inventory_failure_is_not_filtered_away(
) {
    let f = fixture(false);
    let fit = tolerance();
    let mut pieces = f.pieces.clone();
    let mut extra = pieces.first().expect("A").definition().clone();
    extra.id = id(102);
    pieces.push(Piece::new(extra, &f.ledger).expect("unrelated actual Piece"));
    let inventory = PieceInventory::new(pieces).expect("whole inventory");
    let edit = request(SeamSideId::A, id(100));
    let source = edit
        .bind_source(
            &f.graph,
            &f.plan,
            TruingFitProvider {
                source: id(9000),
                tolerance: &fit,
            },
        )
        .expect("source");
    let low = TruingCopyInventoryLimits::new(2, 2, 9);
    assert_eq!(
        source
            .bind_copy_inventory(&inventory, &low)
            .expect_err("all Pieces counted")
            .refusal(),
        &TruingCopyInventoryRefusal::Pieces {
            required: 3,
            allowed: 2
        }
    );
    let full = TruingCopyInventoryLimits::new(2, 3, 9);
    let error = source
        .bind_copy_inventory(&inventory, &full)
        .expect_err("unplanned actual Piece remains a canonical obligation");
    assert!(matches!(
        error.refusal(),
        TruingCopyInventoryRefusal::Inventory(_)
    ));
    assert_eq!(error.inventory().pieces().len(), 3);
}
