//! Actual direct and source-backed ordered recipe Tan calls with independently retained policies.
#![allow(clippy::expect_used)]
use sc_core::{name::MachineToken, ontology::EntityId, recipe::*, value::*};
#[path = "support/formula_tangent_graph.rs"]
mod graph;
use graph::Fixture;
fn id(value: u128) -> EntityId {
    EntityId::from_bits(value)
}
fn context() -> FormulaScalarExecutionContext<'static> {
    FormulaScalarExecutionContext::new(
        FormulaToleranceContext::empty(),
        FormulaSizeContext::empty(),
    )
}
fn allowance() -> FormulaRecipeExecutionLimits {
    FormulaRecipeExecutionLimits::new(16, 256, 768, 16)
}
fn failure<'a, 'r>(error: &'r FormulaScalarRecipeError<'a>) -> &'r FormulaScalarRecipeFailure<'a> {
    let FormulaScalarRecipeErrorCause::Evaluation(failure) = error.cause() else {
        unreachable!("actual semantic failure")
    };
    failure
}
fn scalar<'a, 'r>(error: &'r FormulaScalarRecipeError<'a>) -> &'r FormulaScalarExecutionError<'a> {
    let FormulaScalarRecipeFailure::Expression { error, .. } = failure(error) else {
        unreachable!("actual operand failure")
    };
    error
}
fn tangent<'a, 'r>(error: &'r FormulaScalarRecipeError<'a>) -> &'r FormulaEvaluatedTangentError {
    let FormulaScalarExecutionErrorCause::Evaluation(failure) = scalar(error).cause() else {
        unreachable!("actual native call")
    };
    let FormulaScalarExecutionFailure::Tangent(error) = failure.as_ref() else {
        unreachable!("actual Tan failure")
    };
    error
}
fn angle(n: u128) -> String {
    format!("{}.{:06} deg", n / 1_000_000, n % 1_000_000)
}

#[test]
fn whole_missing_policy_precedes_earlier_joins_and_configured_failure_retains_selected_limits() {
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("inventory");
    let name = MachineToken::new("private_missing").expect("name");
    let namespace =
        FormulaNamespace::new([
            FormulaInitialDeclaration::try_from(FormulaDeclaration::input(
                &name,
                FormulaScalarInputOrigin::Parameter,
                id(1),
                id(99),
                FormulaBindingKind::Length,
            ))
            .expect("actual metadata"),
        ])
        .expect("namespace");
    let recipe = FormulaRecipe::parse(
        "let first:length=private_missing\nassert later:eps_phys=1.0==tan(0 deg)",
    )
    .expect("actual syntax")
    .normalize_literals()
    .expect("actual arenas");
    let proof = recipe.check_kinds(namespace).expect("whole actual kinds");
    let error = proof
        .admit_execution(allowance())
        .expect("complete admission")
        .prepare_scalar(&inputs)
        .expect_err("missing family policy before earlier source join");
    assert!(matches!(
        error.refusal(),
        FormulaScalarRecipePlanRefusal::MissingTangentPolicy
    ));
    assert_eq!(error.tangent_limits(), None);
    assert_eq!(
        error.statement().expect("actual owner").statement_index(),
        2
    );
    let selected = FormulaTangentLimits::new(156, 0);
    let error = proof
        .admit_execution(allowance())
        .expect("admission")
        .prepare_scalar_with_tangent_limits(&inputs, selected)
        .expect_err("actual missing canonical target");
    assert_eq!(error.tangent_limits(), Some(selected));
    assert!(matches!(
        error.refusal(),
        FormulaScalarRecipePlanRefusal::CanonicalInput(_)
    ));
    assert_eq!(
        error
            .statement()
            .expect("actual first statement")
            .statement_index(),
        1
    );
}

