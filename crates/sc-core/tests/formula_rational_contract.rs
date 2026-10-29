//! Focused public value contract for exact formula arithmetic, separate from evaluation.
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

#[test]
#[allow(clippy::expect_used)]
fn signed_sums_reduce_wide_intermediates_before_limits() {
    let value = |numerator, denominator, negative| {
        R::new(numerator, denominator, negative).expect("admitted fixture value")
    };
    assert_eq!(
        value(2, 3, false).checked_sub(value(1, 2, false)),
        Ok(value(1, 6, false))
    );
    assert_eq!(
        value(1, 2, false).checked_sub(value(2, 3, false)),
        Ok(value(1, 6, true))
    );
    assert_eq!(
        value(2, 3, true).checked_add(value(1, 2, true)),
        Ok(value(7, 6, true))
    );
    assert_eq!(R::ZERO.checked_sub(R::ONE), Ok(R::from_signed_integer(-1)));
    assert_eq!(R::ZERO.checked_sub(R::ZERO), Ok(R::ZERO));
    let maximum = u128::MAX;
    let wide = value(maximum, maximum - 1, false);
    // The raw numerator needs 257 bits, but its reduced form fits and must remain admitted.
    assert_eq!(
        wide.checked_add(wide),
        Ok(value(maximum, (maximum - 1) / 2, false))
    );
    assert_eq!(
        wide.checked_add(value(maximum, maximum - 1, true)),
        Ok(R::ZERO)
    );
    assert_eq!(
        value(maximum - 1, maximum, false).checked_add(value(1, maximum, false)),
        Ok(R::ONE)
    );
    assert_eq!(wide.checked_sub(value(1, maximum - 1, false)), Ok(R::ONE));
    let failures = [
        (
            value(maximum, 1, false).checked_add(R::ONE),
            C::Numerator,
            129,
        ),
        (
            wide.checked_add(value(maximum - 1, maximum - 2, false)),
            C::Numerator,
            257,
        ),
        (
            value(1, maximum, false).checked_sub(value(1, maximum - 1, false)),
            C::Denominator,
            256,
        ),
    ];
    for (result, component, measured_bits) in failures {
        let error = result.expect_err("excessive fully reduced component");
        assert_eq!(
            error,
            FormulaRationalError::RationalWidth {
                component,
                measured_bits
            }
        );
        assert_eq!(error.diagnostic_code(), "formula_domain");
        assert_eq!(error.rational_bit_bound(), Some(128));
    }
}

#[test]
#[allow(clippy::expect_used)]
fn four_operators_match_independent_fraction_fixtures() {
    fn value(numerator: &str, denominator: &str) -> R {
        let (negative, magnitude) = numerator
            .strip_prefix('-')
            .map_or((false, numerator), |magnitude| (true, magnitude));
        R::new(
            magnitude.parse().expect("fixture magnitude"),
            denominator.parse().expect("fixture denominator"),
            negative,
        )
        .expect("admitted fixture input")
    }
    for row in include_str!("fixtures/formula_rationals.tsv")
        .lines()
        .filter(|row| !row.starts_with('#'))
    {
        let cells: Vec<_> = row.split('\t').collect();
        assert_eq!(cells.len(), 6, "{row}");
        let &[operation, left_n, left_d, right_n, right_d, expected] = cells.as_slice() else {
            unreachable!("asserted six-cell rational fixture")
        };
        let left = value(left_n, left_d);
        let right = value(right_n, right_d);
        let result = match operation {
            "add" => Some(left.checked_add(right)),
            "sub" => Some(left.checked_sub(right)),
            "mul" => Some(left.checked_mul(right)),
            "div" => Some(left.checked_div(right)),
            _ => None,
        }
        .expect("known fixture operation");
        let actual = match result {
            Ok(result) => format!(
                "value:{}{}/{}",
                if result.is_negative() { "-" } else { "" },
                result.numerator_magnitude(),
                result.denominator()
            ),
            Err(FormulaRationalError::ZeroDivisor) => "ZeroDivisor".to_owned(),
            Err(FormulaRationalError::RationalWidth {
                component,
                measured_bits,
            }) => format!("{component:?}:{measured_bits}"),
            Err(error) => format!("unexpected:{error}"),
        };
        assert_eq!(actual, expected, "{row}");
    }
}

#[test]
#[allow(clippy::expect_used)]
fn total_order_and_sign_reversal_preserve_full_width_and_zero() {
    use core::cmp::Ordering;
    let maximum = u128::MAX;
    let left = R::new(maximum - 1, maximum, false).expect("close fraction");
    let right = R::new(maximum - 2, maximum - 1, false).expect("close fraction");
    // Cross-products differ by exactly one at 256-bit width; floating point would lose the order.
    assert_eq!(left.cmp(&right), Ordering::Greater);
    assert_eq!(left.negated().cmp(&right.negated()), Ordering::Less);
    assert_eq!(left.partial_cmp(&right), Some(Ordering::Greater));
    assert_eq!(left.negated().negated(), left);
    assert_eq!(R::ZERO.negated(), R::ZERO);
    assert!(!R::ZERO.negated().is_negative());
    let huge = R::new(maximum, 1, false).expect("full magnitude");
    assert!(huge.negated() < R::ZERO);
    assert!(R::ZERO < huge);
    assert_eq!(
        R::new(6, 8, false)
            .expect("fraction")
            .cmp(&R::new(3, 4, false).expect("same value")),
        Ordering::Equal
    );
}
