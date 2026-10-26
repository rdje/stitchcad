//! Explicit optional providers never become default tolerances or approval of their source.
#![allow(clippy::expect_used)]
use sc_core::{
    ontology::EntityId,
    recipe::{
        FormulaKind, FormulaRational as R, FormulaReservedContext as Required,
        FormulaToleranceContext as Context, FormulaToleranceContextDefinition as Definition,
        FormulaToleranceName as Name, FormulaToleranceProvider as Provider,
        FormulaToleranceReadError as ReadError, FormulaToleranceValueSource as Source,
    },
};
use sc_units::{Length, Tolerance, ToleranceClass as Class};

fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn tolerance(class: Class, integer: i64) -> Tolerance {
    Tolerance::new(
        class,
        Length::from_micrometres(integer).expect("fixture Length"),
        "explicit scoped contract fixture, never a product default",
    )
    .expect("nonempty derivation")
}

#[test]
fn empty_context_supplies_only_the_two_exact_language_intrinsics() {
    let context = Context::empty();
    assert_eq!(context, Context::default());
    for (name, integer, class) in [
        (Name::Numerical, 1, Class::Numerical),
        (Name::Geometric, 10, Class::GeometricApproximation),
    ] {
        let value = context.read(name).expect("language intrinsic");
        assert_eq!(value.name(), name);
        assert_eq!(value.value().kind(), FormulaKind::Length);
        assert_eq!(value.value().rational(), R::from_signed_integer(integer));
        assert_eq!(value.tolerance().class, class);
        assert!(!value.tolerance().derivation().is_empty());
        assert_eq!(value.source(), Source::Intrinsic);
        assert_eq!(value.context_label(), None);
    }
    for (name, required) in [
        (Name::Format, Required::Export),
        (Name::Importer, Required::Export),
        (Name::Physical, Required::Profile),
    ] {
        let error = context.read(name).expect_err("no provider, no fallback");
        assert_eq!(
            error,
            ReadError::MissingContext {
                name,
                context_label: None
            }
        );
        assert_eq!(error.name(), name);
        assert_eq!(error.required_context(), required);
        assert_eq!(error.diagnostic_code(), "formula_tolerance_unbound");
    }
}

#[test]
fn every_optional_combination_retains_exact_provider_source_and_actual_label() {
    let format = tolerance(Class::FormatQuantization, 25);
    let importer = tolerance(Class::ImporterComparison, 40);
    let physical = tolerance(Class::PhysicalAcceptance, 100);
    let providers = [
        (
            Name::Format,
            Provider {
                source: id(11),
                tolerance: &format,
            },
            25,
        ),
        (
            Name::Importer,
            Provider {
                source: id(12),
                tolerance: &importer,
            },
            40,
        ),
        (
            Name::Physical,
            Provider {
                source: id(13),
                tolerance: &physical,
            },
            100,
        ),
    ];
    for label in [None, Some("explicit_export_context"), Some("")] {
        for mask in 0_u8..8 {
            let context = Context::new(Definition {
                label,
                format: (mask & 1 != 0).then_some(providers[0].1),
                importer: (mask & 2 != 0).then_some(providers[1].1),
                physical: (mask & 4 != 0).then_some(providers[2].1),
            })
            .expect("classes match their actual slots");
            for (index, (name, provider, integer)) in providers.into_iter().enumerate() {
                if mask & (1 << index) != 0 {
                    let actual = context.read(name).expect("explicit supplied provider");
                    assert_eq!(actual.value().rational(), R::from_signed_integer(integer));
                    assert_eq!(actual.context_label(), label);
                    assert_eq!(actual.tolerance(), *provider.tolerance);
                    assert_eq!(actual.source(), Source::Provider(provider));
                    if let Source::Provider(original) = actual.source() {
                        assert!(std::ptr::eq(original.tolerance, provider.tolerance));
                        assert_eq!(original.source, provider.source);
                    }
                } else {
                    assert_eq!(
                        context.read(name),
                        Err(ReadError::MissingContext {
                            name,
                            context_label: label
                        })
                    );
                }
            }
            for (name, integer) in [(Name::Numerical, 1), (Name::Geometric, 10)] {
                let actual = context
                    .read(name)
                    .expect("provider cannot override intrinsics");
                assert_eq!(actual.value().rational(), R::from_signed_integer(integer));
                assert_eq!(actual.source(), Source::Intrinsic);
            }
        }
    }
}

