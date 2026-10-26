//! Actual implemented calls retain input and call sources, even when rounding changes no value.
#![allow(clippy::expect_used)]
use sc_core::{
    name::MachineToken,
    recipe::{
        FormulaBinaryOperator as Op, FormulaBuiltin as B, FormulaEvaluatedScalar as V,
        FormulaKind as K, FormulaNumericError as E, FormulaNumericValue as N, FormulaRational as R,
        FormulaToleranceName as T,
    },
};

fn value(kind: K, integer: i128) -> V {
    V::from_numeric(N::new(kind, R::from_signed_integer(integer)).expect("admitted numeric"))
}
fn names(value: V) -> Vec<&'static str> {
    value.sources().iter().map(B::token).collect()
}
fn assert_value(value: V, kind: K, integer: i128, sources: &[&str]) {
    assert_eq!(value.kind(), kind);
    assert_eq!(
        value.numeric().expect("numeric").rational(),
        R::from_signed_integer(integer)
    );
    assert_eq!(names(value), sources);
}

#[test]
fn implemented_calls_compute_actual_values_and_add_only_their_required_sources() {
    assert_value(
        value(K::Length, -3)
            .with_call_source(B::Sin)
            .checked_abs()
            .expect("abs"),
        K::Length,
        3,
        &["sin"],
    );
    assert_value(
        value(K::Area, 9).checked_sqrt().expect("sqrt"),
        K::Length,
        3,
        &["sqrt"],
    );
    assert_value(
        value(K::Ratio, 1_000_000)
            .checked_sqrt()
            .expect("ratio sqrt"),
        K::Ratio,
        1_000_000,
        &["sqrt"],
    );
    assert_value(
        value(K::Length, 3)
            .checked_hypot(value(K::Length, 4))
            .expect("hypot"),
        K::Length,
        5,
        &["hypot"],
    );
    assert_value(
        value(K::Angle, 720_000_000)
            .checked_arc_length(value(K::Length, 1))
            .expect("arc"),
        K::Length,
        13,
        &["arc_length"],
    );
    assert_value(
        value(K::Count, 4)
            .checked_round_to(value(K::Count, 2))
            .expect("unchanged quantization"),
        K::Count,
        4,
        &["round_to"],
    );
    assert_eq!(
        value(K::Length, 0).checked_sqrt(),
        Err(E::BuiltinInputKind {
            builtin: B::Sqrt,
            argument: 0,
            actual: K::Length
        })
    );
}

#[test]
fn every_evaluated_candidate_and_bound_contributes_even_when_unselected() {
    let low = value(K::Length, 1).with_call_source(B::Sin);
    let middle = value(K::Length, 3).with_call_source(B::Hypot);
    let high = value(K::Length, 5).with_call_source(B::Cos);
    let all = [low, middle, high];
    assert_value(
        V::checked_min(&all).expect("min"),
        K::Length,
        1,
        &["cos", "hypot", "sin"],
    );
    assert_value(
        V::checked_max(&all).expect("max"),
        K::Length,
        5,
        &["cos", "hypot", "sin"],
    );
    for (input, expected) in [(0, 1), (3, 3), (6, 5)] {
        assert_value(
            value(K::Length, input)
                .with_call_source(B::Dir)
                .checked_clamp(low, high)
                .expect("clamp"),
            K::Length,
            expected,
            &["cos", "dir", "sin"],
        );
    }
    let result = middle
        .checked_round_to(value(K::Length, 2).with_call_source(B::Atan))
        .expect("quantize");
    assert_value(result, K::Length, 4, &["atan", "hypot", "round_to"]);
    assert_value(
        low.checked_hypot(high).expect("root inputs"),
        K::Length,
        5,
        &["cos", "hypot", "sin"],
    );
    assert_value(
        value(K::Angle, 0)
            .with_call_source(B::Atan2)
            .checked_arc_length(high)
            .expect("zero arc"),
        K::Length,
        0,
        &["arc_length", "atan2", "cos"],
    );
    assert_value(low, K::Length, 1, &["sin"]);
}

