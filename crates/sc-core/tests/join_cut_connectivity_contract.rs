//! Required original source-port gluing,without synthetic geometry or global currentness claims.
#![allow(clippy::expect_used)]
#[allow(dead_code)]
#[path = "support/join_edges.rs"]
mod common;
use common::{
    id, with_custom_directions, with_directions, with_identity, BankFixture, SpanFixture,
};
use sc_core::{construction::*, ontology::*};
fn use_(tag: u32, direction: Direction) -> JoinCutContourUse {
    JoinCutContourUse::new(LocalTag::new(tag), direction)
}
fn raw(rows: &[(u32, Direction)], holes: Vec<Vec<JoinCutContourUse>>) -> JoinCutContourManifest {
    JoinCutContourManifest::new(
        id(1000),
        rows.iter()
            .map(|(tag, direction)| use_(*tag, *direction))
            .collect(),
        holes,
    )
}
fn admit<'a>(
    raw: &'a JoinCutContourManifest,
    directions: &'a AdmittedJoinDirections<'a>,
) -> DeclaredJoinCutContours<'a> {
    let n = directions.coverage().edges().edge_count();
    let h = raw.holes().len();
    let u = raw.boundary().len() + raw.holes().iter().map(Vec::len).sum::<usize>();
    raw.admit(directions, JoinCutContourLimits::new(n, h, u, 2 * u + h))
        .expect("complete actual cut-use declarations")
}
fn outer_rows(mode: SeamDirection) -> Vec<(u32, Direction)> {
    use Direction::{Original as O, Reversed as R};
    if mode == SeamDirection::Opposite {
        vec![(9, O), (5, R), (6, R), (8, R), (2, O), (4, O)]
    } else {
        vec![(9, O), (5, O), (8, O), (6, O), (2, O), (4, O)]
    }
}
#[test]
fn same_opposite_and_reversed_proposals_follow_actual_directed_source_vertices_and_seam_pairs() {
    for mode in [SeamDirection::Opposite, SeamDirection::Same] {
        with_custom_directions(
            BankFixture {
                span: SpanFixture::RetainedTail,
                extra_boundary_edges: true,
                seam_direction: mode,
                ..BankFixture::basic()
            },
            |directions| {
                let rows = outer_rows(mode);
                let splice = directions
                    .coverage()
                    .edges()
                    .identity()
                    .roles()
                    .banks()
                    .sources()
                    .population()
                    .metadata()
                    .splice();
                for end in [JoinSpliceEnd::AFrom, JoinSpliceEnd::ATo] {
                    let pair = splice.endpoint_pair(end);
                    assert!(std::ptr::eq(pair.context(), splice));
                    assert_eq!(pair.end(), end);
                    let a = pair.source_side(SeamSideId::A).range;
                    let b = pair.source_side(SeamSideId::B).range;
                    let a_from = end == JoinSpliceEnd::AFrom;
                    let b_from = if mode == SeamDirection::Same {
                        a_from
                    } else {
                        !a_from
                    };
                    assert_eq!(
                        pair.anchor(SeamSideId::A),
                        EdgeAnchor {
                            edge: a.edge(),
                            param: if a_from { a.from() } else { a.to() }
                        }
                    );
                    assert_eq!(
                        pair.anchor(SeamSideId::B),
                        EdgeAnchor {
                            edge: b.edge(),
                            param: if b_from { b.from() } else { b.to() }
                        }
                    );
                }
                let reversed = rows
                    .iter()
                    .rev()
                    .map(|(tag, direction)| {
                        (
                            *tag,
                            match direction {
                                Direction::Original => Direction::Reversed,
                                Direction::Reversed => Direction::Original,
                            },
                        )
                    })
                    .collect::<Vec<_>>();
                for rows in [rows, reversed] {
                    let raw = raw(&rows, vec![]);
                    let snapshot = raw.clone();
                    let contours = admit(&raw, &directions);
                    let limits = JoinCutConnectivityLimits::new(0, 6);
                    let connectivity = contours
                        .require_cut_connectivity(limits)
                        .expect("required source path and seam links only");
                    assert!(std::ptr::eq(connectivity.contours(), &contours));
                    assert_eq!(connectivity.limits(), limits);
                    assert_eq!(limits.holes(), 0);
                    assert_eq!(limits.uses(), 6);
                    assert_eq!(connectivity.connection_count(), 6);
                    assert_eq!(raw, snapshot);
                    assert!(format!("{connectivity:?}").contains("RequiredJoinCutConnectivity"));
                }
            },
        );
    }
}
#[test]
fn reordering_and_wrong_direction_refuse_actual_first_pairs_even_for_byte_equal_a_b_edge_fields() {
    use Direction::{Original as O, Reversed as R};
    use JoinCutContourUseLocation::Boundary;
    for shared in [false, true] {
        with_custom_directions(
            BankFixture {
                span: SpanFixture::RetainedTail,
                extra_boundary_edges: true,
                shared_edge_fields: shared,
                ..BankFixture::basic()
            },
            |directions| {
                let sources = directions
                    .coverage()
                    .edges()
                    .identity()
                    .roles()
                    .banks()
                    .sources();
                assert_ne!(
                    sources.piece(SeamSideId::A).id(),
                    sources.piece(SeamSideId::B).id()
                );
                if shared {
                    assert_eq!(
                        sources.piece(SeamSideId::A).definition().boundary,
                        sources.piece(SeamSideId::B).definition().boundary
                    );
                }
                for (rows, from, to) in [
                    (vec![(9, O), (5, R), (8, R), (6, R), (2, O), (4, O)], 1, 2),
                    (vec![(9, O), (5, R), (2, O), (4, O), (6, R), (8, R)], 1, 2),
                    (vec![(9, R), (5, R), (6, R), (8, R), (2, O), (4, O)], 0, 1),
                ] {
                    let raw = raw(&rows, vec![]);
                    let snapshot = raw.clone();
                    let contours = admit(&raw, &directions);
                    let limits = JoinCutConnectivityLimits::new(0, 6);
                    let error = contours
                        .require_cut_connectivity(limits)
                        .expect_err("original source-path contradiction");
                    assert_eq!(
                        error.refusal(),
                        JoinCutConnectivityRefusal::Adjacency {
                            from: Boundary(from),
                            to: Boundary(to)
                        }
                    );
                    assert!(std::ptr::eq(error.contours(), &contours));
                    assert_eq!(error.limits(), limits);
                    assert!(std::ptr::eq(
                        error.from_use().expect("actual preceding use").declared(),
                        raw.boundary().get(from).expect("original preceding field")
                    ));
                    assert!(std::ptr::eq(
                        error.to_use().expect("actual next use").declared(),
                        raw.boundary().get(to).expect("original next field")
                    ));
                    assert_eq!(raw, snapshot);
                    assert!(error
                        .to_string()
                        .starts_with("join required cut connectivity:"));
                    assert!(format!("{error:?}").contains("JoinCutConnectivityError"));
                }
            },
        );
    }
}
#[test]
fn whole_h_u_quotas_precede_bad_adjacency_and_unrelated_hole_loops_remain_distinct() {
    use Direction::{Original as O, Reversed as R};
    use JoinCutContourUseLocation::Boundary;
    with_directions(true, |directions| {
        let holes = vec![vec![use_(10, O)], vec![use_(2, R)]];
        let raw = raw(&[(9, O), (8, O)], holes.clone());
        let contours = admit(&raw, &directions);
        for (limits, cause) in [
            (
                JoinCutConnectivityLimits::new(1, 0),
                JoinCutConnectivityRefusal::Holes {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                JoinCutConnectivityLimits::new(2, 3),
                JoinCutConnectivityRefusal::Uses {
                    required: 4,
                    allowed: 3,
                },
            ),
            (
                JoinCutConnectivityLimits::new(2, 4),
                JoinCutConnectivityRefusal::Adjacency {
                    from: Boundary(0),
                    to: Boundary(1),
                },
            ),
        ] {
            let error = contours
                .require_cut_connectivity(limits)
                .expect_err("quota first,then original adjacency");
            assert_eq!(error.refusal(), cause);
            assert!(std::ptr::eq(error.contours(), &contours));
            if !matches!(cause, JoinCutConnectivityRefusal::Adjacency { .. }) {
                assert!(error.from_use().is_none() && error.to_use().is_none());
            }
        }
        let raw = JoinCutContourManifest::new(id(1000), vec![use_(9, R), use_(8, O)], holes);
        let contours = admit(&raw, &directions);
        assert_eq!(
            contours
                .require_cut_connectivity(JoinCutConnectivityLimits::new(2, 4))
                .expect("separate actual holes close in source topology")
                .connection_count(),
            4
        );
        let raw = JoinCutContourManifest::new(
            id(1000),
            vec![use_(9, R), use_(8, O), use_(10, O), use_(2, R)],
            vec![],
        );
        let contours = admit(&raw, &directions);
        assert_eq!(
            contours
                .require_cut_connectivity(JoinCutConnectivityLimits::new(0, 4))
                .expect_err("an unrelated hole cannot append to boundary")
                .refusal(),
            JoinCutConnectivityRefusal::Adjacency {
                from: Boundary(1),
                to: Boundary(2)
            }
        );
    });
}
#[test]
fn partial_bank_prefix_suffix_and_interior_fragment_ports_remain_in_their_exact_source_frames() {
    use Direction::{Original as O, Reversed as R};
    for mode in [SeamDirection::Opposite, SeamDirection::Same] {
        with_custom_directions(
            BankFixture {
                span: SpanFixture::RetainedMiddle,
                seam_direction: mode,
                ..BankFixture::basic()
            },
            |directions| {
                let rows = if mode == SeamDirection::Opposite {
                    vec![(9, O), (5, R), (4, O), (2, R)]
                } else {
                    vec![(9, O), (4, R), (5, O), (2, R)]
                };
                let raw = raw(&rows, vec![]);
                let contours = admit(&raw, &directions);
                assert_eq!(
                    contours
                        .require_cut_connectivity(JoinCutConnectivityLimits::new(0, 4))
                        .expect("both original outside parts and seam ends")
                        .connection_count(),
                    4
                );
            },
        );
        with_identity(
            BankFixture {
                span: SpanFixture::RetainedMiddle,
                seam_direction: mode,
                ..BankFixture::basic()
            },
            |identity| {
                let p = |n, d| {
                    Param::new(Rational::new(n, d).expect("fraction")).expect("unit interval")
                };
                let a = identity.roles().banks().bank(SeamSideId::A);
                let b = identity.roles().banks().bank(SeamSideId::B);
                let make = |tag, side: SeamSideId, edge, from, to| {
                    JoinEdgeCandidate::new(
                        LocalTag::new(tag),
                        side,
                        JoinSourceOccurrenceLocation::Boundary(0),
                        EdgeRange::new(edge, from, to).expect("exact positive original interval"),
                    )
                };
                let raw_edges = JoinEdgeManifest::new(
                    id(1000),
                    vec![
                        make(9, SeamSideId::A, a.range().edge(), Param::START, p(1, 8)),
                        make(12, SeamSideId::A, a.range().edge(), p(1, 8), p(1, 4)),
                        make(2, SeamSideId::A, a.range().edge(), p(3, 4), Param::END),
                        make(4, SeamSideId::B, b.range().edge(), Param::START, p(1, 4)),
                        make(5, SeamSideId::B, b.range().edge(), p(3, 4), Param::END),
                    ],
                );
                let edges = raw_edges
                    .admit(&identity, JoinEdgeManifestLimits::new(5, 10))
                    .expect("actual original fields");
                let coverage = edges
                    .require_cut_coverage(JoinCutCoverageLimits::new(5, 2, 5))
                    .expect("exact complete source intervals");
                let raw_directions = JoinDirectionManifest::new(
                    id(1000),
                    edges
                        .edges()
                        .map(|e| {
                            JoinDirectionAssignment::new(
                                e.candidate().tag(),
                                JoinParameterDirection::SameAsSource,
                            )
                        })
                        .collect(),
                );
                let directions = raw_directions
                    .admit(&coverage, JoinDirectionLimits::new(5, 5, 10))
                    .expect("complete explicit parameter requests");
                let rows = if mode == SeamDirection::Opposite {
                    vec![(9, O), (12, O), (5, O), (4, O), (2, O)]
                } else {
                    vec![(9, O), (12, O), (4, R), (5, R), (2, O)]
                };
                let raw = raw(&rows, vec![]);
                let contours = admit(&raw, &directions);
                assert_eq!(
                    contours
                        .require_cut_connectivity(JoinCutConnectivityLimits::new(0, 5))
                        .expect("literal interior fragment endpoint")
                        .connection_count(),
                    5
                );
            },
        );
    }
}
