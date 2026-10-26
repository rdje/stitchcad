//! Complete explicit member coordinates, sparse grids and current canonical borrows.
#![allow(clippy::expect_used, clippy::indexing_slicing, clippy::panic)]
use sc_core::{
    ontology::EntityId,
    value::{
        ScalarInputDeclaration, ScalarInputDeclarationDefinition, ScalarInputKind, ScalarInputState,
    },
};
use sc_measure::{
    SizeAxes, SizeAxesDefinition, SizeAxesError, SizeAxis, SizeAxisCoordinate, SizeAxisDefinition,
    SizeAxisError, SizeAxisKind, SizeAxisSourceContext, SizeAxisValueContent,
    SizeAxisValueDefinition, SizeAxisValueSource, SizeLabel, SizeMember, SizeMemberPoint,
    SizeMembership, SizeMembershipDefinition, SizeMembershipError, SizeSetReference, SizeSystem,
};
use sc_units::Count;
use std::error::Error;
fn id(n: u128) -> EntityId {
    EntityId::from_bits(n)
}
fn label(s: &str) -> SizeLabel {
    SizeLabel::new(s.to_owned()).expect("fixture label")
}
fn membership() -> SizeMembership {
    SizeMembership::new(SizeMembershipDefinition {
        reference: SizeSetReference {
            id: id(1),
            revision: Count::new(3),
        },
        system: SizeSystem::Custom,
        members: vec![
            SizeMember {
                id: id(3),
                label: label("0012"),
            },
            SizeMember {
                id: id(2),
                label: label("2XL"),
            },
        ],
        base: id(2),
    })
    .expect("authored membership")
}
fn coordinate(axis: u128, value: u128) -> SizeAxisCoordinate {
    SizeAxisCoordinate {
        axis: id(axis),
        value: id(value),
    }
}
fn axis(n: u128, name: &str, context: &SizeAxisSourceContext<'_>) -> SizeAxis {
    SizeAxis::new(
        SizeAxisDefinition {
            reference: membership().reference(),
            id: id(n),
            name: name.to_owned(),
            kind: SizeAxisKind::Label,
            values: vec![
                SizeAxisValueDefinition {
                    id: id(n + 2),
                    source: SizeAxisValueSource::Label(label("Second")),
                },
                SizeAxisValueDefinition {
                    id: id(n + 1),
                    source: SizeAxisValueSource::Label(label("First")),
                },
            ],
        },
        context,
    )
    .expect("authored axis")
}
fn definition(context: &SizeAxisSourceContext<'_>) -> SizeAxesDefinition {
    SizeAxesDefinition {
        reference: membership().reference(),
        axes: vec![axis(20, "Chest", context), axis(10, "Height", context)],
        points: vec![
            SizeMemberPoint {
                member: id(2),
                coordinates: vec![coordinate(10, 11), coordinate(20, 21)],
            },
            SizeMemberPoint {
                member: id(3),
                coordinates: vec![coordinate(20, 22), coordinate(10, 12)],
            },
        ],
    }
}
fn replace_axis(
    d: &mut SizeAxesDefinition,
    index: usize,
    change: impl FnOnce(&mut SizeAxisDefinition),
    context: &SizeAxisSourceContext<'_>,
) {
    let mut axis = d.axes[index].definition().clone();
    change(&mut axis);
    d.axes[index] = SizeAxis::new(axis, context).expect("locally valid changed axis");
}
#[test]
fn sparse_grid_keeps_independent_authored_orders_and_actual_member_borrows() {
    let context = SizeAxisSourceContext::new(&[], &[]).expect("label sources");
    let members = membership();
    let d = definition(&context);
    let axes = SizeAxes::new(d.clone(), &members, &context).expect("two of four grid points");
    assert_eq!(axes.definition(), &d);
    assert_eq!(members.definition().members[0].id, id(3));
    assert_eq!(axes.definition().points[0].member, id(2));
    assert_eq!(axes.definition().axes[0].definition().id, id(20));
    assert_eq!(axes.definition().points[0].coordinates[0].axis, id(10));
    let point = axes
        .point(id(2), &members, &context)
        .expect("actual member");
    assert!(std::ptr::eq(
        point.member(),
        members.member(id(2)).expect("member")
    ));
    assert!(std::ptr::eq(
        point.definition(),
        &axes.definition().points[0]
    ));
    assert_eq!(point.member().label.as_str(), "2XL");
    let value = axes
        .coordinate(id(2), id(10), &members, &context)
        .expect("explicit coordinate");
    assert_eq!(value.definition().id, id(11));
    assert_eq!(
        value.content(),
        SizeAxisValueContent::Label(&label("First"))
    );
    assert_eq!(
        axes.coordinate(id(2), id(999), &members, &context).err(),
        Some(SizeAxesError::MissingAxis(id(999)))
    );
    let refusal = SizeAxesError::Member(SizeMembershipError::MissingMember {
        set: id(1),
        member: id(999),
    });
    assert_eq!(
        axes.point(id(999), &members, &context).err(),
        Some(refusal.clone())
    );
    assert!(refusal.source().is_some());
}
#[test]
fn singleton_requires_authored_axes_and_each_member_requires_every_coordinate() {
    let context = SizeAxisSourceContext::new(&[], &[]).expect("label sources");
    let members = membership();
    let mut single = members.definition().clone();
    single.members.truncate(1);
    single.base = id(3);
    let single = SizeMembership::new(single).expect("singleton");
    let mut d = definition(&context);
    d.axes.clear();
    d.points.clear();
    assert_eq!(
        SizeAxes::new(d, &single, &context).err(),
        Some(SizeAxesError::EmptyAxes(id(1)))
    );
    let mut d = definition(&context);
    d.axes.truncate(1);
    d.points.retain(|point| point.member == id(3));
    d.points[0].coordinates.retain(|c| c.axis == id(20));
    assert!(SizeAxes::new(d, &single, &context).is_ok());
    let mut d = definition(&context);
    d.points.clear();
    assert_eq!(
        SizeAxes::new(d, &members, &context).err(),
        Some(SizeAxesError::MissingMember(id(3)))
    );
    let mut d = definition(&context);
    d.points[0].coordinates.pop();
    assert_eq!(
        SizeAxes::new(d, &members, &context).err(),
        Some(SizeAxesError::MissingCoordinate {
            member: id(2),
            axis: id(20)
        })
    );
    let mut d = definition(&context);
    d.points[0].coordinates.push(coordinate(10, 12));
    assert_eq!(
        SizeAxes::new(d, &members, &context).err(),
        Some(SizeAxesError::DuplicateCoordinate {
            member: id(2),
            axis: id(10)
        })
    );
    let mut d = definition(&context);
    d.points[0].coordinates[0] = coordinate(99, 11);
    assert_eq!(
        SizeAxes::new(d, &members, &context).err(),
        Some(SizeAxesError::UnknownAxis {
            member: id(2),
            axis: id(99)
        })
    );
    let mut d = definition(&context);
    d.points[0].coordinates[0] = coordinate(10, 21);
    assert_eq!(
        SizeAxes::new(d, &members, &context).err(),
        Some(SizeAxesError::Axis(SizeAxisError::MissingValue {
            axis: id(10),
            value: id(21)
        }))
    );
    let mut d = definition(&context);
    d.points[0].member = id(99);
    assert_eq!(
        SizeAxes::new(d, &members, &context).err(),
        Some(SizeAxesError::Member(SizeMembershipError::MissingMember {
            set: id(1),
            member: id(99)
        }))
    );
}
#[test]
fn pin_and_membership_changes_are_revalidated_before_queries() {
    let context = SizeAxisSourceContext::new(&[], &[]).expect("label sources");
    let members = membership();
    let axes = SizeAxes::new(definition(&context), &members, &context).expect("axes");
    let mut changed = members.definition().clone();
    changed.reference.revision = Count::new(4);
    let changed = SizeMembership::new(changed).expect("new revision");
    let refusal = SizeAxesError::MembershipMismatch {
        expected: members.reference(),
        actual: changed.reference(),
    };
    assert_eq!(
        axes.point(id(999), &changed, &context).err(),
        Some(refusal.clone())
    );
    let mut empty = definition(&context);
    empty.axes.clear();
    assert_eq!(
        SizeAxes::new(empty, &changed, &context).err(),
        Some(refusal)
    );
    let mut d = definition(&context);
    replace_axis(
        &mut d,
        0,
        |a| a.reference.revision = Count::new(4),
        &context,
    );
    assert_eq!(
        SizeAxes::new(d, &members, &context).err(),
        Some(SizeAxesError::AxisMismatch {
            axis: id(20),
            expected: members.reference(),
            actual: changed.reference()
        })
    );
    let mut changed = members.definition().clone();
    changed.members.reverse();
    changed.members[0].label = label("Different actual label");
    changed.base = id(3);
    let changed = SizeMembership::new(changed).expect("explicit replacement");
    let point = axes
        .point(id(2), &changed, &context)
        .expect("same identities/pin");
    assert_eq!(point.member().label.as_str(), "Different actual label");
    assert!(std::ptr::eq(
        point.member(),
        changed.member(id(2)).expect("actual replacement")
    ));
    assert_eq!(
        axes.point(id(2), &members, &context)
            .expect("old snapshot")
            .member()
            .label
            .as_str(),
        "2XL"
    );
    let mut added = members.definition().clone();
    added.members.push(SizeMember {
        id: id(4),
        label: label("new"),
    });
    let added = SizeMembership::new(added).expect("new member");
    assert_eq!(
        axes.validate_current(&added, &context),
        Err(SizeAxesError::MissingMember(id(4)))
    );
}
#[test]
fn aliases_duplicate_names_members_and_order_independent_points_refuse() {
    let context = SizeAxisSourceContext::new(&[], &[]).expect("label sources");
    let members = membership();
    for (index, identity, is_value) in [(0, 2, false), (0, 3, true), (1, 21, true)] {
        let mut d = definition(&context);
        replace_axis(
            &mut d,
            index,
            |a| {
                if is_value {
                    a.values[0].id = id(identity)
                } else {
                    a.id = id(identity)
                }
            },
            &context,
        );
        assert_eq!(
            SizeAxes::new(d, &members, &context).err(),
            Some(SizeAxesError::DuplicateIdentity(id(identity)))
        );
    }
    let mut d = definition(&context);
    d.axes.push(d.axes[0].clone());
    assert_eq!(
        SizeAxes::new(d, &members, &context).err(),
        Some(SizeAxesError::DuplicateIdentity(id(20)))
    );
    let mut d = definition(&context);
    replace_axis(&mut d, 1, |a| a.name = "Chest".to_owned(), &context);
    assert_eq!(
        SizeAxes::new(d, &members, &context).err(),
        Some(SizeAxesError::DuplicateName {
            first: id(20),
            duplicate: id(10),
            name: "Chest".to_owned()
        })
    );
    let mut d = definition(&context);
    replace_axis(&mut d, 1, |a| a.name = " chest ".to_owned(), &context);
    assert!(SizeAxes::new(d, &members, &context).is_ok());
    let mut d = definition(&context);
    d.points.push(d.points[0].clone());
    assert_eq!(
        SizeAxes::new(d, &members, &context).err(),
        Some(SizeAxesError::DuplicateMember(id(2)))
    );
    let mut d = definition(&context);
    d.points[1].coordinates = d.points[0].coordinates.iter().rev().copied().collect();
    assert_eq!(
        SizeAxes::new(d, &members, &context).err(),
        Some(SizeAxesError::DuplicatePoint {
            first: id(2),
            duplicate: id(3)
        })
    );
}
#[test]
fn numeric_coordinate_returns_original_record_and_current_source_failures() {
    let scalar = |kind| {
        ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
            id: id(50),
            source: id(60),
            kind,
            state: ScalarInputState::Unknown {
                observation: id(70),
            },
        })
        .expect("canonical scalar")
    };
    let record = scalar(ScalarInputKind::Length);
    let context = SizeAxisSourceContext::new(&[], &[&record]).expect("actual source");
    let mut members = membership().definition().clone();
    members.members.truncate(1);
    members.base = id(3);
    let members = SizeMembership::new(members).expect("singleton");
    let axis = SizeAxis::new(
        SizeAxisDefinition {
            reference: members.reference(),
            id: id(10),
            name: "Explicit length".to_owned(),
            kind: SizeAxisKind::Scalar(ScalarInputKind::Length),
            values: vec![SizeAxisValueDefinition {
                id: id(11),
                source: SizeAxisValueSource::Scalar(id(50)),
            }],
        },
        &context,
    )
    .expect("numeric axis");
    let axes = SizeAxes::new(
        SizeAxesDefinition {
            reference: members.reference(),
            axes: vec![axis],
            points: vec![SizeMemberPoint {
                member: id(3),
                coordinates: vec![coordinate(10, 11)],
            }],
        },
        &members,
        &context,
    )
    .expect("complete singleton axes");
    let SizeAxisValueContent::Scalar(actual) = axes
        .coordinate(id(3), id(10), &members, &context)
        .expect("coordinate")
        .content()
    else {
        panic!("wrong source role")
    };
    assert!(std::ptr::eq(actual, &record));
    assert!(actual.authored_value().is_err());
    let changed = scalar(ScalarInputKind::Ratio);
    let changed_context = SizeAxisSourceContext::new(&[], &[&changed]).expect("replacement source");
    let refusal = SizeAxesError::Axis(SizeAxisError::Kind {
        axis: id(10),
        value: id(11),
        wanted: SizeAxisKind::Scalar(ScalarInputKind::Length),
        actual: SizeAxisKind::Scalar(ScalarInputKind::Ratio),
    });
    assert_eq!(
        axes.coordinate(id(3), id(10), &members, &changed_context)
            .err(),
        Some(refusal.clone())
    );
    assert!(refusal.source().is_some());
    let alias = ScalarInputDeclaration::new(ScalarInputDeclarationDefinition {
        id: id(3),
        source: id(60),
        kind: ScalarInputKind::Length,
        state: ScalarInputState::Unknown {
            observation: id(70),
        },
    })
    .expect("alias fixture");
    let alias_context =
        SizeAxisSourceContext::new(&[], &[&record, &alias]).expect("distinct sources");
    assert_eq!(
        axes.validate_current(&members, &alias_context),
        Err(SizeAxesError::DuplicateIdentity(id(3)))
    );
}
