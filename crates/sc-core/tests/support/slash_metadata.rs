//! Whole actual metadata boundary fixture shared by metadata and physical-copy contracts.
use super::sources::*;
use sc_core::construction::*;
pub fn with_boundary<T>(
    f: &Fixture,
    raw_contours: &SlashSpreadContours,
    raw_edges: &SlashSpreadEdgeManifest,
    run: impl FnOnce(&RequiredSlashSpreadBoundary<'_>) -> T,
) -> T {
    let intent = intent(f);
    let raw_catalog = catalog();
    let catalog = raw_catalog.admit(catalogue_limits()).expect("catalogue");
    let bound = intent
        .bind_edge_roles(&catalog, catalogue_limits())
        .expect("operation");
    let edges = raw_edges
        .admit(
            &bound,
            SlashSpreadEdgeManifestLimits::new(2, catalogue_limits()),
        )
        .expect("edges");
    let baseline = limits();
    let m = raw_contours.boundary().len()
        + raw_contours.holes().iter().map(Vec::len).sum::<usize>()
        + raw_contours.construction_lines().len();
    let contour_policy = SlashSpreadContourLimits::new(
        baseline.candidates(),
        baseline.source_edges(),
        baseline.source_holes(),
        raw_contours.holes().len(),
        m,
    );
    let contours = raw_contours
        .admit(&edges, &f.piece, &contour_policy)
        .expect("contours");
    let raw_sources = SlashSpreadSourcePlan::new(vec![
        SlashSpreadSourceDisposition::new(f.first.edge, SlashSpreadGlobalSourceIntent::Preserve),
        SlashSpreadSourceDisposition::new(f.second.edge, SlashSpreadGlobalSourceIntent::Preserve),
    ]);
    let source_policy = SlashSpreadSourcePlanLimits::new(2, 2, m, 2);
    let sources = raw_sources
        .bind(&contours, &source_policy)
        .expect("source identities");
    let raw_registrations = SlashSpreadRegistrationPlan::new(vec![]);
    let registration_policy = SlashSpreadRegistrationPlanLimits::new(2, 0, 0);
    let registrations = raw_registrations
        .bind(&sources, &f.ledger, &registration_policy)
        .expect("empty actual registered subset");
    let evidence =
        SlashSpreadConsumerEvidenceSources::new(id(800), id(801), id(802), id(803), id(804));
    let consumers = registrations.require_consumer_evidence(&evidence);
    let request = SlashSpreadBoundaryRequirement::new(
        id(920),
        id(921),
        SlashSpreadBoundaryEvidenceRequest::StraightBridges {
            incidence: id(910),
            closure: id(911),
        },
    );
    let boundary = consumers
        .require_boundary(&request)
        .expect("bridge request");
    run(&boundary)
}
