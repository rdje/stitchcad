//! Independent exact Fraction values/widths watch both actual native tangent entry points.
#![allow(clippy::expect_used)]
use sc_core::recipe::{
    FormulaBuiltin as B, FormulaEvaluatedScalar as V, FormulaKind as K, FormulaNumericError,
    FormulaNumericValue as N, FormulaRational as R, FormulaRationalComponent, FormulaRationalError,
    FormulaTangentError as Error, FormulaTangentLimits as Limits,
};

const FIXTURE: &str = include_str!("fixtures/formula_tangent.tsv");

fn rational(numerator: &str, denominator: &str) -> R {
    let (negative, magnitude) = numerator
        .strip_prefix('-')
        .map_or((false, numerator), |n| (true, n));
    R::new(
        magnitude.parse().expect("full magnitude"),
        denominator.parse().expect("positive denominator"),
        negative,
    )
    .expect("original admitted rational")
}
fn width(error: Error, bits: u32) {
    assert_eq!(
        error,
        Error::Numeric(FormulaNumericError::Rational(
            FormulaRationalError::RationalWidth {
                component: FormulaRationalComponent::Numerator,
                measured_bits: bits,
            }
        ))
    );
    assert_eq!(error.diagnostic_code(), Some("formula_domain"));
}

#[test]
fn independent_fraction_rows_match_values_sources_original_plans_and_exact_widths() {
    let mut counts = (0, 0, 0);
    let mut wide_valid = false;
    let mut midpoint_zero = false;
    let mut midpoint_one = false;
    for row in FIXTURE.lines().filter(|line| !line.starts_with('#')) {
        let [label, n, d, role, payload]: [&str; 5] = row
            .split('\t')
            .collect::<Vec<_>>()
            .try_into()
            .expect("five original columns");
        let original = rational(n, d);
        let input = N::new(K::Angle, original).expect("actual original Angle");
        match role {
            "value" => counts.0 += 1,
            "pole" => counts.1 += 1,
            "width" => counts.2 += 1,
            _ => unreachable!(),
        }
        for limits in [Limits::new(16412, 7680), Limits::new(70000, 40000)] {
            let prepared = input.prepare_tangent(limits);
            let wrapped = V::from_numeric(input).checked_tan(limits);
            if role == "pole" {
                assert_eq!(
                    prepared.expect_err("actual exact pole"),
                    Error::Pole { input: original },
                    "{label}"
                );
                let error = wrapped.expect_err("same actual sourced pole");
                assert_eq!(error.input(), V::from_numeric(input));
                assert_eq!(error.limits(), limits);
                assert_eq!(error.builtin(), B::Tan);
                assert_eq!(error.diagnostic_code(), Some("formula_domain"));
                assert_eq!(
                    error.preparation_error(),
                    Some(Error::Pole { input: original })
                );
                assert!(error.execution_error().is_none());
                continue;
            }
            let result = prepared.expect("actual nonpole preparation").evaluate();
            if role == "width" {
                let bits = payload.parse().expect("independent exact bit count");
                let error = result.expect_err("original exact width");
                assert_eq!(error.plan().input(), input);
                assert_eq!(error.plan().limits(), limits);
                assert!(error.charged_series_terms() <= limits.maximum_series_terms());
                assert!(error.reserved_integer_bits() <= limits.maximum_integer_bits());
                width(error.cause(), bits);
                let error = wrapped.expect_err("original wrapped width");
                assert_eq!(error.input(), V::from_numeric(input));
                assert_eq!(error.limits(), limits);
                assert!(error.preparation_error().is_none());
                width(
                    error
                        .execution_error()
                        .expect("actual native error")
                        .cause(),
                    bits,
                );
                continue;
            }
            let result = result.expect("independently certified finite value");
            let expected = rational(payload, "1");
            assert_eq!(result.value().kind(), K::Ratio);
            assert_eq!(
                result.value().rational(),
                expected,
                "independent native value:{label}"
            );
            assert_eq!(result.plan().input(), input);
            assert_eq!(result.plan().limits(), limits);
            assert!(result.charged_series_terms() <= limits.maximum_series_terms());
            assert!(result.reserved_integer_bits() <= limits.maximum_integer_bits());
            let wrapped = wrapped.expect("actual sourced native value");
            assert_eq!(wrapped.input(), V::from_numeric(input));
            assert_eq!(wrapped.execution().plan().input(), input);
            assert_eq!(wrapped.execution().plan().limits(), limits);
            assert_eq!(wrapped.execution().precision(), result.precision());
            assert_eq!(
                wrapped.execution().charged_series_terms(),
                result.charged_series_terms()
            );
            assert_eq!(
                wrapped.execution().reserved_integer_bits(),
                result.reserved_integer_bits()
            );
            assert_eq!(
                wrapped.value().numeric().expect("Ratio").rational(),
                expected,
                "wrapped:{label}"
            );
            assert_eq!(
                wrapped.value().sources().iter().collect::<Vec<_>>(),
                [B::Tan]
            );
            wide_valid |= expected.numerator_magnitude() > i64::MAX.unsigned_abs() as u128;
            if label.starts_with("near_midpoint") {
                midpoint_zero |= expected.numerator_magnitude() == 0;
                midpoint_one |= expected.numerator_magnitude() == 1;
            }
        }
    }
    assert_eq!(counts, (125, 10, 4));
    assert!(
        wide_valid && midpoint_zero && midpoint_one,
        "real wide finite and both sides of actual midpoint"
    );
    println!("independent tangent:139 exact rows/125 values/10 poles/4 exact widths;two policies/both native entry points");
}

