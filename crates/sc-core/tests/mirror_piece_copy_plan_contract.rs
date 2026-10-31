//! Complete actual candidate plan/inventory association;all authored copy identities remain explicit.
#![allow(clippy::expect_used)]
#[path = "support/mirror_copy_inventory.rs"]
mod common;
use common::copies::context::edges::metadata::source as sources;
use common::copies::{with_cut, with_ids};
use common::inventory;
use sc_core::{construction::*, ontology::*};
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn keep(_: &mut PieceDefinition) {}
fn issue(bits: u128, orientation: CopyOrientation) -> MirrorPieceResultCopyIntent {
    MirrorPieceResultCopyIntent::Issue(CutCopyDefinition {
        id: id(bits),
        piece: id(200),
        orientation,
    })
}
fn initial() -> MirrorPieceResultCopyManifest {
    MirrorPieceResultCopyManifest::new(id(10), vec![issue(500, CopyOrientation::Authored)])
}
fn rows(copies: &AdmittedMirrorPieceResultCopies<'_>) -> Vec<CutCopyDefinition> {
    let mut result = copies
        .identities()
        .source()
        .plan()
        .copies()
        .iter()
        .filter(|copy| copy.piece() != id(200))
        .map(|copy| *copy.definition())
        .collect::<Vec<_>>();
    result.extend(copies.result_actions().map(|action| *action.definition()));
    result
}
fn full_limits(b: usize, c: usize, i: usize, n: usize) -> MirrorPieceCopyPlanLimits {
    MirrorPieceCopyPlanLimits::new(b, c, i, n, i + 3 * c)
}
#[test]
fn complete_candidate_keeps_actual_borrows_order_and_identity_for_all_result_modes() {
    for (quantity, mirroring, fold) in [
        (1, Mirroring::Single, MirrorPieceFoldIntent::NoFold),
        (
            2,
            Mirroring::MirroredPairs,
            MirrorPieceFoldIntent::ReflectedSource,
        ),
        (
            1,
            Mirroring::PairMember {
                handedness: Handedness::Left,
                companion: id(300),
            },
            MirrorPieceFoldIntent::NoFold,
        ),
    ] {
        with_cut(quantity, mirroring, fold, false, |ids| {
            let mut intents = vec![issue(500, CopyOrientation::Authored)];
            if mirroring == Mirroring::MirroredPairs {
                intents.push(issue(501, CopyOrientation::Reflected));
            }
            let raw = MirrorPieceResultCopyManifest::new(id(10), intents);
            let n = raw.intents().len();
            let copies = raw
                .admit(&ids, MirrorPieceResultCopyLimits::new(n, 5, 2 * n))
                .expect("native choices");
            let inventory = inventory(&copies, keep, keep);
            let snapshot = inventory.clone();
            let invlimits = MirrorPieceCopyInventoryLimits::new(3, 8, 32, 220);
            let bound = copies
                .bind_piece_inventory(&inventory, &invlimits)
                .expect("actual candidate fields");
            let mut definitions = rows(&copies);
            definitions.reverse();
            let plan = CutPlan::new(definitions, inventory.pieces()).expect("canonical candidate");
            let original = plan.clone();
            let limits = full_limits(5, plan.copies().len(), 3, n);
            let admitted = bound
                .bind_cut_plan(&plan, &limits)
                .expect("complete actual association");
            assert!(std::ptr::eq(admitted.inventory(), &bound));
            assert!(std::ptr::eq(admitted.plan(), &plan));
            assert!(std::ptr::eq(admitted.limits(), &limits));
            assert_eq!(admitted.workspace_bound(), 3 + 3 * plan.copies().len());
            assert_eq!(
                (
                    limits.before_copies(),
                    limits.after_copies(),
                    limits.pieces(),
                    limits.intents(),
                    limits.workspace_entries()
                ),
                (5, plan.copies().len(), 3, n, 3 + 3 * plan.copies().len())
            );
            assert_eq!(
                admitted.plan().copies().first().expect("actual first").id(),
                if quantity == 2 { id(501) } else { id(500) }
            );
            for copy in plan.copies() {
                assert!(std::ptr::eq(
                    admitted.copy(copy.id()).expect("indexed actual"),
                    copy
                ));
            }
            assert!(admitted.copy(id(999)).is_none());
            assert_eq!(inventory, snapshot);
            assert_eq!(plan, original);
            assert_eq!(
                admitted
                    .copy(id(410))
                    .expect("unchanged source")
                    .orientation(),
                CopyOrientation::Authored
            );
            assert_eq!(
                admitted
                    .copy(id(207))
                    .expect("unchanged source")
                    .orientation(),
                CopyOrientation::Reflected
            );
            if matches!(mirroring, Mirroring::PairMember { .. }) {
                assert_eq!(
                    admitted
                        .inventory()
                        .result_companion()
                        .expect("actual reciprocal")
                        .id(),
                    id(300)
                );
                assert_eq!(
                    admitted
                        .copy(id(333))
                        .expect("companion's old copy")
                        .piece(),
                    id(300)
                );
            }
        });
    }
}
#[test]
fn mixed_native_retain_retire_issue_matches_complete_actual_plan_without_mutation() {
    with_ids(2, Mirroring::Single, true, |ids| {
        let raw = MirrorPieceResultCopyManifest::new(
            id(10),
            vec![
                MirrorPieceResultCopyIntent::Retire(id(551)),
                issue(500, CopyOrientation::Authored),
                MirrorPieceResultCopyIntent::Retain(id(550)),
            ],
        );
        let copies = raw
            .admit(&ids, MirrorPieceResultCopyLimits::new(3, 7, 6))
            .expect("complete explicit choices");
        let inventory = inventory(&copies, keep, keep);
        let invlimits = MirrorPieceCopyInventoryLimits::new(3, 8, 30, 220);
        let bound = copies
            .bind_piece_inventory(&inventory, &invlimits)
            .expect("actual candidate");
        let before = copies.identities().source().plan().clone();
        let plan = CutPlan::new(rows(&copies), inventory.pieces()).expect("actual candidate");
        let limits = full_limits(7, 7, 3, 3);
        let admitted = bound
            .bind_cut_plan(&plan, &limits)
            .expect("complete actual plan");
        assert!(admitted.copy(id(551)).is_none());
        assert_eq!(
            admitted.copy(id(550)).expect("retained").definition(),
            before.copy(id(550)).expect("before").definition()
        );
        assert_eq!(
            admitted.copy(id(500)).expect("issued candidate").piece(),
            id(200)
        );
        assert_eq!(
            admitted
                .plan()
                .copies()
                .iter()
                .map(|c| c.id())
                .collect::<Vec<_>>(),
            vec![
                id(410),
                id(207),
                id(900),
                id(805),
                id(333),
                id(500),
                id(550)
            ]
        );
        assert_eq!(*copies.identities().source().plan(), before);
    });
}
#[test]
fn whole_resources_precede_any_invalid_candidate_identity_and_keep_actual_error_parents() {
    use MirrorPieceCopyPlanRefusal as R;
    with_ids(1, Mirroring::Single, false, |ids| {
        let raw = initial();
        let copies = raw
            .admit(&ids, MirrorPieceResultCopyLimits::new(1, 5, 2))
            .expect("native");
        let inventory = inventory(&copies, keep, keep);
        let invlimits = MirrorPieceCopyInventoryLimits::new(3, 8, 30, 220);
        let bound = copies
            .bind_piece_inventory(&inventory, &invlimits)
            .expect("inventory");
        let mut definitions = rows(&copies);
        definitions.last_mut().expect("result").id = id(999);
        let plan = CutPlan::new(definitions, inventory.pieces())
            .expect("structurally valid different proposal");
        for (limits, expected) in [
            (
                MirrorPieceCopyPlanLimits::new(4, 0, 0, 0, 0),
                R::BeforeCopies {
                    required: 5,
                    allowed: 4,
                },
            ),
            (
                MirrorPieceCopyPlanLimits::new(5, 5, 0, 0, 0),
                R::AfterCopies {
                    required: 6,
                    allowed: 5,
                },
            ),
            (
                MirrorPieceCopyPlanLimits::new(5, 6, 2, 0, 0),
                R::Pieces {
                    required: 3,
                    allowed: 2,
                },
            ),
            (
                MirrorPieceCopyPlanLimits::new(5, 6, 3, 0, 0),
                R::Intents {
                    required: 1,
                    allowed: 0,
                },
            ),
            (
                MirrorPieceCopyPlanLimits::new(5, 6, 3, 1, 20),
                R::Workspace {
                    required: 21,
                    allowed: 20,
                },
            ),
            (
                full_limits(5, 6, 3, 1),
                R::UnexpectedResult {
                    index: 5,
                    id: id(999),
                },
            ),
        ] {
            let error = bound
                .bind_cut_plan(&plan, &limits)
                .expect_err("whole refusal");
            assert_eq!(error.refusal(), &expected);
            assert!(std::ptr::eq(error.inventory(), &bound));
            assert!(std::ptr::eq(error.plan(), &plan));
            assert!(std::ptr::eq(error.limits(), &limits));
            assert!(error.to_string().starts_with("mirror candidate cut plan:"));
        }
    });
}
#[test]
fn candidate_birth_against_other_pieces_cannot_substitute_for_actual_inventory_association() {
    use MirrorPieceCopyPlanRefusal as R;
    with_ids(1, Mirroring::Single, false, |ids| {
        let raw = initial();
        let copies = raw
            .admit(&ids, MirrorPieceResultCopyLimits::new(1, 5, 2))
            .expect("native");
        let inventory = inventory(&copies, keep, keep);
        let invlimits = MirrorPieceCopyInventoryLimits::new(4, 8, 30, 220);
        let bound = copies
            .bind_piece_inventory(&inventory, &invlimits)
            .expect("inventory");
        let missing = CutPlan::new(
            copies
                .identities()
                .source()
                .plan()
                .copies()
                .iter()
                .map(|c| *c.definition())
                .collect(),
            &inventory
                .pieces()
                .iter()
                .filter(|p| p.id() != id(200))
                .cloned()
                .collect::<Vec<_>>(),
        )
        .expect("birth without result Piece");
        let limits = full_limits(5, 8, 4, 1);
        assert_eq!(
            bound
                .bind_cut_plan(&missing, &limits)
                .expect_err("different birth")
                .refusal(),
            &R::Inventory(CutPlanError::QuantityMismatch {
                piece: id(200),
                expected: 1,
                actual: 0
            })
        );
        let (ledger, _) = sources::fixture();
        let mut birth = inventory.pieces().to_vec();
        let position = birth.iter().position(|p| p.id() == id(300)).expect("other");
        let mut other = birth
            .get(position)
            .expect("actual other slot")
            .definition()
            .clone();
        other.quantity = 2;
        other.mirroring = Mirroring::MirroredPairs;
        *birth.get_mut(position).expect("actual other slot") =
            Piece::new(other.clone(), &ledger).expect("alternate birth other");
        let mut definitions = rows(&copies);
        definitions
            .iter_mut()
            .find(|c| c.id == id(333))
            .expect("other")
            .orientation = CopyOrientation::Reflected;
        definitions.push(CutCopyDefinition {
            id: id(334),
            piece: id(300),
            orientation: CopyOrientation::Authored,
        });
        let different = CutPlan::new(definitions, &birth).expect("valid alternate paired birth");
        assert_eq!(
            bound
                .bind_cut_plan(&different, &limits)
                .expect_err("actual authored-only other")
                .refusal(),
            &R::Inventory(CutPlanError::UnexpectedReflection {
                copy: id(333),
                piece: id(300)
            })
        );
        other.mirroring = Mirroring::Single;
        *birth.get_mut(position).expect("actual other slot") =
            Piece::new(other, &ledger).expect("alternate single birth");
        let mut definitions = rows(&copies);
        definitions.push(CutCopyDefinition {
            id: id(334),
            piece: id(300),
            orientation: CopyOrientation::Authored,
        });
        let different = CutPlan::new(definitions, &birth).expect("valid alternate quantity birth");
        assert_eq!(
            bound
                .bind_cut_plan(&different, &limits)
                .expect_err("actual quantity")
                .refusal(),
            &R::Inventory(CutPlanError::QuantityMismatch {
                piece: id(300),
                expected: 1,
                actual: 2
            })
        );
        let plan = CutPlan::new(rows(&copies), inventory.pieces())
            .expect("valid birth before extra Piece");
        let mut extra = inventory
            .piece(id(100))
            .expect("source")
            .definition()
            .clone();
        extra.id = id(500);
        extra.quantity = 1;
        extra.mirroring = Mirroring::Single;
        let mut actual = inventory.pieces().to_vec();
        actual.push(Piece::new(extra, &ledger).expect("extra actual Piece"));
        let actual = PieceInventory::new(actual).expect("complete independent actual inventory");
        let bound = copies
            .bind_piece_inventory(&actual, &invlimits)
            .expect("source/result still exact");
        assert_eq!(
            bound
                .bind_cut_plan(&plan, &limits)
                .expect_err("actual Piece aliases copy")
                .refusal(),
            &R::Inventory(CutPlanError::IdentityCollision { id: id(500) })
        );
    });
}
#[test]
fn original_source_and_other_copy_fields_and_complete_preservation_cannot_change() {
    use MirrorPieceCopyPlanRefusal as R;
    with_ids(1, Mirroring::Single, false, |ids| {
        let raw = initial();
        let copies = raw
            .admit(&ids, MirrorPieceResultCopyLimits::new(1, 5, 2))
            .expect("native");
        let inventory = inventory(&copies, keep, keep);
        let invlimits = MirrorPieceCopyInventoryLimits::new(3, 8, 30, 220);
        let bound = copies
            .bind_piece_inventory(&inventory, &invlimits)
            .expect("inventory");
        let limits = full_limits(5, 6, 3, 1);
        let mut definitions = rows(&copies);
        definitions
            .get_mut(0)
            .expect("explicit fixture row")
            .orientation = CopyOrientation::Reflected;
        definitions
            .get_mut(1)
            .expect("explicit fixture row")
            .orientation = CopyOrientation::Authored;
        let changed =
            CutPlan::new(definitions, inventory.pieces()).expect("balanced but changed source");
        assert_eq!(
            bound
                .bind_cut_plan(&changed, &limits)
                .expect_err("source immutable")
                .refusal(),
            &R::PreservedChanged {
                index: 0,
                id: id(410)
            }
        );
        let mut definitions = rows(&copies);
        definitions.get_mut(4).expect("explicit fixture row").id = id(334);
        let extra =
            CutPlan::new(definitions, inventory.pieces()).expect("different other identity");
        assert_eq!(
            bound
                .bind_cut_plan(&extra, &limits)
                .expect_err("other cannot be recreated")
                .refusal(),
            &R::UnexpectedOther {
                index: 4,
                id: id(334)
            }
        );
        let mut definitions = rows(&copies);
        definitions.get_mut(0).expect("explicit fixture row").piece = id(300);
        definitions.get_mut(4).expect("explicit fixture row").piece = id(100);
        let retargeted = CutPlan::new(definitions, inventory.pieces())
            .expect("same counts but retargeted source/other");
        assert_eq!(
            bound
                .bind_cut_plan(&retargeted, &limits)
                .expect_err("cannot transfer")
                .refusal(),
            &R::PreservedChanged {
                index: 0,
                id: id(410)
            }
        );
        let reduced = PieceInventory::new(
            inventory
                .pieces()
                .iter()
                .filter(|p| p.id() != id(300))
                .cloned()
                .collect(),
        )
        .expect("structural reduced inventory");
        let bound = copies
            .bind_piece_inventory(&reduced, &invlimits)
            .expect("source/result still present");
        let plan = CutPlan::new(
            rows(&copies)
                .into_iter()
                .filter(|c| c.id != id(333))
                .collect(),
            reduced.pieces(),
        )
        .expect("canonical reduced plan");
        assert_eq!(
            bound
                .bind_cut_plan(&plan, &limits)
                .expect_err("all before non-results preserved")
                .refusal(),
            &R::MissingPreserved {
                index: 4,
                id: id(333)
            }
        );
    });
}
#[test]
fn retained_result_definition_retired_presence_and_unlisted_results_refuse_in_candidate_order() {
    use MirrorPieceCopyPlanRefusal as R;
    with_ids(2, Mirroring::MirroredPairs, true, |ids| {
        let raw = MirrorPieceResultCopyManifest::new(
            id(10),
            vec![
                MirrorPieceResultCopyIntent::Retain(id(550)),
                MirrorPieceResultCopyIntent::Retire(id(551)),
                issue(500, CopyOrientation::Reflected),
            ],
        );
        let copies = raw
            .admit(&ids, MirrorPieceResultCopyLimits::new(3, 7, 6))
            .expect("native");
        let inventory = inventory(&copies, keep, keep);
        let invlimits = MirrorPieceCopyInventoryLimits::new(3, 8, 30, 220);
        let bound = copies
            .bind_piece_inventory(&inventory, &invlimits)
            .expect("inventory");
        let limits = full_limits(7, 7, 3, 3);
        let mut definitions = rows(&copies);
        definitions
            .get_mut(5)
            .expect("explicit fixture row")
            .orientation = CopyOrientation::Reflected;
        definitions
            .get_mut(6)
            .expect("explicit fixture row")
            .orientation = CopyOrientation::Authored;
        let changed = CutPlan::new(definitions, inventory.pieces())
            .expect("balanced but changed retained orientation");
        assert_eq!(
            bound
                .bind_cut_plan(&changed, &limits)
                .expect_err("retained exact")
                .refusal(),
            &R::ResultChanged {
                index: 5,
                id: id(550)
            }
        );
        let mut definitions = rows(&copies);
        definitions.get_mut(6).expect("explicit fixture row").id = id(551);
        let retired = CutPlan::new(definitions, inventory.pieces())
            .expect("old retired id remains structurally valid");
        assert_eq!(
            bound
                .bind_cut_plan(&retired, &limits)
                .expect_err("must retire")
                .refusal(),
            &R::RetiredPresent {
                index: 6,
                id: id(551)
            }
        );
        let mut definitions = rows(&copies);
        definitions.get_mut(6).expect("explicit fixture row").id = id(999);
        let unlisted =
            CutPlan::new(definitions, inventory.pieces()).expect("unlisted structural result");
        assert_eq!(
            bound
                .bind_cut_plan(&unlisted, &limits)
                .expect_err("no unlisted result")
                .refusal(),
            &R::UnexpectedResult {
                index: 6,
                id: id(999)
            }
        );
    });
}
