//! Actual source physical-copy association keeps original ids apart from requested result quantities.
#![allow(clippy::expect_used)]
#[path = "support/mirror_correspondence.rs"]
mod common;
use common::edges::metadata::source as sources;
use common::{with_context, with_context_piece};
use sc_core::{construction::*, ontology::*};
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn row(copy: u128, piece: u128, orientation: CopyOrientation) -> CutCopyDefinition {
    CutCopyDefinition {
        id: id(copy),
        piece: id(piece),
        orientation,
    }
}
fn piece(identity: u128, quantity: u32, mirroring: Mirroring) -> Piece {
    let (ledger, original) = sources::fixture();
    let mut definition = original.definition().clone();
    definition.id = id(identity);
    definition.quantity = quantity;
    definition.mirroring = mirroring;
    Piece::new(definition, &ledger).expect("actual birth Piece")
}
fn request() -> MirrorPieceCorrespondenceRequirement {
    MirrorPieceCorrespondenceRequirement::new(
        MirrorPieceAxisEvidenceRequest::ThroughPoints {
            supported_axis: id(700),
        },
        id(701),
        id(702),
        id(703),
        id(704),
        id(705),
        id(706),
    )
}
fn paired() -> Vec<CutCopyDefinition> {
    use CopyOrientation::{Authored as A, Reflected as R};
    vec![
        row(410, 100, A),
        row(207, 100, R),
        row(900, 100, A),
        row(805, 100, R),
    ]
}

#[test]
fn actual_complete_source_ids_orientations_and_order_survive_independently_of_new_cut_quantity() {
    use CopyOrientation::Authored as A;
    let source = piece(100, 4, Mirroring::MirroredPairs);
    let other = piece(300, 2, Mirroring::Single);
    let definitions = vec![
        row(333, 300, A),
        row(410, 100, A),
        row(207, 100, CopyOrientation::Reflected),
        row(334, 300, A),
        row(900, 100, A),
        row(805, 100, CopyOrientation::Reflected),
    ];
    let pieces = vec![source, other];
    let plan = CutPlan::new(definitions.clone(), &pieces).expect("whole inventory");
    let original = plan.clone();
    let mut reversed = definitions.clone();
    reversed.reverse();
    let reordered = CutPlan::new(reversed, &pieces).expect("reordered ids");
    with_context(sources::axis(), vec![], |contours, points| {
        let raw = request();
        let context = contours
            .require_correspondence(&points, &raw, MirrorPieceCorrespondenceLimits::new(4, 0, 8))
            .expect("complete native requests");
        let limits = MirrorPieceSourceCopyLimits::new(6);
        let bound = context
            .preserve_source_copies(&plan, &limits)
            .expect("actual source population");
        assert!(std::ptr::eq(bound.context(), &context));
        assert!(std::ptr::eq(bound.plan(), &plan));
        assert!(std::ptr::eq(bound.limits(), &limits));
        assert_eq!(limits.copies(), 6);
        assert_eq!(
            (
                bound.copy_count(),
                bound.authored_count(),
                bound.reflected_count()
            ),
            (4, 2, 2)
        );
        assert_eq!(
            *context
                .contours()
                .coverage()
                .edges()
                .identity()
                .roles()
                .metadata()
                .cut()
                .quantity(),
            1
        );
        assert_eq!(bound.plan().copies().len(), 6);
        assert_eq!(
            bound.copies().map(|c| *c.definition()).collect::<Vec<_>>(),
            paired()
        );
        assert_eq!(
            bound.copies().rev().map(|c| c.id()).collect::<Vec<_>>(),
            vec![id(805), id(900), id(207), id(410)]
        );
        for copy in bound.copies() {
            assert!(std::ptr::eq(
                copy,
                plan.copy(copy.id()).expect("original field")
            ));
        }
        let reordered = context
            .preserve_source_copies(&reordered, &limits)
            .expect("source identities do not follow ordinals");
        assert_eq!(
            reordered.copies().map(|c| c.id()).collect::<Vec<_>>(),
            vec![id(805), id(900), id(207), id(410)]
        );
    });
    assert_eq!(plan, original);
}

