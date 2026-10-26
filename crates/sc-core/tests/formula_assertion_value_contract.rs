//! Named assertion outcomes retain real comparison values/context; false never becomes success.
#![allow(clippy::expect_used)]
use sc_core::{
    name::MachineToken,
    ontology::EntityId,
    recipe::{
        FormulaBuiltin as B, FormulaEvaluatedScalar as V, FormulaKind as K,
        FormulaNumericValue as N, FormulaRational as R, FormulaToleranceContext as Context,
        FormulaToleranceContextDefinition as Definition, FormulaToleranceName as T,
        FormulaToleranceProvider as Provider,
    },
};
use sc_units::{Length, Tolerance, ToleranceClass as Class};

fn value(kind: K, integer: i128) -> V {
    V::from_numeric(N::new(kind, R::from_signed_integer(integer)).expect("numeric"))
}
fn provider(class: Class, limit: i64) -> Tolerance {
    Tolerance::new(
        class,
        Length::from_micrometres(limit).expect("limit"),
        "test receiver requirement",
    )
    .expect("tolerance")
}

#[test]
fn each_named_class_and_numeric_kind_returns_actual_satisfied_or_false_assertion() {
    let format = provider(Class::FormatQuantization, 25);
    let importer = provider(Class::ImporterComparison, 40);
    let physical = provider(Class::PhysicalAcceptance, 100);
    let context = Context::new(Definition {
        label: Some("receiver_a"),
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
    .expect("actual providers");
    let name = MachineToken::new("actual_closure").expect("name");
    for kind in [K::Length, K::Area, K::Angle, K::Ratio, K::Count] {
        for (class, limit) in [
            (T::Numerical, 1),
            (T::Geometric, 10),
            (T::Format, 25),
            (T::Importer, 40),
            (T::Physical, 100),
        ] {
            let tolerance = context.read(class).expect("actual class");
            for delta in [limit, limit + 1] {
                let comparison = value(kind, 100)
                    .compare_tolerance(value(kind, 100 + delta), tolerance)
                    .expect("admitted class");
                match comparison.assert_named(&name) {
                    Ok(satisfied) => {
                        assert_eq!(delta, limit);
                        assert!(std::ptr::eq(satisfied.name(), &name));
                        assert_eq!(satisfied.comparison(), comparison);
                        assert!(satisfied.comparison().holds());
                    }
                    Err(error) => {
                        assert_eq!(delta, limit + 1);
                        assert_eq!(error.diagnostic_code(), "formula_assertion");
                        assert!(std::ptr::eq(error.name(), &name));
                        assert_eq!(error.comparison(), comparison);
                        assert!(!error.comparison().holds());
                        assert_eq!(error.comparison().tolerance(), tolerance);
                        assert_eq!(
                            error.comparison().tolerance().context_label(),
                            Some("receiver_a")
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn actual_worked_book_false_closure_retains_stored_values_and_named_diagnostic() {
    let name = MachineToken::new("waistband_width_closure").expect("assertion label");
    let input_name = MachineToken::new("waistband_width").expect("binding name");
    let original = value(K::Length, 80_000).bind(&input_name).expect("binding");
    let left = original.read().expect("stored read");
    let right = value(K::Length, 40_000);
    let tolerance = Context::empty().read(T::Numerical).expect("intrinsic");
    let comparison = left.compare_tolerance(right, tolerance).expect("compare");
    let error = comparison
        .assert_named(&name)
        .expect_err("false assertion must refuse");
    assert_eq!(error.comparison().left(), left);
    assert_eq!(error.comparison().right(), right);
    assert_eq!(error.to_string(), "formula_assertion: assert waistband_width_closure: length:80000 != length:40000 at eps_num (length:1)");
    assert!(!format!("{error:?}").contains(name.as_str()));
    assert_eq!(original.read().expect("original binding unchanged"), left);
    assert_eq!(error.clone(), error);
}

#[test]
fn signed_fractional_and_raw_turn_failures_render_actual_internal_values() {
    let name = MachineToken::new("fractional_closure").expect("name");
    let tolerance = Context::empty().read(T::Numerical).expect("intrinsic");
    let left = V::from_numeric(
        N::new(K::Angle, R::new(1_440_000_001, 2, true).expect("fraction"))
            .expect("signed raw turn"),
    );
    let right = value(K::Angle, 0);
    let comparison = left
        .compare_tolerance(right, tolerance)
        .expect("admitted compare");
    let error = comparison.assert_named(&name).expect_err("false");
    assert_eq!(error.to_string(), "formula_assertion: assert fractional_closure: angle:-1440000001/2 != angle:0 at eps_num (length:1)");
    assert_eq!(error.comparison().left(), left);
}

#[test]
fn assertion_outcomes_keep_comparison_sources_and_actual_names_without_erasure() {
    let name = MachineToken::new("source_closure").expect("name");
    let other_name = MachineToken::new("other_closure").expect("name");
    let tolerance = Context::empty().read(T::Geometric).expect("T2");
    let left = value(K::Length, 5).with_call_source(B::Hypot);
    for (right, holds) in [
        (value(K::Length, 10).with_call_source(B::RoundTo), true),
        (value(K::Length, 20), false),
    ] {
        let comparison = left
            .compare_tolerance(right, tolerance)
            .expect("class admitted");
        let retained = if holds {
            let accepted = comparison.assert_named(&name).expect("satisfied");
            assert_eq!(accepted.name(), &name);
            assert!(!format!("{accepted:?}").contains(name.as_str()));
            accepted.comparison()
        } else {
            let failed = comparison.assert_named(&name).expect_err("false");
            assert_eq!(failed.name(), &name);
            let another = comparison
                .assert_named(&other_name)
                .expect_err("other actual name");
            assert_eq!(another.name(), &other_name);
            assert_eq!(another.comparison(), failed.comparison());
            failed.comparison()
        };
        assert_eq!(
            retained.value().sources(),
            left.sources().union(right.sources())
        );
        assert_eq!(retained.holds(), holds);
    }
}
