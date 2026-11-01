//! Explicit complete native cut placement;actual connectivity/winding/geometry remain later.
#![allow(clippy::expect_used)]
#[allow(dead_code)]
#[path = "support/join_edges.rs"]
mod common;
use common::{id, with_directions, with_identity, BankFixture};
use sc_core::{construction::*, ontology::*};
fn declared(t: u32, direction: Direction) -> JoinCutContourUse {
    JoinCutContourUse::new(LocalTag::new(t), direction)
}
fn proposal(operation: u128) -> JoinCutContourManifest {
    JoinCutContourManifest::new(
        id(operation),
        vec![
            declared(9, Direction::Reversed),
            declared(8, Direction::Original),
        ],
        vec![
            vec![declared(10, Direction::Original)],
            vec![declared(2, Direction::Reversed)],
        ],
    )
}
#[test]
fn complete_explicit_boundary_hole_order_and_traversal_keep_actual_fields_contexts_and_directions()
{
    with_directions(true, |directions| {
        let raw = proposal(1000);
        let snapshot = raw.clone();
        let limits = JoinCutContourLimits::new(10, 2, 4, 10);
        let contours = raw
            .admit(&directions, limits)
            .expect("complete explicit cut placement only");
        assert_eq!(raw.operation(), id(1000));
        assert!(std::ptr::eq(contours.manifest(), &raw));
        assert!(std::ptr::eq(contours.directions(), &directions));
        assert_eq!(contours.limits(), limits);
        assert_eq!(
            (
                limits.outputs(),
                limits.holes(),
                limits.uses(),
                limits.workspace_entries()
            ),
            (10, 2, 4, 10)
        );
        assert_eq!(contours.workspace_entries(), 10);
        assert_eq!(contours.use_count(), 4);
        assert_eq!(contours.hole_count(), 2);
        assert_eq!(
            contours
                .boundary()
                .map(|u| *u.declared().tag())
                .collect::<Vec<_>>(),
            [9, 8].map(LocalTag::new)
        );
        assert_eq!(
            contours
                .uses()
                .map(|u| *u.declared().tag())
                .collect::<Vec<_>>(),
            [9, 8, 10, 2].map(LocalTag::new)
        );
        assert_eq!(
            contours
                .uses()
                .rev()
                .map(|u| *u.declared().tag())
                .collect::<Vec<_>>(),
            [2, 10, 8, 9].map(LocalTag::new)
        );
        for placed in contours.uses() {
            let original = match placed.location() {
                JoinCutContourUseLocation::Boundary(index) => raw.boundary().get(index),
                JoinCutContourUseLocation::Hole { hole, index } => {
                    raw.holes().get(hole).and_then(|h| h.get(index))
                }
            }
            .expect("actual original use");
            assert!(std::ptr::eq(placed.context(), &contours));
            assert!(std::ptr::eq(placed.declared(), original));
            assert!(std::ptr::eq(placed.direction(), original.direction()));
            assert!(std::ptr::eq(placed.output().context(), &directions));
            assert_eq!(
                placed.logical_output(),
                EdgeRef::new(id(1000), *original.tag())
            );
            assert!(std::ptr::eq(
                contours
                    .use_for(*original.tag())
                    .expect("unique explicit tag")
                    .declared(),
                original
            ));
            assert_eq!(placed.output().source().candidate().tag(), *original.tag());
        }
        for (index, hole) in contours.holes().enumerate() {
            assert!(std::ptr::eq(hole.context(), &contours));
            assert_eq!(hole.index(), index);
            assert!(std::ptr::eq(
                hole.declared(),
                raw.holes().get(index).expect("actual raw group").as_slice()
            ));
            assert_eq!(hole.uses().len(), 1);
            let placed = hole.uses().next_back().expect("original hole member");
            assert_eq!(
                placed.location(),
                JoinCutContourUseLocation::Hole {
                    hole: index,
                    index: 0
                }
            );
            assert!(std::ptr::eq(
                placed.declared(),
                hole.declared().first().expect("original group field")
            ));
        }
        assert_eq!(
            contours
                .holes()
                .rev()
                .map(|h| h.index())
                .collect::<Vec<_>>(),
            [1, 0]
        );
        let a = contours
            .use_for(LocalTag::new(9))
            .expect("actual A outside");
        assert_eq!(*a.direction(), Direction::Reversed);
        assert_eq!(
            a.output().parameter_direction(),
            JoinParameterDirection::SameAsSource
        );
        assert!(contours.use_for(LocalTag::new(4)).is_none());
        assert_eq!(raw, snapshot);
        assert!(format!("{contours:?}").contains("DeclaredJoinCutContours"));
    });
}
#[test]
fn whole_output_hole_use_and_workspace_quotas_precede_wrong_owner_and_invalid_use_fields() {
    use JoinCutContourRefusal as R;
    with_directions(true, |directions| {
        let bad = declared(99, Direction::Original);
        let raw = JoinCutContourManifest::new(id(1001), vec![bad, bad], vec![vec![bad], vec![bad]]);
        let snapshot = raw.clone();
        for (limits, cause) in [
            (
                JoinCutContourLimits::new(9, 0, 0, 0),
                R::Outputs {
                    required: 10,
                    allowed: 9,
                },
            ),
            (
                JoinCutContourLimits::new(10, 1, 0, 0),
                R::Holes {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                JoinCutContourLimits::new(10, 2, 3, 0),
                R::Uses {
                    required: 4,
                    allowed: 3,
                },
            ),
            (
                JoinCutContourLimits::new(10, 2, 4, 9),
                R::Workspace {
                    required: 10,
                    allowed: 9,
                },
            ),
            (JoinCutContourLimits::new(10, 2, 4, 10), R::Operation),
        ] {
            let error = raw
                .admit(&directions, limits)
                .expect_err("whole limits/owner before use fields");
            assert!(std::ptr::eq(error.manifest(), &raw));
            assert!(std::ptr::eq(error.directions(), &directions));
            assert_eq!(error.limits(), limits);
            assert_eq!(error.refusal(), cause);
            assert!(error.refused_use().is_none());
            assert!(error.to_string().starts_with("join cut-use declarations:"));
            assert!(format!("{error:?}").contains("JoinCutContourError"));
        }
        assert_eq!(raw, snapshot);
    });
}
#[test]
fn nonempty_and_exact_cut_count_precede_fields_and_empty_cut_coverage_never_creates_a_boundary() {
    use JoinCutContourRefusal as R;
    with_directions(true, |directions| {
        let good = proposal(1000);
        let all = good
            .boundary()
            .iter()
            .chain(good.holes().iter().flatten())
            .copied()
            .collect::<Vec<_>>();
        for (raw, cause) in [
            (
                JoinCutContourManifest::new(id(1000), vec![], vec![all.clone()]),
                R::EmptyBoundary,
            ),
            (
                JoinCutContourManifest::new(id(1000), all, vec![vec![]]),
                R::EmptyHole { hole: 0 },
            ),
            (
                JoinCutContourManifest::new(
                    id(1000),
                    vec![declared(99, Direction::Original)],
                    vec![],
                ),
                R::Count {
                    expected: 4,
                    actual: 1,
                },
            ),
        ] {
            let error = raw
                .admit(&directions, JoinCutContourLimits::new(10, 2, 4, 10))
                .expect_err("nonempty/count before tags");
            assert_eq!(error.refusal(), cause);
            assert!(error.refused_use().is_none());
        }
    });
    with_identity(BankFixture::basic(), |identity| {
        let raw = JoinEdgeManifest::new(id(1000), vec![]);
        let edges = raw
            .admit(&identity, JoinEdgeManifestLimits::new(0, 0))
            .expect("empty metadata");
        let coverage = edges
            .require_cut_coverage(JoinCutCoverageLimits::new(0, 2, 0))
            .expect("whole banks consumed");
        let raw = JoinDirectionManifest::new(id(1000), vec![]);
        let directions = raw
            .admit(&coverage, JoinDirectionLimits::new(0, 0, 0))
            .expect("empty direction metadata");
        let raw = JoinCutContourManifest::new(id(1000), vec![], vec![]);
        assert_eq!(
            raw.admit(&directions, JoinCutContourLimits::new(0, 0, 0, 0))
                .expect_err("empty cut bookkeeping is no boundary")
                .refusal(),
            R::EmptyBoundary
        );
    });
}
#[test]
fn duplicate_unknown_and_noncut_use_causes_keep_original_hole_fields_and_no_accepted_prefix() {
    use JoinCutContourRefusal as R;
    use JoinCutContourUseLocation as L;
    with_directions(true, |directions| {
        let location = L::Hole { hole: 1, index: 0 };
        for (t, cause) in [
            (
                9,
                R::DuplicateTag {
                    first: L::Boundary(0),
                    second: location,
                    tag: LocalTag::new(9),
                },
            ),
            (
                99,
                R::UnknownOutput {
                    location,
                    tag: LocalTag::new(99),
                },
            ),
            (
                7,
                R::UnknownOutput {
                    location,
                    tag: LocalTag::new(7),
                },
            ),
            (
                4,
                R::NonCutOutput {
                    location,
                    tag: LocalTag::new(4),
                },
            ),
            (
                6,
                R::NonCutOutput {
                    location,
                    tag: LocalTag::new(6),
                },
            ),
            (
                13,
                R::NonCutOutput {
                    location,
                    tag: LocalTag::new(13),
                },
            ),
        ] {
            let raw = JoinCutContourManifest::new(
                id(1000),
                proposal(1000).boundary().to_vec(),
                vec![
                    vec![declared(10, Direction::Original)],
                    vec![declared(t, Direction::Reversed)],
                ],
            );
            let snapshot = raw.clone();
            let limits = JoinCutContourLimits::new(10, 2, 4, 10);
            let error = raw
                .admit(&directions, limits)
                .expect_err("first original use cause");
            assert_eq!(error.refusal(), cause);
            assert_eq!(error.limits(), limits);
            assert_eq!(raw, snapshot);
            assert!(std::ptr::eq(error.manifest(), &raw));
            assert!(std::ptr::eq(error.directions(), &directions));
            assert!(std::ptr::eq(
                error.refused_use().expect("original refused use"),
                raw.holes()
                    .get(1)
                    .expect("actual original group")
                    .first()
                    .expect("actual original field")
            ));
        }
    });
}
