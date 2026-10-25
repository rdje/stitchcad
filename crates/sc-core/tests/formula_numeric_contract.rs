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

#[test]
#[allow(clippy::expect_used)]
fn unary_operators_and_all_comparisons_keep_exact_values_and_kinds() {
    use sc_core::recipe::{
        FormulaBinaryOperator as B, FormulaRationalComponent as C, FormulaRationalError,
        FormulaUnaryOperator as U,
    };
    let rational = |n, d, negative| R::new(n, d, negative).expect("fixture rational");
    let value =
        |kind, n, d, negative| V::new(kind, rational(n, d, negative)).expect("admitted fixture");
    let angle = value(K::Angle, 720_000_000, 1, false);
    assert_eq!(
        angle.checked_neg().expect("raw sweep sign").rational(),
        rational(720_000_000, 1, true)
    );
    assert_eq!(
        value(K::Length, 0, 1, false)
            .checked_neg()
            .expect("zero sign")
            .rational(),
        R::ZERO
    );
    for count in [value(K::Count, 0, 1, false), value(K::Count, 1, 3, false)] {
        assert_eq!(
            count.checked_neg(),
            Err(Error::UnaryDimension {
                operator: U::Negate,
                kind: K::Count
            })
        );
    }
    let squares = [
        (value(K::Length, 3, 2, true), K::Area, rational(9, 4, false)),
        (
            value(K::Ratio, 500_000, 1, false),
            K::Ratio,
            rational(250_000, 1, false),
        ),
        (
            value(K::Ratio, 1, 2, false),
            K::Ratio,
            rational(1, 4_000_000, false),
        ),
        (
            value(K::Count, 3, 2, false),
            K::Count,
            rational(9, 4, false),
        ),
        (
            value(K::Length, 1_000_000_000, 1, true),
            K::Area,
            rational(1_000_000_000_000_000_000, 1, false),
        ),
    ];
    for (input, kind, expected) in squares {
        let result = input.checked_square().expect("declared square signature");
        assert_eq!((result.kind(), result.rational()), (kind, expected));
    }
    for kind in [K::Angle, K::Area] {
        assert_eq!(
            value(kind, 1, 1, false).checked_square(),
            Err(Error::UnaryDimension {
                operator: U::Square,
                kind
            })
        );
    }
    assert_eq!(
        value(K::Count, u128::MAX, 1, false).checked_square(),
        Err(Error::Rational(FormulaRationalError::RationalWidth {
            component: C::Numerator,
            measured_bits: 256,
        }))
    );
    let smaller = value(K::Count, 1, 3, false);
    let greater = value(K::Count, 1, 2, false);
    for (operator, expected) in [
        (B::Equal, false),
        (B::NotEqual, true),
        (B::Less, true),
        (B::LessEqual, true),
        (B::Greater, false),
        (B::GreaterEqual, false),
    ] {
        assert_eq!(smaller.checked_compare(operator, greater), Ok(expected));
    }
    assert_eq!(
        smaller.checked_compare(B::Equal, value(K::Count, 2, 6, false)),
        Ok(true)
    );
    assert_eq!(
        angle.checked_compare(B::NotEqual, value(K::Angle, 0, 1, false)),
        Ok(true)
    );
    assert_eq!(
        smaller.checked_compare(B::Equal, value(K::Ratio, 1, 3, false)),
        Err(Error::BinaryDimension {
            operator: B::Equal,
            left: K::Count,
            right: K::Ratio,
        })
    );
    assert!(matches!(
        smaller.checked_compare(B::Add, greater),
        Err(Error::BinaryDimension {
            operator: B::Add,
            ..
        })
    ));
    let maximum = u128::MAX;
    assert_eq!(
        value(K::Ratio, maximum - 1, maximum, false)
            .checked_compare(B::Greater, value(K::Ratio, maximum - 2, maximum - 1, false)),
        Ok(true)
    );
}

#[test]
#[allow(clippy::expect_used)]
fn exact_numeric_builtins_keep_operands_and_typed_refusals() {
    use sc_core::recipe::FormulaBuiltin as F;
    let rational = |n, d, negative| R::new(n, d, negative).expect("fixture rational");
    let value =
        |kind, n, d, negative| V::new(kind, rational(n, d, negative)).expect("admitted fixture");
    for kind in [K::Length, K::Area, K::Angle, K::Ratio] {
        let negative = value(kind, 1, 3, true);
        assert_eq!(negative.abs(), value(kind, 1, 3, false));
    }
    let count = value(K::Count, 1, 3, false);
    assert_eq!(count.abs(), count);
    let zero = value(K::Ratio, 0, 1, false);
    assert_eq!(zero.abs(), zero);
    let full = value(K::Ratio, u128::MAX, 1, true);
    assert_eq!(full.abs(), value(K::Ratio, u128::MAX, 1, false));
    assert_eq!(V::checked_min(&[count]), Ok(count));
    assert_eq!(V::checked_max(&[count]), Ok(count));
    let low = value(K::Length, 1, 3, true);
    let middle = value(K::Length, 1, 4, false);
    let high = value(K::Length, 2, 3, false);
    assert_eq!(V::checked_min(&[middle, high, low]), Ok(low));
    assert_eq!(V::checked_max(&[middle, high, low]), Ok(high));
    for (builtin, actual) in [(F::Min, V::checked_min(&[])), (F::Max, V::checked_max(&[]))] {
        let error = actual.expect_err("one or more operands required");
        assert_eq!(error, Error::BuiltinArity { builtin, actual: 0 });
        assert_eq!(error.diagnostic_code(), "formula_parse");
    }
    assert_eq!(
        V::checked_min(&[low, count]),
        Err(Error::BuiltinDimension {
            builtin: F::Min,
            argument: 1,
            wanted: K::Length,
            actual: K::Count,
        })
    );
    assert_eq!(middle.checked_clamp(low, high), Ok(middle));
    assert_eq!(
        value(K::Length, 1, 1, true).checked_clamp(low, high),
        Ok(low)
    );
    assert_eq!(
        value(K::Length, 1, 1, false).checked_clamp(low, high),
        Ok(high)
    );
    assert_eq!(low.checked_clamp(low, low), Ok(low));
    let inverted = middle
        .checked_clamp(high, low)
        .expect_err("inverted exact bounds");
    assert_eq!(
        inverted,
        Error::InvalidClamp {
            kind: K::Length,
            low: high.rational(),
            high: low.rational()
        }
    );
    assert_eq!(inverted.diagnostic_code(), "formula_domain");
    let mixed = middle
        .checked_clamp(low, count)
        .expect_err("bound kind mismatch");
    assert_eq!(
        mixed,
        Error::BuiltinDimension {
            builtin: F::Clamp,
            argument: 2,
            wanted: K::Length,
            actual: K::Count
        }
    );
    assert_eq!(mixed.diagnostic_code(), "formula_dimension");
    let maximum = u128::MAX;
    let near = value(K::Ratio, maximum - 1, maximum, false);
    let lower = value(K::Ratio, maximum - 2, maximum - 1, false);
    assert_eq!(V::checked_min(&[near, lower]), Ok(lower));
    assert_eq!(V::checked_max(&[near, lower]), Ok(near));
}
