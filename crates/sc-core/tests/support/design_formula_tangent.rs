//! Actual Design proof/policy joins and fresh admission before sourced native tangent execution.
use super::{
    formula_check::content,
    formula_scope::observed_definition,
    id,
    input_composition::{definition, MissingFixtureRecord},
    source_recipe::root,
};
use sc_core::{design::*, recipe::*, value::LengthValueError};

fn source_limits() -> FormulaSourceGraphExecutionLimits {
    FormulaSourceGraphExecutionLimits::new(16, 256, 256, 1024, 16)
}
fn recipe_limits() -> FormulaRecipeExecutionLimits {
    FormulaRecipeExecutionLimits::new(16, 256, 1024, 16)
}
fn context() -> FormulaScalarExecutionContext<'static> {
    FormulaScalarExecutionContext::new(
        FormulaToleranceContext::empty(),
        FormulaSizeContext::empty(),
    )
}
pub(super) fn with_checked<H: DesignResourceSource + DesignFormulaHost + 'static>(
    root: &DesignInputs<H>,
    inspect: impl for<'a> FnOnce(
        &'a DesignCheckedFormulaSources<'a, H>,
        &'a DesignCheckedFormulaRecipe<'a, H>,
        &'a FormulaAcyclicSourceDependencies<'a>,
        DesignFormulaScope<'a, H>,
    ),
) {
    let scope = root
        .prepare_formula_scope(content())
        .expect("actual complete admission");
    let normalized = scope
        .normalize_sources(FormulaSourceNormalizationLimits::new(16, 256, 1024))
        .expect("actual source arenas");
    let sources = normalized
        .check_sources(FormulaSourceCheckLimits::new(16, 256, 16, 1024))
        .expect("actual complete source proof");
    let normalized_recipe = scope.normalize_recipe().expect("actual ordered syntax");
    let recipe = normalized_recipe
        .check_kinds()
        .expect("actual whole recipe proof");
    let dependencies = sources
        .proof()
        .prepare_dependencies(FormulaSourceDependencyLimits::new(16, 256, 1024))
        .expect("actual complete relation");
    let graph = dependencies
        .admit_acyclic(FormulaSourceCycleLimits::new(16, 32))
        .expect("actual complete cycle proof");
    inspect(&sources, &recipe, &graph, scope);
}
fn native_tangent<'r>(
    error: &'r FormulaScalarExecutionError<'_>,
) -> &'r FormulaEvaluatedTangentError {
    let FormulaScalarExecutionErrorCause::Evaluation(failure) = error.cause() else {
        unreachable!("actual native call")
    };
    let FormulaScalarExecutionFailure::Tangent(error) = failure.as_ref() else {
        unreachable!("actual tangent failure")
    };
    error
}

#[test]
fn foreign_actual_root_refusal_retains_both_policies_before_all_quota_or_value_work() {
    let first = root("let actual:ratio=tan(30 deg)", "1 mm/3");
    let second = first.clone();
    assert_eq!(first, second);
    let supplier = FormulaTangentLimits::new(0, 0);
    let caller = FormulaTangentLimits::new(8219, 3584);
    with_checked(&first, |sources, _, _, _| {
        with_checked(&second, |_, recipe, _, _| {
            let error = sources
                .plan_inputs_with_tangent_limits(
                    recipe,
                    source_limits(),
                    recipe_limits(),
                    supplier,
                    caller,
                )
                .expect_err("equal content cannot substitute actual root identity");
            assert!(core::ptr::eq(error.sources(), sources));
            assert!(core::ptr::eq(error.recipe(), recipe));
            assert_eq!(error.source_tangent_limits(), Some(supplier));
            assert_eq!(error.recipe_tangent_limits(), Some(caller));
            let legacy = sources
                .plan_inputs(recipe, source_limits(), recipe_limits())
                .expect_err("same foreign root");
            assert_eq!(legacy.source_tangent_limits(), None);
            assert_eq!(legacy.recipe_tangent_limits(), None);
        });
    });
}

