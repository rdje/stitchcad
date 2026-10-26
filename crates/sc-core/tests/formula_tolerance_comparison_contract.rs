//! Inclusive exact thresholds, actual context and retained sources; no statement execution claimed.
#![allow(clippy::expect_used)]
use sc_core::{
    name::MachineToken,
    ontology::EntityId,
    recipe::{
        FormulaBuiltin as B, FormulaEvaluatedScalar as V, FormulaKind as K,
        FormulaNumericError as E, FormulaNumericValue as N, FormulaRational as R,
        FormulaToleranceComparisonRefusal as Refusal, FormulaToleranceContext as Context,
        FormulaToleranceContextDefinition as Definition, FormulaToleranceName as T,
        FormulaToleranceProvider as Provider,
    },
};
use sc_units::{Length, Tolerance, ToleranceClass as Class};

fn value(kind: K, numerator: u128, denominator: u128) -> V {
    V::from_numeric(
        N::new(
            kind,
            R::new(numerator, denominator, false).expect("rational"),
        )
        .expect("numeric"),
    )
}
fn provider(class: Class, limit: i64) -> Tolerance {
    Tolerance::new(
        class,
        Length::from_micrometres(limit).expect("limit"),
        "explicit test requirement",
    )
    .expect("tolerance")
}

#[test]
fn every_numeric_kind_and_named_class_uses_the_exact_inclusive_internal_quantum_threshold() {
    let format = provider(Class::FormatQuantization, 25);
    let importer = provider(Class::ImporterComparison, 40);
    let physical = provider(Class::PhysicalAcceptance, 100);
    let context = Context::new(Definition {
        label: Some("actual_target"),
        format: Some(Provider {
            source: EntityId::from_bits(1),
            tolerance: &format,
        }),
        importer: Some(Provider {
            source: EntityId::from_bits(2),
            tolerance: &importer,
        }),
        physical: Some(Provider {
            source: EntityId::from_bits(3),
            tolerance: &physical,
        }),
    })
    .expect("matching classes");
    for kind in [K::Length, K::Area, K::Angle, K::Ratio, K::Count] {
        for (name, limit) in [
            (T::Numerical, 1),
            (T::Geometric, 10),
            (T::Format, 25),
            (T::Importer, 40),
            (T::Physical, 100),
        ] {
            let tolerance = context.read(name).expect("actual context");
            for (numerator, denominator, holds) in [
                (0, 1, true),
                (limit, 1, true),
                (2 * limit - 1, 2, true),
                (2 * limit + 1, 2, false),
            ] {
                for (left, right) in [
                    (value(kind, 0, 1), value(kind, numerator, denominator)),
                    (value(kind, numerator, denominator), value(kind, 0, 1)),
                ] {
                    let actual = left
                        .compare_tolerance(right, tolerance)
                        .expect("admitted comparison");
                    assert_eq!(
                        actual.holds(),
                        holds,
                        "{kind:?}/{name:?}/{numerator}/{denominator}"
                    );
                    assert_eq!(actual.value().boolean(), Some(holds));
                    assert!(actual.value().sources().is_empty());
                    assert_eq!(actual.left(), left);
                    assert_eq!(actual.right(), right);
                    assert_eq!(actual.tolerance(), tolerance);
                    assert_eq!(actual.tolerance().context_label(), Some("actual_target"));
                }
            }
        }
    }
}

#[test]
fn comparison_workspace_does_not_materialize_a_wide_or_out_of_domain_numeric_difference() {
    let m = u128::MAX;
    let a = R::new(m, m - 1, false).expect("a");
    let b = R::new(1, m - 2, false).expect("b");
    // a-b = 1 - 1/((m-1)*(m-2)): below 1, above 1-1/m, with a 256-bit reduced denominator.
    assert!(a.checked_sub(b).is_err());
    assert!(a.is_within(b, R::ONE));
    assert!(!a.is_within(b, R::new(m - 1, m, false).expect("just below 1")));
    assert!(b.is_within(a, R::ONE));
    let tiny = R::new(1, m, false).expect("tiny");
    let negative = R::new(1, m - 1, true).expect("negative tiny");
    assert!(tiny.checked_sub(negative).is_err());
    assert!(tiny.is_within(negative, R::ONE));
    assert!(!tiny.is_within(negative, R::ZERO));
    assert!(a.is_within(a, R::ZERO));
    assert!(!a.is_within(a, R::from_signed_integer(-1)));
    let maximum = R::new(m, 1, false).expect("maximum");
    assert!(!maximum.is_within(maximum.negated(), maximum));

    let context = Context::empty();
    let left =
        V::from_numeric(N::new(K::Length, R::from_signed_integer(-1_000_000_000)).expect("left"));
    let right = value(K::Length, 1_000_000_000, 1);
    assert!(left
        .checked_binary(sc_core::recipe::FormulaBinaryOperator::Subtract, right)
        .is_err());
    assert!(!left
        .compare_tolerance(right, context.read(T::Geometric).expect("intrinsic"))
        .expect("comparison not subtraction")
        .holds());
    let left = V::from_numeric(N::new(K::Ratio, a).expect("ratio"));
    let right = V::from_numeric(N::new(K::Ratio, b).expect("ratio"));
    assert!(left
        .compare_tolerance(right, context.read(T::Numerical).expect("intrinsic"))
        .expect("no wide result")
        .holds());
}

