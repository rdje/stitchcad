//! Actual complete original inventory/fold context and explicit result-copy request fixtures.
#[path = "join_fold.rs"]
mod fold;
pub use fold::id;
use fold::with_copy_construction;
use sc_core::{construction::*, ontology::*};
pub fn request(bits: u128, orientation: CopyOrientation) -> CutCopyDefinition {
    CutCopyDefinition {
        id: id(bits),
        piece: id(5000),
        orientation,
    }
}
pub fn with_folds(quantity: u32, run: impl FnOnce(DeclaredJoinFoldSources<'_>)) {
    with_copy_construction(quantity, true, |construction| {
        let raw = JoinFoldRequirement::new(id(1000), JoinFoldIntent::NoFold, id(0));
        let fold = construction
            .require_fold(&raw, JoinFoldLimits::new(10, 10, 4))
            .expect("explicit desired NoFold state");
        let sources = fold
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
            .sources();
        let rows = sources
            .occurrences()
            .filter_map(|f| match f.location() {
                JoinSourceOccurrenceLocation::Fold(index) => Some(JoinFoldSourceAssignment::new(
                    f.side(),
                    index,
                    *f.field().edge(),
                    JoinFoldSourceIntent::Omit {
                        evidence_source: id(0),
                    },
                )),
                _ => None,
            })
            .collect();
        let raw = JoinFoldSourceManifest::new(id(1000), rows);
        let declared = raw
            .admit(&fold, JoinFoldSourceLimits::new(10, 2, 4))
            .expect("complete original fold omission requests");
        run(declared);
    });
}

pub fn with_results(run: impl FnOnce(DeclaredJoinResultCopies<'_>)) {
    with_folds(1, |folds| {
        let raw =
            JoinResultCopyManifest::new(id(1000), vec![request(6000, CopyOrientation::Authored)]);
        let results = raw
            .admit(&folds, JoinResultCopyLimits::new(3, 3, 1, 7))
            .expect("explicit whole result-copy requests");
        run(results);
    });
}
