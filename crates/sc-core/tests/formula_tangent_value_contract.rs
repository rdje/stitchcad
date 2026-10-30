//! Focused native worker checks;independent complete value/resource acceptance has a separate owner.
use sc_core::recipe::{
    FormulaKind, FormulaNumericError, FormulaNumericValue, FormulaRational,
    FormulaRationalComponent, FormulaRationalError, FormulaTangentError as Error,
    FormulaTangentLimits as Limits, FormulaTangentResource as Unit,
};
use std::error::Error as _;

#[allow(clippy::expect_used)]
fn angle(n: u128, d: u128, negative: bool) -> FormulaNumericValue {
    FormulaNumericValue::new(
        FormulaKind::Angle,
        FormulaRational::new(n, d, negative).expect("rational"),
    )
    .expect("actual Angle")
}

#[test]
#[allow(clippy::expect_used)]
fn exact_signed_endpoints_keep_original_plan_without_series_or_storage() {
    for degree in [
        -540_i128, -315, -225, -180, -135, -45, 0, 45, 135, 180, 225, 315, 540,
    ] {
        let input = FormulaNumericValue::new(
            FormulaKind::Angle,
            FormulaRational::from_signed_integer(degree * 1_000_000),
        )
        .expect("Angle");
        let expected = match degree.rem_euclid(180) {
            0 => 0,
            45 => 1_000_000,
            135 => -1_000_000,
            _ => unreachable!(),
        };
        let limits = Limits::new(156, 0);
        let result = input
            .prepare_tangent(limits)
            .expect("exact preparation")
            .evaluate()
            .expect("exact value");
        assert_eq!(result.plan().input(), input);
        assert_eq!(result.plan().limits(), limits);
        assert_eq!(result.value().kind(), FormulaKind::Ratio);
        assert_eq!(
            result.value().rational(),
            FormulaRational::from_signed_integer(expected)
        );
        assert_eq!(result.precision(), None);
        assert_eq!(result.reserved_integer_bits(), 156);
        assert_eq!(result.charged_series_terms(), 0);
    }
}

#[test]
#[allow(clippy::expect_used)]
fn unrounded_quotient_handles_reciprocal_signs_and_avoids_rounded_component_error() {
    for (degree, expected) in [
        (30, 577350),
        (60, 1732051),
        (120, -1732051),
        (150, -577350),
        (210, 577350),
        (240, 1732051),
        (-30, -577350),
        (-60, -1732051),
        (-120, 1732051),
        (-150, 577350),
    ] {
        let input = FormulaNumericValue::new(
            FormulaKind::Angle,
            FormulaRational::from_signed_integer(degree * 1_000_000),
        )
        .expect("Angle");
        for limits in [Limits::new(2073, 512), Limits::new(8219, 3584)] {
            let result = input
                .prepare_tangent(limits)
                .expect("prepared")
                .evaluate()
                .expect("whole certificate");
            assert_eq!(
                result.value().rational(),
                FormulaRational::from_signed_integer(expected)
            );
            assert_eq!(result.plan().input(), input);
            assert_eq!(result.plan().limits(), limits);
            assert_eq!(result.precision(), Some(128));
            assert_eq!(result.reserved_integer_bits(), 2073);
            assert_eq!(result.charged_series_terms(), 512);
        }
    }
    // tan30's expected integer follows exact sqrt(3) midpoint square bounds,not rounded sin/cos.
    let low = 2 * 577350_u128 - 1;
    let high = 2 * 577350_u128 + 1;
    assert!(
        3 * low * low < 4 * 1_000_000_u128.pow(2) && 4 * 1_000_000_u128.pow(2) < 3 * high * high
    );
}

