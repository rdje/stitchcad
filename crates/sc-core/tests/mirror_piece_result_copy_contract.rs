//! Explicit result retention/retirement/new definitions cannot mutate or retarget source copies.
#![allow(clippy::expect_used)]
#[path = "support/mirror_result_copies.rs"]
mod common;
use common::with_ids;
use sc_core::{construction::*, ontology::*};
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn issue(copy: u128, piece: u128, orientation: CopyOrientation) -> MirrorPieceResultCopyIntent {
    MirrorPieceResultCopyIntent::Issue(CutCopyDefinition {
        id: id(copy),
        piece: id(piece),
        orientation,
    })
}
fn raw(intents: Vec<MirrorPieceResultCopyIntent>) -> MirrorPieceResultCopyManifest {
    MirrorPieceResultCopyManifest::new(id(10), intents)
}

#[test]
fn initial_new_definition_borrows_original_proposal_without_minting_an_immutable_copy() {
    with_ids(1, Mirroring::Single, false, |identities| {
        // An explicit zero id is ordinary raw metadata,not a missing/default id or freshness proof.
        let raw = raw(vec![issue(0, 200, CopyOrientation::Authored)]);
        let original = raw.clone();
        let plan = identities.source().plan().clone();
        let limits = MirrorPieceResultCopyLimits::new(1, 5, 2);
        let copies = raw
            .admit(&identities, limits)
            .expect("explicit initial copy proposal");
        assert!(std::ptr::eq(copies.manifest(), &raw));
        assert!(std::ptr::eq(copies.identities(), &identities));
        assert_eq!(copies.limits(), limits);
        assert_eq!(
            (
                limits.intents(),
                limits.plan_copies(),
                limits.workspace_entries()
            ),
            (1, 5, 2)
        );
        assert_eq!(copies.workspace_entries(), 2);
        assert_eq!(
            (
                copies.retention_count(),
                copies.retirement_count(),
                copies.issuance_count(),
                copies.result_count(),
                copies.authored_count(),
                copies.reflected_count()
            ),
            (0, 0, 1, 1, 1, 0)
        );
        let action = copies.action(id(0)).expect("explicit persistent id");
        assert!(std::ptr::eq(action.owner(), &copies));
        assert_eq!(action.index(), 0);
        assert!(std::ptr::eq(
            action.intent(),
            raw.intents().first().expect("actual original intent")
        ));
        let original_definition = raw
            .intents()
            .first()
            .and_then(|intent| match intent {
                MirrorPieceResultCopyIntent::Issue(definition) => Some(definition),
                _ => None,
            })
            .expect("original explicit issue field");
        assert!(std::ptr::eq(action.definition(), original_definition));
        assert!(matches!(
            action.source(),
            MirrorPieceResultCopySource::Issue(_)
        ));
        assert!(copies.action(id(999)).is_none());
        assert_eq!(copies.actions().len(), 1);
        assert_eq!(copies.result_actions().count(), 1);
        assert_eq!(copies.identities().source().copy_count(), 4);
        assert_eq!(raw.operation(), id(10));
        assert_eq!(raw, original);
        assert_eq!(identities.source().plan(), &plan);
    });
}

