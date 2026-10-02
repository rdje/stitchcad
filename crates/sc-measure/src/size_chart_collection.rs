//! Explicit per-member/POM coverage over canonical current garment observations.
use crate::{
    MeasurementBinding, MeasurementKind, MeasurementTable, MeasurementTableError, SizeChartContext,
    SizeChartError, SizeChartObservation, SizeMember, SizeMembershipError, SizeSetReference,
};
use core::fmt;
use sc_core::{name::MachineToken, ontology::EntityId, value::LengthDeclaration};
use sc_units::Length;
use std::collections::{BTreeMap, BTreeSet};

/// Saved observation targets; current value/state/source and correspondence provenance are not cached.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SizeChartBinding {
    /// Canonical observation identity.
    pub observation: EntityId,
    /// Exact expected set identity and revision.
    pub membership: SizeSetReference,
    /// Expected stable member identity.
    pub member: EntityId,
    /// Expected named Design input table.
    pub design_table: EntityId,
    /// Expected named chart measurement table.
    pub chart_table: EntityId,
    /// Expected logical garment POM binding.
    pub pom: MeasurementBinding,
    /// Expected authored chart measurement binding.
    pub measurement: MeasurementBinding,
}
impl From<&SizeChartObservation> for SizeChartBinding {
    fn from(observation: &SizeChartObservation) -> Self {
        let definition = observation.definition();
        Self {
            observation: observation.id(),
            membership: definition.membership,
            member: definition.member,
            design_table: definition.design_table,
            chart_table: definition.chart_table,
            pom: definition.pom.clone(),
            measurement: definition.measurement.clone(),
        }
    }
}
/// Authored target order and observation inventory; partial drafts carry no completeness claim.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SizeChartDefinition {
    /// Stable chart identity, separate from its set, members and observations.
    pub id: EntityId,
    /// Pinned owning set identity and revision.
    pub membership: SizeSetReference,
    /// Named Design table whose garment POMs define complete coverage.
    pub design_table: EntityId,
    /// Explicit ordered garment POM targets; an empty or partial inventory is a draft.
    pub poms: Vec<MeasurementBinding>,
    /// Unique observation identities and member/POM cells, in authored inventory order.
    pub observations: Vec<SizeChartBinding>,
}
/// Immutable authored chart; structural completeness is an explicit current query.
/// ```compile_fail
/// fn clear(chart: &mut sc_measure::SizeChart) { chart.definition.observations.clear(); }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SizeChart {
    definition: SizeChartDefinition,
}
/// Borrowed current observation inventory and the member/table/measurement context it resolves in.
#[derive(Debug)]
pub struct SizeChartCollectionContext<'a> {
    observations: BTreeMap<EntityId, &'a SizeChartObservation>,
    records: &'a SizeChartContext<'a>,
}
impl<'a> SizeChartCollectionContext<'a> {
    /// Refuse duplicate or cross-kind identities; unrelated observations are not fully validated.
    /// # Errors
    /// Returns [`SizeChartCollectionError::DuplicateIdentity`] for an ambiguous identity.
    pub fn new(
        observations: &'a [SizeChartObservation],
        records: &'a SizeChartContext<'a>,
    ) -> Result<Self, SizeChartCollectionError> {
        let mut inventory = BTreeMap::new();
        for observation in observations {
            if records.contains(observation.id())
                || inventory.insert(observation.id(), observation).is_some()
            {
                return Err(SizeChartCollectionError::DuplicateIdentity(
                    observation.id(),
                ));
            }
        }
        Ok(Self {
            observations: inventory,
            records,
        })
    }
    fn contains(&self, id: EntityId) -> bool {
        self.observations.contains_key(&id) || self.records.contains(id)
    }
}
/// Scoped chart ambiguity, current-target or structural coverage refusal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SizeChartCollectionError {
    /// Chart/context/entry identity repeats or aliases a different supplied record.
    DuplicateIdentity(EntityId),
    /// Target inventory repeats one logical POM identity.
    DuplicatePom {
        /// Selected chart.
        chart: EntityId,
        /// Repeated POM.
        pom: EntityId,
    },
    /// Target inventory repeats one machine spelling.
    DuplicateToken {
        /// Selected chart.
        chart: EntityId,
        /// Repeated token.
        token: MachineToken,
    },
    /// Two distinct authored observations claim one member/POM cell.
    DuplicateCell {
        /// Selected chart.
        chart: EntityId,
        /// Ambiguous member.
        member: EntityId,
        /// Ambiguous POM.
        pom: EntityId,
    },
    /// An authored target is not a garment POM.
    WrongKind {
        /// Selected chart.
        chart: EntityId,
        /// Refused POM.
        pom: EntityId,
        /// Actual domain.
        actual: MeasurementKind,
    },
    /// Supplied membership differs from the chart's exact pinned reference.
    MembershipMismatch {
        /// Selected chart.
        chart: EntityId,
        /// Authored set identity/revision.
        expected: SizeSetReference,
        /// Supplied set identity/revision.
        actual: SizeSetReference,
    },
    /// Queried stable member is absent, without label/position substitution.
    InvalidMember {
        /// Selected chart.
        chart: EntityId,
        /// Exact member lookup refusal.
        issue: SizeMembershipError,
    },
    /// Named Design table is absent.
    MissingTable {
        /// Selected chart.
        chart: EntityId,
        /// Missing table identity.
        table: EntityId,
    },
    /// A selected binding or the complete named Design table is invalid.
    InvalidTable {
        /// Selected chart.
        chart: EntityId,
        /// Exact table/metadata refusal.
        issue: Box<MeasurementTableError>,
    },
    /// Requested or observed POM is not in the authored target inventory.
    MissingPom {
        /// Selected chart.
        chart: EntityId,
        /// Missing target.
        pom: EntityId,
    },
    /// Current canonical POM differs from the chart's saved binding.
    ReassignedPom {
        /// Selected chart.
        chart: EntityId,
        /// Authored binding.
        expected: Box<MeasurementBinding>,
        /// Current binding.
        actual: Box<MeasurementBinding>,
    },
    /// No authored observation binds this member/POM cell.
    MissingCell {
        /// Selected chart.
        chart: EntityId,
        /// Member without this input.
        member: EntityId,
        /// Required POM.
        pom: EntityId,
    },
    /// Saved observation identity is absent; a same-cell peer never replaces it.
    MissingObservation {
        /// Selected chart.
        chart: EntityId,
        /// Missing observation identity.
        observation: EntityId,
    },
    /// Canonical observation retargeted any saved member/table/measurement reference.
    ReassignedObservation {
        /// Selected chart.
        chart: EntityId,
        /// Authored target snapshot.
        expected: Box<SizeChartBinding>,
        /// Current canonical target snapshot.
        actual: Box<SizeChartBinding>,
    },
    /// Canonical observation refers to a different Design table.
    WrongDesignTable {
        /// Selected chart.
        chart: EntityId,
        /// Selected observation.
        observation: EntityId,
        /// Chart's authored table.
        expected: EntityId,
        /// Observation's table.
        actual: EntityId,
    },
    /// Selected canonical observation fails its required current metadata/value contract.
    InvalidObservation {
        /// Selected chart.
        chart: EntityId,
        /// Selected observation.
        observation: EntityId,
        /// Exact observation/currentness/value refusal.
        issue: Box<SizeChartError>,
    },
    /// An empty POM inventory cannot establish complete garment chart coverage.
    EmptyPoms(EntityId),
}
impl fmt::Display for SizeChartCollectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateIdentity(id) => write!(f, "chart/context reuses identity {id}"),
            Self::DuplicatePom { chart, pom } => write!(f, "chart {chart} repeats POM {pom}"),
            Self::DuplicateToken { chart, token } => write!(f, "chart {chart} repeats token {token}"),
            Self::DuplicateCell { chart, member, pom } => write!(f, "chart {chart} repeats member {member} POM {pom}"),
            Self::WrongKind { chart, pom, actual } => write!(f, "chart {chart} POM {pom} has wrong kind {actual:?}"),
            Self::MembershipMismatch { chart, expected, actual } => write!(f, "chart {chart} expects membership {expected:?}, found {actual:?}"),
            Self::InvalidMember { chart, issue } => write!(f, "chart {chart}: {issue}"),
            Self::MissingTable { chart, table } => write!(f, "chart {chart} Design table {table} is missing"),
            Self::InvalidTable { chart, issue } => write!(f, "chart {chart}: {issue}"),
            Self::MissingPom { chart, pom } => write!(f, "chart {chart} has no target POM {pom}"),
            Self::ReassignedPom { chart, expected, actual } => write!(f, "chart {chart} POM reassigned: {expected:?} -> {actual:?}"),
            Self::MissingCell { chart, member, pom } => write!(f, "chart {chart} member {member} needs POM {pom}"),
            Self::MissingObservation { chart, observation } => write!(f, "chart {chart} observation {observation} is missing"),
            Self::ReassignedObservation { chart, expected, actual } => write!(f, "chart {chart} observation reassigned: {expected:?} -> {actual:?}"),
            Self::WrongDesignTable { chart, observation, expected, actual } => write!(f, "chart {chart} observation {observation} expects Design table {expected}, found {actual}"),
            Self::InvalidObservation { chart, observation, issue } => write!(f, "chart {chart} observation {observation}: {issue}"),
            Self::EmptyPoms(chart) => write!(f, "chart {chart} needs garment POM targets for completeness"),
        }
    }
}
impl std::error::Error for SizeChartCollectionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidMember { issue, .. } => Some(issue),
            Self::InvalidTable { issue, .. } => Some(issue.as_ref()),
            Self::InvalidObservation { issue, .. } => Some(issue.as_ref()),
            _ => None,
        }
    }
}
impl SizeChart {
    /// Validate unique POMs/tokens/observations/cells and all authored current targets.
    /// Allows incomplete drafts; call [`Self::validate_complete`] for full structural coverage.
    /// # Errors
    /// Returns [`SizeChartCollectionError`] for ambiguity, invalid domains or current targets.
    pub fn new(
        definition: SizeChartDefinition,
        context: &SizeChartCollectionContext<'_>,
    ) -> Result<Self, SizeChartCollectionError> {
        let mut identities = BTreeSet::from([definition.id]);
        let mut poms = BTreeSet::new();
        let mut tokens = BTreeSet::new();
        for pom in &definition.poms {
            if pom.kind != MeasurementKind::Garment {
                return Err(SizeChartCollectionError::WrongKind {
                    chart: definition.id,
                    pom: pom.measurement,
                    actual: pom.kind,
                });
            }
            if !poms.insert(pom.measurement) {
                return Err(SizeChartCollectionError::DuplicatePom {
                    chart: definition.id,
                    pom: pom.measurement,
                });
            }
            if !tokens.insert(&pom.token) {
                return Err(SizeChartCollectionError::DuplicateToken {
                    chart: definition.id,
                    token: pom.token.clone(),
                });
            }
        }
        let mut cells = BTreeSet::new();
        for entry in &definition.observations {
            if !identities.insert(entry.observation) {
                return Err(SizeChartCollectionError::DuplicateIdentity(
                    entry.observation,
                ));
            }
            if !cells.insert((entry.member, entry.pom.measurement)) {
                return Err(SizeChartCollectionError::DuplicateCell {
                    chart: definition.id,
                    member: entry.member,
                    pom: entry.pom.measurement,
                });
            }
        }
        let chart = Self { definition };
        chart.validate_current(context)?;
        Ok(chart)
    }
    /// Stable chart identity.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        self.definition.id
    }
    /// Borrow authored inventory without unchecked mutation or scalar/state caching.
    #[must_use]
    pub const fn definition(&self) -> &SizeChartDefinition {
        &self.definition
    }
    fn table<'a>(
        &self,
        context: &SizeChartCollectionContext<'a>,
    ) -> Result<&'a MeasurementTable, SizeChartCollectionError> {
        if context.contains(self.id()) {
            return Err(SizeChartCollectionError::DuplicateIdentity(self.id()));
        }
        let actual = context.records.membership().reference();
        if actual != self.definition.membership {
            return Err(SizeChartCollectionError::MembershipMismatch {
                chart: self.id(),
                expected: self.definition.membership,
                actual,
            });
        }
        context.records.table(self.definition.design_table).ok_or(
            SizeChartCollectionError::MissingTable {
                chart: self.id(),
                table: self.definition.design_table,
            },
        )
    }
    fn pom(&self, pom: EntityId) -> Result<&MeasurementBinding, SizeChartCollectionError> {
        self.definition
            .poms
            .iter()
            .find(|entry| entry.measurement == pom)
            .ok_or(SizeChartCollectionError::MissingPom {
                chart: self.id(),
                pom,
            })
    }
    fn validate_pom(
        &self,
        binding: &MeasurementBinding,
        context: &SizeChartCollectionContext<'_>,
    ) -> Result<(), SizeChartCollectionError> {
        let measurement = self
            .table(context)?
            .measurement_by_id(binding.measurement, context.records.measurements())
            .map_err(|issue| SizeChartCollectionError::InvalidTable {
                chart: self.id(),
                issue: Box::new(issue),
            })?;
        let actual = MeasurementBinding::from(measurement);
        if &actual != binding {
            return Err(SizeChartCollectionError::ReassignedPom {
                chart: self.id(),
                expected: Box::new(binding.clone()),
                actual: Box::new(actual),
            });
        }
        Ok(())
    }
    /// Validate all authored current references, without claiming missing cells are present.
    /// # Errors
    /// Returns [`SizeChartCollectionError`] for any authored target/currentness failure.
    pub fn validate_current(
        &self,
        context: &SizeChartCollectionContext<'_>,
    ) -> Result<(), SizeChartCollectionError> {
        self.table(context)?;
        for pom in &self.definition.poms {
            self.validate_pom(pom, context)?;
        }
        for entry in &self.definition.observations {
            self.resolve(entry, context)?;
        }
        Ok(())
    }
    /// Check nonempty exact Design garment-POM coverage and one cell per member/POM.
    /// Unknown/derived inputs may be structurally complete; numeric/path/release readiness is separate.
    /// # Errors
    /// Returns [`SizeChartCollectionError`] for current failures, empty targets or missing POM/cell.
    pub fn validate_complete(
        &self,
        context: &SizeChartCollectionContext<'_>,
    ) -> Result<(), SizeChartCollectionError> {
        self.validate_current(context)?;
        if self.definition.poms.is_empty() {
            return Err(SizeChartCollectionError::EmptyPoms(self.id()));
        }
        let table = self.table(context)?;
        table
            .validate_current(context.records.measurements())
            .map_err(|issue| SizeChartCollectionError::InvalidTable {
                chart: self.id(),
                issue: Box::new(issue),
            })?;
        for binding in &table.definition().entries {
            if binding.kind == MeasurementKind::Garment {
                self.pom(binding.measurement)?;
            }
        }
        let cells: BTreeSet<_> = self
            .definition
            .observations
            .iter()
            .map(|entry| (entry.member, entry.pom.measurement))
            .collect();
        for member in self.members(context)? {
            for pom in &self.definition.poms {
                if !cells.contains(&(member.id, pom.measurement)) {
                    return Err(SizeChartCollectionError::MissingCell {
                        chart: self.id(),
                        member: member.id,
                        pom: pom.measurement,
                    });
                }
            }
        }
        Ok(())
    }
    /// Borrow membership in authored instantiation order after exact reference and chart-id checks.
    /// Does not certify chart coverage or other records.
    /// # Errors
    /// Returns [`SizeChartCollectionError`] for identity, membership or named Design-table failures.
    pub fn members<'a>(
        &self,
        context: &SizeChartCollectionContext<'a>,
    ) -> Result<&'a [SizeMember], SizeChartCollectionError> {
        self.table(context)?;
        Ok(&context.records.membership().definition().members)
    }
    /// Borrow the exact canonical observation for a stable member/POM cell.
    /// Checks selected targets, not other rows or completeness.
    /// # Errors
    /// Returns [`SizeChartCollectionError`] without a default observation or peer substitution.
    pub fn observation<'a>(
        &self,
        member: EntityId,
        pom: EntityId,
        context: &SizeChartCollectionContext<'a>,
    ) -> Result<&'a SizeChartObservation, SizeChartCollectionError> {
        self.table(context)?;
        context
            .records
            .membership()
            .member(member)
            .map_err(|issue| SizeChartCollectionError::InvalidMember {
                chart: self.id(),
                issue,
            })?;
        self.validate_pom(self.pom(pom)?, context)?;
        let entry = self
            .definition
            .observations
            .iter()
            .find(|entry| entry.member == member && entry.pom.measurement == pom)
            .ok_or(SizeChartCollectionError::MissingCell {
                chart: self.id(),
                member,
                pom,
            })?;
        self.resolve(entry, context)
    }
    /// Borrow one member's authored target observations in declared POM order, refusing missing cells.
    /// Does not imply exact full Design-table coverage or validate unrelated rows.
    /// # Errors
    /// Returns [`SizeChartCollectionError`] for an absent member/cell or selected current-target failure.
    pub fn observations_for_member<'a>(
        &self,
        member: EntityId,
        context: &SizeChartCollectionContext<'a>,
    ) -> Result<Vec<&'a SizeChartObservation>, SizeChartCollectionError> {
        self.table(context)?;
        context
            .records
            .membership()
            .member(member)
            .map_err(|issue| SizeChartCollectionError::InvalidMember {
                chart: self.id(),
                issue,
            })?;
        self.definition
            .poms
            .iter()
            .map(|pom| self.observation(member, pom.measurement, context))
            .collect()
    }
    /// Borrow the current selected chart value/state/source after all required selected target checks.
    /// # Errors
    /// Returns [`SizeChartCollectionError`] for missing/stale cells or required metadata targets.
    pub fn declaration<'a>(
        &self,
        member: EntityId,
        pom: EntityId,
        context: &SizeChartCollectionContext<'a>,
    ) -> Result<&'a LengthDeclaration, SizeChartCollectionError> {
        let observation = self.observation(member, pom, context)?;
        observation
            .declaration(context.records)
            .map_err(|issue| self.invalid_observation(observation.id(), issue))
    }
    /// Read a present canonical authored cell value; never interpolates, converts or defaults it.
    /// # Errors
    /// Returns [`SizeChartCollectionError`] for current references or required observation/evaluation.
    pub fn authored_value(
        &self,
        member: EntityId,
        pom: EntityId,
        context: &SizeChartCollectionContext<'_>,
    ) -> Result<Length, SizeChartCollectionError> {
        let observation = self.observation(member, pom, context)?;
        observation
            .authored_value(context.records)
            .map_err(|issue| self.invalid_observation(observation.id(), issue))
    }
    fn invalid_observation(
        &self,
        observation: EntityId,
        issue: SizeChartError,
    ) -> SizeChartCollectionError {
        SizeChartCollectionError::InvalidObservation {
            chart: self.id(),
            observation,
            issue: Box::new(issue),
        }
    }
    fn resolve<'a>(
        &self,
        entry: &SizeChartBinding,
        context: &SizeChartCollectionContext<'a>,
    ) -> Result<&'a SizeChartObservation, SizeChartCollectionError> {
        self.table(context)?;
        let observation = context
            .observations
            .get(&entry.observation)
            .copied()
            .ok_or(SizeChartCollectionError::MissingObservation {
                chart: self.id(),
                observation: entry.observation,
            })?;
        let actual = SizeChartBinding::from(observation);
        if &actual != entry {
            return Err(SizeChartCollectionError::ReassignedObservation {
                chart: self.id(),
                expected: Box::new(entry.clone()),
                actual: Box::new(actual),
            });
        }
        if actual.design_table != self.definition.design_table {
            return Err(SizeChartCollectionError::WrongDesignTable {
                chart: self.id(),
                observation: observation.id(),
                expected: self.definition.design_table,
                actual: actual.design_table,
            });
        }
        let expected = self.pom(actual.pom.measurement)?;
        if &actual.pom != expected {
            return Err(SizeChartCollectionError::ReassignedPom {
                chart: self.id(),
                expected: Box::new(expected.clone()),
                actual: Box::new(actual.pom),
            });
        }
        self.validate_pom(expected, context)?;
        observation
            .validate_current(context.records)
            .map_err(|issue| self.invalid_observation(observation.id(), issue))?;
        Ok(observation)
    }
}
