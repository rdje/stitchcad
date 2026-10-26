//! Formula size values come from actual authored membership, never labels or inferred bases.
#![allow(clippy::expect_used)]
use sc_core::{
    ontology::EntityId,
    recipe::{FormulaRational as R, FormulaSizeContextSource, FormulaSizeName as Name},
};
use sc_measure::{
    SizeFormulaContextError as Error, SizeLabel, SizeMember, SizeMembership,
    SizeMembershipDefinition as Definition, SizeMembershipError, SizeSetReference, SizeSystem,
};
use sc_units::Count;
use std::error::Error as _;

fn id(bits: u128) -> EntityId {
    EntityId::from_bits(bits)
}
fn reference(set: u128, revision: u32) -> SizeSetReference {
    SizeSetReference {
        id: id(set),
        revision: Count::new(revision),
    }
}
fn member(identity: u128, label: &str) -> SizeMember {
    SizeMember {
        id: id(identity),
        label: SizeLabel::new(label.to_owned()).expect("nonblank authored label"),
    }
}
fn membership(system: SizeSystem) -> SizeMembership {
    SizeMembership::new(Definition {
        reference: reference(1, 0),
        system,
        members: vec![member(11, "42"), member(12, "0012"), member(13, "2XL")],
        base: id(12),
    })
    .expect("valid canonical order/base")
}

#[test]
fn all_designation_intents_preserve_actual_order_base_pin_and_selected_borrows() {
    for system in [
        SizeSystem::En13402,
        SizeSystem::AstmD5585,
        SizeSystem::Alphanumeric,
        SizeSystem::Numeric,
        SizeSystem::Custom,
    ] {
        let membership = membership(system);
        let before = membership.clone();
        for (member_id, ordinal, is_base, label) in [
            (11, 1, false, "42"),
            (12, 2, true, "0012"),
            (13, 3, false, "2XL"),
        ] {
            let context = membership
                .formula_context(reference(1, 0), id(member_id))
                .expect("exact pin/member");
            assert!(std::ptr::eq(context.membership(), &membership));
            assert!(std::ptr::eq(
                context.member(),
                membership.member(id(member_id)).expect("actual member")
            ));
            assert_eq!(context.member().label.as_str(), label);
            let source = FormulaSizeContextSource {
                set: id(1),
                revision: Count::new(0),
                member: id(member_id),
                base: id(12),
            };
            assert_eq!(context.context().source(), Some(source));
            assert_eq!(
                context
                    .read(Name::Index)
                    .expect("ordinal")
                    .numeric_value()
                    .expect("Count")
                    .rational(),
                R::from_signed_integer(ordinal)
            );
            assert_eq!(
                context
                    .read(Name::Count)
                    .expect("count")
                    .numeric_value()
                    .expect("Count")
                    .rational(),
                R::from_signed_integer(3)
            );
            assert_eq!(
                context.read(Name::IsBaseSize).expect("base flag").boolean(),
                Some(is_base)
            );
            assert!(
                !format!("{context:?}").contains(label),
                "Debug omits human labels"
            );
        }
        assert_eq!(membership, before);
    }
}

#[test]
fn expected_reference_mismatch_precedes_missing_member_and_preserves_both_pins() {
    let membership = membership(SizeSystem::Custom);
    for requested in [reference(99, 0), reference(1, 1), reference(99, u32::MAX)] {
        let error = membership
            .formula_context(requested, id(999))
            .expect_err("wrong pin before missing member");
        assert_eq!(
            error,
            Error::ReferenceMismatch {
                requested,
                actual: reference(1, 0),
                member: id(999)
            }
        );
        assert!(error.source().is_none());
    }
    let error = membership
        .formula_context(reference(1, 0), id(999))
        .expect_err("actual absent member");
    let expected = SizeMembershipError::MissingMember {
        set: id(1),
        member: id(999),
    };
    assert_eq!(error, Error::Membership(expected.clone()));
    assert_eq!(
        error
            .source()
            .and_then(|source| source.downcast_ref::<SizeMembershipError>()),
        Some(&expected)
    );
}

