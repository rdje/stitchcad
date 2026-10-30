//! Actual Design namespace factories,complete static witnesses and ordered visibility.
use super::{
    formula_scope::{observed, ObservedHost},
    id,
    input_composition::{definition, Markers, MissingFixtureRecord},
};
use core::num::{NonZeroU64, NonZeroUsize};
use sc_core::{
    design::{
        DesignFormulaError, DesignFormulaHost, DesignFormulaRecipeCheckError,
        DesignFormulaSourceCheckError, DesignInputs, DesignInputsError, DesignMetadataError,
        DesignResourceLimits, DesignSourceReferences,
    },
    recipe::{
        FormulaDeclaration, FormulaDeclarationSource, FormulaExpressionDocument,
        FormulaInputOrigin, FormulaKind, FormulaRecipeDocument, FormulaSource,
        FormulaSourceCheckLimits, FormulaSourceCheckRefusal, FormulaSourceLimits,
        FormulaSourceNormalizationLimits, FormulaSources,
    },
    value::{CanonicalInputs, LengthValueError},
};
impl DesignFormulaHost for Markers {
    fn formula_declarations<'a>(
        &'a self,
        _: DesignSourceReferences,
        _: &'a CanonicalInputs,
    ) -> Result<Vec<FormulaDeclaration<'a>>, Self::Error> {
        Ok(vec![])
    }
}
impl DesignFormulaHost for ObservedHost {
    fn formula_declarations<'a>(
        &'a self,
        _: DesignSourceReferences,
        _: &'a CanonicalInputs,
    ) -> Result<Vec<FormulaDeclaration<'a>>, Self::Error> {
        Ok(vec![])
    }
}
fn content() -> DesignResourceLimits {
    DesignResourceLimits::new(
        NonZeroUsize::new(10000).expect("entries"),
        NonZeroUsize::new(10000).expect("text"),
        NonZeroU64::new(10000).expect("bits"),
    )
}
fn registry(rows: &[(u128, &str)]) -> FormulaSources {
    let sources: Vec<_> = rows
        .iter()
        .map(|(identity, body)| {
            FormulaSource::new(
                id(*identity),
                FormulaExpressionDocument::new((*body).into()).expect("actual expression syntax"),
            )
        })
        .collect();
    let limits = FormulaSourceLimits::new(
        sources.len(),
        sources.iter().map(|s| s.expression().source().len()).sum(),
        sources
            .iter()
            .map(|s| s.expression().canonical().as_str().len())
            .sum(),
    );
    FormulaSources::new(sources, limits).expect("actual distinct owned sources")
}
#[test]
fn actual_root_metadata_and_real_prior_lets_survive_complete_static_proofs_without_values() {
    let mut supplied = definition();
    supplied.recipe = FormulaRecipeDocument::new(
        "let authored:length=declared_width\nlet twice:length=authored+authored".into(),
    )
    .expect("actual recipe");
    let root = DesignInputs::new(supplied).expect("actual root");
    let scope = root.prepare_formula_scope(content()).expect("actual scope");
    let sources = scope
        .normalize_sources(FormulaSourceNormalizationLimits::new(1, 3, 7))
        .expect("actual sources");
    let checked = sources
        .check_sources(FormulaSourceCheckLimits::new(1, 3, 1, 7))
        .expect("actual complete proof");
    assert!(std::ptr::eq(checked.normalized(), &sources));
    assert!(std::ptr::eq(checked.scope().root(), &root));
    assert!(std::ptr::eq(
        checked.proof().normalized(),
        sources.normalized()
    ));
    assert_eq!(checked.proof().target_count(), 1);
    let source = checked
        .proof()
        .sources()
        .first()
        .expect("actual original source");
    assert!(std::ptr::eq(
        source.normalized().source(),
        root.formula_sources()
            .source(id(9003))
            .expect("actual owner")
    ));
    assert_eq!(source.proof().kind(), FormulaKind::Length);
    let recipe = scope.normalize_recipe().expect("actual ordered recipe");
    let checked = recipe.check_kinds().expect("actual names/kinds");
    assert!(std::ptr::eq(checked.normalized(), &recipe));
    assert!(std::ptr::eq(checked.scope().root(), &root));
    assert!(std::ptr::eq(checked.proof().recipe(), recipe.normalized()));
    let mut dependencies = checked.proof().dependencies();
    let first = dependencies.next().expect("actual parameter");
    assert_eq!(first.statement_index(), 1);
    assert!(std::ptr::eq(
        first.declaration().name(),
        root.parameters()
            .parameters()
            .first()
            .expect("actual parameter")
            .definition()
            .token
            .as_str()
    ));
    assert!(
        matches!(first.declaration().source(),FormulaDeclarationSource::LengthInput{
        origin:FormulaInputOrigin::Parameter,input,declaration} if input==id(8003)
        && std::ptr::eq(declaration,root.inputs().length(id(40)).expect("actual canonical")))
    );
    let authored = recipe
        .normalized()
        .statements()
        .first()
        .expect("actual first let");
    for dependency in dependencies {
        assert_eq!(dependency.statement_index(), 2);
        assert!(std::ptr::eq(
            dependency.declaration().name(),
            authored.name()
        ));
        assert!(
            matches!(dependency.declaration().source(),FormulaDeclarationSource::Recipe{
            statement_index:1,span,name_span,..} if span==authored.span() && name_span==authored.name_span())
        );
    }
    assert_eq!(checked.proof().dependencies().count(), 3);
    assert_eq!(
        root.inputs()
            .length(id(40))
            .expect("actual Derived")
            .authored_value(),
        Err(LengthValueError::RequiresEvaluation {
            declaration: id(40),
            formula: id(9003)
        })
    );
}
#[test]
fn source_name_uses_actual_parameter_metadata_and_does_not_claim_cycle_or_numerical_acceptance() {
    let mut supplied = definition();
    supplied.formula_sources = registry(&[(9003, "declared_width/3")]);
    let root = DesignInputs::new(supplied).expect("actual structural root");
    let normalized = root
        .prepare_formula_scope(content())
        .expect("scope")
        .normalize_sources(FormulaSourceNormalizationLimits::new(1, 3, 7))
        .expect("actual syntax");
    let checked = normalized
        .check_sources(FormulaSourceCheckLimits::new(1, 3, 1, 7))
        .expect("kind check is not cycle admission");
    let dependency = checked
        .proof()
        .sources()
        .first()
        .expect("actual source")
        .proof()
        .dependencies()
        .first()
        .expect("actual name");
    assert!(
        matches!(dependency.declaration().source(),FormulaDeclarationSource::LengthInput{
        input,declaration,..} if input==id(8003) && std::ptr::eq(declaration,root.inputs().length(id(40)).expect("actual canonical")))
    );
    assert_eq!(dependency.declaration().name(), "declared_width");
    assert!(std::ptr::eq(checked.scope().root(), &root));
}
#[test]
fn late_unused_source_error_and_consumer_kind_refusal_keep_original_source_and_canonical_witnesses()
{
    let mut supplied = definition();
    supplied.formula_sources = registry(&[(9003, "1 mm/3"), (9004, "missing")]);
    let root = DesignInputs::new(supplied).expect("unused source owned");
    let normalized = root
        .prepare_formula_scope(content())
        .expect("scope")
        .normalize_sources(FormulaSourceNormalizationLimits::new(2, 4, 10))
        .expect("actual syntax");
    let limits = FormulaSourceCheckLimits::new(2, 4, 1, 10);
    let error = normalized
        .check_sources(limits)
        .expect_err("late complete-source failure");
    assert!(std::ptr::eq(error.normalized(), &normalized));
    assert_eq!(error.limits(), limits);
    assert!(
        matches!(&error,DesignFormulaSourceCheckError::Sources{error,..}
        if matches!(error.refusal(),FormulaSourceCheckRefusal::Expression{source_index:1,error:cause}
            if cause.token()=="formula_unbound_name" && std::ptr::eq(cause.expression(),normalized.normalized().sources().get(1).expect("actual failing source").expression())))
    );
    let mut supplied = definition();
    supplied.formula_sources = registry(&[(9003, "1==1")]);
    let root = DesignInputs::new(supplied).expect("source link alone is not kind checking");
    let normalized = root
        .prepare_formula_scope(content())
        .expect("scope")
        .normalize_sources(FormulaSourceNormalizationLimits::new(1, 3, 7))
        .expect("actual syntax");
    let error = normalized
        .check_sources(FormulaSourceCheckLimits::new(1, 3, 1, 7))
        .expect_err("actual consumer expects Length");
    assert!(
        matches!(&error,DesignFormulaSourceCheckError::Sources{error,..}
        if matches!(error.refusal(),FormulaSourceCheckRefusal::TargetKind{source_index:0,target,expected:FormulaKind::Length,actual:FormulaKind::Boolean}
            if std::ptr::eq(target.source(),root.formula_sources().source(id(9003)).expect("actual source")))),
        "{error:?}"
    );
}
#[test]
fn real_recipe_forward_name_failure_keeps_original_order_statement_span_and_owner() {
    let mut supplied = definition();
    supplied.recipe =
        FormulaRecipeDocument::new("let first:length=later\nlet later:length=1 mm".into())
            .expect("actual syntax");
    let root = DesignInputs::new(supplied).expect("actual root");
    let normalized = root
        .prepare_formula_scope(content())
        .expect("scope")
        .normalize_recipe()
        .expect("actual syntax");
    let error = normalized
        .check_kinds()
        .expect_err("later actual let remains invisible");
    assert!(std::ptr::eq(error.normalized(), &normalized));
    assert!(
        matches!(&error,DesignFormulaRecipeCheckError::Recipe{error,..}
        if error.statement_index()==1 && error.token()=="formula_unbound_name"
        && std::ptr::eq(error.recipe(),normalized.normalized())
        && std::ptr::eq(error.statement(),normalized.normalized().statements().first().expect("actual first statement"))
        && root.recipe().source().get(error.span().start()..error.span().end())==Some("later"))
    );
}
#[test]
fn actual_namespace_rechecks_current_source_and_preserves_document_owners_and_selected_limits() {
    let (root, semantic, reject) = observed();
    let scope = root
        .prepare_formula_scope(content())
        .expect("actual admitted scope");
    let sources = scope
        .normalize_sources(FormulaSourceNormalizationLimits::new(1, 3, 7))
        .expect("source syntax");
    let recipe = scope.normalize_recipe().expect("recipe syntax");
    reject.set(true);
    semantic.set(0);
    let limits = FormulaSourceCheckLimits::new(1, 3, 1, 7);
    let error = sources
        .check_sources(limits)
        .expect_err("actual current source now refuses");
    assert!(semantic.get() > 0);
    assert!(std::ptr::eq(error.normalized(), &sources));
    assert_eq!(error.limits(), limits);
    assert!(
        matches!(&error,DesignFormulaSourceCheckError::Namespace{error:DesignFormulaError::Inputs(error),..}
        if matches!(error.as_ref(),DesignInputsError::Sources(cause) if matches!(cause.as_ref(),DesignMetadataError::Host(MissingFixtureRecord))))
    );
    let error = recipe
        .check_kinds()
        .expect_err("same actual current namespace refusal before recipe checking");
    assert!(std::ptr::eq(error.normalized(), &recipe));
    assert!(
        matches!(&error,DesignFormulaRecipeCheckError::Namespace{error:DesignFormulaError::Inputs(error),..}
        if matches!(error.as_ref(),DesignInputsError::Sources(cause) if matches!(cause.as_ref(),DesignMetadataError::Host(MissingFixtureRecord))))
    );
}
#[test]
fn empty_recipe_and_complete_source_quota_keep_the_actual_root_and_original_factory_error() {
    let mut supplied = definition();
    supplied.recipe = FormulaRecipeDocument::new(String::new()).expect("empty actual recipe");
    let root = DesignInputs::new(supplied).expect("actual root");
    let scope = root.prepare_formula_scope(content()).expect("scope");
    let recipe = scope.normalize_recipe().expect("empty syntax");
    let checked = recipe.check_kinds().expect("empty whole proof");
    assert!(checked.proof().statements().is_empty());
    assert!(std::ptr::eq(checked.scope().root(), &root));
    let sources = scope
        .normalize_sources(FormulaSourceNormalizationLimits::new(1, 3, 7))
        .expect("source syntax");
    let limits = FormulaSourceCheckLimits::new(0, 3, 1, 7);
    let error = sources
        .check_sources(limits)
        .expect_err("complete source quota");
    assert_eq!(error.limits(), limits);
    assert!(std::ptr::eq(error.normalized(), &sources));
    assert!(
        matches!(&error,DesignFormulaSourceCheckError::Sources{error,..}
        if matches!(error.refusal(),FormulaSourceCheckRefusal::Sources{allowed:0,required:1})
        && std::ptr::eq(error.normalized(),sources.normalized()))
    );
}
