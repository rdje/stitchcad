use super::failure::SourceOperandFailure;
use super::*;
use crate::recipe::execution::recipe_result::RecipeReadSource;
use crate::{name::MachineToken, ontology::EntityId, recipe::*, value::*};
use sc_units::{Length, Tolerance, ToleranceClass};

struct Fixture {
    inputs: CanonicalInputs,
    sources: FormulaSources,
    names: Vec<MachineToken>,
}
impl Fixture {
    #[allow(clippy::expect_used)]
    fn new(width: &str, triple: &str, flag: &str) -> Self {
        let id = EntityId::from_bits;
        let inputs = CanonicalInputs::new(
            [40, 41, 42]
                .into_iter()
                .map(|target| {
                    LengthDeclaration::new(LengthDeclarationDefinition {
                        id: id(target),
                        source: id(4000),
                        state: match target {
                            40 => LengthState::Derived { formula: id(9000) },
                            41 => LengthState::Derived { formula: id(9001) },
                            _ => LengthState::Unknown {
                                observation: id(4900),
                            },
                        },
                    })
                    .expect("actual canonical length")
                })
                .collect(),
            vec![
                ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
                    id: id(43),
                    source: id(4001),
                    kind: ScalarInputKind::Boolean,
                    state: ScalarInputState::Derived { formula: id(9002) },
                })
                .expect("actual canonical Boolean"),
            ],
        )
        .expect("actual canonical owner");
        let sources: Vec<_> = [(9000, width), (9001, triple), (9002, flag)]
            .into_iter()
            .map(|(identity, body)| {
                FormulaSource::new(
                    id(identity),
                    FormulaExpressionDocument::new(body.into())
                        .expect("actual original source body"),
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
        Self {
            inputs,
            sources: FormulaSources::new(sources, limits).expect("actual source owners"),
            names: ["width", "triple", "unknown", "flag"]
                .into_iter()
                .map(|s| MachineToken::new(s).expect("actual parameter name"))
                .collect(),
        }
    }
    #[allow(clippy::expect_used)]
    fn namespace(&self) -> FormulaNamespace<'_> {
        FormulaNamespace::new(self.names.iter().zip([40, 41, 42, 43]).enumerate().map(
            |(index, (name, target))| {
                let declaration =
                    if let Some(record) = self.inputs.length(EntityId::from_bits(target)) {
                        FormulaDeclaration::length_input(
                            name,
                            FormulaInputOrigin::Parameter,
                            EntityId::from_bits(8000 + index as u128),
                            record,
                        )
                    } else {
                        let record = self
                            .inputs
                            .scalar(EntityId::from_bits(target))
                            .expect("actual general target");
                        FormulaDeclaration::input(
                            name,
                            FormulaScalarInputOrigin::Parameter,
                            EntityId::from_bits(8000 + index as u128),
                            record.id(),
                            record.kind().into(),
                        )
                    };
                FormulaInitialDeclaration::try_from(declaration).expect("actual initial metadata")
            },
        ))
        .expect("actual initial namespace")
    }
    #[allow(clippy::expect_used)]
    fn with_runtimes(
        &self,
        recipe: &str,
        context: Context<'_>,
        inspect: impl for<'a> FnOnce(RecipeRuntime<'a>, SourceRuntime<'a>),
    ) {
        let s = self.sources.sources().len();
        let n = self
            .sources
            .sources()
            .iter()
            .map(|s| s.expression().node_count())
            .sum();
        let normalized = FormulaSourceInputs::new(&self.inputs, &self.sources)
            .expect("actual complete source join")
            .normalize_sources(FormulaSourceNormalizationLimits::new(s, n, s + 2 * n))
            .expect("actual source arenas");
        let checked = normalized
            .check_sources(
                &self.namespace(),
                FormulaSourceCheckLimits::new(s, n, 3, s + 2 * n),
            )
            .expect("complete original source proofs");
        let d = checked
            .sources()
            .iter()
            .map(|s| s.proof().dependencies().len())
            .sum();
        let dependencies = checked
            .prepare_dependencies(FormulaSourceDependencyLimits::new(s, d, s + d + 1))
            .expect("actual complete relation");
        let graph = dependencies
            .admit_acyclic(FormulaSourceCycleLimits::new(s, 2 * s))
            .expect("actual acyclic proof");
        let source_plan = graph
            .prepare_scalar_sources(FormulaSourceGraphExecutionLimits::new(
                s,
                n,
                d,
                2 * s + 2 * n + d,
                s,
            ))
            .expect("complete scalar source plan");
        let recipe = FormulaRecipe::parse(recipe)
            .expect("actual ordered recipe")
            .normalize_literals()
            .expect("actual recipe arenas");
        let proof = recipe
            .check_kinds(self.namespace())
            .expect("actual whole recipe proof");
        let n = proof
            .statements()
            .iter()
            .map(|s| match s.kind() {
                FormulaCheckedStatementKind::Let { expression } => expression.node_count(),
                FormulaCheckedStatementKind::Assert { left, right } => {
                    left.node_count() + right.node_count()
                }
            })
            .sum();
        let s = proof.statements().len();
        let d = proof.dependencies().count();
        let recipe_plan = proof
            .admit_execution(FormulaRecipeExecutionLimits::new(s, n, 2 * n + d, s))
            .expect("complete recipe populations")
            .prepare_scalar(&self.inputs)
            .expect("actual recipe sources");
        let source =
            SourceRuntime::new(source_plan, context).expect("all source buffers before values");
        let recipe =
            RecipeRuntime::new(recipe_plan, context).expect("all recipe buffers before values");
        inspect(recipe, source);
    }
}
fn empty() -> Context<'static> {
    Context::new(
        FormulaToleranceContext::empty(),
        FormulaSizeContext::empty(),
    )
}