#[test]
fn explicit_reorder_changes_only_the_new_snapshot_position_and_retains_stable_base() {
    let original = membership(SizeSystem::Numeric);
    let old_context = original
        .formula_context(reference(1, 0), id(11))
        .expect("old first member");
    let mut replacement = original.definition().clone();
    replacement.reference = replacement
        .reference
        .next_revision()
        .expect("revision successor");
    replacement.members.rotate_left(1);
    let replaced = SizeMembership::new(replacement).expect("explicit reauthored order");
    let new_context = replaced
        .formula_context(reference(1, 1), id(11))
        .expect("same stable selected member");
    assert_eq!(
        old_context
            .read(Name::Index)
            .expect("old index")
            .numeric_value()
            .expect("Count")
            .rational(),
        R::ONE
    );
    assert_eq!(
        new_context
            .read(Name::Index)
            .expect("new index")
            .numeric_value()
            .expect("Count")
            .rational(),
        R::from_signed_integer(3)
    );
    assert_eq!(old_context.member(), new_context.member());
    assert_eq!(
        old_context.context().source().expect("old source").revision,
        Count::new(0)
    );
    assert_eq!(
        new_context.context().source().expect("new source").revision,
        Count::new(1)
    );
    let new_base = replaced
        .formula_context(reference(1, 1), id(12))
        .expect("base now first");
    assert_eq!(
        new_base
            .read(Name::Index)
            .expect("base position")
            .numeric_value()
            .expect("Count")
            .rational(),
        R::ONE
    );
    assert_eq!(
        new_base
            .read(Name::IsBaseSize)
            .expect("explicit base id")
            .boolean(),
        Some(true)
    );
    assert_eq!(
        old_context
            .read(Name::IsBaseSize)
            .expect("old selected non-base")
            .boolean(),
        Some(false)
    );
    assert!(matches!(
        replaced.formula_context(reference(1, 0), id(11)),
        Err(Error::ReferenceMismatch { .. })
    ));
}

#[test]
fn a_single_custom_member_is_base_at_revision_zero_or_the_maximum_pin() {
    for revision in [0, u32::MAX] {
        let reference = reference(50, revision);
        let membership = SizeMembership::new(Definition {
            reference,
            system: SizeSystem::Custom,
            members: vec![member(51, "private bespoke label")],
            base: id(51),
        })
        .expect("singleton membership");
        let context = membership
            .formula_context(reference, id(51))
            .expect("actual singleton");
        for name in [Name::Index, Name::Count] {
            assert_eq!(
                context
                    .read(name)
                    .expect("present Count")
                    .numeric_value()
                    .expect("numeric")
                    .rational(),
                R::ONE
            );
        }
        assert_eq!(
            context.read(Name::IsBaseSize).expect("base").boolean(),
            Some(true)
        );
        assert_eq!(
            context.context().source().expect("pinned source").revision,
            Count::new(revision)
        );
        assert!(!format!("{context:?}").contains("private bespoke label"));
    }
}

#[test]
fn reused_human_labels_in_distinct_sets_never_supply_cross_set_identity() {
    let first = SizeMembership::new(Definition {
        reference: reference(100, 0),
        system: SizeSystem::Custom,
        members: vec![member(101, "42")],
        base: id(101),
    })
    .expect("first singleton");
    let second = SizeMembership::new(Definition {
        reference: reference(200, 0),
        system: SizeSystem::Custom,
        members: vec![member(201, "42")],
        base: id(201),
    })
    .expect("second singleton");
    assert_eq!(
        first
            .formula_context(reference(100, 0), id(201))
            .expect_err("same label does not identify a member"),
        Error::Membership(SizeMembershipError::MissingMember {
            set: id(100),
            member: id(201)
        })
    );
    assert_eq!(
        second
            .formula_context(reference(200, 0), id(201))
            .expect("actual second member")
            .member()
            .id,
        id(201)
    );
}
