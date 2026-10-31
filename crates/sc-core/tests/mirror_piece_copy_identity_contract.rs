//! Bounded actual copy ids and local typed exclusions remain distinct from global freshness.
#![allow(clippy::expect_used)]
#[path = "support/mirror_correspondence.rs"]
mod common;
use common::edges::metadata::source as sources;
use common::{with_context, with_context_metadata};
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
fn paired() -> Vec<CutCopyDefinition> {
    use CopyOrientation::{Authored as A, Reflected as R};
    vec![
        row(410, 100, A),
        row(207, 100, R),
        row(900, 100, A),
        row(805, 100, R),
    ]
}
fn points() -> Vec<MirrorPiecePointCandidate> {
    vec![MirrorPiecePointCandidate::new(
        LocalTag::new(9),
        PointRef::new(id(92), LocalTag::new(5)),
    )]
}
fn request() -> MirrorPieceCorrespondenceRequirement {
    MirrorPieceCorrespondenceRequirement::new(
        MirrorPieceAxisEvidenceRequest::ThroughPoints {
            supported_axis: id(600),
        },
        id(601),
        id(602),
        id(603),
        id(604),
        id(605),
        id(606),
    )
}
fn other_piece(identity: u128, quantity: u32) -> Piece {
    let (ledger, piece) = sources::fixture();
    let mut definition = piece.definition().clone();
    definition.id = id(identity);
    definition.quantity = quantity;
    definition.mirroring = Mirroring::Single;
    Piece::new(definition, &ledger).expect("actual other Piece")
}
fn with_inventory(
    rows: Vec<CutCopyDefinition>,
    others: Vec<Piece>,
    points: Vec<MirrorPiecePointCandidate>,
    run: impl FnOnce(BoundMirrorPieceSourceCopies<'_>),
) {
    with_context(sources::axis(), points, |contours, points| {
        let request = request();
        let context = contours
            .require_correspondence(
                &points,
                &request,
                MirrorPieceCorrespondenceLimits::new(4, 2, 8),
            )
            .expect("actual parents");
        let source = contours
            .coverage()
            .edges()
            .identity()
            .roles()
            .metadata()
            .cut()
            .source()
            .piece();
        let mut pieces = vec![source.clone()];
        pieces.extend(others);
        let plan = CutPlan::new(rows, &pieces).expect("actual complete birth plan");
        let limits = MirrorPieceSourceCopyLimits::new(10);
        let source = context
            .preserve_source_copies(&plan, &limits)
            .expect("whole original source copies");
        run(source);
    });
}

#[test]
fn whole_plan_index_and_typed_exclusions_keep_actual_records_without_claiming_freshness() {
    use CopyOrientation::Authored as A;
    let mut rows = paired();
    rows.insert(0, row(333, 300, A));
    rows.push(row(334, 300, A));
    with_inventory(rows, vec![other_piece(300, 2)], points(), |source| {
        let limits = MirrorPieceCopyIdentityLimits::new(6, 8, 1, 30);
        let bound = source
            .bind_copy_identities(&limits)
            .expect("whole local identity context");
        assert!(std::ptr::eq(bound.source(), &source));
        assert!(std::ptr::eq(bound.limits(), &limits));
        assert_eq!(
            (
                limits.copies(),
                limits.source_occurrences(),
                limits.points(),
                limits.workspace_entries()
            ),
            (6, 8, 1, 30)
        );
        assert_eq!(
            (bound.indexed_copy_count(), bound.workspace_bound()),
            (6, 30)
        );
        for copy in source.plan().copies() {
            assert!(std::ptr::eq(
                bound.copy(copy.id()).expect("actual indexed record"),
                copy
            ));
            assert!(bound.excludes_new_copy(copy.id()));
        }
        for piece in [100, 200, 300] {
            assert!(bound.knows_piece(id(piece)));
            assert!(bound.excludes_new_copy(id(piece)));
        }
        for operation in [10, 40, 41, 42, 43, 90, 91, 92] {
            assert!(bound.excludes_new_copy(id(operation)));
        }
        assert!(bound.copy(id(999)).is_none());
        assert!(!bound.knows_piece(id(999)));
        assert!(!bound.excludes_new_copy(id(999)));
        assert!(!bound.excludes_new_copy(id(0)));
        assert_eq!(source.copy_count(), 4);
    });
}

#[test]
fn complete_quotas_precede_a_known_copy_operation_identity_collision() {
    use MirrorPieceCopyIdentityRefusal as R;
    let mut rows = paired();
    rows.first_mut().expect("first copy").id = id(40);
    with_inventory(rows, vec![], points(), |source| {
        for (limits, refusal) in [
            (
                MirrorPieceCopyIdentityLimits::new(3, 0, 0, 0),
                R::Copies {
                    required: 4,
                    allowed: 3,
                },
            ),
            (
                MirrorPieceCopyIdentityLimits::new(4, 7, 0, 0),
                R::SourceOccurrences {
                    required: 8,
                    allowed: 7,
                },
            ),
            (
                MirrorPieceCopyIdentityLimits::new(4, 8, 0, 0),
                R::Points {
                    required: 1,
                    allowed: 0,
                },
            ),
            (
                MirrorPieceCopyIdentityLimits::new(4, 8, 1, 25),
                R::Workspace {
                    required: 26,
                    allowed: 25,
                },
            ),
            (
                MirrorPieceCopyIdentityLimits::new(4, 8, 1, 26),
                R::CopyIdentityCollision {
                    index: 0,
                    id: id(40),
                },
            ),
        ] {
            let error = source
                .bind_copy_identities(&limits)
                .expect_err("whole refusal before accepting a local identity index");
            assert!(std::ptr::eq(error.source(), &source));
            assert!(std::ptr::eq(error.limits(), &limits));
            assert_eq!(error.refusal(), refusal);
            assert!(format!("{error}").contains("mirror copy identity exclusions"));
        }
    });
}

#[test]
fn unrelated_plan_piece_cannot_alias_a_known_source_creating_operation() {
    use CopyOrientation::Authored as A;
    let mut rows = paired();
    rows.insert(0, row(333, 40, A));
    with_inventory(rows, vec![other_piece(40, 1)], vec![], |source| {
        assert_eq!(
            source
                .bind_copy_identities(&MirrorPieceCopyIdentityLimits::new(5, 8, 0, 27))
                .expect_err("whole plan includes unrelated pattern Piece ids")
                .refusal(),
            MirrorPieceCopyIdentityRefusal::PieceIdentityCollision {
                index: 0,
                piece: id(40)
            }
        );
    });
}

fn with_typed_inventory(companion: u128, run: impl FnOnce(BoundMirrorPieceSourceCopies<'_>)) {
    let (ledger, piece) = sources::fixture();
    let mut definition = piece.definition().clone();
    definition.mirroring = Mirroring::PairMember {
        handedness: Handedness::Right,
        companion: id(companion),
    };
    definition.material = MaterialAssignment::Assigned(id(700));
    let piece = Piece::new(definition, &ledger).expect("explicit source member/material");
    let input = sources::input(10, 100, sources::axis());
    let source_limits = MirrorPieceSourceLimits::new(2, 8);
    let source = input
        .bind_piece(&piece, &source_limits)
        .expect("actual source");
    let cut_policy = MirrorPieceCutPolicy::new(
        1,
        Mirroring::PairMember {
            handedness: Handedness::Left,
            companion: id(888),
        },
        MirrorPieceFoldIntent::NoFold,
        CuttingSide::Face,
    );
    let cut = source
        .bind_cut_policy(&cut_policy)
        .expect("explicit result member");
    let fields = MirrorPieceMetadataPolicy::new(
        MirrorPieceFieldIntent::Replace(MaterialAssignment::Assigned(id(800))),
        MirrorPieceFieldIntent::PreserveSource,
        MirrorPieceFieldIntent::PreserveSource,
    );
    let field_limits = MirrorPieceMetadataLimits::new(27);
    let metadata = cut
        .bind_metadata(&fields, &field_limits)
        .expect("actual material fields");
    with_context_metadata(metadata, vec![], |contours, points| {
        let request = request();
        let context = contours
            .require_correspondence(
                &points,
                &request,
                MirrorPieceCorrespondenceLimits::new(4, 0, 8),
            )
            .expect("same actual parents");
        let plan = CutPlan::new(
            [410, 207, 900, 805]
                .into_iter()
                .map(|copy| row(copy, 100, CopyOrientation::Authored))
                .collect(),
            core::slice::from_ref(&piece),
        )
        .expect("complete actual authored copies");
        let limits = MirrorPieceSourceCopyLimits::new(4);
        run(context
            .preserve_source_copies(&plan, &limits)
            .expect("actual separate-member source copies"));
    });
}

#[test]
fn both_material_and_companion_fields_exclude_new_copy_ids_and_conflicting_contexts_refuse() {
    with_typed_inventory(999, |source| {
        let limits = MirrorPieceCopyIdentityLimits::new(4, 8, 0, 25);
        let bound = source
            .bind_copy_identities(&limits)
            .expect("known typed use ids");
        for identity in [700, 800, 888, 999] {
            assert!(bound.excludes_new_copy(id(identity)));
        }
        for piece in [100, 200, 888, 999] {
            assert!(bound.knows_piece(id(piece)));
        }
    });
    with_typed_inventory(90, |source| {
        assert_eq!(
            source
                .bind_copy_identities(&MirrorPieceCopyIdentityLimits::new(3, 8, 0, 25))
                .expect_err("quota before typed conflict")
                .refusal(),
            MirrorPieceCopyIdentityRefusal::Copies {
                required: 4,
                allowed: 3
            }
        );
        assert_eq!(
            source
                .bind_copy_identities(&MirrorPieceCopyIdentityLimits::new(4, 8, 0, 25))
                .expect_err("Piece companion cannot be an axis-creating operation")
                .refusal(),
            MirrorPieceCopyIdentityRefusal::ContextIdentityCollision { id: id(90) }
        );
    });
}
