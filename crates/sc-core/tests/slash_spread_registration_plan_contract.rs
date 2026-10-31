//! Complete actual registered-source requests;no repair,withdrawal,correspondence or authority proof.
#![allow(clippy::expect_used)]
use sc_core::{construction::*, ontology::*};
#[path = "support/slash_sources.rs"]
mod sources;
use sources::*;
use SlashSpreadRegistrationIntent::{
    KeepOriginalGlobalSource, RequestWithdrawalReview, RequireCorrespondenceOrVisibleRepair,
};
fn half() -> Param {
    Param::new(Rational::new(1, 2).expect("half")).expect("unit")
}
fn request(
    owner: u128,
    edge: EdgeRef,
    intent: SlashSpreadRegistrationIntent,
) -> SlashSpreadRegistrationDisposition {
    SlashSpreadRegistrationDisposition::new(
        id(owner),
        EdgeAnchor {
            edge,
            param: half(),
        },
        intent,
    )
}
fn with_sources<T>(
    f: &Fixture,
    retire_second: bool,
    run: impl FnOnce(&BoundSlashSpreadSourcePlan<'_>) -> T,
) -> T {
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
    let base = contours(f);
    let construction = base.construction_lines().iter().copied().filter(|usage| !retire_second || !matches!(usage, SlashSpreadContourEdge::Retained(edge) if edge.edge == f.second.edge)).collect();
    let raw_contours = SlashSpreadContours::new(
        base.boundary().to_vec(),
        base.holes().to_vec(),
        construction,
    );
    let m = if retire_second { 5 } else { 6 };
    let contour_policy = if retire_second {
        SlashSpreadContourLimits::new(2, 2, 0, 1, m)
    } else {
        limits()
    };
    let contours = raw_contours
        .admit(&edges, &f.piece, &contour_policy)
        .expect("contours");
    let source_plan = SlashSpreadSourcePlan::new(vec![
        SlashSpreadSourceDisposition::new(f.first.edge, SlashSpreadGlobalSourceIntent::Preserve),
        SlashSpreadSourceDisposition::new(
            f.second.edge,
            if retire_second {
                SlashSpreadGlobalSourceIntent::RequestRetirementReview
            } else {
                SlashSpreadGlobalSourceIntent::Preserve
            },
        ),
    ]);
    let policy = SlashSpreadSourcePlanLimits::new(2, 2, m, 2);
    let sources = source_plan
        .bind(&contours, &policy)
        .expect("source requests");
    run(&sources)
}
fn register(f: &mut Fixture, owner: u128, edge: EdgeRef) {
    f.ledger
        .register(id(owner), edge, half())
        .expect("actual registration");
}

#[test]
fn both_registered_owners_and_their_actual_split_choices_survive_in_original_request_order() {
    let mut f = fixture();
    let source = f.first.edge;
    register(&mut f, 200, source);
    register(&mut f, 201, source);
    f.ledger
        .state_split_choice(id(200), source, half(), SplitSide::Second)
        .expect("owner choice");
    let mut ids = DeterministicIdGenerator::from_start(1000);
    f.ledger
        .split(&mut ids, source, half())
        .expect("actual later split");
    let registrations = f.ledger.registrations().collect::<Vec<_>>();
    let journal = f.ledger.journal().len();
    with_sources(&f, false, |sources| {
        let raw = SlashSpreadRegistrationPlan::new(vec![
            request(201, source, RequestWithdrawalReview),
            request(200, source, RequireCorrespondenceOrVisibleRepair),
        ]);
        let policy = SlashSpreadRegistrationPlanLimits::new(2, 2, 2);
        let plan = raw
            .bind(sources, &f.ledger, &policy)
            .expect("all actual registered sources");
        assert!(std::ptr::eq(plan.plan(), &raw));
        assert!(std::ptr::eq(plan.sources(), sources));
        assert!(std::ptr::eq(plan.ledger(), &f.ledger));
        assert!(std::ptr::eq(plan.limits(), &policy));
        assert_eq!(
            (policy.sources(), policy.registrations(), policy.requests()),
            (2, 2, 2)
        );
        assert_eq!(plan.workspace_entries(), 8);
        assert_eq!(plan.requests().len(), 2);
        let b = plan.requests().next().expect("first request is owner B");
        assert_eq!(b.index(), 0);
        assert!(std::ptr::eq(b.plan(), &plan));
        assert!(std::ptr::eq(
            b.request(),
            raw.requests().first().expect("raw B")
        ));
        assert_eq!(b.request().owner(), id(201));
        assert_eq!(
            b.request().anchor(),
            &EdgeAnchor {
                edge: source,
                param: half()
            }
        );
        assert_eq!(b.request().intent(), &RequestWithdrawalReview);
        assert!(std::ptr::eq(b.source().plan(), sources));
        assert_eq!(b.source().request().edge(), source);
        assert_eq!(b.registration().split_choice(), None);
        let a = plan
            .requests()
            .next_back()
            .expect("last request is owner A");
        assert_eq!(
            a.registration(),
            *registrations.first().expect("actual owner A")
        );
        assert_eq!(a.registration().split_choice(), Some(SplitSide::Second));
        assert_eq!(a.source().retained_uses().count(), 1);
    });
    assert_eq!(f.ledger.journal().len(), journal);
    assert_eq!(f.ledger.registrations().collect::<Vec<_>>(), registrations);
    assert_eq!(f.ledger.registration_count(), 2);
}

#[test]
fn unresolved_registrations_are_covered_and_withdrawal_requests_close_no_repairs() {
    let mut f = fixture();
    let source = f.first.edge;
    register(&mut f, 200, source);
    register(&mut f, 201, source);
    let mut ids = DeterministicIdGenerator::from_start(1000);
    f.ledger
        .delete(&mut ids, source)
        .expect("actual global deletion");
    let repairs = f.ledger.open_repairs();
    assert_eq!(repairs.len(), 2);
    let journal = f.ledger.journal().len();
    with_sources(&f, false, |sources| {
        let raw = SlashSpreadRegistrationPlan::new(vec![
            request(200, source, RequireCorrespondenceOrVisibleRepair),
            request(201, source, RequestWithdrawalReview),
        ]);
        let policy = SlashSpreadRegistrationPlanLimits::new(2, 2, 2);
        let plan = raw
            .bind(sources, &f.ledger, &policy)
            .expect("requests accept unresolved evidence without repairing it");
        assert_eq!(plan.requests().count(), 2);
        let evidence =
            SlashSpreadConsumerEvidenceSources::new(id(800), id(801), id(802), id(800), id(800));
        let required = plan.require_consumer_evidence(&evidence);
        assert!(std::ptr::eq(required.registration_plan(), &plan));
        assert!(std::ptr::eq(required.evidence_sources(), &evidence));
        assert_eq!(required.requirements().len(), 5);
        assert_eq!(
            required
                .requirements()
                .filter(|item| *item.source() == id(800))
                .count(),
            3
        );
        use SlashSpreadConsumerEvidenceKind as E;
        assert_eq!(
            required
                .requirements()
                .map(|item| item.kind())
                .collect::<Vec<_>>(),
            vec![
                E::WholeSourceConsumers,
                E::CurrentOwnership,
                E::Correspondence,
                E::VisibleRepairs,
                E::AuthoringAuthority
            ]
        );
        for item in required.requirements() {
            assert!(std::ptr::eq(item.context(), &required));
            assert!(std::ptr::eq(item.source(), evidence.source(item.kind())));
        }
        assert_eq!(plan.ledger().open_repairs(), repairs);
    });
    assert_eq!(f.ledger.open_repairs(), repairs);
    assert_eq!(f.ledger.journal().len(), journal);
    assert_eq!(f.ledger.registration_count(), 2);
}

#[test]
fn whole_ledger_quota_includes_unrelated_owners_before_original_request_refusals() {
    use SlashSpreadRegistrationPlanRefusal as R;
    let mut f = fixture();
    let source = f.first.edge;
    register(&mut f, 200, source);
    register(&mut f, 201, source);
    let unrelated = *f
        .ledger
        .declare_tagged_edges(id(70), &[LocalTag::new(3)])
        .expect("unrelated source")
        .first()
        .expect("one edge");
    register(&mut f, 300, unrelated);
    assert_eq!(f.ledger.registration_count(), 3);
    with_sources(&f, false, |sources| {
        let bad = SlashSpreadRegistrationPlan::new(vec![request(
            300,
            unrelated,
            KeepOriginalGlobalSource,
        )]);
        for (policy, reason) in [
            (
                SlashSpreadRegistrationPlanLimits::new(1, 0, 0),
                R::Sources {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                SlashSpreadRegistrationPlanLimits::new(2, 2, 0),
                R::Registrations {
                    required: 3,
                    allowed: 2,
                },
            ),
            (
                SlashSpreadRegistrationPlanLimits::new(2, 3, 0),
                R::Requests {
                    required: 1,
                    allowed: 0,
                },
            ),
            (
                SlashSpreadRegistrationPlanLimits::new(2, 3, 2),
                R::Source { request: 0 },
            ),
        ] {
            let error = bad
                .bind(sources, &f.ledger, &policy)
                .expect_err("whole quota then exact source");
            assert_eq!(error.refusal(), reason);
            assert!(std::ptr::eq(error.plan(), &bad));
            assert!(std::ptr::eq(error.sources(), sources));
            assert!(std::ptr::eq(error.ledger(), &f.ledger));
            assert!(std::ptr::eq(error.limits(), &policy));
        }
        let policy = SlashSpreadRegistrationPlanLimits::new(2, 3, 2);
        let a = f.ledger.registrations().next().expect("first actual owner");
        for (raw, reason) in [
            (
                SlashSpreadRegistrationPlan::new(vec![]),
                R::Missing { registration: a },
            ),
            (
                SlashSpreadRegistrationPlan::new(vec![request(
                    999,
                    source,
                    KeepOriginalGlobalSource,
                )]),
                R::Registration { request: 0 },
            ),
            (
                SlashSpreadRegistrationPlan::new(vec![
                    request(200, source, KeepOriginalGlobalSource),
                    request(200, source, KeepOriginalGlobalSource),
                ]),
                R::Duplicate {
                    first: 0,
                    second: 1,
                },
            ),
        ] {
            assert_eq!(
                raw.bind(sources, &f.ledger, &policy)
                    .expect_err("request refusal")
                    .refusal(),
                reason
            );
        }
        let only_a =
            SlashSpreadRegistrationPlan::new(vec![request(200, source, KeepOriginalGlobalSource)]);
        let b = f.ledger.registrations().nth(1).expect("second owner");
        assert_eq!(
            only_a
                .bind(sources, &f.ledger, &policy)
                .expect_err("other owner must be covered")
                .refusal(),
            R::Missing { registration: b }
        );
        let all = SlashSpreadRegistrationPlan::new(vec![
            request(200, source, KeepOriginalGlobalSource),
            request(201, source, KeepOriginalGlobalSource),
        ]);
        assert_eq!(
            all.bind(sources, &f.ledger, &policy)
                .expect("unrelated registration needs no source request")
                .requests()
                .count(),
            2
        );
    });
}

#[test]
fn retirement_review_cannot_keep_original_global_source_but_empty_registered_scope_is_valid() {
    let mut f = fixture();
    with_sources(&f, false, |sources| {
        let raw = SlashSpreadRegistrationPlan::new(vec![]);
        let policy = SlashSpreadRegistrationPlanLimits::new(2, 0, 0);
        let plan = raw
            .bind(sources, &f.ledger, &policy)
            .expect("no registered source consumers");
        assert_eq!(plan.requests().len(), 0);
        let evidence =
            SlashSpreadConsumerEvidenceSources::new(id(800), id(801), id(802), id(803), id(804));
        let required = plan.require_consumer_evidence(&evidence);
        assert_eq!(required.requirements().len(), 5); // empty registration subset is no whole-consumer proof
    });
    let source = f.second.edge;
    register(&mut f, 200, source);
    with_sources(&f, true, |sources| {
        let raw =
            SlashSpreadRegistrationPlan::new(vec![request(200, source, KeepOriginalGlobalSource)]);
        let policy = SlashSpreadRegistrationPlanLimits::new(2, 1, 1);
        assert_eq!(
            raw.bind(sources, &f.ledger, &policy)
                .expect_err("contradictory global-source requests")
                .refusal(),
            SlashSpreadRegistrationPlanRefusal::SourceRetirement { request: 0 }
        );
        let raw = SlashSpreadRegistrationPlan::new(vec![request(
            200,
            source,
            RequireCorrespondenceOrVisibleRepair,
        )]);
        assert_eq!(
            raw.bind(sources, &f.ledger, &policy)
                .expect("review remains an obligation")
                .requests()
                .count(),
            1
        );
    });
    assert!(f.ledger.is_live(source));
    assert_eq!(f.ledger.registration_count(), 1);
}
