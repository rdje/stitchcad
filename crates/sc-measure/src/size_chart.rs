//! Authored garment-chart observations over current member, table and measurement identities.
use crate::{
    Measurement, MeasurementBinding, MeasurementKind, MeasurementTable, MeasurementTableContext,
    MeasurementTableError, SizeMember, SizeMembership, SizeMembershipError, SizeSetReference,
};
use core::fmt;
use sc_core::{
    ontology::EntityId,
    value::{LengthDeclaration, LengthValueError},
};
use sc_units::Length;
use std::collections::{BTreeMap, BTreeSet};

/// Selected garment measurement in an authored chart correspondence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SizeChartRole {
    /// Logical quantity named by the Design's input table, not a regenerated geometry result.
    DesignPom,
    /// Authored chart observation of that quantity for one size member.
    Observation,
}
/// Authored correspondence, retaining reference expectations but no scalar or uncertainty cache.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SizeChartObservationDefinition {
    /// Stable observation identity.
    pub id: EntityId,
    /// Exact owning membership identity and revision.
    pub membership: SizeSetReference,
    /// Stable member identity; neither its label nor its position substitutes for it.
    pub member: EntityId,
    /// Table declaring the Design's named input POMs.
    pub design_table: EntityId,
    /// Table containing this member's authored garment measurements.
    pub chart_table: EntityId,
    /// Expected logical garment POM binding in the Design table.
    pub pom: MeasurementBinding,
    /// Expected authored chart measurement binding in the chart table.
    pub measurement: MeasurementBinding,
    /// Correspondence provenance; source truth and physical equivalence remain separate proofs.
    pub provenance: EntityId,
}
/// Immutable validated chart observation; not chart completeness or instantiation readiness.
/// ```compile_fail
/// fn retarget(value: &mut sc_measure::SizeChartObservation) { value.definition.member = sc_core::ontology::EntityId::from_bits(9); }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SizeChartObservation {
    definition: SizeChartObservationDefinition,
}
/// Borrowed current membership, tables and canonical measurement records.
/// Current global Design revision and correspondence/evidence validity remain caller obligations.
#[derive(Debug)]
pub struct SizeChartContext<'a> {
    membership: &'a SizeMembership,
    tables: BTreeMap<EntityId, &'a MeasurementTable>,
    measurements: &'a MeasurementTableContext<'a>,
}
impl<'a> SizeChartContext<'a> {
    /// Reject ambiguous table/set/member/measurement identities before reference lookup.
    /// Unrelated table entries are not fully validated by context construction.
    /// # Errors
    /// Returns [`SizeChartError::DuplicateIdentity`] for any supplied identity collision.
    pub fn new(
        membership: &'a SizeMembership,
        tables: &'a [MeasurementTable],
        measurements: &'a MeasurementTableContext<'a>,
    ) -> Result<Self, SizeChartError> {
        let mut identities = BTreeSet::from([membership.reference().id]);
        identities.extend(
            membership
                .definition()
                .members
                .iter()
                .map(|member| member.id),
        );
        for &id in &identities {
            if measurements.contains(id) {
                return Err(SizeChartError::DuplicateIdentity(id));
            }
        }
        let mut inventory = BTreeMap::new();
        for table in tables {
            let id = table.id();
            if measurements.contains(id) || !identities.insert(id) {
                return Err(SizeChartError::DuplicateIdentity(id));
            }
            inventory.insert(id, table);
        }
        Ok(Self {
            membership,
            tables: inventory,
            measurements,
        })
    }
    fn contains(&self, id: EntityId) -> bool {
        self.membership.reference().id == id
            || self
                .membership
                .definition()
                .members
                .iter()
                .any(|member| member.id == id)
            || self.tables.contains_key(&id)
            || self.measurements.contains(id)
    }
}
/// Scoped current-reference/domain/value refusal for an authored chart observation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SizeChartError {
    /// Identity aliases a different supplied semantic object or repeats in a context.
    DuplicateIdentity(EntityId),
    /// An authored role uses a body measurement instead of a garment POM.
    WrongKind {
        /// Selected observation.
        observation: EntityId,
        /// Authored role.
        role: SizeChartRole,
        /// Refused measurement domain.
        actual: MeasurementKind,
    },
    /// Supplied current membership differs from the observation's pinned reference.
    MembershipMismatch {
        /// Selected observation.
        observation: EntityId,
        /// Authored set identity and revision.
        expected: SizeSetReference,
        /// Supplied set identity and revision.
        actual: SizeSetReference,
    },
    /// Stable member identity is absent; a same-label replacement never substitutes for it.
    InvalidMember {
        /// Selected observation.
        observation: EntityId,
        /// Exact membership lookup refusal.
        issue: SizeMembershipError,
    },
    /// A named table is absent, even if another table has matching entries.
    MissingTable {
        /// Selected observation.
        observation: EntityId,
        /// Role requiring this table.
        role: SizeChartRole,
        /// Missing table identity.
        table: EntityId,
    },
    /// A selected current table binding or its required metadata target is invalid.
    InvalidTable {
        /// Selected observation.
        observation: EntityId,
        /// Selected role.
        role: SizeChartRole,
        /// Exact underlying table/measurement refusal.
        issue: Box<MeasurementTableError>,
    },
    /// A current table explicitly rebound metadata but this observation still pins the old binding.
    ReassignedBinding {
        /// Selected observation.
        observation: EntityId,
        /// Selected role.
        role: SizeChartRole,
        /// Authored expectation.
        expected: Box<MeasurementBinding>,
        /// Current canonical binding.
        actual: Box<MeasurementBinding>,
    },
    /// The chart value requires observation/evaluation; no numeric fallback is supplied.
    UnavailableValue {
        /// Selected observation.
        observation: EntityId,
        /// Exact unresolved value refusal.
        issue: LengthValueError,
    },
}
impl fmt::Display for SizeChartError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateIdentity(id) => write!(f, "size chart/context reuses identity {id}"),
            Self::WrongKind { observation, role, actual } => write!(f, "chart observation {observation} {role:?} has wrong kind {actual:?}"),
            Self::MembershipMismatch { observation, expected, actual } => write!(f, "chart observation {observation} expects membership {expected:?}, found {actual:?}"),
            Self::InvalidMember { observation, issue } => write!(f, "chart observation {observation}: {issue}"),
            Self::MissingTable { observation, role, table } => write!(f, "chart observation {observation} {role:?} table {table} is missing"),
            Self::InvalidTable { observation, role, issue } => write!(f, "chart observation {observation} {role:?}: {issue}"),
            Self::ReassignedBinding { observation, role, expected, actual } => write!(f, "chart observation {observation} {role:?} binding reassigned: {expected:?} -> {actual:?}"),
            Self::UnavailableValue { observation, issue } => write!(f, "chart observation {observation}: {issue}"),
        }
    }
}
impl std::error::Error for SizeChartError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidMember { issue, .. } => Some(issue),
            Self::InvalidTable { issue, .. } => Some(issue.as_ref()),
            Self::UnavailableValue { issue, .. } => Some(issue),
            _ => None,
        }
    }
}
impl SizeChartObservation {
    /// Validate garment domains and all required current member/table/measurement targets.
    /// Unknown/derived chart drafts are inspectable; a numeric value is not required here.
    /// # Errors
    /// Returns [`SizeChartError`] for identity, domain, revision or current-reference failures.
    pub fn new(
        definition: SizeChartObservationDefinition,
        context: &SizeChartContext<'_>,
    ) -> Result<Self, SizeChartError> {
        for (role, binding) in [
            (SizeChartRole::DesignPom, &definition.pom),
            (SizeChartRole::Observation, &definition.measurement),
        ] {
            if binding.kind != MeasurementKind::Garment {
                return Err(SizeChartError::WrongKind {
                    observation: definition.id,
                    role,
                    actual: binding.kind,
                });
            }
        }
        let observation = Self { definition };
        observation.validate_current(context)?;
        Ok(observation)
    }
    /// Stable authored observation identity.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        self.definition.id
    }
    /// Borrow authored references without unchecked mutation or value/state duplication.
    #[must_use]
    pub const fn definition(&self) -> &SizeChartObservationDefinition {
        &self.definition
    }
    /// Borrow the exact current member after checking observation identity and pinned set revision.
    /// # Errors
    /// Returns [`SizeChartError`] for an alias, different set/revision or absent member identity.
    pub fn member<'a>(
        &self,
        context: &SizeChartContext<'a>,
    ) -> Result<&'a SizeMember, SizeChartError> {
        if context.contains(self.id()) {
            return Err(SizeChartError::DuplicateIdentity(self.id()));
        }
        if self.definition.membership != context.membership.reference() {
            return Err(SizeChartError::MembershipMismatch {
                observation: self.id(),
                expected: self.definition.membership,
                actual: context.membership.reference(),
            });
        }
        context
            .membership
            .member(self.definition.member)
            .map_err(|issue| SizeChartError::InvalidMember {
                observation: self.id(),
                issue,
            })
    }
    /// Borrow the selected current metadata after member, named table and saved binding checks.
    /// Does not certify the other role, unrelated table entries or correspondence truth.
    /// # Errors
    /// Returns [`SizeChartError`] for any required selected reference or metadata failure.
    pub fn measurement<'a>(
        &self,
        role: SizeChartRole,
        context: &SizeChartContext<'a>,
    ) -> Result<&'a Measurement, SizeChartError> {
        self.member(context)?;
        let (table_id, expected) = match role {
            SizeChartRole::DesignPom => (self.definition.design_table, &self.definition.pom),
            SizeChartRole::Observation => {
                (self.definition.chart_table, &self.definition.measurement)
            }
        };
        let table = context
            .tables
            .get(&table_id)
            .ok_or(SizeChartError::MissingTable {
                observation: self.id(),
                role,
                table: table_id,
            })?;
        let measurement = table
            .measurement_by_id(expected.measurement, context.measurements)
            .map_err(|issue| SizeChartError::InvalidTable {
                observation: self.id(),
                role,
                issue: Box::new(issue),
            })?;
        let actual = MeasurementBinding::from(measurement);
        if &actual != expected {
            return Err(SizeChartError::ReassignedBinding {
                observation: self.id(),
                role,
                expected: Box::new(expected.clone()),
                actual: Box::new(actual),
            });
        }
        Ok(measurement)
    }
    /// Check both selected current bindings and their required metadata, not entire table inventories.
    /// # Errors
    /// Returns [`SizeChartError`] for any required current-reference failure.
    pub fn validate_current(&self, context: &SizeChartContext<'_>) -> Result<(), SizeChartError> {
        self.measurement(SizeChartRole::DesignPom, context)?;
        self.measurement(SizeChartRole::Observation, context)?;
        Ok(())
    }
    /// Borrow chart value/state/source after validating both required correspondence roles.
    /// The logical Design POM may share this input; generated geometry measurements remain distinct.
    /// # Errors
    /// Returns [`SizeChartError`] for any required member, table, binding or metadata failure.
    pub fn declaration<'a>(
        &self,
        context: &SizeChartContext<'a>,
    ) -> Result<&'a LengthDeclaration, SizeChartError> {
        self.measurement(SizeChartRole::DesignPom, context)?;
        let measurement = self.measurement(SizeChartRole::Observation, context)?;
        measurement
            .declaration(context.measurements.records())
            .map_err(|issue| SizeChartError::InvalidTable {
                observation: self.id(),
                role: SizeChartRole::Observation,
                issue: Box::new(MeasurementTableError::InvalidMeasurement {
                    table: self.definition.chart_table,
                    measurement: measurement.id(),
                    issue,
                }),
            })
    }
    /// Read only a present authored canonical chart value without arithmetic or substitution.
    /// # Errors
    /// Returns [`SizeChartError`] for current references or required observation/evaluation.
    pub fn authored_value(&self, context: &SizeChartContext<'_>) -> Result<Length, SizeChartError> {
        self.declaration(context)?
            .authored_value()
            .map_err(|issue| SizeChartError::UnavailableValue {
                observation: self.id(),
                issue,
            })
    }
}
