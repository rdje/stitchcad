//! Real value operators and stored reads retain approximation sources without hidden fractions.
#![allow(clippy::expect_used)]
use sc_core::{
    name::MachineToken,
    recipe::{
        FormulaBinaryOperator as B, FormulaBindingValueRule, FormulaBuiltin as Call,
        FormulaEvaluatedScalar as V, FormulaKind as K, FormulaNumericError as Error,
        FormulaNumericValue as N, FormulaRational as R, FormulaToleranceName as T,
        FormulaUnaryOperator as U,
    },
};

fn numeric(kind: K, numerator: i128, denominator: u128) -> V {
    let value = R::new(
        numerator.unsigned_abs(),
        denominator,
        numerator.is_negative(),
    )
    .expect("fraction");
    V::from_numeric(N::new(kind, value).expect("admitted numeric value"))
}

fn names(value: V) -> Vec<&'static str> {
    value.sources().iter().map(Call::token).collect()
}

#[test]
fn constructors_preserve_actual_kinds_and_zero_false_presence() {
    for kind in [K::Length, K::Area, K::Angle, K::Ratio, K::Count] {
        let value = numeric(kind, 0, 1);
        assert_eq!(value.kind(), kind);
        assert_eq!(value.numeric().expect("numeric zero").rational(), R::ZERO);
        assert_eq!(value.boolean(), None);
        assert!(value.sources().is_empty());
    }
    for boolean in [false, true] {
        let value = V::from_boolean(boolean);
        assert_eq!(value.kind(), K::Boolean);
        assert_eq!(value.boolean(), Some(boolean));
        assert_eq!(value.numeric(), None);
        assert!(value.sources().is_empty());
    }
}

#[test]
fn cancelling_arithmetic_and_exact_boolean_comparisons_keep_all_executed_sources() {
    let left = numeric(K::Ratio, 1_000_000, 1).with_call_source(Call::Sin);
    let right = numeric(K::Ratio, 1_000_000, 1).with_call_source(Call::Sqrt);
    let zero = left
        .checked_binary(B::Subtract, right)
        .expect("exact cancellation");
    assert_eq!(zero.numeric().expect("numeric").rational(), R::ZERO);
    assert_eq!(names(zero), ["sin", "sqrt"]);
    let equal = zero
        .checked_binary(B::Equal, numeric(K::Ratio, 0, 1))
        .expect("bare exact compare");
    assert_eq!(equal.boolean(), Some(true));
    assert_eq!(names(equal), ["sin", "sqrt"]);
    assert!(equal.sources().check_tolerance(T::Numerical).is_err());
    equal
        .sources()
        .check_tolerance(T::Geometric)
        .expect("class guard only");
    assert_eq!(names(left), ["sin"]);
    assert_eq!(names(right), ["sqrt"]);
}

#[test]
fn every_numeric_operator_computes_without_adding_or_erasing_call_sources() {
    let a = numeric(K::Count, 3, 1).with_call_source(Call::RoundTo);
    let b = numeric(K::Count, 2, 1).with_call_source(Call::Hypot);
    for (operator, expected_kind, expected) in [
        (B::Add, K::Count, 5),
        (B::Subtract, K::Count, 1),
        (B::Multiply, K::Count, 6),
        (B::Divide, K::Ratio, 1_500_000),
    ] {
        let actual = a.checked_binary(operator, b).expect("admitted operator");
        assert_eq!(actual.kind(), expected_kind);
        assert_eq!(
            actual.numeric().expect("numeric").rational(),
            R::from_signed_integer(expected)
        );
        assert_eq!(names(actual), ["hypot", "round_to"]);
    }
    for (operator, expected) in [
        (B::Equal, false),
        (B::NotEqual, true),
        (B::Less, false),
        (B::LessEqual, false),
        (B::Greater, true),
        (B::GreaterEqual, true),
    ] {
        let actual = a.checked_binary(operator, b).expect("comparison");
        assert_eq!(actual.boolean(), Some(expected));
        assert_eq!(names(actual), ["hypot", "round_to"]);
    }
    let length = numeric(K::Length, -3, 1).with_call_source(Call::ArcLength);
    for (operator, kind, answer) in [(U::Negate, K::Length, 3), (U::Square, K::Area, 9)] {
        let actual = length.checked_unary(operator).expect("unary");
        assert_eq!(actual.kind(), kind);
        assert_eq!(
            actual.numeric().expect("numeric").rational(),
            R::from_signed_integer(answer)
        );
        assert_eq!(names(actual), ["arc_length"]);
    }
    assert!(numeric(K::Count, 1, 1)
        .checked_binary(B::Add, numeric(K::Count, 2, 1))
        .expect("exact sum")
        .sources()
        .is_empty());
}

