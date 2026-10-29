//! Independent complete recipe rows; no expected value is computed from the native executor.
use super::*;
use sc_core::recipe::FormulaEvaluatedScalar;
const FIXTURE: &str = include_str!("../../fixtures/formula_scalar_recipe_execution.tsv");

fn scalar(value: FormulaEvaluatedScalar) -> String {
    let payload = if let Some(boolean) = value.boolean() {
        boolean.to_string()
    } else {
        let value = value.numeric().expect("actual numeric").rational();
        format!(
            "{}{}/{}",
            if value.is_negative() { "-" } else { "" },
            value.numerator_magnitude(),
            value.denominator()
        )
    };
    let mut sources = value
        .sources()
        .iter()
        .map(|source| source.token())
        .collect::<Vec<_>>();
    sources.sort_unstable();
    format!(
        "{}:{}:{}",
        value.kind().token(),
        payload,
        if sources.is_empty() {
            "-".to_owned()
        } else {
            sources.join(",")
        }
    )
}
fn token(error: &FormulaScalarRecipeError<'_>) -> &'static str {
    match failure(error) {
        Failure::Expression { error, .. } => {
            let ScalarCause::Evaluation(cause) = error.cause() else {
                unreachable!("no allocation fixture");
            };
            match cause.as_ref() {
                ScalarFailure::Numeric(error) => error.diagnostic_code(),
                ScalarFailure::ToleranceRead(error) => error.diagnostic_code(),
                ScalarFailure::SizeRead(error) => error.diagnostic_code(),
                ScalarFailure::Comparison(error) => error.diagnostic_code(),
                ScalarFailure::LengthInput(_) | ScalarFailure::ScalarInput(_) => {
                    unreachable!("this finite recipe population has no external initial inputs")
                }
                ScalarFailure::WorkspaceReservation { .. } => {
                    unreachable!("actual complete allowance fits")
                }
            }
        }
        Failure::Binding {
            rule: Storage::Storage { .. },
            ..
        } => "formula_domain",
        Failure::Binding {
            rule: Storage::Rounding(_),
            ..
        } => unreachable!("valid actual rational denominator"),
        Failure::ToleranceRead(error) => error.diagnostic_code(),
        Failure::Comparison(error) => error.diagnostic_code(),
        Failure::Assertion(_) => "formula_assertion",
    }
}
fn render(result: Result<FormulaExecutedScalarRecipe<'_>, FormulaScalarRecipeError<'_>>) -> String {
    match result {
        Ok(result) => {
            let mut outcomes = vec!["ok".to_owned()];
            for completed in result.statements() {
                let statement = completed.statement();
                let ordinal = statement.statement_index();
                let name = statement.statement().name();
                outcomes.push(match completed.outcome() {
                    Outcome::Binding {
                        expression,
                        binding,
                    } => format!(
                        "{ordinal}:let:{name}:{}>{}",
                        scalar(expression),
                        scalar(binding.read().expect("actual stored binding"))
                    ),
                    Outcome::Assertion(comparison) => {
                        assert!(
                            comparison.holds(),
                            "only actual true comparisons are accepted"
                        );
                        format!(
                            "{ordinal}:assert:{name}:{}:{}=={}",
                            comparison.tolerance().name().token(),
                            scalar(comparison.left()),
                            scalar(comparison.right())
                        )
                    }
                });
            }
            let reads = result
                .reads()
                .map(|read| {
                    format!(
                        "{}:{}",
                        read.dependency().declaration().name(),
                        scalar(read.value())
                    )
                })
                .collect::<Vec<_>>();
            format!(
                "{}\t{}",
                outcomes.join(";"),
                if reads.is_empty() {
                    "-".to_owned()
                } else {
                    reads.join(";")
                }
            )
        }
        Err(error) => {
            let statement = error.statement().expect("actual refused statement");
            format!(
                "error:{}:{}:{}\t-",
                statement.statement_index(),
                statement.statement().name(),
                token(&error)
            )
        }
    }
}

#[test]
fn actual_whole_executor_matches_every_independent_recipe_outcome_source_read_and_refusal() {
    let inputs = CanonicalInputs::new(vec![], vec![]).expect("intrinsic-only fixture inventory");
    let mut seen = 0;
    let mut refusals = 0;
    for row in FIXTURE.lines().filter(|row| !row.starts_with('#')) {
        let (source, expected) = row.split_once('\t').expect("complete frozen row");
        let source = source.replace("\\n", "\n");
        let recipe = FormulaRecipe::parse(&source)
            .expect("independent actual syntax")
            .normalize_literals()
            .expect("actual input conversion");
        let proof = recipe
            .check_kinds(FormulaNamespace::new([]).expect("namespace"))
            .expect("whole actual static proof");
        let result = proof
            .admit_execution(LIMITS)
            .expect("explicit complete allowance")
            .prepare_scalar(&inputs)
            .expect("all actual sources")
            .evaluate(context());
        assert_eq!(
            render(result),
            expected,
            "complete independent recipe {source}"
        );
        seen += 1;
        refusals += usize::from(expected.starts_with("error:"));
    }
    assert_eq!(
        (seen, refusals),
        (18, 7),
        "complete finite independent row population"
    );
    println!("actual recipe/reference: {seen} complete rows / {refusals} refusals / outcomes,sources,read order");
}

#[test]
fn actual_independent_reference_reproduces_every_frozen_recipe_byte() {
    let root = std::env::current_dir()
        .expect("Cargo package root")
        .join("../..");
    let output = std::process::Command::new("python3")
        .args([
            "-I",
            "-B",
            "docs/tasks/artifacts/formula_structure/scalar_recipe_execution_oracle.py",
        ])
        .current_dir(root)
        .output()
        .expect("guarded independent whole reference");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        output.stdout,
        FIXTURE.as_bytes(),
        "complete exact frozen source/outcome/read/refusal bytes"
    );
}
