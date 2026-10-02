//! Public rounding is total over the complete i128 input domain, with exact signed i64 boundaries.
use sc_units::{round::div_round_half_away_from_zero as round, UnitError};
const OVERFLOW: UnitError = UnitError::Overflow {
    operation: "div_round_half_away_from_zero",
};
const ZERO: UnitError = UnitError::DivisionByZero {
    operation: "div_round_half_away_from_zero",
};
#[allow(clippy::expect_used)] // Contract helper: an unwind is an assertion failure, never a fallback value.
fn total(n: i128, d: i128) -> Result<i64, UnitError> {
    std::panic::catch_unwind(|| round(n, d)).expect("assertion: public rounding must never unwind")
}
#[test]
fn extreme_i128_magnitudes_refuse_with_typed_overflow_without_panicking() {
    for (n, d) in [
        (i128::MIN, 1),
        (i128::MIN, -1),
        (i128::MAX, 1),
        (i128::MAX, -1),
    ] {
        assert_eq!(total(n, d), Err(OVERFLOW), "{n}/{d}");
    }
}
#[test]
fn both_i64_endpoints_are_representable_with_either_denominator_sign() {
    let lo = i128::from(i64::MIN);
    let hi = i128::from(i64::MAX);
    for (n, d, expected) in [
        (lo, 1, i64::MIN),
        (hi, 1, i64::MAX),
        (-lo, -1, i64::MIN),
        (-hi, -1, i64::MAX),
        (lo * 2, 2, i64::MIN),
        (hi * 2, 2, i64::MAX),
        (lo * 2 + 1, 2, i64::MIN),
        (hi * 2 - 1, 2, i64::MAX),
    ] {
        assert_eq!(total(n, d), Ok(expected), "{n}/{d}");
    }
    for (n, d) in [
        (lo - 1, 1),
        (hi + 1, 1),
        (lo, -1),
        (lo * 2 - 1, 2),
        (hi * 2 + 1, 2),
    ] {
        assert_eq!(total(n, d), Err(OVERFLOW), "{n}/{d}");
    }
}
#[test]
fn extreme_denominators_zero_and_signs_preserve_half_away_rounding() {
    for (n, d, expected) in [
        (i128::MIN, i128::MIN, 1),
        (i128::MAX, i128::MIN, -1),
        (i128::MIN, i128::MAX, -1),
        (0, i128::MIN, 0),
        (1, i128::MIN, 0),
        (-1, i128::MIN, 0),
        (1, 2, 1),
        (-1, 2, -1),
        (1, -2, -1),
        (-1, -2, 1),
        (3, 2, 2),
        (-3, 2, -2),
        (2, 5, 0),
        (-2, 5, 0),
        (3, 5, 1),
        (-3, 5, -1),
    ] {
        assert_eq!(total(n, d), Ok(expected), "{n}/{d}");
    }
    for n in [0, 1, -1, i128::MIN, i128::MAX] {
        assert_eq!(total(n, 0), Err(ZERO));
    }
}
#[test]
fn explicit_fraction_oracle_rows_agree_at_wide_magnitudes() {
    let fixture = include_str!("../../../docs/tasks/artifacts/formula_structure/round_cases.tsv");
    let mut count = 0;
    for row in fixture
        .lines()
        .filter(|row| !row.is_empty() && !row.starts_with('#'))
    {
        let cells: Vec<_> = row.split('\t').collect();
        let n = cells.first().expect("numerator").parse().expect("i128");
        let d = cells.get(1).expect("denominator").parse().expect("i128");
        let expected = match *cells.get(2).expect("verdict") {
            "overflow" => Err(OVERFLOW),
            "zero" => Err(ZERO),
            value => Ok(value.parse().expect("i64")),
        };
        assert_eq!(total(n, d), expected, "{row}");
        count += 1;
    }
    assert_eq!(count, 36);
}
