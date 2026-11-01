//! Actual original source/role/identity parents for join Edge contract checks.
#[path = "join_banks.rs"]
mod banks;
use banks::with_custom_banks;
pub use banks::{id, BankFixture, SpanFixture};
use sc_core::{construction::*, name::MachineToken, ontology::*};
pub fn with_identity(options: BankFixture, run: impl FnOnce(DeclaredJoinPieceIdentity<'_>)) {
    with_custom_banks(options, |banks| {
        let tags = [9, 2, 4, 5, 6, 8, 10, 11, 12, 13, 7];
        let raw = EdgeOutputRoleCatalog::new(
            id(1000),
            tags.into_iter()
                .map(|t| {
                    EdgeOutputRoleDefinition::new(
                        LocalTag::new(t),
                        MachineToken::new(format!("edge_{t}")).expect("canonical key"),
                    )
                })
                .collect(),
            tags.into_iter()
                .filter(|t| *t != 7)
                .map(LocalTag::new)
                .collect(),
        );
        let b = raw
            .definitions()
            .iter()
            .map(|d| d.key().as_str().len())
            .sum();
        let limits = EdgeOutputRoleCatalogLimits::new(11, 10, b);
        let catalog = raw.admit(limits).expect("whole actual catalogue");
        let roles = banks
            .bind_edge_roles(&catalog, limits)
            .expect("actual owner");
        let field = id(5000);
        let identity = roles
            .declare_piece_identity(
                &field,
                JoinPieceIdentityLimits::new(
                    if matches!(options.span, SpanFixture::RetainedWhole) {
                        2
                    } else {
                        1
                    },
                ),
            )
            .expect("actual local identity");
        run(identity);
    });
}

/// Actual retained-tail cut partitions plus optional complete unrelated/noncut original fields.
pub fn with_cut_coverage(extra: bool, run: impl FnOnce(CoveredJoinCutSources<'_>)) {
    with_custom_cut_coverage(
        BankFixture {
            span: SpanFixture::RetainedTail,
            extra_occurrences: extra,
            ..BankFixture::basic()
        },
        run,
    );
}
/// Actual complete cut partitions under explicitly selected original source variants.
pub fn with_custom_cut_coverage(options: BankFixture, run: impl FnOnce(CoveredJoinCutSources<'_>)) {
    with_identity(options, |identity| {
        let banks = identity.roles().banks();
        let mut ranges = Vec::new();
        for field in banks.sources().occurrences() {
            let cut = matches!(
                field.location(),
                JoinSourceOccurrenceLocation::Boundary(_)
                    | JoinSourceOccurrenceLocation::Hole { .. }
            );
            let bank = banks.bank(field.side());
            if cut && field.location() == bank.occurrence().location() {
                for (from, to) in [
                    (Param::START, bank.range().from()),
                    (bank.range().to(), Param::END),
                ] {
                    if from < to {
                        ranges.push((
                            field,
                            EdgeRange::new(*field.field().edge(), from, to)
                                .expect("positive outside range"),
                        ));
                    }
                }
            } else {
                ranges.push((field, EdgeRange::whole(*field.field().edge())));
            }
        }
        let tags = [9, 2, 4, 5, 6, 8, 10, 11, 12, 13];
        assert!(ranges.len() <= tags.len());
        let raw = JoinEdgeManifest::new(
            id(1000),
            ranges
                .into_iter()
                .zip(tags)
                .map(|((field, range), t)| {
                    JoinEdgeCandidate::new(LocalTag::new(t), field.side(), field.location(), range)
                })
                .collect(),
        );
        let n = raw.candidates().len();
        let r = banks.sources().occurrence_count();
        let edges = raw
            .admit(&identity, JoinEdgeManifestLimits::new(n, 2 * n))
            .expect("actual original candidate fields");
        let coverage = edges
            .require_cut_coverage(JoinCutCoverageLimits::new(n, r, n))
            .expect("complete exact surviving cut partitions");
        run(coverage);
    });
}

/// Actual complete directions on every cut/noncut candidate under original cut coverage.
pub fn with_directions(extra: bool, run: impl FnOnce(AdmittedJoinDirections<'_>)) {
    with_custom_directions(
        BankFixture {
            span: SpanFixture::RetainedTail,
            extra_occurrences: extra,
            ..BankFixture::basic()
        },
        run,
    );
}
/// Actual complete source-relative directions under explicit original source variants.
pub fn with_custom_directions(options: BankFixture, run: impl FnOnce(AdmittedJoinDirections<'_>)) {
    with_custom_cut_coverage(options, |coverage| {
        let n = coverage.edges().edge_count();
        let raw = JoinDirectionManifest::new(
            id(1000),
            coverage
                .edges()
                .edges()
                .enumerate()
                .map(|(i, e)| {
                    JoinDirectionAssignment::new(
                        e.candidate().tag(),
                        if i.is_multiple_of(2) {
                            JoinParameterDirection::SameAsSource
                        } else {
                            JoinParameterDirection::ReverseSource
                        },
                    )
                })
                .collect(),
        );
        let directions = raw
            .admit(&coverage, JoinDirectionLimits::new(n, n, 2 * n))
            .expect("complete explicit original directions");
        run(directions);
    });
}
