//! Actual source graph policy selection,unbound supplier execution and original failure ownership.
#![allow(clippy::expect_used)]
use sc_core::{ontology::EntityId, recipe::*, value::*};
fn id(value: u128) -> EntityId {
    EntityId::from_bits(value)
}
fn context() -> FormulaScalarExecutionContext<'static> {
    FormulaScalarExecutionContext::new(
        FormulaToleranceContext::empty(),
        FormulaSizeContext::empty(),
    )
}
#[path = "support/formula_tangent_graph.rs"]
mod graph;
use graph::Fixture;
fn exact(n: u128, d: u128) -> FormulaRational {
    FormulaRational::new(n, d, false).expect("independent original rational")
}
fn angle(n: u128) -> String {
    format!("{}.{:06} deg", n / 1_000_000, n % 1_000_000)
}
fn near_fixture() -> Fixture {
    let n = 247385123034593398372138492992328993860_u128;
    let d = 8635369834810561370044731507908276509_u128;
    Fixture::new(vec![
        (
            10,
            ScalarInputKind::Angle,
            format!("({}/{d})*cos(0 deg)", angle(n)),
            Some("angle_source"),
        ),
        (
            20,
            ScalarInputKind::Ratio,
            "tan(angle_source)".into(),
            Some("supplier"),
        ),
        (
            30,
            ScalarInputKind::Ratio,
            "supplier+supplier".into(),
            Some("entry"),
        ),
        (40, ScalarInputKind::Ratio, "tan(90 deg)".into(), None),
    ])
}

#[test]
fn complete_quotas_precede_missing_policy_in_unused_sources_and_keep_selected_limits() {
    let fixture = Fixture::new(vec![
        (10, ScalarInputKind::Ratio, "1.0".into(), None),
        (20, ScalarInputKind::Ratio, "tan(90 deg)".into(), None),
    ]);
    fixture.with_graph(|graph, limits| {
        let selected = FormulaTangentLimits::new(0, 0);
        let error = graph
            .prepare_scalar_sources_with_tangent_limits(
                FormulaSourceGraphExecutionLimits::new(0, 0, 0, 0, 0),
                selected,
            )
            .expect_err("complete source count first");
        assert!(matches!(
            error.refusal(),
            FormulaScalarSourceGraphPlanRefusal::SourceCount {
                allowed: 0,
                required: 2
            }
        ));
        assert_eq!(error.tangent_limits(), Some(selected));
        assert!(core::ptr::eq(error.graph(), graph));
        let error = graph
            .prepare_scalar_sources(limits)
            .expect_err("unused source still requires configured family");
        assert_eq!(error.tangent_limits(), None);
        let FormulaScalarSourceGraphPlanRefusal::MissingTangentPolicy { source_index, node } =
            error.refusal()
        else {
            unreachable!("actual missing policy")
        };
        assert_eq!(*source_index, 1);
        assert!(core::ptr::eq(
            node.expression(),
            graph
                .dependencies()
                .checked()
                .sources()
                .get(1)
                .expect("actual source")
                .proof()
                .expression()
        ));
    });
}

#[test]
fn explicit_zero_policy_keeps_unused_and_untaken_tangent_values_unexecuted() {
    let fixture = Fixture::new(vec![
        (10, ScalarInputKind::Ratio, "1.0".into(), None),
        (
            20,
            ScalarInputKind::Ratio,
            "if(1==1,1.0,tan(90 deg))".into(),
            None,
        ),
        (30, ScalarInputKind::Ratio, "tan(90 deg)".into(), None),
    ]);
    fixture.with_graph(|graph, limits| {
        let selected = FormulaTangentLimits::new(0, 0);
        let plan = graph
            .prepare_scalar_sources_with_tangent_limits(limits, selected)
            .expect("whole configuration");
        assert_eq!(plan.tangent_limits(), Some(selected));
        assert!(plan
            .sources()
            .iter()
            .all(|s| s.plan().tangent_limits() == Some(selected)));
        let result = plan
            .evaluate_source(id(20), context())
            .expect("only taken literal");
        assert_eq!(result.plan().tangent_limits(), Some(selected));
        assert_eq!(
            result.entry().value().numeric().expect("Ratio").rational(),
            exact(1_000_000, 1)
        );
        assert!(result.entry().value().sources().is_empty());
        assert_eq!(result.sources().count(), 1);
        assert!(result.source(id(10)).is_none() && result.source(id(30)).is_none());
    });
}

