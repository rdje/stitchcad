use super::*;
use crate::recipe::execution::{
    machine::{Machine, Progress},
    source_read::source_index,
};
use crate::{name::MachineToken, ontology::EntityId, recipe::*, value::*};

#[test]
#[allow(clippy::expect_used)]
fn actual_recipe_pause_retains_real_prior_binding_and_earns_unbound_source_witnesses() {
    let id = EntityId::from_bits;
    let inputs = CanonicalInputs::new(
        vec![LengthDeclaration::new(LengthDeclarationDefinition {
            id: id(40),
            source: id(4000),
            state: LengthState::Derived { formula: id(9000) },
        })
        .expect("actual Derived")],
        vec![],
    )
    .expect("actual canonical owner");
    let name = MachineToken::new("declared_width").expect("actual parameter name");
    let metadata = FormulaDeclaration::length_input(
        &name,
        FormulaInputOrigin::Parameter,
        id(8000),
        inputs.length(id(40)).expect("actual original declaration"),
    );
    let initial = FormulaInitialDeclaration::try_from(metadata).expect("actual initial parameter");
    let expression = FormulaExpressionDocument::new("1 mm/3".into()).expect("actual bare source");
    let n = expression.node_count();
    let limits = FormulaSourceLimits::new(
        1,
        expression.source().len(),
        expression.canonical().as_str().len(),
    );
    let sources = FormulaSources::new(vec![FormulaSource::new(id(9000), expression)], limits)
        .expect("actual source");
    let normalized = FormulaSourceInputs::new(&inputs, &sources)
        .expect("actual complete source join")
        .normalize_sources(FormulaSourceNormalizationLimits::new(1, n, 1 + 2 * n))
        .expect("actual source arena");
    let checked = normalized
        .check_sources(
            &FormulaNamespace::new([initial]).expect("actual namespace"),
            FormulaSourceCheckLimits::new(1, n, 1, 1 + 2 * n),
        )
        .expect("actual complete source proof");
    let deps = checked
        .prepare_dependencies(FormulaSourceDependencyLimits::new(1, 0, 2))
        .expect("actual relation");
    let graph = deps
        .admit_acyclic(FormulaSourceCycleLimits::new(1, 2))
        .expect("actual acyclic sources");
    let source_plan = graph
        .prepare_scalar_sources(FormulaSourceGraphExecutionLimits::new(
            1,
            n,
            0,
            2 + 2 * n,
            1,
        ))
        .expect("actual source plan");
    let context = Context::new(
        FormulaToleranceContext::empty(),
        FormulaSizeContext::empty(),
    );
    let mut runtime =
        SourceRuntime::new(source_plan, context).expect("all source buffers before recipe values");
    let recipe = FormulaRecipe::parse(
        "let prior:length=1 mm/3\nlet total:length=prior+declared_width+declared_width",
    )
    .expect("real ordered recipe")
    .normalize_literals()
    .expect("actual recipe arenas");
    let proof = recipe
        .check_kinds(FormulaNamespace::new([initial]).expect("actual namespace"))
        .expect("actual ordered proof");
    let mut plan = proof
        .admit_execution(FormulaRecipeExecutionLimits::new(2, 8, 19, 2))
        .expect("actual complete populations")
        .prepare_scalar(&inputs)
        .expect("actual prospective canonical and prior-let suppliers");
    let mut machine = Machine::new(plan.admission().node_count())
        .expect("all recipe machine buffers before values");
    let mut completed = Vec::new();
    completed
        .try_reserve_exact(plan.admission().output_records())
        .expect("all recipe outcomes before values");
    let first = proof.statements().first().expect("real first let");
    let FormulaCheckedStatementKind::Let { expression } = first.kind() else {
        unreachable!("actual let");
    };
    let first_value = machine
        .run(
            expression.root(),
            &mut RecipeSources {
                sources: &mut [],
                completed: &[],
                derived: Some(&runtime),
                tangent_limits: None,
            },
            context,
        )
        .expect("actually execute the original first RHS");
    let binding = first_value
        .store_binding()
        .expect("store only the real first let result");
    completed.push(FormulaExecutedScalarRecipeStatement {
        statement: first,
        outcome: Outcome::Binding {
            expression: first_value,
            binding,
        },
    });
    let second = proof.statements().get(1).expect("actual second let");
    let FormulaCheckedStatementKind::Let { expression } = second.kind() else {
        unreachable!("actual let");
    };
    machine
        .start(expression.root(), context)
        .expect("actual original operand");
    let progress = machine
        .resume(&mut RecipeSources {
            sources: &mut plan.sources,
            completed: &completed,
            derived: Some(&runtime),
            tangent_limits: None,
        })
        .expect("only actual Derived name may pause");
    let Progress::AwaitDerived { node, input } = progress else {
        unreachable!("actual unresolved Derived");
    };
    assert_eq!(
        node.span().end() - node.span().start(),
        "declared_width".len()
    );
    assert!(std::ptr::eq(input.inputs(), &inputs));
    assert_eq!(
        plan.sources.iter().filter(|s| s.read.is_some()).count(),
        1,
        "only the actually completed prior-let read is recorded"
    );
    let prior = plan
        .sources
        .first()
        .and_then(|s| s.read)
        .expect("actual prior-let read before pause");
    assert!(
        matches!(prior.source,RecipeReadSource::Original(ReadKind::PriorLet{statement,..}) if std::ptr::eq(statement,first))
    );
    assert_eq!(
        prior.value.numeric().expect("Length").rational(),
        FormulaRational::new(333, 1, false).expect("actual once-stored first let")
    );
    assert!(
        runtime.read_derived(input).is_none(),
        "no seed or fabricated source root before execution"
    );
    let entry = source_index(
        &checked,
        input.derived_formula().expect("sole actual formula id"),
    )
    .expect("actual source index");
    runtime
        .demand(entry)
        .expect("execute actual bare supplier root");
    let progress = machine
        .resume(&mut RecipeSources {
            sources: &mut plan.sources,
            completed: &completed,
            derived: Some(&runtime),
            tangent_limits: None,
        })
        .expect("resume same pending work without replay");
    let Progress::Complete(value) = progress else {
        unreachable!("actual complete scalar RHS");
    };
    assert_eq!(
        value.numeric().expect("Length").rational(),
        FormulaRational::new(2999, 3, false).expect("333 plus two unbound1000/3 reads")
    );
    let reads: Vec<_> = plan.sources.iter().filter_map(|s| s.read).collect();
    assert_eq!(reads.len(), 3);
    for data in reads.iter().skip(1) {
        assert!(matches!(data.source,RecipeReadSource::Derived(source)
            if std::ptr::eq(source.input().inputs(),&inputs) && std::ptr::eq(source.supplier(),checked.sources().first().expect("actual original source proof"))));
        assert_eq!(
            data.value.numeric().expect("Length").rational(),
            FormulaRational::new(1000, 3, false).expect("real unbound source root")
        );
    }
    let other_inputs = inputs.clone();
    assert_eq!(other_inputs, inputs);
    let other_metadata = FormulaDeclaration::length_input(
        &name,
        FormulaInputOrigin::Parameter,
        id(8000),
        other_inputs
            .length(id(40))
            .expect("actual equal-looking foreign declaration"),
    );
    let other_join = FormulaCanonicalInput::resolve(other_metadata, &other_inputs)
        .expect("valid foreign inventory join");
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(
            || runtime.read_derived(other_join)
        ))
        .is_err(),
        "equal-looking foreign canonical inventory cannot retrieve an earned source root"
    );
    let finished = runtime.finish();
    assert_eq!(finished.completed.iter().flatten().count(), 1);
    assert_eq!(
        finished
            .completed
            .first()
            .copied()
            .flatten()
            .expect("actually completed source")
            .ordinal,
        1
    );
    assert_eq!(
        inputs
            .length(id(40))
            .expect("unchanged actual canonical state")
            .authored_value(),
        Err(LengthValueError::RequiresEvaluation {
            declaration: id(40),
            formula: id(9000)
        })
    );
}
