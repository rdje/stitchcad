//! Explicit source fold state and complete actual outer attribution;no geometric fold proof.
#![allow(clippy::expect_used)]
#[path = "support/truing_contours.rs"]
mod common;
use common::{
    piece::common::{domain::roles::copies::id, with_outside},
    with_rich_contours, FoldFixture,
};
use sc_core::{construction::*, ontology::*};
fn request(intent: TruingFoldIntent) -> TruingFoldRequirement {
    TruingFoldRequirement::new(intent, id(7001))
}
fn refused(contours: &TruingPieceContours<'_>, intent: TruingFoldIntent, cause: TruingFoldRefusal) {
    let request = request(intent);
    let limits = TruingFoldLimits::new(4, 16);
    let error = contours
        .require_fold(&request, limits)
        .expect_err("original-state/attribution cause");
    assert!(std::ptr::eq(error.contours(), contours));
    assert!(std::ptr::eq(error.requirement(), &request));
    assert_eq!(error.limits(), limits);
    assert_eq!(error.refusal(), cause);
    assert!(error.to_string().contains("truing fold requirement"));
}
#[test]
fn explicit_retained_or_derived_fold_keeps_actual_original_fields_and_requires_named_evidence() {
    use TruingFoldIntent as F;
    use TruingFoldRefusal as R;
    for side in [SeamSideId::A, SeamSideId::B] {
        for intent in [
            TruingOutcomeIntent::PreserveSource,
            TruingOutcomeIntent::Refit,
        ] {
            with_rich_contours(side, intent, FoldFixture::TargetBoundary, |contours| {
                let snapshot = contours.source().piece().definition().clone();
                let choices = if intent == TruingOutcomeIntent::Refit {
                    vec![
                        F::Derived {
                            tag: LocalTag::new(2),
                        },
                        F::Derived {
                            tag: LocalTag::new(9),
                        },
                        F::Derived {
                            tag: LocalTag::new(11),
                        },
                        F::Derived {
                            tag: LocalTag::new(5),
                        },
                    ]
                } else {
                    vec![F::RetainOriginal]
                };
                for choice in choices {
                    let request = request(choice);
                    let limits = TruingFoldLimits::new(4, 16);
                    let fold = contours
                        .require_fold(&request, limits)
                        .expect("explicit actual outer fold attribution");
                    assert!(std::ptr::eq(fold.contours(), &contours));
                    assert!(std::ptr::eq(fold.requirement(), &request));
                    assert_eq!(fold.limits(), limits);
                    assert_eq!((limits.outputs(), limits.occurrences()), (4, 16));
                    assert_eq!(*request.intent(), choice);
                    assert_eq!(*request.correspondence(), id(7001));
                    assert!(std::ptr::eq(fold.source_piece(), contours.source().piece()));
                    assert!(std::ptr::eq(
                        fold.source_fold().expect("actual source fold"),
                        contours
                            .source()
                            .piece()
                            .definition()
                            .fold_edges
                            .first()
                            .expect("original field")
                    ));
                    if let F::Derived { tag } = choice {
                        let output = fold.output().expect("actual admitted derived output");
                        assert!(std::ptr::eq(output.context(), contours.directions()));
                        assert_eq!(*output.assignment().tag(), tag);
                        assert_eq!(fold.edge(), Some(EdgeRef::new(id(1000), tag)));
                        assert_eq!(
                            output.source().range().edge(),
                            *fold.source_fold().expect("source fold")
                        );
                    } else {
                        assert!(fold.output().is_none());
                        assert_eq!(fold.edge(), fold.source_fold().copied());
                    }
                    assert!(format!("{fold:?}").contains("RequiredTruingFold"));
                }
                refused(&contours, F::NoFold, R::FoldStatus);
                refused(
                    &contours,
                    F::Derived {
                        tag: LocalTag::new(7),
                    },
                    R::Output,
                );
                if intent == TruingOutcomeIntent::Refit {
                    refused(&contours, F::RetainOriginal, R::FoldNotRetained);
                } else {
                    refused(
                        &contours,
                        F::Derived {
                            tag: LocalTag::new(9),
                        },
                        R::Output,
                    );
                }
                assert_eq!(contours.source().piece().definition(), &snapshot);
            });
        }
    }
}
#[test]
fn actual_outer_use_and_original_fold_provenance_are_independent_requirements() {
    use TruingFoldIntent as F;
    use TruingFoldRefusal as R;
    for side in [SeamSideId::A, SeamSideId::B] {
        for (fixture, cause) in [
            (FoldFixture::UnrelatedBoundary, R::OutputNotFoldSource),
            (FoldFixture::TargetHole, R::OutputNotBoundary),
        ] {
            with_rich_contours(side, TruingOutcomeIntent::Refit, fixture, |contours| {
                let request = request(F::RetainOriginal);
                let retained = contours
                    .require_fold(&request, TruingFoldLimits::new(4, 16))
                    .expect("actual unrelated fold retained on outer contour");
                assert!(retained.output().is_none());
                assert_eq!(retained.edge(), retained.source_fold().copied());
                refused(
                    &contours,
                    F::Derived {
                        tag: LocalTag::new(9),
                    },
                    cause,
                );
            });
        }
    }
}
#[test]
fn no_fold_is_explicit_and_complete_resource_quotas_precede_original_state_and_unknown_tag() {
    use TruingFoldIntent as F;
    use TruingFoldRefusal as R;
    with_rich_contours(
        SeamSideId::B,
        TruingOutcomeIntent::Refit,
        FoldFixture::NoFold,
        |contours| {
            let original = contours.source().piece().definition().clone();
            let request = request(F::NoFold);
            let fold = contours
                .require_fold(&request, TruingFoldLimits::new(4, 16))
                .expect("original source has no fold");
            assert!(fold.source_fold().is_none());
            assert!(fold.output().is_none());
            assert!(fold.edge().is_none());
            let wrong = TruingFoldRequirement::new(
                F::Derived {
                    tag: LocalTag::new(99),
                },
                id(0),
            );
            for (limits, cause) in [
                (
                    TruingFoldLimits::new(3, 15),
                    R::Outputs {
                        required: 4,
                        allowed: 3,
                    },
                ),
                (
                    TruingFoldLimits::new(4, 15),
                    R::Occurrences {
                        required: 16,
                        allowed: 15,
                    },
                ),
            ] {
                let error = contours
                    .require_fold(&wrong, limits)
                    .expect_err("whole count before state/tag");
                assert_eq!(error.refusal(), cause);
                assert_eq!(error.limits(), limits);
                assert!(std::ptr::eq(error.requirement(), &wrong));
                assert!(std::ptr::eq(error.contours(), &contours));
            }
            refused(&contours, F::RetainOriginal, R::FoldStatus);
            refused(
                &contours,
                F::Derived {
                    tag: LocalTag::new(99),
                },
                R::FoldStatus,
            );
            assert_eq!(contours.source().piece().definition(), &original);
        },
    );
    with_outside(
        SeamSideId::A,
        Param::START,
        Param::END,
        TruingOutcomeIntent::PreserveSource,
        TruingOutsideRoles::new(TruingOutsideIntent::Absent, TruingOutsideIntent::Absent),
        |outside| {
            let raw = TruingDirectionManifest::new(id(1000), vec![]);
            let directions = raw
                .admit(&outside, TruingDirectionLimits::new(0, 0, 0))
                .expect("preserve");
            let source = directions
                .admit_exact_piece_source(TruingPieceSourceLimits::new(0, 0, 1))
                .expect("original");
            let contours = source
                .declare_contours(TruingContourLimits::new(0, 0, 1, 1, 1))
                .expect("unchanged contours");
            let request = request(F::NoFold);
            let fold = contours
                .require_fold(&request, TruingFoldLimits::new(0, 1))
                .expect("explicit no-fold source");
            assert!(fold.edge().is_none());
            refused(&contours, F::RetainOriginal, R::FoldStatus);
        },
    );
}
