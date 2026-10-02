//! Full unsigned magnitude rounding; independent fixtures and the existing signed API bridge.
use sc_units::{
    round::{
        div_round_half_away_from_zero as signed, div_round_half_away_from_zero_unsigned as round,
    },
    UnitError,
};

const ZERO: UnitError = UnitError::DivisionByZero {
    operation: "div_round_half_away_from_zero_unsigned",
};

#[allow(clippy::expect_used)] // Contract: any unwind is a failed totality assertion.
fn total(n: u128, d: u128) -> Result<u128, UnitError> {
    std::panic::catch_unwind(|| round(n, d))
        .expect("assertion: unsigned rounding must never unwind")
}

#[test]
fn full_width_inputs_and_high_remainders_round_without_narrowing() {
    let high = 1_u128 << 127;
    for (n, d, expected) in [
        (u128::MAX, 1, u128::MAX),
        (high, 1, high),
        (u128::MAX, 2, high),
        (u128::MAX - 1, 2, high - 1),
        (u128::MAX, u128::MAX, 1),
        (u128::MAX - 1, u128::MAX, 1),
        (0, u128::MAX, 0),
    ] {
        assert_eq!(total(n, d), Ok(expected), "{n}/{d}");
    }
}

#[test]
fn even_and_odd_denominator_half_neighborhoods_are_distinct() {
    let even = u128::MAX - 1;
    let half = even / 2;
    for (n, d, expected) in [
        (half - 1, even, 0),
        (half, even, 1),
        (half + 1, even, 1),
        (half, u128::MAX, 0),
        (half + 1, u128::MAX, 1),
        (1, 2, 1),
        (3, 2, 2),
        (2, 5, 0),
        (3, 5, 1),
    ] {
        assert_eq!(total(n, d), Ok(expected), "{n}/{d}");
    }
}

#[test]
fn zero_denominator_retains_public_operation_without_panicking() {
    for n in [0, 1, 1_u128 << 127, u128::MAX] {
        assert_eq!(total(n, 0), Err(ZERO));
    }
    assert_eq!(
        signed(1, 0),
        Err(UnitError::DivisionByZero {
            operation: "div_round_half_away_from_zero"
        })
    );
}

#[test]
fn signed_bridge_preserves_sign_storage_and_negative_endpoint() {
    for (n, d, expected) in [
        (1, 2, 1),
        (-1, 2, -1),
        (1, -2, -1),
        (-1, -2, 1),
        (i128::from(i64::MIN), 1, i64::MIN),
        (-i128::from(i64::MIN), -1, i64::MIN),
    ] {
        assert_eq!(signed(n, d), Ok(expected));
        assert_eq!(
            total(n.unsigned_abs(), d.unsigned_abs()),
            Ok(u128::from(expected.unsigned_abs()))
        );
    }
    assert_eq!(
        signed(i128::MAX, 1),
        Err(UnitError::Overflow {
            operation: "div_round_half_away_from_zero"
        })
    );
}

#[test]
#[allow(clippy::expect_used)] // Authored fixture shape/data errors are contract failures.
fn independent_decimal_fixture_rows_agree() {
    let fixture =
        include_str!("../../../docs/tasks/artifacts/formula_structure/unsigned_round_cases.tsv");
    let mut count = 0;
    for row in fixture
        .lines()
        .filter(|row| !row.is_empty() && !row.starts_with('#'))
    {
        let cells: Vec<_> = row.split('\t').collect();
        let n = cells.first().expect("numerator").parse().expect("u128");
        let d = cells.get(1).expect("denominator").parse().expect("u128");
        let expected = match *cells.get(2).expect("verdict") {
            "zero" => Err(ZERO),
            value => Ok(value.parse().expect("u128")),
        };
        assert_eq!(total(n, d), expected, "{row}");
        count += 1;
    }
    assert_eq!(count, 138);
}
