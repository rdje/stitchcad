//! Focused public construction contract for exact formula values, separate from evaluation.
use sc_core::recipe::{FormulaRational as R, FormulaRationalError};

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
