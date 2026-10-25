//! Focused arc-length value boundary, independent of source-bearing recipe evaluation.
use sc_core::recipe::{
    FormulaBuiltin as F, FormulaKind as K, FormulaNumericError as Error, FormulaNumericValue as V,
    FormulaRational as R, FormulaRationalComponent as C, FormulaRationalError,
};

#[allow(clippy::expect_used)]
fn value(kind: K, numerator: &str, denominator: &str) -> V {
    let (negative, magnitude) = numerator
        .strip_prefix('-')
        .map_or((false, numerator), |n| (true, n));
    V::new(
        kind,
        R::new(
            magnitude.parse().expect("fixture magnitude"),
            denominator.parse().expect("fixture denominator"),
            negative,
        )
        .expect("fixture rational"),
    )
    .expect("admitted completed operand")
}

#[test]
#[allow(clippy::expect_used)]
fn arcs_match_independent_signed_turn_and_hard_midpoint_fixtures() {
    let mut rows = 0;
    for row in include_str!("fixtures/formula_arcs.tsv")
        .lines()
        .filter(|row| !row.starts_with('#'))
    {
        let cells: Vec<_> = row.split('\t').collect();
        assert_eq!(cells.len(), 5, "{row}");
        let angle = value(K::Angle, cells[0], cells[1]);
        let radius = value(K::Length, cells[2], cells[3]);
        let actual = angle.checked_arc_length(radius);
        if let Some(integer) = cells[4].strip_prefix("integer:") {
            let result = actual.expect("admitted rounded arc");
            assert_eq!(result.kind(), K::Length);
            assert_eq!(
                result.rational(),
                R::from_signed_integer(integer.parse().expect("expected integer")),
                "{row}"
            );
        } else if let Some(integer) = cells[4].strip_prefix("domain:") {
            let error = actual.expect_err("rounded scalar excess");
            assert_eq!(
                error,
                Error::ScalarDomain {
                    operation: "arc_length",
                    kind: K::Length,
                    value: R::from_signed_integer(integer.parse().expect("rounded witness")),
                    minimum: -1_000_000_000,
                    maximum: Some(1_000_000_000)
                },
                "{row}"
            );
            assert_eq!(error.diagnostic_code(), "formula_domain");
        } else {
            let bits = cells[4]
                .strip_prefix("width:")
                .expect("width fixture")
                .parse()
                .expect("measured bits");
            let error = actual.expect_err("rounded reduced-width excess");
            assert_eq!(
                error,
                Error::Rational(FormulaRationalError::RationalWidth {
                    component: C::Numerator,
                    measured_bits: bits
                }),
                "{row}"
            );
            assert_eq!(error.diagnostic_code(), "formula_domain");
        }
        rows += 1;
    }
    assert_eq!(rows, 23, "all authored arc cases must run");
}

#[test]
#[allow(clippy::expect_used)]
fn arc_kind_refusals_preserve_actual_argument_even_for_zero() {
    let angle = value(K::Angle, "0", "1");
    let radius = value(K::Length, "0", "1");
    for kind in [K::Length, K::Area, K::Angle, K::Ratio, K::Count] {
        let other = value(kind, "0", "1");
        if kind != K::Angle {
            let error = other
                .checked_arc_length(radius)
                .expect_err("first argument requires angle");
            assert_eq!(
                error,
                Error::BuiltinInputKind {
                    builtin: F::ArcLength,
                    argument: 0,
                    actual: kind
                }
            );
            assert_eq!(error.diagnostic_code(), "formula_dimension");
        }
        if kind != K::Length {
            let error = angle
                .checked_arc_length(other)
                .expect_err("second argument requires length");
            assert_eq!(
                error,
                Error::BuiltinInputKind {
                    builtin: F::ArcLength,
                    argument: 1,
                    actual: kind
                }
            );
        }
    }
}