#[test]
fn exact_graph_then_recipe_then_source_quota_priority_retains_selected_policies() {
    let root = root("let actual:ratio=tan(30 deg)", "1 mm*tan(30 deg)");
    let supplier = FormulaTangentLimits::new(0, 0);
    let caller = FormulaTangentLimits::new(0, 0);
    with_checked(&root, |sources, recipe, graph, _| {
        let zero_source = FormulaSourceGraphExecutionLimits::new(0, 0, 0, 0, 0);
        let zero_recipe = FormulaRecipeExecutionLimits::new(0, 0, 0, 0);
        let inputs = sources
            .plan_inputs_with_tangent_limits(recipe, zero_source, zero_recipe, supplier, caller)
            .expect("same actual root");
        let other = sources
            .normalized()
            .check_sources(FormulaSourceCheckLimits::new(16, 256, 16, 1024))
            .expect("another proof over same original arenas");
        let deps = other
            .proof()
            .prepare_dependencies(FormulaSourceDependencyLimits::new(16, 256, 1024))
            .expect("actual other relation");
        let other_graph = deps
            .admit_acyclic(FormulaSourceCycleLimits::new(16, 32))
            .expect("actual other graph");
        let error = inputs
            .prepare_scalar(&other_graph)
            .expect_err("exact graph identity first");
        assert!(
            matches!(error.refusal(), DesignFormulaScalarPlanRefusal::ForeignGraph { graph: actual }
            if core::ptr::eq(*actual, &other_graph))
        );
        assert_eq!(error.inputs().source_tangent_limits(), Some(supplier));
        assert_eq!(error.inputs().recipe_tangent_limits(), Some(caller));
        let error = inputs
            .prepare_scalar(graph)
            .expect_err("whole recipe quota before source preparation");
        assert!(matches!(
            error.refusal(),
            DesignFormulaScalarPlanRefusal::RecipeAdmission(_)
        ));
        let inputs = sources
            .plan_inputs_with_tangent_limits(recipe, zero_source, recipe_limits(), supplier, caller)
            .expect("same actual owners");
        let error = inputs
            .prepare_scalar(graph)
            .expect_err("whole source quota before any Tan value");
        assert!(
            matches!(error.refusal(), DesignFormulaScalarPlanRefusal::Sources(cause)
            if cause.limits()==zero_source && cause.tangent_limits()==Some(supplier))
        );
        assert_eq!(error.inputs().recipe_tangent_limits(), Some(caller));
    });
}