#[test]
fn numeric_and_boolean_bindings_read_once_rounded_values_with_original_sources() {
    let name = MachineToken::new("stored_result").expect("name");
    for (kind, numerator, denominator, stored) in [
        (K::Length, 3, 2, 2),
        (K::Area, -3, 2, -2),
        (K::Angle, -1_440_000_001, 2, -720_000_001),
        (K::Ratio, 3, 2, 2),
        (K::Count, 1, 3, 0),
    ] {
        let original = numeric(kind, numerator, denominator).with_call_source(Call::RoundTo);
        let binding = original.bind(&name).expect("store");
        assert_eq!(binding.value().numeric_integer(), Some(stored));
        assert_eq!(binding.sources(), original.sources());
        let read = binding.read().expect("stored read");
        assert_eq!(
            read.numeric().expect("numeric").rational(),
            R::from_signed_integer(i128::from(stored))
        );
        assert_eq!(read.kind(), kind);
        assert_eq!(read.sources(), original.sources());
        assert_ne!(read.numeric(), original.numeric());
        assert_eq!(read.bind(&name).expect("store read"), binding);
    }
    for boolean in [false, true] {
        let value = V::from_boolean(boolean).with_call_source(Call::Sin);
        let binding = value.bind(&name).expect("Boolean storage");
        assert_eq!(binding.value().boolean(), Some(boolean));
        assert_eq!(binding.value().numeric_integer(), None);
        assert_eq!(binding.read().expect("Boolean read"), value);
        assert!(binding
            .read()
            .expect("read")
            .sources()
            .check_tolerance(T::Numerical)
            .is_err());
    }
}

#[test]
fn actual_operator_refusals_and_storage_errors_publish_no_substitute_value() {
    let false_value = V::from_boolean(false).with_call_source(Call::Sin);
    for operator in [B::Add, B::Equal, B::Divide] {
        assert_eq!(
            false_value.checked_binary(operator, numeric(K::Count, 1, 1)),
            Err(Error::BinaryDimension {
                operator,
                left: K::Boolean,
                right: K::Count
            })
        );
    }
    assert_eq!(
        false_value.checked_unary(U::Negate),
        Err(Error::UnaryDimension {
            operator: U::Negate,
            kind: K::Boolean
        })
    );
    assert_eq!(
        numeric(K::Count, 0, 1).checked_unary(U::Negate),
        Err(Error::UnaryDimension {
            operator: U::Negate,
            kind: K::Count
        })
    );
    let divide = numeric(K::Count, 1, 1)
        .checked_binary(B::Divide, numeric(K::Count, 0, 1))
        .expect_err("division zero");
    assert_eq!(divide.diagnostic_code(), "formula_division");
    let name = MachineToken::new("too_wide").expect("name");
    let previous = numeric(K::Count, 1, 1).bind(&name).expect("prior binding");
    let excessive = numeric(K::Ratio, i128::from(i64::MAX) + 1, 1).with_call_source(Call::Cos);
    let error = excessive.bind(&name).expect_err("storage error");
    assert_eq!(error.name(), &name);
    assert_eq!(error.kind(), K::Ratio);
    assert_eq!(
        error.rule(),
        FormulaBindingValueRule::Storage {
            minimum: i64::MIN,
            maximum: i64::MAX,
            measured: R::from_signed_integer(i128::from(i64::MAX) + 1),
        }
    );
    assert_eq!(
        previous
            .read()
            .expect("prior unchanged")
            .numeric()
            .expect("numeric")
            .rational(),
        R::ONE
    );
    assert_eq!(names(excessive), ["cos"]);
}
