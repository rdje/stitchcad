//! Complete proposed occurrence coverage and outstanding explicit additional/omission requirements.
#![allow(clippy::expect_used)]
#[allow(dead_code)]
#[path = "support/join_targets.rs"]
mod common;
use common::edges::with_partitioned_source_actions;
use common::{id, proposed, rows, with_coverage};
use sc_core::{construction::*, ontology::*};
fn with_targets(
    uses: Vec<JoinConstructionUse>,
    intent: JoinAdditionalConstructionIntent,
    run: impl FnOnce(DeclaredJoinConstructionTargets<'_>),
) {
    with_coverage(uses, |coverage| {
        let u = coverage.groups().sources().uses().use_count();
        let raw = JoinConstructionTargetManifest::new(id(1000), rows(), intent);
        let targets = raw
            .admit(&coverage, JoinConstructionTargetLimits::new(4, 8, u, 20))
            .expect("authentic complete retained assignments");
        run(targets);
    });
}
fn extra_uses() -> Vec<JoinConstructionUse> {
    let mut values = proposed();
    // First extra repeats a mapped tag AND traversal;occurrence ownership still cannot be inferred.
    values.push(JoinConstructionUse::new(
        LocalTag::new(4),
        Direction::Original,
    ));
    values.push(JoinConstructionUse::new(
        LocalTag::new(6),
        Direction::Reversed,
    ));
    values
}
#[test]
fn complete_source_mapped_scope_lends_whole_parents_without_additional_or_evidence_claim() {
    with_targets(
        proposed(),
        JoinAdditionalConstructionIntent::SourceMappedOnly,
        |targets| {
            let limits = JoinConstructionTargetCoverageLimits::new(8, 8);
            let required = targets
                .require_target_coverage(limits)
                .expect("all actual occurrences source-mapped");
            assert!(std::ptr::eq(required.targets(), &targets));
            assert_eq!(required.limits(), limits);
            assert_eq!((limits.mappings(), limits.uses()), (8, 8));
            assert_eq!(
                (
                    required.use_count(),
                    required.mapped_count(),
                    required.additional_count()
                ),
                (8, 8, 0)
            );
            assert_eq!(required.additional_uses().count(), 0);
            assert!(required.additional_evidence_source().is_none());
        },
    );
}
#[test]
fn whole_resources_precede_first_actual_unrequested_additional_use_even_for_repeated_tags() {
    with_targets(
        extra_uses(),
        JoinAdditionalConstructionIntent::SourceMappedOnly,
        |targets| {
            use JoinConstructionTargetCoverageRefusal as R;
            for (limits, cause) in [
                (
                    JoinConstructionTargetCoverageLimits::new(7, 0),
                    R::Mappings {
                        required: 8,
                        allowed: 7,
                    },
                ),
                (
                    JoinConstructionTargetCoverageLimits::new(8, 9),
                    R::Uses {
                        required: 10,
                        allowed: 9,
                    },
                ),
                (
                    JoinConstructionTargetCoverageLimits::new(8, 10),
                    R::AdditionalUse { index: 8 },
                ),
            ] {
                let error = targets
                    .require_target_coverage(limits)
                    .expect_err("complete policies then first original extra occurrence");
                assert_eq!(error.refusal(), cause);
                assert_eq!(error.limits(), limits);
                assert!(std::ptr::eq(error.targets(), &targets));
                if matches!(cause, R::AdditionalUse { .. }) {
                    let actual = error.refused_use().expect("actual first extra occurrence");
                    assert_eq!(actual.index(), 8);
                    let raw = targets
                        .coverage()
                        .groups()
                        .sources()
                        .uses()
                        .manifest()
                        .uses();
                    assert!(std::ptr::eq(
                        actual.declared(),
                        raw.get(8).expect("actual original field")
                    ));
                    assert_eq!(
                        actual.declared(),
                        raw.get(4).expect("equal mapped tag and traversal")
                    );
                    assert!(targets.mapping_for_use(actual.index()).is_none());
                    assert!(targets.mapping_for_use(4).is_some());
                } else {
                    assert!(error.refused_use().is_none());
                }
            }
        },
    );
}
#[test]
fn explicit_additional_content_retains_all_actual_occurrences_and_original_evidence_field() {
    with_targets(
        extra_uses(),
        JoinAdditionalConstructionIntent::AllowAdditional {
            evidence_source: id(0),
        },
        |targets| {
            let required = targets
                .require_target_coverage(JoinConstructionTargetCoverageLimits::new(8, 10))
                .expect("explicit additional-content request");
            assert_eq!(
                (
                    required.use_count(),
                    required.mapped_count(),
                    required.additional_count()
                ),
                (10, 8, 2)
            );
            assert_eq!(
                required
                    .additional_uses()
                    .map(|u| u.index())
                    .collect::<Vec<_>>(),
                [8, 9]
            );
            assert_eq!(
                required
                    .additional_uses()
                    .rev()
                    .map(|u| u.index())
                    .collect::<Vec<_>>(),
                [9, 8]
            );
            let evidence_source = match targets.manifest().additional() {
                JoinAdditionalConstructionIntent::AllowAdditional { evidence_source } => {
                    Some(evidence_source)
                }
                JoinAdditionalConstructionIntent::SourceMappedOnly => None,
            }
            .expect("actual explicit intent");
            assert!(std::ptr::eq(
                required
                    .additional_evidence_source()
                    .expect("actual authored field"),
                evidence_source
            ));
            assert_eq!(*evidence_source, id(0)); // No accepted evidence/kind/state/currentness follows.
            for u in required.additional_uses() {
                let raw = targets
                    .coverage()
                    .groups()
                    .sources()
                    .uses()
                    .manifest()
                    .uses()
                    .get(u.index())
                    .expect("actual proposed field");
                assert!(std::ptr::eq(u.declared(), raw));
            }
        },
    );
}
#[test]
fn explicit_no_retain_scope_still_requires_additional_intent_and_preserves_all_omission_requests() {
    for extra in [false, true] {
        for intent in [
            JoinAdditionalConstructionIntent::SourceMappedOnly,
            JoinAdditionalConstructionIntent::AllowAdditional {
                evidence_source: id(0),
            },
        ] {
            let uses = if extra {
                vec![JoinConstructionUse::new(
                    LocalTag::new(6),
                    Direction::Original,
                )]
            } else {
                vec![]
            };
            with_partitioned_source_actions(
                uses,
                |_, _| JoinConstructionSourceIntent::Omit {
                    evidence_source: id(0),
                },
                |sources| {
                    let group_raw = JoinConstructionGroupManifest::new(id(1000), vec![]);
                    let groups = group_raw
                        .admit(&sources, JoinConstructionGroupLimits::new(4, 0, 0, 0))
                        .expect("explicit no-Retain scope");
                    let coverage = groups
                        .require_source_coverage(JoinConstructionCoverageLimits::new(0, 0))
                        .expect("vacuous retained partitions");
                    let raw = JoinConstructionTargetManifest::new(id(1000), vec![], intent);
                    let u = usize::from(extra);
                    let targets = raw
                        .admit(&coverage, JoinConstructionTargetLimits::new(0, 0, u, 0))
                        .expect("explicit empty retained assignments");
                    let result = targets
                        .require_target_coverage(JoinConstructionTargetCoverageLimits::new(0, u));
                    if extra && matches!(intent, JoinAdditionalConstructionIntent::SourceMappedOnly)
                    {
                        let error = result.expect_err("actual extra still needs explicit intent");
                        assert_eq!(
                            error.refusal(),
                            JoinConstructionTargetCoverageRefusal::AdditionalUse { index: 0 }
                        );
                    } else {
                        let required =
                            result.expect("native scope with explicit outstanding requests");
                        assert_eq!(
                            (
                                required.use_count(),
                                required.mapped_count(),
                                required.additional_count()
                            ),
                            (u, 0, u)
                        );
                        assert_eq!(
                            required
                                .targets()
                                .coverage()
                                .groups()
                                .sources()
                                .source_count(),
                            4
                        );
                        assert!(required
                            .targets()
                            .coverage()
                            .groups()
                            .sources()
                            .assignments()
                            .all(|a| a.evidence_source() == Some(&id(0))));
                        assert_eq!(
                            required.additional_evidence_source().is_some(),
                            matches!(
                                intent,
                                JoinAdditionalConstructionIntent::AllowAdditional { .. }
                            )
                        );
                    }
                },
            );
        }
    }
}