#[test]
fn t1_refuses_actual_sources_before_testing_even_a_zero_difference_and_retains_all_context() {
    let context = Context::empty();
    let tolerance = context.read(T::Numerical).expect("T1");
    let left = value(K::Length, 3, 1).with_call_source(B::Hypot);
    for right in [
        value(K::Length, 3, 1),
        value(K::Length, 100, 1).with_call_source(B::RoundTo),
    ] {
        let error = left
            .compare_tolerance(right, tolerance)
            .expect_err("class refusal first");
        assert_eq!(error.diagnostic_code(), "formula_domain");
        assert_eq!(error.left(), left);
        assert_eq!(error.right(), right);
        assert_eq!(error.tolerance(), tolerance);
        assert!(std::error::Error::source(&error).is_some());
        assert!(matches!(error.refusal(), Refusal::Approximation(actual)
            if actual.tolerance() == T::Numerical && actual.sources() == left.sources().union(right.sources())));
    }
    let geometric = context.read(T::Geometric).expect("T2");
    let name = MachineToken::new("within_result").expect("name");
    for (right, holds) in [
        (value(K::Length, 4, 1), true),
        (value(K::Length, 14, 1), false),
    ] {
        let comparison = left
            .compare_tolerance(right, geometric)
            .expect("class admitted");
        assert_eq!(comparison.holds(), holds);
        let result = comparison.value();
        assert_eq!(result.sources(), left.sources());
        let read = result
            .bind(&name)
            .expect("Boolean binding")
            .read()
            .expect("read");
        assert_eq!(read, result);
        assert_eq!(read.boolean(), Some(holds));
        assert!(read.sources().check_tolerance(T::Numerical).is_err());
    }
}

#[test]
fn actual_kind_refusals_precede_approximation_class_and_keep_supplied_values() {
    let context = Context::empty();
    let tolerance = context.read(T::Numerical).expect("T1");
    let mut values = [K::Length, K::Area, K::Angle, K::Ratio, K::Count]
        .map(|kind| value(kind, 0, 1))
        .to_vec();
    values.push(V::from_boolean(false));
    for left in values.iter().copied() {
        for right in values.iter().copied() {
            let left = left.with_call_source(B::Sin);
            let result = left.compare_tolerance(right, tolerance);
            let wanted = if left.kind() == K::Boolean {
                Some(E::BuiltinInputKind {
                    builtin: B::Within,
                    argument: 0,
                    actual: K::Boolean,
                })
            } else if left.kind() != right.kind() {
                Some(E::BuiltinDimension {
                    builtin: B::Within,
                    argument: 1,
                    wanted: left.kind(),
                    actual: right.kind(),
                })
            } else {
                None
            };
            let error = result.expect_err("dimension or T1 refusal");
            if let Some(wanted) = wanted {
                assert_eq!(error.refusal(), Refusal::Dimension(wanted));
                assert_eq!(error.diagnostic_code(), "formula_dimension");
            } else {
                assert!(matches!(error.refusal(), Refusal::Approximation(_)));
            }
            assert_eq!(error.left(), left);
            assert_eq!(error.right(), right);
            assert_eq!(error.tolerance(), tolerance);
        }
    }
}

#[test]
fn explicit_zero_and_negative_provider_limits_are_not_replaced_by_defaults() {
    for (limit, holds) in [(0, true), (-1, false)] {
        let format = provider(Class::FormatQuantization, limit);
        let source = Provider {
            source: EntityId::from_bits(7),
            tolerance: &format,
        };
        let context = Context::new(Definition {
            label: Some(""),
            format: Some(source),
            ..Definition::default()
        })
        .expect("explicit provider");
        let tolerance = context.read(T::Format).expect("format");
        let input = value(K::Ratio, 1_500_000, 1);
        let actual = input
            .compare_tolerance(input, tolerance)
            .expect("class admitted");
        assert_eq!(actual.holds(), holds);
        assert_eq!(actual.tolerance().tolerance(), format);
        assert_eq!(
            actual.tolerance().source(),
            sc_core::recipe::FormulaToleranceValueSource::Provider(source)
        );
        assert_eq!(actual.tolerance().context_label(), Some(""));
    }
}
