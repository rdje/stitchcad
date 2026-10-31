//! Complete reflection/map evidence requests preserve authentic parents and unproved obligations.
#![allow(clippy::expect_used)]
#[path = "support/mirror_correspondence.rs"]
mod common;
use common::with_context;
use sc_core::{construction::*, ontology::*};
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn point(creator: u128, tag: u32) -> PointRef {
    PointRef::new(id(creator), LocalTag::new(tag))
}
fn axis() -> MirrorAxis {
    MirrorAxis::ThroughPoints {
        start: point(90, 7),
        end: point(91, 3),
    }
}
fn candidates() -> Vec<MirrorPiecePointCandidate> {
    vec![
        MirrorPiecePointCandidate::new(LocalTag::new(9), point(90, 7)),
        MirrorPiecePointCandidate::new(LocalTag::new(2), point(91, 3)),
    ]
}
fn request(axis: MirrorPieceAxisEvidenceRequest) -> MirrorPieceCorrespondenceRequirement {
    MirrorPieceCorrespondenceRequirement::new(
        axis,
        id(701),
        id(702),
        id(703),
        id(704),
        id(705),
        id(706),
    )
}
fn range_axis() -> MirrorAxis {
    let param = |n| Param::new(Rational::new(n, 5).expect("exact")).expect("unit interval");
    MirrorAxis::AlongRange(DirectedRange {
        range: EdgeRange::new(EdgeRef::new(id(70), LocalTag::new(37)), param(1), param(4))
            .expect("original range"),
        direction: Direction::Reversed,
    })
}
const KINDS: [MirrorPieceCorrespondenceEvidenceKind; 6] = [
    MirrorPieceCorrespondenceEvidenceKind::SourceFrames,
    MirrorPieceCorrespondenceEvidenceKind::EdgeCorrespondence,
    MirrorPieceCorrespondenceEvidenceKind::PointCorrespondence,
    MirrorPieceCorrespondenceEvidenceKind::Contours,
    MirrorPieceCorrespondenceEvidenceKind::Accuracy,
    MirrorPieceCorrespondenceEvidenceKind::SourcePreservation,
];

#[test]
fn complete_requests_keep_actual_fields_parents_axis_and_every_common_obligation() {
    with_context(axis(), candidates(), |contours, points| {
        let raw = request(MirrorPieceAxisEvidenceRequest::ThroughPoints {
            supported_axis: id(700),
        });
        let original = raw;
        let limits = MirrorPieceCorrespondenceLimits::new(4, 2, 8);
        let bound = contours
            .require_correspondence(&points, &raw, limits)
            .expect("same actual Piece and axis");
        assert!(std::ptr::eq(bound.contours(), &contours));
        assert!(std::ptr::eq(bound.points(), &points));
        assert!(std::ptr::eq(bound.requirement(), &raw));
        assert_eq!(bound.limits(), limits);
        assert_eq!(
            (limits.edges(), limits.points(), limits.source_occurrences()),
            (4, 2, 8)
        );
        assert!(std::ptr::eq(
            bound.axis(),
            contours
                .coverage()
                .edges()
                .identity()
                .roles()
                .metadata()
                .cut()
                .source()
                .input()
                .axis()
        ));
        assert_eq!(*bound.axis(), axis());
        assert!(std::ptr::eq(bound.requirement().axis(), raw.axis()));
        assert_eq!(bound.requirements().len(), 6);
        for (source, kind) in bound.requirements().zip(KINDS) {
            assert!(std::ptr::eq(source.owner(), &bound));
            assert_eq!(source.kind(), kind);
            assert!(std::ptr::eq(source.source(), raw.source(kind)));
        }
        assert_eq!(
            bound
                .requirements()
                .rev()
                .map(|s| s.kind())
                .collect::<Vec<_>>(),
            KINDS.into_iter().rev().collect::<Vec<_>>()
        );
        assert_eq!(
            bound
                .requirements()
                .map(|s| *s.source())
                .collect::<Vec<_>>(),
            (701..707).map(id).collect::<Vec<_>>()
        );
        assert_eq!(raw, original);
    });
}

