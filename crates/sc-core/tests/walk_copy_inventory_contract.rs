//! Actual original physical-copy/pattern association;shared ownership grants no notch coalescing.
#![allow(clippy::expect_used)]
#[path = "support/walk_input.rs"]
mod common;
use common::{fixture, id, t};
use sc_core::{construction::*, ontology::*};
fn rows(plan: &CutPlan) -> Vec<CutCopyDefinition> {
    plan.copies()
        .iter()
        .map(|copy| *copy.definition())
        .collect()
}
fn edit(f: &common::Fixture) -> WalkSeamEdit {
    WalkSeamEdit::new(id(10), f.span().id(), vec![f.pair(t(1, 3), t(2, 5))])
}
#[test]
fn reordered_actual_records_cache_both_copy_and_pattern_owners_and_preserve_original_context() {
    let f = fixture();
    let edit = edit(&f);
    let before_edit = edit.clone();
    let admitted = edit.admit(WalkEditLimits::new(1)).expect("actual pairs");
    let source = admitted.bind_source(&f.graph).expect("actual source");
    let mut pieces = f.pieces.clone();
    pieces.reverse();
    let inventory = PieceInventory::new(pieces).expect("actual inventory");
    let mut definitions = rows(&f.plan);
    definitions.reverse();
    let plan = CutPlan::new(definitions, inventory.pieces()).expect("reordered actual plan");
    let before_plan = plan.clone();
    let before_inventory = inventory.clone();
    let before_graph = f.graph.clone();
    let limits = WalkCopyInventoryLimits::new(2, 2, 8);
    let bound = source
        .bind_copy_inventory(&plan, &inventory, &limits)
        .expect("complete actual association");
    assert!(std::ptr::eq(bound.source(), &source));
    assert!(std::ptr::eq(bound.plan(), &plan));
    assert!(std::ptr::eq(bound.inventory(), &inventory));
    assert!(std::ptr::eq(bound.limits(), &limits));
    assert_eq!(
        (
            limits.copies(),
            limits.pieces(),
            limits.workspace_entries(),
            bound.workspace_bound()
        ),
        (2, 2, 8, 8)
    );
    assert!(!bound.same_pattern_piece());
    for (side, copy, piece) in [
        (SeamSideId::A, id(200), id(100)),
        (SeamSideId::B, id(201), id(101)),
    ] {
        assert!(std::ptr::eq(
            bound.copy(side),
            plan.copy(copy).expect("real member")
        ));
        assert!(std::ptr::eq(
            bound.piece(side),
            inventory.piece(piece).expect("real pattern member")
        ));
        assert_eq!(bound.copy(side).piece(), piece);
    }
    assert_eq!(
        bound.plan().copies().first().expect("authored order").id(),
        id(201)
    );
    assert_eq!(
        bound
            .inventory()
            .pieces()
            .first()
            .expect("authored order")
            .id(),
        id(101)
    );
    let pair = bound
        .source()
        .pairs()
        .next()
        .expect("actual input occurrence");
    assert!(std::ptr::eq(
        pair.pair(),
        edit.pairs().first().expect("original")
    ));
    assert_eq!(pair.source_side(SeamSideId::A).copy, id(200));
    assert_ne!(
        pair.pair().mark(SeamSideId::A).representation(),
        pair.pair().mark(SeamSideId::B).representation()
    );
    assert_eq!(
        inventory
            .piece(id(100))
            .expect("source")
            .definition()
            .construction_lines
            .first()
            .expect("extra")
            .edge,
        f.extra
    );
    assert_eq!(edit, before_edit);
    assert_eq!(plan, before_plan);
    assert_eq!(inventory, before_inventory);
    assert_eq!(f.graph, before_graph);
}
#[test]
fn distinct_physical_copies_can_share_one_pattern_without_coalescing_requested_marks() {
    for mirroring in [Mirroring::Single, Mirroring::MirroredPairs] {
        let mut f = fixture();
        let mut piece = f.pieces.first().expect("source").definition().clone();
        piece.quantity = 2;
        piece.mirroring = mirroring;
        let edge = piece.boundary.first().expect("actual boundary").edge;
        f.pieces = vec![Piece::new(piece, &f.ledger).expect("actual shared pattern")];
        f.plan = CutPlan::new(
            vec![
                CutCopyDefinition {
                    id: id(201),
                    piece: id(100),
                    orientation: if mirroring == Mirroring::MirroredPairs {
                        CopyOrientation::Reflected
                    } else {
                        CopyOrientation::Authored
                    },
                },
                CutCopyDefinition {
                    id: id(200),
                    piece: id(100),
                    orientation: CopyOrientation::Authored,
                },
            ],
            &f.pieces,
        )
        .expect("two actual stable copies");
        let mut span = f.span().definition().clone();
        span.b.range = EdgeRange::new(edge, t(1, 5), t(4, 5)).expect("original B range");
        f.graph = SewingGraph::new(
            SewingGraphDefinition {
                id: id(300),
                spans: vec![span],
            },
            &f.plan,
            &f.pieces,
            &[],
            &f.ledger,
        )
        .expect("copies share pattern frame");
        let edit = edit(&f);
        let admitted = edit.admit(WalkEditLimits::new(1)).expect("two requests");
        let source = admitted.bind_source(&f.graph).expect("actual source");
        let inventory = PieceInventory::new(f.pieces.clone()).expect("one actual pattern");
        let limits = WalkCopyInventoryLimits::new(2, 1, 7);
        let bound = source
            .bind_copy_inventory(&f.plan, &inventory, &limits)
            .expect("actual shared pattern");
        assert!(bound.same_pattern_piece());
        assert!(std::ptr::eq(
            bound.piece(SeamSideId::A),
            bound.piece(SeamSideId::B)
        ));
        assert_ne!(
            bound.copy(SeamSideId::A).id(),
            bound.copy(SeamSideId::B).id()
        );
        assert_eq!(
            bound.copy(SeamSideId::B).orientation(),
            if mirroring == Mirroring::MirroredPairs {
                CopyOrientation::Reflected
            } else {
                CopyOrientation::Authored
            }
        );
        assert_eq!(bound.source().admitted().notch_request_count(), 2);
        let pair = bound.source().pairs().next().expect("original pair");
        assert_ne!(
            pair.pair().mark(SeamSideId::A).anchor().param,
            pair.pair().mark(SeamSideId::B).anchor().param
        );
        assert_ne!(
            pair.pair().mark(SeamSideId::A).representation(),
            pair.pair().mark(SeamSideId::B).representation()
        );
    }
}
#[test]
fn disjoint_self_seam_can_cache_the_same_actual_copy_for_both_original_sides() {
    let mut f = fixture();
    let mut span = f.span().definition().clone();
    let edge = span.a.range.edge();
    span.a.range = EdgeRange::new(edge, t(1, 7), t(2, 7)).expect("first range");
    span.b = SeamSide {
        copy: id(200),
        range: EdgeRange::new(edge, t(4, 7), t(6, 7)).expect("disjoint second range"),
    };
    f.graph = SewingGraph::new(
        SewingGraphDefinition {
            id: id(300),
            spans: vec![span],
        },
        &f.plan,
        &f.pieces,
        &[],
        &f.ledger,
    )
    .expect("documented disjoint self seam");
    let edit = WalkSeamEdit::new(id(10), f.span().id(), vec![f.pair(t(1, 5), t(4, 5))]);
    let admitted = edit.admit(WalkEditLimits::new(1)).expect("two fields");
    let source = admitted.bind_source(&f.graph).expect("original self sides");
    let inventory = PieceInventory::new(f.pieces.clone()).expect("complete actual inventory");
    let limits = WalkCopyInventoryLimits::new(2, 2, 8);
    let bound = source
        .bind_copy_inventory(&f.plan, &inventory, &limits)
        .expect("same actual copy");
    assert!(bound.same_pattern_piece());
    assert!(std::ptr::eq(
        bound.copy(SeamSideId::A),
        bound.copy(SeamSideId::B)
    ));
    assert_eq!(bound.copy(SeamSideId::A).id(), id(200));
    assert_eq!(bound.plan().copies().len(), 2);
    assert_eq!(bound.source().admitted().notch_request_count(), 2);
}
#[test]
fn whole_plan_inventory_workspace_precede_first_original_missing_side_and_errors_keep_all_actual_inputs(
) {
    use WalkCopyInventoryRefusal as R;
    let f = fixture();
    let edit = edit(&f);
    let admitted = edit.admit(WalkEditLimits::new(1)).expect("pairs");
    let source = admitted.bind_source(&f.graph).expect("source");
    let inventory = PieceInventory::new(f.pieces.clone()).expect("inventory");
    let mut definitions = rows(&f.plan);
    definitions
        .iter_mut()
        .find(|c| c.id == id(201))
        .expect("original B")
        .id = id(999);
    let plan =
        CutPlan::new(definitions, inventory.pieces()).expect("canonical different actual identity");
    for (limits, expected) in [
        (
            WalkCopyInventoryLimits::new(1, 0, 0),
            R::Copies {
                required: 2,
                allowed: 1,
            },
        ),
        (
            WalkCopyInventoryLimits::new(2, 1, 0),
            R::Pieces {
                required: 2,
                allowed: 1,
            },
        ),
        (
            WalkCopyInventoryLimits::new(2, 2, 7),
            R::Workspace {
                required: 8,
                allowed: 7,
            },
        ),
        (
            WalkCopyInventoryLimits::new(2, 2, 8),
            R::MissingCopy {
                side: SeamSideId::B,
                copy: id(201),
            },
        ),
    ] {
        let error = source
            .bind_copy_inventory(&plan, &inventory, &limits)
            .expect_err("complete refusal");
        assert_eq!(error.refusal(), &expected);
        assert!(std::ptr::eq(error.source(), &source));
        assert!(std::ptr::eq(error.plan(), &plan));
        assert!(std::ptr::eq(error.inventory(), &inventory));
        assert!(std::ptr::eq(error.limits(), &limits));
        assert!(error.to_string().starts_with("walk copy inventory:"));
    }
    let mut definitions = rows(&plan);
    definitions
        .iter_mut()
        .find(|c| c.id == id(200))
        .expect("original A")
        .id = id(998);
    let both = CutPlan::new(definitions, inventory.pieces()).expect("canonical other ids");
    let limits = WalkCopyInventoryLimits::new(2, 2, 8);
    assert_eq!(
        source
            .bind_copy_inventory(&both, &inventory, &limits)
            .expect_err("A first")
            .refusal(),
        &R::MissingCopy {
            side: SeamSideId::A,
            copy: id(200)
        }
    );
    let empty = CutPlan::new(vec![], &[]).expect("empty canonical birth");
    let inventory = PieceInventory::new(vec![]).expect("empty inventory");
    let limits = WalkCopyInventoryLimits::new(0, 0, 0);
    assert_eq!(
        source
            .bind_copy_inventory(&empty, &inventory, &limits)
            .expect_err("no manufactured source copy")
            .refusal(),
        &R::MissingCopy {
            side: SeamSideId::A,
            copy: id(200)
        }
    );
}
#[test]
fn birth_against_different_pieces_cannot_supply_actual_inventory_reference_quantity_or_orientation()
{
    use WalkCopyInventoryRefusal as R;
    let f = fixture();
    let edit = edit(&f);
    let admitted = edit.admit(WalkEditLimits::new(1)).expect("pairs");
    let source = admitted.bind_source(&f.graph).expect("source");
    let limits = WalkCopyInventoryLimits::new(3, 3, 12);
    let missing = PieceInventory::new(
        f.pieces
            .iter()
            .filter(|p| p.id() == id(100))
            .cloned()
            .collect(),
    )
    .expect("one actual pattern");
    assert_eq!(
        source
            .bind_copy_inventory(&f.plan, &missing, &limits)
            .expect_err("actual missing Piece before side lookup")
            .refusal(),
        &R::Inventory(CutPlanError::MissingPiece {
            copy: id(201),
            piece: id(101)
        })
    );
    let mut changed = f.pieces.clone();
    let mut definition = changed.first().expect("A pattern").definition().clone();
    definition.quantity = 2;
    *changed.first_mut().expect("A slot") =
        Piece::new(definition.clone(), &f.ledger).expect("different actual quantity");
    let inventory = PieceInventory::new(changed.clone()).expect("actual inventory");
    assert_eq!(
        source
            .bind_copy_inventory(&f.plan, &inventory, &limits)
            .expect_err("birth quantity differs")
            .refusal(),
        &R::Inventory(CutPlanError::QuantityMismatch {
            piece: id(100),
            expected: 2,
            actual: 1
        })
    );
    definition.mirroring = Mirroring::MirroredPairs;
    *changed.first_mut().expect("A slot") =
        Piece::new(definition, &f.ledger).expect("alternate birth pairs");
    let mut definitions = rows(&f.plan);
    definitions.first_mut().expect("A copy").orientation = CopyOrientation::Reflected;
    definitions.push(CutCopyDefinition {
        id: id(202),
        piece: id(100),
        orientation: CopyOrientation::Authored,
    });
    let alternate = CutPlan::new(definitions, &changed).expect("valid alternate birth");
    let inventory = PieceInventory::new(f.pieces.clone()).expect("actual authored-only patterns");
    assert_eq!(
        source
            .bind_copy_inventory(&alternate, &inventory, &limits)
            .expect_err("actual orientation precedes count")
            .refusal(),
        &R::Inventory(CutPlanError::UnexpectedReflection {
            copy: id(200),
            piece: id(100)
        })
    );
    let mut extra = f.pieces.first().expect("source").definition().clone();
    extra.id = id(200);
    let mut actual = f.pieces.clone();
    actual.push(Piece::new(extra, &f.ledger).expect("additional Piece"));
    let actual = PieceInventory::new(actual).expect("separate complete inventory");
    assert_eq!(
        source
            .bind_copy_inventory(&f.plan, &actual, &limits)
            .expect_err("actual Piece/copy collision")
            .refusal(),
        &R::Inventory(CutPlanError::IdentityCollision { id: id(200) })
    );
}
