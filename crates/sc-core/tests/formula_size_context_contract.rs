//! Core size frames carry supplied source/range facts, without inventing authored membership.
#![allow(clippy::expect_used)]
use sc_core::{
    name::MachineToken,
    ontology::EntityId,
    recipe::{
        FormulaBindingValueRule, FormulaBoundValue, FormulaKind as K, FormulaNumericValue as V,
        FormulaRational as R, FormulaReservedName, FormulaSizeContext as Context,
        FormulaSizeContextError as Error, FormulaSizeContextSource as Source,
        FormulaSizeName as Name, FormulaSizeReadError,
    },
};
use sc_units::Count;

fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn source() -> Source {
    Source {
        set: id(1),
        revision: Count::new(0),
        member: id(2),
        base: id(3),
    }
}

#[test]
fn size_role_metadata_reuses_only_the_three_existing_reserved_names() {
    for (name, reserved, token, kind) in [
        (
            Name::Index,
            FormulaReservedName::SizeIndex,
            "size_index",
            K::Count,
        ),
        (
            Name::Count,
            FormulaReservedName::SizeCount,
            "size_count",
            K::Count,
        ),
        (
            Name::IsBaseSize,
            FormulaReservedName::IsBaseSize,
            "is_base_size",
            K::Boolean,
        ),
    ] {
        assert_eq!(name.reserved_name(), reserved);
        assert_eq!(name.token(), token);
        assert_eq!(reserved.size_name(), Some(name));
        assert_eq!(reserved.kind(), kind);
        assert_eq!(reserved.tolerance_name(), None);
    }
    for reserved in FormulaReservedName::ALL {
        assert_eq!(
            reserved.size_name().is_some(),
            reserved.tolerance_name().is_none()
        );
    }
}

#[test]
fn absent_context_refuses_each_actual_name_instead_of_zero_or_false() {
    let absent = Context::empty();
    assert_eq!(absent, Context::default());
    assert_eq!(absent.source(), None);
    for name in Name::ALL {
        let error = absent.read(name).expect_err("size context absent");
        assert_eq!(error, FormulaSizeReadError { name });
        assert_eq!(error.diagnostic_code(), "formula_unbound_name");
        assert!(error.to_string().contains(name.token()));
    }
}

#[test]
fn present_counts_and_false_keep_kind_and_exact_pin_without_inferring_base_from_index() {
    let supplied = source();
    let context = Context::new(supplied, 2, 3).expect("valid supplied frame");
    assert_eq!(context.source(), Some(supplied));
    for (name, integer) in [(Name::Index, 2), (Name::Count, 3)] {
        let actual = context.read(name).expect("present Count");
        assert_eq!(actual.name(), name);
        assert_eq!(actual.source(), supplied);
        assert_eq!(actual.kind(), K::Count);
        assert_eq!(
            actual.numeric_value().expect("Count").rational(),
            R::from_signed_integer(integer)
        );
        assert_eq!(actual.boolean(), None);
    }
    let actual = context.read(Name::IsBaseSize).expect("present false");
    assert_eq!(actual.boolean(), Some(false));
    assert_eq!(actual.numeric_value(), None);
    assert_eq!(actual.kind(), K::Boolean);
    let explicit_base = Source {
        member: supplied.base,
        ..supplied
    };
    let base = Context::new(explicit_base, 2, 3).expect("base can be at ordinal2");
    assert_eq!(
        base.read(Name::IsBaseSize).expect("actual base").boolean(),
        Some(true)
    );
    let singleton = Context::new(explicit_base, 1, 1).expect("singleton is base");
    assert_eq!(
        singleton
            .read(Name::Count)
            .expect("Count")
            .numeric_value()
            .expect("numeric")
            .rational(),
        R::ONE
    );
    assert_eq!(
        singleton.read(Name::IsBaseSize).expect("base").boolean(),
        Some(true)
    );
}

#[test]
fn invalid_source_range_and_singleton_refusals_retain_actual_witnesses_in_order() {
    let supplied = source();
    for aliased in [
        Source {
            member: supplied.set,
            ..supplied
        },
        Source {
            base: supplied.set,
            ..supplied
        },
    ] {
        assert_eq!(
            Context::new(aliased, 0, 0),
            Err(Error::IdentityAlias(aliased))
        );
    }
    for (index, count) in [(0, 0), (1, 0), (0, 3), (4, 3), (u128::MAX, 3)] {
        assert_eq!(
            Context::new(supplied, index, count),
            Err(Error::OrdinalRange {
                source: supplied,
                index,
                count
            })
        );
    }
    assert_eq!(
        Context::new(supplied, 1, 1),
        Err(Error::SingletonBase(supplied))
    );
}

#[test]
fn wide_counts_wait_for_binding_and_source_replacement_does_not_mutate_prior_frame() {
    let name = MachineToken::new("stored_count").expect("binding name");
    assert_eq!(V::from_unsigned_count(0).rational(), R::ZERO);
    let supplied = Source {
        revision: Count::new(u32::MAX),
        ..source()
    };
    let original = Context::new(supplied, u128::MAX, u128::MAX).expect("full-width ordinal/count");
    let count = original
        .read(Name::Count)
        .expect("present wide Count")
        .numeric_value()
        .expect("numeric");
    assert_eq!(count, V::from_unsigned_count(u128::MAX));
    let measured = R::new(u128::MAX, 1, false).expect("width128 is admitted");
    let error = FormulaBoundValue::from_numeric(&name, count).expect_err("i64 only at binding");
    assert_eq!(
        error.rule(),
        FormulaBindingValueRule::Storage {
            minimum: 0,
            maximum: i64::MAX,
            measured,
        }
    );
    let replacement_source = Source {
        revision: Count::new(0),
        member: supplied.base,
        ..supplied
    };
    let replacement = Context::new(replacement_source, 1, 3).expect("independent replacement");
    assert_eq!(original.source(), Some(supplied));
    assert_eq!(replacement.source(), Some(replacement_source));
    assert_eq!(
        original
            .read(Name::IsBaseSize)
            .expect("false retained")
            .boolean(),
        Some(false)
    );
    assert_eq!(
        replacement
            .read(Name::IsBaseSize)
            .expect("actual base")
            .boolean(),
        Some(true)
    );
}