#[test]
#[allow(clippy::expect_used)]
fn paired_whole_loop_stores_real_lets_once_and_shares_unbound_sources_across_statements() {
    let fixture = Fixture::new("1 mm/3", "width*3", "1==2");
    fixture.with_runtimes("let prior:length=1 mm/3\nlet total:length=prior+width+width\nlet tripled:length=triple\nassert a:eps_num=total==1000 um\nassert b:eps_num=tripled==1000 um",empty(),|recipe,mut sources|{
        assert!(recipe.statements.is_empty());
        let completed=recipe.execute(Some(&mut sources)).expect("actual complete ordered recipe");
        assert_eq!(completed.statements.len(),5);
        let values:Vec<_>=completed.statements.iter().filter_map(|s|match s.outcome {
            Outcome::Binding{expression,binding}=>Some((expression,binding.read().expect("actual stored binding"))),_=>None}).collect();
        assert_eq!(values.len(),3);
        assert_eq!(values.first().expect("actual prior").1.numeric().expect("Length").rational(),FormulaRational::new(333,1,false).expect("stored first let"));
        let total=values.get(1).expect("actual total");
        assert_eq!(total.0.numeric().expect("Length").rational(),FormulaRational::new(2999,3,false).expect("real unbound RHS"));
        assert_eq!(total.1.numeric().expect("Length").rational(),FormulaRational::new(1000,1,false).expect("only actual total let stores once"));
        assert_eq!(completed.plan.sources.iter().filter_map(|s|s.read).filter(|r|matches!(r.source,RecipeReadSource::Derived(_))).count(),3);
        let sources=sources.finish();assert_eq!(sources.completed.iter().flatten().count(),2);
        let width=sources.completed.first().copied().flatten().expect("actual bare width root");
        assert_eq!(width.value.numeric().expect("Length").rational(),FormulaRational::new(1000,3,false).expect("source edge never stored"));assert_eq!(width.ordinal,1);
        assert_eq!(sources.completed.get(1).copied().flatten().expect("actual triple root").ordinal,2);
        assert!(sources.completed.get(2).expect("unused actual Boolean source").is_none());
    });
}
#[test]
#[allow(clippy::expect_used)]
fn paired_lazy_false_source_keeps_call_flags_and_real_assertion_class_after_storage() {
    let fixture = Fixture::new("unknown", "width*3", "cos(0 deg)==0.0");
    fixture.with_runtimes("let selected:length=if(flag,width,1 mm)\nassert check:eps_num=selected==1 mm",empty(),|recipe,mut sources|{
        let error=recipe.execute(Some(&mut sources)).err().expect("actual T1 approximation guard");
        assert!(matches!(error,RecipeRuntimeError::Recipe(error) if error.statement().expect("actual failed assertion").statement_index()==2
            && matches!(error.cause(),FormulaScalarRecipeErrorCause::Evaluation(failure) if matches!(failure.as_ref(),Failure::Comparison(_)))));
    });
    fixture.with_runtimes(
        "let selected:length=if(flag,width,1 mm)\nassert check:eps_geo=selected==1 mm",
        empty(),
        |recipe, mut sources| {
            let completed = recipe
                .execute(Some(&mut sources))
                .expect("actual T2 assertion");
            assert_eq!(completed.statements.len(), 2);
            let sources = sources.finish();
            assert_eq!(sources.completed.iter().flatten().count(), 1);
            assert!(sources
                .completed
                .first()
                .expect("unused unknown-backed width source")
                .is_none());
            assert_eq!(
                sources
                    .completed
                    .get(2)
                    .copied()
                    .flatten()
                    .expect("actual false condition source")
                    .ordinal,
                1
            );
        },
    );
}
#[test]
#[allow(clippy::expect_used)]
fn paired_source_failure_retains_actual_requester_and_supplier_without_accepted_prefix() {
    let fixture = Fixture::new("1 mm/3", "unknown", "1==2");
    let text = "let prior:length=width\nlet total:length=triple";
    fixture.with_runtimes(text,empty(),|recipe,mut sources|{
        let error=recipe.execute(Some(&mut sources)).err().expect("actual source failure after successful prior let");
        assert!(matches!(error,RecipeRuntimeError::Source{statement,part:Part::Binding,error:SourceOperandFailure::Execution(error)}
            if statement.statement_index()==2 && text.get(error.node.span().start()..error.node.span().end())==Some("triple")
            && core::ptr::eq(error.input.inputs(),&fixture.inputs)
            && matches!(error.cause,FormulaScalarSourceGraphExecutionCause::Evaluation{source_index:1,..})));
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||sources.finish())).is_err(),"failed source runtime cannot publish the earlier root prefix");
    });
}
#[test]
#[allow(clippy::expect_used)]
fn paired_runtime_requires_original_borrowed_provider_and_label_sources() {
    let fixture = Fixture::new("1 mm/3", "width*3", "1==2");
    let first = Tolerance::new(
        ToleranceClass::PhysicalAcceptance,
        Length::from_micrometres(10).expect("Length"),
        "actual fixture factory limit",
    )
    .expect("actual tolerance");
    let second = first.clone();
    assert_eq!(first, second);
    assert!(!core::ptr::eq(&first, &second));
    let label = "actual_fixture";
    let other_label = label.to_owned();
    let context = |tolerance, label| {
        Context::new(
            FormulaToleranceContext::new(FormulaToleranceContextDefinition {
                label: Some(label),
                format: None,
                importer: None,
                physical: Some(FormulaToleranceProvider {
                    source: EntityId::from_bits(5100),
                    tolerance,
                }),
            })
            .expect("actual class-checked provider"),
            FormulaSizeContext::empty(),
        )
    };
    let original = context(&first, label);
    fixture.with_runtimes(
        "let actual:length=width\nassert fit:eps_phys=actual==333 um",
        original,
        |recipe, mut sources| {
            assert!(sources.matches(&fixture.inputs, original));
            assert!(
                !sources.matches(&fixture.inputs, context(&second, label)),
                "equal quantitative copies cannot replace the selected borrowed provider"
            );
            assert!(
                !sources.matches(&fixture.inputs, context(&first, other_label.as_str())),
                "equal label bytes do not replace the original diagnostic source"
            );
            let completed = recipe
                .execute(Some(&mut sources))
                .expect("actual original provider context");
            assert_eq!(completed.statements.len(), 2);
        },
    );
}
