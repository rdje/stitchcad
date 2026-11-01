//! Actual whole source partitions and explicit interleaved proposed occurrence fixtures.
#[path = "join_edges.rs"]
pub mod edges;
pub use edges::id;
use edges::with_partitioned_sources;
use sc_core::{construction::*, ontology::*};
pub fn groups() -> Vec<JoinConstructionMappingGroup> {
    use SeamSideId::{A, B};
    [
        (A, 0, [4, 5]),
        (A, 1, [5, 4]),
        (B, 0, [11, 12]),
        (B, 1, [12, 11]),
    ]
    .into_iter()
    .map(|(s, i, t)| {
        JoinConstructionMappingGroup::new(s, i, t.into_iter().map(LocalTag::new).collect())
    })
    .collect()
}
pub fn proposed() -> Vec<JoinConstructionUse> {
    use Direction::{Original as O, Reversed as R};
    [
        (11, O),
        (5, R),
        (4, R),
        (12, O),
        (4, O),
        (11, R),
        (5, O),
        (12, R),
    ]
    .into_iter()
    .map(|(t, d)| JoinConstructionUse::new(LocalTag::new(t), d))
    .collect()
}
pub fn row(s: SeamSideId, i: usize, u: Vec<usize>) -> JoinConstructionTargetAssignment {
    JoinConstructionTargetAssignment::new(s, i, u)
}
pub fn rows() -> Vec<JoinConstructionTargetAssignment> {
    use SeamSideId::{A, B};
    vec![
        row(A, 0, vec![4, 1]),
        row(A, 1, vec![6, 2]),
        row(B, 0, vec![5, 3]),
        row(B, 1, vec![7, 0]),
    ]
}
pub fn with_coverage(
    uses: Vec<JoinConstructionUse>,
    run: impl FnOnce(CoveredJoinConstructionGroups<'_>),
) {
    with_partitioned_sources(uses, |sources| {
        let raw = JoinConstructionGroupManifest::new(id(1000), groups());
        let groups = raw
            .admit(&sources, JoinConstructionGroupLimits::new(4, 4, 8, 16))
            .expect("all authentic retained source groups");
        let covered = groups
            .require_source_coverage(JoinConstructionCoverageLimits::new(4, 8))
            .expect("all original whole partitions");
        run(covered);
    });
}
