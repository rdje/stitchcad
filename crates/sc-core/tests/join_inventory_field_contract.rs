//! Whole original/candidate body resources,independent of semantic preservation or geometry.
#![allow(clippy::expect_used)]
#[allow(dead_code)]
#[path = "support/join_candidate.rs"]
mod common;
use common::pairing::with_correspondence;
use common::{baseline_inventory, baseline_plan, id, ledger, with_bound_plan};
use sc_core::{construction::*, ontology::*};
fn with_variant(
    unicode: bool,
    hole: bool,
    material: Option<MaterialAssignment>,
    run: impl FnOnce(BoundJoinCandidatePlan<'_>),
) {
    with_correspondence(|pairs| {
        let original = baseline_inventory(&pairs);
        let ledger = ledger();
        let pieces = original
            .pieces()
            .iter()
            .map(|piece| {
                let mut d = piece.definition().clone();
                if d.id == id(102) {
                    if unicode {
                        d.label.name = "ééé".into();
                    }
                    if hole {
                        d.holes.push(vec![DirectedEdge {
                            edge: EdgeRef::new(id(2), LocalTag::new(1)),
                            direction: Direction::Original,
                        }]);
                    }
                }
                if d.id == id(5000) {
                    if let Some(material) = &material {
                        d.material = material.clone();
                    }
                }
                Piece::new(d, &ledger)
                    .expect("born altered candidate;deep field matching stays pending")
            })
            .collect();
        let inventory = PieceInventory::new(pieces).expect("actual complete candidate inventory");
        let plan = baseline_plan(&pairs, &inventory);
        let bound = pairs
            .bind_candidate_plan(
                &inventory,
                &plan,
                JoinCandidatePlanLimits::new(5, 3, 2, 2, 3, 11),
            )
            .expect("canonical exact copy plan does not certify body fields");
        run(bound);
    });
}
#[test]
fn complete_both_inventories_count_consumed_unrelated_and_separate_fold_fields_with_actual_loans() {
    with_bound_plan(|plan| {
        let limits = JoinInventoryFieldLimits::new(3, 2, 0, 24, 114);
        let admitted = plan
            .admit_piece_fields(limits)
            .expect("whole cached source/candidate resources");
        assert!(std::ptr::eq(admitted.plan(), &plan));
        assert_eq!(admitted.limits(), limits);
        assert_eq!(
            (
                limits.sources(),
                limits.pieces(),
                limits.holes(),
                limits.occurrences(),
                limits.text_bytes()
            ),
            (3, 2, 0, 24, 114)
        );
        assert_eq!(
            (
                admitted.hole_count(),
                admitted.occurrence_count(),
                admitted.text_bytes()
            ),
            (0, 24, 114)
        );
        assert_eq!(
            admitted
                .original_pieces()
                .map(Piece::id)
                .collect::<Vec<_>>(),
            [id(100), id(101), id(102)]
        );
        assert_eq!(
            admitted
                .original_pieces()
                .rev()
                .map(Piece::id)
                .collect::<Vec<_>>(),
            [id(102), id(101), id(100)]
        );
        assert_eq!(
            admitted
                .candidate_pieces()
                .map(Piece::id)
                .collect::<Vec<_>>(),
            [id(5000), id(102)]
        );
        for (actual, original) in admitted
            .original_pieces()
            .zip(plan.original_inventory().pieces())
        {
            assert!(std::ptr::eq(actual, original));
        }
        for (actual, candidate) in admitted.candidate_pieces().zip(plan.inventory().pieces()) {
            assert!(std::ptr::eq(actual, candidate));
        }
        // Original R15 (three5-field Pieces),candidate R9 (joined4+unrelated5),including four
        // separate fold declarations. Text B71 original+B43 candidate;equal content counts again.
        assert_eq!(plan.original_retained_count(id(100)), Some(0));
        assert_eq!(admitted.original_pieces().len(), 3);
    });
}
#[test]
fn whole_headers_holes_fields_and_text_precede_later_child_or_semantic_inspection() {
    with_variant(true, true, None, |plan| {
        use JoinInventoryFieldRefusal as R;
        for (limits, cause) in [
            (
                JoinInventoryFieldLimits::new(2, 0, 0, 0, 0),
                R::Sources {
                    required: 3,
                    allowed: 2,
                },
            ),
            (
                JoinInventoryFieldLimits::new(3, 1, 0, 0, 0),
                R::Pieces {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                JoinInventoryFieldLimits::new(3, 2, 0, 0, 0),
                R::Holes {
                    required: 1,
                    allowed: 0,
                },
            ),
            (
                JoinInventoryFieldLimits::new(3, 2, 1, 24, 0),
                R::Occurrences {
                    required: 25,
                    allowed: 24,
                },
            ),
            (
                JoinInventoryFieldLimits::new(3, 2, 1, 25, 115),
                R::TextBytes {
                    required: 116,
                    allowed: 115,
                },
            ),
        ] {
            let error = plan
                .admit_piece_fields(limits)
                .expect_err("whole complete resources before later data or comparison");
            assert_eq!(error.refusal(), cause);
            assert_eq!(error.limits(), limits);
            assert!(std::ptr::eq(error.plan(), &plan));
        }
        let admitted = plan
            .admit_piece_fields(JoinInventoryFieldLimits::new(3, 2, 1, 25, 116))
            .expect("exact resources,not semantic original preservation");
        assert_eq!(
            (
                admitted.hole_count(),
                admitted.occurrence_count(),
                admitted.text_bytes()
            ),
            (1, 25, 116)
        );
    });
}
#[test]
fn utf8_bytes_and_original_candidate_occurrences_are_retained_without_normalization_or_field_approval(
) {
    with_variant(true, true, None, |plan| {
        let admitted = plan
            .admit_piece_fields(JoinInventoryFieldLimits::new(3, 2, 1, 25, 116))
            .expect("whole actual cached lengths");
        let candidate = admitted
            .plan()
            .piece(id(102))
            .expect("actual unrelated candidate");
        let original = admitted
            .plan()
            .original_inventory()
            .piece(id(102))
            .expect("actual unrelated original");
        assert_eq!(candidate.definition().label.name.chars().count(), 3);
        assert_eq!(candidate.definition().label.name.len(), 6);
        assert_eq!(original.definition().label.name, "test");
        assert_eq!(candidate.definition().label.name, "ééé");
        assert_eq!(candidate.definition().holes.len(), 1);
        assert_ne!(candidate.definition().label, original.definition().label); // Resource presence is not equality/permission to change an original.
        assert!(std::ptr::eq(
            admitted
                .candidate_pieces()
                .next_back()
                .expect("last actual candidate"),
            candidate
        ));
    });
}
#[test]
fn assigned_material_has_no_reason_bytes_and_unresolved_reason_bytes_remain_whole_actual_data() {
    for (material, bytes) in [
        (MaterialAssignment::Assigned(id(9000)), 101),
        (MaterialAssignment::Unresolved("ééé".into()), 107),
    ] {
        with_variant(false, false, Some(material.clone()), |plan| {
            let admitted = plan
                .admit_piece_fields(JoinInventoryFieldLimits::new(3, 2, 0, 24, bytes))
                .expect("whole material representation bytes only");
            assert_eq!(admitted.text_bytes(), bytes);
            assert_eq!(
                admitted.plan().joined_piece().definition().material,
                material
            );
            let error = plan
                .admit_piece_fields(JoinInventoryFieldLimits::new(3, 2, 0, 24, bytes - 1))
                .expect_err("no selected prefix can hide the final material bytes");
            assert_eq!(
                error.refusal(),
                JoinInventoryFieldRefusal::TextBytes {
                    required: bytes,
                    allowed: bytes - 1
                }
            );
        });
    }
}
