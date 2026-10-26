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

    pub(crate) const fn axes(&self) -> &'a SizeAxes {
        self.axes
    }
    pub(crate) const fn table(&self) -> &'a MeasurementTable {
        self.table
    }
    pub(crate) const fn measurements(&self) -> &'a MeasurementTableContext<'a> {
        self.measurements
    }
    pub(crate) fn contains_entity(&self, identity: EntityId) -> bool {
        self.metadata.contains(&identity)
            || self.measurements.contains(identity)
            || self.sources.contains(identity)
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
        self.require_axis(definition.axis)?;
        self.validate_pair(definition.from, definition.to)?;
        self.require_dimension(definition.dimension_table, &definition.dimension)?;
        self.sources.length(definition.delta).ok_or_else(|| {
            if self.sources.contains(definition.delta) {
                Error::DeltaRole(definition.delta)
            } else {
                Error::MissingDelta(definition.delta)
            }
        })
    }
    fn require_axis(&self, axis: EntityId) -> Result<(), Error> {
        if !self
            .axes
            .definition()
            .axes
            .iter()
            .any(|value| value.definition().id == axis)
        {
            return Err(Error::MissingAxis(axis));
        }
        Ok(())
    }

    pub(crate) fn validate_pair(&self, from: EntityId, to: EntityId) -> Result<(), Error> {
        self.membership.member(from).map_err(Error::Member)?;
        self.membership.member(to).map_err(Error::Member)?;
        if !self
            .membership
            .definition()
            .members
            .windows(2)
            .any(|pair| matches!(pair, [a, b] if a.id == from && b.id == to))
        {
            return Err(Error::NotAdjacent {
                set: self.membership.reference().id,
                from,
                to,
            });
        }
        Ok(())
    }

    pub(crate) fn validate_dimension(
        &self,
        axis: EntityId,
        table: EntityId,
        binding: &MeasurementBinding,
    ) -> Result<(), Error> {
        self.require_axis(axis)?;
        self.require_dimension(table, binding)
    }

    fn require_dimension(
        &self,
        table: EntityId,
        binding: &MeasurementBinding,
    ) -> Result<(), Error> {
        if table != self.table.id() {
            return Err(Error::TableMismatch {
                expected: table,
                actual: self.table.id(),
            });
        }
        let actual = MeasurementBinding::from(self.dimension(binding.measurement)?);
        if actual != *binding {
            return Err(Error::DimensionMismatch {
                expected: Box::new(binding.clone()),
                actual: Box::new(actual),
            });
        }
        Ok(())
    }
}