#[test]
fn exact_valued_calls_and_cancellation_remain_inadmissible_at_t1_after_storage() {
    let name = MachineToken::new("exact_looking").expect("name");
    for computed in [
        value(K::Area, 0).checked_sqrt().expect("zero root"),
        value(K::Length, 3)
            .checked_hypot(value(K::Length, 4))
            .expect("integer root"),
        value(K::Length, 4)
            .checked_round_to(value(K::Length, 2))
            .expect("unchanged rounding"),
        value(K::Angle, 0)
            .checked_arc_length(value(K::Length, 1))
            .expect("zero arc"),
    ] {
        let cancelled = computed
            .checked_binary(Op::Subtract, computed)
            .expect("cancel");
        assert_eq!(cancelled.numeric().expect("numeric").rational(), R::ZERO);
        let stored = cancelled.bind(&name).expect("binding");
        let read = stored.read().expect("read");
        assert_eq!(read.sources(), computed.sources());
        let error = read
            .sources()
            .check_tolerance(T::Numerical)
            .expect_err("retained contribution");
        assert_eq!(error.sources(), computed.sources());
        read.sources()
            .check_tolerance(T::Geometric)
            .expect("symbolic class guard");
    }
    let exact = value(K::Length, -3).checked_abs().expect("exact abs");
    exact
        .sources()
        .check_tolerance(T::Numerical)
        .expect("no call contribution");
}

#[test]
fn actual_first_argument_and_numerical_refusals_are_preserved() {
    let boolean = V::from_boolean(false);
    assert_eq!(
        value(K::Area, 1).checked_hypot(boolean),
        Err(E::BuiltinInputKind {
            builtin: B::Hypot,
            argument: 0,
            actual: K::Area
        })
    );
    assert_eq!(
        value(K::Length, 1).checked_hypot(boolean),
        Err(E::BuiltinInputKind {
            builtin: B::Hypot,
            argument: 1,
            actual: K::Boolean
        })
    );
    assert_eq!(
        value(K::Length, 1).checked_arc_length(boolean),
        Err(E::BuiltinInputKind {
            builtin: B::ArcLength,
            argument: 0,
            actual: K::Length
        })
    );
    assert_eq!(
        value(K::Angle, 1).checked_arc_length(boolean),
        Err(E::BuiltinInputKind {
            builtin: B::ArcLength,
            argument: 1,
            actual: K::Boolean
        })
    );
    for builtin in [B::Abs, B::Sqrt] {
        let actual = if builtin == B::Abs {
            boolean.checked_abs()
        } else {
            boolean.checked_sqrt()
        };
        assert_eq!(
            actual,
            Err(E::BuiltinInputKind {
                builtin,
                argument: 0,
                actual: K::Boolean
            })
        );
    }
    assert_eq!(
        value(K::Area, -1).checked_sqrt(),
        Err(E::NegativeSquareRoot {
            kind: K::Area,
            value: R::from_signed_integer(-1)
        })
    );
    assert_eq!(
        value(K::Length, 1).checked_clamp(value(K::Ratio, 2), boolean),
        Err(E::BuiltinDimension {
            builtin: B::Clamp,
            argument: 1,
            wanted: K::Length,
            actual: K::Ratio
        })
    );
    assert_eq!(
        value(K::Length, 1).checked_round_to(boolean),
        Err(E::BuiltinDimension {
            builtin: B::RoundTo,
            argument: 1,
            wanted: K::Length,
            actual: K::Boolean
        })
    );
    assert_eq!(
        value(K::Length, 1).checked_clamp(value(K::Length, 3), value(K::Length, 2)),
        Err(E::InvalidClamp {
            kind: K::Length,
            low: R::from_signed_integer(3),
            high: R::from_signed_integer(2)
        })
    );
    assert_eq!(
        value(K::Length, 1)
            .checked_round_to(value(K::Length, 0))
            .expect_err("zero step")
            .diagnostic_code(),
        "formula_division"
    );
}

#[test]
fn extrema_arity_singletons_and_first_mismatched_kinds_are_explicit() {
    for (builtin, apply) in [
        (B::Min, V::checked_min as fn(&[V]) -> Result<V, E>),
        (B::Max, V::checked_max),
    ] {
        assert_eq!(apply(&[]), Err(E::BuiltinArity { builtin, actual: 0 }));
        assert_eq!(
            apply(&[V::from_boolean(false)]),
            Err(E::BuiltinInputKind {
                builtin,
                argument: 0,
                actual: K::Boolean
            })
        );
        let one = value(K::Ratio, -2).with_call_source(B::Sqrt);
        assert_eq!(apply(&[one]).expect("singleton"), one);
        assert_eq!(
            apply(&[
                value(K::Length, 1),
                value(K::Ratio, 0),
                V::from_boolean(false)
            ]),
            Err(E::BuiltinDimension {
                builtin,
                argument: 1,
                wanted: K::Length,
                actual: K::Ratio
            })
        );
        assert_eq!(
            apply(&[value(K::Length, 1), V::from_boolean(false)]),
            Err(E::BuiltinDimension {
                builtin,
                argument: 1,
                wanted: K::Length,
                actual: K::Boolean
            })
        );
    }
}
