//! Desired reflected loop traversal and original construction/fold intent under explicit edge frames.
#![allow(clippy::expect_used)]
#[path = "support/mirror_edges.rs"]
mod common;
use common::metadata::source as sources;
use common::{with_edges, with_edges_metadata};
use sc_core::{construction::*, ontology::*};
fn raw_edges() -> MirrorPieceEdgeManifest {
    MirrorPieceEdgeManifest::new(
        EntityId::from_bits(10),
        [(6, 43), (2, 41), (9, 40), (4, 42)]
            .into_iter()
            .map(|(tag, id)| {
                MirrorPieceEdgeCandidate::new(
                    LocalTag::new(tag),
                    EdgeRef::new(EntityId::from_bits(id), LocalTag::FIRST),
                )
            })
            .collect(),
    )
}
fn policy(direction: Direction) -> MirrorPieceContourPolicy {
    MirrorPieceContourPolicy::new(
        EntityId::from_bits(10),
        [4, 9, 6, 2]
            .into_iter()
            .map(|tag| MirrorPieceEdgeParameterization::new(LocalTag::new(tag), direction))
            .collect(),
    )
}
fn directed(tag: u32, direction: Direction) -> DirectedEdge {
    DirectedEdge {
        edge: EdgeRef::new(EntityId::from_bits(10), LocalTag::new(tag)),
        direction,
    }
}

#[test]
fn explicit_parameter_directions_reverse_loops_and_retain_construction_order() {
    with_edges(|identity| {
        let raw = raw_edges();
        let edges = raw
            .admit(&identity, MirrorPieceEdgeManifestLimits::new(4, 8))
            .expect("actual candidates");
        let coverage = edges
            .admit_source_coverage(MirrorPieceEdgeCoverageLimits::new(4, 8, 12))
            .expect("whole source");
        for parameter in [Direction::Original, Direction::Reversed] {
            let policy = policy(parameter);
            let original = policy.clone();
            let limits = MirrorPieceContourLimits::new(4, 2, 8, 18);
            let result = coverage
                .declare_contours(&policy, limits)
                .expect("complete explicit frames");
            assert!(std::ptr::eq(result.coverage(), &coverage));
            assert!(std::ptr::eq(result.policy(), &policy));
            assert_eq!(result.limits(), limits);
            assert_eq!(result.workspace_bound(), 18);
            assert_eq!(
                (
                    limits.parameters(),
                    limits.holes(),
                    limits.source_occurrences(),
                    limits.workspace_entries()
                ),
                (4, 2, 8, 18)
            );
            for field in policy.parameterizations() {
                let actual = result
                    .parameterization(field.tag())
                    .expect("explicit persistent tag lookup");
                assert!(std::ptr::eq(actual, field));
                assert!(std::ptr::eq(actual.direction(), field.direction()));
            }
            assert!(result.parameterization(LocalTag::new(8)).is_none());
            let adapt = |d: Direction| {
                if parameter == Direction::Reversed {
                    d.reversed()
                } else {
                    d
                }
            };
            assert_eq!(
                result.boundary(),
                [
                    directed(2, adapt(Direction::Reversed)),
                    directed(9, adapt(Direction::Original))
                ]
            );
            assert_eq!(
                result.holes(),
                [
                    vec![directed(4, adapt(Direction::Original))],
                    vec![directed(6, adapt(Direction::Reversed))]
                ]
            );
            assert_eq!(
                result.construction_lines(),
                [
                    directed(2, adapt(Direction::Reversed)),
                    directed(2, adapt(Direction::Original)),
                    directed(6, adapt(Direction::Reversed))
                ]
            );
            assert!(result.fold_edges().is_empty());
            assert_eq!(policy.operation(), EntityId::from_bits(10));
            assert_eq!(policy, original);
        }
    });
}