#[test]
fn each_slot_checks_the_actual_quantitative_class_and_retains_wrong_provider() {
    for class in [
        Class::Numerical,
        Class::GeometricApproximation,
        Class::GeometricChordal,
        Class::FormatQuantization,
        Class::ImporterComparison,
        Class::PhysicalAcceptance,
    ] {
        let quantitative = tolerance(class, 17);
        let provider = Provider {
            source: id(31),
            tolerance: &quantitative,
        };
        for (name, expected, definition) in [
            (
                Name::Format,
                Class::FormatQuantization,
                Definition {
                    format: Some(provider),
                    ..Definition::default()
                },
            ),
            (
                Name::Importer,
                Class::ImporterComparison,
                Definition {
                    importer: Some(provider),
                    ..Definition::default()
                },
            ),
            (
                Name::Physical,
                Class::PhysicalAcceptance,
                Definition {
                    physical: Some(provider),
                    ..Definition::default()
                },
            ),
        ] {
            let result = Context::new(definition);
            if class == expected {
                assert_eq!(
                    result
                        .expect("correct class")
                        .read(name)
                        .expect("present")
                        .tolerance(),
                    quantitative
                );
            } else {
                let error = result.expect_err("wrong class, including the chordal sibling");
                assert_eq!(
                    (error.name, error.expected, error.provider),
                    (name, expected, provider)
                );
                assert!(std::ptr::eq(error.provider.tolerance, &quantitative));
            }
        }
    }
}

#[test]
fn multiple_bad_classes_refuse_the_first_actual_slot_in_declared_order() {
    let wrong = tolerance(Class::Numerical, 1);
    let provider = Provider {
        source: id(32),
        tolerance: &wrong,
    };
    let format = Context::new(Definition {
        format: Some(provider),
        importer: Some(provider),
        physical: Some(provider),
        ..Definition::default()
    })
    .expect_err("format wins");
    assert_eq!(format.name, Name::Format);
    let importer = Context::new(Definition {
        importer: Some(provider),
        physical: Some(provider),
        ..Definition::default()
    })
    .expect_err("then importer");
    assert_eq!(importer.name, Name::Importer);
}

#[test]
fn explicit_zero_is_present_and_cloned_definition_changes_do_not_mutate_prior_context() {
    let zero = tolerance(Class::FormatQuantization, 0);
    let replacement = tolerance(Class::FormatQuantization, 50);
    let original = Context::new(Definition {
        label: Some("original"),
        format: Some(Provider {
            source: id(41),
            tolerance: &zero,
        }),
        ..Definition::default()
    })
    .expect("explicit zero provider");
    let mut changed = *original.definition();
    changed.label = Some("replacement");
    changed.format = Some(Provider {
        source: id(42),
        tolerance: &replacement,
    });
    let revised = Context::new(changed).expect("revalidate replacement");
    let before = original.read(Name::Format).expect("zero is not absent");
    assert_eq!(before.value().rational(), R::ZERO);
    assert_eq!(before.context_label(), Some("original"));
    assert_eq!(
        before.source(),
        Source::Provider(Provider {
            source: id(41),
            tolerance: &zero
        })
    );
    assert_eq!(
        revised
            .read(Name::Format)
            .expect("new provider")
            .value()
            .rational(),
        R::from_signed_integer(50)
    );
    assert_eq!(
        revised
            .read(Name::Format)
            .expect("new provider")
            .context_label(),
        Some("replacement")
    );
    assert!(
        original.read(Name::Importer).is_err(),
        "format never supplies an importer default"
    );
}
