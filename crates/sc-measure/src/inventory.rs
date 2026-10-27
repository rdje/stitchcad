//! Owned current measurement metadata; canonical values remain in the supplied core inventory.
use crate::{
    Landmark, Measurement, MeasurementContext, MeasurementError, MeasurementProcedure,
    MeasurementTable, MeasurementTableContext, MeasurementTableError,
};
use core::fmt;
use sc_core::{ontology::EntityId, value::CanonicalInputs};
use std::collections::BTreeMap;

/// Exact authored metadata vectors; no declaration, numeric value or uncertainty state is copied.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MeasurementInventoryDefinition {
    /// Canonical landmark records in authored order.
    pub landmarks: Vec<Landmark>,
    /// Canonical documented procedures in authored order.
    pub procedures: Vec<MeasurementProcedure>,
    /// Canonical measurements, including unbound draft records.
    pub measurements: Vec<Measurement>,
    /// Complete named table records, including empty drafts.
    pub tables: Vec<MeasurementTable>,
}

/// Original position in the identity census; canonical inputs are borrowed, not owned here.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MeasurementInventoryPosition {
    /// Dedicated canonical length declaration.
    Length {
        /// Zero-based authored position.
        index: usize,
    },
    /// General-scalar canonical declaration, used for identity disjointness only.
    Scalar {
        /// Zero-based authored position.
        index: usize,
    },
    /// Owned landmark.
    Landmark {
        /// Zero-based authored position.
        index: usize,
    },
    /// Owned documented procedure.
    Procedure {
        /// Zero-based authored position.
        index: usize,
    },
    /// Owned measurement metadata.
    Measurement {
        /// Zero-based authored position.
        index: usize,
    },
    /// Owned named table.
    Table {
        /// Zero-based authored position.
        index: usize,
    },
}

/// Whole-inventory refusal, retaining actual owner and original current-reference error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MeasurementInventoryError {
    /// Two metadata/canonical records claim one identity, before reference validation.
    DuplicateIdentity {
        /// Repeated identity.
        identity: EntityId,
        /// First occurrence.
        first: MeasurementInventoryPosition,
        /// First repeated occurrence.
        duplicate: MeasurementInventoryPosition,
    },
    /// Existing borrowed target context refused construction.
    RecordContext(Box<MeasurementError>),
    /// Actual measurement's current required references are invalid.
    Measurement {
        /// Metadata carrying the references.
        measurement: EntityId,
        /// Original typed cause.
        issue: Box<MeasurementError>,
    },
    /// Existing borrowed table context refused construction.
    TableContext(Box<MeasurementTableError>),
    /// Actual table's current bindings are invalid.
    Table {
        /// Table carrying the bindings.
        table: EntityId,
        /// Original typed cause.
        issue: Box<MeasurementTableError>,
    },
}
impl fmt::Display for MeasurementInventoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateIdentity { identity, first, duplicate } => write!(f, "measurement inventory identity {identity} repeats at {duplicate:?}, first supplied at {first:?}"),
            Self::RecordContext(issue) => write!(f, "measurement record context: {issue}"),
            Self::Measurement { measurement, issue } => write!(f, "measurement {measurement}: {issue}"),
            Self::TableContext(issue) => write!(f, "measurement table context: {issue}"),
            Self::Table { table, issue } => write!(f, "measurement table {table}: {issue}"),
        }
    }
}
impl std::error::Error for MeasurementInventoryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::DuplicateIdentity { .. } => None,
            Self::RecordContext(issue) | Self::Measurement { issue, .. } => Some(issue.as_ref()),
            Self::TableContext(issue) | Self::Table { issue, .. } => Some(issue.as_ref()),
        }
    }
}

/// Immutable current metadata inventory, checked against explicit canonical inputs.
///
/// No self-referential contexts are stored. Source/evidence truth, selected Design/table/SizeSet
/// scope, cross-domain names, Ease membership, physical repeatability and readiness remain separate.
///
/// ```compile_fail
/// fn remove(inventory: &mut sc_measure::MeasurementInventory) {
///     inventory.definition.measurements.clear();
/// }
/// ```
/// ```compile_fail
/// fn edit(inventory: &mut sc_measure::MeasurementInventory) {
///     inventory.definition().tables.clear();
/// }
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct MeasurementInventory {
    definition: MeasurementInventoryDefinition,
}
impl fmt::Debug for MeasurementInventory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MeasurementInventory")
            .field("landmark_count", &self.definition.landmarks.len())
            .field("procedure_count", &self.definition.procedures.len())
            .field("measurement_count", &self.definition.measurements.len())
            .field("table_count", &self.definition.tables.len())
            .finish_non_exhaustive()
    }
}
impl MeasurementInventory {
    /// Own exact metadata after complete identity/reference validation against these inputs.
    /// Empty drafts are legal; unused measurements still require their actual current targets.
    ///
    /// # Errors
    /// Returns the first identity or current-reference refusal as described by validate_current.
    pub fn new(
        definition: MeasurementInventoryDefinition,
        inputs: &CanonicalInputs,
    ) -> Result<Self, MeasurementInventoryError> {
        let inventory = Self { definition };
        inventory.validate_current(inputs)?;
        Ok(inventory)
    }