#[test]
fn actual_design_keeps_unbound_roots_once_only_storage_flags_and_different_policies() {
    let root = root("let first:length=declared_width\nlet twice:length=first+first\nlet direct:ratio=tan(45 deg)\nassert checked:eps_geo=twice==0.002 mm", "1 mm*tan(30 deg)/1000");
    let original = root.clone();
    let supplier = FormulaTangentLimits::new(2073, 512);
    let caller = FormulaTangentLimits::new(156, 0);
    with_checked(&root, |sources, recipe, graph, scope| {
        let inputs = sources
            .plan_inputs_with_tangent_limits(
                recipe,
                source_limits(),
                recipe_limits(),
                supplier,
                caller,
            )
            .expect("actual independent selections");
        let plan = inputs
            .prepare_scalar(graph)
            .expect("actual complete native plans");
        assert_eq!(plan.sources().tangent_limits(), Some(supplier));
        assert_eq!(plan.recipe().tangent_limits(), Some(caller));
        let result = plan
            .evaluate(context())
            .expect("actual source-backed Design execution");
        assert!(core::ptr::eq(result.inputs(), &inputs));
        assert!(core::ptr::eq(result.scope().root(), &root));
        assert_eq!(result.scope().demand(), scope.demand());
        let execution = result.execution();
        assert!(core::ptr::eq(execution.source_plan().graph(), graph));
        assert!(core::ptr::eq(
            execution.recipe_plan().admission().proof(),
            recipe.proof()
        ));
        assert_eq!(execution.source_plan().tangent_limits(), Some(supplier));
        assert_eq!(execution.recipe_plan().tangent_limits(), Some(caller));
        let source = execution
            .source(id(9003))
            .expect("actual completed supplier");
        assert_eq!(
            source.value().numeric().expect("Length").rational(),
            FormulaRational::new(11547, 20000, false).expect("unbound native source")
        );
        assert_eq!(
            execution.sources().count(),
            1,
            "unused second supplier never executes"
        );
        assert_eq!(execution.statements().len(), 4);
        for (index, expected) in [(0, 1), (1, 2), (2, 1_000_000)] {
            let FormulaScalarRecipeOutcome::Binding { binding, .. } = execution
                .statements()
                .get(index)
                .expect("actual statement")
                .outcome()
            else {
                unreachable!("real let")
            };
            let value = binding.read().expect("actual stored read");
            assert_eq!(
                value.numeric().expect("numeric").rational(),
                FormulaRational::from_signed_integer(expected)
            );
            assert!(value.sources().contains(FormulaBuiltin::Tan));
        }
        assert_eq!(
            root.inputs()
                .length(id(40))
                .expect("canonical Derived")
                .authored_value(),
            Err(LengthValueError::RequiresEvaluation {
                declaration: id(40),
                formula: id(9003)
            })
        );
    });
    assert_eq!(root, original, "no authored model mutation");
}

#[test]
fn zero_supplier_policy_admits_unused_poles_without_executing_or_defaulting_them() {
    let root = root("let actual:ratio=tan(45 deg)", "1 mm*tan(90 deg)");
    let supplier = FormulaTangentLimits::new(0, 0);
    let caller = FormulaTangentLimits::new(156, 0);
    with_checked(&root, |sources, recipe, graph, _| {
        let legacy = sources
            .plan_inputs(recipe, source_limits(), recipe_limits())
            .expect("root");
        let error = legacy
            .prepare_scalar(graph)
            .expect_err("complete unused source missing policy");
        assert!(
            matches!(error.refusal(), DesignFormulaScalarPlanRefusal::Sources(cause)
            if matches!(cause.refusal(), FormulaScalarSourceGraphPlanRefusal::MissingTangentPolicy { source_index: 0, .. }))
        );
        let inputs = sources
            .plan_inputs_with_tangent_limits(
                recipe,
                source_limits(),
                recipe_limits(),
                supplier,
                caller,
            )
            .expect("explicit zero policy is present");
        let result = inputs
            .prepare_scalar(graph)
            .expect("all capabilities admitted")
            .evaluate(context())
            .expect("unused mathematical pole remains unread");
        assert_eq!(result.execution().sources().count(), 0);
        assert_eq!(result.inputs().source_tangent_limits(), Some(supplier));
        assert_eq!(result.inputs().recipe_tangent_limits(), Some(caller));
    });
}

