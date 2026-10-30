//! Independent complete policy/stage/progress outcomes through both actual tangent entry points.
#![allow(clippy::expect_used)]
use sc_core::recipe::{
    FormulaBinaryOperator as Op, FormulaBuiltin as B, FormulaEvaluatedScalar as V,
    FormulaExecutedTangent, FormulaKind as K, FormulaNumericError, FormulaNumericValue as N,
    FormulaRational as R, FormulaRationalComponent, FormulaRationalError,
    FormulaTangentError as Error, FormulaTangentExecutionError, FormulaTangentLimits as Limits,
    FormulaTangentResource as Unit,
};
use std::error::Error as _;

const FIXTURE: &str = include_str!("fixtures/formula_tangent_resources.tsv");

fn precision(value: Option<usize>) -> String {
    value.map_or_else(|| "-".into(), |bits| bits.to_string())
}

fn rational(numerator: &str, denominator: &str) -> R {
    let (negative, magnitude) = numerator
        .strip_prefix('-')
        .map_or((false, numerator), |n| (true, n));
    R::new(
        magnitude.parse().expect("original numerator"),
        denominator.parse().expect("original denominator"),
        negative,
    )
    .expect("original canonical rational")
}

fn success(result: &FormulaExecutedTangent, input: N, limits: Limits, context: &str) -> String {
    assert_eq!(result.plan().input(), input, "{context}");
    assert_eq!(result.plan().limits(), limits, "{context}");
    assert_eq!(result.value().kind(), K::Ratio, "{context}");
    let value = result.value().rational();
    assert_eq!(value.denominator(), 1, "{context}");
    format!(
        "value:{}{}\t{}\t{}\t{}\t-\t-",
        if value.is_negative() { "-" } else { "" },
        value.numerator_magnitude(),
        precision(result.precision()),
        result.reserved_integer_bits(),
        result.charged_series_terms()
    )
}

fn refusal(
    cause: Error,
    stage: &str,
    input: N,
    limits: Limits,
    progress: Option<(Option<usize>, usize, usize)>,
    context: &str,
) -> String {
    match cause {
        Error::Resources(resource) => {
            assert_eq!(resource.input(), input.rational(), "{context}");
            assert_eq!(resource.limits(), limits, "{context}");
            assert_eq!(cause.diagnostic_code(), None, "{context}");
            if let Some(progress) = progress {
                assert_eq!(
                    (
                        resource.precision(),
                        resource.reserved_integer_bits(),
                        resource.charged_series_terms()
                    ),
                    progress,
                    "{context}"
                );
            }
            let unit = match resource.unit() {
                Unit::IntegerBits => "integer_bits",
                Unit::SeriesTerms => "series_terms",
            };
            format!(
                "{stage}:{unit}:{}\t{}\t{}\t{}\t{}\t-",
                resource.maximum(),
                precision(resource.precision()),
                resource.reserved_integer_bits(),
                resource.charged_series_terms(),
                resource.requested()
            )
        }
        Error::Pole { input: original } => {
            assert_eq!(original, input.rational(), "{context}");
            assert!(progress.is_none(), "{context}");
            format!("{stage}:pole\t-\t-\t-\t-\tformula_domain")
        }
        Error::Numeric(FormulaNumericError::BuiltinInputKind {
            builtin,
            argument,
            actual,
        }) => {
            assert_eq!(
                (builtin, argument, actual),
                (B::Tan, 0, input.kind()),
                "{context}"
            );
            assert!(progress.is_none(), "{context}");
            format!("{stage}:kind\t-\t-\t-\t-\tformula_dimension")
        }
        Error::Numeric(FormulaNumericError::Rational(FormulaRationalError::RationalWidth {
            component,
            measured_bits,
        })) => {
            assert_eq!(component, FormulaRationalComponent::Numerator, "{context}");
            let (bits, reserved, charged) = progress.expect("original execution progress");
            format!(
                "{stage}:width:{measured_bits}\t{}\t{reserved}\t{charged}\t-\tformula_domain",
                precision(bits)
            )
        }
        _ => format!("{stage}:unexpected:{cause:?}"),
    }
}

