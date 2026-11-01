//! Explicit result fold state and actual outer cut attribution,not original fold loss/physical truth.
#![allow(clippy::expect_used)]
#[allow(dead_code)]
#[path = "support/join_fold.rs"]
mod common;
use common::{id, with_construction};
use sc_core::{construction::*, ontology::*};
#[test]
fn explicit_result_fold_choices_lend_actual_request_boundary_and_unissued_edge_tuple() {
    with_construction(true, |construction| {
        for intent in [
            JoinFoldIntent::NoFold,
            JoinFoldIntent::CutOnFold {
                tag: LocalTag::new(9),
            },
            JoinFoldIntent::CutOnFold {
                tag: LocalTag::new(8),
            },
        ] {
            let raw = JoinFoldRequirement::new(id(1000), intent, id(0));
            let snapshot = raw;
            let limits = JoinFoldLimits::new(10, 10, 4);
            let fold = construction
                .require_fold(&raw, limits)
                .expect("explicit actual outer fold or NoFold request");
            assert!(std::ptr::eq(fold.construction(), &construction));
            assert!(std::ptr::eq(fold.requirement(), &raw));
            assert!(std::ptr::eq(fold.requirement().intent(), raw.intent()));
            assert!(std::ptr::eq(
                fold.requirement().correspondence(),
                raw.correspondence()
            ));
            assert_eq!(*raw.correspondence(), id(0));
            assert_eq!(raw.operation(), id(1000));
            assert_eq!(fold.limits(), limits);
            assert_eq!(
                (limits.sources(), limits.outputs(), limits.cut_uses()),
                (10, 10, 4)
            );
            let contours = construction
                .targets()
                .coverage()
                .groups()
                .sources()
                .uses()
                .connectivity()
                .contours();
            match intent {
                JoinFoldIntent::NoFold => {
                    assert!(!fold.cut_on_fold());
                    assert!(fold.boundary().is_none());
                    assert!(fold.edge().is_none());
                }
                JoinFoldIntent::CutOnFold { tag } => {
                    assert!(fold.cut_on_fold());
                    let actual = fold.boundary().expect("actual original boundary use");
                    assert!(matches!(
                        actual.location(),
                        JoinCutContourUseLocation::Boundary(_)
                    ));
                    assert!(std::ptr::eq(
                        actual.declared(),
                        contours.use_for(tag).expect("actual cut use").declared()
                    ));
                    assert_eq!(fold.edge(), Some(EdgeRef::new(id(1000), tag)));
                    assert!(
                        actual.output().source().source().from() > Param::START
                            || actual.output().source().source().to() < Param::END
                    ); // Subrange is not a whole-fold certificate.
                }
            }
            assert_eq!(raw, snapshot);
        }
    });
}
#[test]
fn whole_source_candidate_and_cut_use_quotas_precede_operation_and_fold_tag() {
    with_construction(true, |construction| {
        let raw = JoinFoldRequirement::new(
            id(9999),
            JoinFoldIntent::CutOnFold {
                tag: LocalTag::new(99),
            },
            id(0),
        );
        use JoinFoldRefusal as R;
        for (limits, cause) in [
            (
                JoinFoldLimits::new(9, 0, 0),
                R::Sources {
                    required: 10,
                    allowed: 9,
                },
            ),
            (
                JoinFoldLimits::new(10, 9, 0),
                R::Outputs {
                    required: 10,
                    allowed: 9,
                },
            ),
            (
                JoinFoldLimits::new(10, 10, 3),
                R::CutUses {
                    required: 4,
                    allowed: 3,
                },
            ),
            (JoinFoldLimits::new(10, 10, 4), R::Operation),
        ] {
            let error = construction
                .require_fold(&raw, limits)
                .expect_err("whole resources before operation/tag");
            assert_eq!(error.refusal(), cause);
            assert_eq!(error.limits(), limits);
            assert!(std::ptr::eq(error.construction(), &construction));
            assert!(std::ptr::eq(error.requirement(), &raw));
            assert!(error.refused_tag().is_none());
        }
    });
}
#[test]
fn actual_output_and_outer_membership_refuse_unknown_hole_construction_and_fold_only_roles() {
    with_construction(true, |construction| {
        use JoinFoldRefusal as R;
        for (tag, cause) in [
            (99, R::Output),
            (7, R::Output),
            (2, R::OutputNotBoundary),
            (10, R::OutputNotBoundary),
            (4, R::OutputNotBoundary),
            (6, R::OutputNotBoundary),
        ] {
            let raw = JoinFoldRequirement::new(
                id(1000),
                JoinFoldIntent::CutOnFold {
                    tag: LocalTag::new(tag),
                },
                id(0),
            );
            let error = construction
                .require_fold(&raw, JoinFoldLimits::new(10, 10, 4))
                .expect_err("actual outer use required");
            assert_eq!(error.refusal(), cause);
            let authored = match raw.intent() {
                JoinFoldIntent::CutOnFold { tag } => Some(tag),
                JoinFoldIntent::NoFold => None,
            }
            .expect("actual tag field");
            assert!(std::ptr::eq(
                error.refused_tag().expect("actual refused tag"),
                authored
            ));
        }
    });
}
#[test]
fn explicit_result_state_does_not_infer_original_fold_retention_omission_or_new_axis_permission() {
    for extra in [false, true] {
        with_construction(extra, |construction| {
            let sources = construction
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
                .sources();
            let original_folds = [SeamSideId::A, SeamSideId::B]
                .into_iter()
                .map(|s| sources.piece(s).definition().fold_edges.len())
                .sum::<usize>();
            assert_eq!(original_folds, if extra { 2 } else { 0 });
            for intent in [
                JoinFoldIntent::NoFold,
                JoinFoldIntent::CutOnFold {
                    tag: LocalTag::new(9),
                },
            ] {
                let raw = JoinFoldRequirement::new(id(1000), intent, id(0));
                let n = if extra { 10 } else { 2 };
                let c = if extra { 4 } else { 2 };
                let fold = construction
                    .require_fold(&raw, JoinFoldLimits::new(n, n, c))
                    .expect("explicit desired state only;source fold actions pending");
                assert_eq!(
                    fold.cut_on_fold(),
                    matches!(intent, JoinFoldIntent::CutOnFold { .. })
                );
                assert_eq!(
                    [SeamSideId::A, SeamSideId::B]
                        .into_iter()
                        .map(|s| sources.piece(s).definition().fold_edges.len())
                        .sum::<usize>(),
                    original_folds
                );
            }
        });
    }
}
