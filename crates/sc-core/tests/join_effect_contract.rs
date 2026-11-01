//! Mandatory effect requirements keep full real identity/consumer/copy/splice parents,not verdicts.
#![allow(clippy::expect_used)]
#[allow(dead_code)]
#[path = "support/join_result.rs"]
mod common;
use common::id;
use sc_core::{construction::*, ontology::*};
use std::collections::BTreeSet;
const KINDS: [JoinEffectEvidenceKind; 4] = [
    JoinEffectEvidenceKind::CanonicalIntakes,
    JoinEffectEvidenceKind::Sewing,
    JoinEffectEvidenceKind::AllCopyEffects,
    JoinEffectEvidenceKind::AuthoringAuthority,
];
fn with_identities(
    intent: JoinSourceIdentityIntent,
    run: impl FnOnce(RequiredJoinSourceIdentities<'_>),
) {
    common::candidate::with_bound_plan(|plan| {
        let fields = plan
            .admit_piece_fields(JoinInventoryFieldLimits::new(3, 2, 0, 24, 114))
            .expect("whole original/candidate resources");
        let raw = JoinCompanionRepairManifest::new(id(1000), vec![]);
        let repairs = raw
            .admit(&fields, JoinCompanionRepairLimits::new(0, 0))
            .expect("explicit companion scope");
        let originals = repairs
            .match_original_pieces()
            .expect("original exact fields");
        let inventory = originals
            .match_result_piece()
            .expect("complete actual candidate fields");
        let sources = JoinConsumerSources::new(id(0), id(0), id(0), id(0), id(0));
        let consumers = inventory.require_consumers(&sources);
        let results = plan.correspondence().sources().results();
        let mut edges = BTreeSet::new();
        for bits in [100, 101] {
            for field in results
                .source_piece(id(bits))
                .expect("actual A/B original")
                .edges()
            {
                edges.insert(field.edge);
            }
        }
        assert_eq!(edges.len(), 4);
        let rows = edges
            .into_iter()
            .map(|edge| JoinSourceIdentityAssignment::new(edge, intent))
            .collect();
        let raw = JoinSourceIdentityManifest::new(id(1000), id(0), rows);
        let identities = raw
            .admit(&consumers, JoinSourceIdentityLimits::new(10, 4, 18))
            .expect("complete actual original global edge requirements");
        run(identities);
    });
}
#[test]
fn every_effect_scope_lends_original_source_fields_and_actual_complete_copy_and_exact_splice_contexts(
) {
    with_identities(JoinSourceIdentityIntent::RetireOriginal, |identities| {
        let sources = JoinEffectSources::new(id(7001), id(7002), id(7003), id(7004));
        let snapshot = sources;
        let required = identities.require_effects(&sources);
        assert!(std::ptr::eq(required.identities(), &identities));
        assert!(std::ptr::eq(required.sources(), &sources));
        assert_eq!(required.requirements().len(), 4);
        assert_eq!(
            required
                .requirements()
                .map(|r| r.kind())
                .collect::<Vec<_>>(),
            KINDS
        );
        assert_eq!(
            required
                .requirements()
                .rev()
                .map(|r| r.kind())
                .collect::<Vec<_>>(),
            KINDS.into_iter().rev().collect::<Vec<_>>()
        );
        for ((item, kind), bits) in required.requirements().zip(KINDS).zip(7001..=7004) {
            assert!(std::ptr::eq(item.context(), &required));
            assert_eq!(item.kind(), kind);
            assert_eq!(*item.source(), id(bits));
            assert!(std::ptr::eq(item.source(), sources.source(kind)));
            assert!(std::ptr::eq(
                item.source(),
                required.requirement(kind).source()
            ));
        }
        assert!(std::ptr::eq(
            required.correspondence(),
            identities.consumers().inventory().plan().correspondence()
        ));
        assert_eq!(
            (
                required.correspondence().pair_count(),
                required.correspondence().consumed_copy_count()
            ),
            (2, 4)
        );
        assert_eq!(
            required
                .correspondence()
                .pairs()
                .map(|p| p.result().id())
                .collect::<Vec<_>>(),
            [id(6001), id(6000)]
        );
        let splice = identities.source_pieces().population().metadata().splice();
        assert!(std::ptr::eq(required.splice(), splice));
        assert!(std::ptr::eq(
            required.splice().ease(),
            &splice.source().span().definition().ease
        ));
        assert_eq!(required.splice().endpoint_pairs().len(), 2);
        assert_eq!(required.identities().retirement_count(), 4); // Still requests,not retired identities.
        assert_eq!(required.identities().consumers().requirements().len(), 5);
        assert_eq!(*required.identities().history_source(), id(0));
        assert_eq!(*required.identities().repair_source(), id(0));
        assert_eq!(sources, snapshot);
    });
}
#[test]
fn all_retention_and_shared_zero_source_ids_skip_no_effect_scope_or_earlier_history_consumer_obligation(
) {
    with_identities(JoinSourceIdentityIntent::RetainOriginal, |identities| {
        let sources = JoinEffectSources::new(id(0), id(0), id(0), id(0));
        let required = identities.require_effects(&sources);
        assert_eq!(required.requirements().len(), 4);
        assert_eq!(
            required
                .requirements()
                .map(|r| r.kind())
                .collect::<Vec<_>>(),
            KINDS
        );
        let mut fields = Vec::new();
        for item in required.requirements() {
            assert_eq!(*item.source(), id(0));
            assert!(fields
                .iter()
                .all(|previous| !std::ptr::eq(*previous, item.source())));
            fields.push(item.source());
        }
        assert_eq!(
            (
                required.identities().source_edge_count(),
                required.identities().retention_count(),
                required.identities().retirement_count()
            ),
            (4, 4, 0)
        );
        assert_eq!(required.identities().consumers().requirements().len(), 5);
        assert!(std::ptr::eq(
            required.identities().history_source(),
            identities.manifest().identity_history()
        ));
        assert_eq!(*required.identities().history_source(), id(0));
        assert_eq!(required.correspondence().pair_count(), 2);
        assert_eq!(
            required
                .identities()
                .consumers()
                .inventory()
                .result()
                .geometric_validation(),
            GeometricValidation::DeferredToG2
        );
    });
}
