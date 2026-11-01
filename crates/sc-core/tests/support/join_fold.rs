//! Actual complete construction/cut/source parents with explicit original construction Omit requests.
#[path = "join_edges.rs"]
mod edges;
pub use edges::id;
use sc_core::construction::*;
pub fn with_construction(extra: bool, run: impl FnOnce(RequiredJoinConstructionTargets<'_>)) {
    edges::with_source_actions(
        extra,
        |_, _| JoinConstructionSourceIntent::Omit {
            evidence_source: id(0),
        },
        |sources| {
            let k = sources.source_count();
            let group_raw = JoinConstructionGroupManifest::new(id(1000), vec![]);
            let groups = group_raw
                .admit(&sources, JoinConstructionGroupLimits::new(k, 0, 0, 0))
                .expect("explicit no-Retain construction scope");
            let coverage = groups
                .require_source_coverage(JoinConstructionCoverageLimits::new(0, 0))
                .expect("vacuous construction source partitions");
            let target_raw = JoinConstructionTargetManifest::new(
                id(1000),
                vec![],
                JoinAdditionalConstructionIntent::SourceMappedOnly,
            );
            let targets = target_raw
                .admit(&coverage, JoinConstructionTargetLimits::new(0, 0, 0, 0))
                .expect("explicit no proposed construction occurrences");
            let required = targets
                .require_target_coverage(JoinConstructionTargetCoverageLimits::new(0, 0))
                .expect("whole declared construction scope;omission justification outstanding");
            run(required);
        },
    );
}
