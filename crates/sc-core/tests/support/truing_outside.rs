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
    with_partitioned_outside(original, intent, raw, None, run);
}
pub fn with_partitioned_outside(
    original: BoundTruingEdgeRoles<'_>,
    intent: TruingOutcomeIntent,
    raw: TruingOutsideRoles,
    split: Option<Param>,
    run: impl FnOnce(AdmittedTruingOutsideRoles<'_>),
) {
    let mut definitions = vec![(9, "refit"), (2, "prefix"), (5, "suffix"), (7, "inactive")];
    let mut active = vec![LocalTag::new(5), LocalTag::new(9), LocalTag::new(2)];
    if split.is_some() {
        definitions.push((11, "refit_second"));
        active.push(LocalTag::new(11));
    }
    let catalogue = EdgeOutputRoleCatalog::new(
        id(1000),
        definitions
            .into_iter()
            .map(|(tag, key)| {
                EdgeOutputRoleDefinition::new(
                    LocalTag::new(tag),
                    MachineToken::new(key).expect("key"),
                )
            })
            .collect(),
        active,
    );
    let cat_limits = if split.is_some() {
        EdgeOutputRoleCatalogLimits::new(5, 4, 37)
    } else {
        EdgeOutputRoleCatalogLimits::new(4, 3, 25)
    };
    let catalog = catalogue
        .admit(cat_limits)
        .expect("complete original metadata");
    let roles = original
        .copies()
        .bind_edge_roles(&catalog, cat_limits)
        .expect("actual original parents");
    let candidates = if intent == TruingOutcomeIntent::Refit {
        let range = roles.copies().source().source_side().range;
        if let Some(split) = split {
            vec![
                TruingEdgeCandidate::new(
                    LocalTag::new(11),
                    EdgeRange::new(range.edge(), split, range.to())
                        .expect("second original interval"),
                ),
                TruingEdgeCandidate::new(
                    LocalTag::new(9),
                    EdgeRange::new(range.edge(), range.from(), split)
                        .expect("first original interval"),
                ),
            ]
        } else {
            vec![TruingEdgeCandidate::new(LocalTag::new(9), range)]
        }
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
