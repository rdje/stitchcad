//! Complete present-role host identity choices;no fields,sharing,global reservation or issuance accepted.
#![allow(clippy::expect_used)]
#[path = "support/walk_known.rs"]
mod common;
use common::{existing::assignments::roles::copies::input::id, with_known};
use sc_core::{construction::*, ontology::*};
fn propose(role: u32, bits: u128) -> WalkNotchIdentityIntent {
    WalkNotchIdentityIntent::Propose {
        role: LocalTag::new(role),
        id: id(bits),
    }
}
fn retain(role: u32, bits: u128) -> WalkNotchIdentityIntent {
    WalkNotchIdentityIntent::Retain {
        role: LocalTag::new(role),
        notch: id(bits),
    }
}
fn raw(operation: u128, intents: Vec<WalkNotchIdentityIntent>) -> WalkNotchIdentityManifest {
    WalkNotchIdentityManifest::new(id(operation), intents)
}
#[test]
fn mixed_choices_lend_original_host_fields_active_roles_existing_objects_and_complete_actual_parents(
) {
    with_known(1, |known| {
        let raw = raw(10, vec![retain(9, 701), propose(2, 500)]);
        let snapshot = raw.clone();
        let limits = WalkNotchIdentityLimits::new(2, 2, 8);
        let before = known.existing().notches().to_vec();
        let admitted = raw
            .admit(&known, limits)
            .expect("complete explicit identity choices");
        assert!(std::ptr::eq(admitted.manifest(), &raw));
        assert!(std::ptr::eq(admitted.known(), &known));
        assert_eq!(admitted.limits(), limits);
        assert_eq!(raw.operation(), id(10));
        assert_eq!(
            (
                limits.entries(),
                limits.assignments(),
                limits.workspace_entries(),
                admitted.workspace_bound(),
                admitted.group_count(),
                admitted.retention_count(),
                admitted.proposal_count()
            ),
            (2, 2, 8, 8, 2, 1, 1)
        );
        for (action, index) in admitted.actions().zip(0..) {
            assert!(std::ptr::eq(action.owner(), &admitted));
            assert_eq!(action.index(), index);
            assert!(std::ptr::eq(
                action.intent(),
                raw.intents().get(index).expect("actual original entry")
            ));
            assert!(std::ptr::eq(action.id(), action.intent().id()));
            assert_eq!(*action.role().tag(), *action.intent().role());
            assert!(std::ptr::eq(
                action.role().catalog(),
                known.existing().assignments().roles().catalog()
            ));
        }
        let retained = admitted
            .action(LocalTag::new(9))
            .expect("actual retained choice");
        assert_eq!(retained.index(), 0);
        assert!(
            matches!(retained.source(),WalkNotchIdentitySource::Existing(notch) if std::ptr::eq(notch,known.existing().notch(id(701)).expect("provided object")))
        );
        let proposed = admitted
            .action(LocalTag::new(2))
            .expect("actual proposed identity");
        assert_eq!(proposed.index(), 1);
        assert!(
            matches!(proposed.source(),WalkNotchIdentitySource::Proposed(value) if std::ptr::eq(value,proposed.intent().id()))
        );
        assert_eq!(
            admitted
                .actions()
                .rev()
                .map(|action| *action.intent().role())
                .collect::<Vec<_>>(),
            vec![LocalTag::new(2), LocalTag::new(9)]
        );
        assert!(admitted.action(LocalTag::new(7)).is_none());
        assert_eq!(raw, snapshot);
        assert_eq!(known.existing().notches(), before);
    });
}
#[test]
fn complete_assignment_entry_workspace_limits_precede_wrong_operation_and_invalid_original_choice()
{
    use WalkNotchIdentityRefusal as R;
    with_known(1, |known| {
        let raw = raw(99, vec![propose(7, 700), retain(9, 999)]);
        let snapshot = raw.clone();
        for (limits, expected) in [
            (
                WalkNotchIdentityLimits::new(0, 1, 0),
                R::Assignments {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                WalkNotchIdentityLimits::new(1, 2, 0),
                R::Entries {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                WalkNotchIdentityLimits::new(2, 2, 7),
                R::Workspace {
                    required: 8,
                    allowed: 7,
                },
            ),
            (WalkNotchIdentityLimits::new(2, 2, 8), R::Operation),
        ] {
            let error = raw
                .admit(&known, limits)
                .expect_err("whole resource then operation");
            assert!(std::ptr::eq(error.manifest(), &raw));
            assert!(std::ptr::eq(error.known(), &known));
            assert_eq!(error.limits(), limits);
            assert_eq!(error.refusal(), expected);
            assert!(error
                .to_string()
                .starts_with("walk semantic identity proposals:"));
            assert!(format!("{error:?}").contains("WalkNotchIdentityError"));
            assert_eq!(raw, snapshot);
        }
    });
}
#[test]
fn original_duplicate_role_unused_role_duplicate_semantic_id_and_missing_existing_have_exact_causes(
) {
    use WalkNotchIdentityRefusal as R;
    with_known(1, |known| {
        let limits = WalkNotchIdentityLimits::new(2, 2, 8);
        for (raw, expected) in [
            (
                raw(10, vec![propose(2, 500), retain(2, 999)]),
                R::DuplicateRole {
                    first: 0,
                    second: 1,
                    role: LocalTag::new(2),
                },
            ),
            (
                raw(10, vec![propose(7, 700)]),
                R::UnassignedRole {
                    index: 0,
                    role: LocalTag::new(7),
                },
            ),
            (
                raw(10, vec![retain(2, 700), propose(9, 700)]),
                R::DuplicateId {
                    first: 0,
                    second: 1,
                    id: id(700),
                },
            ),
            (
                raw(10, vec![retain(2, 999)]),
                R::MissingExisting {
                    index: 0,
                    id: id(999),
                },
            ),
        ] {
            assert_eq!(
                raw.admit(&known, limits)
                    .expect_err("original actual cause")
                    .refusal(),
                expected
            );
        }
    });
}
#[test]
fn new_proposals_cannot_reuse_any_known_reference_while_retained_ids_remain_actual_prior_objects() {
    use WalkNotchIdentityRefusal as R;
    with_known(1, |known| {
        for bits in [700, 100, 200, 1, 3, 1000, 300, 301, 10] {
            let raw = raw(10, vec![propose(2, bits), retain(9, 701)]);
            assert_eq!(
                raw.admit(&known, WalkNotchIdentityLimits::new(2, 2, 8))
                    .expect_err("known id never new")
                    .refusal(),
                R::KnownProposalId {
                    index: 0,
                    id: id(bits)
                }
            );
        }
        let retained = raw(10, vec![retain(2, 700), retain(9, 701)]);
        let admitted = retained
            .admit(&known, WalkNotchIdentityLimits::new(2, 2, 8))
            .expect("explicit existing identity choices");
        assert_eq!(
            (admitted.retention_count(), admitted.proposal_count()),
            (2, 0)
        );
        assert!(admitted
            .actions()
            .all(|action| matches!(action.source(), WalkNotchIdentitySource::Existing(_))));
        let ordinary_zero = raw(10, vec![propose(2, 0), retain(9, 701)]);
        let admitted = ordinary_zero
            .admit(&known, WalkNotchIdentityLimits::new(2, 2, 8))
            .expect("explicit zero proposal is ordinary metadata,not globally reserved proof");
        assert_eq!(
            *admitted
                .action(LocalTag::new(2))
                .expect("explicit choice")
                .id(),
            id(0)
        );
    });
}
#[test]
fn every_present_role_needs_a_choice_in_original_source_order_and_unmentioned_objects_are_untouched(
) {
    use WalkNotchIdentityRefusal as R;
    with_known(1, |known| {
        let raw = raw(10, vec![retain(9, 701)]);
        assert_eq!(
            raw.admit(&known, WalkNotchIdentityLimits::new(1, 2, 5))
                .expect_err("source A before supplied B role")
                .refusal(),
            R::MissingRole {
                pair: 0,
                side: SeamSideId::A,
                role: LocalTag::new(2)
            }
        );
        let raw = WalkNotchIdentityManifest::new(id(10), vec![]);
        assert_eq!(
            raw.admit(&known, WalkNotchIdentityLimits::new(0, 2, 2))
                .expect_err("no fabricated identities")
                .refusal(),
            R::MissingRole {
                pair: 0,
                side: SeamSideId::A,
                role: LocalTag::new(2)
            }
        );
        assert_eq!(known.existing().notches().len(), 2);
    });
}
#[test]
fn four_occurrences_can_propose_two_role_identities_without_assuming_two_new_ids_per_pair() {
    with_known(2, |known| {
        let raw = raw(10, vec![propose(2, 500), propose(9, 501)]);
        let admitted = raw
            .admit(&known, WalkNotchIdentityLimits::new(2, 4, 10))
            .expect("complete present-role identity proposals,not shared-mark admission");
        assert_eq!(admitted.known().existing().assignments().request_count(), 4);
        assert_eq!(
            (
                admitted.group_count(),
                admitted.proposal_count(),
                admitted.workspace_bound()
            ),
            (2, 2, 10)
        );
        assert_eq!(
            admitted
                .actions()
                .map(|action| *action.id())
                .collect::<Vec<_>>(),
            vec![id(500), id(501)]
        );
        assert_eq!(admitted.known().existing().notches().len(), 2);
    });
}
