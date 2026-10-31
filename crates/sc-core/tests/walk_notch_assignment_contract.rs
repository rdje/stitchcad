//! Complete original occurrence attribution;repeated roles propose grouping without semantic merging.
#![allow(clippy::expect_used)]
#[path = "support/walk_roles.rs"]
mod common;
use common::{copies::input::id, with_roles};
use sc_core::{construction::*, ontology::*};
fn a(pair: usize, side: SeamSideId, tag: u32) -> WalkNotchRoleAssignment {
    WalkNotchRoleAssignment::new(pair, side, LocalTag::new(tag))
}
fn raw(operation: u128, entries: Vec<WalkNotchRoleAssignment>) -> WalkNotchAssignmentManifest {
    WalkNotchAssignmentManifest::new(id(operation), entries)
}
#[test]
fn complete_shuffled_manifest_and_source_order_views_lend_actual_fields_roles_copies_and_parents() {
    with_roles(1, &[2, 9], |roles| {
        let raw = raw(10, vec![a(0, SeamSideId::B, 9), a(0, SeamSideId::A, 2)]);
        let snapshot = raw.clone();
        let limits = WalkNotchAssignmentLimits::new(1, 2, 4);
        let admitted = raw.admit(&roles, limits).expect("all original occurrences");
        assert!(std::ptr::eq(admitted.manifest(), &raw));
        assert!(std::ptr::eq(admitted.roles(), &roles));
        assert_eq!(admitted.limits(), limits);
        assert_eq!(
            (
                limits.pairs(),
                limits.assignments(),
                limits.workspace_entries(),
                admitted.workspace_entries(),
                admitted.request_count(),
                admitted.assignment_count()
            ),
            (1, 2, 4, 4, 2, 2)
        );
        assert_eq!(admitted.manifest().operation(), id(10));
        let pair = roles
            .copies()
            .source()
            .admitted()
            .edit()
            .pairs()
            .first()
            .expect("actual original pair");
        for (item, index) in admitted.assignments().zip(0..) {
            assert!(std::ptr::eq(item.owner(), &admitted));
            assert_eq!(item.index(), index);
            assert!(std::ptr::eq(
                item.assignment(),
                raw.assignments()
                    .get(index)
                    .expect("original manifest field")
            ));
            assert!(std::ptr::eq(item.pair(), pair));
            assert!(std::ptr::eq(
                item.requested(),
                pair.mark(item.assignment().side())
            ));
            assert!(std::ptr::eq(item.role().catalog(), roles.catalog()));
            assert_eq!(*item.role().tag(), *item.assignment().role());
            assert!(std::ptr::eq(
                item.copy(),
                roles.copies().copy(item.assignment().side())
            ));
            assert!(std::ptr::eq(
                item.piece(),
                roles.copies().piece(item.assignment().side())
            ));
        }
        assert_eq!(
            admitted
                .assignments()
                .map(|item| item.assignment().side())
                .collect::<Vec<_>>(),
            vec![SeamSideId::B, SeamSideId::A]
        );
        assert_eq!(
            admitted
                .assignments()
                .rev()
                .map(|item| item.index())
                .collect::<Vec<_>>(),
            vec![1, 0]
        );
        assert_eq!(
            admitted
                .requests()
                .map(|item| item.assignment().side())
                .collect::<Vec<_>>(),
            vec![SeamSideId::A, SeamSideId::B]
        );
        assert_eq!(
            admitted
                .requests()
                .rev()
                .map(|item| item.index())
                .collect::<Vec<_>>(),
            vec![0, 1]
        );
        assert_eq!(
            admitted
                .assignment(0, SeamSideId::A)
                .expect("actual occurrence")
                .index(),
            1
        );
        assert!(admitted.assignment(99, SeamSideId::A).is_none());
        assert_eq!(raw, snapshot);
    });
}
#[test]
fn whole_pair_manifest_workspace_quotas_precede_wrong_operation_and_invalid_original_entry() {
    use WalkNotchAssignmentRefusal as R;
    with_roles(1, &[2, 9], |roles| {
        let raw = raw(99, vec![a(9, SeamSideId::A, 7), a(0, SeamSideId::B, 9)]);
        let snapshot = raw.clone();
        for (limits, expected) in [
            (
                WalkNotchAssignmentLimits::new(0, 0, 0),
                R::Pairs {
                    required: 1,
                    allowed: 0,
                },
            ),
            (
                WalkNotchAssignmentLimits::new(1, 1, 0),
                R::Assignments {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                WalkNotchAssignmentLimits::new(1, 2, 3),
                R::Workspace {
                    required: 4,
                    allowed: 3,
                },
            ),
            (WalkNotchAssignmentLimits::new(1, 2, 4), R::Operation),
        ] {
            let error = raw
                .admit(&roles, limits)
                .expect_err("complete resource then operation");
            assert!(std::ptr::eq(error.manifest(), &raw));
            assert!(std::ptr::eq(error.roles(), &roles));
            assert_eq!(error.limits(), limits);
            assert_eq!(error.refusal(), expected);
            assert!(error.to_string().starts_with("walk notch assignments:"));
            assert!(format!("{error:?}").contains("WalkNotchAssignmentError"));
            assert_eq!(raw, snapshot);
        }
    });
}
#[test]
fn actual_pair_bounds_and_duplicate_occurrence_precede_second_original_role_cause() {
    use WalkNotchAssignmentRefusal as R;
    with_roles(1, &[2, 9], |roles| {
        let limits = WalkNotchAssignmentLimits::new(1, 2, 4);
        let missing = raw(10, vec![a(1, SeamSideId::A, 88)]);
        assert_eq!(
            missing
                .admit(&roles, limits)
                .expect_err("original pair before role")
                .refusal(),
            R::Pair {
                index: 0,
                pair: 1,
                available: 1
            }
        );
        let duplicate = raw(10, vec![a(0, SeamSideId::A, 2), a(0, SeamSideId::A, 7)]);
        assert_eq!(
            duplicate
                .admit(&roles, limits)
                .expect_err("same occurrence before inactive second role")
                .refusal(),
            R::DuplicateOccurrence {
                first: 0,
                second: 1,
                pair: 0,
                side: SeamSideId::A
            }
        );
    });
}
#[test]
fn original_role_refusals_precede_complete_source_pair_side_coverage_and_empty_never_manufactures_outputs(
) {
    use WalkNotchAssignmentRefusal as R;
    with_roles(1, &[2, 9], |roles| {
        let limits = WalkNotchAssignmentLimits::new(1, 2, 4);
        for (tag, expected) in [
            (
                88,
                R::UndefinedRole {
                    index: 0,
                    role: LocalTag::new(88),
                },
            ),
            (
                7,
                R::InactiveRole {
                    index: 0,
                    role: LocalTag::new(7),
                },
            ),
        ] {
            let raw = raw(10, vec![a(0, SeamSideId::A, tag)]);
            assert_eq!(
                raw.admit(&roles, limits)
                    .expect_err("original role before source coverage")
                    .refusal(),
                expected
            );
        }
        let missing = raw(10, vec![a(0, SeamSideId::A, 2)]);
        assert_eq!(
            missing
                .admit(&roles, limits)
                .expect_err("complete actual source B")
                .refusal(),
            R::MissingOccurrence {
                pair: 0,
                side: SeamSideId::B
            }
        );
        let empty = raw(10, vec![]);
        assert_eq!(
            empty
                .admit(&roles, WalkNotchAssignmentLimits::new(1, 0, 0))
                .expect_err("no fabricated assignment")
                .refusal(),
            R::MissingOccurrence {
                pair: 0,
                side: SeamSideId::A
            }
        );
    });
    with_roles(2, &[2, 9], |roles| {
        let missing = raw(
            10,
            vec![
                a(1, SeamSideId::B, 9),
                a(0, SeamSideId::A, 2),
                a(1, SeamSideId::A, 2),
            ],
        );
        assert_eq!(
            missing
                .admit(&roles, WalkNotchAssignmentLimits::new(2, 3, 6))
                .expect_err("first original input pair/A/B gap")
                .refusal(),
            R::MissingOccurrence {
                pair: 0,
                side: SeamSideId::B
            }
        );
    });
}
#[test]
fn repeated_tags_retain_all_occurrences_as_grouping_proposals_while_unused_active_metadata_stays_legal(
) {
    with_roles(2, &[7, 2, 9], |roles| {
        let raw = raw(
            10,
            vec![
                a(1, SeamSideId::B, 2),
                a(0, SeamSideId::B, 2),
                a(1, SeamSideId::A, 2),
                a(0, SeamSideId::A, 2),
            ],
        );
        let admitted = raw
            .admit(&roles, WalkNotchAssignmentLimits::new(2, 4, 8))
            .expect("complete attribution only,not shared-mark admission");
        assert_eq!(
            (
                admitted.request_count(),
                admitted.assignment_count(),
                admitted.workspace_entries()
            ),
            (4, 4, 8)
        );
        assert!(admitted
            .assignments()
            .all(|item| *item.role().tag() == LocalTag::new(2)));
        let source_pairs = roles.copies().source().admitted().edit().pairs();
        for item in admitted.requests() {
            let pair = source_pairs
                .get(item.assignment().pair())
                .expect("actual original repeated pair field");
            assert!(std::ptr::eq(item.pair(), pair));
            assert!(std::ptr::eq(
                item.requested(),
                pair.mark(item.assignment().side())
            ));
        }
        assert_eq!(
            admitted
                .requests()
                .map(|item| (
                    item.assignment().pair(),
                    item.assignment().side(),
                    item.piece().id()
                ))
                .collect::<Vec<_>>(),
            vec![
                (0, SeamSideId::A, id(100)),
                (0, SeamSideId::B, id(101)),
                (1, SeamSideId::A, id(100)),
                (1, SeamSideId::B, id(101))
            ]
        );
        assert_eq!(admitted.roles().catalog().active_roles().len(), 3);
        assert!(admitted
            .assignments()
            .all(|item| item.piece().id() == id(100) || item.piece().id() == id(101)));
    });
}
