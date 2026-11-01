//! Complete guarded original physical-copy actions,not accepted no-fork pairing or effects.
#![allow(clippy::expect_used)]
#[allow(dead_code)]
#[path = "support/join_copies.rs"]
mod common;
use common::{id, with_results};
use sc_core::{construction::*, ontology::*};
fn row(
    results: &DeclaredJoinResultCopies<'_>,
    copy: u128,
    intent: JoinCopySourceIntent,
) -> JoinCopySourceAssignment {
    JoinCopySourceAssignment::new(
        *results
            .source_copy(id(copy))
            .expect("actual original copy")
            .definition(),
        intent,
    )
}
fn join() -> JoinCopySourceIntent {
    JoinCopySourceIntent::JoinTo {
        result_copy: id(6000),
    }
}
fn rows(results: &DeclaredJoinResultCopies<'_>) -> Vec<JoinCopySourceAssignment> {
    vec![
        row(results, 202, JoinCopySourceIntent::RetainOriginal),
        row(results, 200, join()),
        row(results, 201, join()),
    ]
}
#[test]
fn complete_reordered_source_actions_preserve_unrelated_copy_and_lend_actual_fields() {
    with_results(|results| {
        let raw = JoinCopySourceManifest::new(id(1000), rows(&results));
        let snapshot = raw.clone();
        let limits = JoinCopySourceLimits::new(3, 3, 6);
        let declared = raw
            .admit(&results, limits)
            .expect("one guarded action for every actual original copy");
        assert!(std::ptr::eq(declared.manifest(), &raw));
        assert!(std::ptr::eq(declared.results(), &results));
        assert_eq!(declared.limits(), limits);
        assert_eq!(raw.operation(), id(1000));
        assert_eq!(
            (
                limits.copies(),
                limits.assignments(),
                limits.workspace_entries()
            ),
            (3, 3, 6)
        );
        assert_eq!(
            (declared.copy_count(), declared.workspace_entries()),
            (3, 6)
        );
        assert_eq!(
            declared
                .assignments()
                .map(|a| a.source().id())
                .collect::<Vec<_>>(),
            [id(202), id(200), id(201)]
        );
        assert_eq!(
            declared
                .assignments()
                .rev()
                .map(|a| a.index())
                .collect::<Vec<_>>(),
            [2, 1, 0]
        );
        for a in declared.assignments() {
            assert!(std::ptr::eq(a.context(), &declared));
            let original = raw.assignments().get(a.index()).expect("actual raw row");
            assert!(std::ptr::eq(a.assignment(), original));
            assert!(std::ptr::eq(a.intent(), original.intent()));
            assert_eq!(*a.source_definition(), *original.expected());
            assert!(!std::ptr::eq(a.source_definition(), original.expected()));
            assert!(std::ptr::eq(a.source_definition(), a.source().definition()));
            assert!(std::ptr::eq(
                a.source(),
                results
                    .source_copy(original.expected().id)
                    .expect("actual immutable source copy")
            ));
            assert!(std::ptr::eq(
                declared
                    .assignment(a.source().id())
                    .expect("actual indexed source action")
                    .assignment(),
                original
            ));
            if a.source().id() == id(202) {
                assert!(a.side().is_none());
                assert!(a.result().is_none());
                assert!(original.result_copy().is_none());
                assert!(matches!(a.intent(), JoinCopySourceIntent::RetainOriginal));
            } else {
                assert_eq!(
                    a.side(),
                    Some(if a.source().id() == id(200) {
                        SeamSideId::A
                    } else {
                        SeamSideId::B
                    })
                );
                let target = a.result().expect("actual result request");
                assert!(std::ptr::eq(
                    target.definition(),
                    results
                        .copy(id(6000))
                        .expect("actual result id")
                        .definition()
                ));
                assert_eq!(original.result_copy(), Some(&target.id()));
            }
        }
        assert!(declared.assignment(id(6000)).is_none());
        assert_eq!(raw, snapshot);
    });
}
#[test]
fn complete_original_and_raw_resources_precede_operation_counts_and_invalid_first_fields() {
    with_results(|results| {
        let mut values = rows(&results);
        *values.first_mut().expect("first row") = JoinCopySourceAssignment::new(
            CutCopyDefinition {
                id: id(999),
                piece: id(999),
                orientation: CopyOrientation::Reflected,
            },
            join(),
        );
        let raw = JoinCopySourceManifest::new(id(9999), values);
        use JoinCopySourceRefusal as R;
        for (limits, cause) in [
            (
                JoinCopySourceLimits::new(2, 0, 0),
                R::Copies {
                    required: 3,
                    allowed: 2,
                },
            ),
            (
                JoinCopySourceLimits::new(3, 2, 0),
                R::Assignments {
                    required: 3,
                    allowed: 2,
                },
            ),
            (
                JoinCopySourceLimits::new(3, 3, 5),
                R::Workspace {
                    required: 6,
                    allowed: 5,
                },
            ),
            (JoinCopySourceLimits::new(3, 3, 6), R::Operation),
        ] {
            let error = raw
                .admit(&results, limits)
                .expect_err("whole resources before owner/count/fields");
            assert_eq!(error.refusal(), cause);
            assert_eq!(error.limits(), limits);
            assert!(std::ptr::eq(error.manifest(), &raw));
            assert!(std::ptr::eq(error.results(), &results));
            assert!(error.refused_assignment().is_none());
            assert!(error.refused_copy().is_none());
            assert!(error.refused_result().is_none());
        }
        for values in [vec![], rows(&results).into_iter().take(2).collect()] {
            let actual = values.len();
            let raw = JoinCopySourceManifest::new(id(1000), values);
            let error = raw
                .admit(&results, JoinCopySourceLimits::new(3, 3, 6))
                .expect_err("complete original copy actions mandatory");
            assert_eq!(
                error.refusal(),
                R::Count {
                    expected: 3,
                    actual
                }
            );
            assert!(error.refused_assignment().is_none());
        }
    });
}
#[test]
fn first_original_duplicate_missing_whole_guard_foreign_and_unknown_target_causes_keep_actual_loans(
) {
    with_results(|results| {
        use JoinCopySourceRefusal as R;
        let base = rows(&results);
        let unrelated = *base.first().expect("actual unrelated row");
        let a = *base.get(1).expect("actual A row");
        let b = *base.get(2).expect("actual B row");
        for (values, cause, index) in [
            (
                vec![unrelated, a, a],
                R::DuplicateCopy {
                    first: 1,
                    second: 2,
                },
                2,
            ),
            (
                vec![
                    unrelated,
                    a,
                    JoinCopySourceAssignment::new(
                        CutCopyDefinition {
                            id: id(999),
                            ..*b.expected()
                        },
                        join(),
                    ),
                ],
                R::MissingCopy { index: 2 },
                2,
            ),
            (
                vec![
                    unrelated,
                    JoinCopySourceAssignment::new(
                        CutCopyDefinition {
                            piece: id(101),
                            ..*a.expected()
                        },
                        join(),
                    ),
                    b,
                ],
                R::SourceField { index: 1 },
                1,
            ),
            (
                vec![
                    unrelated,
                    a,
                    JoinCopySourceAssignment::new(
                        CutCopyDefinition {
                            orientation: CopyOrientation::Reflected,
                            ..*b.expected()
                        },
                        join(),
                    ),
                ],
                R::SourceField { index: 2 },
                2,
            ),
            (
                vec![
                    JoinCopySourceAssignment::new(*unrelated.expected(), join()),
                    a,
                    b,
                ],
                R::ForeignPiece { index: 0 },
                0,
            ),
            (
                vec![
                    unrelated,
                    JoinCopySourceAssignment::new(
                        *a.expected(),
                        JoinCopySourceIntent::JoinTo {
                            result_copy: id(9999),
                        },
                    ),
                    b,
                ],
                R::UnknownResult { index: 1 },
                1,
            ),
        ] {
            let raw = JoinCopySourceManifest::new(id(1000), values);
            let snapshot = raw.clone();
            let error = raw
                .admit(&results, JoinCopySourceLimits::new(3, 3, 6))
                .expect_err("actual whole original fields/source role/result request required");
            assert_eq!(error.refusal(), cause);
            let original = raw
                .assignments()
                .get(index)
                .expect("raw refused original row");
            assert!(std::ptr::eq(
                error.refused_assignment().expect("actual raw refusal"),
                original
            ));
            if matches!(cause, R::MissingCopy { .. }) {
                assert!(error.refused_copy().is_none());
            } else {
                assert!(std::ptr::eq(
                    error.refused_copy().expect("actual original copy"),
                    results
                        .source_copy(original.expected().id)
                        .expect("actual whole original plan member")
                ));
            }
            if matches!(cause, R::UnknownResult { .. }) {
                assert!(std::ptr::eq(
                    error.refused_result().expect("actual authored target id"),
                    original.result_copy().expect("raw JoinTo field")
                ));
            } else {
                assert!(error.refused_result().is_none());
            }
            assert_eq!(raw, snapshot);
        }
    });
}
#[test]
fn complete_retention_request_is_not_selected_consumption_or_pairing_certificate() {
    with_results(|results| {
        let values = [201, 200, 202]
            .into_iter()
            .map(|c| row(&results, c, JoinCopySourceIntent::RetainOriginal))
            .collect();
        let raw = JoinCopySourceManifest::new(id(1000), values);
        let declared = raw
            .admit(&results, JoinCopySourceLimits::new(3, 3, 6))
            .expect("complete original preservation requests only");
        assert_eq!(declared.copy_count(), 3);
        assert!(declared.assignments().all(|a| a.result().is_none()));
        assert_eq!(
            declared
                .assignment(id(200))
                .expect("actual selected A still present")
                .side(),
            Some(SeamSideId::A)
        );
        assert_eq!(
            declared
                .assignment(id(201))
                .expect("actual selected B still present")
                .side(),
            Some(SeamSideId::B)
        );
        assert_eq!(declared.results().copy_count(), 1); // Exact selected A/B pairing must refuse this later.
    });
}