#[test]
fn mixed_actions_preserve_existing_fields_and_original_order_with_retirement_explicit() {
    use MirrorPieceResultCopyIntent::{Retain, Retire};
    with_ids(2, Mirroring::Single, true, |identities| {
        let raw = raw(vec![
            issue(500, 200, CopyOrientation::Authored),
            Retire(id(551)),
            Retain(id(550)),
        ]);
        let original = raw.clone();
        let plan = identities.source().plan().clone();
        let copies = raw
            .admit(&identities, MirrorPieceResultCopyLimits::new(3, 7, 6))
            .expect("all old result ids explicitly covered");
        assert_eq!(
            (
                copies.retention_count(),
                copies.retirement_count(),
                copies.issuance_count(),
                copies.result_count(),
                copies.authored_count(),
                copies.reflected_count()
            ),
            (1, 1, 1, 2, 2, 0)
        );
        assert_eq!(
            copies
                .actions()
                .map(|a| a.definition().id)
                .collect::<Vec<_>>(),
            vec![id(500), id(551), id(550)]
        );
        assert_eq!(
            copies
                .actions()
                .rev()
                .map(|a| a.index())
                .collect::<Vec<_>>(),
            vec![2, 1, 0]
        );
        assert_eq!(
            copies
                .result_actions()
                .map(|a| a.definition().id)
                .collect::<Vec<_>>(),
            vec![id(500), id(550)]
        );
        assert_eq!(
            copies
                .result_actions()
                .rev()
                .map(|a| a.definition().id)
                .collect::<Vec<_>>(),
            vec![id(550), id(500)]
        );
        for action in copies.actions() {
            assert!(std::ptr::eq(action.owner(), &copies));
            assert!(std::ptr::eq(
                action.intent(),
                raw.intents().get(action.index()).expect("actual raw field")
            ));
            match action.source() {
                MirrorPieceResultCopySource::Retain(copy)
                | MirrorPieceResultCopySource::Retire(copy) => assert!(std::ptr::eq(
                    copy,
                    identities.copy(copy.id()).expect("actual original record")
                )),
                MirrorPieceResultCopySource::Issue(definition) => {
                    assert!(std::ptr::eq(definition, action.definition()))
                }
            }
        }
        assert_eq!(
            copies
                .action(id(550))
                .expect("persistent lookup independent of both orders")
                .index(),
            2
        );
        assert!(
            matches!(copies.action(id(551)).expect("explicit retirement").source(),MirrorPieceResultCopySource::Retire(copy) if copy.orientation()==CopyOrientation::Reflected)
        );
        assert_eq!(raw, original);
        assert_eq!(identities.source().plan(), &plan);
        assert_eq!(identities.source().copy_count(), 4);
    });
}

