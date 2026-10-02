//! D89: public length operators preserve the constructor invariant without panics or saturation.
use sc_units::{Length, UnitError, MAX_LENGTH_UM};

// Allows this public-interface contract to reproduce the predecessor's infallible operators.
// A successful legacy Length is observed as Ok, so leaving the domain cannot mimic a refusal.
trait Observation {
    fn observation(self) -> Result<Length, UnitError>;
}
impl Observation for Length {
    fn observation(self) -> Result<Length, UnitError> {
        Ok(self)
    }
}
impl Observation for Result<Length, UnitError> {
    fn observation(self) -> Result<Length, UnitError> {
        self
    }
}
#[allow(clippy::expect_used)] // An unwind is an assertion failure, never a fallback value.
fn add(a: Length, b: Length) -> Result<Length, UnitError> {
    std::panic::catch_unwind(|| (a + b).observation())
        .expect("assertion: length addition must not unwind")
}
#[allow(clippy::expect_used)] // An unwind is an assertion failure, never a fallback value.
fn sub(a: Length, b: Length) -> Result<Length, UnitError> {
    std::panic::catch_unwind(|| (a - b).observation())
        .expect("assertion: length subtraction must not unwind")
}
#[allow(clippy::expect_used)] // The curated fixture values are all inside the constructor's domain.
fn length(value: i64) -> Length {
    Length::from_micrometres(value).expect("fixture length is inside the declared domain")
}
fn expected(value: i128, operation: &'static str) -> Result<i128, UnitError> {
    if value.abs() > i128::from(MAX_LENGTH_UM) {
        Err(UnitError::DomainExceeded {
            operation,
            kind: "length",
            value,
            limit: i128::from(MAX_LENGTH_UM),
        })
    } else {
        Ok(value)
    }
}
#[test]
fn signed_addition_crossings_refuse_instead_of_constructing_invalid_lengths() {
    for (a, b) in [
        (MAX_LENGTH_UM, 1),
        (MAX_LENGTH_UM, MAX_LENGTH_UM),
        (-MAX_LENGTH_UM, -1),
        (-MAX_LENGTH_UM, -MAX_LENGTH_UM),
    ] {
        assert_eq!(
            add(length(a), length(b)).map(|value| i128::from(value.as_micrometres())),
            expected(i128::from(a) + i128::from(b), "Length::checked_add"),
            "add {a} {b}"
        );
    }
}
#[test]
fn signed_subtraction_crossings_refuse_instead_of_constructing_invalid_lengths() {
    for (a, b) in [
        (MAX_LENGTH_UM, -1),
        (MAX_LENGTH_UM, -MAX_LENGTH_UM),
        (-MAX_LENGTH_UM, 1),
        (-MAX_LENGTH_UM, MAX_LENGTH_UM),
    ] {
        assert_eq!(
            sub(length(a), length(b)).map(|value| i128::from(value.as_micrometres())),
            expected(i128::from(a) - i128::from(b), "Length::checked_sub"),
            "sub {a} {b}"
        );
    }
}
#[test]
fn inclusive_endpoints_cancellation_zero_and_ordinary_values_match_wide_integer_oracle() {
    let values = [
        -MAX_LENGTH_UM,
        -MAX_LENGTH_UM + 1,
        -10000,
        -1,
        0,
        1,
        10000,
        MAX_LENGTH_UM - 1,
        MAX_LENGTH_UM,
    ];
    for a in values {
        for b in values {
            let left = length(a);
            let right = length(b);
            assert_eq!(
                add(left, right).map(|value| i128::from(value.as_micrometres())),
                expected(i128::from(a) + i128::from(b), "Length::checked_add"),
                "add {a} {b}"
            );
            assert_eq!(
                sub(left, right).map(|value| i128::from(value.as_micrometres())),
                expected(i128::from(a) - i128::from(b), "Length::checked_sub"),
                "sub {a} {b}"
            );
            assert_eq!(
                add(left, right),
                left.checked_add(right),
                "checked add {a} {b}"
            );
            assert_eq!(
                sub(left, right),
                left.checked_sub(right),
                "checked sub {a} {b}"
            );
        }
    }
}

#[test]
fn operator_outputs_require_explicit_result_handling() {
    let sum: Result<Length, UnitError> = Length::ZERO + Length::ZERO;
    let difference: Result<Length, UnitError> = Length::ZERO - Length::ZERO;
    assert_eq!(sum, Ok(Length::ZERO));
    assert_eq!(difference, Ok(Length::ZERO));
}
