//! Focused public construction contract for exact formula values, separate from evaluation.
use sc_core::recipe::{FormulaRational as R, FormulaRationalComponent as C, FormulaRationalError};

#[test]
#[allow(clippy::expect_used)] // A refusal of an admitted fixture is a failed public contract.
fn canonical_values_preserve_sign_zero_and_full_width() {
    let cases = [
        ((6, 8, false), (3, 4, false)),
        ((6, 8, true), (3, 4, true)),
        ((0, u128::MAX, true), (0, 1, false)),
        ((u128::MAX, u128::MAX, true), (1, 1, true)),
        ((u128::MAX, 1, false), (u128::MAX, 1, false)),
        ((u128::MAX, 1, true), (u128::MAX, 1, true)),
        ((1, u128::MAX, true), (1, u128::MAX, true)),
        (
            (u128::MAX - 1, u128::MAX, false),
            (u128::MAX - 1, u128::MAX, false),
        ),
    ];
    for ((numerator, denominator, negative), expected) in cases {
        let value = R::new(numerator, denominator, negative).expect("admitted components");
        assert_eq!(
            (
                value.numerator_magnitude(),
                value.denominator(),
                value.is_negative(),
            ),
            expected
        );
        assert_eq!(value.is_zero(), numerator == 0);
    }
    assert_eq!(R::new(2, 2, false), Ok(R::ONE));
    assert_eq!(R::new(0, 7, true), Ok(R::ZERO));
    assert_eq!(R::default(), R::ZERO);
    for numerator in [0, 1, u128::MAX] {
        let error = R::new(numerator, 0, false).expect_err("zero denominator must refuse");
        assert_eq!(error, FormulaRationalError::ZeroDenominator);
        assert_eq!(error.diagnostic_code(), "formula_division");
    }
    let minimum = R::from_signed_integer(i128::MIN);
    assert!(minimum.is_negative());
    assert_eq!(minimum.numerator_magnitude(), 1_u128 << 127);
    assert_eq!(minimum.denominator(), 1);
    assert_eq!(R::from_signed_integer(0), R::ZERO);
    assert_eq!(R::from_signed_integer(1), R::ONE);
}

#[test]
#[allow(clippy::expect_used)]
fn products_and_quotients_cross_cancel_before_width_refusal() {
    let value = |numerator, denominator, negative| {
        R::new(numerator, denominator, negative).expect("admitted fixture value")
    };
    let huge = value(u128::MAX, 2, false);
    let inverse = value(2, u128::MAX, true);
    assert_eq!(huge.checked_mul(inverse), Ok(R::from_signed_integer(-1)));
    assert_eq!(huge.checked_div(huge), Ok(R::ONE));
    assert_eq!(
        value(2, 3, true).checked_mul(value(9, 4, true)),
        Ok(value(3, 2, false))
    );
    assert_eq!(
        value(2, 3, true).checked_div(value(4, 9, false)),
        Ok(value(3, 2, true))
    );
    assert_eq!(R::ZERO.checked_mul(huge), Ok(R::ZERO));
    assert_eq!(R::ZERO.checked_div(inverse), Ok(R::ZERO));
    assert_eq!(
        value(u128::MAX, 1, false).checked_mul(R::ONE),
        Ok(value(u128::MAX, 1, false))
    );
    let failures = [
        (
            value(u128::MAX, 1, false).checked_mul(value(2, 1, false)),
            C::Numerator,
            129,
        ),
        (
            value(u128::MAX, 1, false).checked_mul(value(u128::MAX, 1, false)),
            C::Numerator,
            256,
        ),
        (
            value(1, u128::MAX, false).checked_mul(value(1, 2, false)),
            C::Denominator,
            129,
        ),
        (
            value(u128::MAX, 1, false).checked_div(value(1, 2, false)),
            C::Numerator,
            129,
        ),
        (
            value(1, u128::MAX, false).checked_div(value(2, 1, false)),
            C::Denominator,
            129,
        ),
    ];
    for (result, component, measured_bits) in failures {
        let error = result.expect_err("fully reduced excessive component must refuse");
        assert_eq!(
            error,
            FormulaRationalError::RationalWidth {
                component,
                measured_bits
            }
        );
        assert_eq!(error.diagnostic_code(), "formula_domain");
        assert_eq!(error.rational_bit_bound(), Some(128));
        assert_eq!(error.limit_token(), Some("max_rational_bits"));
    }
    for numerator in [R::ZERO, R::ONE] {
        let error = numerator.checked_div(R::ZERO).expect_err("zero divisor");
        assert_eq!(error, FormulaRationalError::ZeroDivisor);
        assert_eq!(error.diagnostic_code(), "formula_division");
    }
}
