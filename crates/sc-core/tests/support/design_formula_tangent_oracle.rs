//! Exact independent paired reference watches both native and actual Design tangent entry points.
use super::super::super::formula_tangent::with_checked;
use super::super::context;
use super::{refusal as legacy_refusal, render, root};
use sc_core::{design::*, recipe::*};
const FIXTURE: &str = include_str!("../fixtures/formula_tangent_design_recipe.tsv");

fn limits(text: &str) -> FormulaTangentLimits {
    let (bits, terms) = text.split_once('/').expect("actual selected policy");
    FormulaTangentLimits::new(bits.parse().expect("bits"), terms.parse().expect("terms"))
}
fn token(error: &FormulaScalarExecutionError<'_>) -> Option<String> {
    let FormulaScalarExecutionErrorCause::Evaluation(failure) = error.cause() else {
        return None;
    };
    let FormulaScalarExecutionFailure::Tangent(tangent) = failure.as_ref() else {
        return None;
    };
    assert_eq!(error.tangent_limits(), Some(tangent.limits()));
    let cause = tangent.preparation_error().unwrap_or_else(|| {
        tangent
            .execution_error()
            .expect("original native failure")
            .cause()
    });
    Some(match cause {
        FormulaTangentError::Resources(resource) => {
            assert_eq!(resource.limits(), tangent.limits());
            assert_eq!(
                resource.input(),
                tangent.input().numeric().expect("actual Angle").rational()
            );
            assert_eq!(
                tangent.diagnostic_code(),
                None,
                "no invented language diagnostic"
            );
            let angle = resource.input();
            let unit = match resource.unit() {
                FormulaTangentResource::IntegerBits => "integer_bits",
                FormulaTangentResource::SeriesTerms => "series_terms",
            };
            format!(
                "resources.{unit}.{}{}.{}.{}.{}.{}.{}.{}",
                if angle.is_negative() { "-" } else { "" },
                angle.numerator_magnitude(),
                angle.denominator(),
                resource.precision().unwrap_or(0),
                resource.charged_series_terms(),
                resource.reserved_integer_bits(),
                resource.maximum(),
                resource.requested()
            )
        }
        FormulaTangentError::Numeric(FormulaNumericError::Rational(
            FormulaRationalError::RationalWidth {
                component: FormulaRationalComponent::Numerator,
                measured_bits,
            },
        )) => format!("width.{measured_bits}"),
        _ => tangent
            .diagnostic_code()
            .expect("original mathematical diagnostic")
            .into(),
    })
}
fn refusal(error: &FormulaScalarSourceRecipeError<'_>) -> String {
    let (statement, supplier, scalar) = match error.cause() {
        FormulaScalarSourceRecipeErrorCause::Recipe(recipe) => {
            let FormulaScalarRecipeErrorCause::Evaluation(failure) = recipe.cause() else {
                return legacy_refusal(error);
            };
            let FormulaScalarRecipeFailure::Expression { error, .. } = failure.as_ref() else {
                return legacy_refusal(error);
            };
            assert_eq!(recipe.tangent_limits(), error.tangent_limits());
            (
                recipe.statement().expect("actual failed statement"),
                "recipe".into(),
                error,
            )
        }
        FormulaScalarSourceRecipeErrorCause::RequestedSource(request) => {
            let FormulaScalarSourceGraphExecutionCause::Evaluation {
                source_index,
                error: scalar,
            } = request.cause()
            else {
                return legacy_refusal(error);
            };
            let source = error
                .graph()
                .dependencies()
                .checked()
                .sources()
                .get(*source_index)
                .expect("actual deepest failed supplier");
            assert_eq!(scalar.tangent_limits(), error.source_tangent_limits());
            (
                request.statement(),
                source.normalized().source().id().as_bits().to_string(),
                scalar.as_ref(),
            )
        }
        _ => return legacy_refusal(error),
    };
    let Some(token) = token(scalar) else {
        return legacy_refusal(error);
    };
    format!(
        "error:{}:{}:{supplier}:{token}\t-\t-\t-",
        statement.statement_index(),
        statement.statement().name()
    )
}

