//! Complete original-order replacement and traversal;no canonical Piece or geometric evidence.
#![allow(clippy::expect_used)]
#[path = "support/truing_piece.rs"]
mod fixture;
use fixture::{
    common::{
        domain::{q, roles::copies::id, with_domain_inventory},
        with_outside, with_partitioned_outside,
    },
    rich,
};
use sc_core::{construction::*, ontology::*};
fn raw(outside: &AdmittedTruingOutsideRoles<'_>) -> TruingDirectionManifest {
    use TruingParameterDirection as D;
    let assignments = if outside.evidence().locks().outcome().intent() == TruingOutcomeIntent::Refit
    {
        vec![
            (5, D::SameAsSource),
            (11, D::ReverseSource),
            (2, D::ReverseSource),
            (9, D::SameAsSource),
        ]
    } else {
        vec![]
    };
    TruingDirectionManifest::new(
        id(1000),
        assignments
            .into_iter()
            .map(|(tag, d)| TruingDirectionAssignment::new(LocalTag::new(tag), d))
            .collect(),
    )
}
fn outside(intent: TruingOutcomeIntent) -> TruingOutsideRoles {
    if intent == TruingOutcomeIntent::Refit {
        TruingOutsideRoles::new(
            TruingOutsideIntent::Role(LocalTag::new(2)),
            TruingOutsideIntent::Role(LocalTag::new(5)),
        )
    } else {
        TruingOutsideRoles::new(TruingOutsideIntent::Absent, TruingOutsideIntent::Absent)
    }
}
fn edge(tag: u32, direction: Direction) -> DirectedEdge {
    DirectedEdge {
        edge: EdgeRef::new(id(1000), LocalTag::new(tag)),
        direction,
    }
}
#[test]
fn complete_partial_source_partition_replaces_boundary_or_hole_and_every_repeated_construction_use()
{
    use Direction::{Original as O, Reversed as R};
    use TruingPieceOccurrenceLocation as L;
    let forward = [edge(2, R), edge(9, O), edge(11, R), edge(5, O)];
    let reverse = [edge(5, R), edge(11, O), edge(9, R), edge(2, O)];
    for side in [SeamSideId::A, SeamSideId::B] {
        for in_hole in [false, true] {
            with_domain_inventory(
                side,
                q(1, 4),
                q(3, 4),
                |f| rich(f, side, true, in_hole),
                |roles| {
                    with_partitioned_outside(
                        roles,
                        TruingOutcomeIntent::Refit,
                        outside(TruingOutcomeIntent::Refit),
                        Some(q(1, 2)),
                        |outside| {
                            let raw = raw(&outside);
                            let directions = raw
                                .admit(&outside, TruingDirectionLimits::new(4, 4, 8))
                                .expect("complete mixed directions");
                            let source = directions
                                .admit_exact_piece_source(TruingPieceSourceLimits::new(4, 2, 8))
                                .expect("actual rich source");
                            let snapshot = source.piece().definition().clone();
                            let limits = TruingContourLimits::new(4, 2, 8, 16, 20);
                            let plan = source
                                .declare_contours(limits)
                                .expect("complete generated plan");
                            assert!(std::ptr::eq(plan.source(), &source));
                            assert!(std::ptr::eq(plan.directions(), &directions));
                            assert_eq!(plan.limits(), limits);
                            assert_eq!(
                                (
                                    limits.outputs(),
                                    limits.holes(),
                                    limits.source_occurrences(),
                                    limits.generated_occurrences(),
                                    limits.workspace_entries()
                                ),
                                (4, 2, 8, 16, 20)
                            );
                            assert_eq!(
                                (
                                    plan.occurrence_count(),
                                    plan.hole_count(),
                                    plan.workspace_bound()
                                ),
                                (16, 2, 20)
                            );
                            let original = source.piece().definition();
                            let mut expected =
                                vec![(L::Boundary(0), *original.boundary.first().expect("outer"))];
                            if in_hole {
                                expected.push((
                                    L::Boundary(1),
                                    *original.boundary.get(1).expect("untouched second boundary"),
                                ));
                            } else {
                                expected.extend(
                                    reverse
                                        .iter()
                                        .enumerate()
                                        .map(|(i, e)| (L::Boundary(i + 1), *e)),
                                );
                            }
                            if in_hole {
                                expected.extend(
                                    reverse
                                        .iter()
                                        .enumerate()
                                        .map(|(index, e)| (L::Hole { hole: 0, index }, *e)),
                                );
                            } else {
                                expected.push((
                                    L::Hole { hole: 0, index: 0 },
                                    *original
                                        .holes
                                        .first()
                                        .and_then(|h| h.first())
                                        .expect("unrelated first hole"),
                                ));
                            }
                            expected.push((
                                L::Hole { hole: 1, index: 0 },
                                *original
                                    .holes
                                    .get(1)
                                    .and_then(|h| h.first())
                                    .expect("unrelated second hole"),
                            ));
                            expected.extend(
                                forward
                                    .iter()
                                    .enumerate()
                                    .map(|(i, e)| (L::Construction(i), *e)),
                            );
                            expected.push((
                                L::Construction(4),
                                *original
                                    .construction_lines
                                    .get(1)
                                    .expect("untouched construction"),
                            ));
                            expected.extend(
                                reverse
                                    .iter()
                                    .enumerate()
                                    .map(|(i, e)| (L::Construction(i + 5), *e)),
                            );
                            assert_eq!(
                                plan.occurrences()
                                    .map(|e| (e.location(), *e.directed()))
                                    .collect::<Vec<_>>(),
                                expected
                            );
                            assert_eq!(
                                plan.occurrences()
                                    .rev()
                                    .map(|e| (e.location(), *e.directed()))
                                    .collect::<Vec<_>>(),
                                expected.into_iter().rev().collect::<Vec<_>>()
                            );
                            for (i, placed) in plan.occurrences().enumerate() {
                                assert!(std::ptr::eq(placed.context(), &plan));
                                assert_eq!(placed.index(), i);
                                assert!(std::ptr::eq(placed.original().context(), &source));
                                if let Some(output) = placed.output() {
                                    assert!(placed.original().is_target());
                                    assert!(std::ptr::eq(output.context(), &directions));
                                    assert_eq!(
                                        placed.directed().edge,
                                        output.source().logical_output()
                                    );
                                } else {
                                    assert!(!placed.original().is_target());
                                    let field = match placed.original().field() {
                                        TruingPieceOccurrenceField::Directed(field) => Some(field),
                                        TruingPieceOccurrenceField::Fold(_) => None,
                                    }
                                    .expect("generated contour has an original directed field");
                                    assert_eq!(placed.directed(), field);
                                }
                            }
                            assert_eq!(source.piece().definition(), &snapshot);
                            assert!(format!("{plan:?}").contains("TruingPieceContours"));
                        },
                    );
                },
            );
        }
    }
}
#[test]
fn whole_prefix_only_suffix_only_and_two_sided_domains_have_exact_complete_source_order() {
    for side in [SeamSideId::A, SeamSideId::B] {
        for (from, to, tags) in [
            (Param::START, Param::END, vec![9]),
            (Param::START, q(3, 4), vec![9, 5]),
            (q(1, 4), Param::END, vec![2, 9]),
            (q(1, 4), q(3, 4), vec![2, 9, 5]),
        ] {
            let outside = TruingOutsideRoles::new(
                if from == Param::START {
                    TruingOutsideIntent::Absent
                } else {
                    TruingOutsideIntent::Role(LocalTag::new(2))
                },
                if to == Param::END {
                    TruingOutsideIntent::Absent
                } else {
                    TruingOutsideIntent::Role(LocalTag::new(5))
                },
            );
            with_outside(
                side,
                from,
                to,
                TruingOutcomeIntent::Refit,
                outside,
                |outside| {
                    let t = tags.len();
                    let raw = TruingDirectionManifest::new(
                        id(1000),
                        tags.iter()
                            .rev()
                            .map(|t| {
                                TruingDirectionAssignment::new(
                                    LocalTag::new(*t),
                                    TruingParameterDirection::ReverseSource,
                                )
                            })
                            .collect(),
                    );
                    let directions = raw
                        .admit(&outside, TruingDirectionLimits::new(t, t, 2 * t))
                        .expect("explicit reverse output orientation");
                    let source = directions
                        .admit_exact_piece_source(TruingPieceSourceLimits::new(t, 0, 1))
                        .expect("simple original source");
                    let plan = source
                        .declare_contours(TruingContourLimits::new(t, 0, 1, t, 2 * t))
                        .expect("complete whole edge replacement");
                    assert_eq!(
                        plan.occurrences()
                            .map(|e| *e.directed())
                            .collect::<Vec<_>>(),
                        tags.into_iter()
                            .map(|tag| edge(tag, Direction::Reversed))
                            .collect::<Vec<_>>()
                    );
                },
            );
        }
    }
}
#[test]
fn complete_counts_and_additional_workspace_refuse_before_any_partial_plan() {
    use TruingContourRefusal as R;
    with_domain_inventory(
        SeamSideId::B,
        q(1, 4),
        q(3, 4),
        |f| rich(f, SeamSideId::B, true, false),
        |roles| {
            with_partitioned_outside(
                roles,
                TruingOutcomeIntent::Refit,
                outside(TruingOutcomeIntent::Refit),
                Some(q(1, 2)),
                |outside| {
                    let raw = raw(&outside);
                    let directions = raw
                        .admit(&outside, TruingDirectionLimits::new(4, 4, 8))
                        .expect("directions");
                    let source = directions
                        .admit_exact_piece_source(TruingPieceSourceLimits::new(4, 2, 8))
                        .expect("rich source");
                    for (limits, cause) in [
                        (
                            TruingContourLimits::new(3, 1, 7, 15, 19),
                            R::Outputs {
                                required: 4,
                                allowed: 3,
                            },
                        ),
                        (
                            TruingContourLimits::new(4, 1, 7, 15, 19),
                            R::Holes {
                                required: 2,
                                allowed: 1,
                            },
                        ),
                        (
                            TruingContourLimits::new(4, 2, 7, 15, 19),
                            R::SourceOccurrences {
                                required: 8,
                                allowed: 7,
                            },
                        ),
                        (
                            TruingContourLimits::new(4, 2, 8, 15, 19),
                            R::GeneratedOccurrences {
                                required: 16,
                                allowed: 15,
                            },
                        ),
                        (
                            TruingContourLimits::new(4, 2, 8, 16, 19),
                            R::Workspace {
                                required: 20,
                                allowed: 19,
                            },
                        ),
                    ] {
                        let error = source.declare_contours(limits).expect_err("whole quota");
                        assert!(std::ptr::eq(error.source(), &source));
                        assert_eq!(error.limits(), limits);
                        assert_eq!(error.refusal(), cause);
                        assert!(error.to_string().contains("truing contour declaration"));
                    }
                    assert!(source
                        .declare_contours(TruingContourLimits::new(4, 2, 8, 16, 20))
                        .is_ok());
                },
            );
        },
    );
}
#[test]
fn explicit_preservation_keeps_every_original_contour_and_does_not_guess_a_fold() {
    for side in [SeamSideId::A, SeamSideId::B] {
        with_domain_inventory(
            side,
            q(1, 4),
            q(3, 4),
            |f| rich(f, side, true, false),
            |roles| {
                with_partitioned_outside(
                    roles,
                    TruingOutcomeIntent::PreserveSource,
                    outside(TruingOutcomeIntent::PreserveSource),
                    None,
                    |outside| {
                        let raw = raw(&outside);
                        let directions = raw
                            .admit(&outside, TruingDirectionLimits::new(0, 0, 0))
                            .expect("preserve");
                        let source = directions
                            .admit_exact_piece_source(TruingPieceSourceLimits::new(0, 2, 8))
                            .expect("actual source");
                        let plan = source
                            .declare_contours(TruingContourLimits::new(0, 2, 8, 7, 7))
                            .expect("unchanged contour declaration");
                        assert_eq!((plan.occurrence_count(), plan.workspace_bound()), (7, 7));
                        assert_eq!(
                            plan.occurrences()
                                .map(|e| *e.directed())
                                .collect::<Vec<_>>(),
                            source
                                .occurrences()
                                .filter_map(|e| match e.field() {
                                    TruingPieceOccurrenceField::Directed(f) => Some(*f),
                                    TruingPieceOccurrenceField::Fold(_) => None,
                                })
                                .collect::<Vec<_>>()
                        );
                        assert!(plan.occurrences().all(|e| e.output().is_none()));
                        assert_eq!(plan.source().piece().definition().fold_edges.len(), 1);
                    },
                );
            },
        );
    }
    with_outside(
        SeamSideId::A,
        Param::START,
        Param::END,
        TruingOutcomeIntent::PreserveSource,
        outside(TruingOutcomeIntent::PreserveSource),
        |outside| {
            let raw = raw(&outside);
            let directions = raw
                .admit(&outside, TruingDirectionLimits::new(0, 0, 0))
                .expect("empty new outputs");
            let source = directions
                .admit_exact_piece_source(TruingPieceSourceLimits::new(0, 0, 1))
                .expect("simple original");
            let plan = source
                .declare_contours(TruingContourLimits::new(0, 0, 1, 1, 1))
                .expect("simple retained plan");
            assert_eq!(plan.occurrences().len(), 1);
        },
    );
}
