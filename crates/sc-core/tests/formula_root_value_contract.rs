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
        if cells[0] != "sqrt" {
            continue;
        }
        rows += 1;
        let kind = K::from_token(cells[1]).expect("fixture kind");
        let operand = value(kind, cells[2], cells[3]);
        let result = operand.checked_sqrt();
        if cells[7] == "negative" {
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
            let expected = cells[7]
                .strip_prefix("integer:")
                .expect("integer root fixture");
            assert_eq!(result.kind().token(), cells[6], "{row}");
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
