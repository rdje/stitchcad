//! Actual tangent preparation only: no series evaluation or certified value is implied.
use sc_core::recipe::{
    FormulaBuiltin, FormulaKind, FormulaNumericError, FormulaNumericValue, FormulaRational,
    FormulaTangentError as Error, FormulaTangentLimits as Limits, FormulaTangentResource as Unit,
};
use std::error::Error as _;

#[allow(clippy::expect_used)]
fn angle(n: u128, d: u128, negative: bool) -> FormulaNumericValue {
    FormulaNumericValue::new(
        FormulaKind::Angle,
        FormulaRational::new(n, d, negative).expect("input"),
    )
    .expect("Angle")
}

#[test]
#[allow(clippy::expect_used)]
fn actual_kind_precedes_even_zero_resource_limits() {
    for kind in [
        FormulaKind::Length,
        FormulaKind::Area,
        FormulaKind::Ratio,
        FormulaKind::Count,
    ] {
        let input = FormulaNumericValue::new(kind, FormulaRational::ONE).expect("numeric input");
        let error = input
            .prepare_tangent(Limits::new(0, 0))
            .expect_err("wrong kind first");
        assert_eq!(
            error,
            Error::Numeric(FormulaNumericError::BuiltinInputKind {
                builtin: FormulaBuiltin::Tan,
                argument: 0,
                actual: kind,
            })
        );
        assert_eq!(error.diagnostic_code(), Some("formula_dimension"));
        assert!(error.source().is_some());
    }
}

#[test]
#[allow(clippy::expect_used)]
fn fold_reservation_precedes_zero_unit_and_pole_and_never_charges_series() {
    for (n, pole) in [
        (0, false),
        (45_000_000, false),
        (90_000_000, true),
        (135_000_000, false),
        (180_000_000, false),
        (270_000_000, true),
    ] {
        for negative in [false, true] {
            let input = angle(n, 1, negative);
            let limits = Limits::new(155, 0);
            let error = input
                .prepare_tangent(limits)
                .expect_err("fold does not fit");
            assert_eq!(error.diagnostic_code(), None);
            let Error::Resources(resource) = error else {
                unreachable!()
            };
            assert_eq!(resource.input(), input.rational());
            assert_eq!(resource.limits(), limits);
            assert_eq!(resource.unit(), Unit::IntegerBits);
            assert_eq!(resource.maximum(), 155);
            assert_eq!(resource.requested(), 156);
            assert_eq!(resource.precision(), None);
            assert_eq!(resource.reserved_integer_bits(), 0);
            assert_eq!(resource.charged_series_terms(), 0);
            let admitted = input.prepare_tangent(Limits::new(156, 0));
            if pole {
                let error = admitted.expect_err("actual odd quarter pole");
                assert_eq!(
                    error,
                    Error::Pole {
                        input: input.rational()
                    }
                );
                assert_eq!(error.diagnostic_code(), Some("formula_domain"));
                assert!(error.source().is_none());
            } else {
                let plan = admitted.expect("exact endpoint needs no approximation");
                assert_eq!(plan.input(), input);
                assert_eq!(plan.limits(), Limits::new(156, 0));
                assert_eq!(plan.initial_integer_bits(), 156);
                assert_eq!(plan.initial_series_terms(), 0);
            }
        }
    }
}

#[test]
#[allow(clippy::expect_used)]
fn complete_first_iteration_is_admitted_atomically_with_bit_priority() {
    let input = angle(30_000_000, 1, true);
    for (limits, unit, requested) in [
        (Limits::new(2072, 0), Unit::IntegerBits, 2073),
        (Limits::new(2073, 511), Unit::SeriesTerms, 512),
        (Limits::new(usize::MAX, 0), Unit::SeriesTerms, 512),
    ] {
        let error = input
            .prepare_tangent(limits)
            .expect_err("complete demand refused");
        assert_eq!(error.diagnostic_code(), None);
        assert!(error.source().is_some());
        let Error::Resources(resource) = error else {
            unreachable!()
        };
        assert_eq!(resource.input(), input.rational());
        assert_eq!(resource.limits(), limits);
        assert_eq!(resource.unit(), unit);
        assert_eq!(resource.requested(), requested);
        assert_eq!(resource.precision(), Some(128));
        assert_eq!(resource.reserved_integer_bits(), 156);
        assert_eq!(resource.charged_series_terms(), 0);
        assert_eq!(
            resource.maximum(),
            match unit {
                Unit::IntegerBits => limits.maximum_integer_bits(),
                Unit::SeriesTerms => limits.maximum_series_terms(),
            }
        );
    }
    let plan = input
        .prepare_tangent(Limits::new(2073, 512))
        .expect("both inclusive quotas fit");
    assert_eq!(plan.input(), input);
    assert_eq!(plan.initial_integer_bits(), 2073);
    assert_eq!(plan.initial_series_terms(), 512);
}

#[test]
#[allow(clippy::expect_used)]
fn original_full_magnitudes_and_complete_turns_survive_private_preparation() {
    for negative in [false, true] {
        for (n, d) in [
            (u128::MAX, 1),
            (1, u128::MAX),
            (u128::MAX, u128::MAX - 1),
            (
                247385123034593398372138492992328993860,
                8635369834810561370044731507908276509,
            ),
        ] {
            let input = angle(n, d, negative);
            let plan = input
                .prepare_tangent(Limits::new(2073, 512))
                .expect("initial reservation fits");
            assert_eq!(plan.input(), input);
            assert_eq!(plan.input().rational().numerator_magnitude(), n);
            assert_eq!(plan.input().rational().denominator(), d);
            assert_eq!(plan.input().rational().is_negative(), negative);
            assert_eq!(plan.initial_series_terms(), 512);
        }
        let turns = u128::MAX / 180_000_000;
        let input = angle(turns * 180_000_000, 1, negative);
        let plan = input
            .prepare_tangent(Limits::new(156, 0))
            .expect("full-width exact full turns");
        assert_eq!(plan.input(), input);
        assert_eq!(plan.initial_series_terms(), 0);
        let odd = ((u128::MAX / 90_000_000) - 1) | 1;
        let input = angle(odd * 90_000_000, 1, negative);
        assert_eq!(
            input
                .prepare_tangent(Limits::new(156, 0))
                .expect_err("wide exact pole"),
            Error::Pole {
                input: input.rational()
            }
        );
    }
}