#[test]
#[allow(clippy::expect_used)]
fn actual_near_midpoint_refines_or_preserves_next_demand_and_prior_charges() {
    let input = angle(
        247385123034593398372138492992328993860,
        8635369834810561370044731507908276509,
        false,
    );
    for (limits, unit, requested) in [
        (Limits::new(4122, 3584), Unit::IntegerBits, 8219),
        (Limits::new(8219, 1536), Unit::SeriesTerms, 2048),
        (Limits::new(4122, 1536), Unit::IntegerBits, 8219),
    ] {
        let error = input
            .prepare_tangent(limits)
            .expect("initial admission")
            .evaluate()
            .expect_err("next demand");
        assert_eq!(error.plan().input(), input);
        assert_eq!(error.plan().limits(), limits);
        assert_eq!(error.diagnostic_code(), None);
        assert!(error.source().is_some());
        assert_eq!(error.precision(), Some(512));
        assert_eq!(error.charged_series_terms(), 1536);
        assert_eq!(error.reserved_integer_bits(), 4122);
        let Error::Resources(resource) = error.cause() else {
            unreachable!()
        };
        assert_eq!(resource.input(), input.rational());
        assert_eq!(resource.limits(), limits);
        assert_eq!(resource.unit(), unit);
        assert_eq!(resource.requested(), requested);
        assert_eq!(resource.precision(), Some(512));
        assert_eq!(resource.charged_series_terms(), 1536);
        assert_eq!(resource.reserved_integer_bits(), 4122);
    }
    // Independent inverse/alternate-pi controls establish this actual input lies below half ppm.
    for limits in [Limits::new(8219, 3584), Limits::new(16412, 7680)] {
        let result = input
            .prepare_tangent(limits)
            .expect("prepared")
            .evaluate()
            .expect("certified at512");
        assert_eq!(result.value().rational(), FormulaRational::ZERO);
        assert_eq!(result.plan().input(), input);
        assert_eq!(result.plan().limits(), limits);
        assert_eq!(result.precision(), Some(512));
        assert_eq!(result.reserved_integer_bits(), 8219);
        assert_eq!(result.charged_series_terms(), 3584);
    }
}

#[test]
#[allow(clippy::expect_used)]
fn width_requires_exact_bit_count_and_never_becomes_a_pole_or_guessed_width() {
    let d = u128::MAX / 90_000_000;
    let n = 90_000_000 * d - 1;
    for negative in [false, true] {
        let input = angle(n, d, negative);
        let short = input
            .prepare_tangent(Limits::new(2073, 512))
            .expect("first iteration")
            .evaluate()
            .expect_err("width142..148 still needs its exact diagnostic");
        assert_eq!(short.diagnostic_code(), None);
        let Error::Resources(resource) = short.cause() else {
            unreachable!()
        };
        assert_eq!(resource.unit(), Unit::IntegerBits);
        assert_eq!(resource.requested(), 4122);
        assert_eq!(resource.precision(), Some(256));
        assert_eq!(resource.charged_series_terms(), 512);
        assert_eq!(resource.reserved_integer_bits(), 2073);
        let limits = Limits::new(4122, 1536);
        let error = input
            .prepare_tangent(limits)
            .expect("prepared")
            .evaluate()
            .expect_err("actual width");
        assert_eq!(error.plan().input(), input);
        assert_eq!(error.plan().limits(), limits);
        assert_eq!(error.diagnostic_code(), Some("formula_domain"));
        assert_eq!(error.precision(), Some(256));
        assert_eq!(error.charged_series_terms(), 1536);
        assert_eq!(error.reserved_integer_bits(), 4122);
        assert_eq!(
            error.cause(),
            Error::Numeric(FormulaNumericError::Rational(
                FormulaRationalError::RationalWidth {
                    component: FormulaRationalComponent::Numerator,
                    measured_bits: 148
                }
            ))
        );
    }
}

#[test]
#[allow(clippy::expect_used)]
fn full_width_rational_inputs_can_certify_zero_without_narrowing_or_binding() {
    for negative in [false, true] {
        for (n, d) in [(1, u128::MAX), (u128::MAX, u128::MAX - 1)] {
            let input = angle(n, d, negative);
            let result = input
                .prepare_tangent(Limits::new(2073, 512))
                .expect("full-width preparation")
                .evaluate()
                .expect("tiny mathematical value");
            assert_eq!(result.plan().input(), input);
            assert_eq!(result.value().rational(), FormulaRational::ZERO);
            assert_eq!(result.precision(), Some(128));
            assert_eq!(result.charged_series_terms(), 512);
        }
    }
}