#[test]
fn exact_calls_keep_tan_through_false_storage_selected_branches_and_real_t2_assertion() {
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("inventory");
    let recipe=FormulaRecipe::parse("let first:ratio=tan(45 deg)\nlet zero:ratio=first-first\nlet flag:boolean=zero!=0.0\nlet chosen:length=if(flag,eps_phys,1 mm)\nassert checked:eps_geo=chosen==1 mm")
        .expect("actual statements").normalize_literals().expect("actual literals");
    let proof = recipe
        .check_kinds(FormulaNamespace::new([]).expect("namespace"))
        .expect("whole kinds");
    let selected = FormulaTangentLimits::new(156, 0);
    let result = proof
        .admit_execution(allowance())
        .expect("admission")
        .prepare_scalar_with_tangent_limits(&inputs, selected)
        .expect("whole configured plan")
        .evaluate(context())
        .expect("actual declarations/assertion");
    assert_eq!(result.plan().tangent_limits(), Some(selected));
    assert_eq!(result.statements().len(), 5);
    assert!(core::ptr::eq(result.plan().inputs(), &inputs));
    assert!(core::ptr::eq(result.plan().admission().proof(), &proof));
    let FormulaScalarRecipeOutcome::Binding {
        expression,
        binding,
    } = result
        .statements()
        .get(2)
        .expect("actual false let")
        .outcome()
    else {
        unreachable!("actual binding")
    };
    assert_eq!(expression.boolean(), Some(false));
    let read = binding.read().expect("actual stored Boolean");
    assert_eq!(read.boolean(), Some(false));
    assert_eq!(
        read.sources().iter().collect::<Vec<_>>(),
        [FormulaBuiltin::Tan]
    );
    assert_eq!(
        result
            .reads()
            .filter(|r| r.dependency().declaration().name() == "eps_phys")
            .count(),
        0
    );
    for statement in result.statements().iter().take(4) {
        let FormulaScalarRecipeOutcome::Binding {
            expression,
            binding,
        } = statement.outcome()
        else {
            unreachable!("let")
        };
        assert!(expression.sources().contains(FormulaBuiltin::Tan));
        assert!(binding.sources().contains(FormulaBuiltin::Tan));
    }
}

#[test]
fn actual_tangent_then_fraction_binds_once_and_later_reads_only_the_stored_integer() {
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("inventory");
    let recipe=FormulaRecipe::parse("let first:ratio=tan(30 deg)/1000000\nlet repeated:ratio=first+first\nlet independent:ratio=tan(30 deg)")
        .expect("actual recipe").normalize_literals().expect("actual arenas");
    let proof = recipe
        .check_kinds(FormulaNamespace::new([]).expect("namespace"))
        .expect("whole proof");
    let selected = FormulaTangentLimits::new(2073, 512);
    let result = proof
        .admit_execution(allowance())
        .expect("admission")
        .prepare_scalar_with_tangent_limits(&inputs, selected)
        .expect("configured")
        .evaluate(context())
        .expect("two independent native budgets");
    let FormulaScalarRecipeOutcome::Binding {
        expression,
        binding,
    } = result.statements().first().expect("actual first").outcome()
    else {
        unreachable!("binding")
    };
    assert_eq!(
        expression.numeric().expect("unbound Ratio").rational(),
        FormulaRational::new(577350, 1_000_000, false).expect("exact fraction")
    );
    assert_eq!(
        binding
            .read()
            .expect("actual stored value")
            .numeric()
            .expect("Ratio")
            .rational(),
        FormulaRational::ONE
    );
    let FormulaScalarRecipeOutcome::Binding { expression, .. } =
        result.statements().get(1).expect("second actual").outcome()
    else {
        unreachable!("binding")
    };
    assert_eq!(
        expression
            .numeric()
            .expect("actual stored reads")
            .rational(),
        FormulaRational::from_signed_integer(2)
    );
    assert!(result
        .reads()
        .all(|r| r.value().sources().contains(FormulaBuiltin::Tan)));
}

#[test]
fn assertion_left_right_then_header_and_t1_causes_retain_original_roles_and_policy() {
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("inventory");
    let selected = FormulaTangentLimits::new(156, 0);
    for (text, role) in [
        (
            "assert check:eps_phys=tan(90 deg)==1.0/0",
            Some(FormulaStatementExpression::AssertionLeft),
        ),
        (
            "assert check:eps_phys=0.0==tan(90 deg)",
            Some(FormulaStatementExpression::AssertionRight),
        ),
        ("assert check:eps_phys=tan(0 deg)==0.0", None),
    ] {
        let recipe = FormulaRecipe::parse(text)
            .expect("actual recipe")
            .normalize_literals()
            .expect("arenas");
        let proof = recipe
            .check_kinds(FormulaNamespace::new([]).expect("namespace"))
            .expect("actual kinds");
        let error = proof
            .admit_execution(allowance())
            .expect("admission")
            .prepare_scalar_with_tangent_limits(&inputs, selected)
            .expect("configured")
            .evaluate(context())
            .expect_err("actual ordered first refusal");
        assert_eq!(error.tangent_limits(), Some(selected));
        assert_eq!(
            error.statement().expect("actual owner").statement_index(),
            1
        );
        if let Some(role) = role {
            let FormulaScalarRecipeFailure::Expression { part, .. } = failure(&error) else {
                unreachable!("actual expression")
            };
            assert_eq!(*part, role);
            assert_eq!(scalar(&error).tangent_limits(), Some(selected));
            assert_eq!(
                tangent(&error).preparation_error(),
                Some(FormulaTangentError::Pole {
                    input: FormulaRational::from_signed_integer(90_000_000)
                })
            );
        } else {
            assert!(matches!(
                failure(&error),
                FormulaScalarRecipeFailure::ToleranceRead(_)
            ));
        }
    }
    let recipe =
        FormulaRecipe::parse("let first:ratio=tan(0 deg)\nassert refused:eps_num=first==0.0")
            .expect("recipe")
            .normalize_literals()
            .expect("inputs");
    let proof = recipe
        .check_kinds(FormulaNamespace::new([]).expect("namespace"))
        .expect("actual proof");
    let error = proof
        .admit_execution(allowance())
        .expect("admission")
        .prepare_scalar_with_tangent_limits(&inputs, selected)
        .expect("configured")
        .evaluate(context())
        .expect_err("actual T1 source refusal after storage");
    assert_eq!(error.tangent_limits(), Some(selected));
    assert_eq!(
        error.statement().expect("actual owner").statement_index(),
        2
    );
    assert!(matches!(
        failure(&error),
        FormulaScalarRecipeFailure::Comparison(_)
    ));
}