#[test]
fn all_resource_counts_precede_original_operation_and_parameter_checks() {
    use MirrorPieceContourRefusal as R;
    with_edges(|identity| {
        let raw = raw_edges();
        let edges = raw
            .admit(&identity, MirrorPieceEdgeManifestLimits::new(4, 8))
            .expect("candidates");
        let coverage = edges
            .admit_source_coverage(MirrorPieceEdgeCoverageLimits::new(4, 8, 12))
            .expect("source");
        let bad = MirrorPieceContourPolicy::new(
            EntityId::from_bits(99),
            vec![MirrorPieceEdgeParameterization::new(LocalTag::new(88), Direction::Original); 4],
        );
        for (limits, refusal) in [
            (
                MirrorPieceContourLimits::new(3, 0, 0, 0),
                R::Parameters {
                    required: 4,
                    allowed: 3,
                },
            ),
            (
                MirrorPieceContourLimits::new(4, 1, 0, 0),
                R::Holes {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                MirrorPieceContourLimits::new(4, 2, 7, 0),
                R::SourceOccurrences {
                    required: 8,
                    allowed: 7,
                },
            ),
            (
                MirrorPieceContourLimits::new(4, 2, 8, 17),
                R::Workspace {
                    required: 18,
                    allowed: 17,
                },
            ),
            (MirrorPieceContourLimits::new(4, 2, 8, 18), R::Operation),
        ] {
            let error = coverage
                .declare_contours(&bad, limits)
                .expect_err("whole refusal before raw fields");
            assert!(std::ptr::eq(error.coverage(), &coverage));
            assert!(std::ptr::eq(error.policy(), &bad));
            assert_eq!(error.limits(), limits);
            assert_eq!(error.refusal(), refusal);
            assert!(format!("{error}").contains("mirror contour declaration"));
        }
        let empty = MirrorPieceContourPolicy::new(EntityId::from_bits(10), vec![]);
        assert_eq!(
            coverage
                .declare_contours(&empty, MirrorPieceContourLimits::new(3, 0, 0, 0))
                .expect_err("actual output N also bounded")
                .refusal(),
            R::Outputs {
                required: 4,
                allowed: 3
            }
        );
    });
}

#[test]
fn every_actual_candidate_needs_one_explicit_parameter_without_unknown_or_duplicate_tags() {
    use MirrorPieceContourRefusal as R;
    with_edges(|identity| {
        let raw = raw_edges();
        let edges = raw
            .admit(&identity, MirrorPieceEdgeManifestLimits::new(4, 8))
            .expect("candidates");
        let coverage = edges
            .admit_source_coverage(MirrorPieceEdgeCoverageLimits::new(4, 8, 12))
            .expect("source");
        for (tags, refusal) in [
            (
                vec![9, 9],
                R::DuplicateTag {
                    first: 0,
                    second: 1,
                    tag: LocalTag::new(9),
                },
            ),
            (
                vec![8],
                R::UnknownCandidate {
                    index: 0,
                    tag: LocalTag::new(8),
                },
            ),
            (
                vec![7],
                R::UnknownCandidate {
                    index: 0,
                    tag: LocalTag::new(7),
                },
            ),
            (
                vec![88],
                R::UnknownCandidate {
                    index: 0,
                    tag: LocalTag::new(88),
                },
            ),
            (
                vec![9, 2, 4],
                R::MissingParameterization {
                    tag: LocalTag::new(6),
                },
            ),
        ] {
            let policy = MirrorPieceContourPolicy::new(
                EntityId::from_bits(10),
                tags.into_iter()
                    .map(|tag| {
                        MirrorPieceEdgeParameterization::new(
                            LocalTag::new(tag),
                            Direction::Original,
                        )
                    })
                    .collect(),
            );
            let original = policy.clone();
            assert_eq!(
                coverage
                    .declare_contours(&policy, MirrorPieceContourLimits::new(4, 2, 8, 18))
                    .expect_err("no direction fallback or partial contours")
                    .refusal(),
                refusal
            );
            assert_eq!(policy, original);
        }
    });
}

fn with_variant(
    fold: MirrorPieceFoldIntent,
    combine_holes: bool,
    run: impl FnOnce(DeclaredMirrorPieceIdentity<'_>),
) {
    let (ledger, piece) = sources::fixture();
    let mut definition = piece.definition().clone();
    if combine_holes {
        let second = definition.holes.pop().expect("second original hole");
        definition
            .holes
            .first_mut()
            .expect("first original hole")
            .extend(second);
    }
    let piece = Piece::new(definition, &ledger).expect("actual structural source");
    let input = sources::input(10, 100, sources::axis());
    let source_limits = MirrorPieceSourceLimits::new(2, 8);
    let source = input
        .bind_piece(&piece, &source_limits)
        .expect("actual source");
    let cut_policy = MirrorPieceCutPolicy::new(1, Mirroring::Single, fold, CuttingSide::Face);
    let cut = source
        .bind_cut_policy(&cut_policy)
        .expect("explicit source fold policy");
    let fields = MirrorPieceMetadataPolicy::new(
        MirrorPieceFieldIntent::PreserveSource,
        MirrorPieceFieldIntent::PreserveSource,
        MirrorPieceFieldIntent::PreserveSource,
    );
    let limits = MirrorPieceMetadataLimits::new(55);
    let metadata = cut
        .bind_metadata(&fields, &limits)
        .expect("actual source fields");
    with_edges_metadata(metadata, run);
}

#[test]
fn mixed_parameter_frames_reverse_inside_a_multi_edge_hole_and_derive_actual_fold_intent() {
    for fold in [
        MirrorPieceFoldIntent::NoFold,
        MirrorPieceFoldIntent::ReflectedSource,
    ] {
        with_variant(fold, true, |identity| {
            let raw = raw_edges();
            let edges = raw
                .admit(&identity, MirrorPieceEdgeManifestLimits::new(4, 8))
                .expect("candidates");
            let coverage = edges
                .admit_source_coverage(MirrorPieceEdgeCoverageLimits::new(4, 8, 12))
                .expect("all source occurrences");
            let policy = MirrorPieceContourPolicy::new(
                EntityId::from_bits(10),
                [
                    (4, Direction::Original),
                    (9, Direction::Original),
                    (6, Direction::Reversed),
                    (2, Direction::Reversed),
                ]
                .into_iter()
                .map(|(tag, d)| MirrorPieceEdgeParameterization::new(LocalTag::new(tag), d))
                .collect(),
            );
            let result = coverage
                .declare_contours(&policy, MirrorPieceContourLimits::new(4, 1, 8, 17))
                .expect("explicit mixed source frames");
            assert_eq!(
                result.boundary(),
                [
                    directed(2, Direction::Original),
                    directed(9, Direction::Original)
                ]
            );
            assert_eq!(
                result.holes(),
                [vec![
                    directed(6, Direction::Original),
                    directed(4, Direction::Original)
                ]]
            );
            assert_eq!(
                result.construction_lines(),
                [
                    directed(2, Direction::Original),
                    directed(2, Direction::Reversed),
                    directed(6, Direction::Original)
                ]
            );
            let expected = if fold == MirrorPieceFoldIntent::ReflectedSource {
                vec![directed(9, Direction::Original).edge]
            } else {
                vec![]
            };
            assert_eq!(result.fold_edges(), expected);
            assert_eq!(
                identity.roles().metadata().cut().cut_on_fold(),
                !result.fold_edges().is_empty()
            );
            assert_eq!(coverage.occurrence_count(), 8);
        });
    }
}
