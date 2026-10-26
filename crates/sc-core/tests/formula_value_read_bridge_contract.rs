//! Canonical input and stored-value reads add no rounding, scaling, coercion or hidden fractions.
#![allow(clippy::expect_used)]
use sc_core::{
    name::MachineToken,
    recipe::{
        FormulaBoundValue as Bound, FormulaKind as K, FormulaNumericError as Error,
        FormulaNumericValue as V, FormulaRational as R,
    },
    value::ScalarInputValue as Input,
};
use sc_units::Length;

#[test]
fn canonical_numeric_inputs_keep_actual_kind_signed_endpoints_and_internal_units() {
    let mut cases = Vec::new();
    for integer in [-1_000_000_000, 0, 1_000_000_000] {
        cases.push((
            Input::from(Length::from_micrometres(integer).expect("valid Length")),
            K::Length,
            integer,
        ));
    }
    for integer in [i64::MIN, -720_000_000, -1, 0, 1, 720_000_000, i64::MAX] {
        cases.push((Input::from_angle_microdegrees(integer), K::Angle, integer));
        cases.push((Input::from_ratio_ppm(integer), K::Ratio, integer));
    }
    for integer in [0, 1, i64::MAX] {
        cases.push((
            Input::from_count(integer).expect("nonnegative canonical Count"),
            K::Count,
            integer,
        ));
    }
    for (input, kind, integer) in cases {
        let value = V::try_from(input).expect("numeric canonical scalar");
        assert_eq!(value.kind(), kind);
        assert_eq!(
            value.rational(),
            R::from_signed_integer(i128::from(integer))
        );
        assert_eq!(input.numeric_integer(), Some(integer));
    }
}

#[test]
fn boolean_inputs_and_bindings_refuse_numeric_conversion_even_for_false() {
    for boolean in [false, true] {
        for actual in [
            V::try_from(Input::from_boolean(boolean)),
            V::try_from(Bound::from_boolean(boolean)),
        ] {
            let error = actual.expect_err("Boolean is never numeric zero/one");
            assert_eq!(error, Error::NonNumericKind { kind: K::Boolean });
            assert_eq!(error.diagnostic_code(), "formula_dimension");
        }
        assert_eq!(Input::from_boolean(boolean).boolean(), Some(boolean));
        assert_eq!(Bound::from_boolean(boolean).boolean(), Some(boolean));
    }
}

#[test]
fn subsequent_reads_use_the_once_rounded_integer_for_every_numeric_kind() {
    let name = MachineToken::new("stored_value").expect("name");
    for (kind, negative, numerator, denominator, stored) in [
        (K::Length, false, 3, 2, 2),
        (K::Area, true, 3, 2, -2),
        (K::Angle, true, 1_440_000_001, 2, -720_000_001),
        (K::Ratio, false, 3, 2, 2),
        (K::Count, false, 1, 2, 1),
        (K::Count, false, 1, 3, 0),
    ] {
        let exact = V::new(
            kind,
            R::new(numerator, denominator, negative).expect("fraction"),
        )
        .expect("admitted exact value");
        let binding = Bound::from_numeric(&name, exact).expect("binding conversion");
        let read = V::try_from(binding).expect("numeric stored read");
        assert_eq!(binding.numeric_integer(), Some(stored));
        assert_eq!(read.kind(), kind);
        assert_eq!(read.rational(), R::from_signed_integer(i128::from(stored)));
        assert_ne!(
            read.rational(),
            exact.rational(),
            "pre-binding fraction is not a stored read"
        );
        assert_eq!(
            Bound::from_numeric(&name, read).expect("integer re-store"),
            binding
        );
    }
}

#[test]
fn stored_numeric_limits_round_trip_without_angle_modulo_or_i64_min_overflow() {
    let name = MachineToken::new("boundary_value").expect("name");
    for (kind, integers) in [
        (K::Length, [-1_000_000_000, 0, 1_000_000_000]),
        (
            K::Area,
            [-1_000_000_000_000_000_000, 0, 1_000_000_000_000_000_000],
        ),
        (K::Angle, [i64::MIN, -720_000_000, i64::MAX]),
        (K::Ratio, [i64::MIN, 0, i64::MAX]),
        (K::Count, [0, 1, i64::MAX]),
    ] {
        for integer in integers {
            let exact = V::new(kind, R::from_signed_integer(i128::from(integer)))
                .expect("admitted integer");
            let bound = Bound::from_numeric(&name, exact).expect("within storage");
            assert_eq!(V::try_from(bound).expect("exact read"), exact);
            assert_eq!(bound.numeric_integer(), Some(integer));
        }
    }
}

#[test]
fn input_read_and_binding_read_do_not_rescale_ratio_quantities() {
    let name = MachineToken::new("ratio_value").expect("name");
    let input = Input::from_ratio_ppm(1_500_000);
    let value = V::try_from(input).expect("raw ppm");
    assert_eq!(value.rational(), R::from_signed_integer(1_500_000));
    let bound = Bound::from_numeric(&name, value).expect("signed storage");
    assert_eq!(V::try_from(bound).expect("raw ppm read"), value);
    assert_eq!(value.kind(), K::Ratio);
}
