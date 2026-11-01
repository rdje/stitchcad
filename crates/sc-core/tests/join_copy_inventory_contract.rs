//! Complete held join inventories preserve original sources;no result copy or weld is inferred.
#![allow(clippy::expect_used)]
use sc_core::{construction::*, ontology::*};
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
#[path = "support/seam_edit_sources.rs"]
mod common;
use common::{first_span, fixture};
fn request() -> JoinPiecesAtSeam {
    JoinPiecesAtSeam::new(id(1000), id(301), id(100), id(101)).expect("distinct original Pieces")
}

#[test]
fn complete_original_inventory_keeps_both_sides_unrelated_copies_and_reflected_orientation() {
    let f = fixture(false);
    let a = f.pieces.first().expect("A").clone();
    let mut b = f.pieces.get(1).expect("B").definition().clone();
    b.quantity = 2;
    b.mirroring = Mirroring::MirroredPairs;
    let b = Piece::new(b, &f.ledger).expect("paired B");
    let mut unrelated = a.definition().clone();
    unrelated.id = id(102);
    let unrelated = Piece::new(unrelated, &f.ledger).expect("unrelated actual Piece");
    let inventory = PieceInventory::new(vec![unrelated, b, a]).expect("complete actual inventory");
    let copy = |identity, piece, orientation| CutCopyDefinition {
        id: id(identity),
        piece: id(piece),
        orientation,
    };
    let plan = CutPlan::new(
        vec![
            copy(201, 101, CopyOrientation::Reflected),
            copy(202, 102, CopyOrientation::Authored),
            copy(200, 100, CopyOrientation::Authored),
            copy(203, 101, CopyOrientation::Authored),
        ],
        inventory.pieces(),
    )
    .expect("complete canonical plan");
    let before = inventory.clone();
    let edit = request();
    let source = edit
        .bind_source(&f.graph, &plan, inventory.pieces())
        .expect("actual original sources");
    let limits = JoinCopyInventoryLimits::new(4, 3, 15);
    let bound = source
        .bind_copy_inventory(&inventory, &limits)
        .expect("whole held source admitted");
    assert!(std::ptr::eq(bound.source(), &source));
    assert!(std::ptr::eq(bound.inventory(), &inventory));
    assert!(std::ptr::eq(bound.limits(), &limits));
    assert!(std::ptr::eq(bound.source().pieces(), inventory.pieces()));
    assert!(std::ptr::eq(bound.source().plan(), &plan));
    assert!(std::ptr::eq(bound.source().span(), first_span(&f.graph)));
    assert_eq!(
        (
            limits.copies(),
            limits.pieces(),
            limits.workspace_entries(),
            bound.workspace_bound()
        ),
        (4, 3, 15, 15)
    );
    for side in [SeamSideId::A, SeamSideId::B] {
        let view = bound.source().side(side);
        assert!(std::ptr::eq(
            view.piece(),
            inventory
                .piece(edit.piece_id(side))
                .expect("original Piece")
        ));
        assert!(std::ptr::eq(
            view.copy(),
            plan.copy(view.source_side().copy).expect("original copy")
        ));
    }
    assert_eq!(
        bound.source().side(SeamSideId::B).copy().orientation(),
        CopyOrientation::Reflected
    );
    assert_eq!(
        bound
            .source()
            .side(SeamSideId::B)
            .source_copies()
            .map(|copy| copy.id())
            .collect::<Vec<_>>(),
        vec![id(201), id(203)]
    );
    assert_eq!(
        bound.source().span().definition().stops.len(),
        f.marks.len()
    );
    assert_eq!(inventory, before);
    assert!(format!("{bound:?}").contains("BoundJoinCopyInventory"));
}

#[test]
fn whole_c_i_workspace_quotas_precede_source_identity_and_canonical_quantity_failure() {
    use JoinCopyInventoryRefusal as R;
    let f = fixture(false);
    let mut pieces = f.pieces.clone();
    let mut b = pieces.get(1).expect("B").definition().clone();
    b.quantity = 2;
    *pieces.get_mut(1).expect("B slot") =
        Piece::new(b, &f.ledger).expect("actual changed quantity");
    let inventory = PieceInventory::new(pieces).expect("canonical companion collection");
    let edit = request();
    let same = edit
        .bind_source(&f.graph, &f.plan, inventory.pieces())
        .expect("quantity is not source association");
    let different = edit
        .bind_source(&f.graph, &f.plan, &f.pieces)
        .expect("original other slice");
    for source in [&same, &different] {
        for (limits, cause) in [
            (
                JoinCopyInventoryLimits::new(1, 0, 0),
                R::Copies {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                JoinCopyInventoryLimits::new(2, 1, 0),
                R::Pieces {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                JoinCopyInventoryLimits::new(2, 2, 7),
                R::Workspace {
                    required: 8,
                    allowed: 7,
                },
            ),
        ] {
            let error = source
                .bind_copy_inventory(&inventory, &limits)
                .expect_err("whole population first");
            assert!(std::ptr::eq(error.source(), source));
            assert!(std::ptr::eq(error.inventory(), &inventory));
            assert!(std::ptr::eq(error.limits(), &limits));
            assert_eq!(error.refusal(), &cause);
            assert!(error.to_string().starts_with("join copy inventory:"));
            assert!(format!("{error:?}").contains("JoinCopyInventoryError"));
        }
    }
    let limits = JoinCopyInventoryLimits::new(2, 2, 8);
    assert_eq!(
        different
            .bind_copy_inventory(&inventory, &limits)
            .expect_err("source mismatch first")
            .refusal(),
        &R::SourceInventory
    );
    let expected = CutPlan::new(
        f.plan
            .copies()
            .iter()
            .map(|copy| *copy.definition())
            .collect(),
        inventory.pieces(),
    )
    .expect_err("canonical quantity mismatch");
    assert_eq!(
        expected,
        CutPlanError::QuantityMismatch {
            piece: id(101),
            expected: 2,
            actual: 1
        }
    );
    assert_eq!(
        same.bind_copy_inventory(&inventory, &limits)
            .expect_err("actual canonical refusal")
            .refusal(),
        &R::Inventory(expected)
    );
}

#[test]
fn equal_content_copy_is_not_the_exact_original_inventory_slice() {
    let f = fixture(false);
    let inventory = PieceInventory::new(f.pieces.clone()).expect("actual inventory");
    let equal = inventory.clone();
    assert_eq!(equal, inventory);
    let edit = request();
    let source = edit
        .bind_source(&f.graph, &f.plan, inventory.pieces())
        .expect("actual original slice");
    let limits = JoinCopyInventoryLimits::new(2, 2, 8);
    let error = source
        .bind_copy_inventory(&equal, &limits)
        .expect_err("equal records cannot replace original provenance");
    assert_eq!(error.refusal(), &JoinCopyInventoryRefusal::SourceInventory);
    assert!(std::ptr::eq(error.inventory(), &equal));
    assert!(std::ptr::eq(error.source().pieces(), inventory.pieces()));
    assert_eq!(
        source
            .bind_copy_inventory(&inventory, &limits)
            .expect("original admitted")
            .workspace_bound(),
        8
    );
}
