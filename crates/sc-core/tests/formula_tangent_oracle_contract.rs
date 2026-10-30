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
