//! Focused completed-value boundary contract; no evaluator or binding certificate.
use sc_core::recipe::{
    FormulaExpression, FormulaKind as K, FormulaNumericError as Error, FormulaNumericValue as V,
    FormulaRational as R,
};

#[test]
#[allow(clippy::expect_used)]
fn completed_values_check_exact_scalar_domains_without_binding_limits() {
    for kind in [K::Length, K::Area, K::Angle, K::Ratio, K::Count] {
        let value = V::new(kind, R::ZERO).expect("numeric zero");
        assert_eq!(value.kind(), kind);
        assert_eq!(value.rational(), R::ZERO);
    }
    for kind in [K::Boolean, K::Point, K::Edge] {
        let error = V::new(kind, R::ONE).expect_err("nonnumeric kind");
        assert_eq!(error, Error::NonNumericKind { kind });
        assert_eq!(error.diagnostic_code(), "formula_dimension");
    }
    for (kind, limit) in [
        (K::Length, 1_000_000_000_i64),
        (K::Area, 1_000_000_000_000_000_000),
    ] {
        for negative in [false, true] {
            let magnitude = u128::from(limit.unsigned_abs());
            let boundary = R::new(magnitude, 1, negative).expect("boundary rational");
            assert_eq!(
                V::new(kind, boundary)
                    .expect("inclusive boundary")
                    .rational(),
                boundary
            );
            let inside = R::new(magnitude * 10 - 1, 10, negative).expect("inside fraction");
            assert!(V::new(kind, inside).is_ok());
            let outside = R::new(magnitude * 10 + 1, 10, negative).expect("outside fraction");
            let error = V::new(kind, outside).expect_err("exact excess before rounding");
            assert_eq!(
                error,
                Error::ScalarDomain {
                    operation: "numeric_value",
                    kind,
                    value: outside,
                    minimum: -limit,
                    maximum: Some(limit),
                }
            );
            assert_eq!(error.diagnostic_code(), "formula_domain");
        }
    }
    let fractional = R::new(1, 3, false).expect("fractional count");
    assert_eq!(
        V::new(K::Count, fractional)
            .expect("nonnegative fraction")
            .rational(),
        fractional
    );
    let negative = R::new(1, 3, true).expect("negative fraction");
    assert_eq!(
        V::new(K::Count, negative),
        Err(Error::ScalarDomain {
            operation: "numeric_value",
            kind: K::Count,
            value: negative,
            minimum: 0,
            maximum: None,
        })
    );
    for kind in [K::Angle, K::Ratio, K::Count] {
        let wide = R::new(u128::MAX, 1, false).expect("full-width exact temporary");
        assert_eq!(
            V::new(kind, wide)
                .expect("storage limit is later")
                .rational(),
            wide
        );
    }
    for kind in [K::Angle, K::Ratio] {
        let negative = R::new(u128::MAX, 1, true).expect("full negative magnitude");
        assert_eq!(
            V::new(kind, negative).expect("signed temporary").rational(),
            negative
        );
    }
}

#[test]
#[allow(clippy::expect_used)]
fn normalized_literals_keep_their_once_rounded_internal_integer() {
    let cases = [
        ("2", K::Count, 2),
        ("1.5", K::Ratio, 1_500_000),
        ("2 pct", K::Ratio, 20_000),
        ("0.0000004", K::Ratio, 0),
        ("0.0000005", K::Ratio, 1),
        ("0.4 um", K::Length, 0),
        ("0.5 um", K::Length, 1),
        ("720 deg", K::Angle, 720_000_000),
        (
            "340282366920938463463374607431768211455",
            K::Count,
            u128::MAX,
        ),
    ];
    for (source, kind, magnitude) in cases {
        let syntax = FormulaExpression::parse(source).expect("literal syntax");
        let literal = syntax
            .root()
            .normalized_literal()
            .expect("input conversion")
            .expect("literal root");
        let value = V::from_literal(literal).expect("admitted completed input");
        assert_eq!(value.kind(), kind, "{source}");
        assert_eq!(
            value.rational(),
            R::new(magnitude, 1, false).expect("expected integer"),
            "{source}"
        );
    }
}