#[test]
fn independent_fraction_producer_regenerates_every_frozen_byte() {
    let root = std::env::current_dir()
        .expect("Cargo package root")
        .join("../..");
    let output = std::process::Command::new("python3")
        .args([
            "-I",
            "-B",
            "docs/tasks/artifacts/formula_structure/tangent_value_oracle.py",
        ])
        .current_dir(root)
        .output()
        .expect("independent guarded Fraction reference");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        output.stdout,
        FIXTURE.as_bytes(),
        "all original independent fixture bytes"
    );
}

#[test]
fn actual_upstream_sources_preserve_all_original_tangent_outcomes_through_signed_storage() {
    use sc_core::{
        name::MachineToken,
        recipe::{FormulaBinaryOperator as Op, FormulaBindingValueRule, FormulaToleranceName},
    };
    let exact =
        |kind, rational| V::from_numeric(N::new(kind, rational).expect("actual exact operand"));
    let cosine = exact(K::Angle, R::ZERO)
        .checked_cos()
        .expect("actual unit cosine");
    let root = exact(K::Ratio, R::from_signed_integer(1_000_000))
        .checked_sqrt()
        .expect("actual perfect Ratio root");
    let zero = root
        .checked_binary(Op::Multiply, exact(K::Angle, R::ZERO))
        .expect("actual sourced zero Angle");
    let limits = Limits::new(16412, 7680);
    let name = MachineToken::new("actual_tangent_binding").expect("binding name");
    let mut counts = (0, 0, 0, 0);
    for (index, row) in FIXTURE
        .lines()
        .filter(|line| !line.starts_with('#'))
        .enumerate()
    {
        let row_number = index + 1;
        let [_label, n, d, role, payload]: [&str; 5] = row
            .split('\t')
            .collect::<Vec<_>>()
            .try_into()
            .expect("five original columns");
        let original = N::new(K::Angle, rational(n, d)).expect("original full-width Angle");
        let input = V::from_numeric(original)
            .checked_binary(Op::Add, zero)
            .expect("original Angle plus actual zero")
            .checked_binary(Op::Multiply, cosine)
            .expect("original Angle times actual unit Ratio");
        assert_eq!(
            input.numeric(),
            Some(original),
            "fixture-row={row_number};{row}"
        );
        for call in [B::Cos, B::Sqrt] {
            assert!(input.sources().contains(call));
        }
        assert_eq!(input.sources().iter().count(), 2);
        let actual = input.checked_tan(limits);
        if role == "value" {
            counts.0 += 1;
            assert!(
                actual.is_ok(),
                "fixture-row={row_number};{row}:original admitted value:{actual:?}"
            );
            let result = actual.expect("asserted actual successful call");
            let output = result.value();
            let expected = rational(payload, "1");
            assert_eq!(
                output.numeric().expect("Ratio").rational(),
                expected,
                "fixture-row={row_number};{row}"
            );
            assert_eq!(result.input(), input, "fixture-row={row_number};{row}");
            assert_eq!(result.execution().plan().input(), original);
            assert_eq!(result.execution().plan().limits(), limits);
            assert!(result.execution().charged_series_terms() <= limits.maximum_series_terms());
            assert!(result.execution().reserved_integer_bits() <= limits.maximum_integer_bits());
            for call in [B::Cos, B::Sqrt, B::Tan] {
                assert!(
                    output.sources().contains(call),
                    "fixture-row={row_number};{row}:missing actual source:{call:?}"
                );
            }
            assert_eq!(
                output.sources().iter().count(),
                3,
                "fixture-row={row_number};{row}"
            );
            assert!(output
                .sources()
                .check_tolerance(FormulaToleranceName::Numerical)
                .is_err());
            let maximum_magnitude = u128::from(if expected.is_negative() {
                i64::MIN.unsigned_abs()
            } else {
                i64::MAX.unsigned_abs()
            });
            if expected.numerator_magnitude() > maximum_magnitude {
                counts.3 += 1;
                let failure = output
                    .bind(&name)
                    .expect_err("original signed storage refusal");
                assert_eq!(failure.kind(), K::Ratio);
                assert_eq!(failure.name(), &name);
                assert_eq!(
                    failure.rule(),
                    FormulaBindingValueRule::Storage {
                        minimum: i64::MIN,
                        maximum: i64::MAX,
                        measured: expected,
                    },
                    "fixture-row={row_number};{row}"
                );
            } else {
                let binding = output.bind(&name).expect("actual signed Ratio storage");
                assert_eq!(
                    binding.read().expect("actual stored read"),
                    output,
                    "fixture-row={row_number};{row}"
                );
            }
        } else {
            assert!(
                actual.is_err(),
                "fixture-row={row_number};{row}:original mathematical refusal:{actual:?}"
            );
            let error = actual.expect_err("asserted original mathematical refusal");
            assert_eq!(error.input(), input, "fixture-row={row_number};{row}");
            assert_eq!(error.limits(), limits);
            assert_eq!(error.diagnostic_code(), Some("formula_domain"));
            assert!(!error.input().sources().contains(B::Tan));
            if role == "pole" {
                counts.1 += 1;
                assert_eq!(
                    error.preparation_error(),
                    Some(Error::Pole {
                        input: original.rational()
                    }),
                    "fixture-row={row_number};{row}"
                );
                assert!(error.execution_error().is_none());
            } else {
                assert_eq!(role, "width");
                counts.2 += 1;
                assert!(error.preparation_error().is_none());
                let native = error
                    .execution_error()
                    .expect("actual native execution cause");
                let bits = payload.parse().expect("independent exact bit count");
                assert_eq!(
                    native.cause(),
                    Error::Numeric(FormulaNumericError::Rational(
                        FormulaRationalError::RationalWidth {
                            component: FormulaRationalComponent::Numerator,
                            measured_bits: bits
                        }
                    )),
                    "fixture-row={row_number};{row}"
                );
                assert_eq!(native.plan().input(), original);
                assert_eq!(native.plan().limits(), limits);
                assert!(native.charged_series_terms() <= limits.maximum_series_terms());
                assert!(native.reserved_integer_bits() <= limits.maximum_integer_bits());
            }
        }
    }
    assert_eq!(
        counts,
        (125, 10, 4, 4),
        "all original outcomes and separate signed-storage refusals"
    );
}
