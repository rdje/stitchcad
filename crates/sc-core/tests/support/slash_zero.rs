//! Actual zero-candidate pipeline fixture;source/hole allowances and requests cover the actual Piece.
use super::sources::*;
use sc_core::{construction::*, ontology::*};
pub fn edge_limits(n: usize) -> SlashSpreadEdgeManifestLimits {
    SlashSpreadEdgeManifestLimits::new(n, catalogue_limits())
}
pub fn with_zero<T>(
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
    let source_allowance = f.piece.edges().count().max(baseline.source_edges());
    let m = raw.boundary().len()
        + raw.holes().iter().map(Vec::len).sum::<usize>()
        + raw.construction_lines().len();
    let contour_policy = SlashSpreadContourLimits::new(
        0,
        source_allowance,
        f.piece
            .definition()
            .holes
            .len()
            .max(baseline.source_holes()),
        raw.holes().len(),
        m,
    );
    let contours = raw
        .admit(&edges, &f.piece, &contour_policy)
        .expect("all original contour rules");
    let mut seen = std::collections::BTreeSet::new();
    let raw_sources = SlashSpreadSourcePlan::new(
        f.piece
            .edges()
            .filter(|edge| seen.insert(edge.edge))
            .map(|edge| {
                SlashSpreadSourceDisposition::new(
                    edge.edge,
                    SlashSpreadGlobalSourceIntent::Preserve,
                )
            })
            .collect(),
    );
    let source_policy =
        SlashSpreadSourcePlanLimits::new(0, source_allowance, m, raw_sources.sources().len());
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
    let registration_policy =
        SlashSpreadRegistrationPlanLimits::new(raw_sources.sources().len(), 2, 2);
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