#[test]
fn real_full_width_angle_supplier_stays_unbound_and_shared_tangent_finishes_once() {
    let fixture = near_fixture();
    fixture.with_graph(|graph, limits| {
        let selected = FormulaTangentLimits::new(8219, 3584);
        let result = graph
            .prepare_scalar_sources_with_tangent_limits(limits, selected)
            .expect("all actual source plans")
            .evaluate_source(id(30), context())
            .expect("actual supplier demand");
        assert!(core::ptr::eq(result.plan().graph(), graph));
        assert_eq!(result.plan().tangent_limits(), Some(selected));
        assert_eq!(result.sources().count(), 3);
        assert!(result.source(id(40)).is_none());
        let original = result
            .source(id(10))
            .expect("actual unbound Angle supplier");
        assert_eq!(
            original.value().numeric().expect("Angle").rational(),
            exact(
                247385123034593398372138492992328993860,
                8635369834810561370044731507908276509
            )
        );
        assert_eq!(original.completion_ordinal(), 1);
        assert_eq!(
            result
                .source(id(20))
                .expect("native Tan supplier")
                .completion_ordinal(),
            2
        );
        assert_eq!(result.entry().completion_ordinal(), 3);
        assert_eq!(result.entry().reads().count(), 2);
        assert_eq!(
            result.entry().value().numeric().expect("Ratio").rational(),
            FormulaRational::ZERO
        );
        assert_eq!(
            result.entry().value().sources().iter().collect::<Vec<_>>(),
            [FormulaBuiltin::Cos, FormulaBuiltin::Tan]
        );
    });
}

#[test]
fn original_deep_supplier_resource_retains_both_policies_node_and_progress_without_prefix() {
    let fixture = near_fixture();
    fixture.with_graph(|graph, limits| {
        let selected = FormulaTangentLimits::new(4122, 3584);
        let error = graph
            .prepare_scalar_sources_with_tangent_limits(limits, selected)
            .expect("actual preparation")
            .evaluate_source(id(30), context())
            .expect_err("actual later native reservation");
        assert!(core::ptr::eq(error.graph(), graph));
        assert_eq!(error.tangent_limits(), Some(selected));
        assert_eq!(error.source_id(), id(30));
        assert_eq!(error.limits(), limits);
        let FormulaScalarSourceGraphExecutionCause::Evaluation {
            source_index,
            error: scalar,
        } = error.cause()
        else {
            unreachable!("original actual supplier")
        };
        assert_eq!(*source_index, 1);
        assert_eq!(scalar.tangent_limits(), Some(selected));
        let FormulaScalarExecutionErrorCause::Evaluation(failure) = scalar.cause() else {
            unreachable!("actual call")
        };
        let FormulaScalarExecutionFailure::Tangent(tangent) = failure.as_ref() else {
            unreachable!("actual tangent cause")
        };
        assert_eq!(tangent.limits(), selected);
        assert_eq!(tangent.diagnostic_code(), None);
        assert_eq!(
            tangent.input().sources().iter().collect::<Vec<_>>(),
            [FormulaBuiltin::Cos]
        );
        let native = tangent.execution_error().expect("original native progress");
        assert_eq!(native.precision(), Some(512));
        assert_eq!(native.charged_series_terms(), 1536);
        let FormulaTangentError::Resources(resource) = native.cause() else {
            unreachable!("exact next demand")
        };
        assert_eq!(resource.unit(), FormulaTangentResource::IntegerBits);
        assert_eq!(resource.requested(), 8219);
        let original = graph
            .dependencies()
            .checked()
            .sources()
            .get(1)
            .expect("original supplying expression");
        assert!(core::ptr::eq(
            scalar.node().expression(),
            original.proof().expression()
        ));
    });
}

