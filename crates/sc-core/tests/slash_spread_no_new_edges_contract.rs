//! Explicit zero-candidate declarations reuse actual reconciliation;no physical no-change proof.
#![allow(clippy::expect_used)]
use sc_core::{construction::*, ontology::*};
#[path = "support/slash_sources.rs"]
mod sources;
use sources::*;

fn edge_limits(n: usize) -> SlashSpreadEdgeManifestLimits {
    SlashSpreadEdgeManifestLimits::new(n, catalogue_limits())
}
fn retained(f: &Fixture, keep_construction: bool) -> SlashSpreadContours {
    let base = contours(f);
    SlashSpreadContours::new(
        base.boundary()
            .iter()
            .copied()
            .filter(|usage| matches!(usage, SlashSpreadContourEdge::Retained(_)))
            .collect(),
        vec![],
        base.construction_lines()
            .iter()
            .copied()
            .filter(|usage| {
                keep_construction && matches!(usage, SlashSpreadContourEdge::Retained(_))
            })
            .collect(),
    )
}
fn with_zero<T>(
    f: &Fixture,
    raw: &SlashSpreadContours,
    fullness: SlashSpreadFullnessIntent,
    run: impl FnOnce(&RequiredSlashSpreadEffect<'_>) -> T,
) -> T {
    let original = intent(f);
    let intent = SlashSpreadOutputIntent::new(
        original.input().clone(),
        *original.boundary_policy(),
        fullness,
    );
    let raw_catalog = catalog();
    let catalog = raw_catalog
        .admit(catalogue_limits())
        .expect("actual catalogue");
    let roles = intent
        .bind_edge_roles(&catalog, catalogue_limits())
        .expect("operation");
    let raw_edges = SlashSpreadEdgeManifest::new(id(10), vec![]);
    assert_eq!(
        raw_edges
            .admit(&roles, edge_limits(0))
            .expect_err("ordinary primitive remains nonempty")
            .refusal(),
        SlashSpreadEdgeManifestRefusal::Empty
    );
    let edges = raw_edges
        .admit_no_new_edges(&roles, edge_limits(0))
        .expect("explicit no-new-edge declaration");
    assert!(std::ptr::eq(edges.manifest(), &raw_edges));
    assert!(std::ptr::eq(edges.bound_roles(), &roles));
    assert_eq!(edges.outputs().len(), 0);
    assert_eq!(edges.workspace_entries(), 0);
    assert_eq!(
        roles.catalog().catalog().active_tags(),
        &[LocalTag::new(2), LocalTag::new(9)]
    );
    let baseline = limits();
    let m = raw.boundary().len()
        + raw.holes().iter().map(Vec::len).sum::<usize>()
        + raw.construction_lines().len();
    let contour_policy = SlashSpreadContourLimits::new(
        0,
        baseline.source_edges(),
        baseline.source_holes(),
        raw.holes().len(),
        m,
    );
    let contours = raw
        .admit(&edges, &f.piece, &contour_policy)
        .expect("all original contour rules");
    let raw_sources = SlashSpreadSourcePlan::new(vec![
        SlashSpreadSourceDisposition::new(f.first.edge, SlashSpreadGlobalSourceIntent::Preserve),
        SlashSpreadSourceDisposition::new(f.second.edge, SlashSpreadGlobalSourceIntent::Preserve),
    ]);
    let source_policy = SlashSpreadSourcePlanLimits::new(0, 2, m, 2);
    let sources = raw_sources
        .bind(&contours, &source_policy)
        .expect("whole source identities");
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
    let consumer_sources =
        SlashSpreadConsumerEvidenceSources::new(id(800), id(801), id(802), id(803), id(804));
    let consumers = registrations.require_consumer_evidence(&consumer_sources);
    let boundary_request = SlashSpreadBoundaryRequirement::new(
        id(920),
        id(921),
        SlashSpreadBoundaryEvidenceRequest::StraightBridges {
            incidence: id(910),
            closure: id(911),
        },
    );
    let boundary = consumers
        .require_boundary(&boundary_request)
        .expect("boundary requests,not evidence");
    let metadata_request = SlashSpreadPieceMetadataRequirement::preserve_source(
        SlashSpreadFoldIntent::NoFold,
        id(930),
        id(931),
    );
    let metadata = boundary
        .preserve_piece_metadata(&metadata_request)
        .expect("original canonical metadata");
    let copy_plan = CutPlan::new(
        vec![CutCopyDefinition {
            id: id(410),
            piece: f.piece.id(),
            orientation: CopyOrientation::Authored,
        }],
        core::slice::from_ref(&f.piece),
    )
    .expect("actual copy inventory");
    let copy_policy = SlashSpreadCopyLimits::new(1);
    let copies = metadata
        .preserve_physical_copies(&copy_plan, &copy_policy)
        .expect("every actual copy");
    let effect_request = SlashSpreadEffectRequirement::new(id(940), id(941), id(942));
    let effect = copies.require_attributed_effect(&effect_request);
    run(&effect)
}

#[test]
fn zero_candidate_declaration_reuses_the_complete_pipeline_and_does_not_retire_active_or_issued_roles(
) {
    let mut f = fixture();
    let old_outputs = f
        .ledger
        .declare_tagged_edges(id(10), &[LocalTag::new(2), LocalTag::new(9)])
        .expect("previously issued actual role edges");
    f.ledger
        .register(id(500), f.first.edge, Param::START)
        .expect("owner A");
    f.ledger
        .register(id(501), f.first.edge, Param::END)
        .expect("owner B");
    let journal = f.ledger.journal().len();
    let registrations = f.ledger.registrations().collect::<Vec<_>>();
    let original = f.piece.clone();
    with_zero(
        &f,
        &retained(&f, true),
        SlashSpreadFullnessIntent::Unchanged,
        |effect| {
            let copies = effect.copies();
            let contours = copies
                .metadata()
                .boundary()
                .consumers()
                .registration_plan()
                .sources()
                .contours();
            assert_eq!(contours.edges().outputs().len(), 0);
            assert_eq!(contours.uses().len(), 2);
            assert_eq!(contours.workspace_entries(), 5);
            assert_eq!(
                copies.copies().map(|copy| copy.id()).collect::<Vec<_>>(),
                vec![id(410)]
            );
            assert!(std::ptr::eq(copies.metadata().source_piece(), &f.piece));
            assert_eq!(
                effect.requested_fullness(),
                &SlashSpreadFullnessIntent::Unchanged
            );
            let policy = SlashSpreadReferenceQueryLimits::new(0, 2, 2);
            let query = contours
                .query_source_reference(
                    EdgeAnchor {
                        edge: f.first.edge,
                        param: Param::START,
                    },
                    &policy,
                )
                .expect("original source scope");
            assert_eq!(query.kind(), SlashSpreadReferenceMatchKind::Unique);
            assert_eq!(query.maximum_match_slots(), 1);
            assert_eq!(
                query.matches().next().expect("retained identity").edge(),
                f.first.edge
            );
            assert_eq!(
                copies
                    .metadata()
                    .boundary()
                    .consumers()
                    .requirements()
                    .len(),
                5
            );
        },
    );
    assert_eq!(f.ledger.journal().len(), journal);
    assert_eq!(f.ledger.registrations().collect::<Vec<_>>(), registrations);
    assert_eq!(f.piece, original);
    assert!(old_outputs.iter().all(|edge| f.ledger.is_live(*edge)));
}

#[test]
fn whole_quotas_then_exact_population_and_operation_preserve_original_inputs() {
    use SlashSpreadEdgeManifestRefusal as R;
    let f = fixture();
    let intent = intent(&f);
    let raw_catalog = catalog();
    let catalog = raw_catalog.admit(catalogue_limits()).expect("catalogue");
    let roles = intent
        .bind_edge_roles(&catalog, catalogue_limits())
        .expect("operation");
    let empty = SlashSpreadEdgeManifest::new(id(999), vec![]);
    for (limits, reason) in [
        (
            SlashSpreadEdgeManifestLimits::new(0, EdgeOutputRoleCatalogLimits::new(1, 0, 0)),
            R::Definitions {
                required: 2,
                allowed: 1,
            },
        ),
        (
            SlashSpreadEdgeManifestLimits::new(0, EdgeOutputRoleCatalogLimits::new(2, 1, 0)),
            R::Active {
                required: 2,
                allowed: 1,
            },
        ),
        (
            SlashSpreadEdgeManifestLimits::new(0, EdgeOutputRoleCatalogLimits::new(2, 2, 6)),
            R::KeyBytes {
                required: 7,
                allowed: 6,
            },
        ),
        (edge_limits(0), R::Operation),
    ] {
        let error = empty
            .admit_no_new_edges(&roles, limits)
            .expect_err("complete quota then owner");
        assert_eq!(error.refusal(), reason);
        assert!(std::ptr::eq(error.manifest(), &empty));
        assert!(std::ptr::eq(error.bound_roles(), &roles));
        assert_eq!(error.limits(), limits);
    }
    assert_eq!(
        empty
            .admit(&roles, edge_limits(0))
            .expect_err("ordinary empty precedes owner")
            .refusal(),
        R::Empty
    );
    let nonempty = manifest(&f);
    assert_eq!(
        nonempty
            .admit_no_new_edges(&roles, edge_limits(0))
            .expect_err("whole population quota first")
            .refusal(),
        R::Candidates {
            required: 2,
            allowed: 0
        }
    );
    assert_eq!(
        nonempty
            .admit_no_new_edges(&roles, edge_limits(2))
            .expect_err("no candidate may be dropped")
            .refusal(),
        R::CandidatesPresent
    );
    assert_eq!(
        nonempty
            .admit(&roles, edge_limits(2))
            .expect("ordinary nonempty declaration")
            .outputs()
            .len(),
        2
    );
    let empty_catalog = EdgeOutputRoleCatalog::new(id(10), vec![], vec![]);
    let policy = EdgeOutputRoleCatalogLimits::new(0, 0, 0);
    let catalog = empty_catalog
        .admit(policy)
        .expect("empty canonical metadata");
    let roles = intent
        .bind_edge_roles(&catalog, policy)
        .expect("empty roles bound to actual operation");
    let raw = SlashSpreadEdgeManifest::new(id(10), vec![]);
    assert_eq!(
        raw.admit_no_new_edges(&roles, SlashSpreadEdgeManifestLimits::new(0, policy))
            .expect("explicit empty initial declaration")
            .outputs()
            .len(),
        0
    );
}

#[test]
fn retained_only_contour_change_is_not_unchanged_source_or_a_registration_withdrawal() {
    let mut f = fixture();
    f.ledger
        .register(id(500), f.second.edge, Param::START)
        .expect("actual held source point");
    let registrations = f.ledger.registrations().collect::<Vec<_>>();
    with_zero(
        &f,
        &retained(&f, false),
        SlashSpreadFullnessIntent::Added,
        |effect| {
            let contours = effect
                .copies()
                .metadata()
                .boundary()
                .consumers()
                .registration_plan()
                .sources()
                .contours();
            assert_eq!(contours.edges().outputs().len(), 0);
            assert_eq!(contours.uses().len(), 1);
            let policy = SlashSpreadReferenceQueryLimits::new(0, 2, 1);
            let query = contours
                .query_source_reference(
                    EdgeAnchor {
                        edge: f.second.edge,
                        param: Param::START,
                    },
                    &policy,
                )
                .expect("known original scope");
            assert_eq!(query.kind(), SlashSpreadReferenceMatchKind::Gap);
            assert_eq!(
                effect.requested_fullness(),
                &SlashSpreadFullnessIntent::Added
            ); // zero candidate count leaves the authored effect request untouched
            assert_eq!(
                effect
                    .copies()
                    .metadata()
                    .boundary()
                    .consumers()
                    .requirements()
                    .len(),
                5
            );
        },
    );
    assert_eq!(f.ledger.registrations().collect::<Vec<_>>(), registrations);
    assert!(f.ledger.is_live(f.second.edge));
    assert!(f
        .ledger
        .resolve_registered(id(500), f.second.edge, Param::START)
        .expect("actual registration remains")
        .resolved()
        .is_some());
}

#[test]
fn no_new_edges_cannot_bypass_nonempty_boundary_output_membership_or_source_self_cycle() {
    let mut f = fixture();
    let intent = intent(&f);
    let raw_catalog = catalog();
    let catalog = raw_catalog.admit(catalogue_limits()).expect("catalogue");
    let roles = intent
        .bind_edge_roles(&catalog, catalogue_limits())
        .expect("operation");
    let raw = SlashSpreadEdgeManifest::new(id(10), vec![]);
    let edges = raw
        .admit_no_new_edges(&roles, edge_limits(0))
        .expect("explicit zero population");
    let policy = SlashSpreadContourLimits::new(0, 2, 0, 1, 6);
    let empty = SlashSpreadContours::new(vec![], vec![], vec![]);
    assert_eq!(
        empty
            .admit(&edges, &f.piece, &policy)
            .expect_err("not an empty Piece")
            .refusal(),
        SlashSpreadContourRefusal::EmptyBoundary
    );
    assert_eq!(
        contours(&f)
            .admit(&edges, &f.piece, &policy)
            .expect_err("cannot name an undeclared output")
            .refusal(),
        SlashSpreadContourRefusal::MissingOutput {
            location: SlashSpreadContourLocation::Boundary(1),
            tag: LocalTag::new(9)
        }
    );
    let own = *f
        .ledger
        .declare_tagged_edges(id(10), &[LocalTag::new(7)])
        .expect("actual same-operation edge")
        .first()
        .expect("one");
    let mut definition = f.piece.definition().clone();
    definition.construction_lines = vec![DirectedEdge {
        edge: own,
        direction: Direction::Original,
    }];
    let source = Piece::new(definition, &f.ledger)
        .expect("born-valid source with definite operation self-input");
    let target = retained(&f, false);
    assert_eq!(
        target
            .admit(&edges, &source, &policy)
            .expect_err("source self-cycle still checked")
            .refusal(),
        SlashSpreadContourRefusal::SelfSource { source_edge: 1 }
    );
}
