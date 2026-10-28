//! Explicit required break dimensions and current adjacent-pair coverage, without numeric defaults.
use crate::{MeasurementBinding, SizeBreak, SizeBreakContext, SizeSetReference};
use sc_core::ontology::EntityId;
use std::collections::{BTreeMap, BTreeSet};
mod error;
pub use error::SizeBreaksError;

/// One authored required axis/measurement dimension, independent of available break records.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SizeBreakDimension {
    /// Explicit actual axis identity.
    pub axis: EntityId,
    /// Named current table owning this measurement.
    pub dimension_table: EntityId,
    /// Exact canonical identity/token/domain/declaration binding.
    pub dimension: MeasurementBinding,
}

/// One exact required or queried adjacent break cell, without a numeric or label-derived delta.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SizeBreakCell {
    /// Actual axis identity.
    pub axis: EntityId,
    /// Actual measurement dimension identity.
    pub dimension: EntityId,
    /// Earlier authored member.
    pub from: EntityId,
    /// Immediately following authored member.
    pub to: EntityId,
}
impl SizeBreakCell {
    fn key(self) -> CellKey {
        (self.axis, self.dimension, self.from, self.to)
    }
}
type CellKey = (EntityId, EntityId, EntityId, EntityId);

/// Authored requirement inventory and available declarations; partial drafts are representable.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SizeBreaksDefinition {
    /// Exact SizeSet identity/revision shared by the current context and every declaration.
    pub reference: SizeSetReference,
    /// Explicit ordered required dimensions, never inferred from available deltas or chart labels.
    pub dimensions: Vec<SizeBreakDimension>,
    /// Available authored declarations, preserving their input order.
    pub breaks: Vec<SizeBreak>,
}

/// Immutable structurally valid break inventory. Coverage is an explicit current query, not a cache.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SizeBreaks {
    definition: SizeBreaksDefinition,
}
impl SizeBreaks {
    pub(crate) fn resource_definition(&self) -> &SizeBreaksDefinition {
        let Self { definition } = self;
        definition
    }
    /// Validate all actual requirement/break bindings and uniqueness, allowing explicitly partial drafts.
    /// # Errors
    /// Returns current-reference errors, duplicate cells/identities or an undeclared dimension.
    pub fn new(
        definition: SizeBreaksDefinition,
        context: &SizeBreakContext<'_>,
    ) -> Result<Self, SizeBreaksError> {
        let result = Self { definition };
        result.validate_current(context)?;
        Ok(result)
    }

    /// Original independent requirement and declaration vectors, without unchecked mutation or sorting.
    #[must_use]
    pub const fn definition(&self) -> &SizeBreaksDefinition {
        &self.definition
    }

    /// Revalidate every actual saved dimension and declaration against the current supplied snapshot.
    /// # Errors
    /// Returns original source/key failures or ambiguous/undeclared inventory evidence.
    pub fn validate_current(&self, context: &SizeBreakContext<'_>) -> Result<(), SizeBreaksError> {
        self.inventory(context).map(|_| ())
    }

    /// All missing cells in requirement order, then authored member-pair order; no cells are filled.
    /// # Errors
    /// Returns current structural/source errors before reporting any coverage gap.
    pub fn missing(
        &self,
        context: &SizeBreakContext<'_>,
    ) -> Result<Vec<SizeBreakCell>, SizeBreaksError> {
        let actual = self.inventory(context)?;
        let members = &context.membership().definition().members;
        let mut missing = Vec::new();
        for dimension in &self.definition.dimensions {
            for (from, to) in members.iter().zip(members.iter().skip(1)) {
                let cell = SizeBreakCell {
                    axis: dimension.axis,
                    dimension: dimension.dimension.measurement,
                    from: from.id,
                    to: to.id,
                };
                if !actual.contains_key(&cell.key()) {
                    missing.push(cell);
                }
            }
        }
        Ok(missing)
    }

    /// Require nonempty explicit requirements and complete adjacent-pair coverage for a graded range.
    /// This proves authored coverage, not numeric availability, actual grading or artifact policy.
    /// # Errors
    /// Refuses current metadata failures, any singleton, absent requirements or the first named gap.
    pub fn require_grade_coverage(
        &self,
        context: &SizeBreakContext<'_>,
    ) -> Result<(), SizeBreaksError> {
        let gaps = self.missing(context)?;
        if let [member] = context.membership().definition().members.as_slice() {
            return Err(SizeBreaksError::Singleton {
                set: self.definition.reference.id,
                member: member.id,
            });
        }
        if self.definition.dimensions.is_empty() {
            return Err(SizeBreaksError::MissingDimensions(
                self.definition.reference.id,
            ));
        }
        if let Some(cell) = gaps.first() {
            return Err(SizeBreaksError::MissingCell(*cell));
        }
        Ok(())
    }

    /// Borrow the actual authored declaration for one explicitly required axis/dimension/adjacent pair.
    /// # Errors
    /// Returns current inventory errors, an undeclared dimension, invalid pair or exact missing cell.
    pub fn declaration(
        &self,
        cell: SizeBreakCell,
        context: &SizeBreakContext<'_>,
    ) -> Result<&SizeBreak, SizeBreaksError> {
        let inventory = self.inventory(context)?;
        if !self
            .definition
            .dimensions
            .iter()
            .any(|value| value.axis == cell.axis && value.dimension.measurement == cell.dimension)
        {
            return Err(SizeBreaksError::UndeclaredDimension {
                axis: cell.axis,
                dimension: cell.dimension,
            });
        }
        context
            .validate_pair(cell.from, cell.to)
            .map_err(SizeBreaksError::Context)?;
        inventory
            .get(&cell.key())
            .copied()
            .ok_or(SizeBreaksError::MissingCell(cell))
    }

    fn inventory(
        &self,
        context: &SizeBreakContext<'_>,
    ) -> Result<BTreeMap<CellKey, &SizeBreak>, SizeBreaksError> {
        if self.definition.reference != context.membership().reference() {
            return Err(SizeBreaksError::MembershipMismatch {
                expected: self.definition.reference,
                actual: context.membership().reference(),
            });
        }
        let mut dimensions = BTreeSet::new();
        for value in &self.definition.dimensions {
            context
                .validate_dimension(value.axis, value.dimension_table, &value.dimension)
                .map_err(SizeBreaksError::Context)?;
            if !dimensions.insert((value.axis, value.dimension.measurement)) {
                return Err(SizeBreaksError::DuplicateDimension {
                    axis: value.axis,
                    dimension: value.dimension.measurement,
                });
            }
        }
        let mut identities = BTreeSet::new();
        let mut inventory = BTreeMap::new();
        for value in &self.definition.breaks {
            let d = value.definition();
            if !identities.insert(d.id) {
                return Err(SizeBreaksError::DuplicateIdentity(d.id));
            }
            value
                .validate_current(context)
                .map_err(|issue| SizeBreaksError::Break {
                    declaration: d.id,
                    issue: Box::new(issue),
                })?;
            if !dimensions.contains(&(d.axis, d.dimension.measurement)) {
                return Err(SizeBreaksError::UndeclaredDimension {
                    axis: d.axis,
                    dimension: d.dimension.measurement,
                });
            }
            let cell = SizeBreakCell {
                axis: d.axis,
                dimension: d.dimension.measurement,
                from: d.from,
                to: d.to,
            };
            if inventory.insert(cell.key(), value).is_some() {
                return Err(SizeBreaksError::DuplicateCell(cell));
            }
        }
        Ok(inventory)
    }
}
