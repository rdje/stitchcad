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
        with_construction_folds(&construction, run);
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

fn with_construction_folds(
    construction: &RequiredJoinConstructionTargets<'_>,
    run: impl FnOnce(DeclaredJoinFoldSources<'_>),
) {
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
}
/// Actual four A/B source copies (one Authored/Reflected each),one unrelated copy and two explicit results.
pub fn with_paired_results(run: impl FnOnce(DeclaredJoinResultCopies<'_>)) {
    fold::with_paired_construction(|construction| {
        with_construction_folds(&construction, |folds| {
            let raw = JoinResultCopyManifest::new(
                id(1000),
                vec![
                    request(6001, CopyOrientation::Reflected),
                    request(6000, CopyOrientation::Authored),
                ],
            );
            let results = raw
                .admit(&folds, JoinResultCopyLimits::new(5, 3, 2, 10))
                .expect("explicit two-copy result under actual whole original inventory");
            run(results);
        });
    });
}

pub fn with_companion_results(mode: Mirroring, run: impl FnOnce(DeclaredJoinResultCopies<'_>)) {
    fold::with_companion_construction(mode, |construction| {
        with_construction_folds(&construction, |folds| {
            let raw = JoinResultCopyManifest::new(
                id(1000),
                vec![request(6000, CopyOrientation::Authored)],
            );
            let results = raw
                .admit(&folds, JoinResultCopyLimits::new(3, 3, 1, 7))
                .expect("explicit cut-once result/member request");
            run(results);
        });
    });
}
/// Two authored copies of each original source;only the selected pair is consumed later.
pub fn with_partial_results(run: impl FnOnce(DeclaredJoinResultCopies<'_>)) {
    fold::with_partial_construction(|construction| {
        with_construction_folds(&construction, |folds| {
            let raw = JoinResultCopyManifest::new(
                id(1000),
                vec![request(6000, CopyOrientation::Authored)],
            );
            let results = raw
                .admit(&folds, JoinResultCopyLimits::new(5, 3, 1, 9))
                .expect("one requested result;additional source copies remain explicit");
            run(results);
        });
    });
}
