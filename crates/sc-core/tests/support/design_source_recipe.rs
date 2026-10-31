//! Public native paired API over actual Design-owned canonical/source/recipe factories.
use super::{
    formula_check::{content, registry},
    id,
    input_composition::{definition, Markers},
};
use sc_core::{
    design::{
        DesignInputs, DesignParameterDefinition, DesignParameterTarget, DesignParameters,
        DesignStructure,
    },
    name::MachineToken,
    recipe::*,
    value::*,
};
pub(super) fn root(recipe: &str, body: &str) -> DesignInputs<Markers> {
    let mut supplied = definition();
    let mut structure = supplied.structure.definition().clone();
    let mut scalars = structure.inputs.scalars().to_vec();
    let actual = scalars
        .iter_mut()
        .find(|s| s.id() == id(50))
        .expect("actual general Length target");
    let mut declaration = actual.definition().clone();
    declaration.state = ScalarInputState::Derived { formula: id(9004) };
    *actual = ScalarInputDeclaration::new(declaration).expect("explicit actual general Derived");
    structure.inputs = CanonicalInputs::new(structure.inputs.lengths().to_vec(), scalars)
        .expect("single actual inventory");
    supplied.structure =
        DesignStructure::new(structure).expect("actual unchanged historical structure");
    let mut parameters: Vec<_> = supplied
        .parameters
        .parameters()
        .iter()
        .map(|p| p.definition().clone())
        .collect();
    parameters.extend([
        DesignParameterDefinition {
            id: id(8004),
            token: MachineToken::new("tripled_source").expect("authored name"),
            target: DesignParameterTarget::Scalar {
                declaration: id(50),
                kind: ScalarInputKind::Length,
            },
        },
        DesignParameterDefinition {
            id: id(8005),
            token: MachineToken::new("flag").expect("actual unknown name"),
            target: DesignParameterTarget::Scalar {
                declaration: id(8000),
                kind: ScalarInputKind::Boolean,
            },
        },
    ]);
    supplied.parameters = DesignParameters::new(parameters, supplied.structure.inputs())
        .expect("actual parameter metadata");
    supplied.formula_sources = registry(&[(9003, body), (9004, "declared_width*3")]);
    supplied.recipe =
        FormulaRecipeDocument::new(recipe.into()).expect("actual owned ordered recipe");
    DesignInputs::new(supplied).expect("actual complete Design source references")
}
fn with_plans(
    recipe_root: &DesignInputs<Markers>,
    source_root: &DesignInputs<Markers>,
    inspect: impl for<'a> FnOnce(FormulaScalarRecipePlan<'a>, FormulaScalarSourceGraphPlan<'a>),
) {
    with_policies(recipe_root, source_root, None, None, inspect);
}
fn with_policies(
    recipe_root: &DesignInputs<Markers>,
    source_root: &DesignInputs<Markers>,
    recipe_policy: Option<FormulaTangentLimits>,
    source_policy: Option<FormulaTangentLimits>,
    inspect: impl for<'a> FnOnce(FormulaScalarRecipePlan<'a>, FormulaScalarSourceGraphPlan<'a>),
) {
    let scope = source_root
        .prepare_formula_scope(content())
        .expect("actual source input admission");
    let s = source_root.formula_sources().sources().len();
    let n = source_root
        .formula_sources()
        .sources()
        .iter()
        .map(|s| s.expression().node_count())
        .sum();
    let normalized = scope
        .normalize_sources(FormulaSourceNormalizationLimits::new(s, n, s + 2 * n))
        .expect("actual source owners");
    let checked = normalized
        .check_sources(FormulaSourceCheckLimits::new(s, n, 2, s + 2 * n))
        .expect("actual Design namespace proofs");
    let d = checked
        .proof()
        .sources()
        .iter()
        .map(|s| s.proof().dependencies().len())
        .sum();
    let deps = checked
        .proof()
        .prepare_dependencies(FormulaSourceDependencyLimits::new(s, d, s + d + 1))
        .expect("actual complete dependencies");
    let graph = deps
        .admit_acyclic(FormulaSourceCycleLimits::new(s, 2 * s))
        .expect("actual whole acyclic graph");
    let limits = FormulaSourceGraphExecutionLimits::new(s, n, d, 2 * s + 2 * n + d, s);
    let sources = match source_policy {
        Some(policy) => graph.prepare_scalar_sources_with_tangent_limits(limits, policy),
        None => graph.prepare_scalar_sources(limits),
    }
    .expect("complete source plan");
    let normalized = recipe_root
        .prepare_formula_scope(content())
        .expect("actual recipe input admission")
        .normalize_recipe()
        .expect("actual recipe owner");
    let checked = normalized
        .check_kinds()
        .expect("actual Design namespace/ordered proof");
    let s = checked.proof().statements().len();
    let d = checked.proof().dependencies().count();
    let n = checked
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
    let admitted = checked
        .proof()
        .admit_execution(FormulaRecipeExecutionLimits::new(s, n, 2 * n + d, s))
        .expect("complete whole recipe admission");
    let recipe = match recipe_policy {
        Some(policy) => admitted.prepare_scalar_with_tangent_limits(recipe_root.inputs(), policy),
        None => admitted.prepare_scalar(recipe_root.inputs()),
    }
    .expect("actual recipe canonical/prior-let sources");
    inspect(recipe, sources);
}
fn context() -> FormulaScalarExecutionContext<'static> {
    FormulaScalarExecutionContext::new(
        FormulaToleranceContext::empty(),
        FormulaSizeContext::empty(),
    )
}
fn stored_values(result: &FormulaExecutedScalarSourceRecipe<'_>) -> Vec<i64> {
    result
        .statements()
        .iter()
        .filter_map(|statement| match statement.outcome() {
            FormulaScalarRecipeOutcome::Binding { binding, .. } => {
                Some(binding.value().numeric_integer().expect("numeric fixture"))
            }
            FormulaScalarRecipeOutcome::Assertion(_) => None,
        })
        .collect()
}
fn source_ordinals(
    result: &FormulaExecutedScalarSourceRecipe<'_>,
) -> Vec<(sc_core::ontology::EntityId, usize)> {
    result
        .sources()
        .map(|source| {
            (
                source.source().normalized().source().id(),
                source.completion_ordinal(),
            )
        })
        .collect()
}
#[test]
fn fresh_paired_replay_recomputes_unbound_sources_and_once_stored_bindings() {
    let root = root(
        "let a:length=declared_width*3\nlet b:length=tripled_source\nassert equal:eps_num=a==b",
        "1 mm/3",
    );
    let before = root.clone();
    with_plans(&root, &root, |recipe, sources| {
        let first = recipe
            .replay_with_sources(&sources, context())
            .expect("fresh paired run");
        let second = first
            .recipe_plan()
            .replay_with_sources(first.source_plan(), context())
            .expect("replay accepted pair");
        let fresh = recipe
            .evaluate_with_sources(sources, context())
            .expect("ordinary fresh pair");
        for result in [&first, &second, &fresh] {
            assert_eq!(stored_values(result), [1000, 1000]);
            assert_eq!(source_ordinals(result), [(id(9003), 1), (id(9004), 2)]);
            assert_eq!(
                result
                    .source(id(9003))
                    .expect("unbound original source")
                    .value()
                    .numeric()
                    .expect("Length")
                    .rational(),
                FormulaRational::new(1000, 3, false).expect("exact root")
            );
            assert_eq!(
                result
                    .reads()
                    .map(|read| read.dependency().declaration().name())
                    .collect::<Vec<_>>(),
                ["declared_width", "tripled_source", "a", "b", "eps_num"]
            );
            assert!(std::ptr::eq(result.recipe_plan().inputs(), root.inputs()));
            assert!(std::ptr::eq(
                result.source_plan().graph(),
                first.source_plan().graph()
            ));
            assert_eq!(
                result.recipe_plan().admission().limits(),
                first.recipe_plan().admission().limits()
            );
            assert_eq!(result.source_plan().limits(), first.source_plan().limits());
        }
    });
    assert_eq!(root, before);
}
#[test]
fn changed_context_replay_starts_empty_and_late_failure_preserves_accepted_pair() {
    use sc_units::{Length, Tolerance, ToleranceClass};
    let root = root("let safe:length=1 um/3\nlet a:length=if(eps_phys<5 um,tripled_source,9 um)\nassert accepted:eps_phys=a==1000 um", "1 mm/3");
    let physical = |value| {
        Tolerance::new(
            ToleranceClass::PhysicalAcceptance,
            Length::from_micrometres(value).expect("positive fixture"),
            "paired replay fixture",
        )
        .expect("quantitative provider")
    };
    let small = physical(1);
    let large = physical(1000);
    let refuses = physical(6);
    let supplied = |physical| {
        FormulaScalarExecutionContext::new(
            FormulaToleranceContext::new(FormulaToleranceContextDefinition {
                label: Some("paired_replay_fixture"),
                format: None,
                importer: None,
                physical: Some(FormulaToleranceProvider {
                    source: id(90),
                    tolerance: physical,
                }),
            })
            .expect("checked physical class"),
            FormulaSizeContext::empty(),
        )
    };
    with_plans(&root, &root, |recipe, sources| {
        let first = recipe
            .replay_with_sources(&sources, supplied(&small))
            .expect("actual source demands");
        assert_eq!(stored_values(&first), [0, 1000]);
        assert_eq!(source_ordinals(&first), [(id(9003), 1), (id(9004), 2)]);
        let changed = first
            .recipe_plan()
            .replay_with_sources(first.source_plan(), supplied(&large))
            .expect("new context selects no source");
        assert_eq!(stored_values(&changed), [0, 9]);
        assert_eq!(changed.sources().count(), 0);
        assert!(changed.source(id(9003)).is_none());
        assert!(changed.source(id(9004)).is_none());
        assert_eq!(
            changed
                .reads()
                .map(|r| r.dependency().declaration().name())
                .collect::<Vec<_>>(),
            ["eps_phys", "a", "eps_phys"]
        );
        let FormulaScalarSourceRecipeReplayError::Execution(error) = first
            .recipe_plan()
            .replay_with_sources(first.source_plan(), supplied(&refuses))
            .expect_err("actual late assertion rejects the whole attempt")
        else {
            unreachable!("original paired runtime cause");
        };
        assert!(
            matches!(error.cause(), FormulaScalarSourceRecipeErrorCause::Recipe(error)
            if error.statement().expect("actual assertion").statement_index()==3
            && matches!(error.cause(), FormulaScalarRecipeErrorCause::Evaluation(failure)
            if matches!(failure.as_ref(), FormulaScalarRecipeFailure::Assertion(_))))
        );
        assert_eq!(stored_values(&first), [0, 1000]);
        assert_eq!(source_ordinals(&first), [(id(9003), 1), (id(9004), 2)]);
        let again = changed
            .recipe_plan()
            .replay_with_sources(changed.source_plan(), supplied(&small))
            .expect("fresh demand after empty attempt");
        assert_eq!(stored_values(&again), [0, 1000]);
        assert_eq!(source_ordinals(&again), [(id(9003), 1), (id(9004), 2)]);
    });
}
#[test]
fn replay_uses_independently_selected_recipe_and_supplier_tangent_policies() {
    let root = root(
        "let t:ratio=tan(30 deg)\nlet a:length=declared_width",
        "tan(30 deg)*1 mm",
    );
    let recipe_good = FormulaTangentLimits::new(2073, 512);
    let source_good = FormulaTangentLimits::new(2073, 513);
    let zero = FormulaTangentLimits::new(0, 0);
    for (recipe_policy, source_policy) in [
        (recipe_good, source_good),
        (zero, source_good),
        (recipe_good, zero),
    ] {
        with_policies(
            &root,
            &root,
            Some(recipe_policy),
            Some(source_policy),
            |recipe, sources| {
                let first = recipe.replay_with_sources(&sources, context());
                if recipe_policy == zero || source_policy == zero {
                    let FormulaScalarSourceRecipeReplayError::Execution(error) =
                        first.expect_err("executed selected zero policy refuses")
                    else {
                        unreachable!("actual runtime cause");
                    };
                    assert_eq!(error.recipe_tangent_limits(), Some(recipe_policy));
                    assert_eq!(error.source_tangent_limits(), Some(source_policy));
                    if recipe_policy == zero {
                        assert!(
                            matches!(error.cause(), FormulaScalarSourceRecipeErrorCause::Recipe(error)
                        if error.statement().expect("first Tan").statement_index()==1)
                        );
                    } else {
                        assert!(
                            matches!(error.cause(), FormulaScalarSourceRecipeErrorCause::RequestedSource(error)
                        if error.statement().statement_index()==2)
                        );
                    }
                } else {
                    let first = first.expect("both actual policies succeed");
                    let second = first
                        .recipe_plan()
                        .replay_with_sources(first.source_plan(), context())
                        .expect("fresh configured replay");
                    for result in [&first, &second] {
                        assert_eq!(stored_values(result), [577350, 577]);
                        assert_eq!(result.recipe_plan().tangent_limits(), Some(recipe_policy));
                        assert_eq!(result.source_plan().tangent_limits(), Some(source_policy));
                    }
                }
            },
        );
    }
}
#[test]
fn complete_public_result_keeps_actual_both_family_sources_real_bindings_ordinals_and_read_witnesses(
) {
    let root = root(
        "let a:length=declared_width*3\nlet b:length=tripled_source\nassert equal:eps_num=a==b",
        "1 mm/3",
    );
    with_plans(&root, &root, |recipe, sources| {
        let recipe_limits = recipe.admission().limits();
        let source_limits = sources.limits();
        let proof = recipe.admission().proof();
        let graph = sources.graph();
        let result = recipe
            .evaluate_with_sources(sources, context())
            .expect("actual complete paired execution");
        assert!(std::ptr::eq(
            result.recipe_plan().admission().proof(),
            proof
        ));
        assert!(std::ptr::eq(result.source_plan().graph(), graph));
        assert!(std::ptr::eq(result.recipe_plan().inputs(), root.inputs()));
        assert_eq!(result.recipe_plan().admission().limits(), recipe_limits);
        assert_eq!(result.source_plan().limits(), source_limits);
        assert_eq!(result.statements().len(), 3);
        assert_eq!(result.sources().count(), 2);
        for statement in result.statements().iter().take(2) {
            assert!(
                matches!(statement.outcome(),FormulaScalarRecipeOutcome::Binding{binding,..}
                if binding.read().expect("actual stored binding").numeric().expect("Length").rational()==FormulaRational::new(1000,1,false).expect("earned value"))
            );
        }
        let width = result
            .source(id(9003))
            .expect("actual completed width source");
        assert_eq!(
            width.value().numeric().expect("Length").rational(),
            FormulaRational::new(1000, 3, false).expect("bare unbound source root")
        );
        assert_eq!(width.completion_ordinal(), 1);
        let triple = result
            .source(id(9004))
            .expect("actual completed general Length source");
        assert_eq!(triple.completion_ordinal(), 2);
        assert!(std::ptr::eq(
            triple.source().normalized().source(),
            root.formula_sources()
                .source(id(9004))
                .expect("actual original owner")
        ));
        let derived: Vec<_> = result
            .reads()
            .filter_map(|r| match r.source() {
                FormulaScalarSourceRecipeReadKind::Derived(source) => Some(source),
                _ => None,
            })
            .collect();
        assert_eq!(derived.len(), 2);
        assert!(derived
            .iter()
            .all(|r| std::ptr::eq(r.input().inputs(), root.inputs())));
        assert!(matches!(
            derived
                .first()
                .expect("actual dedicated read")
                .input()
                .adapter(),
            FormulaCanonicalInputAdapter::Length(_)
        ));
        assert!(matches!(
            derived
                .get(1)
                .expect("actual general read")
                .input()
                .adapter(),
            FormulaCanonicalInputAdapter::Scalar(_)
        ));
        let headers: Vec<_> = result
            .reads()
            .filter(|r| r.dependency().role() == FormulaRecipeDependencyRole::AssertionTolerance)
            .collect();
        assert_eq!(headers.len(), 1);
        assert!(headers.first().expect("real header").node().is_none());
        assert!(result.source(id(999999)).is_none());
        assert_eq!(
            root.inputs()
                .length(id(40))
                .expect("unchanged actual state")
                .authored_value(),
            Err(LengthValueError::RequiresEvaluation {
                declaration: id(40),
                formula: id(9003)
            })
        );
    });
}
#[test]
fn equal_complete_models_with_foreign_canonical_owners_refuse_with_both_original_policies() {
    let root = root("let a:length=declared_width", "1 mm/3");
    let other = root.clone();
    assert_eq!(root, other);
    with_plans(&root, &other, |recipe, sources| {
        let recipe_limits = recipe.admission().limits();
        let source_limits = sources.limits();
        let proof = recipe.admission().proof();
        let graph = sources.graph();
        let FormulaScalarSourceRecipeReplayError::Execution(replay_error) = recipe
            .replay_with_sources(&sources, context())
            .expect_err("fresh replay refuses foreign owner before preparation")
        else {
            unreachable!("actual ownership refusal");
        };
        assert!(std::ptr::eq(replay_error.inputs(), root.inputs()));
        assert!(std::ptr::eq(replay_error.recipe(), proof));
        assert!(std::ptr::eq(replay_error.graph(), graph));
        assert_eq!(replay_error.recipe_limits(), recipe_limits);
        assert_eq!(replay_error.source_limits(), source_limits);
        assert!(
            matches!(replay_error.cause(), FormulaScalarSourceRecipeErrorCause::ForeignInputs { sources }
            if std::ptr::eq(*sources, other.inputs()))
        );
        let error = recipe
            .evaluate_with_sources(sources, context())
            .expect_err("canonical identity before runtime preparation");
        assert!(std::ptr::eq(error.inputs(), root.inputs()));
        assert!(std::ptr::eq(error.recipe(), proof));
        assert!(std::ptr::eq(error.graph(), graph));
        assert_eq!(error.recipe_limits(), recipe_limits);
        assert_eq!(error.source_limits(), source_limits);
        assert!(
            matches!(error.cause(),FormulaScalarSourceRecipeErrorCause::ForeignInputs{sources} if std::ptr::eq(*sources,other.inputs()))
        );
    });
}
#[test]
fn nested_supplier_failure_keeps_original_requester_actual_join_and_supplier_without_result() {
    let text = "let safe:length=1 mm\nlet blocked:length=tripled_source";
    let root = root(text, "if(flag,1 mm,2 mm)");
    with_plans(&root, &root, |recipe, sources| {
        let error = recipe
            .evaluate_with_sources(sources, context())
            .expect_err("actual unknown supplier requirement after prior success");
        let request = match error.cause() {
            FormulaScalarSourceRecipeErrorCause::RequestedSource(error) => Some(error),
            _ => None,
        }
        .expect("actual requesting source failure");
        assert_eq!(request.statement().statement_index(), 2);
        assert_eq!(request.part(), FormulaStatementExpression::Binding);
        assert_eq!(
            text.get(request.node().span().start()..request.node().span().end()),
            Some("tripled_source")
        );
        assert!(std::ptr::eq(request.input().inputs(), root.inputs()));
        assert!(matches!(
            request.cause(),
            FormulaScalarSourceGraphExecutionCause::Evaluation {
                source_index: 0,
                ..
            }
        ));
        assert!(std::ptr::eq(
            error
                .graph()
                .dependencies()
                .checked()
                .normalized()
                .inputs()
                .sources(),
            root.formula_sources()
        ));
    });
}
#[test]
fn original_recipe_storage_failure_stays_typed_in_the_complete_paired_scope() {
    let root = root("let n:count=9223372036854775807+1", "1 mm/3");
    with_plans(&root, &root, |recipe, sources| {
        let error = recipe
            .evaluate_with_sources(sources, context())
            .expect_err("actual once-only storage overflow");
        assert!(
            matches!(error.cause(),FormulaScalarSourceRecipeErrorCause::Recipe(error)
            if error.statement().expect("actual let").statement_index()==1
            && matches!(error.cause(),FormulaScalarRecipeErrorCause::Evaluation(failure) if matches!(failure.as_ref(),FormulaScalarRecipeFailure::Binding{..})))
        );
        assert!(std::ptr::eq(error.inputs(), root.inputs()));
    });
}
#[test]
fn empty_actual_recipe_returns_no_fabricated_entry_root_or_read_prefix() {
    let root = root("", "1 mm/3");
    with_plans(&root, &root, |recipe, sources| {
        let result = recipe
            .evaluate_with_sources(sources, context())
            .expect("actual empty whole recipe");
        assert!(result.statements().is_empty());
        assert_eq!(result.sources().count(), 0);
        assert_eq!(result.reads().count(), 0);
        assert!(result.source(id(9003)).is_none());
        assert_eq!(result.source_plan().sources().len(), 2);
        assert_eq!(result.recipe_plan().admission().output_records(), 0);
    });
}
#[test]
fn untaken_actual_source_with_unknown_input_remains_unexecuted_in_public_paired_mode() {
    let root = root(
        "let selected:length=if(1==2,declared_width,1 mm)\nassert check:eps_num=selected==1 mm",
        "if(flag,1 mm,2 mm)",
    );
    with_plans(&root, &root, |recipe, sources| {
        let result = recipe
            .evaluate_with_sources(sources, context())
            .expect("only the actual selected branch reads values");
        assert_eq!(result.statements().len(), 2);
        assert_eq!(result.sources().count(), 0);
        assert!(result
            .reads()
            .all(|r| !matches!(r.source(), FormulaScalarSourceRecipeReadKind::Derived(_))));
        assert!(std::ptr::eq(
            result
                .source_plan()
                .graph()
                .dependencies()
                .checked()
                .normalized()
                .inputs()
                .inputs(),
            root.inputs()
        ));
    });
}
