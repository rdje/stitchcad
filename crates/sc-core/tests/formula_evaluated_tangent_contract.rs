//! Actual supplied-value tangent calls,without manually attaching a value or call-source flag.
#![allow(clippy::expect_used)]
use sc_core::{
    name::MachineToken,
    recipe::{
        FormulaBinaryOperator as Op, FormulaBuiltin as B, FormulaEvaluatedScalar as V,
        FormulaKind as K, FormulaNumericError as NumericError, FormulaNumericValue as N,
        FormulaRational as R, FormulaTangentError as Error, FormulaTangentLimits as Limits,
        FormulaTangentResource as Unit, FormulaToleranceName as T,
    },
};
use std::error::Error as _;

fn value(kind: K, integer: i128) -> V {
    V::from_numeric(N::new(kind, R::from_signed_integer(integer)).expect("actual numeric"))
}
fn names(value: V) -> Vec<B> {
    value.sources().iter().collect()
}
fn sourced_angle(n: u128, d: u128, negative: bool) -> V {
    let input = V::from_numeric(
        N::new(K::Angle, R::new(n, d, negative).expect("original rational")).expect("Angle"),
    );
    let actual_one = value(K::Angle, 0)
        .checked_cos()
        .expect("actual cos0 unit Ratio");
    input
        .checked_binary(Op::Multiply, actual_one)
        .expect("exact Angle*Ratio,real cos contribution")
}

#[test]
fn zero_and_unit_calls_keep_real_input_and_tan_through_false_storage_and_class_guards() {
    let name = MachineToken::new("tan_result").expect("actual binding name");
    for microdegrees in [0, 45_000_000, -45_000_000, 180_000_000] {
        let original = value(K::Angle, microdegrees)
            .checked_round_to(value(K::Angle, 1))
            .expect("actual upstream call");
        assert_eq!(names(original), [B::RoundTo]);
        let result = original
            .checked_tan(Limits::new(156, 0))
            .expect("exact Tan");
        assert_eq!(result.input(), original);
        assert_eq!(
            result.execution().plan().input(),
            original.numeric().expect("Angle")
        );
        assert_eq!(result.execution().plan().limits(), Limits::new(156, 0));
        assert_eq!(result.execution().charged_series_terms(), 0);
        let output = result.value();
        assert_eq!(names(output), [B::RoundTo, B::Tan]);
        let zero = output
            .checked_binary(Op::Subtract, output)
            .expect("actual cancellation");
        assert_eq!(zero.numeric().expect("zero Ratio").rational(), R::ZERO);
        let false_value = zero
            .checked_binary(Op::NotEqual, value(K::Ratio, 0))
            .expect("actual false comparison");
        assert_eq!(false_value.boolean(), Some(false));
        let read = false_value
            .bind(&name)
            .expect("actual Boolean storage")
            .read()
            .expect("actual read");
        assert_eq!(read.boolean(), Some(false));
        assert_eq!(names(read), [B::RoundTo, B::Tan]);
        let refusal = read
            .sources()
            .check_tolerance(T::Numerical)
            .expect_err("T1 sources refuse");
        assert_eq!(refusal.sources(), read.sources());
        read.sources()
            .check_tolerance(T::Geometric)
            .expect("T2 source guard admits");
    }
}

#[test]
fn actual_native_result_automatically_unions_cosine_and_tangent_without_source_seeding() {
    let original = sourced_angle(30_000_000, 1, true);
    assert_eq!(names(original), [B::Cos]);
    let limits = Limits::new(2073, 512);
    let result = original
        .checked_tan(limits)
        .expect("certified negative tan30");
    assert_eq!(result.input(), original);
    assert_eq!(
        result.value().numeric().expect("Ratio").rational(),
        R::from_signed_integer(-577350)
    );
    assert_eq!(names(result.value()), [B::Cos, B::Tan]);
    assert_eq!(result.execution().plan().limits(), limits);
    assert_eq!(result.execution().precision(), Some(128));
    assert_eq!(result.execution().charged_series_terms(), 512);
}