#[test]
fn complete_resources_precede_original_operation_and_invalid_intents() {
    use MirrorPieceResultCopyRefusal as R;
    with_ids(1, Mirroring::Single, true, |identities| {
        let raw = MirrorPieceResultCopyManifest::new(
            id(99),
            vec![
                MirrorPieceResultCopyIntent::Retain(id(999)),
                issue(500, 100, CopyOrientation::Authored),
            ],
        );
        let original = raw.clone();
        for (limits, refusal) in [
            (
                MirrorPieceResultCopyLimits::new(1, 0, 0),
                R::Intents {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                MirrorPieceResultCopyLimits::new(2, 6, 0),
                R::PlanCopies {
                    required: 7,
                    allowed: 6,
                },
            ),
            (
                MirrorPieceResultCopyLimits::new(2, 7, 3),
                R::Workspace {
                    required: 4,
                    allowed: 3,
                },
            ),
            (MirrorPieceResultCopyLimits::new(2, 7, 4), R::Operation),
        ] {
            let error = raw
                .admit(&identities, limits)
                .expect_err("whole refusal before any original intent");
            assert!(std::ptr::eq(error.manifest(), &raw));
            assert!(std::ptr::eq(error.identities(), &identities));
            assert_eq!(error.limits(), limits);
            assert_eq!(error.refusal(), refusal);
            assert!(format!("{error}").contains("mirror result-copy proposal"));
            assert_eq!(raw, original);
        }
    });
}

#[test]
fn old_result_coverage_is_explicit_and_source_or_unrelated_copies_cannot_transfer() {
    use MirrorPieceResultCopyIntent::{Retain, Retire};
    use MirrorPieceResultCopyRefusal as R;
    with_ids(1, Mirroring::Single, true, |identities| {
        for (intents, refusal) in [
            (
                vec![Retain(id(999))],
                R::MissingExisting {
                    index: 0,
                    id: id(999),
                },
            ),
            (
                vec![Retain(id(410))],
                R::ForeignExisting {
                    index: 0,
                    id: id(410),
                    piece: id(100),
                },
            ),
            (
                vec![Retire(id(333))],
                R::ForeignExisting {
                    index: 0,
                    id: id(333),
                    piece: id(300),
                },
            ),
            (
                vec![Retain(id(550))],
                R::UnmentionedExisting {
                    plan_index: 5,
                    id: id(551),
                },
            ),
            (
                vec![Retire(id(551))],
                R::UnmentionedExisting {
                    plan_index: 4,
                    id: id(550),
                },
            ),
            (
                vec![],
                R::UnmentionedExisting {
                    plan_index: 4,
                    id: id(550),
                },
            ),
        ] {
            assert_eq!(
                raw(intents)
                    .admit(&identities, MirrorPieceResultCopyLimits::new(2, 7, 4))
                    .expect_err("no implicit old-result drop or source selection")
                    .refusal(),
                refusal
            );
        }
    });
    with_ids(1, Mirroring::Single, false, |identities| {
        assert_eq!(
            raw(vec![])
                .admit(&identities, MirrorPieceResultCopyLimits::new(0, 5, 0))
                .expect_err("no fabricated initial copy")
                .refusal(),
            R::Quantity {
                expected: 1,
                actual: 0
            }
        );
    });
}

#[test]
fn known_ids_and_retirement_cannot_be_reused_and_duplicate_ids_precede_second_kind_checks() {
    use MirrorPieceResultCopyIntent::Retire;
    use MirrorPieceResultCopyRefusal as R;
    with_ids(1, Mirroring::Single, true, |identities| {
        for copy in [410, 550, 200, 40, 90, 300] {
            assert_eq!(
                raw(vec![issue(copy, 200, CopyOrientation::Authored)])
                    .admit(&identities, MirrorPieceResultCopyLimits::new(1, 7, 2))
                    .expect_err("known id is not a new issuance")
                    .refusal(),
                R::KnownIssueId {
                    index: 0,
                    id: id(copy)
                }
            );
        }
        for (intents, refusal) in [
            (
                vec![issue(500, 100, CopyOrientation::Authored)],
                R::IssuePiece {
                    index: 0,
                    expected: id(200),
                    actual: id(100),
                },
            ),
            (
                vec![
                    issue(500, 200, CopyOrientation::Authored),
                    issue(500, 100, CopyOrientation::Authored),
                ],
                R::DuplicateId {
                    first: 0,
                    second: 1,
                    id: id(500),
                },
            ),
            (
                vec![Retire(id(551)), issue(551, 200, CopyOrientation::Authored)],
                R::DuplicateId {
                    first: 0,
                    second: 1,
                    id: id(551),
                },
            ),
        ] {
            assert_eq!(
                raw(intents)
                    .admit(&identities, MirrorPieceResultCopyLimits::new(2, 7, 4))
                    .expect_err("no id reuse or silent kind fallback")
                    .refusal(),
                refusal
            );
        }
    });
}

#[test]
fn actual_result_cut_policy_owns_active_orientations_quantity_and_pair_balance() {
    use CopyOrientation::{Authored as A, Reflected as Rf};
    use MirrorPieceResultCopyRefusal as R;
    for mirroring in [
        Mirroring::Single,
        Mirroring::PairMember {
            handedness: Handedness::Left,
            companion: id(888),
        },
    ] {
        with_ids(1, mirroring, false, |identities| {
            assert_eq!(
            raw(vec![issue(500, 200, Rf), issue(501, 200, A)])
                .admit(&identities, MirrorPieceResultCopyLimits::new(2, 5, 4))
                .expect_err(
                    "result authored-only policy refuses reflection before a simultaneous quantity mismatch"
                )
                .refusal(),
            R::UnexpectedReflection {
                index: 0,
                id: id(500)
            }
        );
            assert_eq!(
                raw(vec![issue(500, 200, A), issue(501, 200, A)])
                    .admit(&identities, MirrorPieceResultCopyLimits::new(2, 5, 4))
                    .expect_err("complete result quantity")
                    .refusal(),
                R::Quantity {
                    expected: 1,
                    actual: 2
                }
            );
        });
    }
    with_ids(2, Mirroring::MirroredPairs, false, |identities| {
        assert_eq!(
            raw(vec![issue(500, 200, A)])
                .admit(&identities, MirrorPieceResultCopyLimits::new(2, 5, 4))
                .expect_err("quantity before imbalance")
                .refusal(),
            R::Quantity {
                expected: 2,
                actual: 1
            }
        );
        assert_eq!(
            raw(vec![issue(500, 200, A), issue(501, 200, A)])
                .admit(&identities, MirrorPieceResultCopyLimits::new(2, 5, 4))
                .expect_err("complete active pair balance")
                .refusal(),
            R::UnbalancedPairs {
                authored: 2,
                reflected: 0
            }
        );
        let raw = raw(vec![issue(500, 200, A), issue(501, 200, Rf)]);
        let copies = raw
            .admit(&identities, MirrorPieceResultCopyLimits::new(2, 5, 4))
            .expect("explicit paired result");
        assert_eq!(
            (
                copies.result_count(),
                copies.authored_count(),
                copies.reflected_count()
            ),
            (2, 1, 1)
        );
    });
}
