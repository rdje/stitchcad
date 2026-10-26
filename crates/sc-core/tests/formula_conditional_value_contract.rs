//! Real lazy callbacks: supplied kinds checked first; untaken unknowns and sources stay untouched.
#![allow(clippy::expect_used)]
use sc_core::{
    ontology::EntityId,
    recipe::{
        FormulaBuiltin as B, FormulaConditionalBranch as Branch,
        FormulaConditionalValueError as Error, FormulaEvaluatedScalar as V, FormulaKind as K,
        FormulaNumericValue as N, FormulaRational as R,
    },
    value::{LengthDeclaration, LengthDeclarationDefinition, LengthState, LengthValueError},
};
use std::cell::Cell;

fn value(kind: K, integer: i128) -> V {
    V::from_numeric(N::new(kind, R::from_signed_integer(integer)).expect("numeric"))
}

#[test]
fn both_supplied_branch_kinds_and_condition_are_checked_before_any_callback() {
    let kinds = [
        K::Length,
        K::Area,
        K::Angle,
        K::Ratio,
        K::Count,
        K::Boolean,
        K::Point,
        K::Edge,
    ];
    for condition in [
        value(K::Count, 1),
        V::from_boolean(false),
        V::from_boolean(true),
    ] {
        for then_kind in kinds {
            for else_kind in kinds {
                let calls = Cell::new(0);
                let body = || {
                    calls.set(calls.get() + 1);
                    Ok::<_, ()>(value(K::Length, 1))
                };
                let result = condition.checked_if(then_kind, else_kind, body, body);
                let valid = condition.kind() == K::Boolean
                    && then_kind == else_kind
                    && matches!(
                        then_kind,
                        K::Length | K::Area | K::Angle | K::Ratio | K::Count
                    );
                if !valid {
                    let error = result.expect_err("invalid supplied kind metadata");
                    assert!(
                        matches!(error, Error::InputKinds { condition: actual, then_kind: actual_then, else_kind: actual_else }
                        if actual == condition.kind() && actual_then == then_kind && actual_else == else_kind)
                    );
                    assert_eq!(error.diagnostic_code(), Some("formula_dimension"));
                    assert_eq!(calls.get(), 0);
                } else {
                    assert_eq!(calls.get(), 1);
                }
            }
        }
    }
}

#[test]
fn only_the_selected_branch_executes_once_and_only_its_sources_join_the_condition() {
    for boolean in [false, true] {
        for kind in [K::Length, K::Area, K::Angle, K::Ratio, K::Count] {
            let then_calls = Cell::new(0);
            let else_calls = Cell::new(0);
            let condition = V::from_boolean(boolean).with_call_source(B::Sin);
            let result = condition
                .checked_if(
                    kind,
                    kind,
                    || {
                        then_calls.set(then_calls.get() + 1);
                        Ok::<_, ()>(value(kind, 3).with_call_source(B::Sqrt))
                    },
                    || {
                        else_calls.set(else_calls.get() + 1);
                        Ok::<_, ()>(value(kind, 4).with_call_source(B::Hypot))
                    },
                )
                .expect("selected callback");
            assert_eq!(then_calls.get(), u32::from(boolean));
            assert_eq!(else_calls.get(), u32::from(!boolean));
            assert_eq!(
                result.numeric().expect("numeric").rational(),
                R::from_signed_integer(if boolean { 3 } else { 4 })
            );
            assert_eq!(result.kind(), kind);
            assert_eq!(
                result.sources().iter().map(B::token).collect::<Vec<_>>(),
                if boolean {
                    vec!["sin", "sqrt"]
                } else {
                    vec!["hypot", "sin"]
                }
            );
            assert!(condition.sources().contains(B::Sin));
            assert!(!condition.sources().contains(B::Sqrt));
        }
    }
}

#[test]
fn actual_unknown_authored_input_is_never_read_untaken_and_propagates_unchanged_when_taken() {
    let record = LengthDeclaration::new(LengthDeclarationDefinition {
        id: EntityId::from_bits(1),
        source: EntityId::from_bits(2),
        state: LengthState::Unknown {
            observation: EntityId::from_bits(3),
        },
    })
    .expect("actual unknown");
    for unknown_is_then in [false, true] {
        for take_unknown in [false, true] {
            let reads = Cell::new(0);
            let unknown = || {
                reads.set(reads.get() + 1);
                record
                    .authored_value()
                    .map(|length| V::from_numeric(length.into()))
            };
            let known = || Ok::<_, LengthValueError>(value(K::Length, 5));
            let condition = V::from_boolean(unknown_is_then == take_unknown);
            let result = if unknown_is_then {
                condition.checked_if(K::Length, K::Length, unknown, known)
            } else {
                condition.checked_if(K::Length, K::Length, known, unknown)
            };
            assert_eq!(reads.get(), u32::from(take_unknown));
            if take_unknown {
                let error = result.expect_err("observation still required");
                let wanted = record
                    .authored_value()
                    .expect_err("actual canonical reason");
                assert_eq!(error.computation_error(), Some(&wanted));
                assert_eq!(error.diagnostic_code(), None);
                assert_eq!(error.to_string(), wanted.to_string());
                assert!(std::error::Error::source(&error).is_some());
                assert!(matches!(error, Error::Computation { branch, .. }
                    if branch == if unknown_is_then { Branch::Then } else { Branch::Else }));
            } else {
                assert_eq!(
                    result.expect("untaken unknown is untouched"),
                    value(K::Length, 5)
                );
            }
        }
    }
}

#[test]
fn selected_callback_kind_violation_has_actual_branch_and_no_other_callback_execution() {
    for boolean in [false, true] {
        let calls = Cell::new(0);
        let wrong = || {
            calls.set(calls.get() + 1);
            Ok::<_, ()>(V::from_boolean(false))
        };
        let error = V::from_boolean(boolean)
            .checked_if(K::Length, K::Length, wrong, wrong)
            .expect_err("selected result violated kind contract");
        assert_eq!(calls.get(), 1);
        assert_eq!(error.diagnostic_code(), Some("formula_dimension"));
        assert!(
            matches!(error, Error::OutputKind { branch, declared: K::Length, actual: K::Boolean }
            if branch == if boolean { Branch::Then } else { Branch::Else })
        );
        assert!(error.computation_error().is_none());
    }
}
