//! Explicit stable result-copy request fields and whole original inventory,not final copies/pairs.
#![allow(clippy::expect_used)]
#[allow(dead_code)]
#[path = "support/join_copies.rs"]
mod common;
use common::{id, request, with_folds};
use sc_core::{construction::*, ontology::*};
#[test]
fn explicit_copies_preserve_raw_ids_piece_orientation_order_and_whole_actual_inventory() {
    with_folds(2, |folds| {
        let raw = JoinResultCopyManifest::new(
            id(1000),
            vec![
                request(6001, CopyOrientation::Reflected),
                request(6000, CopyOrientation::Authored),
            ],
        );
        let snapshot = raw.clone();
        let limits = JoinResultCopyLimits::new(3, 3, 2, 8);
        let declared = raw
            .admit(&folds, limits)
            .expect("complete explicit requested population;orientation/full plan remains later");
        assert!(std::ptr::eq(declared.manifest(), &raw));
        assert!(std::ptr::eq(declared.folds(), &folds));
        assert_eq!(declared.limits(), limits);
        assert_eq!(raw.operation(), id(1000));
        assert_eq!(
            (
                limits.copies(),
                limits.pieces(),
                limits.results(),
                limits.workspace_entries()
            ),
            (3, 3, 2, 8)
        );
        assert_eq!(
            (
                declared.source_copy_count(),
                declared.source_piece_count(),
                declared.copy_count(),
                declared.workspace_entries()
            ),
            (3, 3, 2, 8)
        );
        assert_eq!(
            declared.copies().map(|c| c.id()).collect::<Vec<_>>(),
            [id(6001), id(6000)]
        );
        assert_eq!(
            declared
                .copies()
                .rev()
                .map(|c| c.index())
                .collect::<Vec<_>>(),
            [1, 0]
        );
        for c in declared.copies() {
            assert!(std::ptr::eq(c.context(), &declared));
            let raw_copy = raw
                .copies()
                .get(c.index())
                .expect("actual original copy request");
            assert!(std::ptr::eq(c.definition(), raw_copy));
            assert!(std::ptr::eq(c.piece_id(), &raw_copy.piece));
            assert!(std::ptr::eq(c.orientation(), &raw_copy.orientation));
            assert!(std::ptr::eq(
                declared
                    .copy(c.id())
                    .expect("actual requested id")
                    .definition(),
                raw_copy
            ));
        }
        let inventory = folds
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
            .population()
            .metadata()
            .splice()
            .inventory();
        assert!(std::ptr::eq(
            declared
                .source_copy(id(202))
                .expect("actual unrelated copy"),
            inventory
                .source()
                .plan()
                .copy(id(202))
                .expect("original unrelated copy")
        ));
        assert!(std::ptr::eq(
            declared
                .source_piece(id(102))
                .expect("actual unrelated Piece"),
            inventory
                .inventory()
                .piece(id(102))
                .expect("original unrelated Piece")
        ));
        assert!(declared.copy(id(202)).is_none());
        assert!(declared.source_copy(id(6000)).is_none());
        assert!(declared.source_piece(id(5000)).is_none());
        assert_eq!(raw, snapshot);
    });
}
#[test]
fn complete_copy_piece_request_and_workspace_quotas_precede_operation_quantity_and_fields() {
    with_folds(1, |folds| {
        use JoinResultCopyRefusal as R;
        let raw = JoinResultCopyManifest::new(
            id(9999),
            vec![CutCopyDefinition {
                id: id(1000),
                piece: id(999),
                orientation: CopyOrientation::Reflected,
            }],
        );
        for (limits, cause) in [
            (
                JoinResultCopyLimits::new(2, 0, 0, 0),
                R::Copies {
                    required: 3,
                    allowed: 2,
                },
            ),
            (
                JoinResultCopyLimits::new(3, 2, 0, 0),
                R::Pieces {
                    required: 3,
                    allowed: 2,
                },
            ),
            (
                JoinResultCopyLimits::new(3, 3, 0, 0),
                R::Results {
                    required: 1,
                    allowed: 0,
                },
            ),
            (
                JoinResultCopyLimits::new(3, 3, 1, 6),
                R::Workspace {
                    required: 7,
                    allowed: 6,
                },
            ),
            (JoinResultCopyLimits::new(3, 3, 1, 7), R::Operation),
        ] {
            let error = raw
                .admit(&folds, limits)
                .expect_err("whole resources before operation/quantity/request fields");
            assert_eq!(error.refusal(), cause);
            assert_eq!(error.limits(), limits);
            assert!(std::ptr::eq(error.manifest(), &raw));
            assert!(std::ptr::eq(error.folds(), &folds));
            assert!(error.refused_copy().is_none());
            assert!(error.original_piece().is_none());
            assert!(error.original_copy().is_none());
        }
        for copies in [
            vec![],
            vec![
                request(6000, CopyOrientation::Authored),
                request(6001, CopyOrientation::Authored),
            ],
        ] {
            let actual = copies.len();
            let raw = JoinResultCopyManifest::new(id(1000), copies);
            let error = raw
                .admit(&folds, JoinResultCopyLimits::new(3, 3, 2, 8))
                .expect_err("no implicit expansion or ignored declared population");
            assert_eq!(
                error.refusal(),
                R::Count {
                    expected: 1,
                    actual
                }
            );
            assert!(error.refused_copy().is_none());
        }
    });
}
#[test]
fn first_original_request_piece_duplicate_and_scoped_id_conflicts_lend_actual_fields() {
    with_folds(1, |folds| {
        use JoinResultCopyRefusal as R;
        for (copy, cause) in [
            (
                CutCopyDefinition {
                    id: id(1000),
                    piece: id(101),
                    orientation: CopyOrientation::Authored,
                },
                R::Piece { index: 0 },
            ),
            (
                request(1000, CopyOrientation::Authored),
                R::OperationIdentity { index: 0 },
            ),
            (
                request(5000, CopyOrientation::Authored),
                R::ResultPieceIdentity { index: 0 },
            ),
            (
                request(100, CopyOrientation::Authored),
                R::OriginalPieceIdentity { index: 0 },
            ),
            (
                request(102, CopyOrientation::Authored),
                R::OriginalPieceIdentity { index: 0 },
            ),
            (
                request(200, CopyOrientation::Authored),
                R::OriginalCopyIdentity { index: 0 },
            ),
            (
                request(202, CopyOrientation::Authored),
                R::OriginalCopyIdentity { index: 0 },
            ),
        ] {
            let raw = JoinResultCopyManifest::new(id(1000), vec![copy]);
            let snapshot = raw.clone();
            let error = raw
                .admit(&folds, JoinResultCopyLimits::new(3, 3, 1, 7))
                .expect_err("actual declared Piece and scoped namespaces required");
            assert_eq!(error.refusal(), cause);
            assert!(std::ptr::eq(
                error.refused_copy().expect("actual raw copy request"),
                raw.copies().first().expect("original request")
            ));
            let inventory = folds
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
                .population()
                .metadata()
                .splice()
                .inventory();
            if matches!(cause, R::OriginalPieceIdentity { .. }) {
                assert!(std::ptr::eq(
                    error.original_piece().expect("actual conflict Piece"),
                    inventory
                        .inventory()
                        .piece(copy.id)
                        .expect("actual original inventory member")
                ));
            } else {
                assert!(error.original_piece().is_none());
            }
            if matches!(cause, R::OriginalCopyIdentity { .. }) {
                assert!(std::ptr::eq(
                    error.original_copy().expect("actual conflict copy"),
                    inventory
                        .source()
                        .plan()
                        .copy(copy.id)
                        .expect("actual original plan member")
                ));
            } else {
                assert!(error.original_copy().is_none());
            }
            assert_eq!(raw, snapshot);
        }
    });
    with_folds(2, |folds| {
        let raw = JoinResultCopyManifest::new(
            id(1000),
            vec![
                request(6000, CopyOrientation::Authored),
                request(6000, CopyOrientation::Reflected),
            ],
        );
        let error = raw
            .admit(&folds, JoinResultCopyLimits::new(3, 3, 2, 8))
            .expect_err("original second duplicate rejected");
        assert_eq!(
            error.refusal(),
            JoinResultCopyRefusal::Duplicate {
                first: 0,
                second: 1
            }
        );
        assert!(std::ptr::eq(
            error.refused_copy().expect("actual second request"),
            raw.copies().get(1).expect("original second request")
        ));
    });
}
#[test]
fn explicit_orientation_and_local_id_requests_are_not_global_issuance_or_complete_copy_plan() {
    with_folds(1, |folds| {
        for orientation in [CopyOrientation::Authored, CopyOrientation::Reflected] {
            let raw = JoinResultCopyManifest::new(id(1000), vec![request(6000, orientation)]);
            let declared = raw
                .admit(&folds, JoinResultCopyLimits::new(3, 3, 1, 7))
                .expect("requested orientation only,not canonical CutPlan compatibility");
            assert_eq!(
                *declared
                    .copy(id(6000))
                    .expect("actual explicit copy")
                    .orientation(),
                orientation
            );
            let original = declared
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
                .population();
            assert_eq!(*original.mirroring(), Mirroring::Single);
            // Source actions/exact A+B pairing and full canonical candidate inventory still follow.
            assert_eq!(declared.source_copy_count(), 3);
            assert!(declared.source_copy(id(200)).is_some());
            assert!(declared.source_copy(id(201)).is_some());
        }
    });
}
