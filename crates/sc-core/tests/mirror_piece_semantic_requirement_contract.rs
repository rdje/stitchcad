//! Whole original semantic intent and mandatory evidence sources;no provider/effect acceptance.
#![allow(clippy::expect_used)]
#[path = "support/mirror_copy_inventory.rs"]
mod common;
use common::copies::with_ids;
use common::inventory;
use sc_core::{construction::*, ontology::*};
fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn keep(_: &mut PieceDefinition) {}
fn with_plan(
    mirroring: Mirroring,
    existing: bool,
    intents: Vec<MirrorPieceResultCopyIntent>,
    run: impl FnOnce(BoundMirrorPieceCopyPlan<'_>),
) {
    with_ids(1, mirroring, existing, |ids| {
        let raw = MirrorPieceResultCopyManifest::new(id(10), intents);
        let n = raw.intents().len();
        let b = ids.source().plan().copies().len();
        let copies = raw
            .admit(&ids, MirrorPieceResultCopyLimits::new(n, b, 2 * n))
            .expect("whole native choices");
        let inventory = inventory(&copies, keep, keep);
        let invlimits = MirrorPieceCopyInventoryLimits::new(3, 8, 30, 220);
        let inventory = copies
            .bind_piece_inventory(&inventory, &invlimits)
            .expect("actual candidate members");
        let mut definitions = ids
            .source()
            .plan()
            .copies()
            .iter()
            .filter(|copy| copy.piece() != id(200))
            .map(|copy| *copy.definition())
            .collect::<Vec<_>>();
        definitions.extend(copies.result_actions().map(|action| *action.definition()));
        let plan = CutPlan::new(definitions, inventory.inventory().pieces())
            .expect("actual candidate plan");
        let c = plan.copies().len();
        let limits = MirrorPieceCopyPlanLimits::new(b, c, 3, n, 3 + 3 * c);
        let plan = inventory
            .bind_cut_plan(&plan, &limits)
            .expect("complete association");
        run(plan);
    });
}
fn new_result() -> Vec<MirrorPieceResultCopyIntent> {
    vec![MirrorPieceResultCopyIntent::Issue(CutCopyDefinition {
        id: id(500),
        piece: id(200),
        orientation: CopyOrientation::Authored,
    })]
}
fn policy(intent: MirrorPieceSemanticIntent, shared: bool) -> MirrorPieceSemanticPolicy {
    let source = |bits| if shared { id(0) } else { id(bits) };
    MirrorPieceSemanticPolicy::new(
        intent,
        MirrorPieceSemanticConsumerSources::new(
            source(700),
            source(701),
            source(702),
            source(703),
            source(704),
        ),
        MirrorPieceSemanticEffectSources::new(source(705), source(706), source(707), source(708)),
    )
}
fn kinds() -> [MirrorPieceSemanticEvidenceKind; 9] {
    use MirrorPieceSemanticEvidenceKind as K;
    [
        K::WholeSourceConsumers,
        K::SourcePreservation,
        K::CurrentOwnership,
        K::ResultSemantics,
        K::VisibleRepairs,
        K::CanonicalIntakes,
        K::Sewing,
        K::AllCopyEffects,
        K::AuthoringAuthority,
    ]
}
#[test]
fn both_explicit_intents_keep_all_original_fields_and_actual_complete_parents_unchanged() {
    for intent in [
        MirrorPieceSemanticIntent::SourceOnly,
        MirrorPieceSemanticIntent::ExplicitResultMappings,
    ] {
        with_plan(Mirroring::Single, false, new_result(), |plan| {
            let policy = policy(intent, false);
            let original = policy;
            let before = plan
                .inventory()
                .copies()
                .identities()
                .source()
                .plan()
                .clone();
            let candidate = plan.plan().clone();
            let inventory = plan.inventory().inventory().clone();
            let source = plan.inventory().source_piece().clone();
            let required = plan.require_semantics(&policy);
            assert!(std::ptr::eq(required.plan(), &plan));
            assert!(std::ptr::eq(required.policy(), &policy));
            assert!(std::ptr::eq(required.intent(), policy.intent()));
            assert_eq!(required.intent(), &intent);
            assert!(std::ptr::eq(
                required.correspondence(),
                plan.inventory().copies().identities().source().context()
            ));
            assert!(std::ptr::eq(
                required.policy().consumer_sources(),
                policy.consumer_sources()
            ));
            assert!(std::ptr::eq(
                required.policy().effect_sources(),
                policy.effect_sources()
            ));
            let requirements = required.requirements().collect::<Vec<_>>();
            assert_eq!(requirements.len(), 9);
            for ((item, kind), bits) in requirements.iter().zip(kinds()).zip(700..709) {
                assert!(std::ptr::eq(item.context(), &required));
                assert_eq!(item.kind(), kind);
                assert!(std::ptr::eq(item.source(), policy.source(kind)));
                assert_eq!(*item.source(), id(bits));
            }
            assert_eq!(policy, original);
            assert_eq!(*plan.plan(), candidate);
            assert_eq!(*plan.inventory().inventory(), inventory);
            assert_eq!(*plan.inventory().source_piece(), source);
            assert_eq!(
                *plan.inventory().copies().identities().source().plan(),
                before
            );
            assert_eq!(required.correspondence().requirements().count(), 6);
        });
    }
}
#[test]
fn shared_zero_provider_never_collapses_obligations_and_each_view_lends_its_actual_distinct_field()
{
    for intent in [
        MirrorPieceSemanticIntent::SourceOnly,
        MirrorPieceSemanticIntent::ExplicitResultMappings,
    ] {
        with_plan(Mirroring::Single, false, new_result(), |plan| {
            let policy = policy(intent, true);
            let required = plan.require_semantics(&policy);
            let items = required.requirements().collect::<Vec<_>>();
            assert_eq!(items.len(), 9);
            assert_eq!(
                items.iter().map(|item| item.kind()).collect::<Vec<_>>(),
                kinds()
            );
            let addresses = items
                .iter()
                .map(|item| item.source() as *const EntityId)
                .collect::<std::collections::BTreeSet<_>>();
            assert_eq!(addresses.len(), 9);
            for item in &items {
                assert_eq!(*item.source(), id(0));
                assert!(std::ptr::eq(item.source(), policy.source(item.kind())));
                assert!(std::ptr::eq(item.context(), &required));
            }
            assert_eq!(
                required
                    .requirements()
                    .rev()
                    .map(|item| item.kind())
                    .collect::<Vec<_>>(),
                kinds().into_iter().rev().collect::<Vec<_>>()
            );
            let mut both = required.requirements();
            assert_eq!(both.len(), 9);
            assert_eq!(
                both.next().expect("first").kind(),
                MirrorPieceSemanticEvidenceKind::WholeSourceConsumers
            );
            assert_eq!(
                both.next_back().expect("last").kind(),
                MirrorPieceSemanticEvidenceKind::AuthoringAuthority
            );
            assert_eq!(both.len(), 7);
            assert_eq!(both.count(), 7);
        });
    }
}
#[test]
fn source_only_with_retirement_and_actual_companion_still_requires_every_copy_and_sewing_scope() {
    with_plan(
        Mirroring::PairMember {
            handedness: Handedness::Left,
            companion: id(300),
        },
        true,
        vec![
            MirrorPieceResultCopyIntent::Retire(id(551)),
            MirrorPieceResultCopyIntent::Retain(id(550)),
        ],
        |plan| {
            let policy = policy(MirrorPieceSemanticIntent::SourceOnly, false);
            let required = plan.require_semantics(&policy);
            let copy_context = required.plan().inventory().copies();
            assert_eq!(
                (
                    copy_context.retention_count(),
                    copy_context.retirement_count(),
                    copy_context.issuance_count()
                ),
                (1, 1, 0)
            );
            assert!(required.plan().copy(id(551)).is_none());
            assert!(
                matches!(copy_context.action(id(551)).expect("original retirement").source(),MirrorPieceResultCopySource::Retire(copy) if copy.id()==id(551))
            );
            assert_eq!(
                required
                    .plan()
                    .inventory()
                    .result_companion()
                    .expect("actual reciprocal")
                    .id(),
                id(300)
            );
            assert_eq!(
                required
                    .plan()
                    .copy(id(333))
                    .expect("companion actual copy")
                    .piece(),
                id(300)
            );
            assert_eq!(required.requirements().len(), 9);
            for kind in [
                MirrorPieceSemanticEvidenceKind::SourcePreservation,
                MirrorPieceSemanticEvidenceKind::ResultSemantics,
                MirrorPieceSemanticEvidenceKind::CanonicalIntakes,
                MirrorPieceSemanticEvidenceKind::Sewing,
                MirrorPieceSemanticEvidenceKind::AllCopyEffects,
                MirrorPieceSemanticEvidenceKind::AuthoringAuthority,
            ] {
                assert_eq!(
                    required
                        .requirements()
                        .filter(|item| item.kind() == kind)
                        .count(),
                    1
                );
            }
        },
    );
}
