//! Complete original construction action requests with expected persistent edge/traversal guards.
#![allow(clippy::expect_used)]
#[allow(dead_code)]
#[path = "support/join_edges.rs"]
mod common;
use common::{id, with_construction_uses};
use sc_core::{construction::*, ontology::*};
fn sources<'a>(uses: &DeclaredJoinConstructionUses<'a>) -> &'a BoundJoinSourcePieces<'a> {
    uses.connectivity()
        .contours()
        .directions()
        .coverage()
        .edges()
        .identity()
        .roles()
        .banks()
        .sources()
}
fn original(
    uses: &DeclaredJoinConstructionUses<'_>,
    side: SeamSideId,
    index: usize,
) -> DirectedEdge {
    *sources(uses)
        .piece(side)
        .definition()
        .construction_lines
        .get(index)
        .expect("actual original field")
}
fn row(
    uses: &DeclaredJoinConstructionUses<'_>,
    side: SeamSideId,
    index: usize,
    intent: JoinConstructionSourceIntent,
) -> JoinConstructionSourceAssignment {
    JoinConstructionSourceAssignment::new(side, index, original(uses, side, index), intent)
}
#[test]
fn complete_reordered_repeated_source_scopes_keep_actual_fields_and_ordinary_omission_requests() {
    use JoinConstructionSourceIntent::{Omit, Retain};
    use SeamSideId::{A, B};
    with_construction_uses(true, |uses| {
        let raw = JoinConstructionSourceManifest::new(
            id(1000),
            vec![
                row(
                    &uses,
                    B,
                    1,
                    Omit {
                        evidence_source: id(0),
                    },
                ),
                row(&uses, A, 0, Retain),
                row(
                    &uses,
                    B,
                    0,
                    Omit {
                        evidence_source: id(0),
                    },
                ),
                row(&uses, A, 1, Retain),
            ],
        );
        let snapshot = raw.clone();
        let limits = JoinConstructionSourceLimits::new(10, 4, 8);
        let actions = raw
            .admit(&uses, limits)
            .expect("complete explicit original actions only");
        assert_eq!(raw.operation(), id(1000));
        assert!(std::ptr::eq(actions.manifest(), &raw));
        assert!(std::ptr::eq(actions.uses(), &uses));
        assert_eq!(actions.limits(), limits);
        assert_eq!(actions.source_count(), 4);
        assert_eq!(actions.workspace_entries(), 8);
        assert_eq!(
            (
                limits.source_occurrences(),
                limits.assignments(),
                limits.workspace_entries()
            ),
            (10, 4, 8)
        );
        assert_eq!(
            actions
                .assignments()
                .map(|a| (a.assignment().side(), a.assignment().source_index()))
                .collect::<Vec<_>>(),
            [(B, 1), (A, 0), (B, 0), (A, 1)]
        );
        assert_eq!(
            actions
                .assignments()
                .rev()
                .map(|a| a.index())
                .collect::<Vec<_>>(),
            [3, 2, 1, 0]
        );
        for action in actions.assignments() {
            let field = raw
                .assignments()
                .get(action.index())
                .expect("actual raw assignment");
            let source = sources(&uses)
                .piece(field.side())
                .definition()
                .construction_lines
                .get(field.source_index())
                .expect("actual original source");
            assert!(std::ptr::eq(action.context(), &actions));
            assert!(std::ptr::eq(action.assignment(), field));
            assert!(std::ptr::eq(action.source_field(), source));
            assert_eq!(field.expected(), source);
            assert!(!std::ptr::eq(field.expected(), source));
            assert!(std::ptr::eq(action.intent(), field.intent()));
            assert!(std::ptr::eq(
                action.piece(),
                sources(&uses).piece(field.side())
            ));
            let copy = sources(&uses)
                .population()
                .metadata()
                .splice()
                .source()
                .side(field.side())
                .copy();
            assert!(std::ptr::eq(action.copy(), copy));
            assert!(std::ptr::eq(
                actions
                    .assignment(field.side(), field.source_index())
                    .expect("actual source slot")
                    .assignment(),
                field
            ));
            match field.evidence_source() {
                Some(id) => {
                    assert_eq!(*id, common::id(0));
                    assert!(std::ptr::eq(
                        action.evidence_source().expect("original evidence field"),
                        id
                    ));
                }
                None => assert!(action.evidence_source().is_none()),
            }
        }
        assert!(actions.assignment(A, 99).is_none());
        assert_eq!(raw, snapshot);
        assert_eq!(actions.uses().use_count(), 0); // Retained target maps/omission truth still require later proof.
        assert!(format!("{actions:?}").contains("DeclaredJoinConstructionSources"));
    });
}
#[test]
fn whole_r_m_and_2m_precede_wrong_owner_and_unknown_expected_source_fields() {
    use JoinConstructionSourceRefusal as R;
    with_construction_uses(true, |uses| {
        let bad = JoinConstructionSourceAssignment::new(
            SeamSideId::A,
            99,
            original(&uses, SeamSideId::A, 0),
            JoinConstructionSourceIntent::Retain,
        );
        let raw = JoinConstructionSourceManifest::new(id(1001), vec![bad; 4]);
        let snapshot = raw.clone();
        for (limits, cause) in [
            (
                JoinConstructionSourceLimits::new(9, 0, 0),
                R::SourceOccurrences {
                    required: 10,
                    allowed: 9,
                },
            ),
            (
                JoinConstructionSourceLimits::new(10, 3, 0),
                R::Assignments {
                    required: 4,
                    allowed: 3,
                },
            ),
            (
                JoinConstructionSourceLimits::new(10, 4, 7),
                R::Workspace {
                    required: 8,
                    allowed: 7,
                },
            ),
            (JoinConstructionSourceLimits::new(10, 4, 8), R::Operation),
        ] {
            let error = raw
                .admit(&uses, limits)
                .expect_err("whole resources/owner before fields");
            assert!(std::ptr::eq(error.manifest(), &raw));
            assert!(std::ptr::eq(error.uses(), &uses));
            assert_eq!(error.limits(), limits);
            assert_eq!(error.refusal(), cause);
            assert!(error.refused_assignment().is_none() && error.refused_source_field().is_none());
            assert!(error
                .to_string()
                .starts_with("join original construction actions:"));
            assert!(format!("{error:?}").contains("JoinConstructionSourceError"));
        }
        assert_eq!(raw, snapshot);
    });
}
#[test]
fn count_duplicate_missing_edge_and_direction_guards_preserve_first_original_rows_and_actual_sources(
) {
    use JoinConstructionSourceIntent::Retain;
    use JoinConstructionSourceRefusal as R;
    use SeamSideId::{A, B};
    with_construction_uses(true, |uses| {
        let bad =
            JoinConstructionSourceAssignment::new(A, usize::MAX, original(&uses, A, 0), Retain);
        for m in [3, 5] {
            let raw = JoinConstructionSourceManifest::new(id(1000), vec![bad; m]);
            let error = raw
                .admit(&uses, JoinConstructionSourceLimits::new(10, m, 2 * m))
                .expect_err("complete count before source fields");
            assert_eq!(
                error.refusal(),
                R::Count {
                    expected: 4,
                    actual: m
                }
            );
            assert!(error.refused_assignment().is_none());
        }
        let base = vec![
            row(&uses, A, 0, Retain),
            row(&uses, A, 1, Retain),
            row(&uses, B, 0, Retain),
            row(&uses, B, 1, Retain),
        ];
        let mut duplicate = base.clone();
        *duplicate.get_mut(1).expect("second") =
            JoinConstructionSourceAssignment::new(A, 0, original(&uses, A, 1), Retain);
        let mut missing = base.clone();
        *missing.get_mut(3).expect("last") =
            JoinConstructionSourceAssignment::new(B, 99, original(&uses, B, 1), Retain);
        let mut direction = base.clone();
        *direction.get_mut(3).expect("last") =
            JoinConstructionSourceAssignment::new(B, 1, original(&uses, B, 0), Retain);
        let mut edge = base;
        let foreign = DirectedEdge {
            edge: EdgeRef::new(id(6000), LocalTag::new(7)),
            direction: Direction::Reversed,
        };
        *edge.get_mut(3).expect("last") =
            JoinConstructionSourceAssignment::new(B, 1, foreign, Retain);
        for (rows, cause, index) in [
            (
                duplicate,
                R::DuplicateSource {
                    first: 0,
                    second: 1,
                    side: A,
                    source_index: 0,
                },
                1,
            ),
            (missing, R::MissingSource { assignment: 3 }, 3),
            (direction, R::SourceField { assignment: 3 }, 3),
            (edge, R::SourceField { assignment: 3 }, 3),
        ] {
            let raw = JoinConstructionSourceManifest::new(id(1000), rows);
            let snapshot = raw.clone();
            let error = raw
                .admit(&uses, JoinConstructionSourceLimits::new(10, 4, 8))
                .expect_err("first original source contradiction");
            assert_eq!(error.refusal(), cause);
            assert_eq!(raw, snapshot);
            assert!(std::ptr::eq(
                error.refused_assignment().expect("actual raw row"),
                raw.assignments().get(index).expect("original refused row")
            ));
            if matches!(cause, R::SourceField { .. }) {
                assert!(std::ptr::eq(
                    error
                        .refused_source_field()
                        .expect("actual original source"),
                    sources(&uses)
                        .piece(B)
                        .definition()
                        .construction_lines
                        .get(1)
                        .expect("original B1")
                ));
            } else {
                assert!(error.refused_source_field().is_none());
            }
        }
    });
}
#[test]
fn explicit_empty_source_scope_is_complete_only_when_there_are_no_original_construction_fields() {
    for extra in [false, true] {
        with_construction_uses(extra, |uses| {
            let raw = JoinConstructionSourceManifest::new(id(1000), vec![]);
            let limits = JoinConstructionSourceLimits::new(if extra { 10 } else { 2 }, 0, 0);
            if extra {
                assert_eq!(
                    raw.admit(&uses, limits)
                        .expect_err("all four original occurrences need actions")
                        .refusal(),
                    JoinConstructionSourceRefusal::Count {
                        expected: 4,
                        actual: 0
                    }
                );
            } else {
                let actions = raw
                    .admit(&uses, limits)
                    .expect("explicit complete empty original scope");
                assert_eq!(actions.source_count(), 0);
                assert_eq!(actions.assignments().len(), 0);
                assert_eq!(actions.workspace_entries(), 0);
                assert!(actions.assignment(SeamSideId::A, 0).is_none());
            }
        });
    }
}