#[test]
fn missing_entry_and_unrelated_failure_keep_the_graph_policy_before_any_tangent_value() {
    let fixture = Fixture::new(vec![(10, ScalarInputKind::Ratio, "1.0/0".into(), None)]);
    fixture.with_graph(|graph, limits| {
        let selected = FormulaTangentLimits::new(0, 0);
        let error = graph
            .prepare_scalar_sources_with_tangent_limits(limits, selected)
            .expect("configured")
            .evaluate_source(id(999), context())
            .expect_err("exact missing request");
        assert_eq!(error.tangent_limits(), Some(selected));
        assert_eq!(error.source_id(), id(999));
        assert!(matches!(
            error.cause(),
            FormulaScalarSourceGraphExecutionCause::MissingSource
        ));
        let error = graph
            .prepare_scalar_sources_with_tangent_limits(limits, selected)
            .expect("configured")
            .evaluate_source(id(10), context())
            .expect_err("original division");
        assert_eq!(error.tangent_limits(), Some(selected));
        let FormulaScalarSourceGraphExecutionCause::Evaluation { error, .. } = error.cause() else {
            unreachable!("actual failure")
        };
        assert_eq!(error.tangent_limits(), Some(selected));
        let FormulaScalarExecutionErrorCause::Evaluation(failure) = error.cause() else {
            unreachable!("original cause")
        };
        assert!(matches!(
            failure.as_ref(),
            FormulaScalarExecutionFailure::Numeric(_)
        ));
    });
}

#[test]
fn source_backed_recipe_retains_graph_policy_on_success_and_pre_value_foreign_owner_failure() {
    let fixture = Fixture::new(vec![(
        10,
        ScalarInputKind::Ratio,
        "tan(30 deg)".into(),
        Some("supplier"),
    )]);
    fixture.with_graph(|graph, limits| {
        let selected = FormulaTangentLimits::new(2073, 512);
        let recipe = FormulaRecipe::parse("let received: ratio = supplier")
            .expect("actual recipe")
            .normalize_literals()
            .expect("actual literals");
        let proof = recipe
            .check_kinds(fixture.namespace())
            .expect("actual names/kinds");
        let plan = proof
            .admit_execution(FormulaRecipeExecutionLimits::new(1, 256, 768, 1))
            .expect("actual allowance")
            .prepare_scalar(&fixture.inputs)
            .expect("legacy recipe has no direct Tan");
        let sources = graph
            .prepare_scalar_sources_with_tangent_limits(limits, selected)
            .expect("actual configured graph");
        let result = plan
            .evaluate_with_sources(sources, context())
            .expect("actual sourced recipe");
        assert_eq!(result.source_plan().tangent_limits(), Some(selected));
        assert_eq!(
            result
                .source(id(10))
                .expect("actual Tan supplier")
                .value()
                .numeric()
                .expect("Ratio")
                .rational(),
            exact(577350, 1)
        );
        assert_eq!(result.reads().count(), 1);
        let foreign = CanonicalInputs::new(vec![], vec![]).expect("actual foreign owner");
        let recipe = FormulaRecipe::parse("let received: ratio = 1.0")
            .expect("recipe")
            .normalize_literals()
            .expect("inputs");
        let proof = recipe
            .check_kinds(FormulaNamespace::new([]).expect("empty actual namespace"))
            .expect("actual proof");
        let plan = proof
            .admit_execution(FormulaRecipeExecutionLimits::new(1, 256, 768, 1))
            .expect("allowance")
            .prepare_scalar(&foreign)
            .expect("actual literal-only foreign plan");
        let sources = graph
            .prepare_scalar_sources_with_tangent_limits(limits, selected)
            .expect("original source plan");
        let error = plan
            .evaluate_with_sources(sources, context())
            .expect_err("canonical owner before allocation/value");
        assert_eq!(error.source_tangent_limits(), Some(selected));
        assert!(matches!(
            error.cause(),
            FormulaScalarSourceRecipeErrorCause::ForeignInputs { .. }
        ));
    });
}
