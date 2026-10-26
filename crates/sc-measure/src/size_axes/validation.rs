//! Identity/current-source inventory precedes complete member-coordinate coverage.
use super::{SizeAxesDefinition, SizeAxesError as Error, SizeMemberPoint};
use crate::{SizeAxis, SizeAxisError, SizeAxisSourceContext, SizeMembership};
use sc_core::ontology::EntityId;
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn validate(
    definition: &SizeAxesDefinition,
    membership: &SizeMembership,
    sources: &SizeAxisSourceContext<'_>,
) -> Result<(), Error> {
    if definition.reference != membership.reference() {
        return Err(Error::MembershipMismatch {
            expected: definition.reference,
            actual: membership.reference(),
        });
    }
    if definition.axes.is_empty() {
        return Err(Error::EmptyAxes(definition.reference.id));
    }
    let axes = axes(definition, membership, sources)?;
    let mut members = BTreeSet::new();
    let mut points = BTreeMap::new();
    for point in &definition.points {
        membership.member(point.member).map_err(Error::Member)?;
        if !members.insert(point.member) {
            return Err(Error::DuplicateMember(point.member));
        }
        let coordinates = coordinates(point, &axes)?;
        if let Some(first) = points.insert(coordinates, point.member) {
            return Err(Error::DuplicatePoint {
                first,
                duplicate: point.member,
            });
        }
    }
    for member in &membership.definition().members {
        if !members.contains(&member.id) {
            return Err(Error::MissingMember(member.id));
        }
    }
    Ok(())
}

fn axes<'a>(
    definition: &'a SizeAxesDefinition,
    membership: &SizeMembership,
    sources: &SizeAxisSourceContext<'_>,
) -> Result<BTreeMap<EntityId, &'a SizeAxis>, Error> {
    let mut identities = BTreeSet::new();
    for identity in std::iter::once(definition.reference.id).chain(
        membership
            .definition()
            .members
            .iter()
            .map(|member| member.id),
    ) {
        if !identities.insert(identity) || sources.contains(identity) {
            return Err(Error::DuplicateIdentity(identity));
        }
    }
    let mut axes = BTreeMap::new();
    let mut names = BTreeMap::new();
    for axis in &definition.axes {
        let actual = axis.definition();
        if actual.reference != definition.reference {
            return Err(Error::AxisMismatch {
                axis: actual.id,
                expected: definition.reference,
                actual: actual.reference,
            });
        }
        for identity in std::iter::once(actual.id).chain(actual.values.iter().map(|value| value.id))
        {
            if !identities.insert(identity) || sources.contains(identity) {
                return Err(Error::DuplicateIdentity(identity));
            }
        }
        if let Some(first) = names.insert(actual.name.as_str(), actual.id) {
            return Err(Error::DuplicateName {
                first,
                duplicate: actual.id,
                name: actual.name.clone(),
            });
        }
        axis.validate_current(sources).map_err(Error::Axis)?;
        axes.insert(actual.id, axis);
    }
    Ok(axes)
}

fn coordinates(
    point: &SizeMemberPoint,
    axes: &BTreeMap<EntityId, &SizeAxis>,
) -> Result<BTreeMap<EntityId, EntityId>, Error> {
    let mut coordinates = BTreeMap::new();
    for coordinate in &point.coordinates {
        let axis = axes.get(&coordinate.axis).ok_or(Error::UnknownAxis {
            member: point.member,
            axis: coordinate.axis,
        })?;
        if coordinates
            .insert(coordinate.axis, coordinate.value)
            .is_some()
        {
            return Err(Error::DuplicateCoordinate {
                member: point.member,
                axis: coordinate.axis,
            });
        }
        if !axis
            .definition()
            .values
            .iter()
            .any(|value| value.id == coordinate.value)
        {
            return Err(Error::Axis(SizeAxisError::MissingValue {
                axis: coordinate.axis,
                value: coordinate.value,
            }));
        }
    }
    for axis in axes.keys() {
        if !coordinates.contains_key(axis) {
            return Err(Error::MissingCoordinate {
                member: point.member,
                axis: *axis,
            });
        }
    }
    Ok(coordinates)
}
