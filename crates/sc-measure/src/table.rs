//! Stable table bindings over borrowed current metadata and canonical scalar inputs.
use crate::{Measurement, MeasurementContext, MeasurementError, MeasurementKind};
use core::fmt;
use sc_core::{name::MachineToken, ontology::EntityId, value::LengthDeclaration};
use std::collections::{BTreeMap, BTreeSet};

/// Expected identity and domain of one authored table input; no numeric/state cache.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MeasurementBinding {
    /// Stable measurement identity; never inferred from a matching peer's token.
    pub measurement: EntityId,
    /// Exact stable spelling used by this table's API and recipe inputs.
    pub token: MachineToken,
    /// Expected body-measurement or garment-POM domain.
    pub kind: MeasurementKind,
    /// Sole canonical scalar identity; reassignment needs an explicit new binding.
    pub declaration: EntityId,
}
impl From<&Measurement> for MeasurementBinding {
    fn from(measurement: &Measurement) -> Self {
        let definition = measurement.definition();
        Self {
            measurement: measurement.id(),
            token: definition.token.clone(),
            kind: definition.kind,
            declaration: definition.declaration,
        }
    }
}

/// Named, ordered authored table inventory. Empty drafts are legal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MeasurementTableDefinition {
    /// Stable semantic table identity.
    pub id: EntityId,
    /// Nonblank human-readable name; never a substitute for identity or token.
    pub name: String,
    /// Authored entries, unique by measurement identity and exact token within this table.
    pub entries: Vec<MeasurementBinding>,
}

/// Immutable named bindings with explicit current-reference validation.
/// ```compile_fail
/// fn retarget(table: &mut sc_measure::MeasurementTable) {
///     table.definition.entries.clear();
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MeasurementTable {
    definition: MeasurementTableDefinition,
}

/// Unambiguous borrowed current measurements and their canonical target records.
/// Token uniqueness belongs to each table, not to unrelated tables' shared inventory.
/// The caller still owns current Design revision and global source/evidence validation.
#[derive(Debug)]
pub struct MeasurementTableContext<'a> {
    measurements: BTreeMap<EntityId, &'a Measurement>,
    records: &'a MeasurementContext<'a>,
}
impl<'a> MeasurementTableContext<'a> {
    /// Validate unique metadata identities across supplied measurements and target records.
    /// # Errors
    /// Returns [`MeasurementTableError::DuplicateIdentity`] before lookup on an identity collision.
    pub fn new(
        measurements: &'a [Measurement],
        records: &'a MeasurementContext<'a>,
    ) -> Result<Self, MeasurementTableError> {
        let mut inventory = BTreeMap::new();
        for measurement in measurements {
            let id = measurement.id();
            if records.contains(id) || inventory.insert(id, measurement).is_some() {
                return Err(MeasurementTableError::DuplicateIdentity(id));
            }
        }
        Ok(Self {
            measurements: inventory,
            records,
        })
    }
    /// Borrow the same target context for selected metadata's declaration/landmark/procedure queries.
    #[must_use]
    pub const fn records(&self) -> &'a MeasurementContext<'a> {
        self.records
    }
    pub(crate) fn current_measurement(&self, id: EntityId) -> Option<&'a Measurement> {
        self.measurements.get(&id).copied()
    }
    pub(crate) fn contains(&self, id: EntityId) -> bool {
        self.measurements.contains_key(&id) || self.records.contains(id)
    }
}

