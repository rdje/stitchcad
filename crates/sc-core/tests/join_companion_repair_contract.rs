//! Explicit reference repair requirements for retained separate members,not permission/evidence truth.
#![allow(clippy::expect_used)]
#[allow(dead_code)]
#[path = "support/join_candidate.rs"]
mod common;
use common::{id, with_bound_plan, with_companion_fields};
use sc_core::{construction::*, ontology::*};
fn mode() -> Mirroring {
    Mirroring::PairMember {
        handedness: Handedness::Left,
        companion: id(102),
    }
}
fn old() -> Mirroring {
    Mirroring::PairMember {
        handedness: Handedness::Right,
        companion: id(100),
    }
}
fn request() -> JoinCompanionRepairRequest {
    JoinCompanionRepairRequest::new(id(102), old(), id(5000), id(0))
}
#[test]
fn actual_reciprocal_inventory_requires_explicit_companion_replacement_and_lends_all_original_fields(
) {
    with_companion_fields(mode(), true, |fields| {
        let results = fields.plan().correspondence().sources().results();
        let old_mate = results.source_piece(id(102)).expect("actual original mate");
        let error = PieceInventory::new(vec![
            fields.plan().joined_piece().clone(),
            old_mate.clone(),
        ])
        .expect_err(
            "old source100 was consumed;old companion link cannot remain in candidate inventory",
        );
        assert_eq!(
            error,
            PieceInventoryError::NonReciprocalCompanion {
                piece: id(5000),
                companion: id(102),
                actual: id(100)
            }
        );
        let raw = JoinCompanionRepairManifest::new(id(1000), vec![request()]);
        let snapshot = raw.clone();
        let limits = JoinCompanionRepairLimits::new(1, 2);
        let bound = raw
            .admit(&fields, limits)
            .expect("guarded explicit required counterpart link");
        assert!(std::ptr::eq(bound.manifest(), &raw));
        assert!(std::ptr::eq(bound.fields(), &fields));
        assert_eq!(bound.limits(), limits);
        assert_eq!((limits.repairs(), limits.workspace_entries()), (1, 2));
        assert_eq!((bound.repair_count(), bound.workspace_entries()), (1, 2));
        assert_eq!(raw.operation(), id(1000));
        let actual = bound.repairs().next().expect("actual repair requirement");
        let authored = raw.requests().first().expect("actual original raw request");
        assert!(std::ptr::eq(actual.context(), &bound));
        assert_eq!(actual.index(), 0);
        assert!(std::ptr::eq(actual.request(), authored));
        assert_eq!(*authored.piece(), id(102));
        assert_eq!(*authored.expected(), old());
        assert_eq!(*authored.companion(), id(5000));
        assert!(std::ptr::eq(
            actual.evidence_source(),
            authored.evidence_source()
        ));
        assert_eq!(*actual.evidence_source(), id(0));
        assert!(std::ptr::eq(actual.original(), old_mate));
        assert!(std::ptr::eq(
            actual.candidate(),
            fields
                .plan()
                .piece(id(102))
                .expect("actual supplied candidate")
        ));
        assert!(std::ptr::eq(
            actual.retired_companion(),
            results
                .source_piece(id(100))
                .expect("actual fully consumed original")
        ));
        assert!(std::ptr::eq(
            actual.original_mirroring(),
            &old_mate.definition().mirroring
        ));
        assert!(std::ptr::eq(
            actual.candidate_mirroring(),
            &actual.candidate().definition().mirroring
        ));
        assert_eq!(
            *actual.candidate_mirroring(),
            Mirroring::PairMember {
                handedness: Handedness::Right,
                companion: id(5000)
            }
        );
        assert!(std::ptr::eq(
            bound
                .repair(id(102))
                .expect("actual original-id query")
                .request(),
            authored
        ));
        assert!(bound.repair(id(100)).is_none());
        assert_eq!(
            bound.repairs().rev().map(|r| r.index()).collect::<Vec<_>>(),
            [0]
        );
        assert_eq!(raw, snapshot);
    });
}
#[test]
fn complete_raw_resources_precede_operation_and_first_guarded_source_field() {
    with_companion_fields(mode(), true, |fields| {
        let raw = JoinCompanionRepairManifest::new(
            id(9999),
            vec![JoinCompanionRepairRequest::new(
                id(999),
                Mirroring::Single,
                id(999),
                id(0),
            )],
        );
        use JoinCompanionRepairRefusal as R;
        for (limits, cause) in [
            (
                JoinCompanionRepairLimits::new(0, 0),
                R::Repairs {
                    required: 1,
                    allowed: 0,
                },
            ),
            (
                JoinCompanionRepairLimits::new(1, 1),
                R::Workspace {
                    required: 2,
                    allowed: 1,
                },
            ),
            (JoinCompanionRepairLimits::new(1, 2), R::Operation),
        ] {
            let error = raw
                .admit(&fields, limits)
                .expect_err("whole resources before operation/source fields");
            assert_eq!(error.refusal(), cause);
            assert_eq!(error.limits(), limits);
            assert!(std::ptr::eq(error.manifest(), &raw));
            assert!(std::ptr::eq(error.fields(), &fields));
            assert!(error.refused_request().is_none());
            assert!(error.original_piece().is_none());
            assert!(error.candidate_piece().is_none());
        }
    });
}
#[test]
fn first_original_duplicate_missing_expected_kind_retention_and_new_target_guards_keep_actual_loans(
) {
    with_companion_fields(mode(), true, |fields| {
        use JoinCompanionRepairRefusal as R;
        for (rows, cause, index) in [
            (
                vec![request(), request()],
                R::Duplicate {
                    first: 0,
                    second: 1,
                },
                1,
            ),
            (
                vec![JoinCompanionRepairRequest::new(
                    id(999),
                    old(),
                    id(5000),
                    id(0),
                )],
                R::MissingSource { index: 0 },
                0,
            ),
            (
                vec![JoinCompanionRepairRequest::new(
                    id(102),
                    Mirroring::Single,
                    id(5000),
                    id(0),
                )],
                R::SourceField { index: 0 },
                0,
            ),
            (
                vec![JoinCompanionRepairRequest::new(
                    id(101),
                    Mirroring::Single,
                    id(5000),
                    id(0),
                )],
                R::NotPairMember { index: 0 },
                0,
            ),
            (
                vec![JoinCompanionRepairRequest::new(
                    id(100),
                    mode(),
                    id(5000),
                    id(0),
                )],
                R::SourceNotRetained { index: 0 },
                0,
            ),
            (
                vec![JoinCompanionRepairRequest::new(
                    id(102),
                    old(),
                    id(101),
                    id(0),
                )],
                R::ResultIdentity { index: 0 },
                0,
            ),
        ] {
            let raw = JoinCompanionRepairManifest::new(id(1000), rows);
            let snapshot = raw.clone();
            let error = raw
                .admit(&fields, JoinCompanionRepairLimits::new(2, 4))
                .expect_err("guarded narrow reference replacement only");
            assert_eq!(error.refusal(), cause);
            let original = raw.requests().get(index).expect("original refused request");
            assert!(std::ptr::eq(
                error.refused_request().expect("actual raw request"),
                original
            ));
            let results = fields.plan().correspondence().sources().results();
            if matches!(cause, R::MissingSource { .. }) {
                assert!(error.original_piece().is_none());
                assert!(error.candidate_piece().is_none());
            } else {
                assert!(std::ptr::eq(
                    error.original_piece().expect("actual original source"),
                    results
                        .source_piece(*original.piece())
                        .expect("actual full original member")
                ));
                if let Some(actual) = error.candidate_piece() {
                    assert!(std::ptr::eq(
                        actual,
                        fields
                            .plan()
                            .piece(*original.piece())
                            .expect("actual supplied candidate")
                    ));
                }
            }
            assert_eq!(raw, snapshot);
        }
    });
}
#[test]
fn requested_result_pairing_and_candidate_hand_cannot_be_inferred_and_empty_requests_approve_no_changes(
) {
    with_companion_fields(Mirroring::Single, true, |fields| {
        let raw = JoinCompanionRepairManifest::new(id(1000), vec![request()]);
        let error = raw
            .admit(&fields, JoinCompanionRepairLimits::new(1, 2))
            .expect_err(
                "candidate reciprocity alone cannot replace explicit result population intent",
            );
        assert_eq!(
            error.refusal(),
            JoinCompanionRepairRefusal::ResultPairing { index: 0 }
        );
    });
    with_companion_fields(mode(), false, |fields| {
        let raw = JoinCompanionRepairManifest::new(id(1000), vec![request()]);
        let error = raw
            .admit(&fields, JoinCompanionRepairLimits::new(1, 2))
            .expect_err("candidate member cannot change its original hand");
        assert_eq!(
            error.refusal(),
            JoinCompanionRepairRefusal::CandidateField { index: 0 }
        );
    });
    with_companion_fields(mode(), true, |fields| {
        let raw = JoinCompanionRepairManifest::new(id(1000), vec![]);
        let bound = raw
            .admit(&fields, JoinCompanionRepairLimits::new(0, 0))
            .expect("explicit no exceptions requested;original field comparison remains next");
        assert_eq!(bound.repair_count(), 0);
        assert_ne!(
            fields
                .plan()
                .piece(id(102))
                .expect("actual changed member")
                .definition()
                .mirroring,
            old()
        );
    });
    with_bound_plan(|plan| {
        let fields = plan
            .admit_piece_fields(JoinInventoryFieldLimits::new(3, 2, 0, 24, 114))
            .expect("whole baseline fields");
        let raw = JoinCompanionRepairManifest::new(id(1000), vec![]);
        assert_eq!(
            raw.admit(&fields, JoinCompanionRepairLimits::new(0, 0))
                .expect("no exception declarations needed")
                .repair_count(),
            0
        );
    });
}
