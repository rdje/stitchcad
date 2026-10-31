//! Explicit source-identity requests do not execute global retirement or hide consumer repairs.
#![allow(clippy::expect_used)]
use sc_core::{construction::*, ontology::*, recipe::FormulaExpressionDocument};
#[path = "support/slash_sources.rs"]
mod sources;
use sources::*;
use SlashSpreadGlobalSourceIntent::{Preserve, RequestRetirementReview};

fn with_contours<T>(f: &Fixture, run: impl FnOnce(&AdmittedSlashSpreadContours<'_>) -> T) -> T {
    let intent = intent(f);
    let raw_catalog = catalog();
    let catalog = raw_catalog.admit(catalogue_limits()).expect("catalogue");
    let bound = intent
        .bind_edge_roles(&catalog, catalogue_limits())
        .expect("operation");
    let raw_edges = manifest(f);
    let edges = raw_edges
        .admit(
            &bound,
            SlashSpreadEdgeManifestLimits::new(2, catalogue_limits()),
        )
        .expect("edges");
    let raw = contours(f);
    let policy = limits();
    let contours = raw.admit(&edges, &f.piece, &policy).expect("contours");
    run(&contours)
}
fn preserve(f: &Fixture) -> SlashSpreadSourcePlan {
    SlashSpreadSourcePlan::new(vec![
        SlashSpreadSourceDisposition::new(f.second.edge, Preserve),
        SlashSpreadSourceDisposition::new(f.first.edge, Preserve),
    ])
}
fn policy() -> SlashSpreadSourcePlanLimits {
    SlashSpreadSourcePlanLimits::new(2, 2, 6, 2)
}

#[test]
fn complete_plan_retains_original_requests_and_all_occurrence_evidence_without_editing_sources() {
    let f = fixture();
    let journal = f.ledger.journal().len();
    with_contours(&f, |contours| {
        let raw = preserve(&f);
        let policy = policy();
        let bound = raw.bind(contours, &policy).expect("complete identity plan");
        assert!(std::ptr::eq(bound.plan(), &raw));
        assert!(std::ptr::eq(bound.contours(), contours));
        assert!(std::ptr::eq(bound.limits(), &policy));
        assert_eq!(
            (
                policy.candidates(),
                policy.source_edges(),
                policy.uses(),
                policy.requests()
            ),
            (2, 2, 6, 2)
        );
        assert_eq!(bound.workspace_entries(), 7);
        assert_eq!(bound.sources().len(), 2);
        let first = bound
            .sources()
            .next_back()
            .expect("original first source,second request");
        assert!(std::ptr::eq(first.plan(), &bound));
        assert!(std::ptr::eq(
            first.request(),
            raw.sources().get(1).expect("request")
        ));
        assert_eq!(first.index(), 1);
        assert_eq!(first.request().edge(), f.first.edge);
        assert_eq!(first.request().intent(), &Preserve);
        assert_eq!(first.piece_uses().collect::<Vec<_>>(), vec![f.first]);
        assert!(std::ptr::eq(
            first.cut().expect("cut"),
            contours.edges().bound_roles().intent().input().cut()
        ));
        let candidate = first
            .derived_outputs()
            .next()
            .expect("derived source candidate");
        assert!(std::ptr::eq(
            candidate.candidate(),
            contours
                .edges()
                .manifest()
                .candidates()
                .first()
                .expect("source")
        ));
        assert_eq!(first.derived_outputs().count(), 1); // repeated contour uses do not invent candidates
        assert_eq!(first.retained_uses().count(), 1);
        let second = bound.sources().next().expect("second source,first request");
        assert_eq!(second.piece_uses().collect::<Vec<_>>(), vec![f.second]);
        assert!(second.cut().is_none());
        assert_eq!(second.derived_outputs().count(), 0);
        assert_eq!(second.retained_uses().count(), 1);
        assert!(!f.ledger.is_live(candidate.logical_output()));
    });
    assert_eq!(f.ledger.journal().len(), journal);
    assert!(f.ledger.is_live(f.first.edge) && f.ledger.is_live(f.second.edge));
}

#[test]
fn whole_quotas_precede_unknown_duplicate_missing_and_retained_retirement_requests() {
    use SlashSpreadSourcePlanRefusal as R;
    let f = fixture();
    with_contours(&f, |contours| {
        let bad = SlashSpreadSourcePlan::new(vec![SlashSpreadSourceDisposition::new(
            EdgeRef::new(id(77), LocalTag::FIRST),
            Preserve,
        )]);
        for (policy, reason) in [
            (
                SlashSpreadSourcePlanLimits::new(1, 0, 0, 0),
                R::Candidates {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                SlashSpreadSourcePlanLimits::new(2, 1, 0, 0),
                R::SourceEdges {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                SlashSpreadSourcePlanLimits::new(2, 2, 5, 0),
                R::Uses {
                    required: 6,
                    allowed: 5,
                },
            ),
            (
                SlashSpreadSourcePlanLimits::new(2, 2, 6, 0),
                R::Requests {
                    required: 1,
                    allowed: 0,
                },
            ),
            (policy(), R::Source { request: 0 }),
        ] {
            let error = bad
                .bind(contours, &policy)
                .expect_err("whole quota or unknown source");
            assert_eq!(error.refusal(), reason);
            assert!(std::ptr::eq(error.plan(), &bad));
            assert!(std::ptr::eq(error.contours(), contours));
            assert!(std::ptr::eq(error.limits(), &policy));
        }
        let policy = policy();
        for (raw, reason) in [
            (
                SlashSpreadSourcePlan::new(vec![]),
                R::Missing { edge: f.first.edge },
            ),
            (
                SlashSpreadSourcePlan::new(vec![SlashSpreadSourceDisposition::new(
                    f.first.edge,
                    Preserve,
                )]),
                R::Missing {
                    edge: f.second.edge,
                },
            ),
            (
                SlashSpreadSourcePlan::new(vec![
                    SlashSpreadSourceDisposition::new(f.first.edge, Preserve),
                    SlashSpreadSourceDisposition::new(f.first.edge, Preserve),
                ]),
                R::Duplicate {
                    first: 0,
                    second: 1,
                },
            ),
            (
                SlashSpreadSourcePlan::new(vec![SlashSpreadSourceDisposition::new(
                    f.first.edge,
                    RequestRetirementReview,
                )]),
                R::RetainedRetirement { request: 0 },
            ),
        ] {
            assert_eq!(
                raw.bind(contours, &policy)
                    .expect_err("original request refusal")
                    .refusal(),
                reason
            );
        }
    });
}

#[test]
fn external_cut_and_candidate_sources_require_separate_requests_and_grant_no_global_authority() {
    use SlashSpreadSourcePlanRefusal as R;
    let f = fixture();
    let cut = EdgeRef::new(id(55), LocalTag::new(4));
    let source = EdgeRef::new(id(56), LocalTag::new(8));
    let input = SlashSpreadConstruction::rotation(
        id(10),
        id(100),
        DirectedRange {
            range: EdgeRange::whole(cut),
            direction: Direction::Reversed,
        },
        SlashSpreadSide::Left,
        PointRef::new(id(60), LocalTag::FIRST),
        FormulaExpressionDocument::new("10 deg".into()).expect("amount"),
    )
    .expect("input");
    let intent = SlashSpreadOutputIntent::new(
        input,
        SlashSpreadBoundaryPolicy::StraightBridges,
        SlashSpreadFullnessIntent::Added,
    );
    let raw_catalog = catalog();
    let catalog = raw_catalog.admit(catalogue_limits()).expect("catalogue");
    let bound = intent
        .bind_edge_roles(&catalog, catalogue_limits())
        .expect("operation");
    let raw_edges = SlashSpreadEdgeManifest::new(
        id(10),
        vec![
            SlashSpreadEdgeCandidate::new(
                LocalTag::new(9),
                SlashSpreadEdgeOrigin::Source {
                    range: DirectedRange {
                        range: EdgeRange::whole(source),
                        direction: Direction::Reversed,
                    },
                    motion: SlashSpreadSourceMotion::Moving,
                },
            ),
            *manifest(&f).candidates().get(1).expect("bridge"),
        ],
    );
    let edges = raw_edges
        .admit(
            &bound,
            SlashSpreadEdgeManifestLimits::new(2, catalogue_limits()),
        )
        .expect("edges");
    let raw_contours = contours(&f);
    let contour_policy = limits();
    let contours = raw_contours
        .admit(&edges, &f.piece, &contour_policy)
        .expect("contours");
    let policy = SlashSpreadSourcePlanLimits::new(2, 2, 6, 4);
    let mut requests = preserve(&f).sources().to_vec();
    let missing_cut = SlashSpreadSourcePlan::new(requests.clone());
    assert_eq!(
        missing_cut
            .bind(&contours, &policy)
            .expect_err("external cut required")
            .refusal(),
        R::Missing { edge: cut }
    );
    requests.push(SlashSpreadSourceDisposition::new(
        cut,
        RequestRetirementReview,
    ));
    let missing_source = SlashSpreadSourcePlan::new(requests.clone());
    assert_eq!(
        missing_source
            .bind(&contours, &policy)
            .expect_err("external candidate source required")
            .refusal(),
        R::Missing { edge: source }
    );
    requests.push(SlashSpreadSourceDisposition::new(
        source,
        RequestRetirementReview,
    ));
    let raw = SlashSpreadSourcePlan::new(requests);
    let plan = raw
        .bind(&contours, &policy)
        .expect("complete declared identity requests only");
    assert_eq!(plan.workspace_entries(), 9);
    let source_request = plan.sources().next_back().expect("external source");
    assert_eq!(source_request.request().intent(), &RequestRetirementReview);
    assert_eq!(source_request.piece_uses().count(), 0);
    assert!(source_request.cut().is_none());
    assert_eq!(source_request.derived_outputs().count(), 1);
    assert_eq!(source_request.retained_uses().count(), 0);
    assert!(!f.ledger.is_live(source) && !f.ledger.is_live(cut)); // identity requests prove no liveness
}

#[test]
fn repeated_original_and_retained_occurrences_survive_without_duplicate_identity_requests() {
    let mut f = fixture();
    let mut definition = f.piece.definition().clone();
    definition.construction_lines.push(DirectedEdge {
        direction: Direction::Reversed,
        ..f.second
    });
    f.piece = Piece::new(definition, &f.ledger).expect("repeated original construction occurrence");
    let intent = intent(&f);
    let raw_catalog = catalog();
    let catalog = raw_catalog.admit(catalogue_limits()).expect("catalogue");
    let bound = intent
        .bind_edge_roles(&catalog, catalogue_limits())
        .expect("operation");
    let raw_edges = manifest(&f);
    let edges = raw_edges
        .admit(
            &bound,
            SlashSpreadEdgeManifestLimits::new(2, catalogue_limits()),
        )
        .expect("edges");
    let base = contours(&f);
    let mut construction = base.construction_lines().to_vec();
    construction.push(SlashSpreadContourEdge::Retained(DirectedEdge {
        direction: Direction::Reversed,
        ..f.second
    }));
    let raw_contours = SlashSpreadContours::new(
        base.boundary().to_vec(),
        base.holes().to_vec(),
        construction,
    );
    let contour_policy = SlashSpreadContourLimits::new(2, 3, 0, 1, 7);
    let contours = raw_contours
        .admit(&edges, &f.piece, &contour_policy)
        .expect("repeated uses");
    let raw = preserve(&f);
    let short_policy = SlashSpreadSourcePlanLimits::new(2, 2, 7, 2);
    assert_eq!(
        raw.bind(&contours, &short_policy)
            .expect_err("R counts occurrences")
            .refusal(),
        SlashSpreadSourcePlanRefusal::SourceEdges {
            required: 3,
            allowed: 2
        }
    );
    let policy = SlashSpreadSourcePlanLimits::new(2, 3, 7, 2);
    let bound = raw
        .bind(&contours, &policy)
        .expect("complete original occurrences");
    let second = bound
        .sources()
        .next()
        .expect("one request for second identity");
    assert_eq!(
        second
            .piece_uses()
            .map(|edge| edge.direction)
            .collect::<Vec<_>>(),
        vec![Direction::Original, Direction::Reversed]
    );
    assert_eq!(
        second
            .retained_uses()
            .map(|usage| usage.direction())
            .collect::<Vec<_>>(),
        vec![Direction::Original, Direction::Reversed]
    );
}
