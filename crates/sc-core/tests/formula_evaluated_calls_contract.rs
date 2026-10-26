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

#[test]
fn sincos_retain_real_upstream_and_own_sources_through_storage_and_tolerance() {
    let name = MachineToken::new("trig_result").expect("name");
    for (degrees, sine, cosine) in [
        (0, 0, 1_000_000),
        (30, 500_000, 866_025),
        (45, 707_107, 707_107),
        (-450, -1_000_000, 0),
    ] {
        let angle = value(K::Angle, degrees * 1_000_000)
            .checked_round_to(value(K::Angle, 1))
            .expect("real upstream quantization");
        for (result, integer, builtin) in [
            (angle.checked_sin(), sine, B::Sin),
            (angle.checked_cos(), cosine, B::Cos),
        ] {
            let computed = result.expect("actual native call");
            let expected = if builtin == B::Sin {
                vec!["round_to", "sin"]
            } else {
                vec!["cos", "round_to"]
            };
            assert_value(computed, K::Ratio, integer, &expected);
            let cancelled = computed
                .checked_binary(Op::Subtract, computed)
                .expect("cancel");
            let read = cancelled.bind(&name).expect("bind").read().expect("read");
            assert_value(read, K::Ratio, 0, &expected);
            let failure = read
                .sources()
                .check_tolerance(T::Numerical)
                .expect_err("T1 retains call");
            assert_eq!(failure.sources(), computed.sources());
            read.sources()
                .check_tolerance(T::Geometric)
                .expect("T2 class guard");
        }
        assert_eq!(
            names(angle),
            ["round_to"],
            "original input remains immutable"
        );
    }
    for input in [
        V::from_boolean(false),
        value(K::Length, 0),
        value(K::Ratio, 0),
    ] {
        for (result, builtin) in [(input.checked_sin(), B::Sin), (input.checked_cos(), B::Cos)] {
            assert_eq!(
                result,
                Err(E::BuiltinInputKind {
                    builtin,
                    argument: 0,
                    actual: input.kind()
                })
            );
        }
        assert!(input.sources().is_empty());
    }
}

#[test]
fn inverse_calls_keep_real_operand_sources_even_at_axes_and_after_cancellation() {
    let name = MachineToken::new("inverse_result").expect("name");
    for (ppm, expected) in [(0, 0), (1_000_000, 45_000_000), (-1_000_000, -45_000_000)] {
        let input = value(K::Ratio, ppm)
            .checked_round_to(value(K::Ratio, 1))
            .expect("upstream round");
        let result = input.checked_atan().expect("actual inverse");
        assert_value(result, K::Angle, expected, &["atan", "round_to"]);
        let read = result
            .checked_binary(Op::Subtract, result)
            .expect("cancel")
            .bind(&name)
            .expect("bind")
            .read()
            .expect("read");
        assert_value(read, K::Angle, 0, &["atan", "round_to"]);
        let failure = read
            .sources()
            .check_tolerance(T::Numerical)
            .expect_err("T1 source guard");
        assert_eq!(failure.sources(), result.sources());
        read.sources()
            .check_tolerance(T::Geometric)
            .expect("T2 source guard");
        assert_eq!(names(input), ["round_to"]);
    }
    for (kind, y, x, expected) in [
        (K::Length, 0, -1, 180_000_000),
        (K::Length, 3, 4, 36_869_898),
        (K::Ratio, -1, -1, -135_000_000),
    ] {
        let y = value(kind, y)
            .checked_round_to(value(kind, 1))
            .expect("real y source");
        let x = value(kind, x)
            .checked_round_to(value(kind, 1))
            .expect("real x source");
        let result = y.checked_atan2(x).expect("principal atan2");
        assert_value(result, K::Angle, expected, &["atan2", "round_to"]);
        let read = result.bind(&name).expect("bind").read().expect("read");
        assert_eq!(read, result);
        assert!(read.sources().check_tolerance(T::Numerical).is_err());
    }
    // Distinct actual calls on both operands must survive independently, including zero y.
    for (area, expected) in [(0, 0), (9, 36_869_898)] {
        let y = value(K::Area, area).checked_sqrt().expect("y sqrt");
        let x = value(K::Length, 0)
            .checked_hypot(value(K::Length, 4))
            .expect("x hypot");
        assert_value(
            y.checked_atan2(x).expect("both sources"),
            K::Angle,
            expected,
            &["atan2", "hypot", "sqrt"],
        );
        assert_eq!(names(y), ["sqrt"]);
        assert_eq!(names(x), ["hypot"]);
    }
}

#[test]
fn inverse_scalar_kind_priority_and_zero_vector_match_the_numeric_boundary() {
    let kinds = [K::Length, K::Area, K::Angle, K::Ratio, K::Count, K::Boolean];
    let zero = |kind| {
        if kind == K::Boolean {
            V::from_boolean(false)
        } else {
            value(kind, 0)
        }
    };
    for ykind in kinds {
        let y = zero(ykind);
        if ykind != K::Ratio {
            assert_eq!(
                y.checked_atan(),
                Err(E::BuiltinInputKind {
                    builtin: B::Atan,
                    argument: 0,
                    actual: ykind,
                })
            );
        }
        for xkind in kinds {
            let expected = if !matches!(ykind, K::Length | K::Ratio) {
                E::BuiltinInputKind {
                    builtin: B::Atan2,
                    argument: 0,
                    actual: ykind,
                }
            } else if xkind != ykind {
                E::BuiltinDimension {
                    builtin: B::Atan2,
                    argument: 1,
                    wanted: ykind,
                    actual: xkind,
                }
            } else {
                E::Atan2ZeroVector {
                    kind: ykind,
                    y: R::ZERO,
                    x: R::ZERO,
                }
            };
            assert_eq!(y.checked_atan2(zero(xkind)), Err(expected));
        }
        assert!(y.sources().is_empty());
    }
}