    /// Original authored metadata; shared access grants no mutation or copied scalar state.
    #[must_use]
    pub const fn definition(&self) -> &MeasurementInventoryDefinition {
        &self.definition
    }

    /// Borrow canonical owned measurement metadata by identity.
    #[must_use]
    pub fn measurement(&self, identity: EntityId) -> Option<&Measurement> {
        self.definition
            .measurements
            .iter()
            .find(|measurement| measurement.id() == identity)
    }

    /// Borrow canonical owned table metadata by identity.
    #[must_use]
    pub fn table(&self, identity: EntityId) -> Option<&MeasurementTable> {
        self.definition
            .tables
            .iter()
            .find(|table| table.id() == identity)
    }

    /// Build the existing borrowed target context from actual owned landmarks/procedures and inputs.
    /// This validates that context's identities only; complete current validation is separate.
    /// Keep this returned owner alive while constructing a MeasurementTableContext over its borrow.
    ///
    /// # Errors
    /// Propagates the original MeasurementContext identity refusal without inventing an owner id.
    pub fn record_context<'a>(
        &'a self,
        inputs: &'a CanonicalInputs,
    ) -> Result<MeasurementContext<'a>, MeasurementError> {
        MeasurementContext::new(
            inputs.lengths(),
            &self.definition.landmarks,
            &self.definition.procedures,
        )
    }

    pub(crate) fn identity_positions<'a>(
        &'a self,
        inputs: &'a CanonicalInputs,
    ) -> impl Iterator<Item = (EntityId, MeasurementInventoryPosition)> + 'a {
        inputs
            .lengths()
            .iter()
            .enumerate()
            .map(|(index, value)| (value.id(), MeasurementInventoryPosition::Length { index }))
            .chain(
                inputs.scalars().iter().enumerate().map(|(index, value)| {
                    (value.id(), MeasurementInventoryPosition::Scalar { index })
                }),
            )
            .chain(
                self.definition
                    .landmarks
                    .iter()
                    .enumerate()
                    .map(|(index, value)| {
                        (value.id(), MeasurementInventoryPosition::Landmark { index })
                    }),
            )
            .chain(
                self.definition
                    .procedures
                    .iter()
                    .enumerate()
                    .map(|(index, value)| {
                        (
                            value.id(),
                            MeasurementInventoryPosition::Procedure { index },
                        )
                    }),
            )
            .chain(
                self.definition
                    .measurements
                    .iter()
                    .enumerate()
                    .map(|(index, value)| {
                        (
                            value.id(),
                            MeasurementInventoryPosition::Measurement { index },
                        )
                    }),
            )
            .chain(
                self.definition
                    .tables
                    .iter()
                    .enumerate()
                    .map(|(index, value)| {
                        (value.id(), MeasurementInventoryPosition::Table { index })
                    }),
            )
    }

    /// Validate identities first, then every actual measurement and table in their original order.
    ///
    /// Census order is dedicated lengths, general scalars, landmarks, procedures, measurements,
    /// tables, each in authored order. Source/evidence/procedure/landmark/table binding references
    /// are uses, not extra owned identities. Tokens remain table-scoped. Existing current-reference
    /// methods enforce domains and saved binding identity without reimplementing or weakening them.
    ///
    /// # Errors
    /// Returns the first duplicate, then context/measurement/table refusal with original typed cause.
    pub fn validate_current(
        &self,
        inputs: &CanonicalInputs,
    ) -> Result<(), MeasurementInventoryError> {
        let mut seen = BTreeMap::new();
        for (identity, position) in self.identity_positions(inputs) {
            if let Some(first) = seen.insert(identity, position) {
                return Err(MeasurementInventoryError::DuplicateIdentity {
                    identity,
                    first,
                    duplicate: position,
                });
            }
        }
        let records = self
            .record_context(inputs)
            .map_err(|issue| MeasurementInventoryError::RecordContext(Box::new(issue)))?;
        for measurement in &self.definition.measurements {
            measurement.validate_current(&records).map_err(|issue| {
                MeasurementInventoryError::Measurement {
                    measurement: measurement.id(),
                    issue: Box::new(issue),
                }
            })?;
        }
        let context = MeasurementTableContext::new(&self.definition.measurements, &records)
            .map_err(|issue| MeasurementInventoryError::TableContext(Box::new(issue)))?;
        for table in &self.definition.tables {
            table
                .validate_current(&context)
                .map_err(|issue| MeasurementInventoryError::Table {
                    table: table.id(),
                    issue: Box::new(issue),
                })?;
        }
        Ok(())
    }
}