/// Structural table/binding refusal; values, factual truth and release policy remain distinct.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MeasurementTableError {
    /// A table needs a nonblank human-readable name.
    EmptyName(EntityId),
    /// An identity is reused by the table, its entries or supplied canonical inventories.
    DuplicateIdentity(EntityId),
    /// Two entries in this table bind the same machine spelling.
    DuplicateToken {
        /// Table whose namespace is ambiguous.
        table: EntityId,
        /// Repeated spelling.
        token: MachineToken,
    },
    /// A token is not bound by this table, even if another supplied measurement uses it.
    MissingToken {
        /// Selected table.
        table: EntityId,
        /// Requested spelling.
        token: MachineToken,
    },
    /// A measurement identity is not bound by this table.
    MissingBinding {
        /// Selected table.
        table: EntityId,
        /// Requested measurement identity.
        measurement: EntityId,
    },
    /// An authored entry's current measurement is absent; peers never replace it.
    MissingMeasurement {
        /// Table holding the binding.
        table: EntityId,
        /// Missing canonical metadata identity.
        measurement: EntityId,
    },
    /// Current same-id metadata changed the binding's stable machine spelling.
    TokenMismatch {
        /// Table holding the binding.
        table: EntityId,
        /// Current metadata identity.
        measurement: EntityId,
        /// Authored spelling.
        expected: MachineToken,
        /// Current spelling.
        actual: MachineToken,
    },
    /// Current same-id metadata changed body measurement into POM or vice versa.
    KindMismatch {
        /// Table holding the binding.
        table: EntityId,
        /// Current metadata identity.
        measurement: EntityId,
        /// Authored domain.
        expected: MeasurementKind,
        /// Current domain.
        actual: MeasurementKind,
    },
    /// Current same-id metadata reassigned the scalar's canonical identity.
    DeclarationMismatch {
        /// Table holding the binding.
        table: EntityId,
        /// Current metadata identity.
        measurement: EntityId,
        /// Authored canonical scalar.
        expected: EntityId,
        /// Current canonical scalar.
        actual: EntityId,
    },
    /// The selected current metadata fails its own required identity/target/domain checks.
    InvalidMeasurement {
        /// Table holding the binding.
        table: EntityId,
        /// Selected metadata identity.
        measurement: EntityId,
        /// Exact underlying structural refusal.
        issue: MeasurementError,
    },
}
impl fmt::Display for MeasurementTableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName(id) => write!(f, "measurement table {id} needs a name"),
            Self::DuplicateIdentity(id) => write!(f, "measurement table/context reuses identity {id}"),
            Self::DuplicateToken { table, token } => write!(f, "table {table} repeats input token {token}"),
            Self::MissingToken { table, token } => write!(f, "table {table} has no input token {token}"),
            Self::MissingBinding { table, measurement } => write!(f, "table {table} has no binding for measurement {measurement}"),
            Self::MissingMeasurement { table, measurement } => write!(f, "table {table} names missing measurement {measurement}"),
            Self::TokenMismatch { table, measurement, expected, actual } => write!(f, "table {table} expects measurement {measurement} token {expected}, found {actual}"),
            Self::KindMismatch { table, measurement, expected, actual } => write!(f, "table {table} expects measurement {measurement} kind {expected:?}, found {actual:?}"),
            Self::DeclarationMismatch { table, measurement, expected, actual } => write!(f, "table {table} expects measurement {measurement} declaration {expected}, found {actual}"),
            Self::InvalidMeasurement { table, measurement, issue } => write!(f, "table {table} measurement {measurement}: {issue}"),
        }
    }
}
impl std::error::Error for MeasurementTableError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidMeasurement { issue, .. } => Some(issue),
            _ => None,
        }
    }
}