#[test]
fn whole_counts_precede_actual_parent_lineage_then_axis_mode_even_with_equal_ids() {
    use MirrorPieceCorrespondenceRefusal as R;
    with_context(axis(), candidates(), |contours, points| {
        let identity = points.roles().identity();
        let other = identity
            .roles()
            .declare_piece_identity(identity.piece_id())
            .expect("same values,different declaration");
        assert_eq!(other.piece_id(), identity.piece_id());
        let roles = other
            .bind_point_roles(points.roles().catalog(), points.roles().limits())
            .expect("other authentic parent");
        let other_points = points
            .manifest()
            .admit(&roles, points.limits())
            .expect("same proposal,other parent");
        let raw = request(MirrorPieceAxisEvidenceRequest::AlongRange {
            straight_axis: id(699),
            supported_axis: id(700),
        });
        for (limits, refusal) in [
            (
                MirrorPieceCorrespondenceLimits::new(3, 0, 0),
                R::Edges {
                    required: 4,
                    allowed: 3,
                },
            ),
            (
                MirrorPieceCorrespondenceLimits::new(4, 1, 0),
                R::Points {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                MirrorPieceCorrespondenceLimits::new(4, 2, 7),
                R::SourceOccurrences {
                    required: 8,
                    allowed: 7,
                },
            ),
            (
                MirrorPieceCorrespondenceLimits::new(4, 2, 8),
                R::PieceContext,
            ),
        ] {
            let error = contours
                .require_correspondence(&other_points, &raw, limits)
                .expect_err("whole counts then lineage before mode");
            assert!(std::ptr::eq(error.contours(), &contours));
            assert!(std::ptr::eq(error.points(), &other_points));
            assert!(std::ptr::eq(error.requirement(), &raw));
            assert_eq!(error.limits(), limits);
            assert_eq!(error.refusal(), refusal);
            assert!(format!("{error}").contains("mirror correspondence request"));
        }
    });
}

#[test]
fn both_original_axis_modes_require_their_actual_mode_without_range_straightness_inference() {
    let through = MirrorPieceAxisEvidenceRequest::ThroughPoints {
        supported_axis: id(700),
    };
    let along = MirrorPieceAxisEvidenceRequest::AlongRange {
        straight_axis: id(699),
        supported_axis: id(700),
    };
    for (axis, expected, wrong) in [(axis(), through, along), (range_axis(), along, through)] {
        with_context(axis, candidates(), |contours, points| {
            let raw = request(expected);
            let bound = contours
                .require_correspondence(
                    &points,
                    &raw,
                    MirrorPieceCorrespondenceLimits::new(4, 2, 8),
                )
                .expect("actual mode");
            assert_eq!(*bound.axis(), axis);
            assert_eq!(*bound.requirement().axis(), expected);
            let wrong = request(wrong);
            assert_eq!(
                contours
                    .require_correspondence(
                        &points,
                        &wrong,
                        MirrorPieceCorrespondenceLimits::new(4, 2, 8)
                    )
                    .expect_err("no axis-kind fallback")
                    .refusal(),
                MirrorPieceCorrespondenceRefusal::AxisKind
            );
        });
    }
}

#[test]
fn zero_point_candidates_and_one_shared_zero_provider_do_not_remove_any_requirement() {
    with_context(axis(), vec![], |contours, points| {
        let zero = id(0);
        let raw = MirrorPieceCorrespondenceRequirement::new(
            MirrorPieceAxisEvidenceRequest::ThroughPoints {
                supported_axis: zero,
            },
            zero,
            zero,
            zero,
            zero,
            zero,
            zero,
        );
        let bound = contours
            .require_correspondence(&points, &raw, MirrorPieceCorrespondenceLimits::new(4, 0, 8))
            .expect("explicit empty point metadata");
        assert_eq!(bound.points().point_count(), 0);
        assert_eq!(bound.requirements().len(), 6);
        assert_eq!(
            bound.requirements().map(|s| s.kind()).collect::<Vec<_>>(),
            KINDS
        );
        for source in bound.requirements() {
            assert_eq!(*source.source(), zero);
            assert!(std::ptr::eq(source.source(), raw.source(source.kind())));
        }
        assert!(points.roles().catalog().is_active(LocalTag::new(9)));
    });
}
