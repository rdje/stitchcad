//! Actual bounded source lock requests;no current mapping,position,observer or fit certificate.
#![allow(clippy::expect_used)]
#[path = "support/truing_domain.rs"]
mod common;
use common::{
    q,
    roles::copies::{id, with_copies},
    roles::with_roles,
    with_partial,
};
use sc_core::{construction::*, ontology::*};

fn with_outcome(
    roles: &BoundTruingEdgeRoles<'_>,
    intent: TruingOutcomeIntent,
    run: impl FnOnce(RequiredTruingOutcome<'_>),
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
        .expect("whole metadata");
    let membership = edges
        .bind_source_domain(TruingSourceDomainLimits::new(n))
        .expect("membership");
    let outcome = membership
        .require_outcome(intent, TruingSourceCoverageLimits::new(n, n))
        .expect("explicit outcome");
    run(outcome);
}

#[test]
fn both_outcomes_and_adjusted_sides_keep_actual_selected_endpoints_and_every_original_stop_on_its_own_side(
) {
    for side in [SeamSideId::A, SeamSideId::B] {
        for intent in [
            TruingOutcomeIntent::PreserveSource,
            TruingOutcomeIntent::Refit,
        ] {
            with_partial(side, |roles| {
                with_outcome(&roles, intent, |outcome| {
                    let limits = TruingLockLimits::new(2, 1);
                    let locks = outcome
                        .require_locks(limits)
                        .expect("whole lock requirements");
                    assert!(std::ptr::eq(locks.outcome(), &outcome));
                    assert!(std::ptr::eq(locks.copies(), roles.copies()));
                    assert!(std::ptr::eq(locks.source(), roles.copies().source()));
                    assert_eq!(locks.limits(), limits);
                    assert_eq!((limits.copies(), limits.stops()), (2, 1));
                    assert_eq!(locks.endpoints().len(), 2);
                    for (endpoint, (kind, param)) in locks.endpoints().zip([
                        (TruingEndpoint::From, q(1, 4)),
                        (TruingEndpoint::To, q(3, 4)),
                    ]) {
                        assert!(std::ptr::eq(endpoint.context(), &locks));
                        assert!(std::ptr::eq(
                            endpoint.source_range(),
                            &locks.source().source_side().range
                        ));
                        assert_eq!(endpoint.endpoint(), kind);
                        assert_eq!(
                            endpoint.anchor(),
                            EdgeAnchor {
                                edge: locks.source().source_side().range.edge(),
                                param
                            }
                        );
                    }
                    assert_eq!(
                        locks
                            .endpoints()
                            .rev()
                            .map(|e| e.endpoint())
                            .collect::<Vec<_>>(),
                        vec![TruingEndpoint::To, TruingEndpoint::From]
                    );
                    assert_eq!(locks.stops().len(), 1);
                    let stop = locks
                        .stops()
                        .next()
                        .expect("all original stops,even counterpart side");
                    assert!(std::ptr::eq(stop.context(), &locks));
                    assert_eq!(stop.index(), 0);
                    assert!(std::ptr::eq(
                        stop.stop(),
                        locks
                            .source()
                            .span()
                            .definition()
                            .stops
                            .first()
                            .expect("original stop")
                    ));
                    assert_eq!(stop.stop().side, SeamSideId::A);
                    assert!(std::ptr::eq(
                        stop.side(),
                        &locks.source().span().definition().a
                    ));
                    assert!(std::ptr::eq(
                        stop.copy(),
                        roles.copies().copy(SeamSideId::A)
                    ));
                    assert!(std::ptr::eq(
                        stop.piece(),
                        roles.copies().piece(SeamSideId::A)
                    ));
                    assert_eq!(stop.copy().id(), id(200));
                    assert_eq!(stop.piece().id(), id(100));
                    assert_eq!(
                        locks
                            .stops()
                            .next_back()
                            .expect("reverse original stop")
                            .index(),
                        0
                    );
                    assert_eq!(
                        locks.affected_copies().map(|c| c.id()).collect::<Vec<_>>(),
                        vec![id(if side == SeamSideId::A { 200 } else { 201 })]
                    );
                    assert_eq!(outcome.intent(), intent);
                    assert!(format!("{locks:?}").contains("RequiredTruingLocks"));
                })
            });
        }
    }
}

#[test]
fn whole_plan_quota_includes_unrelated_copies_before_all_stop_quota_even_for_empty_preservation() {
    with_roles(false, |roles| {
        with_outcome(&roles, TruingOutcomeIntent::PreserveSource, |outcome| {
            for (limits, cause) in [
                (
                    TruingLockLimits::new(1, 0),
                    TruingLockRefusal::Copies {
                        required: 2,
                        allowed: 1,
                    },
                ),
                (
                    TruingLockLimits::new(2, 0),
                    TruingLockRefusal::Stops {
                        required: 1,
                        allowed: 0,
                    },
                ),
            ] {
                let error = outcome
                    .require_locks(limits)
                    .expect_err("whole C then S before filtering");
                assert!(std::ptr::eq(error.outcome(), &outcome));
                assert_eq!(error.limits(), limits);
                assert_eq!(error.refusal(), cause);
                assert!(error.to_string().starts_with("truing lock requirements:"));
                assert!(format!("{error:?}").contains("TruingLockError"));
            }
            let locks = outcome
                .require_locks(TruingLockLimits::new(2, 1))
                .expect("quota equality");
            assert_eq!(locks.affected_copies().count(), 1);
            assert_eq!(locks.stops().count(), 1);
        })
    });
}

#[test]
fn shared_pattern_counterpart_stays_in_the_affected_copy_requests_in_original_plan_order() {
    with_copies(true, |copies| {
        let raw = EdgeOutputRoleCatalog::new(id(1000), vec![], vec![]);
        let limits = EdgeOutputRoleCatalogLimits::new(0, 0, 0);
        let catalog = raw.admit(limits).expect("explicit empty role metadata");
        let roles = copies
            .bind_edge_roles(&catalog, limits)
            .expect("actual shared Piece context");
        with_outcome(&roles, TruingOutcomeIntent::PreserveSource, |outcome| {
            let locks = outcome
                .require_locks(TruingLockLimits::new(2, 1))
                .expect("bounded shared-copy lock request");
            assert!(locks.copies().same_pattern_piece());
            let all = locks.source().plan().copies();
            assert_eq!(all.len(), 2);
            assert_eq!(
                locks.affected_copies().map(|c| c.id()).collect::<Vec<_>>(),
                all.iter().map(|c| c.id()).collect::<Vec<_>>()
            );
            assert!(locks
                .affected_copies()
                .any(|c| std::ptr::eq(c, locks.copies().copy(SeamSideId::B))));
            let stop = locks.stops().next().expect("original A stop");
            assert!(std::ptr::eq(stop.piece(), locks.copies().selected_piece()));
            assert!(std::ptr::eq(
                stop.piece(),
                locks.copies().counterpart_piece()
            ));
        });
    });
}
