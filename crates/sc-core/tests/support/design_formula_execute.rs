//! Actual containing Design plan execution re-admits held content/current state before values.
use super::{
    formula_check::content,
    formula_scope::{observed_definition, ObservedHost},
    id,
    input_composition::{definition, MissingFixtureRecord},
    source_recipe::root,
};
use core::num::{NonZeroU64, NonZeroUsize};
use sc_core::{design::*, recipe::*, value::LengthValueError};

fn model_limits<H: DesignResourceSource>(root: &DesignInputs<H>) -> DesignResourceLimits {
    let mut meter = DesignResourceMeter::new(content());
    charge_design_input_resources(root, &mut meter).expect("all actual held input data");
    let demand = meter.finish().expect("complete logical content");
    DesignResourceLimits::new(
        NonZeroUsize::new(demand.collection_entries()).expect("entries"),
        NonZeroUsize::new(demand.text_bytes()).expect("text"),
        NonZeroU64::new(demand.counter_bits()).expect("bits"),
    )
}
fn with_plan<H: DesignResourceSource + DesignFormulaHost + 'static>(
    root: &DesignInputs<H>,
    limits: DesignResourceLimits,
    inspect: impl for<'a> FnOnce(
        DesignFormulaScalarPlan<'a, H>,
        &'a DesignFormulaPlanInputs<'a, H>,
        DesignFormulaScope<'a, H>,
    ),
) {
    let scope = root
        .prepare_formula_scope(limits)
        .expect("actual content/current scope");
    let s = root.formula_sources().sources().len();
    let n = root
        .formula_sources()
        .sources()
        .iter()
        .map(|s| s.expression().node_count())
        .sum();
    let normalized = scope
        .normalize_sources(FormulaSourceNormalizationLimits::new(s, n, s + 2 * n))
        .expect("actual owned source arenas");
    let sources = normalized
        .check_sources(FormulaSourceCheckLimits::new(
            s,
            n,
            scope.source_inputs().derived_inputs().count(),
            s + 2 * n,
        ))
        .expect("actual Design source proof");
    let normalized_recipe = scope.normalize_recipe().expect("actual ordered recipe");
    let recipe = normalized_recipe
        .check_kinds()
        .expect("actual Design recipe proof");
    let d = sources
        .proof()
        .sources()
        .iter()
        .map(|s| s.proof().dependencies().len())
        .sum();
    let deps = sources
        .proof()
        .prepare_dependencies(FormulaSourceDependencyLimits::new(s, d, s + d + 1))
        .expect("actual original relation");
    let graph = deps
        .admit_acyclic(FormulaSourceCycleLimits::new(s, 2 * s))
        .expect("actual acyclic relation");
    let statements = recipe.proof().statements().len();
    let names = recipe.proof().dependencies().count();
    let nodes = recipe
        .proof()
        .statements()
        .iter()
        .map(|s| match s.kind() {
            FormulaCheckedStatementKind::Let { expression } => expression.node_count(),
            FormulaCheckedStatementKind::Assert { left, right } => {
                left.node_count() + right.node_count()
            }
        })
        .sum();
    let inputs = sources
        .plan_inputs(
            &recipe,
            FormulaSourceGraphExecutionLimits::new(s, n, d, 2 * s + 2 * n + d, s),
            FormulaRecipeExecutionLimits::new(statements, nodes, 2 * nodes + names, statements),
        )
        .expect("same actual Design root/proof owners");
    let plan = inputs
        .prepare_scalar(&graph)
        .expect("complete actual native plans");
    inspect(plan, &inputs, scope);
}
fn context() -> FormulaScalarExecutionContext<'static> {
    FormulaScalarExecutionContext::new(
        FormulaToleranceContext::empty(),
        FormulaSizeContext::empty(),
    )
}
fn observed(
    text: &str,
) -> (
    DesignInputs<ObservedHost>,
    std::rc::Rc<core::cell::Cell<usize>>,
    std::rc::Rc<core::cell::Cell<bool>>,
) {
    let mut supplied = definition();
    supplied.recipe = FormulaRecipeDocument::new(text.into()).expect("actual ordered recipe");
    observed_definition(supplied)
}
#[test]
fn exact_actual_root_scope_limits_original_plans_and_complete_execution_survive_without_model_values(
) {
    let root = root(
        "let a:length=declared_width*3\nlet b:length=tripled_source\nassert equal:eps_num=a==b",
        "1 mm/3",
    );
    let limits = model_limits(&root);
    with_plan(&root, limits, |plan, inputs, original_scope| {
        let result = plan
            .evaluate(context())
            .expect("actual Design-backed complete execution");
        assert!(std::ptr::eq(result.inputs(), inputs));
        assert!(std::ptr::eq(result.scope().root(), &root));
        assert_eq!(result.scope().limits(), limits);
        assert_eq!(result.scope().demand(), original_scope.demand());
        assert!(std::ptr::eq(
            result.execution().recipe_plan().admission().proof(),
            inputs.recipe().proof()
        ));
        assert!(std::ptr::eq(
            result
                .execution()
                .source_plan()
                .graph()
                .dependencies()
                .checked(),
            inputs.sources().proof()
        ));
        assert_eq!(result.execution().statements().len(), 3);
        assert_eq!(result.execution().sources().count(), 2);
        assert_eq!(
            result
                .execution()
                .source(id(9003))
                .expect("actual source")
                .value()
                .numeric()
                .expect("Length")
                .rational(),
            FormulaRational::new(1000, 3, false).expect("unbound actual root")
        );
        assert_eq!(
            root.inputs()
                .length(id(40))
                .expect("original canonical state")
                .authored_value(),
            Err(LengthValueError::RequiresEvaluation {
                declaration: id(40),
                formula: id(9003)
            })
        );
    });
}
#[test]
fn host_needs_no_clone_or_debug_and_fresh_current_validation_runs_after_plan_preparation() {
    let (root, semantic, _) = observed("let actual:length=declared_width*3");
    let limits = model_limits(&root);
    with_plan(&root, limits, |plan, inputs, original_scope| {
        semantic.set(0);
        let result = plan
            .evaluate(context())
            .expect("host is neither Clone nor Debug");
        assert!(
            semantic.get() > 0,
            "actual current recheck occurs at execution admission"
        );
        assert!(std::ptr::eq(result.inputs(), inputs));
        assert!(std::ptr::eq(result.scope().root(), &root));
        assert_eq!(result.scope().demand(), original_scope.demand());
        assert_eq!(result.execution().statements().len(), 1);
    });
}
#[test]
fn late_current_host_refusal_keeps_actual_root_limits_and_original_cause_before_native_execution() {
    let (root, semantic, reject) = observed("let actual:length=declared_width*3");
    let limits = model_limits(&root);
    with_plan(&root, limits, |plan, inputs, _| {
        reject.set(true);
        semantic.set(0);
        let error = plan
            .evaluate(context())
            .expect_err("actual current host now refuses");
        assert!(semantic.get() > 0);
        assert!(std::ptr::eq(error.inputs(), inputs));
        assert!(error.scope().is_none());
        assert!(
            matches!(&error,DesignFormulaExecutionError::Admission{error,..}
            if std::ptr::eq(error.root(),&root) && error.limits()==limits
            && matches!(error.refusal(),DesignFormulaScopeRefusal::Inputs(DesignInputsError::Sources(cause))
                if matches!(cause.as_ref(),DesignMetadataError::Host(MissingFixtureRecord))))
        );
    });
}
#[test]
fn native_failure_retains_fresh_admitted_design_scope_and_actual_requester_supplier_context() {
    let root = root(
        "let safe:length=1 mm\nlet blocked:length=tripled_source",
        "if(flag,1 mm,2 mm)",
    );
    let limits = model_limits(&root);
    with_plan(&root, limits, |plan, inputs, original_scope| {
        let error = plan
            .evaluate(context())
            .expect_err("actual unknown supplier");
        assert!(std::ptr::eq(error.inputs(), inputs));
        let scope = error.scope().expect("actual input admission succeeded");
        assert!(std::ptr::eq(scope.root(), &root));
        assert_eq!(scope.limits(), limits);
        assert_eq!(scope.demand(), original_scope.demand());
        assert!(
            matches!(&error,DesignFormulaExecutionError::Execution{error,..}
            if std::ptr::eq(error.inputs(),root.inputs())
            && matches!(error.cause(),FormulaScalarSourceRecipeErrorCause::RequestedSource(request)
                if request.statement().statement_index()==2 && std::ptr::eq(request.input().inputs(),root.inputs())))
        );
    });
}
#[test]
fn empty_actual_recipe_keeps_fresh_containing_scope_without_an_entry_or_source_result() {
    let root = root("", "1 mm/3");
    let limits = model_limits(&root);
    with_plan(&root, limits, |plan, inputs, original_scope| {
        let result = plan.evaluate(context()).expect("actual empty recipe");
        assert!(std::ptr::eq(result.inputs(), inputs));
        assert!(std::ptr::eq(result.scope().root(), &root));
        assert_eq!(result.scope().demand(), original_scope.demand());
        assert!(result.execution().statements().is_empty());
        assert_eq!(result.execution().sources().count(), 0);
        assert_eq!(result.execution().reads().count(), 0);
    });
}

#[path = "design_formula_execute_oracle.rs"]
mod independent;