fn execution_refusal(
    error: &FormulaTangentExecutionError,
    input: N,
    limits: Limits,
    context: &str,
) -> String {
    assert_eq!(error.plan().input(), input, "{context}");
    assert_eq!(error.plan().limits(), limits, "{context}");
    assert_eq!(
        error
            .source()
            .expect("original native cause")
            .downcast_ref::<Error>(),
        Some(&error.cause()),
        "{context}"
    );
    refusal(
        error.cause(),
        "execute",
        input,
        limits,
        Some((
            error.precision(),
            error.reserved_integer_bits(),
            error.charged_series_terms(),
        )),
        context,
    )
}

#[test]
fn independent_resource_rows_preserve_actual_stages_limits_demands_and_prior_progress() {
    let cosine = V::from_numeric(N::new(K::Angle, R::ZERO).expect("zero Angle"))
        .checked_cos()
        .expect("real upstream unit Ratio");
    let mut count = 0;
    for (index, row) in FIXTURE
        .lines()
        .filter(|line| !line.starts_with('#'))
        .enumerate()
    {
        let context = format!("fixture-row={};{row}", index + 1);
        let fields: [&str; 12] = row
            .split('\t')
            .collect::<Vec<_>>()
            .try_into()
            .expect("12 independent columns");
        let kind = match fields[1] {
            "angle" => K::Angle,
            "length" => K::Length,
            _ => unreachable!("authored numeric kinds"),
        };
        let input = N::new(kind, rational(fields[2], fields[3])).expect("original numeric");
        let limits = Limits::new(
            fields[4].parse().expect("bits"),
            fields[5].parse().expect("terms"),
        );
        let expected = fields[6..].join("\t");
        let native = match input.prepare_tangent(limits) {
            Err(cause) => refusal(cause, "prepare", input, limits, None, &context),
            Ok(plan) => match plan.evaluate() {
                Ok(result) => success(&result, input, limits, &context),
                Err(error) => execution_refusal(&error, input, limits, &context),
            },
        };
        assert_eq!(native, expected, "{context}:native");
        let sourced = V::from_numeric(input)
            .checked_binary(Op::Multiply, cosine)
            .expect("real unit retains original kind/value");
        assert_eq!(sourced.numeric(), Some(input), "{context}");
        assert_eq!(
            sourced.sources().iter().collect::<Vec<_>>(),
            [B::Cos],
            "{context}"
        );
        let actual = match sourced.checked_tan(limits) {
            Ok(result) => {
                assert_eq!(result.input(), sourced, "{context}");
                assert_eq!(
                    result.value().sources().iter().collect::<Vec<_>>(),
                    [B::Cos, B::Tan],
                    "{context}"
                );
                success(result.execution(), input, limits, &context)
            }
            Err(error) => {
                assert_eq!(error.input(), sourced, "{context}");
                assert_eq!(error.limits(), limits, "{context}");
                assert_eq!(error.builtin(), B::Tan, "{context}");
                assert_eq!(
                    error.diagnostic_code().unwrap_or("-"),
                    fields[11],
                    "{context}"
                );
                assert!(!error.input().sources().contains(B::Tan), "{context}");
                if let Some(cause) = error.preparation_error() {
                    assert!(error.execution_error().is_none(), "{context}");
                    assert_eq!(
                        error
                            .source()
                            .expect("original preparation cause")
                            .downcast_ref::<Error>(),
                        Some(&cause),
                        "{context}"
                    );
                    refusal(cause, "prepare", input, limits, None, &context)
                } else {
                    let native = error.execution_error().expect("original native error");
                    assert!(
                        core::ptr::eq(
                            error
                                .source()
                                .expect("original native source")
                                .downcast_ref::<FormulaTangentExecutionError>()
                                .expect("native source type"),
                            native
                        ),
                        "{context}"
                    );
                    execution_refusal(native, input, limits, &context)
                }
            }
        };
        assert_eq!(actual, expected, "{context}:sourced");
        count += 1;
    }
    assert_eq!(
        count, 33,
        "complete independent resource cohort in both native and sourced modes"
    );
}

#[test]
fn independent_resource_producer_regenerates_every_frozen_byte() {
    let root = std::env::current_dir()
        .expect("Cargo package root")
        .join("../..");
    let output = std::process::Command::new("python3")
        .args([
            "-I",
            "-B",
            "docs/tasks/artifacts/formula_structure/tangent_resource_oracle.py",
        ])
        .current_dir(root)
        .output()
        .expect("independent resource producer");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        output.stdout,
        FIXTURE.as_bytes(),
        "all independent resource/stage/progress bytes"
    );
}
