//! Fresh complete original rule work before missing/kind lookup;no rule/Profile query or values.
use super::{
    limits, root_with_host_and_semantic,
    rule_sources::Provider,
    scope,
    semantic_root::{canonical, recipe},
    EmptyHost, TestResult,
};
use sc_core::{
    construction::*,
    design::*,
    ontology::*,
    recipe::{SemanticOperationKind, SemanticOperationLookupError},
};
use std::error::Error;
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn root() -> TestResult<DesignInputs<EmptyHost>> {
    root_with_host_and_semantic(
        "",
        vec![],
        EmptyHost,
        canonical()?,
        vec![],
        DesignSourceReferences {
            measurement_table: None,
            ease_set: None,
            size_set: None,
        },
        recipe(70)?,
    )
}
fn pin() -> TestResult<SelectedProfilePin> {
    Ok(SelectedProfilePin::new(SelectedProfilePinDefinition {
        profile_id: "atelier é".into(),
        schema_version: " 2.0 ".into(),
        content_version: "7".into(),
    })?)
}
#[test]
fn actual_operation_and_explicit_intent_survive_complete_fresh_work_without_source_queries(
) -> TestResult {
    let root = root()?;
    let other = super::root()?;
    for (operation, kind, profile, index, rule) in [
        (70, SemanticOperationKind::Buttonhole, false, 0, 900),
        (80, SemanticOperationKind::LogicalDerivation, true, 1, 901),
    ] {
        let source = Provider::new(&root, &other)?;
        let context = scope(&root)?
            .bind_drafting_context(&source, limits(1000, 10000, 1000)?)
            .map_err(|e| e.to_string())?;
        // Selection is created after the longer-lived context;no pin lifetime is widened.
        let selected = pin()?;
        let selection = if profile {
            DraftingRuleSelection::Profile(&selected)
        } else {
            DraftingRuleSelection::Design
        };
        let text = source.text(&selected) + 24 + if profile { 16 } else { 0 };
        let policy = DraftingRuleBindingLimits::new(limits(21, text, 138)?, 841);
        let before = source.charge_count();
        let work = context
            .prepare_rule_binding(id(operation), kind, selection, policy)
            .map_err(|e| e.to_string())?;
        assert_eq!(
            (source.charge_count(), source.query_count()),
            (before + 1, 0)
        );
        assert!(core::ptr::eq(work.context().scope().root(), &root));
        assert!(core::ptr::eq(work.context().source(), &source));
        assert!(work.context().snapshot().is_same_record(context.snapshot()));
        assert_eq!(work.operation().index(), index);
        assert!(core::ptr::eq(
            work.operation().recipe(),
            root.semantic_recipe()
        ));
        assert!(core::ptr::eq(
            work.operation().operation(),
            root.semantic_recipe()
                .operations()
                .get(index)
                .ok_or("original operation")?
        ));
        assert_eq!(work.limits(), policy);
        assert_eq!(
            (
                work.demand().collection_entries(),
                work.demand().text_bytes(),
                work.demand().counter_bits()
            ),
            (21, text, 138)
        );
        assert_eq!(
            (
                work.shape().held_collection_entries(),
                work.shape().matching_inspection_bound()
            ),
            (21, 841)
        );
        let request = work.request();
        assert_eq!((request.rule, request.kind), (id(rule), kind));
        match request.origin {
            DraftingRuleSelection::Profile(actual) => {
                assert!(profile);
                assert!(core::ptr::eq(actual, &selected));
            }
            DraftingRuleSelection::Design => assert!(!profile),
        }
        let mut meter = DesignResourceMeter::new(limits(23, text + 3, 139)?);
        let prior = DesignResourceLocation {
            scope: DesignResourceScope::CommandState,
            owner: Some(id(1)),
            field: "prior",
            index: None,
        };
        meter.charge_entries(2, prior)?;
        meter.charge_text("old", prior)?;
        meter.charge_counter_bits(1, prior)?;
        work.charge_resources(&mut meter)?;
        let demand = meter.finish()?;
        assert_eq!(
            (
                demand.collection_entries(),
                demand.text_bytes(),
                demand.counter_bits()
            ),
            (23, text + 3, 139)
        );
        assert_eq!(
            (source.charge_count(), source.query_count()),
            (before + 2, 0)
        );
        assert!(!format!("{work:?}").contains("atelier"));
    }
    Ok(())
}
#[test]
fn complete_content_and_work_shortages_precede_original_missing_or_kind_member_causes() -> TestResult
{
    let root = root()?;
    let other = super::root()?;
    let selected = pin()?;
    for ignore in [false, true] {
        let mut source = Provider::new(&root, &other)?;
        if ignore {
            source.ignore_refusals();
        }
        let context = scope(&root)?
            .bind_drafting_context(&source, limits(1000, 10000, 1000)?)
            .map_err(|e| e.to_string())?;
        let text = source.text(&selected) + 24;
        let policy = DraftingRuleBindingLimits::new(limits(20, text, 138)?, 0);
        let error = context
            .prepare_rule_binding(
                id(999),
                SemanticOperationKind::Buttonhole,
                DraftingRuleSelection::Design,
                policy,
            )
            .err()
            .ok_or("whole content refusal")?;
        let cause = DesignResourceLimitError::Count {
            unit: DesignResourceCount::CollectionEntries,
            maximum: 20,
            consumed: 20,
            requested: 1,
            location: DesignResourceLocation {
                scope: DesignResourceScope::SourceHost,
                owner: Some(id(902)),
                field: "semantic.rule.inputs",
                index: None,
            },
        };
        assert!(
            matches!(error.refusal(),DraftingRuleWorkRefusal::Resources(actual) if actual==&cause)
        );
        assert!(error
            .source()
            .and_then(|e| e.downcast_ref::<DesignResourceLimitError>())
            .is_some());
        assert_eq!(source.query_count(), 0);
    }
    for (operation, inspections) in [(999, 840), (999, 841), (80, 841)] {
        let source = Provider::new(&root, &other)?;
        let context = scope(&root)?
            .bind_drafting_context(&source, limits(1000, 10000, 1000)?)
            .map_err(|e| e.to_string())?;
        let policy = DraftingRuleBindingLimits::new(
            limits(21, source.text(&selected) + 24, 138)?,
            inspections,
        );
        let error = context
            .prepare_rule_binding(
                id(operation),
                SemanticOperationKind::Buttonhole,
                DraftingRuleSelection::Design,
                policy,
            )
            .err()
            .ok_or("complete work/member refusal")?;
        assert_eq!(error.operation(), id(operation));
        assert_eq!(error.kind(), SemanticOperationKind::Buttonhole);
        assert_eq!(error.limits(), policy);
        assert!(core::ptr::eq(error.context().scope().root(), &root));
        assert!(matches!(error.selection(), DraftingRuleSelection::Design));
        match (operation, inspections, error.refusal()) {
            (999, 840, DraftingRuleWorkRefusal::MatchingInspections { allowed, required }) => {
                assert_eq!((*allowed, *required), (840, 841))
            }
            (
                999,
                841,
                DraftingRuleWorkRefusal::Operation(SemanticOperationLookupError::Missing {
                    operation,
                    expected,
                }),
            ) => assert_eq!(
                (*operation, *expected),
                (id(999), SemanticOperationKind::Buttonhole)
            ),
            (
                80,
                841,
                DraftingRuleWorkRefusal::Operation(SemanticOperationLookupError::Kind {
                    actual,
                    expected,
                }),
            ) => {
                assert_eq!(*expected, SemanticOperationKind::Buttonhole);
                assert!(core::ptr::eq(
                    *actual,
                    root.semantic_recipe()
                        .operations()
                        .get(1)
                        .ok_or("actual wrong-kind member")?
                ));
            }
            _ => return Err("first complete work/member cause".into()),
        }
        assert_eq!(source.query_count(), 0);
    }
    Ok(())
}
#[test]
fn actual_request_pin_text_refuses_before_work_member_or_source_lookup() -> TestResult {
    let root = root()?;
    let other = super::root()?;
    let source = Provider::new(&root, &other)?;
    let context = scope(&root)?
        .bind_drafting_context(&source, limits(1000, 10000, 1000)?)
        .map_err(|e| e.to_string())?;
    let selected = pin()?;
    let text = source.text(&selected) + 24 + 16;
    let policy = DraftingRuleBindingLimits::new(limits(21, text - 1, 138)?, 0);
    let error = context
        .prepare_rule_binding(
            id(999),
            SemanticOperationKind::Buttonhole,
            DraftingRuleSelection::Profile(&selected),
            policy,
        )
        .err()
        .ok_or("actual requested pin refusal")?;
    let cause = DesignResourceLimitError::Count {
        unit: DesignResourceCount::TextBytes,
        maximum: text - 1,
        consumed: text - 1,
        requested: 1,
        location: DesignResourceLocation {
            scope: DesignResourceScope::CommandState,
            owner: Some(id(1)),
            field: "selected_profile.content_version",
            index: None,
        },
    };
    assert!(matches!(error.refusal(),DraftingRuleWorkRefusal::Resources(actual) if actual==&cause));
    let DraftingRuleSelection::Profile(actual) = error.selection() else {
        return Err("original selection intent".into());
    };
    assert!(core::ptr::eq(actual, &selected));
    assert_eq!(source.query_count(), 0);
    Ok(())
}
