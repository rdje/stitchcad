//! Whole actual Design execution compared with independently parsed recursive book results.
use super::super::{formula_check::registry, id, input_composition::Markers, source_recipe};
use super::{context, model_limits, with_plan};
use sc_core::{design::*, name::MachineToken, recipe::*, value::*};
const FIXTURE: &str = include_str!("../fixtures/formula_scalar_design_recipe.tsv");

fn root(recipe: &str, sources: &str) -> DesignInputs<Markers> {
    let base = source_recipe::root(recipe, "1 mm/3");
    let mut supplied = base.definition().clone();
    let mut structure = supplied.structure.definition().clone();
    let mut scalars = structure.inputs.scalars().to_vec();
    let mut parameters: Vec<_> = supplied
        .parameters
        .parameters()
        .iter()
        .map(|p| p.definition().clone())
        .collect();
    // Same explicit canonical/parameter facts as the independent population,not expected values.
    for (position, (name, kind, supplier)) in [
        ("ratio_value", ScalarInputKind::Ratio, 9100),
        ("count_value", ScalarInputKind::Count, 9101),
        ("boolean_value", ScalarInputKind::Boolean, 9102),
        ("angle_value", ScalarInputKind::Angle, 9103),
        ("area_value", ScalarInputKind::Area, 9104),
        ("length_mirror", ScalarInputKind::Length, 9003),
    ]
    .into_iter()
    .enumerate()
    {
        let declaration = id(20000 + position as u128);
        scalars.push(
            ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
                id: declaration,
                source: id(4001),
                kind,
                state: ScalarInputState::Derived {
                    formula: id(supplier),
                },
            })
            .expect("actual canonical Derived with no authored value"),
        );
        parameters.push(DesignParameterDefinition {
            id: id(30000 + position as u128),
            token: MachineToken::new(name).expect("actual parameter token"),
            target: DesignParameterTarget::Scalar { declaration, kind },
        });
    }
    parameters.push(DesignParameterDefinition {
        id: id(30006),
        token: MachineToken::new("width_alias").expect("actual alias"),
        target: DesignParameterTarget::Length(id(40)),
    });
    structure.inputs = CanonicalInputs::new(structure.inputs.lengths().to_vec(), scalars)
        .expect("actual disjoint inventory");
    supplied.structure =
        DesignStructure::new(structure).expect("actual unchanged structural history");
    supplied.parameters =
        DesignParameters::new(parameters, supplied.structure.inputs()).expect("actual parameters");
    let sources: Vec<_> = sources
        .split(';')
        .map(|source| {
            let (identity, body) = source.split_once('=').expect("original source");
            (identity.parse().expect("actual id"), body)
        })
        .collect();
    supplied.formula_sources = registry(&sources);
    DesignInputs::new(supplied).expect("actual whole Design input ownership/current joins")
}
fn scalar(value: FormulaEvaluatedScalar) -> String {
    let payload = if let Some(boolean) = value.boolean() {
        boolean.to_string()
    } else {
        let r = value.numeric().expect("actual numeric").rational();
        format!(
            "{}{}/{}",
            if r.is_negative() { "-" } else { "" },
            r.numerator_magnitude(),
            r.denominator()
        )
    };
    let mut sources = value
        .sources()
        .iter()
        .map(|s| s.token())
        .collect::<Vec<_>>();
    sources.sort_unstable();
    format!(
        "{}:{}:{}",
        value.kind().token(),
        payload,
        if sources.is_empty() {
            "-".into()
        } else {
            sources.join(",")
        }
    )
}
fn scalar_token(error: &FormulaScalarExecutionError<'_>) -> &'static str {
    let FormulaScalarExecutionErrorCause::Evaluation(cause) = error.cause() else {
        unreachable!("no allocation fault fixture")
    };
    match cause.as_ref() {
        FormulaScalarExecutionFailure::Numeric(e) => e.diagnostic_code(),
        FormulaScalarExecutionFailure::ToleranceRead(e) => e.diagnostic_code(),
        FormulaScalarExecutionFailure::SizeRead(e) => e.diagnostic_code(),
        FormulaScalarExecutionFailure::Comparison(e) => e.diagnostic_code(),
        FormulaScalarExecutionFailure::LengthInput(e) => {
            assert!(matches!(
                e.reason(),
                LengthValueError::RequiresObservation { .. }
            ));
            "formula_unknown"
        }
        FormulaScalarExecutionFailure::ScalarInput(e) => {
            assert!(matches!(
                e.reason(),
                ScalarInputReadError::RequiresObservation { .. }
            ));
            "formula_unknown"
        }
        FormulaScalarExecutionFailure::Tangent(_)
        | FormulaScalarExecutionFailure::MissingTangentPolicy { .. } => {
            unreachable!("this original independent fixture contains no tangent calls")
        }
        FormulaScalarExecutionFailure::WorkspaceReservation { .. } => {
            unreachable!("complete allowances fit")
        }
    }
}
fn recipe_token(error: &FormulaScalarRecipeError<'_>) -> &'static str {
    let FormulaScalarRecipeErrorCause::Evaluation(failure) = error.cause() else {
        unreachable!("no allocation fault fixture")
    };
    match failure.as_ref() {
        FormulaScalarRecipeFailure::Expression { error, .. } => scalar_token(error),
        FormulaScalarRecipeFailure::Binding {
            rule: FormulaBindingValueRule::Storage { .. },
            ..
        } => "formula_domain",
        FormulaScalarRecipeFailure::Binding { .. } => unreachable!("valid rational denominator"),
        FormulaScalarRecipeFailure::ToleranceRead(e) => e.diagnostic_code(),
        FormulaScalarRecipeFailure::Comparison(e) => e.diagnostic_code(),
        FormulaScalarRecipeFailure::Assertion(_) => "formula_assertion",
    }
}
fn refusal(error: &FormulaScalarSourceRecipeError<'_>) -> String {
    let (statement, supplier, token) = match error.cause() {
        FormulaScalarSourceRecipeErrorCause::Recipe(e) => (
            e.statement().expect("actual failed statement"),
            "recipe".into(),
            recipe_token(e),
        ),
        FormulaScalarSourceRecipeErrorCause::RequestedSource(request) => {
            let FormulaScalarSourceGraphExecutionCause::Evaluation {
                source_index,
                error: scalar,
            } = request.cause()
            else {
                unreachable!("no allocation fixture")
            };
            let source = error
                .graph()
                .dependencies()
                .checked()
                .sources()
                .get(*source_index)
                .expect("actual deepest failed source");
            (
                request.statement(),
                source.normalized().source().id().as_bits().to_string(),
                scalar_token(scalar),
            )
        }
        _ => unreachable!(
            "same actual root;complete logical reservations and no allocator injection"
        ),
    };
    format!(
        "error:{}:{}:{supplier}:{token}\t-\t-\t-",
        statement.statement_index(),
        statement.statement().name()
    )
}
fn canonical_id(input: FormulaCanonicalInput<'_>) -> String {
    match input.adapter() {
        FormulaCanonicalInputAdapter::Length(a) => a.canonical_declaration().id(),
        FormulaCanonicalInputAdapter::Scalar(a) => a.canonical_declaration().id(),
    }
    .as_bits()
    .to_string()
}
fn joined(items: Vec<String>) -> String {
    if items.is_empty() {
        "-".into()
    } else {
        items.join(";")
    }
}
fn render(result: &FormulaExecutedScalarSourceRecipe<'_>, root: &DesignInputs<Markers>) -> String {
    let mut outcomes = vec!["ok".to_owned()];
    for completed in result.statements() {
        let statement = completed.statement();
        let ordinal = statement.statement_index();
        let name = statement.statement().name();
        assert!(std::ptr::eq(
            statement,
            result
                .recipe_plan()
                .admission()
                .proof()
                .statements()
                .get(ordinal - 1)
                .expect("original statement proof")
        ));
        outcomes.push(match completed.outcome() {
            FormulaScalarRecipeOutcome::Binding {
                expression,
                binding,
            } => format!(
                "{ordinal}:let:{name}:{}>{}",
                scalar(expression),
                scalar(binding.read().expect("actual once-stored binding"))
            ),
            FormulaScalarRecipeOutcome::Assertion(comparison) => {
                assert!(comparison.holds());
                format!(
                    "{ordinal}:assert:{name}:{}:{}=={}",
                    comparison.tolerance().name().token(),
                    scalar(comparison.left()),
                    scalar(comparison.right())
                )
            }
        });
    }
    let mut roots = vec![];
    let mut source_reads = vec![];
    for source in result.sources() {
        let identity = source.source().normalized().source().id();
        assert!(std::ptr::eq(
            source.source().normalized().source(),
            root.formula_sources()
                .source(identity)
                .expect("original source owner")
        ));
        roots.push(format!(
            "{}@{}:{}",
            identity.as_bits(),
            source.completion_ordinal(),
            scalar(source.value())
        ));
        for read in source.reads() {
            let FormulaCheckedNodeForm::Name { name, .. } = read.node().form() else {
                unreachable!("actual selected name")
            };
            let (target, role) = match read.source() {
                FormulaScalarSourceGraphReadKind::Derived(w) => {
                    assert!(std::ptr::eq(w.input().inputs(), root.inputs()));
                    (
                        canonical_id(w.input()),
                        format!(
                            "derived_{}",
                            w.supplier().normalized().source().id().as_bits()
                        ),
                    )
                }
                FormulaScalarSourceGraphReadKind::Direct(direct) => {
                    let target = match direct {
                        FormulaScalarExecutionReadKind::Length(r) => {
                            r.input().canonical_declaration().id().as_bits().to_string()
                        }
                        FormulaScalarExecutionReadKind::Scalar(r) => {
                            r.input().canonical_declaration().id().as_bits().to_string()
                        }
                        FormulaScalarExecutionReadKind::Tolerance(_)
                        | FormulaScalarExecutionReadKind::Size(_) => "-".into(),
                    };
                    (target, "direct".into())
                }
            };
            source_reads.push(format!(
                "{}:{name}:{target}:{role}:{}",
                identity.as_bits(),
                scalar(read.value())
            ));
        }
    }
    let recipe_reads = result
        .reads()
        .map(|read| {
            if read.dependency().role() == FormulaRecipeDependencyRole::AssertionTolerance {
                assert!(read.node().is_none());
            } else {
                assert_eq!(
                    read.node().expect("original name node").span(),
                    read.dependency().span()
                );
            }
            match read.source() {
                FormulaScalarSourceRecipeReadKind::Derived(w) => {
                    assert!(std::ptr::eq(w.input().inputs(), root.inputs()))
                }
                FormulaScalarSourceRecipeReadKind::Recipe(
                    FormulaScalarRecipeReadKind::PriorLet { statement, binding },
                ) => {
                    assert!(statement.statement_index() < read.dependency().statement_index());
                    assert!(std::ptr::eq(
                        statement,
                        result
                            .recipe_plan()
                            .admission()
                            .proof()
                            .statements()
                            .get(statement.statement_index() - 1)
                            .expect("real earlier statement")
                    ));
                    assert_eq!(binding.read().expect("actual stored read"), read.value());
                }
                _ => {}
            }
            format!(
                "{}:{}",
                read.dependency().declaration().name(),
                scalar(read.value())
            )
        })
        .collect();
    format!(
        "{}\t{}\t{}\t{}",
        outcomes.join(";"),
        joined(roots),
        joined(recipe_reads),
        joined(source_reads)
    )
}
#[test]
fn actual_design_matches_every_independent_complete_recipe_source_root_read_and_first_failure() {
    let mut seen = 0;
    let mut refusals = 0;
    for row in FIXTURE.lines().filter(|r| !r.starts_with('#')) {
        let mut fields = row.splitn(4, '\t');
        let label = fields.next().expect("case");
        let sources = fields.next().expect("original source collection");
        let recipe = fields
            .next()
            .expect("original ordered recipe")
            .replace("\\n", "\n");
        let expected = fields.next().expect("independent complete result/refusal");
        let root = root(&recipe, sources);
        let before = root.clone();
        let mut actual = None;
        with_plan(
            &root,
            model_limits(&root),
            |plan, inputs, original_scope| {
                actual = Some(match plan.evaluate(context()) {
                    Ok(result) => {
                        assert!(std::ptr::eq(result.inputs(), inputs));
                        assert!(std::ptr::eq(result.scope().root(), &root));
                        assert_eq!(result.scope().demand(), original_scope.demand());
                        render(result.execution(), &root)
                    }
                    Err(DesignFormulaExecutionError::Execution {
                        inputs: original,
                        scope,
                        error,
                    }) => {
                        assert!(std::ptr::eq(original, inputs));
                        assert!(std::ptr::eq(scope.root(), &root));
                        assert_eq!(scope.demand(), original_scope.demand());
                        refusal(&error)
                    }
                    Err(DesignFormulaExecutionError::Admission { .. }) => {
                        unreachable!("actual unchanged current fixture and exact content room")
                    }
                });
            },
        );
        assert_eq!(
            actual.as_deref(),
            Some(expected),
            "actual complete Design case {label}"
        );
        assert_eq!(
            root, before,
            "actual canonical states/provenance/whole model unchanged:{label}"
        );
        seen += 1;
        refusals += usize::from(expected.starts_with("error:"));
    }
    assert_eq!(
        (seen, refusals),
        (23, 12),
        "entire independent finite population"
    );
    println!("actual Design/reference:{seen} complete rows/{refusals} refusals/root joins,states,values,flags,ordinals,reads");
}
#[test]
fn actual_independent_book_reference_reproduces_all_frozen_design_recipe_bytes() {
    let root = std::env::current_dir()
        .expect("Cargo package root")
        .join("../..");
    let output = std::process::Command::new("python3")
        .args([
            "-I",
            "-B",
            "docs/tasks/artifacts/formula_structure/scalar_design_recipe_oracle.py",
        ])
        .current_dir(root)
        .output()
        .expect("guarded independent book reference");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        output.stdout,
        FIXTURE.as_bytes(),
        "complete original source/recipe/result/read/refusal bytes"
    );
}
