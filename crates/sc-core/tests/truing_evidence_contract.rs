//! Mandatory original truing evidence requests;no provider resolution,fit or readiness verdict.
#![allow(clippy::expect_used)]
#[path = "support/truing_domain.rs"]
mod common;
use common::{
    roles::{copies::id, with_roles},
    with_partial,
};
use sc_core::{construction::*, ontology::*};

fn with_locks(
    roles: &BoundTruingEdgeRoles<'_>,
    intent: TruingOutcomeIntent,
    run: impl FnOnce(RequiredTruingLocks<'_>),
) {
    let candidates = if intent == TruingOutcomeIntent::Refit {
        vec![TruingEdgeCandidate::new(
            LocalTag::new(9),
            roles.copies().source().source_side().range,
        )]
    } else {
        vec![]
    };
    let n = candidates.len();
    let raw = TruingEdgeManifest::new(id(1000), candidates);
    let edges = raw
        .admit(roles, TruingEdgeManifestLimits::new(n, n * 2))
        .expect("actual candidate association");
    let membership = edges
        .bind_source_domain(TruingSourceDomainLimits::new(n))
        .expect("actual membership");
    let outcome = membership
        .require_outcome(intent, TruingSourceCoverageLimits::new(n, n))
        .expect("actual outcome");
    let locks = outcome
        .require_locks(TruingLockLimits::new(2, 1))
        .expect("actual complete source locks");
    run(locks);
}
fn distinct_policy() -> TruingEvidencePolicy {
    TruingEvidencePolicy::new(
        TruingContextSources::new(id(101), id(102), id(103)),
        TruingFitSources::new(id(201), id(202), id(203), id(204)),
        TruingEffectSources::new(id(301), id(302), id(303), id(304)),
    )
}
#[test]
fn every_scope_keeps_its_original_field_and_complete_lock_outcome_context_for_either_side_and_outcome(
) {
    use TruingEvidenceKind as K;
    let expected = [
        (K::CurrentOwnership, id(101)),
        (K::IdentityHistory, id(102)),
        (K::VisibleRepairs, id(103)),
        (K::DeterministicCurve, id(201)),
        (K::Outcome, id(202)),
        (K::LengthFit, id(203)),
        (K::DifferentialAndEase, id(204)),
        (K::EndpointAndStopCorrespondence, id(301)),
        (K::Profiles, id(302)),
        (K::AllCopyEffects, id(303)),
        (K::AuthoringAuthority, id(304)),
    ];
    let policy = distinct_policy();
    assert_eq!(
        *policy.context_sources(),
        TruingContextSources::new(id(101), id(102), id(103))
    );
    assert_eq!(
        *policy.fit_sources(),
        TruingFitSources::new(id(201), id(202), id(203), id(204))
    );
    assert_eq!(
        *policy.effect_sources(),
        TruingEffectSources::new(id(301), id(302), id(303), id(304))
    );
    for side in [SeamSideId::A, SeamSideId::B] {
        for intent in [
            TruingOutcomeIntent::PreserveSource,
            TruingOutcomeIntent::Refit,
        ] {
            with_partial(side, |roles| {
                with_locks(&roles, intent, |locks| {
                    let required = locks.require_evidence(&policy);
                    assert!(std::ptr::eq(required.locks(), &locks));
                    assert!(std::ptr::eq(required.policy(), &policy));
                    assert_eq!(required.requirements().len(), expected.len());
                    for (item, (kind, source)) in required.requirements().zip(expected) {
                        assert!(std::ptr::eq(item.context(), &required));
                        assert_eq!(item.kind(), kind);
                        assert_eq!(*item.source(), source);
                        assert_eq!(*policy.source(kind), source);
                        assert!(std::ptr::eq(item.source(), policy.source(kind)));
                    }
                    assert_eq!(
                        required
                            .requirements()
                            .rev()
                            .map(|r| r.kind())
                            .collect::<Vec<_>>(),
                        expected
                            .into_iter()
                            .rev()
                            .map(|(kind, _)| kind)
                            .collect::<Vec<_>>()
                    );
                    assert!(std::ptr::eq(
                        required.locks().source(),
                        roles.copies().source()
                    ));
                    assert_eq!(required.locks().outcome().intent(), intent);
                    assert_eq!(required.locks().source().edit().adjusted_side(), side);
                    assert_eq!(required.locks().endpoints().len(), 2);
                    assert_eq!(required.locks().stops().len(), 1);
                    assert!(format!("{required:?}").contains("RequiredTruingEvidence"));
                })
            });
        }
    }
}
#[test]
fn shared_zero_source_requests_and_empty_preservation_discharge_none_of_the_mandatory_scopes() {
    let zero = id(0);
    let policy = TruingEvidencePolicy::new(
        TruingContextSources::new(zero, zero, zero),
        TruingFitSources::new(zero, zero, zero, zero),
        TruingEffectSources::new(zero, zero, zero, zero),
    );
    with_roles(false, |roles| {
        with_locks(&roles, TruingOutcomeIntent::PreserveSource, |locks| {
            let required = locks.require_evidence(&policy);
            assert_eq!(required.requirements().len(), 11);
            assert!(required.requirements().all(|r| *r.source() == zero));
            assert_eq!(required.locks().outcome().source_domain().edge_count(), 0);
            assert_eq!(required.locks().stops().len(), 1);
            assert_eq!(required.locks().endpoints().len(), 2);
            assert_eq!(required.locks().affected_copies().count(), 1);
            assert!(std::ptr::eq(required.policy(), &policy));
        })
    });
}
