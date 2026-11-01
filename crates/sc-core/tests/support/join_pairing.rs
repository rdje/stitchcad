//! Actual complete source/result action fixtures for exact native pairing.
#[path = "join_copies.rs"]
pub mod copies;
pub use copies::id;
use sc_core::construction::*;
pub fn row(
    results: &DeclaredJoinResultCopies<'_>,
    source: u128,
    target: Option<u128>,
) -> JoinCopySourceAssignment {
    let intent = target.map_or(JoinCopySourceIntent::RetainOriginal, |t| {
        JoinCopySourceIntent::JoinTo { result_copy: id(t) }
    });
    JoinCopySourceAssignment::new(
        *results
            .source_copy(id(source))
            .expect("actual original copy")
            .definition(),
        intent,
    )
}
pub fn base(results: &DeclaredJoinResultCopies<'_>) -> Vec<JoinCopySourceAssignment> {
    let mut r = vec![
        row(results, 202, None),
        row(results, 200, Some(6000)),
        row(results, 201, Some(6000)),
    ];
    if results.source_copy(id(203)).is_some() {
        r.extend([row(results, 203, Some(6001)), row(results, 204, Some(6001))]);
    }
    r
}
pub fn admit<'a>(
    raw: &'a JoinCopySourceManifest,
    results: &'a DeclaredJoinResultCopies<'a>,
) -> DeclaredJoinCopySources<'a> {
    let c = results.source_copy_count();
    raw.admit(results, JoinCopySourceLimits::new(c, c, 2 * c))
        .expect("complete guarded actual original actions")
}

pub fn with_correspondence(run: impl FnOnce(RequiredJoinCopyCorrespondence<'_>)) {
    copies::with_paired_results(|results| {
        let raw = JoinCopySourceManifest::new(id(1000), base(&results));
        let sources = admit(&raw, &results);
        let pairs = sources
            .require_pairs(JoinCopyPairLimits::new(5, 2, 6))
            .expect("complete explicit native pairs");
        run(pairs);
    });
}

pub fn with_companion_correspondence(
    mode: sc_core::ontology::Mirroring,
    run: impl FnOnce(RequiredJoinCopyCorrespondence<'_>),
) {
    copies::with_companion_results(mode, |results| {
        let raw = JoinCopySourceManifest::new(id(1000), base(&results));
        let sources = admit(&raw, &results);
        let pairs = sources
            .require_pairs(JoinCopyPairLimits::new(3, 1, 3))
            .expect("selected sources consume one actual pair;mate copy retained");
        run(pairs);
    });
}