impl MeasurementTable {
    /// Validate table name, unique authored inventory and every required current metadata target.
    /// # Errors
    /// Returns [`MeasurementTableError`] for invalid content, ambiguity or current reassignment.
    pub fn new(
        definition: MeasurementTableDefinition,
        context: &MeasurementTableContext<'_>,
    ) -> Result<Self, MeasurementTableError> {
        if definition.name.trim().is_empty() {
            return Err(MeasurementTableError::EmptyName(definition.id));
        }
        let mut identities = BTreeSet::from([definition.id]);
        let mut tokens = BTreeSet::new();
        for entry in &definition.entries {
            if !identities.insert(entry.measurement) {
                return Err(MeasurementTableError::DuplicateIdentity(entry.measurement));
            }
            if !tokens.insert(&entry.token) {
                return Err(MeasurementTableError::DuplicateToken {
                    table: definition.id,
                    token: entry.token.clone(),
                });
            }
        }
        let table = Self { definition };
        table.validate_current(context)?;
        Ok(table)
    }
    /// Stable semantic table identity.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        self.definition.id
    }
    /// Borrow authored content without unchecked mutation or numeric/state duplication.
    #[must_use]
    pub const fn definition(&self) -> &MeasurementTableDefinition {
        &self.definition
    }
    /// Validate every saved binding against supplied current metadata and target records.
    /// # Errors
    /// Returns [`MeasurementTableError`] for collision, missing metadata or changed/invalid targets.
    pub fn validate_current(
        &self,
        context: &MeasurementTableContext<'_>,
    ) -> Result<(), MeasurementTableError> {
        self.validate_identity(context)?;
        for binding in &self.definition.entries {
            self.resolve(binding, context)?;
        }
        Ok(())
    }
    /// Borrow the current measurement selected by this table's exact saved token.
    /// Validates the selected binding/metadata, not the whole table or a Design revision.
    /// # Errors
    /// Returns [`MeasurementTableError`] without selecting an unrelated same-token peer.
    pub fn measurement<'a>(
        &self,
        token: &MachineToken,
        context: &MeasurementTableContext<'a>,
    ) -> Result<&'a Measurement, MeasurementTableError> {
        let binding = self
            .definition
            .entries
            .iter()
            .find(|entry| &entry.token == token)
            .ok_or_else(|| MeasurementTableError::MissingToken {
                table: self.id(),
                token: token.clone(),
            })?;
        self.resolve(binding, context)
    }
    /// Borrow current metadata by its saved measurement identity; inventory order is irrelevant.
    /// # Errors
    /// Returns [`MeasurementTableError`] for an unbound identity or invalid current binding/metadata.
    pub fn measurement_by_id<'a>(
        &self,
        measurement: EntityId,
        context: &MeasurementTableContext<'a>,
    ) -> Result<&'a Measurement, MeasurementTableError> {
        let binding = self
            .definition
            .entries
            .iter()
            .find(|entry| entry.measurement == measurement)
            .ok_or(MeasurementTableError::MissingBinding {
                table: self.id(),
                measurement,
            })?;
        self.resolve(binding, context)
    }
    /// Borrow the selected input's current canonical value/state/source, never a cached number.
    /// # Errors
    /// Returns [`MeasurementTableError`] for any selected binding or required metadata-target failure.
    pub fn declaration<'a>(
        &self,
        token: &MachineToken,
        context: &MeasurementTableContext<'a>,
    ) -> Result<&'a LengthDeclaration, MeasurementTableError> {
        let measurement = self.measurement(token, context)?;
        measurement.declaration(context.records).map_err(|issue| {
            MeasurementTableError::InvalidMeasurement {
                table: self.id(),
                measurement: measurement.id(),
                issue,
            }
        })
    }
    fn validate_identity(
        &self,
        context: &MeasurementTableContext<'_>,
    ) -> Result<(), MeasurementTableError> {
        if context.contains(self.id()) {
            return Err(MeasurementTableError::DuplicateIdentity(self.id()));
        }
        Ok(())
    }
    fn resolve<'a>(
        &self,
        binding: &MeasurementBinding,
        context: &MeasurementTableContext<'a>,
    ) -> Result<&'a Measurement, MeasurementTableError> {
        self.validate_identity(context)?;
        let measurement = context
            .measurements
            .get(&binding.measurement)
            .copied()
            .ok_or(MeasurementTableError::MissingMeasurement {
                table: self.id(),
                measurement: binding.measurement,
            })?;
        let current = measurement.definition();
        if current.token != binding.token {
            return Err(MeasurementTableError::TokenMismatch {
                table: self.id(),
                measurement: measurement.id(),
                expected: binding.token.clone(),
                actual: current.token.clone(),
            });
        }
        if current.kind != binding.kind {
            return Err(MeasurementTableError::KindMismatch {
                table: self.id(),
                measurement: measurement.id(),
                expected: binding.kind,
                actual: current.kind,
            });
        }
        if current.declaration != binding.declaration {
            return Err(MeasurementTableError::DeclarationMismatch {
                table: self.id(),
                measurement: measurement.id(),
                expected: binding.declaration,
                actual: current.declaration,
            });
        }
        measurement
            .validate_current(context.records)
            .map_err(|issue| MeasurementTableError::InvalidMeasurement {
                table: self.id(),
                measurement: measurement.id(),
                issue,
            })?;
        Ok(measurement)
    }
}
