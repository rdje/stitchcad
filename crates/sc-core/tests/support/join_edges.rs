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

/// Actual complete cut-use/path parents for proposed noncut construction declarations.
pub fn with_connectivity(extra: bool, run: impl FnOnce(RequiredJoinCutConnectivity<'_>)) {
    with_directions(extra, |directions| {
        let make = |tag, direction| JoinCutContourUse::new(LocalTag::new(tag), direction);
        let (boundary, holes) = if extra {
            (
                vec![make(9, Direction::Reversed), make(8, Direction::Original)],
                vec![
                    vec![make(10, Direction::Original)],
                    vec![make(2, Direction::Reversed)],
                ],
            )
        } else {
            (
                vec![make(9, Direction::Original), make(2, Direction::Reversed)],
                vec![],
            )
        };
        let h = holes.len();
        let u = boundary.len() + holes.iter().map(Vec::len).sum::<usize>();
        let raw = JoinCutContourManifest::new(id(1000), boundary, holes);
        let n = directions.coverage().edges().edge_count();
        let contours = raw
            .admit(&directions, JoinCutContourLimits::new(n, h, u, 2 * u + h))
            .expect("actual complete cut-use declarations");
        let path = contours
            .require_cut_connectivity(JoinCutConnectivityLimits::new(h, u))
            .expect("required original cut-source path");
        run(path);
    });
}

/// Explicit empty proposed construction data,with complete actual source/cut-path parents.
pub fn with_construction_uses(extra: bool, run: impl FnOnce(DeclaredJoinConstructionUses<'_>)) {
    with_connectivity(extra, |path| {
        let raw = JoinConstructionManifest::new(id(1000), vec![]);
        let n = path.contours().directions().coverage().edges().edge_count();
        let uses = raw
            .admit(&path, JoinConstructionUseLimits::new(n, 0, 0))
            .expect("explicit proposed construction data");
        run(uses);
    });
}

/// Complete original field actions with an explicit per-snapshot-field intent factory.
pub fn with_source_actions(
    extra: bool,
    intent: impl Fn(SeamSideId, usize) -> JoinConstructionSourceIntent,
    run: impl FnOnce(DeclaredJoinConstructionSources<'_>),
) {
    with_construction_uses(extra, |uses| {
        let sources = uses
            .connectivity()
            .contours()
            .directions()
            .coverage()
            .edges()
            .identity()
            .roles()
            .banks()
            .sources();
        let mut assignments = Vec::new();
        for side in [SeamSideId::A, SeamSideId::B] {
            for (index, field) in sources
                .piece(side)
                .definition()
                .construction_lines
                .iter()
                .enumerate()
            {
                assignments.push(JoinConstructionSourceAssignment::new(
                    side,
                    index,
                    *field,
                    intent(side, index),
                ));
            }
        }
        let m = assignments.len();
        let raw = JoinConstructionSourceManifest::new(id(1000), assignments);
        let actions = raw
            .admit(
                &uses,
                JoinConstructionSourceLimits::new(sources.occurrence_count(), m, 2 * m),
            )
            .expect("complete explicit original source actions");
        run(actions);
    });
}
