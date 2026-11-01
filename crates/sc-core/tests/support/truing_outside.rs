//! Actual complete truing source/outcome/evidence/outside parents for direction/placement contracts.
#[path = "truing_domain.rs"]
pub mod domain;
use domain::{
    q,
    roles::{copies::id, with_roles},
    with_domain, with_partial,
};
use sc_core::{construction::*, name::MachineToken, ontology::*};

pub fn with_outside(
    side: SeamSideId,
    from: Param,
    to: Param,
    intent: TruingOutcomeIntent,
    raw: TruingOutsideRoles,
    run: impl FnOnce(AdmittedTruingOutsideRoles<'_>),
) {
    let build = |original: BoundTruingEdgeRoles<'_>| with_bound_outside(original, intent, raw, run);
    if side == SeamSideId::A && from == Param::START && to == Param::END {
        with_roles(false, build);
    } else if from == q(1, 4) && to == q(3, 4) {
        with_partial(side, build);
    } else {
        with_domain(side, from, to, build);
    }
}
pub fn with_bound_outside(
    original: BoundTruingEdgeRoles<'_>,
    intent: TruingOutcomeIntent,
    raw: TruingOutsideRoles,
    run: impl FnOnce(AdmittedTruingOutsideRoles<'_>),
) {
    let catalogue = EdgeOutputRoleCatalog::new(
        id(1000),
        [(9, "refit"), (2, "prefix"), (5, "suffix"), (7, "inactive")]
            .into_iter()
            .map(|(tag, key)| {
                EdgeOutputRoleDefinition::new(
                    LocalTag::new(tag),
                    MachineToken::new(key).expect("key"),
                )
            })
            .collect(),
        vec![LocalTag::new(5), LocalTag::new(9), LocalTag::new(2)],
    );
    let cat_limits = EdgeOutputRoleCatalogLimits::new(4, 3, 25);
    let catalog = catalogue
        .admit(cat_limits)
        .expect("complete original metadata");
    let roles = original
        .copies()
        .bind_edge_roles(&catalog, cat_limits)
        .expect("actual original parents");
    let candidates = if intent == TruingOutcomeIntent::Refit {
        vec![TruingEdgeCandidate::new(
            LocalTag::new(9),
            roles.copies().source().source_side().range,
        )]
    } else {
        vec![]
    };
    let n = candidates.len();
    let manifest = TruingEdgeManifest::new(id(1000), candidates);
    let edges = manifest
        .admit(&roles, TruingEdgeManifestLimits::new(n, n * 2))
        .expect("selected attributions");
    let domain = edges
        .bind_source_domain(TruingSourceDomainLimits::new(n))
        .expect("actual held domain");
    let outcome = domain
        .require_outcome(intent, TruingSourceCoverageLimits::new(n, n))
        .expect("explicit outcome");
    let locks = outcome
        .require_locks(TruingLockLimits::new(2, 1))
        .expect("bounded original locks");
    let source = id(7000);
    let policy = TruingEvidencePolicy::new(
        TruingContextSources::new(source, source, source),
        TruingFitSources::new(source, source, source, source),
        TruingEffectSources::new(source, source, source, source),
    );
    let evidence = locks.require_evidence(&policy);
    let outside = raw
        .admit(&evidence, TruingOutsideRoleLimits::new(n, cat_limits))
        .expect("explicit actual outside fields");
    run(outside);
}
