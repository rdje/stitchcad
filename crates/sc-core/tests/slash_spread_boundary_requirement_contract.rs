//! Matching explicit boundary requests does not select trim geometry or discharge actual repairs.
#![allow(clippy::expect_used)]
use sc_core::{construction::*, ontology::*};
#[path = "support/slash_sources.rs"]
mod sources;
use sources::*;

fn with_consumers<T>(
    f: &Fixture,
    policy: SlashSpreadBoundaryPolicy,
    run: impl FnOnce(&RequiredSlashSpreadConsumerEvidence<'_>) -> T,
) -> T {
    let original = intent(f);
    let intent =
        SlashSpreadOutputIntent::new(original.input().clone(), policy, *original.fullness());
    let raw_catalog = catalog();
    let catalog = raw_catalog.admit(catalogue_limits()).expect("catalogue");
    let bound = intent
        .bind_edge_roles(&catalog, catalogue_limits())
        .expect("operation");
    let base = manifest(f);
    let raw_edges = if policy == SlashSpreadBoundaryPolicy::StraightBridges {
        base
    } else {
        SlashSpreadEdgeManifest::new(
            id(10),
            vec![
                *base.candidates().first().expect("first source"),
                SlashSpreadEdgeCandidate::new(
                    LocalTag::new(2),
                    SlashSpreadEdgeOrigin::Source {
                        range: DirectedRange {
                            range: EdgeRange::whole(f.second.edge),
                            direction: Direction::Reversed,
                        },
                        motion: SlashSpreadSourceMotion::Stationary,
                    },
                ),
            ],
        )
    };
    let edges = raw_edges
        .admit(
            &bound,
            SlashSpreadEdgeManifestLimits::new(2, catalogue_limits()),
        )
        .expect("policy-compatible provenance");
    let raw_contours = contours(f);
    let contour_policy = limits();
    let contours = raw_contours
        .admit(&edges, &f.piece, &contour_policy)
        .expect("contour declaration");
    let raw_sources = SlashSpreadSourcePlan::new(vec![
        SlashSpreadSourceDisposition::new(f.first.edge, SlashSpreadGlobalSourceIntent::Preserve),
        SlashSpreadSourceDisposition::new(f.second.edge, SlashSpreadGlobalSourceIntent::Preserve),
    ]);
    let source_policy = SlashSpreadSourcePlanLimits::new(2, 2, 6, 2);
    let sources = raw_sources
        .bind(&contours, &source_policy)
        .expect("all source identities");
    let raw_registrations = SlashSpreadRegistrationPlan::new(
        f.ledger
            .registrations()
            .map(|registration| {
                SlashSpreadRegistrationDisposition::new(
                    registration.owner(),
                    EdgeAnchor {
                        edge: registration.edge(),
                        param: registration.param(),
                    },
                    SlashSpreadRegistrationIntent::RequireCorrespondenceOrVisibleRepair,
                )
            })
            .collect(),
    );
    let registration_policy = SlashSpreadRegistrationPlanLimits::new(2, 2, 2);
    let registrations = raw_registrations
        .bind(&sources, &f.ledger, &registration_policy)
        .expect("all actual source registrations");
    let evidence =
        SlashSpreadConsumerEvidenceSources::new(id(900), id(901), id(902), id(903), id(904));
    let consumers = registrations.require_consumer_evidence(&evidence);
    run(&consumers)
}
fn evidence(policy: SlashSpreadBoundaryPolicy) -> SlashSpreadBoundaryEvidenceRequest {
    match policy {
        SlashSpreadBoundaryPolicy::StraightBridges => {
            SlashSpreadBoundaryEvidenceRequest::StraightBridges {
                incidence: id(910),
                closure: id(911),
            }
        }
        SlashSpreadBoundaryPolicy::TrimOverlaps => {
            SlashSpreadBoundaryEvidenceRequest::TrimOverlaps {
                arrangement: id(912),
                unique_selection: id(913),
            }
        }
    }
}

#[test]
fn both_actual_boundary_policies_require_common_and_matching_specific_sources() {
    for policy in [
        SlashSpreadBoundaryPolicy::StraightBridges,
        SlashSpreadBoundaryPolicy::TrimOverlaps,
    ] {
        let f = fixture();
        let journal = f.ledger.journal().len();
        with_consumers(&f, policy, |consumers| {
            let raw = SlashSpreadBoundaryRequirement::new(id(920), id(921), evidence(policy));
            let required = consumers
                .require_boundary(&raw)
                .expect("actual boundary mode");
            assert!(std::ptr::eq(required.requirement(), &raw));
            assert!(std::ptr::eq(required.consumers(), consumers));
            assert!(std::ptr::eq(
                required.policy(),
                consumers
                    .registration_plan()
                    .sources()
                    .contours()
                    .edges()
                    .bound_roles()
                    .intent()
                    .boundary_policy()
            ));
            assert_eq!(required.policy(), &policy);
            assert_eq!(required.requirement().supported_domain(), &id(920));
            assert_eq!(required.requirement().source_coverage(), &id(921));
            assert_eq!(required.requirement().boundary(), &evidence(policy));
            assert_eq!(required.consumers().requirements().len(), 5);
            assert_eq!(required.consumers().registration_plan().requests().len(), 0);
            let other = match policy {
                SlashSpreadBoundaryPolicy::StraightBridges => {
                    SlashSpreadBoundaryPolicy::TrimOverlaps
                }
                SlashSpreadBoundaryPolicy::TrimOverlaps => {
                    SlashSpreadBoundaryPolicy::StraightBridges
                }
            };
            let wrong = SlashSpreadBoundaryRequirement::new(id(920), id(921), evidence(other));
            let error = consumers
                .require_boundary(&wrong)
                .expect_err("foreign boundary mode");
            assert!(std::ptr::eq(error.requirement(), &wrong));
            assert!(std::ptr::eq(error.consumers(), consumers));
            assert!(std::ptr::eq(error.required_policy(), required.policy()));
        });
        assert_eq!(f.ledger.journal().len(), journal);
        assert_eq!(f.ledger.registration_count(), 0);
        assert!(f.ledger.is_live(f.first.edge) && f.ledger.is_live(f.second.edge));
    }
}

#[test]
fn trim_coverage_and_selection_requests_neither_hide_actual_repairs_nor_create_geometry() {
    let mut f = fixture();
    let source = f.first.edge;
    f.ledger
        .register(id(200), source, Param::START)
        .expect("owner A");
    f.ledger
        .register(id(201), source, Param::END)
        .expect("owner B");
    let mut ids = DeterministicIdGenerator::from_start(1000);
    f.ledger
        .delete(&mut ids, source)
        .expect("later global deletion");
    let repairs = f.ledger.open_repairs();
    assert_eq!(repairs.len(), 2);
    let registrations = f.ledger.registrations().collect::<Vec<_>>();
    let journal = f.ledger.journal().len();
    with_consumers(&f, SlashSpreadBoundaryPolicy::TrimOverlaps, |consumers| {
        let raw = SlashSpreadBoundaryRequirement::new(
            id(920),
            id(920),
            SlashSpreadBoundaryEvidenceRequest::TrimOverlaps {
                arrangement: id(920),
                unique_selection: id(920),
            },
        );
        let required = consumers
            .require_boundary(&raw)
            .expect("four requested scopes,not four accepted proofs");
        assert_eq!(
            required.requirement().supported_domain(),
            required.requirement().source_coverage()
        );
        assert_eq!(
            required
                .consumers()
                .registration_plan()
                .ledger()
                .open_repairs(),
            repairs
        );
        assert_eq!(
            required
                .consumers()
                .registration_plan()
                .sources()
                .sources()
                .len(),
            2
        );
        for output in required
            .consumers()
            .registration_plan()
            .sources()
            .contours()
            .edges()
            .outputs()
        {
            assert!(!f.ledger.is_live(output.logical_output()));
        }
    });
    assert_eq!(f.ledger.open_repairs(), repairs);
    assert_eq!(f.ledger.registrations().collect::<Vec<_>>(), registrations);
    assert_eq!(f.ledger.journal().len(), journal);
}
