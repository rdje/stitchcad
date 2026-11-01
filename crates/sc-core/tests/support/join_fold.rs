//! Actual complete construction/cut/source parents with explicit original construction Omit requests.
#[path = "join_edges.rs"]
pub mod edges;
pub use edges::id;
use sc_core::construction::*;
pub fn with_construction(extra: bool, run: impl FnOnce(RequiredJoinConstructionTargets<'_>)) {
    edges::with_source_actions(
        extra,
        |_, _| JoinConstructionSourceIntent::Omit {
            evidence_source: id(0),
        },
        |sources| {
            with_source_construction(&sources, run);
        },
    );
}

fn with_source_construction(
    sources: &DeclaredJoinConstructionSources<'_>,
    run: impl FnOnce(RequiredJoinConstructionTargets<'_>),
) {
    let k = sources.source_count();
    let group_raw = JoinConstructionGroupManifest::new(id(1000), vec![]);
    let groups = group_raw
        .admit(sources, JoinConstructionGroupLimits::new(k, 0, 0, 0))
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
}
/// Actual two-field outer loops with the original fold on the added boundary edge,not the bank.
/// Shared=true keeps distinct A/B Pieces but byte-equal old geometry,including fold ids.
pub fn with_alternative_construction(
    shared: bool,
    run: impl FnOnce(RequiredJoinConstructionTargets<'_>),
) {
    with_alternative_options(
        shared,
        1,
        false,
        ExtraCopies::None,
        false,
        sc_core::ontology::Mirroring::Single,
        run,
    );
}
/// Actual complete source inventory plus an explicit result quantity request;no pairing inferred.
pub fn with_copy_construction(
    quantity: u32,
    unrelated: bool,
    run: impl FnOnce(RequiredJoinConstructionTargets<'_>),
) {
    with_alternative_options(
        false,
        quantity,
        unrelated,
        ExtraCopies::None,
        false,
        sc_core::ontology::Mirroring::Single,
        run,
    );
}
pub fn with_paired_construction(run: impl FnOnce(RequiredJoinConstructionTargets<'_>)) {
    with_alternative_options(
        false,
        2,
        true,
        ExtraCopies::Mirrored,
        false,
        sc_core::ontology::Mirroring::MirroredPairs,
        run,
    );
}
pub fn with_companion_construction(
    mode: sc_core::ontology::Mirroring,
    run: impl FnOnce(RequiredJoinConstructionTargets<'_>),
) {
    with_alternative_options(false, 1, true, ExtraCopies::None, true, mode, run);
}
pub fn with_partial_construction(run: impl FnOnce(RequiredJoinConstructionTargets<'_>)) {
    with_alternative_options(
        false,
        1,
        true,
        ExtraCopies::Authored,
        false,
        sc_core::ontology::Mirroring::Single,
        run,
    );
}
enum ExtraCopies {
    None,
    Mirrored,
    Authored,
}
fn with_alternative_options(
    shared: bool,
    quantity: u32,
    unrelated: bool,
    additional: ExtraCopies,
    companion: bool,
    mode: sc_core::ontology::Mirroring,
    run: impl FnOnce(RequiredJoinConstructionTargets<'_>),
) {
    use sc_core::ontology::*;
    edges::with_custom_directions(
        edges::BankFixture {
            span: edges::SpanFixture::RetainedTail,
            extra_occurrences: true,
            fold_on_other_boundary: true,
            shared_edge_fields: shared,
            result_quantity: quantity,
            extra_piece: unrelated,
            additional_source_pair: matches!(additional, ExtraCopies::Mirrored),
            additional_authored_copies: matches!(additional, ExtraCopies::Authored),
            source_companion: companion,
            mirroring: mode,
            ..edges::BankFixture::basic()
        },
        |directions| {
            let make = |t, d| JoinCutContourUse::new(LocalTag::new(t), d);
            let raw = JoinCutContourManifest::new(
                id(1000),
                vec![
                    make(9, Direction::Original),
                    make(8, Direction::Reversed),
                    make(10, Direction::Reversed),
                    make(2, Direction::Original),
                ],
                vec![],
            );
            let contours = raw
                .admit(&directions, JoinCutContourLimits::new(10, 0, 4, 8))
                .expect("actual four-field cut boundary");
            let path = contours
                .require_cut_connectivity(JoinCutConnectivityLimits::new(0, 4))
                .expect("actual original source gluing");
            let proposed = JoinConstructionManifest::new(id(1000), vec![]);
            let uses = proposed
                .admit(&path, JoinConstructionUseLimits::new(10, 0, 0))
                .expect("explicit empty construction proposal");
            edges::with_use_sources(
                &uses,
                |_, _| JoinConstructionSourceIntent::Omit {
                    evidence_source: id(0),
                },
                |sources| with_source_construction(&sources, run),
            );
        },
    );
}
