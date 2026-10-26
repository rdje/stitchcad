//! Consistent current borrowed inventories for axis/POM/adjacent-pair declarations.
use super::{SizeBreakDefinition, SizeBreakError as Error};
use crate::{
    Measurement, MeasurementBinding, MeasurementTable, MeasurementTableContext, SizeAxes,
    SizeAxisSourceContext, SizeMembership,
};
use sc_core::{ontology::EntityId, value::LengthDeclaration};
use std::collections::BTreeSet;

/// Immutable locally validated snapshot of actual membership, axes, measurement table and sources.
/// The shared numeric inventory includes axis inputs and break deltas; identical canonical Length
/// records shared with measurement inputs are accepted, but conflicting content/roles are refused.
/// Reconstruct for replacement records. This is not global registry/evidence/currentness certification.
#[derive(Debug)]
pub struct SizeBreakContext<'a> {
    membership: &'a SizeMembership,
    axes: &'a SizeAxes,
    sources: &'a SizeAxisSourceContext<'a>,
    table: &'a MeasurementTable,
    measurements: &'a MeasurementTableContext<'a>,
    metadata: BTreeSet<EntityId>,
}
impl<'a> SizeBreakContext<'a> {
    /// Validate complete current axes/table metadata and unambiguous identities/content across inventories.
    /// Numeric Unknown/Derived sources remain unread.
    /// # Errors
    /// Returns original axes/table failures or the first metadata/canonical source conflict.
    pub fn new(
        membership: &'a SizeMembership,
        axes: &'a SizeAxes,
        sources: &'a SizeAxisSourceContext<'a>,
        table: &'a MeasurementTable,
        measurements: &'a MeasurementTableContext<'a>,
    ) -> Result<Self, Error> {
        axes.validate_current(membership, sources)
            .map_err(Error::Axes)?;
        table.validate_current(measurements).map_err(Error::Table)?;
        if let Some(identity) = sources.conflicts_with(measurements) {
            return Err(Error::ConflictingSource(identity));
        }
        let mut metadata = BTreeSet::new();
        let axis_ids = axes.definition().axes.iter().flat_map(|axis| {
            std::iter::once(axis.definition().id)
                .chain(axis.definition().values.iter().map(|value| value.id))
        });
        for identity in std::iter::once(membership.reference().id)
            .chain(
                membership
                    .definition()
                    .members
                    .iter()
                    .map(|member| member.id),
            )
            .chain(axis_ids)
            .chain(std::iter::once(table.id()))
        {
            if !metadata.insert(identity)
                || measurements.contains(identity)
                || sources.contains(identity)
            {
                return Err(Error::DuplicateIdentity(identity));
            }
        }
        Ok(Self {
            membership,
            axes,
            sources,
            table,
            measurements,
            metadata,
        })
    }

    /// Actual membership snapshot that supplies stable identities, authored order and base.
    #[must_use]
    pub const fn membership(&self) -> &'a SizeMembership {
        self.membership
    }

    pub(super) fn dimension(&self, identity: EntityId) -> Result<&'a Measurement, Error> {
        self.table
            .measurement_by_id(identity, self.measurements)
            .map_err(Error::Table)
    }

    pub(super) fn resolve(
        &self,
        definition: &SizeBreakDefinition,
    ) -> Result<&'a LengthDeclaration, Error> {
        if definition.reference != self.membership.reference() {
            return Err(Error::MembershipMismatch {
                expected: definition.reference,
                actual: self.membership.reference(),
            });
        }
        if self.metadata.contains(&definition.id)
            || self.measurements.contains(definition.id)
            || self.sources.contains(definition.id)
        {
            return Err(Error::DuplicateIdentity(definition.id));
        }
        if !self
            .axes
            .definition()
            .axes
            .iter()
            .any(|axis| axis.definition().id == definition.axis)
        {
            return Err(Error::MissingAxis(definition.axis));
        }
        self.membership
            .member(definition.from)
            .map_err(Error::Member)?;
        self.membership
            .member(definition.to)
            .map_err(Error::Member)?;
        let adjacent = self.membership.definition().members.windows(2).any(|pair| {
            matches!(pair, [from, to] if from.id == definition.from && to.id == definition.to)
        });
        if !adjacent {
            return Err(Error::NotAdjacent {
                set: definition.reference.id,
                from: definition.from,
                to: definition.to,
            });
        }
        if definition.dimension_table != self.table.id() {
            return Err(Error::TableMismatch {
                expected: definition.dimension_table,
                actual: self.table.id(),
            });
        }
        let actual = MeasurementBinding::from(self.dimension(definition.dimension.measurement)?);
        if actual != definition.dimension {
            return Err(Error::DimensionMismatch {
                expected: Box::new(definition.dimension.clone()),
                actual: Box::new(actual),
            });
        }
        self.sources.length(definition.delta).ok_or_else(|| {
            if self.sources.contains(definition.delta) {
                Error::DeltaRole(definition.delta)
            } else {
                Error::MissingDelta(definition.delta)
            }
        })
    }
}
