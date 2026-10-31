//! Exact original selected held-domain membership only;coverage and fitting remain separate.
#![allow(clippy::expect_used)]
#[path = "support/truing_domain.rs"]
mod common;
use common::{
    q,
    roles::{copies::id, with_roles},
    with_partial,
};
use sc_core::{construction::*, ontology::*};
fn raw(ranges: &[(u32, EdgeRange)]) -> TruingEdgeManifest {
    TruingEdgeManifest::new(
        id(1000),
        ranges
            .iter()
            .map(|(tag, r)| TruingEdgeCandidate::new(LocalTag::new(*tag), *r))
            .collect(),
    )
}
#[test]
fn either_selected_side_lends_actual_partial_domain_and_every_original_candidate_with_complete_context(
) {
    for side in [SeamSideId::A, SeamSideId::B] {
        with_partial(side, |roles| {
            let domain = &roles.copies().source().source_side().range;
            assert_eq!((domain.from(), domain.to()), (q(1, 4), q(3, 4)));
            let ranges = [
                (
                    9,
                    EdgeRange::new(domain.edge(), q(1, 4), q(1, 2)).expect("left"),
                ),
                (
                    2,
                    EdgeRange::new(domain.edge(), q(1, 2), q(3, 4)).expect("right"),
                ),
            ];
            let raw = raw(&ranges);
            let snapshot = raw.clone();
            let edges = raw
                .admit(&roles, TruingEdgeManifestLimits::new(2, 4))
                .expect("candidate metadata");
            let limits = TruingSourceDomainLimits::new(2);
            let bound = edges
                .bind_source_domain(limits)
                .expect("all actual selected held-domain members");
            assert!(std::ptr::eq(bound.candidates(), &edges));
            assert!(std::ptr::eq(bound.domain(), domain));
            assert_eq!(bound.limits(), limits);
            assert_eq!(limits.candidates(), 2);
            assert_eq!(bound.edge_count(), 2);
            for (item, index) in bound.edges().zip(0..) {
                assert!(std::ptr::eq(item.context(), &bound));
                assert!(std::ptr::eq(item.domain(), domain));
                assert!(std::ptr::eq(item.candidate().owner(), &edges));
                assert_eq!(item.candidate().index(), index);
                assert!(std::ptr::eq(
                    item.source(),
                    raw.candidates()
                        .get(index)
                        .expect("original candidate")
                        .source()
                ));
                assert!(std::ptr::eq(
                    item.candidate().role().catalog(),
                    roles.catalog()
                ));
            }
            assert_eq!(
                bound
                    .edges()
                    .rev()
                    .map(|e| e.candidate().candidate().tag())
                    .collect::<Vec<_>>(),
                vec![LocalTag::new(2), LocalTag::new(9)]
            );
            assert_eq!(
                bound
                    .edge(LocalTag::new(9))
                    .expect("actual tag")
                    .candidate()
                    .index(),
                0
            );
            assert!(bound.edge(LocalTag::new(7)).is_none());
            assert_eq!(raw, snapshot);
            assert!(format!("{bound:?}").contains("BoundTruingSourceDomain"));
        });
    }
}
#[test]
fn whole_candidate_quota_precedes_first_wrong_edge_and_errors_keep_actual_complete_range_parents() {
    use TruingSourceDomainRefusal as R;
    with_partial(SeamSideId::A, |roles| {
        let domain = &roles.copies().source().source_side().range;
        let wrong = EdgeRef::new(id(555), LocalTag::FIRST);
        let raw = raw(&[(9, EdgeRange::whole(wrong)), (2, *domain)]);
        let edges = raw
            .admit(&roles, TruingEdgeManifestLimits::new(2, 4))
            .expect("local metadata");
        for (limits, cause) in [
            (
                TruingSourceDomainLimits::new(1),
                R::Candidates {
                    required: 2,
                    allowed: 1,
                },
            ),
            (
                TruingSourceDomainLimits::new(2),
                R::Edge {
                    index: 0,
                    edge: wrong,
                },
            ),
        ] {
            let error = edges
                .bind_source_domain(limits)
                .expect_err("whole quota then first source cause");
            assert!(std::ptr::eq(error.edges(), &edges));
            assert!(std::ptr::eq(error.domain(), domain));
            assert_eq!(error.limits(), limits);
            assert_eq!(error.refusal(), cause);
            assert!(error
                .to_string()
                .starts_with("truing selected source domain:"));
            assert!(format!("{error:?}").contains("TruingSourceDomainError"));
        }
    });
}
#[test]
fn exact_creator_and_tag_then_lower_upper_bounds_refuse_in_original_order_and_closed_endpoint_touches_pass(
) {
    use TruingSourceDomainRefusal as R;
    for side in [SeamSideId::A, SeamSideId::B] {
        with_partial(side, |roles| {
            let domain = roles.copies().source().source_side().range;
            let other = roles.copies().source().counterpart_side().range.edge();
            assert_eq!(other.creator(), domain.edge().creator());
            assert_ne!(other.tag(), domain.edge().tag());
            let range = |from, to| {
                EdgeRange::new(domain.edge(), from, to).expect("positive original range")
            };
            for (source, cause) in [
                (
                    EdgeRange::whole(other),
                    R::Edge {
                        index: 0,
                        edge: other,
                    },
                ),
                (EdgeRange::whole(domain.edge()), R::BeforeStart { index: 0 }),
                (range(q(1, 8), q(1, 2)), R::BeforeStart { index: 0 }),
                (range(q(1, 2), q(7, 8)), R::AfterEnd { index: 0 }),
            ] {
                let raw = raw(&[(9, source)]);
                let edges = raw
                    .admit(&roles, TruingEdgeManifestLimits::new(1, 2))
                    .expect("original metadata");
                assert_eq!(
                    edges
                        .bind_source_domain(TruingSourceDomainLimits::new(1))
                        .expect_err("exact original first cause")
                        .refusal(),
                    cause
                );
            }
            let late = raw(&[(9, domain), (2, EdgeRange::whole(domain.edge()))]);
            let late = late
                .admit(&roles, TruingEdgeManifestLimits::new(2, 4))
                .expect("complete original candidates");
            assert_eq!(
                late.bind_source_domain(TruingSourceDomainLimits::new(2))
                    .expect_err("valid first member never yields a prefix context")
                    .refusal(),
                R::BeforeStart { index: 1 }
            );
            for source in [domain, range(q(1, 4), q(1, 2)), range(q(1, 2), q(3, 4))] {
                let raw = raw(&[(2, source)]);
                let edges = raw
                    .admit(&roles, TruingEdgeManifestLimits::new(1, 2))
                    .expect("metadata");
                let bound = edges
                    .bind_source_domain(TruingSourceDomainLimits::new(1))
                    .expect("closed selected endpoint membership");
                assert_eq!(
                    *bound
                        .edge(LocalTag::new(2))
                        .expect("actual source")
                        .source(),
                    source
                );
            }
        });
    }
}
#[test]
fn empty_and_overlapping_inside_members_preserve_positive_domain_without_claiming_coverage_or_no_op(
) {
    with_roles(false, |roles| {
        let raw = raw(&[]);
        let edges = raw
            .admit(&roles, TruingEdgeManifestLimits::new(0, 0))
            .expect("empty metadata");
        let bound = edges
            .bind_source_domain(TruingSourceDomainLimits::new(0))
            .expect("empty membership only");
        assert_eq!(bound.edge_count(), 0);
        assert_eq!(bound.edges().len(), 0);
        assert!(bound.domain().from() < bound.domain().to());
        assert!(bound
            .candidates()
            .roles()
            .catalog()
            .active_role(LocalTag::new(9))
            .is_some());
        assert_eq!(
            bound
                .candidates()
                .roles()
                .copies()
                .source()
                .span()
                .definition()
                .stops
                .len(),
            1
        );
    });
    with_partial(SeamSideId::B, |roles| {
        let domain = roles.copies().source().source_side().range;
        let middle = EdgeRange::new(domain.edge(), q(3, 8), q(5, 8))
            .expect("inside but incomplete and repeated range");
        let raw = raw(&[(9, middle), (2, middle)]);
        let edges = raw
            .admit(&roles, TruingEdgeManifestLimits::new(2, 4))
            .expect("explicit attributions");
        let bound = edges
            .bind_source_domain(TruingSourceDomainLimits::new(2))
            .expect("membership is not coverage/nonoverlap");
        assert_eq!(bound.edge_count(), 2);
        assert!(bound.edges().all(|e| *e.source() == middle));
        assert_eq!(*bound.domain(), domain);
    });
}
