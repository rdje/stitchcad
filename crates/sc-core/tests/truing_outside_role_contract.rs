//! Actual fixed outside source-role attribution;no whole-Piece placement,geometry or issuance.
#![allow(clippy::expect_used)]
#[path = "support/truing_domain.rs"]
mod common;
use common::{
    q,
    roles::{copies::id, with_roles},
    with_domain, with_partial,
};
use sc_core::{construction::*, name::MachineToken, ontology::*};

fn with_extended(roles: &BoundTruingEdgeRoles<'_>, run: impl FnOnce(BoundTruingEdgeRoles<'_>)) {
    let raw = EdgeOutputRoleCatalog::new(
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
    let limits = EdgeOutputRoleCatalogLimits::new(4, 3, 25);
    let catalog = raw
        .admit(limits)
        .expect("actual complete extended catalogue");
    assert_eq!(catalog.key_bytes(), 25);
    let bound = roles
        .copies()
        .bind_edge_roles(&catalog, limits)
        .expect("same actual source/copy/Piece parents");
    run(bound);
}
fn with_evidence(
    roles: &BoundTruingEdgeRoles<'_>,
    intent: TruingOutcomeIntent,
    run: impl FnOnce(RequiredTruingEvidence<'_>),
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
        .expect("selected metadata");
    let domain = edges
        .bind_source_domain(TruingSourceDomainLimits::new(n))
        .expect("actual domain");
    let outcome = domain
        .require_outcome(intent, TruingSourceCoverageLimits::new(n, n))
        .expect("actual outcome");
    let locks = outcome
        .require_locks(TruingLockLimits::new(2, 1))
        .expect("actual complete locks");
    let source = id(7000);
    let policy = TruingEvidencePolicy::new(
        TruingContextSources::new(source, source, source),
        TruingFitSources::new(source, source, source, source),
        TruingEffectSources::new(source, source, source, source),
    );
    let required = locks.require_evidence(&policy);
    run(required);
}
fn limits(n: usize) -> TruingOutsideRoleLimits {
    TruingOutsideRoleLimits::new(n, EdgeOutputRoleCatalogLimits::new(4, 3, 25))
}
#[test]
fn both_sides_and_outcomes_keep_exact_outside_ranges_original_fields_and_actual_catalogue_roles() {
    use TruingOutsideIntent as I;
    use TruingOutsidePart as P;
    for side in [SeamSideId::A, SeamSideId::B] {
        for (from, to) in [
            (Param::START, Param::END),
            (Param::START, q(3, 4)),
            (q(1, 4), Param::END),
            (q(1, 4), q(3, 4)),
        ] {
            for intent in [
                TruingOutcomeIntent::PreserveSource,
                TruingOutcomeIntent::Refit,
            ] {
                with_domain(side, from, to, |original| {
                    with_extended(&original, |roles| {
                        with_evidence(&roles, intent, |evidence| {
                            let prefix =
                                intent == TruingOutcomeIntent::Refit && from > Param::START;
                            let suffix = intent == TruingOutcomeIntent::Refit && to < Param::END;
                            let raw = TruingOutsideRoles::new(
                                if prefix {
                                    I::Role(LocalTag::new(2))
                                } else {
                                    I::Absent
                                },
                                if suffix {
                                    I::Role(LocalTag::new(5))
                                } else {
                                    I::Absent
                                },
                            );
                            let snapshot = raw;
                            let n = usize::from(intent == TruingOutcomeIntent::Refit);
                            let bound = raw
                                .admit(&evidence, limits(n))
                                .expect("exact fixed outside attribution");
                            assert!(std::ptr::eq(bound.proposal(), &raw));
                            assert!(std::ptr::eq(bound.evidence(), &evidence));
                            assert_eq!(bound.limits(), limits(n));
                            assert_eq!(bound.limits().candidates(), n);
                            assert_eq!(
                                bound.limits().catalog(),
                                EdgeOutputRoleCatalogLimits::new(4, 3, 25)
                            );
                            let mut expected = vec![];
                            if prefix {
                                expected.push((P::Prefix, Param::START, from, LocalTag::new(2)));
                            }
                            if suffix {
                                expected.push((P::Suffix, to, Param::END, LocalTag::new(5)));
                            }
                            assert_eq!(bound.roles().count(), expected.len());
                            for (item, (part, lower, upper, tag)) in
                                bound.roles().zip(expected.clone())
                            {
                                assert!(std::ptr::eq(item.context(), &bound));
                                assert_eq!(item.part(), part);
                                assert!(std::ptr::eq(item.intent(), raw.intent(part)));
                                assert_eq!(item.intent(), &I::Role(tag));
                                assert_eq!(
                                    *item.source(),
                                    EdgeRange::new(
                                        roles.copies().source().source_side().range.edge(),
                                        lower,
                                        upper
                                    )
                                    .expect("positive exact outside")
                                );
                                assert!(std::ptr::eq(item.role().catalog(), roles.catalog()));
                                assert_eq!(*item.role().tag(), tag);
                                assert_eq!(item.logical_output(), EdgeRef::new(id(1000), tag));
                            }
                            assert_eq!(
                                bound.roles().rev().map(|r| r.part()).collect::<Vec<_>>(),
                                expected
                                    .into_iter()
                                    .rev()
                                    .map(|(part, _, _, _)| part)
                                    .collect::<Vec<_>>()
                            );
                            assert_eq!(raw, snapshot);
                            assert!(format!("{bound:?}").contains("AdmittedTruingOutsideRoles"));
                        })
                    })
                });
            }
        }
    }
}
#[test]
fn whole_selected_then_definition_active_and_key_quotas_precede_any_fixed_field_failure() {
    use TruingEdgeRoleBindingRefusal as C;
    use TruingOutsideRoleRefusal as R;
    with_partial(SeamSideId::B, |original| {
        with_extended(&original, |roles| {
            with_evidence(&roles, TruingOutcomeIntent::Refit, |evidence| {
                let raw = TruingOutsideRoles::new(
                    TruingOutsideIntent::Role(LocalTag::new(7)),
                    TruingOutsideIntent::Absent,
                );
                for (policy, cause) in [
                    (
                        TruingOutsideRoleLimits::new(0, EdgeOutputRoleCatalogLimits::new(0, 0, 0)),
                        R::Candidates {
                            required: 1,
                            allowed: 0,
                        },
                    ),
                    (
                        TruingOutsideRoleLimits::new(1, EdgeOutputRoleCatalogLimits::new(3, 2, 24)),
                        R::Catalog(C::Definitions {
                            required: 4,
                            allowed: 3,
                        }),
                    ),
                    (
                        TruingOutsideRoleLimits::new(1, EdgeOutputRoleCatalogLimits::new(4, 2, 24)),
                        R::Catalog(C::Active {
                            required: 3,
                            allowed: 2,
                        }),
                    ),
                    (
                        TruingOutsideRoleLimits::new(1, EdgeOutputRoleCatalogLimits::new(4, 3, 24)),
                        R::Catalog(C::KeyBytes {
                            required: 25,
                            allowed: 24,
                        }),
                    ),
                    (
                        limits(1),
                        R::Inactive {
                            part: TruingOutsidePart::Prefix,
                            tag: LocalTag::new(7),
                        },
                    ),
                ] {
                    let error = raw
                        .admit(&evidence, policy)
                        .expect_err("whole quotas before first actual field");
                    assert!(std::ptr::eq(error.proposal(), &raw));
                    assert!(std::ptr::eq(error.evidence(), &evidence));
                    assert_eq!(error.limits(), policy);
                    assert_eq!(error.refusal(), cause);
                    assert!(error.to_string().starts_with("truing outside roles:"));
                    assert!(format!("{error:?}").contains("TruingOutsideRoleError"));
                }
            })
        })
    });
}
#[test]
fn missing_empty_undefined_inactive_selected_duplicate_and_preservation_fields_refuse_without_prefix_context(
) {
    use TruingOutsideIntent as I;
    use TruingOutsidePart as P;
    use TruingOutsideRoleRefusal as R;
    let role = |tag| I::Role(LocalTag::new(tag));
    with_partial(SeamSideId::A, |original| {
        with_extended(&original, |roles| {
            with_evidence(&roles, TruingOutcomeIntent::Refit, |evidence| {
                for (prefix, suffix, cause) in [
                    (I::Absent, role(5), R::Missing { part: P::Prefix }),
                    (role(2), I::Absent, R::Missing { part: P::Suffix }),
                    (
                        role(99),
                        role(5),
                        R::Undefined {
                            part: P::Prefix,
                            tag: LocalTag::new(99),
                        },
                    ),
                    (
                        role(7),
                        role(5),
                        R::Inactive {
                            part: P::Prefix,
                            tag: LocalTag::new(7),
                        },
                    ),
                    (
                        role(9),
                        role(5),
                        R::SelectedRole {
                            part: P::Prefix,
                            tag: LocalTag::new(9),
                        },
                    ),
                    (
                        role(2),
                        role(2),
                        R::Duplicate {
                            tag: LocalTag::new(2),
                        },
                    ),
                    (
                        role(2),
                        role(99),
                        R::Undefined {
                            part: P::Suffix,
                            tag: LocalTag::new(99),
                        },
                    ),
                ] {
                    let raw = TruingOutsideRoles::new(prefix, suffix);
                    let error = raw
                        .admit(&evidence, limits(1))
                        .expect_err("first fixed-field cause;no prefix output context");
                    assert_eq!(error.refusal(), cause);
                    assert!(std::ptr::eq(error.proposal(), &raw));
                    assert!(std::ptr::eq(error.evidence(), &evidence));
                }
            });
            with_evidence(&roles, TruingOutcomeIntent::PreserveSource, |evidence| {
                for (prefix, suffix, part) in [
                    (role(2), I::Absent, P::Prefix),
                    (I::Absent, role(5), P::Suffix),
                ] {
                    let raw = TruingOutsideRoles::new(prefix, suffix);
                    assert_eq!(
                        raw.admit(&evidence, limits(0))
                            .expect_err("preservation admits no outside output identities")
                            .refusal(),
                        R::PreserveSource { part }
                    );
                }
            });
        })
    });
    with_roles(false, |original| {
        with_extended(&original, |roles| {
            with_evidence(&roles, TruingOutcomeIntent::Refit, |evidence| {
                for (prefix, suffix, part, at) in [
                    (role(2), I::Absent, P::Prefix, Param::START),
                    (I::Absent, role(5), P::Suffix, Param::END),
                ] {
                    let raw = TruingOutsideRoles::new(prefix, suffix);
                    assert_eq!(
                        raw.admit(&evidence, limits(1))
                            .expect_err("whole selected domain has no positive outside fragments")
                            .refusal(),
                        R::SourceRange {
                            part,
                            cause: RangeError { from: at, to: at }
                        }
                    );
                }
            })
        })
    });
}
