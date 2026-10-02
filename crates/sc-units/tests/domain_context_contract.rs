//! D90/D92: typed domain context and truthful rendering at direct/forwarded public boundaries.
use core::fmt::Debug;
use sc_units::{Area, Length, Ratio, Unit, UnitError, MAX_AREA_UM2, MAX_LENGTH_UM};

#[allow(clippy::expect_used)] // A missing refusal fails the contract; it is never substituted.
fn domain<T: Debug>(
    result: Result<T, UnitError>,
    operation: &'static str,
    kind: &'static str,
    value: i128,
    limit: i128,
) {
    let error = result.expect_err("assertion: expected domain refusal");
    assert_eq!(
        error,
        UnitError::DomainExceeded {
            operation,
            kind,
            value,
            limit
        }
    );
    assert_eq!(
        error.to_string(),
        format!("{operation}: {kind} {value} is outside the declared domain (limit {limit})")
    );
}
#[allow(clippy::expect_used)] // Curated fixture operands are all valid constructor inputs.
fn length(value: i64) -> Length {
    Length::from_micrometres(value).expect("fixture length is inside its domain")
}
#[test]
fn direct_constructors_name_themselves_and_preserve_signed_domain_payloads() {
    for value in [MAX_LENGTH_UM + 1, -MAX_LENGTH_UM - 1] {
        domain(
            Length::from_micrometres(value),
            "Length::from_micrometres",
            "length",
            i128::from(value),
            i128::from(MAX_LENGTH_UM),
        );
    }
    for value in [i128::from(MAX_AREA_UM2) + 1, -i128::from(MAX_AREA_UM2) - 1] {
        domain(
            Area::from_square_micrometres(value),
            "Area::from_square_micrometres",
            "area",
            value,
            i128::from(MAX_AREA_UM2),
        );
    }
    for value in [MAX_LENGTH_UM, -MAX_LENGTH_UM] {
        assert_eq!(
            Length::from_micrometres(value).map(Length::as_micrometres),
            Ok(value)
        );
    }
    for value in [MAX_AREA_UM2, -MAX_AREA_UM2] {
        assert_eq!(
            Area::from_square_micrometres(i128::from(value)).map(Area::as_square_micrometres),
            Ok(value)
        );
    }
}
#[test]
fn forwarded_input_constructors_retain_the_calling_operation() {
    for (integer, float) in [
        (1_000_000_001, 1_000_000_001.0),
        (-1_000_000_001, -1_000_000_001.0),
    ] {
        domain(
            Length::from_rational(integer, 1, Unit::Micrometre),
            "Length::from_rational",
            "length",
            i128::from(integer),
            i128::from(MAX_LENGTH_UM),
        );
        domain(
            Length::from_f64_in(float, Unit::Micrometre),
            "Length::from_f64_in",
            "length",
            i128::from(integer),
            i128::from(MAX_LENGTH_UM),
        );
    }
}
#[test]
fn checked_arithmetic_and_public_operators_report_the_operation_that_produced_the_value() {
    for input in [MAX_LENGTH_UM, -MAX_LENGTH_UM] {
        let maximum = length(input);
        let value = i128::from(input) * 2;
        domain(
            maximum.checked_add(maximum),
            "Length::checked_add",
            "length",
            value,
            i128::from(MAX_LENGTH_UM),
        );
        domain(
            maximum + maximum,
            "Length::checked_add",
            "length",
            value,
            i128::from(MAX_LENGTH_UM),
        );
        domain(
            maximum.checked_sub(-maximum),
            "Length::checked_sub",
            "length",
            value,
            i128::from(MAX_LENGTH_UM),
        );
        domain(
            maximum - -maximum,
            "Length::checked_sub",
            "length",
            value,
            i128::from(MAX_LENGTH_UM),
        );
        domain(
            maximum.checked_mul_i64(2),
            "Length::checked_mul_i64",
            "length",
            value,
            i128::from(MAX_LENGTH_UM),
        );
        domain(
            Ratio::from_parts_per_million(2_000_000).scale(maximum),
            "Ratio::scale",
            "length",
            value,
            i128::from(MAX_LENGTH_UM),
        );
    }
    assert_eq!(
        length(1).checked_div_i64(2).map(Length::as_micrometres),
        Ok(1)
    );
    assert_eq!(
        length(3)
            .checked_area(length(4))
            .map(Area::as_square_micrometres),
        Ok(12)
    );
}
#[test]
fn generic_domain_rendering_does_not_invent_a_cause_or_upper_bound_relation() {
    for (kind, value, limit) in [
        ("positive range width", 0, 1),
        ("length", -1_000_000_001, 1_000_000_000),
    ] {
        let error = UnitError::DomainExceeded {
            operation: "contract::domain",
            kind,
            value,
            limit,
        };
        let message = error.to_string();
        assert!(
            !message.contains("unit-conversion bug"),
            "invented cause: {message}"
        );
        assert!(
            !message.contains("exceeds"),
            "false upper-bound relation: {message}"
        );
        assert!(message.starts_with("contract::domain:"));
        assert!(message.contains(&value.to_string()) && message.contains(&limit.to_string()));
    }
}
#[test]
fn non_domain_refusals_keep_their_existing_variant_and_operation() {
    assert_eq!(
        length(1).checked_div_i64(0),
        Err(UnitError::DivisionByZero {
            operation: "Length::checked_div_i64"
        })
    );
    assert_eq!(
        Length::from_rational(1, 0, Unit::Micrometre),
        Err(UnitError::DivisionByZero {
            operation: "Unit::to_micrometres"
        })
    );
    assert_eq!(
        Length::from_f64_in(f64::NAN, Unit::Micrometre),
        Err(UnitError::NonFinite {
            operation: "Length::from_f64_in"
        })
    );
    assert_eq!(
        length(MAX_LENGTH_UM).checked_mul_i64(i64::MAX),
        Err(UnitError::Overflow {
            operation: "Length::checked_mul_i64"
        })
    );
    assert_eq!(
        UnitError::DivisionByZero {
            operation: "divide"
        }
        .to_string(),
        "divide divided by zero"
    );
}
