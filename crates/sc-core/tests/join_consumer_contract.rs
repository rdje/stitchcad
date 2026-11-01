//! Explicit unresolved whole-consumer scopes over actual complete matched candidate/source parents.
#![allow(clippy::expect_used)]
#[allow(dead_code)]
#[path = "support/join_result.rs"]
mod common;
use common::id;
use sc_core::{construction::*, ontology::*};
const KINDS: [JoinConsumerEvidenceKind; 5] = [
    JoinConsumerEvidenceKind::WholeSourceConsumers,
    JoinConsumerEvidenceKind::SourcePreservation,
    JoinConsumerEvidenceKind::CurrentOwnership,
    JoinConsumerEvidenceKind::ResultSemantics,
    JoinConsumerEvidenceKind::VisibleRepairs,
];
#[test]
fn every_distinct_semantic_scope_lends_the_actual_original_source_and_complete_inventory_parent() {
    common::with_result(
        |_| {},
        |fields| {
            let raw = JoinCompanionRepairManifest::new(id(1000), vec![]);
            let repairs = raw
                .admit(&fields, JoinCompanionRepairLimits::new(0, 0))
                .expect("explicit companion request scope");
            let originals = repairs
                .match_original_pieces()
                .expect("complete original field preservation");
            let inventory = originals
                .match_result_piece()
                .expect("actual complete joined fields");
            let sources =
                JoinConsumerSources::new(id(9001), id(9002), id(9003), id(9004), id(9005));
            let snapshot = sources;
            let required = inventory.require_consumers(&sources);
            assert!(std::ptr::eq(required.inventory(), &inventory));
            assert!(std::ptr::eq(required.sources(), &sources));
            assert_eq!(required.requirements().len(), 5);
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
            for ((item, kind), bits) in required.requirements().zip(KINDS).zip(9001..=9005) {
                assert!(std::ptr::eq(item.context(), &required));
                assert_eq!(item.kind(), kind);
                assert_eq!(*item.source(), id(bits));
                assert!(std::ptr::eq(item.source(), sources.source(kind)));
                assert!(std::ptr::eq(
                    item.source(),
                    required.requirement(kind).source()
                ));
            }
            assert!(std::ptr::eq(required.inventory().plan(), fields.plan()));
            let folds = required
                .inventory()
                .plan()
                .correspondence()
                .sources()
                .results()
                .folds();
            assert_eq!(folds.assignments().filter(|a|matches!(a.intent(),JoinFoldSourceIntent::Omit{evidence_source} if *evidence_source==id(0))).count(),2);
            assert_eq!(
                required.inventory().result().geometric_validation(),
                GeometricValidation::DeferredToG2
            );
            assert_eq!(sources, snapshot); // No registry/evidence lookup or original mutation.
        },
    );
}
#[test]
fn zero_shared_source_ids_never_deduplicate_scopes_or_discharge_actual_companion_repair_requests() {
    common::candidate::with_companion_fields(
        Mirroring::PairMember {
            handedness: Handedness::Left,
            companion: id(102),
        },
        true,
        |fields| {
            let raw = JoinCompanionRepairManifest::new(
                id(1000),
                vec![JoinCompanionRepairRequest::new(
                    id(102),
                    Mirroring::PairMember {
                        handedness: Handedness::Right,
                        companion: id(100),
                    },
                    id(5000),
                    id(0),
                )],
            );
            let snapshot = raw.clone();
            let repairs = raw
                .admit(&fields, JoinCompanionRepairLimits::new(1, 2))
                .expect("explicit actual original/candidate companion requirement");
            let originals = repairs
                .match_original_pieces()
                .expect("complete original fields except exact admitted reference");
            let inventory = originals
                .match_result_piece()
                .expect("complete joined-result fields");
            let sources = JoinConsumerSources::new(id(0), id(0), id(0), id(0), id(0));
            let required = inventory.require_consumers(&sources);
            assert_eq!(required.requirements().len(), 5);
            assert_eq!(
                required
                    .requirements()
                    .map(|r| r.kind())
                    .collect::<Vec<_>>(),
                KINDS
            );
            let mut actual_fields = Vec::new();
            for item in required.requirements() {
                assert_eq!(*item.source(), id(0));
                assert!(actual_fields
                    .iter()
                    .all(|previous| !std::ptr::eq(*previous, item.source())));
                actual_fields.push(item.source());
            }
            let actual = required
                .inventory()
                .originals()
                .original(id(102))
                .expect("actual retained member")
                .companion_repair()
                .expect("still outstanding explicit repair");
            assert!(std::ptr::eq(actual.context(), &repairs));
            assert_eq!(*actual.evidence_source(), id(0));
            assert_eq!(actual.retired_companion().id(), id(100));
            assert_eq!(
                actual.original().definition().mirroring,
                Mirroring::PairMember {
                    handedness: Handedness::Right,
                    companion: id(100)
                }
            );
            assert_eq!(raw, snapshot);
        },
    );
}
