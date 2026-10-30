//! Focused correctly rounded root-value contracts; no recipe/source provenance certificate.
use sc_core::recipe::{
    FormulaBuiltin as F, FormulaKind as K, FormulaNumericError as Error, FormulaNumericValue as V,
    FormulaRational as R,
};

#[allow(clippy::expect_used)]
fn value(kind: K, numerator: &str, denominator: &str) -> V {
    let (negative, magnitude) = numerator
        .strip_prefix('-')
        .map_or((false, numerator), |n| (true, n));
    let rational = R::new(
        magnitude.parse().expect("fixture magnitude"),
        denominator.parse().expect("fixture denominator"),
        negative,
    )
    .expect("fixture rational");
    V::new(kind, rational).expect("completed fixture operand")
}

#[test]
#[allow(clippy::expect_used)]
fn sqrt_matches_independent_integer_root_and_exact_midpoint_fixtures() {
    let mut rows = 0;
    for row in include_str!("fixtures/formula_roots.tsv")
        .lines()
        .filter(|row| !row.starts_with('#'))
    {
        let cells: Vec<_> = row.split('\t').collect();
        assert_eq!(cells.len(), 8, "{row}");
        let &[operation, kind, numerator, denominator, _, _, expected_kind, witness] =
            cells.as_slice()
        else {
            unreachable!("asserted eight-cell root fixture")
        };
        if operation != "sqrt" {
            continue;
        }
        rows += 1;
        let kind = K::from_token(kind).expect("fixture kind");
        let operand = value(kind, numerator, denominator);
        let result = operand.checked_sqrt();
        if witness == "negative" {
            let error = result.expect_err("negative square root");
            assert_eq!(
                error,
                Error::NegativeSquareRoot {
                    kind,
                    value: operand.rational()
                }
            );
            assert_eq!(error.diagnostic_code(), "formula_domain");
        } else {
            let result = result.expect("admitted rounded root");
            let expected = witness
                .strip_prefix("integer:")
                .expect("integer root fixture");
            assert_eq!(result.kind().token(), expected_kind, "{row}");
            assert_eq!(
                result.rational(),
                R::new(expected.parse().expect("expected integer"), 1, false)
                    .expect("expected rational"),
                "{row}"
            );
        }
    }
    assert_eq!(rows, 51, "the authored sqrt subset must be exercised");
}

#[test]
#[allow(clippy::expect_used)]
fn sqrt_refuses_actual_undeclared_kinds_before_value_execution() {
    for kind in [K::Length, K::Angle, K::Count] {
        let error = value(kind, "0", "1")
            .checked_sqrt()
            .expect_err("absent sqrt signature, even zero");
        assert_eq!(
            error,
            Error::BuiltinInputKind {
                builtin: F::Sqrt,
                argument: 0,
                actual: kind
            }
        );
        assert_eq!(error.diagnostic_code(), "formula_dimension");
    }
}

#[test]
#[allow(clippy::expect_used)]
fn hypot_matches_independent_fixtures_without_an_intermediate_area_limit() {
    let mut rows = 0;
    for row in include_str!("fixtures/formula_roots.tsv")
        .lines()
        .filter(|row| !row.starts_with('#'))
    {
        let cells: Vec<_> = row.split('\t').collect();
        assert_eq!(cells.len(), 8, "{row}");
        let &[operation, _, left_n, left_d, right_n, right_d, expected_kind, witness] =
            cells.as_slice()
        else {
            unreachable!("asserted eight-cell root fixture")
        };
        if operation != "hypot" {
            continue;
        }
        rows += 1;
        let left = value(K::Length, left_n, left_d);
        let right = value(K::Length, right_n, right_d);
        let result = left.checked_hypot(right);
        if let Some(integer) = witness.strip_prefix("integer:") {
            let result = result.expect("admitted rounded length");
            assert_eq!(result.kind().token(), expected_kind);
            assert_eq!(
                result.rational(),
                R::new(integer.parse().expect("expected integer"), 1, false)
                    .expect("expected rational"),
                "{row}"
            );
        } else {
            let integer = witness
                .strip_prefix("domain:")
                .expect("exact rounded-domain witness");
            let error = result.expect_err("rounded length exceeds scalar limit");
            assert_eq!(
                error,
                Error::ScalarDomain {
                    operation: "hypot",
                    kind: K::Length,
                    value: R::new(integer.parse().expect("rounded integer"), 1, false)
                        .expect("witness rational"),
                    minimum: -1_000_000_000,
                    maximum: Some(1_000_000_000),
                }
            );
            assert_eq!(error.diagnostic_code(), "formula_domain");
        }
    }
    assert_eq!(rows, 34, "the authored hypot subset must be exercised");
}

#[test]
#[allow(clippy::expect_used)]
fn hypot_refuses_the_actual_first_wrong_kind_even_for_zero_values() {
    let length = value(K::Length, "0", "1");
    for kind in [K::Area, K::Angle, K::Ratio, K::Count] {
        let other = value(kind, "0", "1");
        for (argument, result) in [
            (0, other.checked_hypot(length)),
            (1, length.checked_hypot(other)),
            (0, other.checked_hypot(other)),
        ] {
            let error = result.expect_err("hypot requires two lengths");
            assert_eq!(
                error,
                Error::BuiltinInputKind {
                    builtin: F::Hypot,
                    argument,
                    actual: kind
                }
            );
            assert_eq!(error.diagnostic_code(), "formula_dimension");
        }
    }
}

