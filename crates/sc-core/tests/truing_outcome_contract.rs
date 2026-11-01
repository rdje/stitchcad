//! Explicit unchanged/refit requirements;empty metadata is never an achieved no-op or fit.
#![allow(clippy::expect_used)]
#[path = "support/truing_domain.rs"]
mod common;
use common::{q, roles::copies::id, roles::with_roles, with_partial};
use sc_core::{construction::*, ontology::*};

fn proposal(ranges: &[EdgeRange]) -> TruingEdgeManifest {
    TruingEdgeManifest::new(
        id(1000),
        [9, 2]
            .into_iter()
            .zip(ranges)
            .map(|(tag, range)| TruingEdgeCandidate::new(LocalTag::new(tag), *range))
            .collect(),
    )
}

#[test]
fn explicit_empty_source_preservation_keeps_actual_whole_parents_and_active_roles_while_empty_refit_refuses(
) {
    for side in [SeamSideId::A, SeamSideId::B] {
        with_partial(side, |roles| {
            let raw = proposal(&[]);
            let edges = raw
                .admit(&roles, TruingEdgeManifestLimits::new(0, 0))
                .expect("explicit empty metadata");
            let membership = edges
                .bind_source_domain(TruingSourceDomainLimits::new(0))
                .expect("empty membership");
            let limits = TruingSourceCoverageLimits::new(0, 0);
            let required = membership
                .require_outcome(TruingOutcomeIntent::PreserveSource, limits)
                .expect("explicit unchanged-source requirement");
            assert!(std::ptr::eq(required.source_domain(), &membership));
            assert_eq!(required.intent(), TruingOutcomeIntent::PreserveSource);
            assert_eq!(required.limits(), limits);
            assert!(required.coverage().is_none());
            assert!(required
                .source_domain()
                .candidates()
                .roles()
                .catalog()
                .active_role(LocalTag::new(9))
                .is_some());
            assert!(std::ptr::eq(
                required.source_domain().candidates().roles(),
                &roles
            ));
            let source = roles.copies().source();
            assert_eq!(source.edit().adjusted_side(), side);
            assert_eq!(source.span().definition().stops.len(), 1);
            assert!(std::ptr::eq(
                source.fit_provider().tolerance,
                roles.copies().source().fit_provider().tolerance
            ));
            assert_eq!(source.affected_copies().count(), 1);
            assert!(format!("{required:?}").contains("RequiredTruingOutcome"));
            let error = membership
                .require_outcome(TruingOutcomeIntent::Refit, limits)
                .expect_err("empty cannot supply a refit partition");
            assert!(std::ptr::eq(error.source_domain(), &membership));
            assert_eq!(error.intent(), TruingOutcomeIntent::Refit);
            assert_eq!(error.limits(), limits);
            assert_eq!(
                error.refusal(),
                TruingOutcomeRefusal::Coverage(TruingSourceCoverageRefusal::Gap {
                    from: q(1, 4),
                    to: q(3, 4),
                    next: None,
                })
            );
            assert!(error.to_string().starts_with("truing outcome requirement:"));
            assert!(format!("{error:?}").contains("TruingOutcomeError"));
        });
    }
}

#[test]
fn whole_quota_precedes_source_preservation_contradiction_and_refit_keeps_authentic_complete_partition(
) {
    with_roles(false, |roles| {
        let original = roles.copies().source().source_side().range;
        let raw = proposal(&[original]);
        let snapshot = raw.clone();
        let edges = raw
            .admit(&roles, TruingEdgeManifestLimits::new(1, 2))
            .expect("complete original candidate");
        let membership = edges
            .bind_source_domain(TruingSourceDomainLimits::new(1))
            .expect("membership");
        for (limits, cause) in [
            (
                TruingSourceCoverageLimits::new(0, 0),
                TruingOutcomeRefusal::Candidates {
                    required: 1,
                    allowed: 0,
                },
            ),
            (
                TruingSourceCoverageLimits::new(1, 0),
                TruingOutcomeRefusal::NewEdges { count: 1 },
            ),
        ] {
            let error = membership
                .require_outcome(TruingOutcomeIntent::PreserveSource, limits)
                .expect_err("no new candidate ids in a source-preserving request");
            assert!(std::ptr::eq(error.source_domain(), &membership));
            assert_eq!(error.intent(), TruingOutcomeIntent::PreserveSource);
            assert_eq!(error.limits(), limits);
            assert_eq!(error.refusal(), cause);
        }
        let limits = TruingSourceCoverageLimits::new(1, 1);
        let required = membership
            .require_outcome(TruingOutcomeIntent::Refit, limits)
            .expect("explicit refit request");
        assert_eq!(required.intent(), TruingOutcomeIntent::Refit);
        assert!(std::ptr::eq(required.source_domain(), &membership));
        assert_eq!(required.limits(), limits);
        let coverage = required.coverage().expect("authentic refit partition");
        assert!(std::ptr::eq(coverage.source_domain(), &membership));
        assert_eq!(coverage.limits(), limits);
        assert_eq!(coverage.source_order().len(), 1);
        let member = coverage
            .source_order()
            .next()
            .expect("actual original member");
        assert!(std::ptr::eq(member.context(), &membership));
        assert!(std::ptr::eq(
            member.source(),
            raw.candidates()
                .first()
                .expect("original candidate")
                .source()
        ));
        assert_eq!(raw, snapshot);
    });
}

#[test]
fn refit_retains_original_workspace_and_gap_overlap_causes_without_accepted_prefix_or_mode_fallback(
) {
    with_partial(SeamSideId::B, |roles| {
        let domain = roles.copies().source().source_side().range;
        let first = EdgeRange::new(domain.edge(), q(1, 4), q(5, 8)).expect("left");
        let second = EdgeRange::new(domain.edge(), q(1, 2), q(3, 4)).expect("right");
        let raw = proposal(&[first, second]);
        let edges = raw
            .admit(&roles, TruingEdgeManifestLimits::new(2, 4))
            .expect("metadata");
        let membership = edges
            .bind_source_domain(TruingSourceDomainLimits::new(2))
            .expect("membership");
        for (limits, cause) in [
            (
                TruingSourceCoverageLimits::new(1, 0),
                TruingOutcomeRefusal::Candidates {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                TruingSourceCoverageLimits::new(2, 1),
                TruingOutcomeRefusal::Coverage(TruingSourceCoverageRefusal::Workspace {
                    required: 2,
                    allowed: 1,
                }),
            ),
            (
                TruingSourceCoverageLimits::new(2, 2),
                TruingOutcomeRefusal::Coverage(TruingSourceCoverageRefusal::Overlap {
                    previous: 0,
                    index: 1,
                    from: q(1, 2),
                    to: q(5, 8),
                }),
            ),
        ] {
            let error = membership
                .require_outcome(TruingOutcomeIntent::Refit, limits)
                .expect_err("refit retains exact cause");
            assert_eq!(error.refusal(), cause);
            assert_eq!(error.intent(), TruingOutcomeIntent::Refit);
            assert!(std::ptr::eq(error.source_domain(), &membership));
        }
    });
}
