//! Complete guarded original fold actions and exact A/B/edge attribution,without evidence truth.
#![allow(clippy::expect_used)]
#[allow(dead_code)]
#[path = "support/join_fold.rs"]
mod common;
use common::{id, with_alternative_construction, with_construction};
use sc_core::{construction::*, ontology::*};
fn rows(
    fold: &RequiredJoinFold<'_>,
    intent: impl Fn(SeamSideId) -> JoinFoldSourceIntent,
) -> Vec<JoinFoldSourceAssignment> {
    fold.construction()
        .targets()
        .coverage()
        .groups()
        .sources()
        .uses()
        .connectivity()
        .contours()
        .directions()
        .coverage()
        .edges()
        .identity()
        .roles()
        .banks()
        .sources()
        .occurrences()
        .filter_map(|f| match f.location() {
            JoinSourceOccurrenceLocation::Fold(index) => Some(JoinFoldSourceAssignment::new(
                f.side(),
                index,
                *f.field().edge(),
                intent(f.side()),
            )),
            _ => None,
        })
        .collect()
}
fn omit(_: SeamSideId) -> JoinFoldSourceIntent {
    JoinFoldSourceIntent::Omit {
        evidence_source: id(0),
    }
}
fn with_fold(intent: JoinFoldIntent, run: impl FnOnce(RequiredJoinFold<'_>)) {
    with_construction(true, |construction| {
        let request = JoinFoldRequirement::new(id(1000), intent, id(0));
        let fold = construction
            .require_fold(&request, JoinFoldLimits::new(10, 10, 4))
            .expect("explicit actual desired result fold");
        run(fold);
    });
}
#[test]
fn complete_reordered_actions_lend_actual_source_fields_and_independent_omission_requests() {
    with_fold(
        JoinFoldIntent::CutOnFold {
            tag: LocalTag::new(9),
        },
        |fold| {
            let mut values = rows(&fold, |s| {
                if s == SeamSideId::A {
                    JoinFoldSourceIntent::Retain
                } else {
                    omit(s)
                }
            });
            values.reverse();
            let raw = JoinFoldSourceManifest::new(id(1000), values);
            let snapshot = raw.clone();
            let limits = JoinFoldSourceLimits::new(10, 2, 4);
            let declared = raw
                .admit(&fold, limits)
                .expect("every original fold action guarded");
            assert!(std::ptr::eq(declared.manifest(), &raw));
            assert!(std::ptr::eq(declared.fold(), &fold));
            assert_eq!(declared.limits(), limits);
            assert_eq!(
                (
                    limits.sources(),
                    limits.assignments(),
                    limits.workspace_entries()
                ),
                (10, 2, 4)
            );
            assert_eq!(
                (declared.source_count(), declared.workspace_entries()),
                (2, 4)
            );
            assert_eq!(raw.operation(), id(1000));
            assert_eq!(
                declared
                    .assignments()
                    .map(|a| a.assignment().side())
                    .collect::<Vec<_>>(),
                [SeamSideId::B, SeamSideId::A]
            );
            assert_eq!(
                declared
                    .assignments()
                    .rev()
                    .map(|a| a.index())
                    .collect::<Vec<_>>(),
                [1, 0]
            );
            for action in declared.assignments() {
                assert!(std::ptr::eq(action.context(), &declared));
                let row = raw
                    .assignments()
                    .get(action.index())
                    .expect("actual raw row");
                assert!(std::ptr::eq(action.assignment(), row));
                assert!(std::ptr::eq(action.intent(), row.intent()));
                assert_eq!(
                    (row.index(), action.source().location()),
                    (0, JoinSourceOccurrenceLocation::Fold(0))
                );
                assert!(std::ptr::eq(
                    action.source_field(),
                    action.source().field().edge()
                ));
                assert_eq!(*action.source_field(), *row.expected());
                assert!(!std::ptr::eq(action.source_field(), row.expected()));
                let actual = declared
                    .assignment(row.side(), row.index())
                    .expect("actual indexed source action");
                assert!(std::ptr::eq(actual.assignment(), row));
                if let Some(evidence) = action.evidence_source() {
                    assert!(std::ptr::eq(
                        evidence,
                        row.evidence_source()
                            .expect("actual authored evidence field")
                    ));
                    assert_eq!(*evidence, id(0));
                }
            }
            assert!(declared.assignment(SeamSideId::A, 99).is_none());
            assert_eq!(raw, snapshot);
        },
    );
    with_fold(JoinFoldIntent::NoFold, |fold| {
        let raw = JoinFoldSourceManifest::new(id(1000), rows(&fold, omit));
        let declared = raw
            .admit(&fold, JoinFoldSourceLimits::new(10, 2, 4))
            .expect("two explicit independent omission requests");
        assert_eq!(
            declared
                .assignments()
                .filter_map(|a| a.evidence_source())
                .count(),
            2
        );
        assert!(!declared.fold().cut_on_fold()); // Neither deletion nor evidence truth is accepted.
    });
}
#[test]
fn complete_resources_and_owner_counts_precede_first_malformed_source_selector() {
    with_fold(JoinFoldIntent::NoFold, |fold| {
        let mut values = rows(&fold, omit);
        let original = values.first().expect("actual row");
        *values.first_mut().expect("first row") = JoinFoldSourceAssignment::new(
            SeamSideId::A,
            99,
            *original.expected(),
            JoinFoldSourceIntent::Retain,
        );
        let raw = JoinFoldSourceManifest::new(id(9999), values);
        use JoinFoldSourceRefusal as R;
        for (limits, cause) in [
            (
                JoinFoldSourceLimits::new(9, 0, 0),
                R::Sources {
                    required: 10,
                    allowed: 9,
                },
            ),
            (
                JoinFoldSourceLimits::new(10, 1, 0),
                R::Assignments {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                JoinFoldSourceLimits::new(10, 2, 3),
                R::Workspace {
                    required: 4,
                    allowed: 3,
                },
            ),
            (JoinFoldSourceLimits::new(10, 2, 4), R::Operation),
        ] {
            let error = raw
                .admit(&fold, limits)
                .expect_err("whole resources before source/action fields");
            assert_eq!(error.refusal(), cause);
            assert_eq!(error.limits(), limits);
            assert!(std::ptr::eq(error.manifest(), &raw));
            assert!(std::ptr::eq(error.fold(), &fold));
            assert!(error.refused_assignment().is_none());
            assert!(error.refused_source().is_none());
        }
        let raw =
            JoinFoldSourceManifest::new(id(1000), rows(&fold, omit).into_iter().take(1).collect());
        let error = raw
            .admit(&fold, JoinFoldSourceLimits::new(10, 2, 4))
            .expect_err("whole original fold actions required");
        assert_eq!(
            error.refusal(),
            R::Count {
                expected: 2,
                actual: 1
            }
        );
        assert!(error.refused_assignment().is_none());
    });
}
#[test]
fn first_original_duplicate_missing_guard_and_result_frame_causes_preserve_actual_fields() {
    for intent in [
        JoinFoldIntent::NoFold,
        JoinFoldIntent::CutOnFold {
            tag: LocalTag::new(9),
        },
    ] {
        with_fold(intent, |fold| {
            use JoinFoldSourceRefusal as R;
            let base = rows(&fold, omit);
            let a = *base.first().expect("actual A");
            let b = *base.get(1).expect("actual B");
            let mut cases = vec![
                (
                    vec![a, a],
                    R::DuplicateSource {
                        first: 0,
                        second: 1,
                    },
                    1,
                ),
                (
                    vec![
                        a,
                        JoinFoldSourceAssignment::new(b.side(), 99, *b.expected(), omit(b.side())),
                    ],
                    R::MissingSource { index: 1 },
                    1,
                ),
                (
                    vec![
                        JoinFoldSourceAssignment::new(a.side(), 0, *b.expected(), omit(a.side())),
                        b,
                    ],
                    R::SourceField { index: 0 },
                    0,
                ),
            ];
            if matches!(intent, JoinFoldIntent::NoFold) {
                cases.push((
                    vec![
                        JoinFoldSourceAssignment::new(
                            a.side(),
                            0,
                            *a.expected(),
                            JoinFoldSourceIntent::Retain,
                        ),
                        b,
                    ],
                    R::ResultNoFold { index: 0 },
                    0,
                ));
            } else {
                cases.push((
                    vec![
                        a,
                        JoinFoldSourceAssignment::new(
                            b.side(),
                            0,
                            *b.expected(),
                            JoinFoldSourceIntent::Retain,
                        ),
                    ],
                    R::SourceSide { index: 1 },
                    1,
                ));
            }
            for (values, cause, index) in cases {
                let raw = JoinFoldSourceManifest::new(id(1000), values);
                let snapshot = raw.clone();
                let error = raw
                    .admit(&fold, JoinFoldSourceLimits::new(10, 2, 4))
                    .expect_err("actual guarded source frame required");
                assert_eq!(error.refusal(), cause);
                let row = error.refused_assignment().expect("actual refused raw row");
                assert!(std::ptr::eq(
                    row,
                    raw.assignments().get(index).expect("raw original row")
                ));
                if matches!(cause, R::MissingSource { .. }) {
                    assert!(error.refused_source().is_none());
                } else {
                    let source = error.refused_source().expect("actual original fold field");
                    assert_eq!(
                        (source.side(), source.location()),
                        (row.side(), JoinSourceOccurrenceLocation::Fold(row.index()))
                    );
                    assert!(matches!(source.field(), JoinSourceOccurrenceField::Fold(_)));
                    let actual_field = source
                        .piece()
                        .definition()
                        .fold_edges
                        .get(row.index())
                        .expect("actual original Piece field");
                    assert!(std::ptr::eq(source.field().edge(), actual_field));
                }
                assert_eq!(raw, snapshot);
            }
        });
    }
    for shared in [false, true] {
        with_alternative_construction(shared, |construction| {
            // Tag9 is a real outer bank fragment,but the original A fold is the other outer edge.
            let request = JoinFoldRequirement::new(
                id(1000),
                JoinFoldIntent::CutOnFold {
                    tag: LocalTag::new(9),
                },
                id(0),
            );
            let fold = construction
                .require_fold(&request, JoinFoldLimits::new(10, 10, 4))
                .expect("actual outer bank use");
            let raw = JoinFoldSourceManifest::new(
                id(1000),
                rows(&fold, |s| {
                    if s == SeamSideId::A {
                        JoinFoldSourceIntent::Retain
                    } else {
                        omit(s)
                    }
                }),
            );
            let error = raw
                .admit(&fold, JoinFoldSourceLimits::new(10, 2, 4))
                .expect_err("outer membership alone cannot replace source fold attribution");
            assert_eq!(
                error.refusal(),
                JoinFoldSourceRefusal::SourceEdge { index: 0 }
            );
            // Tag2 is the actual A fold. Even byte-equal B fold geometry remains another source frame.
            let request = JoinFoldRequirement::new(
                id(1000),
                JoinFoldIntent::CutOnFold {
                    tag: LocalTag::new(2),
                },
                id(0),
            );
            let fold = construction
                .require_fold(&request, JoinFoldLimits::new(10, 10, 4))
                .expect("actual outer original fold output");
            let values = rows(&fold, |_| JoinFoldSourceIntent::Retain);
            if shared {
                assert_eq!(
                    values.first().expect("A").expected(),
                    values.get(1).expect("B").expected()
                );
            }
            let raw = JoinFoldSourceManifest::new(id(1000), values);
            let error = raw
                .admit(&fold, JoinFoldSourceLimits::new(10, 2, 4))
                .expect_err("A/B source frames remain distinct");
            assert_eq!(
                error.refusal(),
                JoinFoldSourceRefusal::SourceSide { index: 1 }
            );
        });
    }
}
#[test]
fn original_no_fold_scope_keeps_explicit_desired_result_and_correspondence_requests() {
    with_construction(false, |construction| {
        for intent in [
            JoinFoldIntent::NoFold,
            JoinFoldIntent::CutOnFold {
                tag: LocalTag::new(9),
            },
        ] {
            let request = JoinFoldRequirement::new(id(1000), intent, id(0));
            let fold = construction
                .require_fold(&request, JoinFoldLimits::new(2, 2, 2))
                .expect("explicit desired result state");
            let raw = JoinFoldSourceManifest::new(id(1000), vec![]);
            let declared = raw
                .admit(&fold, JoinFoldSourceLimits::new(2, 0, 0))
                .expect("complete original no-fold scope");
            assert_eq!(declared.source_count(), 0);
            assert_eq!(declared.workspace_entries(), 0);
            assert_eq!(*declared.fold().requirement().correspondence(), id(0)); // Real axis/state compatibility is still outstanding.
        }
    });
}
