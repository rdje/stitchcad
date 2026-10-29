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
    assert_eq!(rows, 15, "the authored sqrt subset must be exercised");
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
    assert_eq!(rows, 10, "the authored hypot subset must be exercised");
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
