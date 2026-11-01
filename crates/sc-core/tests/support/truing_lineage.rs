//! Actual lineage/fold/contour/source parents for bounded curve-proposal contracts.
#[path = "truing_contours.rs"]
pub mod contours;
use contours::{piece::common::domain::roles::copies::id, with_rich_contours, FoldFixture};
use sc_core::{construction::*, ontology::*};
pub fn with_lineage(
    side: SeamSideId,
    intent: TruingOutcomeIntent,
    variant: FoldFixture,
    run: impl FnOnce(RequiredTruingLineage<'_>),
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
        let fold = contours
            .require_fold(&request, TruingFoldLimits::new(4, 16))
            .expect("explicit original fold state");
        let identity = if intent == TruingOutcomeIntent::Refit {
            TruingSourceIdentityIntent::RetireOriginal
        } else {
            TruingSourceIdentityIntent::RetainOriginal
        };
        let lineage = fold
            .require_lineage(&identity, TruingLineageLimits::new(4, 16))
            .expect("complete original lineage request");
        run(lineage);
    });
}
