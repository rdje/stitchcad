//! Focused scalar storage boundary; no environment publication or recipe replay certificate.
use sc_core::{
    name::MachineToken,
    recipe::{
        FormulaBindingValueRule as Rule, FormulaBoundValue as Bound, FormulaExpression,
        FormulaKind as K, FormulaNumericValue as V, FormulaRational as R,
    },
};

#[test]
#[allow(clippy::expect_used)]
fn all_numeric_kinds_round_once_and_preserve_scalar_domains_and_raw_turns() {
    let name = MachineToken::new("bound").expect("binding name");
    for kind in [K::Length, K::Area, K::Angle, K::Ratio, K::Count] {
        for (negative, expected) in [(false, 1), (true, -1)] {
            if negative && kind == K::Count {
                continue;
            }
            let value =
                V::new(kind, R::new(1, 2, negative).expect("half")).expect("completed value");
            let stored = Bound::from_numeric(&name, value).expect("rounded binding");
            assert_eq!(stored.kind(), kind);
            assert_eq!(stored.numeric_integer(), Some(expected));
            assert_eq!(stored.boolean(), None);
        }
    }
    for (kind, limit) in [
        (K::Length, 1_000_000_000_i64),
        (K::Area, 1_000_000_000_000_000_000),
    ] {
        for integer in [-limit, 0, limit] {
            let value =
                V::new(kind, R::from_signed_integer(i128::from(integer))).expect("scalar boundary");
            assert_eq!(
                Bound::from_numeric(&name, value)
                    .expect("inclusive scalar boundary")
                    .numeric_integer(),
                Some(integer)
            );
        }
    }
    for integer in [-720_000_000_i64, 360_000_000, 720_000_000] {
        let value =
            V::new(K::Angle, R::from_signed_integer(i128::from(integer))).expect("raw sweep");
        assert_eq!(
            Bound::from_numeric(&name, value)
                .expect("raw angle binding")
                .numeric_integer(),
            Some(integer)
        );
    }
}

#[test]
#[allow(clippy::expect_used)]
fn full_signed_endpoints_and_rounded_refusals_retain_actual_binding_context() {
    let name = MachineToken::new("saved_angle").expect("actual binding name");
    let high = u128::from(i64::MAX.unsigned_abs());
    let low_magnitude = u128::from(i64::MIN.unsigned_abs());
    let cases = [
        (high, 1, false, Some(i64::MAX), high),
        (low_magnitude, 1, true, Some(i64::MIN), low_magnitude),
        (high * 100 + 49, 100, false, Some(i64::MAX), high),
        (high * 100 + 50, 100, false, None, high + 1),
        (high * 100 - 50, 100, false, Some(i64::MAX), high),
        (
            low_magnitude * 100 + 49,
            100,
            true,
            Some(i64::MIN),
            low_magnitude,
        ),
        (low_magnitude * 100 + 50, 100, true, None, low_magnitude + 1),
        (
            low_magnitude * 100 - 50,
            100,
            true,
            Some(i64::MIN),
            low_magnitude,
        ),
        (u128::MAX, 1, false, None, u128::MAX),
        (u128::MAX, 1, true, None, u128::MAX),
    ];
    for kind in [K::Angle, K::Ratio, K::Count] {
        for (n, d, negative, expected, rounded_magnitude) in cases {
            if negative && kind == K::Count {
                continue;
            }
            let value = V::new(kind, R::new(n, d, negative).expect("exact temporary"))
                .expect("completed value");
            let result = Bound::from_numeric(&name, value);
            if let Some(expected) = expected {
                assert_eq!(
                    result.expect("valid signed endpoint").numeric_integer(),
                    Some(expected)
                );
            } else {
                let error = result.expect_err("storage excess");
                assert_eq!(error.name(), &name);
                assert_eq!(error.kind(), kind);
                assert_eq!(error.diagnostic_code(), "formula_domain");
                assert_eq!(
                    error.rule(),
                    Rule::Storage {
                        minimum: if kind == K::Count { 0 } else { i64::MIN },
                        maximum: i64::MAX,
                        measured: R::new(rounded_magnitude, 1, negative)
                            .expect("exact rounded witness"),
                    }
                );
                assert!(error.to_string().contains("saved_angle"));
            }
        }
    }
    let syntax =
        FormulaExpression::parse("9223372036854.775808 deg").expect("positive literal child");
    let literal = syntax
        .root()
        .normalized_literal()
        .expect("input conversion")
        .expect("literal");
    let negative = V::from_literal(literal)
        .expect("wide positive child")
        .checked_neg()
        .expect("unary sign");
    assert_eq!(
        Bound::from_numeric(&name, negative)
            .expect("full negative endpoint")
            .numeric_integer(),
        Some(i64::MIN)
    );
}

#[test]
#[allow(clippy::expect_used)]
fn later_reads_use_returned_integers_and_boolean_stays_boolean() {
    let name = MachineToken::new("saved").expect("binding name");
    let half = V::new(K::Length, R::new(1, 2, false).expect("half")).expect("completed length");
    let stored = Bound::from_numeric(&name, half).expect("one binding round");
    let read = V::new(
        stored.kind(),
        R::from_signed_integer(i128::from(
            stored.numeric_integer().expect("stored integer"),
        )),
    )
    .expect("later stored read");
    assert_eq!(
        read.checked_add(read)
            .expect("subsequent arithmetic")
            .rational(),
        R::from_signed_integer(2)
    );
    let wide = V::new(
        K::Count,
        R::new(u128::from(i64::MAX.unsigned_abs()) + 1, 1, false).expect("wide count"),
    )
    .expect("unbound temporary");
    assert!(Bound::from_numeric(&name, wide).is_err());
    let one = V::new(K::Count, R::ONE).expect("one count");
    assert_eq!(
        Bound::from_numeric(&name, wide.checked_sub(one).expect("cancel before binding"))
            .expect("admitted binding")
            .numeric_integer(),
        Some(i64::MAX)
    );
    for value in [false, true] {
        let stored = Bound::from_boolean(value);
        assert_eq!(stored.kind(), K::Boolean);
        assert_eq!(stored.boolean(), Some(value));
        assert_eq!(stored.numeric_integer(), None);
    }
}
