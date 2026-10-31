//! Explicit exact-field coalescing and retained-field choices;no physical/effect publication proof.
#![allow(clippy::expect_used)]
#[path = "support/walk_field_context.rs"]
mod common;
use common::{
    known::{
        existing::assignments::roles::copies::input::{id, profiles, t},
        with_known,
    },
    with_requests, Request,
};
use sc_core::{construction::*, ontology::*};
use WalkNotchFieldPolicy::{NewRequested, RetainUnchanged, ReviseRetained};
use WalkNotchSharingPolicy::{CoalesceIdentical, SingleOccurrence};
fn intent(
    role: u32,
    sharing: WalkNotchSharingPolicy,
    fields: WalkNotchFieldPolicy,
) -> WalkNotchFieldIntent {
    WalkNotchFieldIntent::new(LocalTag::new(role), sharing, fields)
}
fn raw(choices: Vec<WalkNotchFieldIntent>) -> WalkNotchFieldManifest {
    WalkNotchFieldManifest::new(id(10), choices)
}
fn proposals(tags: &[u32]) -> WalkNotchIdentityManifest {
    WalkNotchIdentityManifest::new(
        id(10),
        tags.iter()
            .enumerate()
            .map(|(index, tag)| WalkNotchIdentityIntent::Propose {
                role: LocalTag::new(*tag),
                id: id(500 + index as u128),
            })
            .collect(),
    )
}
fn retained() -> WalkNotchIdentityManifest {
    WalkNotchIdentityManifest::new(
        id(10),
        vec![
            WalkNotchIdentityIntent::Retain {
                role: LocalTag::new(2),
                notch: id(700),
            },
            WalkNotchIdentityIntent::Retain {
                role: LocalTag::new(9),
                notch: id(701),
            },
        ],
    )
}
#[test]
fn original_policy_fields_common_source_and_actual_identity_parents_are_borrowed_only_after_complete_success(
) {
    with_known(1, |known| {
        let ids = WalkNotchIdentityManifest::new(
            id(10),
            vec![
                WalkNotchIdentityIntent::Propose {
                    role: LocalTag::new(2),
                    id: id(500),
                },
                WalkNotchIdentityIntent::Retain {
                    role: LocalTag::new(9),
                    notch: id(701),
                },
            ],
        );
        let ids = ids
            .admit(&known, WalkNotchIdentityLimits::new(2, 2, 8))
            .expect("identities");
        let raw = raw(vec![
            intent(9, SingleOccurrence, RetainUnchanged),
            intent(2, SingleOccurrence, NewRequested),
        ]);
        let snapshot = raw.clone();
        let old = known.existing().notches().to_vec();
        let limits = WalkNotchFieldLimits::new(2, 2, 2, 6);
        let fields = raw.admit(&ids, limits).expect("explicit fields");
        assert!(std::ptr::eq(fields.manifest(), &raw));
        assert!(std::ptr::eq(fields.identities(), &ids));
        assert_eq!(fields.limits(), limits);
        assert_eq!(
            (
                limits.identities(),
                limits.assignments(),
                limits.policies(),
                limits.workspace_entries(),
                fields.workspace_bound(),
                fields.group_count()
            ),
            (2, 2, 2, 6, 6, 2)
        );
        assert_eq!(raw.operation(), id(10));
        for (group, index) in fields.groups().zip(0..) {
            assert!(std::ptr::eq(group.owner(), &fields));
            assert_eq!(group.index(), index);
            assert!(std::ptr::eq(
                group.intent(),
                raw.intents().get(index).expect("original policy")
            ));
            assert!(std::ptr::eq(group.identity().owner(), &ids));
            assert_eq!(*group.identity().role().tag(), *group.intent().role());
            assert_eq!(group.occurrence_count(), 1);
            let source = group.first();
            assert!(std::ptr::eq(source.owner(), known.existing().assignments()));
            assert!(std::ptr::eq(group.piece(), source.piece()));
            assert!(std::ptr::eq(group.anchor(), source.requested().anchor()));
            assert!(std::ptr::eq(
                group.representation(),
                source.requested().representation()
            ));
        }
        assert_eq!(
            fields
                .groups()
                .rev()
                .map(|g| *g.intent().role())
                .collect::<Vec<_>>(),
            vec![LocalTag::new(2), LocalTag::new(9)]
        );
        assert!(fields.group(LocalTag::new(7)).is_none());
        assert_eq!(raw, snapshot);
        assert_eq!(known.existing().notches(), old);
    });
}
#[test]
fn complete_m_e_f_workspace_quotas_precede_operation_and_invalid_first_policy() {
    use WalkNotchFieldRefusal as R;
    with_known(1, |known| {
        let ids = proposals(&[2, 9]);
        let ids = ids
            .admit(&known, WalkNotchIdentityLimits::new(2, 2, 8))
            .expect("ids");
        let raw = WalkNotchFieldManifest::new(
            id(99),
            vec![
                intent(7, SingleOccurrence, ReviseRetained),
                intent(7, CoalesceIdentical, NewRequested),
            ],
        );
        let snapshot = raw.clone();
        for (limits, cause) in [
            (
                WalkNotchFieldLimits::new(0, 1, 0, 0),
                R::Assignments {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                WalkNotchFieldLimits::new(1, 2, 0, 0),
                R::Identities {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                WalkNotchFieldLimits::new(2, 2, 1, 0),
                R::Policies {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                WalkNotchFieldLimits::new(2, 2, 2, 5),
                R::Workspace {
                    required: 6,
                    allowed: 5,
                },
            ),
            (WalkNotchFieldLimits::new(2, 2, 2, 6), R::Operation),
        ] {
            let error = raw.admit(&ids, limits).expect_err("whole bound first");
            assert!(std::ptr::eq(error.manifest(), &raw));
            assert!(std::ptr::eq(error.identities(), &ids));
            assert_eq!(error.limits(), limits);
            assert_eq!(error.refusal(), cause);
            assert!(error.to_string().starts_with("walk semantic field policy:"));
            assert!(format!("{error:?}").contains("WalkNotchFieldError"));
            assert_eq!(raw, snapshot);
        }
    });
}
#[test]
fn original_policy_duplicates_membership_kind_then_source_coverage_have_deterministic_causes() {
    use WalkNotchFieldRefusal as R;
    with_known(1, |known| {
        let ids = proposals(&[2, 9]);
        let ids = ids
            .admit(&known, WalkNotchIdentityLimits::new(2, 2, 8))
            .expect("ids");
        let limits = WalkNotchFieldLimits::new(2, 2, 2, 6);
        for (raw, cause) in [
            (
                raw(vec![
                    intent(2, SingleOccurrence, NewRequested),
                    intent(2, SingleOccurrence, ReviseRetained),
                ]),
                R::DuplicateRole {
                    first: 0,
                    second: 1,
                    role: LocalTag::new(2),
                },
            ),
            (
                raw(vec![intent(7, SingleOccurrence, NewRequested)]),
                R::UnassignedRole {
                    index: 0,
                    role: LocalTag::new(7),
                },
            ),
            (
                raw(vec![intent(2, SingleOccurrence, RetainUnchanged)]),
                R::IdentityKind {
                    index: 0,
                    role: LocalTag::new(2),
                },
            ),
            (
                raw(vec![intent(9, SingleOccurrence, NewRequested)]),
                R::MissingRole {
                    pair: 0,
                    side: SeamSideId::A,
                    role: LocalTag::new(2),
                },
            ),
            (
                raw(vec![]),
                R::MissingRole {
                    pair: 0,
                    side: SeamSideId::A,
                    role: LocalTag::new(2),
                },
            ),
        ] {
            assert_eq!(
                raw.admit(&ids, limits)
                    .expect_err("first actual cause")
                    .refusal(),
                cause
            );
        }
        let old = retained();
        let old = old
            .admit(&known, WalkNotchIdentityLimits::new(2, 2, 8))
            .expect("retained");
        assert_eq!(
            raw(vec![intent(2, SingleOccurrence, NewRequested)])
                .admit(&old, limits)
                .expect_err("retained requires explicit retained policy")
                .refusal(),
            R::IdentityKind {
                index: 0,
                role: LocalTag::new(2)
            }
        );
    });
}
#[test]
fn repeated_original_pairs_require_explicit_coalescing_even_when_every_field_is_identical() {
    with_known(2, |known| {
        let ids = proposals(&[2, 9]);
        let ids = ids
            .admit(&known, WalkNotchIdentityLimits::new(2, 4, 10))
            .expect("ids");
        let limits = WalkNotchFieldLimits::new(2, 4, 2, 6);
        let single = raw(vec![
            intent(2, SingleOccurrence, NewRequested),
            intent(9, CoalesceIdentical, NewRequested),
        ]);
        assert_eq!(
            single
                .admit(&ids, limits)
                .expect_err("single does not silently coalesce")
                .refusal(),
            WalkNotchFieldRefusal::RepeatedOccurrence {
                first_pair: 0,
                first_side: SeamSideId::A,
                pair: 1,
                side: SeamSideId::A,
                role: LocalTag::new(2)
            }
        );
        let raw = raw(vec![
            intent(9, CoalesceIdentical, NewRequested),
            intent(2, CoalesceIdentical, NewRequested),
        ]);
        let fields = raw
            .admit(&ids, limits)
            .expect("two deliberate groups,four original requests");
        assert_eq!(fields.group_count(), 2);
        assert!(fields.groups().all(|g| g.occurrence_count() == 2));
        assert_eq!(
            fields
                .identities()
                .known()
                .existing()
                .assignments()
                .request_count(),
            4
        );
        assert_eq!(known.existing().notches().len(), 2);
    });
}
#[test]
fn distinct_physical_copies_of_one_actual_pattern_can_explicitly_share_one_semantic_identity() {
    with_requests(true, &[Request::same()], None, |known| {
        let ids = proposals(&[2]);
        let ids = ids
            .admit(&known, WalkNotchIdentityLimits::new(1, 2, 5))
            .expect("one identity for two actual copies");
        let raw = raw(vec![intent(2, CoalesceIdentical, NewRequested)]);
        let fields = raw
            .admit(&ids, WalkNotchFieldLimits::new(1, 2, 1, 3))
            .expect("exact common fields");
        let group = fields.group(LocalTag::new(2)).expect("one complete group");
        assert_eq!(fields.group_count(), 1);
        assert_eq!(group.occurrence_count(), 2);
        assert_eq!(*group.identity().id(), id(500));
        assert_eq!(group.piece().id(), id(100));
        assert_eq!(group.first().assignment().side(), SeamSideId::A);
        let source = known.existing().assignments();
        let a = source.assignment(0, SeamSideId::A).expect("A");
        let b = source.assignment(0, SeamSideId::B).expect("B");
        assert_ne!(a.copy().id(), b.copy().id());
        assert!(std::ptr::eq(a.piece(), b.piece()));
        assert!(std::ptr::eq(group.piece(), a.piece()));
        assert!(std::ptr::eq(group.anchor(), a.requested().anchor()));
        assert!(std::ptr::eq(
            group.representation(),
            a.requested().representation()
        ));
    });
}
#[test]
fn same_role_never_hides_different_pattern_owners_anchors_or_any_of_six_profile_bindings() {
    let mut cases = vec![(false, Request::same(), WalkNotchField::Piece)];
    let mut anchor = Request::same();
    anchor.anchors[1] = t(2, 5);
    cases.push((true, anchor, WalkNotchField::Anchor));
    for n in 0..6 {
        let mut r = Request::same();
        let q = profiles(3000);
        match n {
            0 => r.profiles[1].style = q.style,
            1 => r.profiles[1].sample.depth = q.sample.depth,
            2 => r.profiles[1].sample.width = q.sample.width,
            3 => r.profiles[1].production.depth = q.production.depth,
            4 => r.profiles[1].production.width = q.production.width,
            _ => r.profiles[1].encoding = q.encoding,
        };
        cases.push((true, r, WalkNotchField::Profiles));
    }
    for (shared, r, field) in cases {
        with_requests(shared, &[r], None, |known| {
            let ids = proposals(&[2]);
            let ids = ids
                .admit(&known, WalkNotchIdentityLimits::new(1, 2, 5))
                .expect("identity is only a proposal");
            let raw = raw(vec![intent(2, CoalesceIdentical, NewRequested)]);
            assert_eq!(
                raw.admit(&ids, WalkNotchFieldLimits::new(1, 2, 1, 3))
                    .expect_err("no arbitrary first-field choice")
                    .refusal(),
                WalkNotchFieldRefusal::GroupFields {
                    first_pair: 0,
                    first_side: SeamSideId::A,
                    pair: 0,
                    side: SeamSideId::B,
                    role: LocalTag::new(2),
                    field
                }
            );
        });
    }
}
#[test]
fn retained_revision_is_explicit_lends_requested_fields_and_never_reassigns_a_prior_pattern_owner()
{
    for field in [WalkNotchField::Anchor, WalkNotchField::Profiles] {
        let mut r = Request::same();
        r.roles = [2, 9];
        with_requests(false, &[r], Some(field), |known| {
            let prior = known.existing().notches().to_vec();
            let ids = retained();
            let ids = ids
                .admit(&known, WalkNotchIdentityLimits::new(2, 2, 8))
                .expect("actual retained objects");
            let limits = WalkNotchFieldLimits::new(2, 2, 2, 6);
            let unchanged = raw(vec![
                intent(2, SingleOccurrence, RetainUnchanged),
                intent(9, SingleOccurrence, RetainUnchanged),
            ]);
            assert_eq!(
                unchanged
                    .admit(&ids, limits)
                    .expect_err("unchanged means exact prior fields")
                    .refusal(),
                WalkNotchFieldRefusal::RetainedFields {
                    pair: 0,
                    side: SeamSideId::A,
                    role: LocalTag::new(2),
                    field
                }
            );
            let revised = raw(vec![
                intent(2, SingleOccurrence, ReviseRetained),
                intent(9, SingleOccurrence, RetainUnchanged),
            ]);
            let fields = revised
                .admit(&ids, limits)
                .expect("explicit revision proposal only");
            let group = fields.group(LocalTag::new(2)).expect("A result fields");
            assert!(
                matches!(group.identity().source(),WalkNotchIdentitySource::Existing(old) if std::ptr::eq(old,known.existing().notch(id(700)).expect("actual old object")))
            );
            assert!(std::ptr::eq(
                group.anchor(),
                group.first().requested().anchor()
            ));
            assert!(std::ptr::eq(
                group.representation(),
                group.first().requested().representation()
            ));
            assert_eq!(known.existing().notches(), prior);
        });
    }
    with_known(1, |known| {
        let ids = WalkNotchIdentityManifest::new(
            id(10),
            vec![
                WalkNotchIdentityIntent::Retain {
                    role: LocalTag::new(2),
                    notch: id(701),
                },
                WalkNotchIdentityIntent::Retain {
                    role: LocalTag::new(9),
                    notch: id(700),
                },
            ],
        );
        let ids = ids
            .admit(&known, WalkNotchIdentityLimits::new(2, 2, 8))
            .expect("identity choices do not accept ownership");
        let raw = raw(vec![
            intent(2, SingleOccurrence, ReviseRetained),
            intent(9, SingleOccurrence, ReviseRetained),
        ]);
        assert_eq!(
            raw.admit(&ids, WalkNotchFieldLimits::new(2, 2, 2, 6))
                .expect_err("explicit revision still cannot change pattern owner")
                .refusal(),
            WalkNotchFieldRefusal::RetainedFields {
                pair: 0,
                side: SeamSideId::A,
                role: LocalTag::new(2),
                field: WalkNotchField::Piece
            }
        );
    });
}