#[test]
fn whole_plan_quota_precedes_source_quantity_then_complete_paired_population() {
    use CopyOrientation::Authored as A;
    use MirrorPieceSourceCopyRefusal as R;
    let birth_source = piece(100, 2, Mirroring::Single);
    let other = piece(300, 2, Mirroring::Single);
    let short = CutPlan::new(
        vec![
            row(410, 100, A),
            row(207, 100, A),
            row(333, 300, A),
            row(334, 300, A),
        ],
        &[birth_source, other],
    )
    .expect("different valid birth metadata");
    let authored = CutPlan::new(
        [410, 207, 900, 805]
            .into_iter()
            .map(|copy| row(copy, 100, A))
            .collect(),
        &[piece(100, 4, Mirroring::Single)],
    )
    .expect("unpaired birth inventory");
    let empty = CutPlan::new(vec![], &[]).expect("empty inventory");
    with_context(sources::axis(), vec![], |contours, points| {
        let raw = request();
        let context = contours
            .require_correspondence(&points, &raw, MirrorPieceCorrespondenceLimits::new(4, 0, 8))
            .expect("actual source");
        for (plan, limits, refusal) in [
            (
                &short,
                MirrorPieceSourceCopyLimits::new(3),
                R::Copies {
                    required: 4,
                    allowed: 3,
                },
            ),
            (
                &short,
                MirrorPieceSourceCopyLimits::new(4),
                R::Quantity {
                    expected: 4,
                    actual: 2,
                },
            ),
            (
                &authored,
                MirrorPieceSourceCopyLimits::new(4),
                R::UnbalancedPairs {
                    authored: 4,
                    reflected: 0,
                },
            ),
            (
                &empty,
                MirrorPieceSourceCopyLimits::new(0),
                R::Quantity {
                    expected: 4,
                    actual: 0,
                },
            ),
        ] {
            let error = context
                .preserve_source_copies(plan, &limits)
                .expect_err("no missing copy or accepted prefix");
            assert!(std::ptr::eq(error.context(), &context));
            assert!(std::ptr::eq(error.plan(), plan));
            assert!(std::ptr::eq(error.limits(), &limits));
            assert_eq!(error.refusal(), refusal);
            assert!(format!("{error}").contains("mirror source physical copies"));
        }
    });
}

#[test]
fn actual_single_and_separate_member_sources_refuse_reflected_copies_before_quantity_checks() {
    use CopyOrientation::Authored as A;
    let reflected = CutPlan::new(paired(), &[piece(100, 4, Mirroring::MirroredPairs)])
        .expect("actual paired birth");
    let authored = CutPlan::new(
        [410, 207]
            .into_iter()
            .map(|copy| row(copy, 100, A))
            .collect(),
        &[piece(100, 2, Mirroring::Single)],
    )
    .expect("actual authored birth");
    for mirroring in [
        Mirroring::Single,
        Mirroring::PairMember {
            handedness: Handedness::Right,
            companion: id(200),
        },
    ] {
        let source = piece(100, 2, mirroring);
        with_context_piece(&source, sources::axis(), vec![], |contours, points| {
            let raw = request();
            let context = contours
                .require_correspondence(
                    &points,
                    &raw,
                    MirrorPieceCorrespondenceLimits::new(4, 0, 8),
                )
                .expect("actual source mode");
            let limits = MirrorPieceSourceCopyLimits::new(4);
            assert_eq!(
                context
                    .preserve_source_copies(&reflected, &limits)
                    .expect_err("actual source metadata owns orientation")
                    .refusal(),
                MirrorPieceSourceCopyRefusal::UnexpectedReflection { copy: id(207) }
            );
            let bound = context
                .preserve_source_copies(&authored, &limits)
                .expect("complete authored population");
            assert_eq!(
                (
                    bound.copy_count(),
                    bound.authored_count(),
                    bound.reflected_count()
                ),
                (2, 2, 0)
            );
        });
    }
}