#[test]
fn wrong_kind_retains_false_and_prior_successful_tan_before_resource_admission() {
    let cosine = value(K::Angle, 0).checked_cos().expect("actual cos unit");
    let false_value = cosine
        .checked_binary(Op::NotEqual, value(K::Ratio, 1_000_000))
        .expect("actual false");
    let prior_tan = value(K::Angle, 0)
        .checked_tan(Limits::new(156, 0))
        .expect("actual successful Tan")
        .value();
    for original in [false_value, prior_tan] {
        let limits = Limits::new(0, 0);
        let error = original
            .checked_tan(limits)
            .expect_err("kind before empty quotas");
        assert_eq!(error.input(), original);
        assert_eq!(error.limits(), limits);
        assert_eq!(error.builtin(), B::Tan);
        assert_eq!(error.diagnostic_code(), Some("formula_dimension"));
        assert_eq!(
            error.preparation_error(),
            Some(Error::Numeric(NumericError::BuiltinInputKind {
                builtin: B::Tan,
                argument: 0,
                actual: original.kind(),
            }))
        );
        assert!(error.execution_error().is_none());
        assert_eq!(error.input().sources(), original.sources());
        assert!(error
            .source()
            .expect("original source")
            .downcast_ref::<Error>()
            .is_some());
    }
    assert_eq!(false_value.boolean(), Some(false));
    assert_eq!(names(false_value), [B::Cos]);
    assert_eq!(names(prior_tan), [B::Tan]);
}

#[test]
fn original_sources_limits_and_pole_or_fold_refusal_survive_preparation_failure() {
    let input = value(K::Angle, -270_000_000)
        .checked_round_to(value(K::Angle, 1))
        .expect("actual round_to");
    for limits in [Limits::new(155, 0), Limits::new(156, 0)] {
        let error = input
            .checked_tan(limits)
            .expect_err("fold refusal or actual pole");
        assert_eq!(error.input(), input);
        assert_eq!(error.limits(), limits);
        assert_eq!(names(error.input()), [B::RoundTo]);
        assert_eq!(error.builtin(), B::Tan);
        assert!(error.execution_error().is_none());
        match error.preparation_error().expect("original prep cause") {
            Error::Resources(resource) => {
                assert_eq!(resource.input(), input.numeric().expect("Angle").rational());
                assert_eq!(resource.limits(), limits);
                assert_eq!(resource.unit(), Unit::IntegerBits);
                assert_eq!(resource.requested(), 156);
                assert_eq!(error.diagnostic_code(), None);
            }
            Error::Pole { input: original } => {
                assert_eq!(original, input.numeric().expect("Angle").rational());
                assert_eq!(error.diagnostic_code(), Some("formula_domain"));
            }
            Error::Numeric(_) => unreachable!(),
        }
    }
}

#[test]
fn original_sourced_operand_and_native_progress_survive_late_resource_and_width_failure() {
    let near = sourced_angle(
        247385123034593398372138492992328993860,
        8635369834810561370044731507908276509,
        false,
    );
    let d = u128::MAX / 90_000_000;
    let wide = sourced_angle(90_000_000 * d - 1, d, true);
    for (input, limits, code, precision, terms) in [
        (near, Limits::new(4122, 3584), None, 512, 1536),
        (
            wide,
            Limits::new(4122, 1536),
            Some("formula_domain"),
            256,
            1536,
        ),
    ] {
        let error = input
            .checked_tan(limits)
            .expect_err("original native failure");
        assert_eq!(error.input(), input);
        assert_eq!(error.limits(), limits);
        assert_eq!(names(error.input()), [B::Cos]);
        assert_eq!(error.builtin(), B::Tan);
        assert_eq!(error.diagnostic_code(), code);
        assert!(error.preparation_error().is_none());
        let original = error.execution_error().expect("actual native error");
        assert_eq!(
            original.plan().input(),
            input.numeric().expect("original Angle")
        );
        assert_eq!(original.plan().limits(), limits);
        assert_eq!(original.precision(), Some(precision));
        assert_eq!(original.charged_series_terms(), terms);
        assert!(core::ptr::eq(
            error
                .source()
                .expect("original chain")
                .downcast_ref()
                .expect("typed source"),
            original
        ));
    }
}
