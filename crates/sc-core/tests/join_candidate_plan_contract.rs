//! Actual canonical candidate copy set and derived retained counts,not deep Piece/physical proof.
#![allow(clippy::expect_used)]
#[allow(dead_code)]
#[path = "support/join_candidate.rs"]
mod common;
use common::pairing::with_correspondence;
use common::{baseline_inventory, baseline_plan, definitions, id, inventory};
use sc_core::{construction::*, ontology::*};
fn limits() -> JoinCandidatePlanLimits {
    JoinCandidatePlanLimits::new(5, 3, 2, 2, 3, 11)
}
#[test]
fn actual_candidate_plan_borrows_same_inventory_and_exact_retained_result_members_with_known_zero_counts(
) {
    with_correspondence(|pairs| {
        let inventory = baseline_inventory(&pairs);
        let plan = baseline_plan(&pairs, &inventory);
        let limits = limits();
        let bound = pairs
            .bind_candidate_plan(&inventory, &plan, limits)
            .expect("complete exact canonical copy set");
        assert!(std::ptr::eq(bound.correspondence(), &pairs));
        assert!(std::ptr::eq(bound.inventory(), &inventory));
        assert!(std::ptr::eq(bound.plan(), &plan));
        assert_eq!(bound.limits(), limits);
        assert_eq!(
            (
                limits.source_copies(),
                limits.source_pieces(),
                limits.results(),
                limits.pieces(),
                limits.copies(),
                limits.workspace_entries()
            ),
            (5, 3, 2, 2, 3, 11)
        );
        assert_eq!(
            (
                bound.piece_count(),
                bound.copy_count(),
                bound.retained_copy_count(),
                bound.workspace_entries()
            ),
            (2, 3, 1, 11)
        );
        assert_eq!(bound.original_retained_count(id(100)), Some(0));
        assert_eq!(bound.original_retained_count(id(101)), Some(0));
        assert_eq!(bound.original_retained_count(id(102)), Some(1));
        assert_eq!(bound.original_retained_count(id(5000)), None);
        assert!(std::ptr::eq(
            bound.joined_piece(),
            inventory
                .piece(id(5000))
                .expect("actual supplied joined Piece")
        ));
        assert!(std::ptr::eq(
            bound.piece(id(102)).expect("actual unrelated candidate"),
            inventory.piece(id(102)).expect("candidate unrelated")
        ));
        for copy in plan.copies() {
            assert!(std::ptr::eq(
                bound
                    .copy(copy.id())
                    .expect("actual indexed candidate copy"),
                copy
            ));
        }
        assert!(bound.copy(id(200)).is_none());
        assert!(bound.copy(id(201)).is_none());
        assert!(bound.piece(id(100)).is_none());
    });
}
#[test]
fn complete_original_result_candidate_resources_precede_actual_canonical_mismatch() {
    with_correspondence(|pairs| {
        let original = baseline_inventory(&pairs);
        let plan = baseline_plan(&pairs, &original);
        let changed = inventory(&pairs, 2, Mirroring::Single, 102, false);
        use JoinCandidatePlanRefusal as R;
        for (limits, cause) in [
            (
                JoinCandidatePlanLimits::new(4, 0, 0, 0, 0, 0),
                R::SourceCopies {
                    required: 5,
                    allowed: 4,
                },
            ),
            (
                JoinCandidatePlanLimits::new(5, 2, 0, 0, 0, 0),
                R::SourcePieces {
                    required: 3,
                    allowed: 2,
                },
            ),
            (
                JoinCandidatePlanLimits::new(5, 3, 1, 0, 0, 0),
                R::Results {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                JoinCandidatePlanLimits::new(5, 3, 2, 1, 0, 0),
                R::Pieces {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                JoinCandidatePlanLimits::new(5, 3, 2, 2, 2, 0),
                R::Copies {
                    required: 3,
                    allowed: 2,
                },
            ),
            (
                JoinCandidatePlanLimits::new(5, 3, 2, 2, 3, 10),
                R::Workspace {
                    required: 11,
                    allowed: 10,
                },
            ),
        ] {
            let error = pairs
                .bind_candidate_plan(&changed, &plan, limits)
                .expect_err("whole resources before candidate mirroring/copy fields");
            assert_eq!(error.refusal(), &cause);
            assert_eq!(error.limits(), limits);
            assert!(std::ptr::eq(error.correspondence(), &pairs));
            assert!(std::ptr::eq(error.inventory(), &changed));
            assert!(std::ptr::eq(error.plan(), &plan));
            assert!(error.canonical_cause().is_none());
            assert!(error.refused_source().is_none());
            assert!(error.refused_result().is_none());
            assert!(error.candidate_copy().is_none());
        }
        let error = pairs
            .bind_candidate_plan(&changed, &plan, limits())
            .expect_err("a plan born against another inventory is not a proof here");
        assert_eq!(
            error.canonical_cause(),
            Some(&CutPlanError::UnexpectedReflection {
                copy: id(6001),
                piece: id(5000)
            })
        );
        assert_eq!(
            error.refusal(),
            &R::Canonical(Box::new(CutPlanError::UnexpectedReflection {
                copy: id(6001),
                piece: id(5000)
            }))
        );
        assert!(error.candidate_copy().is_none());
        let changed = inventory(&pairs, 4, Mirroring::MirroredPairs, 102, false);
        let error = pairs
            .bind_candidate_plan(&changed, &plan, limits())
            .expect_err("actual candidate quantity must match complete plan");
        assert_eq!(
            error.canonical_cause(),
            Some(&CutPlanError::QuantityMismatch {
                piece: id(5000),
                expected: 4,
                actual: 2
            })
        );
    });
}
#[test]
fn exact_count_and_original_retained_members_refuse_hidden_or_reassigned_copies_with_actual_loans()
{
    with_correspondence(|pairs| {
        use JoinCandidatePlanRefusal as R;
        let inv = inventory(&pairs, 2, Mirroring::MirroredPairs, 102, true);
        let mut defs = definitions(&pairs);
        defs.push(CutCopyDefinition {
            id: id(9998),
            piece: id(104),
            orientation: CopyOrientation::Authored,
        });
        let plan = CutPlan::new(defs, inv.pieces()).expect("born complete extra candidate plan");
        let error = pairs
            .bind_candidate_plan(&inv, &plan, JoinCandidatePlanLimits::new(5, 3, 2, 3, 4, 15))
            .expect_err("no hidden candidate copies");
        assert_eq!(
            error.refusal(),
            &R::Count {
                expected: 3,
                actual: 4
            }
        );
        assert!(error.refused_source().is_none());
        let inv = baseline_inventory(&pairs);
        let mut defs = definitions(&pairs);
        defs.get_mut(1).expect("unrelated candidate").id = id(9999);
        let plan = CutPlan::new(defs, inv.pieces()).expect("canonical but wrong retained identity");
        let error = pairs
            .bind_candidate_plan(&inv, &plan, limits())
            .expect_err("original retained id required");
        assert_eq!(error.refusal(), &R::MissingRetained { source: 0 });
        assert_eq!(
            error
                .refused_source()
                .expect("actual retained source action")
                .source()
                .id(),
            id(202)
        );
        assert!(error.candidate_copy().is_none());
        let inv = inventory(&pairs, 2, Mirroring::MirroredPairs, 103, false);
        let mut defs = definitions(&pairs);
        defs.get_mut(1).expect("retained candidate id").piece = id(103);
        let plan = CutPlan::new(defs, inv.pieces())
            .expect("canonical but reassigned old physical-copy id");
        let error = pairs
            .bind_candidate_plan(&inv, &plan, limits())
            .expect_err("RetainOriginal cannot reassign Piece");
        assert_eq!(error.refusal(), &R::RetainedField { source: 0 });
        assert!(std::ptr::eq(
            error
                .candidate_copy()
                .expect("actual changed candidate copy"),
            plan.copy(id(202)).expect("actual supplied copy")
        ));
        assert!(std::ptr::eq(
            error
                .refused_source()
                .expect("actual guarded original")
                .source(),
            pairs
                .sources()
                .results()
                .source_copy(id(202))
                .expect("actual original plan copy")
        ));
    });
}
#[test]
fn requested_result_members_keep_original_order_and_actual_candidate_mismatch_fields() {
    with_correspondence(|pairs| {
        use JoinCandidatePlanRefusal as R;
        let inv = baseline_inventory(&pairs);
        let mut defs = definitions(&pairs);
        defs.first_mut().expect("result6000").id = id(9999);
        let plan =
            CutPlan::new(defs, inv.pieces()).expect("canonical but missing requested result");
        let error = pairs
            .bind_candidate_plan(&inv, &plan, limits())
            .expect_err("explicit result id cannot be substituted");
        assert_eq!(error.refusal(), &R::MissingResult { copy: id(6000) });
        assert!(std::ptr::eq(
            error
                .refused_result()
                .expect("actual missing result request")
                .definition(),
            pairs
                .sources()
                .results()
                .copy(id(6000))
                .expect("actual request")
                .definition()
        ));
        assert!(error.candidate_copy().is_none());
        let mut defs = definitions(&pairs);
        defs.first_mut().expect("result6000").orientation = CopyOrientation::Reflected;
        defs.get_mut(2).expect("result6001").orientation = CopyOrientation::Authored;
        let plan = CutPlan::new(defs, inv.pieces())
            .expect("canonical balanced but request orientations swapped");
        let error = pairs
            .bind_candidate_plan(&inv, &plan, limits())
            .expect_err("original requested order6001 then6000 controls first field cause");
        assert_eq!(error.refusal(), &R::ResultField { copy: id(6001) });
        assert!(std::ptr::eq(
            error
                .refused_result()
                .expect("actual first requested result")
                .definition(),
            pairs
                .sources()
                .results()
                .manifest()
                .copies()
                .first()
                .expect("original result order")
        ));
        assert!(std::ptr::eq(
            error
                .candidate_copy()
                .expect("actual mismatching candidate"),
            plan.copy(id(6001)).expect("actual supplied copy")
        ));
        assert!(error.refused_source().is_none());
    });
}