#[test]
#[allow(clippy::expect_used)]
fn coupled_root_rows_retain_real_upstream_calls_through_success_storage_and_original_refusals() {
    use sc_core::{
        name::MachineToken,
        recipe::{
            FormulaBinaryOperator as B, FormulaBindingValueRule, FormulaEvaluatedScalar as S,
            FormulaToleranceName,
        },
    };
    let angle = S::from_numeric(V::new(K::Angle, R::ZERO).expect("actual zero Angle"));
    let cosine = angle.checked_cos().expect("actual upstream cos");
    let sine = angle.checked_sin().expect("actual upstream sin");
    let name = MachineToken::new("actual_root_binding").expect("actual binding name");
    let mut counts = (0, 0, 0);
    for (index, row) in include_str!("fixtures/formula_roots.tsv")
        .lines()
        .filter(|r| !r.starts_with('#'))
        .enumerate()
    {
        let row_number = index + 1;
        let fields: Vec<_> = row.split('\t').collect();
        let [operation, kind, left_n, left_d, right_n, right_d, expected_kind, witness] =
            fields.as_slice()
        else {
            unreachable!("original eight-column root fixture")
        };
        let kind = K::from_token(kind).expect("actual original input kind");
        let zero = S::from_numeric(V::new(kind, R::ZERO).expect("same-kind zero"));
        // These real upstream calls contribute through exact zero arithmetic;no flags are seeded.
        let left = S::from_numeric(value(kind, left_n, left_d))
            .checked_binary(
                B::Add,
                cosine
                    .checked_binary(B::Multiply, zero)
                    .expect("actual zero product"),
            )
            .expect("original full fraction plus actual sourced zero");
        let result = if *operation == "sqrt" {
            left.checked_sqrt()
        } else {
            let right = S::from_numeric(value(K::Length, right_n, right_d))
                .checked_binary(
                    B::Add,
                    sine.checked_binary(B::Multiply, zero)
                        .expect("actual right zero product"),
                )
                .expect("original right operand");
            left.checked_hypot(right)
        };
        counts.0 += 1;
        if let Some(integer) = witness.strip_prefix("integer:") {
            let expected: u128 = integer.parse().expect("independent nonnegative root");
            let result = result.expect("independent successful rooted value");
            assert_eq!(
                result.kind().token(),
                *expected_kind,
                "fixture-row={row_number};{row}"
            );
            assert_eq!(
                result.numeric().expect("numeric root").rational(),
                R::new(expected, 1, false).expect("independent integer"),
                "fixture-row={row_number};{row}"
            );
            assert!(result.sources().contains(F::Cos));
            assert!(result.sources().contains(if *operation == "sqrt" {
                F::Sqrt
            } else {
                F::Hypot
            }));
            assert_eq!(
                result.sources().iter().count(),
                if *operation == "sqrt" { 2 } else { 3 }
            );
            if *operation == "hypot" {
                assert!(result.sources().contains(F::Sin));
            }
            assert!(
                result
                    .sources()
                    .check_tolerance(FormulaToleranceName::Numerical)
                    .is_err(),
                "even exact/zero roots retain their actual call contribution"
            );
            if expected > i64::MAX as u128 {
                let error = result
                    .bind(&name)
                    .expect_err("only the original signed storage boundary refuses");
                assert!(matches!(
                    error.rule(),
                    FormulaBindingValueRule::Storage { .. }
                ));
                counts.2 += 1;
            } else {
                let binding = result.bind(&name).expect("actual once-only storage");
                let read = binding.read().expect("actual stored read");
                assert_eq!(
                    read, result,
                    "integer root storage preserves actual value and contributions"
                );
            }
        } else {
            let error = result.expect_err("original negative/final-domain refusal");
            let expected = if *witness == "negative" {
                Error::NegativeSquareRoot {
                    kind,
                    value: left.numeric().expect("actual negative operand").rational(),
                }
            } else {
                let integer = witness
                    .strip_prefix("domain:")
                    .expect("independent rounded domain witness");
                Error::ScalarDomain {
                    operation: "hypot",
                    kind: K::Length,
                    value: R::new(integer.parse().expect("independent integer"), 1, false)
                        .expect("actual witness"),
                    minimum: -1_000_000_000,
                    maximum: Some(1_000_000_000),
                }
            };
            assert_eq!(
                error, expected,
                "actual original typed refusal:fixture-row={row_number};{row}"
            );
            counts.1 += 1;
        }
    }
    assert_eq!(counts.0, 85, "entire original and coupled root population");
    assert_eq!(
        (counts.1, counts.2),
        (3, 5),
        "actual mathematical and later storage boundaries"
    );
    println!(
        "coupled roots:{} rows/{} original refusals/{} later storage refusals",
        counts.0, counts.1, counts.2
    );
}

#[test]
#[allow(clippy::expect_used)]
fn independent_isqrt_fraction_reference_regenerates_the_entire_root_fixture() {
    let root = std::env::current_dir()
        .expect("Cargo package root")
        .join("../..");
    let output = std::process::Command::new("python3")
        .args([
            "-I",
            "-B",
            "docs/tasks/artifacts/formula_structure/root_value_oracle.py",
        ])
        .current_dir(root)
        .output()
        .expect("pure standard-library root oracle");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        output.stdout,
        include_bytes!("fixtures/formula_roots.tsv"),
        "every original root input,kind,result and typed refusal witness"
    );
}
