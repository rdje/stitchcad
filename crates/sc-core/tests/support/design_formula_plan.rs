//! Actual proof/root identity and original complete component plan admissions.
use super::{
    formula_check::{content, registry},
    id,
    input_composition::{definition, Markers},
};
use sc_core::{
    design::{DesignFormulaScalarPlanRefusal as Refusal, DesignInputs},
    recipe::{
        FormulaBuiltin, FormulaRecipeDocument, FormulaRecipeExecutionAdmissionRefusal,
        FormulaRecipeExecutionLimits, FormulaScalarRecipePlanRefusal,
        FormulaScalarRecipeSourceKind, FormulaScalarSourceGraphPlanRefusal,
        FormulaSourceCheckLimits, FormulaSourceCycleLimits, FormulaSourceDependencyLimits,
        FormulaSourceGraphExecutionLimits, FormulaSourceNormalizationLimits,
    },
    value::LengthValueError,
};
const RECIPE: &str = "let authored:length=declared_width\nlet twice:length=authored+authored";
fn root(recipe: &str) -> DesignInputs<Markers> {
    let mut supplied = definition();
    supplied.recipe = FormulaRecipeDocument::new(recipe.into()).expect("actual recipe syntax");
    DesignInputs::new(supplied).expect("actual complete input root")
}
fn syntax() -> FormulaSourceNormalizationLimits {
    FormulaSourceNormalizationLimits::new(10, 100, 210)
}
fn kinds() -> FormulaSourceCheckLimits {
    FormulaSourceCheckLimits::new(10, 100, 10, 210)
}
fn edges() -> FormulaSourceDependencyLimits {
    FormulaSourceDependencyLimits::new(10, 100, 111)
}
fn cycles() -> FormulaSourceCycleLimits {
    FormulaSourceCycleLimits::new(10, 20)
}
fn source_limits() -> FormulaSourceGraphExecutionLimits {
    FormulaSourceGraphExecutionLimits::new(1, 3, 0, 8, 1)
}
fn recipe_limits() -> FormulaRecipeExecutionLimits {
    FormulaRecipeExecutionLimits::new(2, 4, 11, 2)
}
#[test]
fn exact_complete_native_plans_keep_original_owners_limits_canonical_targets_and_prior_lets() {
    let root = root(RECIPE);
    let scope = root.prepare_formula_scope(content()).expect("actual scope");
    let sources = scope
        .normalize_sources(syntax())
        .expect("actual source syntax");
    let sources = sources.check_sources(kinds()).expect("actual source proof");
    let recipe = scope.normalize_recipe().expect("actual recipe syntax");
    let recipe = recipe.check_kinds().expect("actual recipe proof");
    let inputs = sources
        .plan_inputs(&recipe, source_limits(), recipe_limits())
        .expect("same actual root");
    let dependencies = inputs
        .sources()
        .proof()
        .prepare_dependencies(edges())
        .expect("actual complete relation");
    let graph = dependencies
        .admit_acyclic(cycles())
        .expect("actual complete acyclic graph");
    let plan = inputs
        .prepare_scalar(&graph)
        .expect("exact complete native plan allowances");
    assert!(std::ptr::eq(plan.inputs(), &inputs));
    assert!(std::ptr::eq(plan.scope().root(), &root));
    assert!(std::ptr::eq(plan.sources().graph(), &graph));
    assert!(std::ptr::eq(
        plan.sources().graph().dependencies().checked(),
        sources.proof()
    ));
    assert!(std::ptr::eq(
        plan.recipe().admission().proof(),
        recipe.proof()
    ));
    assert!(std::ptr::eq(plan.recipe().inputs(), root.inputs()));
    assert_eq!(plan.sources().limits(), source_limits());
    assert_eq!(plan.recipe().admission().limits(), recipe_limits());
    assert_eq!(
        (
            plan.sources().node_count(),
            plan.sources().name_use_count(),
            plan.sources().workspace_records(),
            plan.sources().output_records()
        ),
        (3, 0, 8, 1)
    );
    assert_eq!(
        (
            plan.recipe().admission().node_count(),
            plan.recipe().admission().name_use_count(),
            plan.recipe().admission().workspace_records(),
            plan.recipe().admission().output_records()
        ),
        (4, 3, 11, 2)
    );
    let mut occurrences = plan.recipe().sources().iter();
    assert!(matches!(
        occurrences
            .next()
            .expect("actual canonical dependency")
            .kind(),
        FormulaScalarRecipeSourceKind::Canonical(_)
    ));
    for occurrence in occurrences {
        assert!(
            matches!(occurrence.kind(),FormulaScalarRecipeSourceKind::PriorLet(statement)
            if std::ptr::eq(statement,recipe.proof().statements().first().expect("actual first let")))
        );
    }
    assert_eq!(
        root.inputs()
            .length(id(40))
            .expect("canonical Derived unchanged")
            .authored_value(),
        Err(LengthValueError::RequiresEvaluation {
            declaration: id(40),
            formula: id(9003)
        })
    );
}
#[test]
fn equal_ids_revision_and_complete_model_content_do_not_authorize_cross_root_recipe_proofs() {
    let first = root(RECIPE);
    let second = first.clone();
    assert_eq!(first, second);
    assert_eq!(first.id(), second.id());
    assert_eq!(first.revision(), second.revision());
    assert!(!std::ptr::eq(&first, &second));
    let source_owner = first
        .prepare_formula_scope(content())
        .expect("first actual scope")
        .normalize_sources(syntax())
        .expect("source syntax");
    let sources = source_owner
        .check_sources(kinds())
        .expect("first actual source proof");
    let recipe_owner = second
        .prepare_formula_scope(content())
        .expect("second actual scope")
        .normalize_recipe()
        .expect("recipe syntax");
    let recipe = recipe_owner
        .check_kinds()
        .expect("second actual recipe proof");
    let error = sources
        .plan_inputs(&recipe, source_limits(), recipe_limits())
        .expect_err("equal content is not the same root");
    assert!(std::ptr::eq(error.sources(), &sources));
    assert!(std::ptr::eq(error.recipe(), &recipe));
    assert!(std::ptr::eq(error.sources().scope().root(), &first));
    assert!(std::ptr::eq(error.recipe().scope().root(), &second));
}
#[test]
fn another_source_proof_over_the_same_actual_root_and_arenas_refuses_before_every_quota() {
    let root = root(RECIPE);
    let scope = root.prepare_formula_scope(content()).expect("scope");
    let normalized = scope.normalize_sources(syntax()).expect("syntax");
    let sources = normalized
        .check_sources(kinds())
        .expect("selected source proof");
    let other = normalized
        .check_sources(kinds())
        .expect("another proof over exact same normalized owner");
    assert!(std::ptr::eq(sources.normalized(), other.normalized()));
    let recipe = scope.normalize_recipe().expect("syntax");
    let recipe = recipe.check_kinds().expect("proof");
    let source_limits = FormulaSourceGraphExecutionLimits::new(0, 0, 0, 0, 0);
    let recipe_limits = FormulaRecipeExecutionLimits::new(0, 0, 0, 0);
    let inputs = sources
        .plan_inputs(&recipe, source_limits, recipe_limits)
        .expect("same actual containing root");
    let deps = other
        .proof()
        .prepare_dependencies(edges())
        .expect("other exact source proof relation");
    let graph = deps.admit_acyclic(cycles()).expect("other acyclic graph");
    let error = inputs
        .prepare_scalar(&graph)
        .expect_err("exact selected proof provenance required before quotas");
    assert!(std::ptr::eq(error.inputs(), &inputs));
    assert_eq!(error.inputs().source_limits(), source_limits);
    assert_eq!(error.inputs().recipe_limits(), recipe_limits);
    assert!(
        matches!(error.refusal(),Refusal::ForeignGraph{graph:actual} if std::ptr::eq(*actual,&graph))
    );
}
#[test]
fn recipe_output_admission_precedes_source_preparation_and_source_workspace_failure_stays_original()
{
    let root = root(RECIPE);
    let scope = root.prepare_formula_scope(content()).expect("scope");
    let normalized = scope.normalize_sources(syntax()).expect("syntax");
    let sources = normalized.check_sources(kinds()).expect("proof");
    let recipe = scope.normalize_recipe().expect("syntax");
    let recipe = recipe.check_kinds().expect("proof");
    let deps = sources
        .proof()
        .prepare_dependencies(edges())
        .expect("actual relation");
    let graph = deps.admit_acyclic(cycles()).expect("acyclic");
    let zero_sources = FormulaSourceGraphExecutionLimits::new(0, 0, 0, 0, 0);
    let short_recipe = FormulaRecipeExecutionLimits::new(2, 4, 11, 1);
    let inputs = sources
        .plan_inputs(&recipe, zero_sources, short_recipe)
        .expect("root join");
    let error = inputs
        .prepare_scalar(&graph)
        .expect_err("whole recipe output before source preparation");
    assert!(matches!(error.refusal(),Refusal::RecipeAdmission(cause)
        if cause.refusal()==FormulaRecipeExecutionAdmissionRefusal::OutputRecords{allowed:1,required:2}
        && cause.limits()==short_recipe && std::ptr::eq(cause.proof(),recipe.proof())));
    let short_source = FormulaSourceGraphExecutionLimits::new(1, 3, 0, 7, 1);
    let inputs = sources
        .plan_inputs(&recipe, short_source, recipe_limits())
        .expect("same actual proof join");
    let error = inputs
        .prepare_scalar(&graph)
        .expect_err("complete source workspace");
    assert!(matches!(error.refusal(),Refusal::Sources(cause)
        if matches!(cause.refusal(),FormulaScalarSourceGraphPlanRefusal::WorkspaceRecords{allowed:7,required:8})
        && cause.limits()==short_source && std::ptr::eq(cause.graph(),&graph)));
    assert_eq!(error.inputs().recipe_limits(), recipe_limits());
}
#[test]
fn unused_source_and_actual_recipe_unavailable_nodes_keep_original_factory_causes_without_partial_plan(
) {
    let mut supplied = definition();
    supplied.recipe = FormulaRecipeDocument::new(RECIPE.into()).expect("syntax");
    supplied.formula_sources = registry(&[(9003, "1 mm/3"), (9004, "tan(1 deg)")]);
    let actual = DesignInputs::new(supplied).expect("actual unused source owner");
    let scope = actual.prepare_formula_scope(content()).expect("scope");
    let normalized = scope.normalize_sources(syntax()).expect("syntax");
    let sources = normalized
        .check_sources(kinds())
        .expect("all actual kinds valid");
    let recipe = scope.normalize_recipe().expect("syntax");
    let recipe = recipe.check_kinds().expect("proof");
    let source_policy = FormulaSourceGraphExecutionLimits::new(2, 5, 0, 14, 2);
    let inputs = sources
        .plan_inputs(&recipe, source_policy, recipe_limits())
        .expect("actual root");
    let deps = sources
        .proof()
        .prepare_dependencies(edges())
        .expect("relation");
    let graph = deps.admit_acyclic(cycles()).expect("acyclic");
    let error = inputs
        .prepare_scalar(&graph)
        .expect_err("unused tangent cannot prepare scalar source plan");
    assert!(matches!(error.refusal(),Refusal::Sources(cause)
        if matches!(cause.refusal(),FormulaScalarSourceGraphPlanRefusal::MissingTangentPolicy{source_index:1,node}
            if node.span().start()==0 && node.span().end()==10)));
    let actual = root("let authored:ratio=tan(1 deg)");
    let scope = actual.prepare_formula_scope(content()).expect("scope");
    let normalized = scope.normalize_sources(syntax()).expect("syntax");
    let sources = normalized.check_sources(kinds()).expect("source proof");
    let recipe = scope.normalize_recipe().expect("syntax");
    let recipe = recipe.check_kinds().expect("recipe kind proof");
    let recipe_policy = FormulaRecipeExecutionLimits::new(1, 2, 4, 1);
    let inputs = sources
        .plan_inputs(&recipe, source_limits(), recipe_policy)
        .expect("actual root");
    let deps = sources
        .proof()
        .prepare_dependencies(edges())
        .expect("relation");
    let graph = deps.admit_acyclic(cycles()).expect("acyclic");
    let error = inputs
        .prepare_scalar(&graph)
        .expect_err("actual recipe tangent cannot prepare");
    assert!(matches!(error.refusal(),Refusal::Recipe(cause)
        if matches!(cause.refusal(),FormulaScalarRecipePlanRefusal::UnavailableBuiltin(FormulaBuiltin::Tan))
        && std::ptr::eq(cause.proof(),recipe.proof()) && std::ptr::eq(cause.inputs(),actual.inputs())));
    assert_eq!(error.inputs().recipe_limits(), recipe_policy);
}
#[test]
fn empty_actual_recipe_fits_zero_native_populations_while_required_source_remains_complete() {
    let root = root("");
    let scope = root.prepare_formula_scope(content()).expect("scope");
    let normalized = scope.normalize_sources(syntax()).expect("syntax");
    let sources = normalized.check_sources(kinds()).expect("source proof");
    let recipe = scope.normalize_recipe().expect("syntax");
    let recipe = recipe.check_kinds().expect("empty whole proof");
    let inputs = sources
        .plan_inputs(
            &recipe,
            source_limits(),
            FormulaRecipeExecutionLimits::new(0, 0, 0, 0),
        )
        .expect("actual root");
    let deps = sources
        .proof()
        .prepare_dependencies(edges())
        .expect("relation");
    let graph = deps.admit_acyclic(cycles()).expect("acyclic");
    let plan = inputs
        .prepare_scalar(&graph)
        .expect("complete real source plus actual empty recipe");
    assert_eq!(plan.sources().sources().len(), 1);
    assert!(plan.recipe().sources().is_empty());
    assert_eq!(plan.recipe().admission().statement_count(), 0);
    assert!(std::ptr::eq(plan.scope().root(), &root));
}