#[test]
fn later_native_resource_and_unrelated_failure_keep_recipe_and_scalar_policies() {
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("inventory");
    let n = 247385123034593398372138492992328993860_u128;
    let d = 8635369834810561370044731507908276509_u128;
    let text = format!("let result:ratio=tan((({}/{d})*cos(0 deg)))", angle(n));
    let recipe = FormulaRecipe::parse(&text)
        .expect("actual syntax")
        .normalize_literals()
        .expect("actual full-width Angle");
    let proof = recipe
        .check_kinds(FormulaNamespace::new([]).expect("namespace"))
        .expect("whole actual proof");
    let selected = FormulaTangentLimits::new(4122, 3584);
    let error = proof
        .admit_execution(allowance())
        .expect("admission")
        .prepare_scalar_with_tangent_limits(&inputs, selected)
        .expect("actual policy")
        .evaluate(context())
        .expect_err("next actual reservation");
    assert_eq!(error.tangent_limits(), Some(selected));
    assert_eq!(scalar(&error).tangent_limits(), Some(selected));
    let native = tangent(&error).execution_error().expect("original native");
    assert_eq!(native.precision(), Some(512));
    let FormulaTangentError::Resources(resource) = native.cause() else {
        unreachable!("original resource")
    };
    assert_eq!(resource.requested(), 8219);
    assert_eq!(resource.charged_series_terms(), 1536);
    let recipe = FormulaRecipe::parse("let first:length=1 mm/0")
        .expect("syntax")
        .normalize_literals()
        .expect("inputs");
    let proof = recipe
        .check_kinds(FormulaNamespace::new([]).expect("namespace"))
        .expect("proof");
    let error = proof
        .admit_execution(allowance())
        .expect("admission")
        .prepare_scalar_with_tangent_limits(&inputs, selected)
        .expect("configured unused policy")
        .evaluate(context())
        .expect_err("original earlier division");
    assert_eq!(error.tangent_limits(), Some(selected));
    assert_eq!(scalar(&error).tangent_limits(), Some(selected));
}

#[test]
fn graph_and_recipe_calls_use_their_own_original_selected_policies_during_suspension() {
    let n = 247385123034593398372138492992328993860_u128;
    let d = 8635369834810561370044731507908276509_u128;
    let fixture = Fixture::new(vec![(
        10,
        ScalarInputKind::Angle,
        format!("({}/{d})*cos(0 deg)", angle(n)),
        Some("angle_source"),
    )]);
    fixture.with_graph(|graph, limits| {
        let recipe =
            FormulaRecipe::parse("let first:ratio=tan(angle_source)\nlet second:ratio=tan(0 deg)")
                .expect("actual recipe")
                .normalize_literals()
                .expect("actual arenas");
        let proof = recipe
            .check_kinds(fixture.namespace())
            .expect("whole actual proof");
        let graph_policy = FormulaTangentLimits::new(0, 0);
        let recipe_policy = FormulaTangentLimits::new(8219, 3584);
        let plan = proof
            .admit_execution(allowance())
            .expect("admission")
            .prepare_scalar_with_tangent_limits(&fixture.inputs, recipe_policy)
            .expect("actual recipe policy");
        let sources = graph
            .prepare_scalar_sources_with_tangent_limits(limits, graph_policy)
            .expect("actual graph policy");
        let result = plan
            .evaluate_with_sources(sources, context())
            .expect("source policy cannot replace recipe call policy");
        assert_eq!(result.recipe_plan().tangent_limits(), Some(recipe_policy));
        assert_eq!(result.source_plan().tangent_limits(), Some(graph_policy));
        assert_eq!(
            result
                .source(id(10))
                .expect("actual unbound Angle")
                .value()
                .numeric()
                .expect("Angle")
                .rational(),
            FormulaRational::new(n, d, false).expect("original fraction")
        );
        let graph_policy = FormulaTangentLimits::new(16412, 7680);
        let recipe_policy = FormulaTangentLimits::new(2073, 512);
        let plan = proof
            .admit_execution(allowance())
            .expect("admission")
            .prepare_scalar_with_tangent_limits(&fixture.inputs, recipe_policy)
            .expect("recipe policy");
        let sources = graph
            .prepare_scalar_sources_with_tangent_limits(limits, graph_policy)
            .expect("source policy");
        let error = plan
            .evaluate_with_sources(sources, context())
            .expect_err("recipe budget remains its own");
        assert_eq!(error.recipe_tangent_limits(), Some(recipe_policy));
        assert_eq!(error.source_tangent_limits(), Some(graph_policy));
        let FormulaScalarSourceRecipeErrorCause::Recipe(original) = error.cause() else {
            unreachable!("actual recipe call")
        };
        assert_eq!(original.tangent_limits(), Some(recipe_policy));
        assert_eq!(tangent(original).limits(), recipe_policy);
    });
}

