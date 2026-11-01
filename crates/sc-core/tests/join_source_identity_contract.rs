//! Complete explicit original global-edge fates preserve real fields/frames and unresolved history.
#![allow(clippy::expect_used)]
#[allow(dead_code)]
#[path = "support/join_result.rs"]
mod common;
use common::id;
use sc_core::{construction::*, ontology::*};
use std::collections::BTreeSet;
fn source<'a>(consumers: &'a RequiredJoinConsumers<'a>) -> &'a BoundJoinSourcePieces<'a> {
    consumers
        .inventory()
        .plan()
        .correspondence()
        .sources()
        .results()
        .folds()
        .fold()
        .construction()
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
}
fn with_consumers(run: impl FnOnce(RequiredJoinConsumers<'_>)) {
    common::candidate::with_bound_plan(|plan| {
        let fields = plan
            .admit_piece_fields(JoinInventoryFieldLimits::new(3, 2, 0, 24, 114))
            .expect("whole original/candidate fields");
        let raw = JoinCompanionRepairManifest::new(id(1000), vec![]);
        let repairs = raw
            .admit(&fields, JoinCompanionRepairLimits::new(0, 0))
            .expect("no companion exceptions");
        let originals = repairs
            .match_original_pieces()
            .expect("actual original fields");
        let inventory = originals
            .match_result_piece()
            .expect("actual complete joined fields");
        let sources = JoinConsumerSources::new(id(0), id(0), id(0), id(0), id(0));
        run(inventory.require_consumers(&sources));
    });
}
fn rows(
    consumers: &RequiredJoinConsumers<'_>,
    retire_all: bool,
) -> Vec<JoinSourceIdentityAssignment> {
    let edges = source(consumers)
        .occurrences()
        .map(|f| *f.field().edge())
        .collect::<BTreeSet<_>>();
    assert_eq!(edges.len(), 4);
    let mut rows = edges
        .into_iter()
        .enumerate()
        .map(|(index, edge)| {
            JoinSourceIdentityAssignment::new(
                edge,
                if retire_all || index % 2 == 0 {
                    JoinSourceIdentityIntent::RetireOriginal
                } else {
                    JoinSourceIdentityIntent::RetainOriginal
                },
            )
        })
        .collect::<Vec<_>>();
    rows.rotate_right(1);
    rows
}
#[test]
fn every_global_identity_has_one_explicit_intent_but_all_original_occurrences_and_frames_survive() {
    with_consumers(|consumers| {
        let raw = JoinSourceIdentityManifest::new(id(1000), id(0), rows(&consumers, false));
        let snapshot = raw.clone();
        let limits = JoinSourceIdentityLimits::new(10, 4, 18);
        let required = raw
            .admit(&consumers, limits)
            .expect("complete four-global-edge requests over all ten original fields");
        assert!(std::ptr::eq(required.manifest(), &raw));
        assert!(std::ptr::eq(required.consumers(), &consumers));
        assert!(std::ptr::eq(required.source_pieces(), source(&consumers)));
        assert_eq!(required.limits(), limits);
        assert_eq!(
            (
                limits.sources(),
                limits.assignments(),
                limits.workspace_entries()
            ),
            (10, 4, 18)
        );
        assert_eq!(required.workspace_entries(), 18);
        assert_eq!(
            (
                required.source_edge_count(),
                required.retirement_count(),
                required.retention_count()
            ),
            (4, 2, 2)
        );
        assert_eq!(raw.operation(), id(1000));
        assert!(std::ptr::eq(
            required.history_source(),
            raw.identity_history()
        ));
        assert!(std::ptr::eq(
            required.repair_source(),
            consumers
                .sources()
                .source(JoinConsumerEvidenceKind::VisibleRepairs)
        ));
        assert_eq!(
            required
                .assignments()
                .map(|r| r.index())
                .collect::<Vec<_>>(),
            [0, 1, 2, 3]
        );
        assert_eq!(
            required
                .assignments()
                .rev()
                .map(|r| r.index())
                .collect::<Vec<_>>(),
            [3, 2, 1, 0]
        );
        let mut occurrence_total = 0;
        for actual in required.assignments() {
            let row = raw
                .assignments()
                .get(actual.index())
                .expect("actual raw original row");
            assert!(std::ptr::eq(actual.context(), &required));
            assert!(std::ptr::eq(actual.definition(), row));
            assert!(std::ptr::eq(actual.intent(), row.intent()));
            assert_eq!(actual.source_edge(), row.edge());
            assert!(std::ptr::eq(
                actual.source_edge(),
                actual.first_source().field().edge()
            ));
            assert!(!std::ptr::eq(actual.source_edge(), row.edge()));
            assert!(std::ptr::eq(
                required
                    .assignment(*row.edge())
                    .expect("actual indexed global identity")
                    .definition(),
                row
            ));
            let expected = source(&consumers)
                .occurrences()
                .filter(|f| f.field().edge() == row.edge())
                .collect::<Vec<_>>();
            let actual_fields = actual.sources().collect::<Vec<_>>();
            occurrence_total += actual_fields.len();
            assert_eq!(actual_fields.len(), expected.len());
            for (field, original) in actual_fields.iter().zip(&expected) {
                assert!(std::ptr::eq(field.context(), original.context()));
                assert_eq!(
                    (field.side(), field.location()),
                    (original.side(), original.location())
                );
                assert!(std::ptr::eq(field.field().edge(), original.field().edge()));
                assert!(std::ptr::eq(field.piece(), original.piece()));
                assert!(std::ptr::eq(field.copy(), original.copy()));
                assert!(std::ptr::eq(field.source_side(), original.source_side()));
            }
            assert_eq!(
                actual
                    .sources()
                    .rev()
                    .map(|f| (f.side(), f.location()))
                    .collect::<Vec<_>>(),
                expected
                    .iter()
                    .rev()
                    .map(|f| (f.side(), f.location()))
                    .collect::<Vec<_>>()
            );
        }
        assert_eq!(occurrence_total, 10); // Repeated construction/fold uses were not erased by global indexing.
        assert!(required
            .assignment(EdgeRef::new(id(1000), LocalTag::new(9)))
            .is_none());
        assert_eq!(raw, snapshot);
    });
}
#[test]
fn whole_source_rows_and_workspace_precede_wrong_operation_and_bad_first_selector() {
    with_consumers(|consumers| {
        let mut values = rows(&consumers, false);
        *values.first_mut().expect("first row") = JoinSourceIdentityAssignment::new(
            EdgeRef::new(id(9999), LocalTag::new(99)),
            JoinSourceIdentityIntent::RetireOriginal,
        );
        let raw = JoinSourceIdentityManifest::new(id(9999), id(0), values);
        use JoinSourceIdentityRefusal as R;
        for (limits, cause) in [
            (
                JoinSourceIdentityLimits::new(9, 0, 0),
                R::Sources {
                    required: 10,
                    allowed: 9,
                },
            ),
            (
                JoinSourceIdentityLimits::new(10, 3, 0),
                R::Assignments {
                    required: 4,
                    allowed: 3,
                },
            ),
            (
                JoinSourceIdentityLimits::new(10, 4, 17),
                R::Workspace {
                    required: 18,
                    allowed: 17,
                },
            ),
            (JoinSourceIdentityLimits::new(10, 4, 18), R::Operation),
        ] {
            let error = raw
                .admit(&consumers, limits)
                .expect_err("complete resources before operation/raw source fields");
            assert_eq!(error.refusal(), cause);
            assert_eq!(error.limits(), limits);
            assert!(std::ptr::eq(error.manifest(), &raw));
            assert!(std::ptr::eq(error.consumers(), &consumers));
            assert!(error.refused_assignment().is_none());
            assert!(error.first_source().is_none());
        }
    });
}
#[test]
fn whole_distinct_source_count_and_first_duplicate_unknown_selector_keep_actual_original_causes() {
    with_consumers(|consumers| {
        let mut fewer = rows(&consumers, false);
        fewer.pop();
        *fewer.first_mut().expect("first row") = JoinSourceIdentityAssignment::new(
            EdgeRef::new(id(9999), LocalTag::new(99)),
            JoinSourceIdentityIntent::RetainOriginal,
        );
        let raw = JoinSourceIdentityManifest::new(id(1000), id(0), fewer);
        let error = raw
            .admit(&consumers, JoinSourceIdentityLimits::new(10, 4, 18))
            .expect_err("complete K header precedes bad raw fields");
        assert_eq!(
            error.refusal(),
            JoinSourceIdentityRefusal::Count {
                expected: 4,
                actual: 3
            }
        );
        assert!(error.refused_assignment().is_none());
        assert!(error.first_source().is_none());
        let mut duplicate = rows(&consumers, false);
        let first = *duplicate.first().expect("first actual row");
        *duplicate.get_mut(1).expect("second row") = first;
        let mut missing = rows(&consumers, false);
        *missing.first_mut().expect("first row") = JoinSourceIdentityAssignment::new(
            EdgeRef::new(id(1000), LocalTag::new(9)),
            JoinSourceIdentityIntent::RetireOriginal,
        );
        for (values, cause, index, known) in [
            (
                duplicate,
                JoinSourceIdentityRefusal::Duplicate {
                    first: 0,
                    second: 1,
                },
                1,
                true,
            ),
            (
                missing,
                JoinSourceIdentityRefusal::MissingSource { index: 0 },
                0,
                false,
            ),
        ] {
            let raw = JoinSourceIdentityManifest::new(id(1000), id(0), values);
            let snapshot = raw.clone();
            let error = raw
                .admit(&consumers, JoinSourceIdentityLimits::new(10, 4, 18))
                .expect_err("literal original source identity coverage only");
            assert_eq!(error.refusal(), cause);
            assert!(std::ptr::eq(
                error.refused_assignment().expect("actual refused row"),
                raw.assignments().get(index).expect("same actual raw row")
            ));
            assert_eq!(error.first_source().is_some(), known);
            if let Some(actual) = error.first_source() {
                assert!(std::ptr::eq(actual.context(), source(&consumers)));
                assert_eq!(
                    actual.field().edge(),
                    raw.assignments().get(index).expect("actual raw row").edge()
                );
            }
            assert_eq!(raw, snapshot);
        }
    });
}
#[test]
fn old_held_references_do_not_decide_global_retirement_or_discharge_history_and_repair_requests() {
    with_consumers(|consumers| {
        let raw = JoinSourceIdentityManifest::new(id(1000), id(0), rows(&consumers, true));
        let snapshot = raw.clone();
        let candidate = consumers
            .inventory()
            .plan()
            .piece(id(102))
            .expect("actual retained unrelated original sharing source A geometry");
        let before = candidate.definition().clone();
        let held = candidate
            .definition()
            .boundary
            .first()
            .expect("actual held original global edge")
            .edge;
        let required=raw.admit(&consumers,JoinSourceIdentityLimits::new(10,4,18)).expect("only explicit requirements:journal correspondence/visible repair must decide actual retirement");
        assert_eq!(required.retirement_count(), 4);
        assert_eq!(required.retention_count(), 0);
        assert_eq!(
            *required
                .assignment(held)
                .expect("actual original global identity still held by candidate")
                .intent(),
            JoinSourceIdentityIntent::RetireOriginal
        );
        assert_eq!(*required.history_source(), id(0));
        assert_eq!(*required.repair_source(), id(0));
        assert_eq!(required.consumers().requirements().len(), 5);
        assert_eq!(candidate.definition(), &before);
        assert_eq!(raw, snapshot);
    });
}
