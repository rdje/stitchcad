//! Complete source identity attribution retains every repeated directed/fold occurrence.
#![allow(clippy::expect_used)]
#[path = "support/mirror_edges.rs"]
mod common;
use common::with_edges;
use sc_core::{construction::*, ontology::*};

fn source(creator: u128) -> EdgeRef {
    EdgeRef::new(EntityId::from_bits(creator), LocalTag::FIRST)
}
fn raw(candidates: &[(u32, u128)]) -> MirrorPieceEdgeManifest {
    MirrorPieceEdgeManifest::new(
        EntityId::from_bits(10),
        candidates
            .iter()
            .map(|(tag, creator)| {
                MirrorPieceEdgeCandidate::new(LocalTag::new(*tag), source(*creator))
            })
            .collect(),
    )
}
fn complete() -> MirrorPieceEdgeManifest {
    raw(&[(6, 43), (2, 41), (9, 40), (4, 42)])
}

#[test]
fn unique_identities_keep_every_authentic_occurrence_and_original_direction() {
    with_edges(|identity| {
        let raw = complete();
        let original = raw.clone();
        let edges = raw
            .admit(&identity, MirrorPieceEdgeManifestLimits::new(4, 8))
            .expect("whole candidates");
        let limits = MirrorPieceEdgeCoverageLimits::new(4, 8, 12);
        let coverage = edges
            .admit_source_coverage(limits)
            .expect("every original source");
        assert!(std::ptr::eq(coverage.edges(), &edges));
        assert_eq!(coverage.limits(), limits);
        assert_eq!(
            (
                limits.candidates(),
                limits.occurrences(),
                limits.workspace_entries()
            ),
            (4, 8, 12)
        );
        assert_eq!(
            (
                coverage.source_count(),
                coverage.occurrence_count(),
                coverage.workspace_entries()
            ),
            (4, 8, 12)
        );
        let copy = identity.roles().metadata().cut().source();
        for (mapped, original) in coverage.occurrences().zip(copy.edges()) {
            assert!(std::ptr::eq(mapped.coverage(), &coverage));
            assert!(std::ptr::eq(mapped.source().copy(), copy));
            assert_eq!(mapped.source().location(), original.location());
            assert!(std::ptr::eq(
                mapped.source().source().edge(),
                original.source().edge()
            ));
            assert!(
                match (mapped.source().source(), original.source()) {
                    (MirrorPieceEdgeSource::Directed(a), MirrorPieceEdgeSource::Directed(b)) => {
                        std::ptr::eq(a, b)
                    }
                    (MirrorPieceEdgeSource::Fold(a), MirrorPieceEdgeSource::Fold(b)) => {
                        std::ptr::eq(a, b)
                    }
                    _ => false,
                },
                "original directed/fold field changed domain"
            );
            assert!(std::ptr::eq(mapped.output().owner(), &edges));
            assert_eq!(mapped.output().source(), original.source().edge());
            assert_eq!(
                mapped.output().logical_output(),
                EdgeRef::new(EntityId::from_bits(10), mapped.output().candidate().tag())
            );
        }
        assert_eq!(
            coverage
                .occurrences()
                .map(|m| m.output().candidate().tag().get())
                .collect::<Vec<_>>(),
            vec![9, 2, 4, 6, 2, 2, 6, 9]
        );
        assert_eq!(
            coverage
                .occurrences()
                .rev()
                .map(|m| m.output().candidate().tag().get())
                .collect::<Vec<_>>(),
            vec![9, 6, 2, 2, 6, 4, 2, 9]
        );
        let mapped = coverage
            .edge_for_source(&source(41))
            .expect("exact original identity");
        assert_eq!(mapped.index(), 1);
        assert_eq!(mapped.candidate().tag(), LocalTag::new(2));
        assert!(coverage
            .edge_for_source(&EdgeRef::new(EntityId::from_bits(41), LocalTag::new(7)))
            .is_none());
        assert!(coverage.edge_for_source(&source(99)).is_none());
        assert_eq!(raw, original);
    });
}

#[test]
fn whole_candidate_occurrence_and_workspace_limits_precede_ambiguous_source() {
    use MirrorPieceEdgeCoverageRefusal as R;
    with_edges(|identity| {
        let raw = raw(&[(9, 40), (2, 40)]);
        let edges = raw
            .admit(&identity, MirrorPieceEdgeManifestLimits::new(2, 4))
            .expect("intermediate duplicate sources");
        for (limits, refusal) in [
            (
                MirrorPieceEdgeCoverageLimits::new(1, 0, 0),
                R::Candidates {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                MirrorPieceEdgeCoverageLimits::new(2, 7, 0),
                R::Occurrences {
                    required: 8,
                    allowed: 7,
                },
            ),
            (
                MirrorPieceEdgeCoverageLimits::new(2, 8, 9),
                R::Workspace {
                    required: 10,
                    allowed: 9,
                },
            ),
            (
                MirrorPieceEdgeCoverageLimits::new(2, 8, 10),
                R::AmbiguousSource {
                    first: 0,
                    second: 1,
                    source: source(40),
                },
            ),
        ] {
            let error = edges
                .admit_source_coverage(limits)
                .expect_err("whole refusal without partial coverage");
            assert!(std::ptr::eq(error.edges(), &edges));
            assert_eq!(error.limits(), limits);
            assert_eq!(error.refusal(), refusal);
            assert!(format!("{error}").contains("mirror source-edge coverage"));
        }
    });
}

#[test]
fn missing_original_hole_and_empty_proposals_refuse_with_original_locations() {
    use MirrorPieceEdgeCoverageRefusal as R;
    with_edges(|identity| {
        for (raw, expected) in [
            (
                raw(&[(9, 40), (2, 41), (6, 43)]),
                R::MissingSource {
                    location: MirrorPieceEdgeLocation::Hole { hole: 0, edge: 0 },
                    source: source(42),
                },
            ),
            (
                raw(&[]),
                R::MissingSource {
                    location: MirrorPieceEdgeLocation::Boundary(0),
                    source: source(40),
                },
            ),
        ] {
            let edges = raw
                .admit(&identity, MirrorPieceEdgeManifestLimits::new(3, 6))
                .expect("intermediate presence metadata");
            assert_eq!(
                edges
                    .admit_source_coverage(MirrorPieceEdgeCoverageLimits::new(3, 8, 11))
                    .expect_err("not a complete copy")
                    .refusal(),
                expected
            );
        }
        assert!(identity.roles().catalog().is_active(LocalTag::new(9)));
    });
}

#[test]
fn complete_sources_reject_unreferenced_candidates_in_original_proposal_order() {
    with_edges(|identity| {
        // Unused creator99 sorts after all actual sources,but remains original candidate0.
        let raw = raw(&[(8, 99), (5, 1), (6, 43), (2, 41), (9, 40), (4, 42)]);
        let original = raw.clone();
        let edges = raw
            .admit(&identity, MirrorPieceEdgeManifestLimits::new(6, 12))
            .expect("intermediate unrelated source");
        assert_eq!(
            edges
                .admit_source_coverage(MirrorPieceEdgeCoverageLimits::new(6, 8, 14))
                .expect_err("every candidate needs an actual occurrence")
                .refusal(),
            MirrorPieceEdgeCoverageRefusal::UnusedCandidate {
                index: 0,
                source: source(99)
            }
        );
        assert_eq!(raw, original);
    });
}