#[test]
fn paired_source_failure_and_foreign_owner_keep_both_independent_policies() {
    let n = 247385123034593398372138492992328993860_u128;
    let d = 8635369834810561370044731507908276509_u128;
    let fixture = Fixture::new(vec![(
        10,
        ScalarInputKind::Ratio,
        format!("tan({}/{d})", angle(n)),
        Some("supplier"),
    )]);
    fixture.with_graph(|graph, limits| {
        let recipe = FormulaRecipe::parse("let result:ratio=supplier")
            .expect("syntax")
            .normalize_literals()
            .expect("arenas");
        let proof = recipe.check_kinds(fixture.namespace()).expect("proof");
        let source_policy = FormulaTangentLimits::new(2073, 512);
        let recipe_policy = FormulaTangentLimits::new(16412, 7680);
        let plan = proof
            .admit_execution(allowance())
            .expect("admission")
            .prepare_scalar_with_tangent_limits(&fixture.inputs, recipe_policy)
            .expect("recipe policy");
        let sources = graph
            .prepare_scalar_sources_with_tangent_limits(limits, source_policy)
            .expect("source policy");
        let error = plan
            .evaluate_with_sources(sources, context())
            .expect_err("supplier keeps its own smaller budget");
        assert_eq!(error.source_tangent_limits(), Some(source_policy));
        assert_eq!(error.recipe_tangent_limits(), Some(recipe_policy));
        assert!(matches!(
            error.cause(),
            FormulaScalarSourceRecipeErrorCause::RequestedSource(_)
        ));
        let foreign = CanonicalInputs::new(vec![], vec![]).expect("actual foreign owner");
        let recipe = FormulaRecipe::parse("let result:ratio=1.0")
            .expect("syntax")
            .normalize_literals()
            .expect("inputs");
        let proof = recipe
            .check_kinds(FormulaNamespace::new([]).expect("namespace"))
            .expect("proof");
        let plan = proof
            .admit_execution(allowance())
            .expect("admission")
            .prepare_scalar_with_tangent_limits(&foreign, recipe_policy)
            .expect("actual foreign plan");
        let sources = graph
            .prepare_scalar_sources_with_tangent_limits(limits, source_policy)
            .expect("actual source plan");
        let error = plan
            .evaluate_with_sources(sources, context())
            .expect_err("foreign owner before allocations");
        assert_eq!(error.source_tangent_limits(), Some(source_policy));
        assert_eq!(error.recipe_tangent_limits(), Some(recipe_policy));
        assert!(matches!(
            error.cause(),
            FormulaScalarSourceRecipeErrorCause::ForeignInputs { .. }
        ));
    });
}

#[test]
fn wide_native_ratio_reaches_only_the_original_binding_refusal_with_selected_policy() {
    let d = 100_000_000_000_000_000_000_u128;
    let n = 90_000_000 * d - 1;
    let text = format!("let result:ratio=tan({}/{d})", angle(n));
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("actual inventory");
    let recipe = FormulaRecipe::parse(&text)
        .expect("actual syntax")
        .normalize_literals()
        .expect("actual full Angle");
    let proof = recipe
        .check_kinds(FormulaNamespace::new([]).expect("actual namespace"))
        .expect("actual kinds");
    let selected = FormulaTangentLimits::new(16412, 7680);
    let error = proof
        .admit_execution(allowance())
        .expect("actual admission")
        .prepare_scalar_with_tangent_limits(&inputs, selected)
        .expect("actual plan")
        .evaluate(context())
        .expect_err("native finite value exceeds stored Ratio domain");
    assert_eq!(error.tangent_limits(), Some(selected));
    assert_eq!(error.statement().expect("actual let").statement_index(), 1);
    assert!(matches!(
        failure(&error),
        FormulaScalarRecipeFailure::Binding {
            kind: FormulaKind::Ratio,
            ..
        }
    ));
}
