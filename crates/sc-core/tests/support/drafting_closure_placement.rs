//! Exact actual requiring Closure/instance role and canonical notion membership only.
use super::{garment_source, limits, scope, TestResult};
use sc_core::{construction::*, design::*, ontology::*};
use std::error::Error;
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn kinds() -> Vec<ClosureKind> {
    vec![
        ClosureKind::HookAndBar {
            hook: NotionSize::Declaration(id(80)),
            bar: NotionSize::Declaration(id(80)),
        },
        ClosureKind::ButtonAndButtonhole {
            button: NotionSize::Declaration(id(80)),
            hole: ButtonholeDerivation { operation: id(81) },
        },
    ]
}
#[test]
fn closure_placement_members_preserve_original_roles_indices_and_reordered_canonical_loans(
) -> TestResult {
    for mode in [0, 2, 3] {
        let garment = garment_source::garment_with_placement_members(kinds(), mode)?;
        let provider = garment_source::Provider::new(garment.inputs(), &garment, false, false);
        let content = limits(100000, 100000, 100000)?;
        let context = scope(garment.inputs())?
            .bind_drafting_context(&provider, content)
            .map_err(|e| e.to_string())?;
        let admitted = context
            .admit_garment_source(content)
            .map_err(|e| e.to_string())?;
        let closure = garment
            .closures()
            .closures()
            .get(1)
            .ok_or("actual Closure member")?;
        for (instance_index, instance) in closure.definition().instances.iter().enumerate() {
            for role in [ClosurePlacementRole::First, ClosurePlacementRole::Second] {
                let request = DraftingClosurePlacementRequest {
                    closure: closure.id(),
                    instance: instance.id,
                    role,
                };
                let selected = DraftingClosurePlacementLimits::new(content, usize::MAX);
                provider.base.trace.set(0);
                let proof = admitted
                    .bind_closure_placement_member(request, selected)
                    .map_err(|e| e.to_string())?;
                assert!(std::ptr::eq(proof.parent(), &admitted));
                let owner = proof.owner();
                assert!(std::ptr::eq(owner.closure(), closure));
                assert_eq!(owner.closure_index(), 1);
                assert!(std::ptr::eq(owner.instance(), instance));
                assert_eq!(owner.instance_index(), instance_index);
                assert_eq!(owner.role(), role);
                assert_eq!(owner.target(), instance.placement(role));
                let (notion_index, original) = garment
                    .notions()
                    .placements()
                    .iter()
                    .enumerate()
                    .find(|(_, n)| n.id() == owner.target())
                    .ok_or("original canonical notion")?;
                assert!(std::ptr::eq(proof.notion(), original));
                assert_eq!(proof.notion_index(), notion_index);
                assert_eq!(
                    proof.notion().geometric_validation(),
                    GeometricValidation::DeferredToG2
                );
                assert_eq!(proof.limits(), selected);
                let mut meter = DesignResourceMeter::new(content);
                admitted.charge_resources(&mut meter)?;
                let expected = meter.finish()?;
                assert_eq!(proof.demand(), expected);
                let required = (expected.collection_entries() + 8).pow(2);
                assert_eq!(
                    proof.shape().held_collection_entries(),
                    expected.collection_entries()
                );
                assert_eq!(proof.shape().matching_inspection_bound(), required);
                assert_eq!(
                    provider.base.trace.get(),
                    22,
                    "two fresh walks and no source/value/currentness query"
                );
                assert!(admitted
                    .bind_closure_placement_member(
                        request,
                        DraftingClosurePlacementLimits::new(content, required)
                    )
                    .is_ok());
                let error = admitted
                    .bind_closure_placement_member(
                        request,
                        DraftingClosurePlacementLimits::new(content, required - 1),
                    )
                    .err()
                    .ok_or("whole matching shortage")?;
                assert!(
                    matches!(error.refusal(),DraftingClosurePlacementMemberRefusal::MatchingInspections {allowed,required:actual} if *allowed==required-1 && *actual==required)
                );
            }
        }
    }
    Ok(())
}
#[test]
fn closure_placement_members_keep_first_original_missing_causes_and_work_priority() -> TestResult {
    for case in 0..3 {
        let garment =
            garment_source::garment_with_placement_members(kinds(), if case == 2 { 1 } else { 0 })?;
        let provider = garment_source::Provider::new(garment.inputs(), &garment, false, false);
        let content = limits(100000, 100000, 100000)?;
        let context = scope(garment.inputs())?
            .bind_drafting_context(&provider, content)
            .map_err(|e| e.to_string())?;
        let admitted = context
            .admit_garment_source(content)
            .map_err(|e| e.to_string())?;
        let request = DraftingClosurePlacementRequest {
            closure: id(if case == 0 { 999 } else { 20012 }),
            instance: id(if case == 1 { 20003 } else { 20013 }),
            role: ClosurePlacementRole::Second,
        };
        let selected = DraftingClosurePlacementLimits::new(content, usize::MAX);
        provider.base.trace.set(0);
        let error = admitted
            .bind_closure_placement_member(request, selected)
            .err()
            .ok_or("original member missing")?;
        assert!(std::ptr::eq(error.parent(), &admitted));
        assert_eq!(error.request(), request);
        assert_eq!(error.limits(), selected);
        assert!(error.demand().is_some());
        assert!(error.shape().is_some());
        match (case, error.refusal()) {
            (0, DraftingClosurePlacementMemberRefusal::MissingClosure) => {}
            (
                1,
                DraftingClosurePlacementMemberRefusal::MissingInstance {
                    closure_index,
                    closure,
                },
            ) => {
                assert_eq!(*closure_index, 1);
                assert!(std::ptr::eq(
                    *closure,
                    garment
                        .closures()
                        .closures()
                        .get(1)
                        .ok_or("actual Closure member")?
                ));
            }
            (2, DraftingClosurePlacementMemberRefusal::MissingNotion { owner }) => {
                assert!(std::ptr::eq(
                    owner.closure(),
                    garment
                        .closures()
                        .closures()
                        .get(1)
                        .ok_or("actual Closure member")?
                ));
                assert!(std::ptr::eq(
                    owner.instance(),
                    garment
                        .closures()
                        .closures()
                        .get(1)
                        .ok_or("actual Closure member")?
                        .definition()
                        .instances
                        .first()
                        .ok_or("actual original instance")?
                ));
                assert_eq!(owner.role(), ClosurePlacementRole::Second);
                assert_eq!(owner.target(), id(20011));
            }
            _ => return Err(format!("unexpected case {case}: {error:?}").into()),
        }
        assert_eq!(provider.base.trace.get(), 2, "fresh content only");
        let error = admitted
            .bind_closure_placement_member(
                request,
                DraftingClosurePlacementLimits::new(limits(1, 1, 1)?, 0),
            )
            .err()
            .ok_or("fresh content first")?;
        assert!(matches!(
            error.refusal(),
            DraftingClosurePlacementMemberRefusal::Resources(_)
        ));
        assert!(error
            .source()
            .is_some_and(|c| c.is::<DesignResourceLimitError>()));
        assert_eq!(error.demand(), None);
        assert_eq!(error.shape(), None);
        let error = admitted
            .bind_closure_placement_member(request, DraftingClosurePlacementLimits::new(content, 0))
            .err()
            .ok_or("work before lookup")?;
        assert!(matches!(
            error.refusal(),
            DraftingClosurePlacementMemberRefusal::MatchingInspections { .. }
        ));
        assert!(error.demand().is_some());
        assert_eq!(error.shape(), None);
        if case == 2 {
            assert!(
                garment.validate_current().is_err(),
                "held second-target damage is observable"
            );
            let first = DraftingClosurePlacementRequest {
                role: ClosurePlacementRole::First,
                ..request
            };
            assert!(
                admitted
                    .bind_closure_placement_member(first, selected)
                    .is_ok(),
                "one member proof is not whole Closure currentness"
            );
        }
    }
    Ok(())
}
