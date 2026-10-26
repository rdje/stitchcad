//! Shared snapshot and cross-inventory identity checks for both canonical chart families.
use super::{SizeSetInputError as Error, SizeSetSnapshot};
use crate::{
    MeasurementTable, MeasurementTableContext, SizeBreakContext, SizeBreaks, SizeMembership,
};
use sc_core::ontology::EntityId;
use std::{collections::BTreeSet, ptr};

pub(super) fn validate_snapshot(
    membership: &SizeMembership,
    measurements: &MeasurementTableContext<'_>,
    table: &MeasurementTable,
    breaks: &SizeBreakContext<'_>,
) -> Result<(), Error> {
    if !ptr::eq(membership, breaks.membership()) {
        return Err(Error::SharedSnapshot(SizeSetSnapshot::Membership));
    }
    if !ptr::eq(measurements, breaks.measurements()) {
        return Err(Error::SharedSnapshot(SizeSetSnapshot::Measurements));
    }
    if !ptr::eq(table, breaks.table()) {
        return Err(Error::SharedSnapshot(SizeSetSnapshot::DesignTable));
    }
    Ok(())
}
pub(super) fn validate_entities<'a>(
    chart: EntityId,
    entities: impl Iterator<Item = EntityId>,
    tables: impl Iterator<Item = &'a MeasurementTable>,
    breaks: &SizeBreaks,
    context: &SizeBreakContext<'_>,
) -> Result<(), Error> {
    let ids = breaks
        .definition()
        .breaks
        .iter()
        .map(|value| value.definition().id)
        .collect::<BTreeSet<_>>();
    for identity in std::iter::once(chart).chain(entities) {
        if context.contains_entity(identity) || ids.contains(&identity) {
            return Err(Error::DuplicateIdentity(identity));
        }
    }
    for table in tables {
        if ids.contains(&table.id())
            || (!ptr::eq(table, context.table()) && context.contains_entity(table.id()))
        {
            return Err(Error::DuplicateIdentity(table.id()));
        }
    }
    Ok(())
}
