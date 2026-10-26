//! Focused native sin/cos value boundary; sourced expression execution follows separately.
use sc_core::recipe::{
    FormulaBuiltin as F, FormulaKind as K, FormulaNumericError as Error, FormulaNumericValue as V,
    FormulaRational as R,
};

#[test]
#[allow(clippy::expect_used)]
fn compiled_values_match_independent_full_turn_and_midpoint_neighbors() {
    let mut rows = 0;
    for row in include_str!("fixtures/formula_sincos.tsv")
        .lines()
        .filter(|row| !row.starts_with('#'))
    {
        let cells: [&str; 4] = row
            .split('\t')
            .collect::<Vec<_>>()
            .try_into()
            .expect("four fixture columns");
        let [numerator, denominator, sine, cosine] = cells;
        let (negative, magnitude) = numerator
            .strip_prefix('-')
            .map_or((false, numerator), |n| (true, n));
        let angle = V::new(
            K::Angle,
            R::new(
                magnitude.parse().expect("angle magnitude"),
                denominator.parse().expect("angle denominator"),
                negative,
            )
            .expect("admitted full-width angle"),
        )
        .expect("completed angle");
        for (actual, expected) in [(angle.checked_sin(), sine), (angle.checked_cos(), cosine)] {
            let result = actual.expect("certified rounded ratio");
            assert_eq!(result.kind(), K::Ratio);
            assert_eq!(
                result.rational(),
                R::from_signed_integer(expected.parse().expect("oracle ppm")),
                "{row}"
            );
        }
        rows += 1;
    }
    assert_eq!(rows, 44, "every retained authored case must run");
}

#[test]
#[allow(clippy::expect_used)]
fn retained_oracle_is_reproduced_without_native_folding_or_pi_input() {
    let root = std::env::current_dir()
        .expect("Cargo package working directory")
        .join("../..");
    let result = std::process::Command::new("python3")
        .args([
            "-I",
            "-B",
            "docs/tasks/artifacts/formula_structure/sincos_value_oracle.py",
        ])
        .current_dir(root)
        .output()
        .expect("independent direct-power/factorial producer");
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert_eq!(result.stdout, include_bytes!("fixtures/formula_sincos.tsv"));
}

#[test]
#[allow(clippy::expect_used)]
fn kind_refusals_preserve_actual_zero_operand_and_call() {
    for kind in [K::Length, K::Area, K::Ratio, K::Count] {
        let value = V::new(kind, R::ZERO).expect("actual numeric zero");
        for (result, builtin) in [(value.checked_sin(), F::Sin), (value.checked_cos(), F::Cos)] {
            let error = result.expect_err("only angle is admitted");
            assert_eq!(
                error,
                Error::BuiltinInputKind {
                    builtin,
                    argument: 0,
                    actual: kind
                }
            );
            assert_eq!(error.diagnostic_code(), "formula_dimension");
        }
    }
}
