//! Exact half comparison is sufficient for the shared magnitude rule, without a ratio width cap.
use core::cmp::Ordering;
use sc_units::{round::round_magnitude_from_half_comparison as round, UnitError};

const OVERFLOW: UnitError = UnitError::Overflow {
    operation: "round_magnitude_from_half_comparison",
};

#[allow(clippy::expect_used)] // Extract only after checking the public call's totality.
fn total(floor: u128, half: Ordering) -> Result<u128, UnitError> {
    let result = std::panic::catch_unwind(|| round(floor, half));
    assert!(
        result.is_ok(),
        "decomposed magnitude rounding must never unwind"
    );
    result.expect("public totality assertion already passed")
}

#[test]
fn exact_half_classes_and_full_width_floor_have_explicit_results() {
    let high = 1_u128 << 127;
    for (floor, half, expected) in [
        (0, Ordering::Less, Ok(0)),
        (0, Ordering::Equal, Ok(1)),
        (0, Ordering::Greater, Ok(1)),
        (high, Ordering::Less, Ok(high)),
        (high, Ordering::Equal, Ok(high + 1)),
        (high, Ordering::Greater, Ok(high + 1)),
        (u128::MAX - 1, Ordering::Equal, Ok(u128::MAX)),
        (u128::MAX, Ordering::Less, Ok(u128::MAX)),
        (u128::MAX, Ordering::Equal, Err(OVERFLOW)),
        (u128::MAX, Ordering::Greater, Err(OVERFLOW)),
    ] {
        assert_eq!(total(floor, half), expected, "{floor} {half:?}");
    }
}

#[test]
#[allow(clippy::expect_used)] // The independent frozen fixture is test data.
fn independent_decimal_results_also_hold_for_exact_decomposed_inputs() {
    let fixture =
        include_str!("../../../docs/tasks/artifacts/formula_structure/unsigned_round_cases.tsv");
    let mut count = 0;
    for row in fixture
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
    {
        let fields = row.split('\t').collect::<Vec<_>>();
        let numerator = fields
            .first()
            .expect("numerator")
            .parse::<u128>()
            .expect("u128");
        let denominator = fields
            .get(1)
            .expect("denominator")
            .parse::<u128>()
            .expect("u128");
        if denominator == 0 {
            // There is no floor/remainder for division by zero. The ratio entry points own that
            // refusal; supplying an invented comparison here would not validate the original input.
            assert_eq!(*fields.get(2).expect("verdict"), "zero");
            continue;
        }
        let expected = fields.get(2).expect("verdict").parse().expect("u128");
        let remainder = numerator % denominator;
        assert_eq!(
            total(
                numerator / denominator,
                remainder.cmp(&(denominator - remainder))
            ),
            Ok(expected),
            "{row}"
        );
        count += 1;
    }
    assert_eq!(count, 125);
}
