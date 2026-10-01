//! Human labels, authored order/base, stable identity and pinned revision contracts.
#![allow(clippy::unwrap_used, clippy::indexing_slicing)]
use sc_core::ontology::EntityId;
use sc_measure::{
    SizeLabel, SizeMember, SizeMembership, SizeMembershipDefinition, SizeMembershipError,
    SizeSetReference, SizeSystem,
};
use sc_units::Count;
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn label(s: &str) -> SizeLabel {
    SizeLabel::new(s.to_owned()).unwrap()
}
fn definition() -> SizeMembershipDefinition {
    SizeMembershipDefinition {
        reference: SizeSetReference {
            id: id(1),
            revision: Count::new(0),
        },
        system: SizeSystem::Alphanumeric,
        members: vec![
            SizeMember {
                id: id(2),
                label: label("2XL"),
            },
            SizeMember {
                id: id(3),
                label: label("S"),
            },
            SizeMember {
                id: id(4),
                label: label("M"),
            },
        ],
        base: id(3),
    }
}
#[test]
fn human_labels_preserve_exact_unicode_spacing_case_and_numeric_spelling() {
    for s in [
        "2XL",
        "170/88A",
        "12",
        "0012",
        "尺碼 中",
        "  M  ",
        "client size",
    ] {
        let value = label(s);
        assert_eq!(value.as_str(), s);
        assert_eq!(value.to_string(), s);
    }
    assert_ne!(label("12"), label("0012"));
    assert_ne!(label("M"), label("m"));
    assert_ne!(label(" M "), label("M"));
}
#[test]
fn blank_labels_refuse_including_unicode_whitespace() {
    for s in ["", " ", "\t\n", "\u{2003}"] {
        assert_eq!(
            SizeLabel::new(s.to_owned()).err(),
            Some(SizeMembershipError::BlankLabel)
        );
    }
}
#[test]
fn authored_order_and_base_do_not_follow_lexical_or_numeric_sort() {
    let membership = SizeMembership::new(definition()).unwrap();
    assert_eq!(
        membership
            .definition()
            .members
            .iter()
            .map(|member| member.label.as_str())
            .collect::<Vec<_>>(),
        vec!["2XL", "S", "M"]
    );
    assert_eq!(membership.base().unwrap().id, id(3));
    assert_eq!(membership.base().unwrap().label, label("S"));
    assert_eq!(
        membership.reference(),
        SizeSetReference {
            id: id(1),
            revision: Count::new(0)
        }
    );
    let mut d = definition();
    d.system = SizeSystem::Numeric;
    d.members[0].label = label("42");
    d.members[1].label = label("38");
    d.members[2].label = label("40");
    let membership = SizeMembership::new(d).unwrap();
    assert_eq!(
        membership
            .definition()
            .members
            .iter()
            .map(|m| m.label.as_str())
            .collect::<Vec<_>>(),
        vec!["42", "38", "40"]
    );
    assert_eq!(membership.base().unwrap().label, label("38"));
}
#[test]
fn reordering_retains_stable_member_and_base_identities_without_mutating_original() {
    let original = SizeMembership::new(definition()).unwrap();
    let mut d = original.definition().clone();
    d.members.reverse();
    d.reference = d.reference.next_revision().unwrap();
    let changed = SizeMembership::new(d).unwrap();
    for n in [2, 3, 4] {
        assert_eq!(
            original.member(id(n)).unwrap(),
            changed.member(id(n)).unwrap()
        );
    }
    assert_eq!(original.base().unwrap(), changed.base().unwrap());
    assert_eq!(original.definition().members[0].id, id(2));
    assert_eq!(changed.definition().members[0].id, id(4));
    assert_eq!(original.reference().revision, Count::new(0));
    assert_eq!(changed.reference().revision, Count::new(1));
}
#[test]
fn exactly_one_base_requires_nonempty_members_and_existing_identity() {
    let mut d = definition();
    d.members.clear();
    assert_eq!(
        SizeMembership::new(d).err(),
        Some(SizeMembershipError::EmptyMembers(id(1)))
    );
    let mut d = definition();
    d.base = id(100);
    assert_eq!(
        SizeMembership::new(d).err(),
        Some(SizeMembershipError::MissingBase {
            set: id(1),
            base: id(100)
        })
    );
    let mut d = definition();
    d.base = d.reference.id;
    assert_eq!(
        SizeMembership::new(d).err(),
        Some(SizeMembershipError::MissingBase {
            set: id(1),
            base: id(1)
        })
    );
}
#[test]
fn member_ids_and_exact_labels_must_be_unique_and_distinct_from_set() {
    let mut d = definition();
    d.members[1].id = d.members[0].id;
    assert_eq!(
        SizeMembership::new(d).err(),
        Some(SizeMembershipError::DuplicateIdentity(id(2)))
    );
    let mut d = definition();
    d.members[0].id = d.reference.id;
    assert_eq!(
        SizeMembership::new(d).err(),
        Some(SizeMembershipError::DuplicateIdentity(id(1)))
    );
    let mut d = definition();
    d.members[1].label = d.members[0].label.clone();
    assert_eq!(
        SizeMembership::new(d).err(),
        Some(SizeMembershipError::DuplicateLabel {
            set: id(1),
            label: label("2XL")
        })
    );
}
#[test]
fn lookup_is_by_exact_identity_or_label_without_peer_fallback() {
    let membership = SizeMembership::new(definition()).unwrap();
    assert_eq!(membership.member_by_label(&label("S")).unwrap().id, id(3));
    assert!(std::ptr::eq(
        membership.member(id(3)).unwrap(),
        membership.member_by_label(&label("S")).unwrap()
    ));
    assert_eq!(
        membership.member(id(100)).err(),
        Some(SizeMembershipError::MissingMember {
            set: id(1),
            member: id(100)
        })
    );
    assert_eq!(
        membership.member_by_label(&label("s")).err(),
        Some(SizeMembershipError::MissingLabel {
            set: id(1),
            label: label("s")
        })
    );
    assert_eq!(
        membership.member_by_label(&label(" S ")).err(),
        Some(SizeMembershipError::MissingLabel {
            set: id(1),
            label: label(" S ")
        })
    );
}
#[test]
fn removed_member_identity_never_transfers_to_same_label_replacement() {
    let original = SizeMembership::new(definition()).unwrap();
    let mut d = original.definition().clone();
    d.members[0].id = id(100);
    let changed = SizeMembership::new(d).unwrap();
    assert_eq!(
        changed.member(id(2)).err(),
        Some(SizeMembershipError::MissingMember {
            set: id(1),
            member: id(2)
        })
    );
    assert_eq!(changed.member_by_label(&label("2XL")).unwrap().id, id(100));
    assert_eq!(original.member(id(2)).unwrap().id, id(2));
}
#[test]
fn same_labels_across_sets_do_not_imply_same_identity_or_measurements() {
    let a = SizeMembership::new(definition()).unwrap();
    let mut d = definition();
    d.reference.id = id(100);
    let b = SizeMembership::new(d).unwrap();
    assert_eq!(a.definition().members, b.definition().members);
    assert_ne!(a.reference(), b.reference());
    assert_eq!(a.definition().system, b.definition().system);
}
#[test]
fn custom_single_member_has_its_explicit_base_without_claiming_mtm_chart_readiness() {
    let mut d = definition();
    d.system = SizeSystem::Custom;
    d.members = vec![SizeMember {
        id: id(2),
        label: label("Client A"),
    }];
    d.base = id(2);
    let membership = SizeMembership::new(d).unwrap();
    assert_eq!(membership.base().unwrap().label, label("Client A"));
    assert_eq!(membership.definition().members.len(), 1);
}
#[test]
fn all_designation_systems_are_explicit_metadata_without_label_inference() {
    for system in [
        SizeSystem::En13402,
        SizeSystem::AstmD5585,
        SizeSystem::Alphanumeric,
        SizeSystem::Numeric,
        SizeSystem::Custom,
    ] {
        let mut d = definition();
        d.system = system;
        let membership = SizeMembership::new(d).unwrap();
        assert_eq!(membership.definition().system, system);
        assert_eq!(membership.definition().members[0].label, label("2XL"));
    }
}
#[test]
fn revision_successors_retain_identity_and_refuse_overflow_without_wrapping() {
    let first = SizeSetReference {
        id: id(1),
        revision: Count::new(0),
    };
    let next = first.next_revision().unwrap();
    assert_eq!(
        next,
        SizeSetReference {
            id: id(1),
            revision: Count::new(1)
        }
    );
    assert_eq!(first.revision, Count::new(0));
    let before_max = SizeSetReference {
        id: id(1),
        revision: Count::new(u32::MAX - 1),
    };
    let max = before_max.next_revision().unwrap();
    assert_eq!(max.revision, Count::new(u32::MAX));
    assert_eq!(
        max.next_revision().err(),
        Some(SizeMembershipError::RevisionOverflow(max))
    );
    assert_eq!(max.revision, Count::new(u32::MAX));
}
