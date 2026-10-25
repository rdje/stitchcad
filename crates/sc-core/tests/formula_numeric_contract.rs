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
