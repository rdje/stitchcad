//! Exact-held complete surviving cut partitions;no synthetic joined geometry.
#![allow(clippy::expect_used)]
#[allow(dead_code)]
#[path = "support/join_edges.rs"]
mod common;
use common::{id, with_identity, BankFixture, SpanFixture};
use sc_core::{construction::*, ontology::*};
fn param(n: i64, d: i64) -> Param {
    Param::new(Rational::new(n, d).expect("rational")).expect("unit interval")
}
fn candidate(
    identity: &DeclaredJoinPieceIdentity<'_>,
    t: u32,
    side: SeamSideId,
    from: Param,
    to: Param,
) -> JoinEdgeCandidate {
    let bank = identity.roles().banks().bank(side);
    JoinEdgeCandidate::new(
        LocalTag::new(t),
        side,
        bank.occurrence().location(),
        EdgeRange::new(bank.range().edge(), from, to).expect("positive original range"),
    )
}
fn admit<'a>(
    raw: &'a JoinEdgeManifest,
    identity: &'a DeclaredJoinPieceIdentity<'a>,
) -> AdmittedJoinEdges<'a> {
    let n = raw.candidates().len();
    raw.admit(identity, JoinEdgeManifestLimits::new(n, 2 * n))
        .expect("actual original fields")
}
#[test]
fn both_bank_outside_parts_are_exact_and_outputs_keep_original_roles_fields_and_range_order() {
    with_identity(
        BankFixture {
            span: SpanFixture::RetainedMiddle,
            ..BankFixture::basic()
        },
        |identity| {
            use SeamSideId::{A, B};
            let q = param(1, 4);
            let r = param(3, 4);
            let raw = JoinEdgeManifest::new(
                id(1000),
                vec![
                    candidate(&identity, 5, B, r, Param::END),
                    candidate(&identity, 4, A, r, Param::END),
                    candidate(&identity, 2, B, Param::START, q),
                    candidate(&identity, 9, A, Param::START, q),
                ],
            );
            let edges = admit(&raw, &identity);
            let limits = JoinCutCoverageLimits::new(4, 2, 4);
            let coverage = edges
                .require_cut_coverage(limits)
                .expect("complete exact outside partitions");
            assert!(std::ptr::eq(coverage.edges(), &edges));
            assert_eq!(coverage.limits(), limits);
            assert_eq!(limits.candidates(), 4);
            assert_eq!(limits.source_occurrences(), 2);
            assert_eq!(limits.workspace_entries(), 4);
            assert_eq!(coverage.workspace_entries(), 4);
            assert_eq!(coverage.cut_occurrence_count(), 2);
            assert_eq!(coverage.cut_edge_count(), 4);
            assert_eq!(
                coverage
                    .cut_edges()
                    .map(|e| e.candidate().tag())
                    .collect::<Vec<_>>(),
                [9, 4, 2, 5].map(LocalTag::new)
            );
            assert_eq!(
                coverage
                    .cut_edges()
                    .rev()
                    .map(|e| e.index())
                    .collect::<Vec<_>>(),
                [0, 2, 1, 3]
            );
            for side in [A, B] {
                let location = identity.roles().banks().bank(side).occurrence().location();
                let outputs = coverage.outputs(side, location).collect::<Vec<_>>();
                assert_eq!(outputs.len(), 2);
                assert_eq!(
                    (
                        outputs.first().expect("prefix").source().from(),
                        outputs.first().expect("prefix").source().to()
                    ),
                    (Param::START, q)
                );
                assert_eq!(
                    (
                        outputs.get(1).expect("suffix").source().from(),
                        outputs.get(1).expect("suffix").source().to()
                    ),
                    (r, Param::END)
                );
                assert_eq!(
                    coverage
                        .outputs(side, location)
                        .next_back()
                        .expect("reverse output")
                        .index(),
                    outputs.get(1).expect("suffix").index()
                );
                for edge in outputs {
                    assert!(std::ptr::eq(edge.owner(), &edges));
                    assert!(std::ptr::eq(
                        edge.candidate(),
                        raw.candidates().get(edge.index()).expect("actual original")
                    ));
                }
            }
            assert_eq!(
                coverage
                    .outputs(A, JoinSourceOccurrenceLocation::Boundary(99))
                    .len(),
                0
            );
            assert_eq!(
                coverage
                    .outputs(A, JoinSourceOccurrenceLocation::Fold(0))
                    .len(),
                0
            );
            assert!(format!("{coverage:?}").contains("CoveredJoinCutSources"));
        },
    );
}
#[test]
fn whole_n_r_and_index_admission_precede_consumed_bank_fields_and_keep_actual_parent_policy() {
    use JoinCutCoverageRefusal as R;
    with_identity(BankFixture::basic(), |identity| {
        let raw = JoinEdgeManifest::new(
            id(1000),
            vec![candidate(
                &identity,
                9,
                SeamSideId::A,
                Param::START,
                Param::END,
            )],
        );
        let edges = admit(&raw, &identity);
        for (limits, cause) in [
            (
                JoinCutCoverageLimits::new(0, 0, 0),
                R::Candidates {
                    required: 1,
                    allowed: 0,
                },
            ),
            (
                JoinCutCoverageLimits::new(1, 1, 0),
                R::SourceOccurrences {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                JoinCutCoverageLimits::new(1, 2, 0),
                R::Workspace {
                    required: 1,
                    allowed: 0,
                },
            ),
            (
                JoinCutCoverageLimits::new(1, 2, 1),
                R::ConsumedBank { index: 0 },
            ),
        ] {
            let error = edges
                .require_cut_coverage(limits)
                .expect_err("quota before exact bank intersection");
            assert!(std::ptr::eq(error.edges(), &edges));
            assert_eq!(error.limits(), limits);
            assert_eq!(error.refusal(), cause);
            if matches!(cause, R::ConsumedBank { .. }) {
                assert!(std::ptr::eq(
                    error.refused_candidate().expect("actual candidate"),
                    raw.candidates().first().expect("raw field")
                ));
                assert!(std::ptr::eq(
                    error
                        .source_occurrence()
                        .expect("original bank")
                        .field()
                        .edge(),
                    identity
                        .roles()
                        .banks()
                        .bank(SeamSideId::A)
                        .occurrence()
                        .field()
                        .edge()
                ));
            } else {
                assert!(error.refused_candidate().is_none() && error.source_occurrence().is_none());
            }
            assert!(error.to_string().starts_with("join cut-source coverage:"));
            assert!(format!("{error:?}").contains("JoinCutCoverageError"));
        }
    });
}
#[test]
fn missing_overlap_and_consumed_interiors_refuse_without_hiding_endpoint_contacts_or_original_causes(
) {
    use JoinCutCoverageRefusal as R;
    use SeamSideId::{A, B};
    with_identity(
        BankFixture {
            span: SpanFixture::RetainedTail,
            ..BankFixture::basic()
        },
        |identity| {
            let half = param(1, 2);
            let q = param(1, 4);
            let r = param(3, 4);
            let a = candidate(&identity, 9, A, Param::START, half);
            let b = candidate(&identity, 2, B, half, Param::END);
            for (candidates, cause) in [
                (
                    vec![b],
                    R::Gap {
                        side: A,
                        location: JoinSourceOccurrenceLocation::Boundary(0),
                        from: Param::START,
                        to: half,
                    },
                ),
                (
                    vec![candidate(&identity, 9, A, Param::START, q), b],
                    R::Gap {
                        side: A,
                        location: JoinSourceOccurrenceLocation::Boundary(0),
                        from: q,
                        to: half,
                    },
                ),
                (
                    vec![a, candidate(&identity, 4, A, q, half), b],
                    R::Overlap {
                        index: 1,
                        covered_to: half,
                    },
                ),
                (
                    vec![b, candidate(&identity, 9, A, q, r)],
                    R::ConsumedBank { index: 1 },
                ),
            ] {
                let raw = JoinEdgeManifest::new(id(1000), candidates);
                let snapshot = raw.clone();
                let edges = admit(&raw, &identity);
                let n = edges.edge_count();
                let error = edges
                    .require_cut_coverage(JoinCutCoverageLimits::new(n, 2, n))
                    .expect_err("first exact original domain cause");
                assert_eq!(error.refusal(), cause);
                assert_eq!(raw, snapshot);
                let original = error.source_occurrence().expect("actual domain occurrence");
                assert_eq!(original.side(), A);
                assert!(std::ptr::eq(
                    original.context(),
                    identity.roles().banks().sources()
                ));
                match cause {
                    R::Gap { .. } => assert!(error.refused_candidate().is_none()),
                    _ => assert!(std::ptr::eq(
                        error
                            .refused_candidate()
                            .expect("original rejected candidate"),
                        raw.candidates().get(1).expect("original second field")
                    )),
                }
            }
            let raw = JoinEdgeManifest::new(id(1000), vec![b, a]);
            let edges = admit(&raw, &identity);
            assert_eq!(
                edges
                    .require_cut_coverage(JoinCutCoverageLimits::new(2, 2, 2))
                    .expect("endpoint contacts preserve outside ranges")
                    .cut_edge_count(),
                2
            );
        },
    );
}
#[test]
fn complete_unrelated_boundary_hole_domains_remain_required_and_noncut_fields_never_substitute() {
    use JoinCutCoverageRefusal as R;
    use JoinSourceOccurrenceLocation as L;
    for bank_in_holes in [false, true] {
        with_identity(
            BankFixture {
                extra_occurrences: true,
                bank_in_holes,
                ..BankFixture::basic()
            },
            |identity| {
                let banks = identity.roles().banks();
                let tags = [9, 2, 4, 5, 6, 8, 10, 11, 12, 13];
                let candidates = banks
                    .sources()
                    .occurrences()
                    .zip(tags)
                    .filter_map(|(field, t)| {
                        let cut = matches!(field.location(), L::Boundary(_) | L::Hole { .. });
                        if cut
                            && field.location() == banks.bank(field.side()).occurrence().location()
                        {
                            return None;
                        }
                        Some(JoinEdgeCandidate::new(
                            LocalTag::new(t),
                            field.side(),
                            field.location(),
                            EdgeRange::whole(*field.field().edge()),
                        ))
                    })
                    .collect::<Vec<_>>();
                assert_eq!(candidates.len(), 8);
                let raw = JoinEdgeManifest::new(id(1000), candidates.clone());
                let edges = admit(&raw, &identity);
                let coverage = edges
                    .require_cut_coverage(JoinCutCoverageLimits::new(8, 10, 8))
                    .expect("all surviving original cut fields");
                assert_eq!(coverage.cut_occurrence_count(), 4);
                assert_eq!(coverage.cut_edge_count(), 2);
                assert_eq!(coverage.workspace_entries(), 8);
                assert_eq!(coverage.edges().edge_count(), 8);
                assert_eq!(coverage.cut_edges().len(), 2);
                let location = if bank_in_holes {
                    L::Boundary(0)
                } else {
                    L::Hole { hole: 0, index: 0 }
                };
                let missing = candidates
                    .into_iter()
                    .filter(|c| !(c.side() == SeamSideId::A && c.location() == location))
                    .collect();
                let raw = JoinEdgeManifest::new(id(1000), missing);
                let edges = admit(&raw, &identity);
                let error = edges
                    .require_cut_coverage(JoinCutCoverageLimits::new(7, 10, 7))
                    .expect_err("unrelated whole cut source cannot disappear");
                assert_eq!(
                    error.refusal(),
                    R::Gap {
                        side: SeamSideId::A,
                        location,
                        from: Param::START,
                        to: Param::END
                    }
                );
                let error = edges
                    .require_cut_coverage(JoinCutCoverageLimits::new(7, 4, 7))
                    .expect_err("full R includes repeated construction and separate folds");
                assert_eq!(
                    error.refusal(),
                    R::SourceOccurrences {
                        required: 10,
                        allowed: 4
                    }
                );
                let error = edges
                    .require_cut_coverage(JoinCutCoverageLimits::new(7, 10, 1))
                    .expect_err("index admission counts all candidates before filtering");
                assert_eq!(
                    error.refusal(),
                    R::Workspace {
                        required: 7,
                        allowed: 1
                    }
                );
            },
        );
    }
    with_identity(BankFixture::basic(), |identity| {
        let raw = JoinEdgeManifest::new(id(1000), vec![]);
        let edges = admit(&raw, &identity);
        let coverage = edges
            .require_cut_coverage(JoinCutCoverageLimits::new(0, 2, 0))
            .expect("whole banks have no outside intervals");
        assert_eq!(coverage.cut_occurrence_count(), 2);
        assert_eq!(coverage.cut_edge_count(), 0);
        assert_eq!(
            *coverage
                .edges()
                .identity()
                .roles()
                .banks()
                .sources()
                .population()
                .quantity(),
            1
        );
        // Whole-result nonempty/closure/topology remains a separate declaration requirement.
    });
}