#[test]
fn both_actual_paired_modes_match_independent_tangent_values_resources_storage_reads_and_failures()
{
    let mut counts = (0, 0, 0);
    for row in FIXTURE.lines().filter(|row| !row.starts_with('#')) {
        let mut fields = row.splitn(6, '\t');
        let label = fields.next().expect("case");
        let supplier = limits(fields.next().expect("supplier policy"));
        let caller = limits(fields.next().expect("caller policy"));
        let sources = fields.next().expect("actual original source text");
        let text = fields
            .next()
            .expect("actual original recipe")
            .replace("\\n", "\n");
        let expected = fields
            .next()
            .expect("independent complete result or first failure");
        let root = root(&text, sources);
        let before = root.clone();
        with_checked(&root, |sources, recipe, graph, original_scope| {
            let source_limits = FormulaSourceGraphExecutionLimits::new(16, 256, 256, 1024, 16);
            let recipe_limits = FormulaRecipeExecutionLimits::new(16, 256, 1024, 16);
            let inputs = sources
                .plan_inputs_with_tangent_limits(
                    recipe,
                    source_limits,
                    recipe_limits,
                    supplier,
                    caller,
                )
                .expect("actual same-root proof/policy join");
            let actual = match inputs
                .prepare_scalar(graph)
                .expect("actual complete native preparation")
                .evaluate(context())
            {
                Ok(result) => {
                    assert!(core::ptr::eq(result.inputs(), &inputs));
                    assert!(core::ptr::eq(result.scope().root(), &root));
                    assert_eq!(result.scope().demand(), original_scope.demand());
                    assert_eq!(
                        result.execution().source_plan().tangent_limits(),
                        Some(supplier)
                    );
                    assert_eq!(
                        result.execution().recipe_plan().tangent_limits(),
                        Some(caller)
                    );
                    render(result.execution(), &root)
                }
                Err(DesignFormulaExecutionError::Execution {
                    inputs: actual,
                    scope,
                    error,
                }) => {
                    assert!(core::ptr::eq(actual, &inputs));
                    assert!(core::ptr::eq(scope.root(), &root));
                    assert_eq!(scope.demand(), original_scope.demand());
                    assert_eq!(error.source_tangent_limits(), Some(supplier));
                    assert_eq!(error.recipe_tangent_limits(), Some(caller));
                    refusal(&error)
                }
                Err(DesignFormulaExecutionError::Admission { .. }) => {
                    unreachable!("unchanged actual fixture remains current")
                }
            };
            assert_eq!(actual, expected, "actual Design case {label}");
            let sources = graph
                .prepare_scalar_sources_with_tangent_limits(source_limits, supplier)
                .expect("actual complete native graph");
            let recipe = recipe
                .proof()
                .admit_execution(recipe_limits)
                .expect("actual whole recipe admission")
                .prepare_scalar_with_tangent_limits(root.inputs(), caller)
                .expect("actual complete native recipe");
            let actual = match recipe.evaluate_with_sources(sources, context()) {
                Ok(result) => {
                    assert_eq!(result.source_plan().tangent_limits(), Some(supplier));
                    assert_eq!(result.recipe_plan().tangent_limits(), Some(caller));
                    render(&result, &root)
                }
                Err(error) => {
                    assert_eq!(error.source_tangent_limits(), Some(supplier));
                    assert_eq!(error.recipe_tangent_limits(), Some(caller));
                    refusal(&error)
                }
            };
            assert_eq!(actual, expected, "actual native paired case {label}");
            counts.2 += 2;
        });
        assert_eq!(
            root, before,
            "all actual model/canonical states unchanged:{label}"
        );
        counts.0 += 1;
        counts.1 += usize::from(expected.starts_with("error:"));
    }
    assert_eq!(
        counts,
        (24, 14, 48),
        "entire independent paired population in both actual modes"
    );
    println!(
        "actual paired tangent reference:{} rows/{} refusals/{} complete mode comparisons",
        counts.0, counts.1, counts.2
    );
}

#[test]
fn independent_fraction_book_composition_regenerates_every_frozen_paired_tangent_byte() {
    let root = std::env::current_dir()
        .expect("Cargo package root")
        .join("../..");
    let output = std::process::Command::new("python3")
        .args([
            "-I",
            "-B",
            "docs/tasks/artifacts/formula_structure/tangent_design_recipe_oracle.py",
        ])
        .current_dir(root)
        .output()
        .expect("guarded independent paired tangent oracle");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        output.stdout,
        FIXTURE.as_bytes(),
        "all original source/recipe/policy/result/refusal/read bytes"
    );
}