#[test]
#[allow(clippy::expect_used)]
fn quantity_operators_scale_exactly_and_refuse_completed_excess() {
    use sc_core::recipe::{
        FormulaBinaryOperator as B, FormulaRationalComponent as C, FormulaRationalError,
    };
    let rational = |n, d, negative| R::new(n, d, negative).expect("fixture rational");
    let value = |kind, n, d| V::new(kind, rational(n, d, false)).expect("admitted fixture");
    let length = value(K::Length, 1, 3);
    let ratio = value(K::Ratio, 500_000, 1);
    let count = value(K::Count, 3, 1);
    let cases = [
        (length.checked_add(length), K::Length, rational(2, 3, false)),
        (
            length.checked_sub(value(K::Length, 1, 2)),
            K::Length,
            rational(1, 6, true),
        ),
        (length.checked_mul(ratio), K::Length, rational(1, 6, false)),
        (ratio.checked_mul(length), K::Length, rational(1, 6, false)),
        (length.checked_div(ratio), K::Length, rational(2, 3, false)),
        (length.checked_mul(length), K::Area, rational(1, 9, false)),
        (
            value(K::Area, 6, 1).checked_div(value(K::Length, 4, 1)),
            K::Length,
            rational(3, 2, false),
        ),
        (
            length.checked_div(value(K::Length, 1, 1)),
            K::Ratio,
            rational(1_000_000, 3, false),
        ),
        (
            count.checked_div(value(K::Count, 2, 1)),
            K::Ratio,
            rational(1_500_000, 1, false),
        ),
        (
            count.checked_div(value(K::Ratio, 2_000_000, 1)),
            K::Count,
            rational(3, 2, false),
        ),
        (
            ratio.checked_mul(count),
            K::Ratio,
            rational(1_500_000, 1, false),
        ),
        (
            ratio.checked_mul(ratio),
            K::Ratio,
            rational(250_000, 1, false),
        ),
        (
            value(K::Angle, 720_000_000, 1).checked_div(value(K::Angle, 360_000_000, 1)),
            K::Ratio,
            rational(2_000_000, 1, false),
        ),
    ];
    for (actual, kind, expected) in cases {
        let actual = actual.expect("exact quantity operation");
        assert_eq!((actual.kind(), actual.rational()), (kind, expected));
    }
    let maximum = u128::MAX;
    let wide = value(K::Ratio, maximum, 1);
    // The unscaled product is 129 bits; the complete ppm-scaled result is admitted.
    assert_eq!(
        wide.checked_mul(value(K::Ratio, 2, 1))
            .expect("scaled cancellation")
            .rational(),
        rational(maximum / 5, 100_000, false)
    );
    assert_eq!(
        wide.checked_mul(value(K::Count, 2, 1)),
        Err(Error::Rational(FormulaRationalError::RationalWidth {
            component: C::Numerator,
            measured_bits: 129,
        }))
    );
    assert_eq!(
        value(K::Ratio, 1, maximum).checked_mul(value(K::Ratio, 1, 1)),
        Err(Error::Rational(FormulaRationalError::RationalWidth {
            component: C::Denominator,
            measured_bits: 148,
        }))
    );
    let zero = value(K::Length, 0, 1);
    assert_eq!(
        zero.checked_div(zero),
        Err(Error::Rational(FormulaRationalError::ZeroDivisor))
    );
    let dimension = value(K::Angle, 0, 1)
        .checked_mul(zero)
        .expect_err("angle times length");
    assert_eq!(
        dimension,
        Error::BinaryDimension {
            operator: B::Multiply,
            left: K::Angle,
            right: K::Length
        }
    );
    assert_eq!(dimension.diagnostic_code(), "formula_dimension");
    assert!(dimension.arc_length_hint());
    assert!(dimension.to_string().contains("arc_length"));
    assert!(matches!(
        length.checked_add(ratio),
        Err(Error::BinaryDimension {
            operator: B::Add,
            ..
        })
    ));
    assert_eq!(
        value(K::Count, 1, 1).checked_sub(value(K::Count, 2, 1)),
        Err(Error::ScalarDomain {
            operation: "-",
            kind: K::Count,
            value: rational(1, 1, true),
            minimum: 0,
            maximum: None,
        })
    );
    let boundary = value(K::Length, 1_000_000_000, 1);
    assert_eq!(
        boundary.checked_add(value(K::Length, 1, 10)),
        Err(Error::ScalarDomain {
            operation: "+",
            kind: K::Length,
            value: rational(10_000_000_001, 10, false),
            minimum: -1_000_000_000,
            maximum: Some(1_000_000_000),
        })
    );
}