#[test]
fn each_actual_owner_keeps_its_own_resource_refusal_under_a_larger_other_policy() {
    for supplier_fails in [true, false] {
        let (recipe_text, source_text, supplier, caller) = if supplier_fails {
            (
                "let request:length=declared_width",
                "1 mm*tan(30 deg)",
                FormulaTangentLimits::new(156, 0),
                FormulaTangentLimits::new(16412, 7680),
            )
        } else {
            (
                "let prior:length=declared_width\nlet request:ratio=tan(30 deg)",
                "1 mm*tan(45 deg)",
                FormulaTangentLimits::new(16412, 7680),
                FormulaTangentLimits::new(156, 0),
            )
        };
        let root = root(recipe_text, source_text);
        with_checked(&root, |sources, recipe, graph, scope| {
            let inputs = sources
                .plan_inputs_with_tangent_limits(
                    recipe,
                    source_limits(),
                    recipe_limits(),
                    supplier,
                    caller,
                )
                .expect("same actual owners");
            let error = inputs
                .prepare_scalar(graph)
                .expect("actual native plans")
                .evaluate(context())
                .expect_err("original selected owner budget refuses");
            assert!(core::ptr::eq(error.inputs(), &inputs));
            assert_eq!(
                error.scope().expect("fresh complete scope").demand(),
                scope.demand()
            );
            let DesignFormulaExecutionError::Execution { error, .. } = &error else {
                unreachable!("actual native refusal after fresh admission")
            };
            assert_eq!(error.source_tangent_limits(), Some(supplier));
            assert_eq!(error.recipe_tangent_limits(), Some(caller));
            let scalar = match error.cause() {
                FormulaScalarSourceRecipeErrorCause::RequestedSource(request) if supplier_fails => {
                    assert_eq!(request.statement().statement_index(), 1);
                    let FormulaScalarSourceGraphExecutionCause::Evaluation {
                        source_index,
                        error,
                    } = request.cause()
                    else {
                        unreachable!("actual supplier call")
                    };
                    assert_eq!(*source_index, 0);
                    error.as_ref()
                }
                FormulaScalarSourceRecipeErrorCause::Recipe(error) if !supplier_fails => {
                    assert_eq!(
                        error
                            .statement()
                            .expect("actual statement")
                            .statement_index(),
                        2
                    );
                    assert_eq!(error.tangent_limits(), Some(caller));
                    let FormulaScalarRecipeErrorCause::Evaluation(failure) = error.cause() else {
                        unreachable!("actual recipe operand")
                    };
                    let FormulaScalarRecipeFailure::Expression { error, .. } = failure.as_ref()
                    else {
                        unreachable!("actual recipe call")
                    };
                    error
                }
                _ => unreachable!("original failing owner"),
            };
            let selected = if supplier_fails { supplier } else { caller };
            assert_eq!(scalar.tangent_limits(), Some(selected));
            let tangent = native_tangent(scalar);
            assert_eq!(tangent.limits(), selected);
            let FormulaTangentError::Resources(resource) = tangent
                .preparation_error()
                .expect("initial admission refuses before computing series")
            else {
                unreachable!("actual scheduled resource demand")
            };
            assert_eq!(resource.requested(), 2073);
        });
    }
}

#[test]
fn fresh_nonclone_host_rejection_precedes_tangent_poles_and_retains_both_policies() {
    let mut supplied = definition();
    supplied.recipe =
        FormulaRecipeDocument::new("let actual:ratio=tan(90 deg)".into()).expect("actual recipe");
    let (root, semantic, reject) = observed_definition(supplied);
    let supplier = FormulaTangentLimits::new(0, 0);
    let caller = FormulaTangentLimits::new(156, 0);
    with_checked(&root, |sources, recipe, graph, _| {
        let inputs = sources
            .plan_inputs_with_tangent_limits(
                recipe,
                source_limits(),
                recipe_limits(),
                supplier,
                caller,
            )
            .expect("actual non-Clone/non-Debug host owners");
        let plan = inputs
            .prepare_scalar(graph)
            .expect("no values during plan preparation");
        reject.set(true);
        semantic.set(0);
        let error = plan
            .evaluate(context())
            .expect_err("fresh actual currentness wins before pole");
        assert!(semantic.get() > 0);
        assert!(error.scope().is_none());
        assert!(core::ptr::eq(error.inputs(), &inputs));
        assert_eq!(error.inputs().source_tangent_limits(), Some(supplier));
        assert_eq!(error.inputs().recipe_tangent_limits(), Some(caller));
        assert!(
            matches!(&error, DesignFormulaExecutionError::Admission { error, .. }
            if core::ptr::eq(error.root(), &root) && error.limits()==content()
            && matches!(error.refusal(), DesignFormulaScopeRefusal::Inputs(DesignInputsError::Sources(cause))
                if matches!(cause.as_ref(), DesignMetadataError::Host(MissingFixtureRecord))))
        );
    });
}
