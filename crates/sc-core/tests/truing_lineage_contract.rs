//! Whole held-edge disposition and actual evidence sources;no history/currentness certificate.
#![allow(clippy::expect_used)]
#[path = "support/truing_contours.rs"]
mod common;
use common::{
    piece::common::{domain::roles::copies::id, with_outside},
    with_rich_contours, FoldFixture,
};
use sc_core::{construction::*, ontology::*};
fn with_fold(
    side: SeamSideId,
    intent: TruingOutcomeIntent,
    variant: FoldFixture,
    run: impl FnOnce(RequiredTruingFold<'_>),
) {
    with_rich_contours(side, intent, variant, |contours| {
        let choice = if matches!(variant, FoldFixture::NoFold) {
            TruingFoldIntent::NoFold
        } else if intent == TruingOutcomeIntent::Refit
            && matches!(variant, FoldFixture::TargetBoundary)
        {
            TruingFoldIntent::Derived {
                tag: LocalTag::new(9),
            }
        } else {
            TruingFoldIntent::RetainOriginal
        };
        let request = TruingFoldRequirement::new(choice, id(7001));
        run(contours
            .require_fold(&request, TruingFoldLimits::new(4, 16))
            .expect("explicit whole outer fold request"));
    });
}
#[test]
fn explicit_disposition_reuses_whole_original_piece_and_actual_history_and_visible_repair_sources()
{
    use TruingSourceIdentityIntent as I;
    for side in [SeamSideId::A, SeamSideId::B] {
        for intent in [
            TruingOutcomeIntent::PreserveSource,
            TruingOutcomeIntent::Refit,
        ] {
            for variant in [
                FoldFixture::TargetBoundary,
                FoldFixture::UnrelatedBoundary,
                FoldFixture::TargetHole,
                FoldFixture::NoFold,
            ] {
                with_fold(side, intent, variant, |fold| {
                    let snapshot = fold.source_piece().definition().clone();
                    for choice in [I::RetainOriginal, I::RetireOriginal] {
                        if intent == TruingOutcomeIntent::PreserveSource
                            && choice == I::RetireOriginal
                        {
                            continue;
                        }
                        let limits = TruingLineageLimits::new(4, 16);
                        let lineage = fold
                            .require_lineage(&choice, limits)
                            .expect("explicit locally consistent requirement");
                        assert!(std::ptr::eq(lineage.fold(), &fold));
                        assert!(std::ptr::eq(lineage.intent(), &choice));
                        assert_eq!(lineage.limits(), limits);
                        assert_eq!((limits.outputs(), limits.occurrences()), (4, 16));
                        assert_eq!(lineage.source_edge(), fold.contours().source().target());
                        assert!(std::ptr::eq(
                            lineage.piece_id(),
                            &fold.source_piece().definition().id
                        ));
                        let evidence = fold.contours().directions().outside().evidence();
                        assert!(std::ptr::eq(lineage.evidence(), evidence));
                        assert!(std::ptr::eq(
                            lineage.history_source(),
                            evidence
                                .policy()
                                .source(TruingEvidenceKind::IdentityHistory)
                        ));
                        assert!(std::ptr::eq(
                            lineage.repair_source(),
                            evidence.policy().source(TruingEvidenceKind::VisibleRepairs)
                        ));
                        assert!(!std::ptr::eq(
                            lineage.history_source(),
                            lineage.repair_source()
                        ));
                        let expected = fold
                            .contours()
                            .directions()
                            .assignments()
                            .map(|o| *o.assignment().tag())
                            .collect::<Vec<_>>();
                        assert_eq!(
                            lineage
                                .outputs()
                                .map(|o| *o.assignment().tag())
                                .collect::<Vec<_>>(),
                            expected
                        );
                        assert_eq!(
                            lineage
                                .outputs()
                                .rev()
                                .map(|o| *o.assignment().tag())
                                .collect::<Vec<_>>(),
                            expected.into_iter().rev().collect::<Vec<_>>()
                        );
                        for output in lineage.outputs() {
                            assert!(std::ptr::eq(output.context(), fold.contours().directions()));
                            assert_eq!(output.source().range().edge(), lineage.source_edge());
                        }
                        let retained = fold
                            .contours()
                            .occurrences()
                            .filter(|p| p.output().is_none())
                            .map(|p| (p.location(), *p.directed()))
                            .collect::<Vec<_>>();
                        assert_eq!(
                            lineage
                                .retained_contours()
                                .map(|p| (p.location(), *p.directed()))
                                .collect::<Vec<_>>(),
                            retained
                        );
                        assert_eq!(
                            lineage
                                .retained_contours()
                                .rev()
                                .map(|p| (p.location(), *p.directed()))
                                .collect::<Vec<_>>(),
                            retained.into_iter().rev().collect::<Vec<_>>()
                        );
                        assert_eq!(
                            lineage.retained_contours().count(),
                            if intent == TruingOutcomeIntent::Refit {
                                4
                            } else {
                                7
                            }
                        );
                        if intent == TruingOutcomeIntent::Refit {
                            assert!(fold
                                .contours()
                                .occurrences()
                                .all(|p| p.directed().edge != lineage.source_edge()));
                            assert_ne!(fold.edge(), Some(lineage.source_edge()));
                        }
                        assert!(format!("{lineage:?}").contains("RequiredTruingLineage"));
                    }
                    assert_eq!(fold.source_piece().definition(), &snapshot);
                });
            }
        }
    }
}
#[test]
fn whole_counts_precede_retirement_during_preservation_and_errors_keep_exact_original_parents() {
    use TruingLineageRefusal as R;
    use TruingSourceIdentityIntent as I;
    for intent in [
        TruingOutcomeIntent::PreserveSource,
        TruingOutcomeIntent::Refit,
    ] {
        with_fold(SeamSideId::B, intent, FoldFixture::TargetBoundary, |fold| {
            let retire = I::RetireOriginal;
            let cases = if intent == TruingOutcomeIntent::Refit {
                vec![
                    (
                        TruingLineageLimits::new(3, 15),
                        R::Outputs {
                            required: 4,
                            allowed: 3,
                        },
                    ),
                    (
                        TruingLineageLimits::new(4, 15),
                        R::Occurrences {
                            required: 16,
                            allowed: 15,
                        },
                    ),
                ]
            } else {
                vec![
                    (
                        TruingLineageLimits::new(0, 6),
                        R::Occurrences {
                            required: 7,
                            allowed: 6,
                        },
                    ),
                    (TruingLineageLimits::new(0, 7), R::PreserveSource),
                ]
            };
            for (limits, cause) in cases {
                let error = fold
                    .require_lineage(&retire, limits)
                    .expect_err("whole count or explicit outcome contradiction");
                assert!(std::ptr::eq(error.fold(), &fold));
                assert!(std::ptr::eq(error.intent(), &retire));
                assert_eq!(error.limits(), limits);
                assert_eq!(error.refusal(), cause);
                assert!(error.to_string().contains("truing whole-edge lineage"));
            }
        });
    }
}
#[test]
fn empty_new_outputs_still_require_original_identity_and_visible_repair_sources() {
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
                .expect("explicit preservation");
            let source = directions
                .admit_exact_piece_source(TruingPieceSourceLimits::new(0, 0, 1))
                .expect("actual original source");
            let contours = source
                .declare_contours(TruingContourLimits::new(0, 0, 1, 1, 1))
                .expect("original contour");
            let request = TruingFoldRequirement::new(TruingFoldIntent::NoFold, id(7001));
            let fold = contours
                .require_fold(&request, TruingFoldLimits::new(0, 1))
                .expect("actual no-fold state");
            let retain = TruingSourceIdentityIntent::RetainOriginal;
            let lineage = fold
                .require_lineage(&retain, TruingLineageLimits::new(0, 1))
                .expect("explicit retained identity");
            assert_eq!(lineage.outputs().len(), 0);
            assert_eq!(lineage.retained_contours().count(), 1);
            assert_eq!(*lineage.piece_id(), id(100));
            assert_eq!(*lineage.history_source(), id(7000));
            assert_eq!(*lineage.repair_source(), id(7000));
            assert_eq!(lineage.source_edge(), source.target());
        },
    );
}
