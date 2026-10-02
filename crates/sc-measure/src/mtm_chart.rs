//! Single-member custom body-input charts, with explicit canonical Ease correspondence.
use crate::{
    Ease, EaseError, EaseSet, EaseSetContext, EaseSetDefinition, EaseSetError, EaseSide,
    Measurement, MeasurementTableError, SizeMember, SizeMembership, SizeMembershipError,
    SizeSetReference, SizeSystem,
};
use core::fmt;
use sc_core::{
    ontology::EntityId,
    value::{LengthDeclaration, LengthValueError},
};
use sc_units::Length;
use std::collections::{BTreeMap, BTreeSet};

/// Authored MTM correspondence; values, state and fit remain canonical rather than copied here.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MtmChartDefinition {
    /// Stable chart identity.
    pub id: EntityId,
    /// Exact expected owning set identity and revision.
    pub membership: SizeSetReference,
    /// Stable sole/base member identity, not its label or position.
    pub member: EntityId,
    /// Expected canonical Ease-set identity, table identities and ordered mapping targets.
    /// This is a saved reference snapshot, not an embedded authoritative Ease set or numeric cache.
    pub ease_set: EaseSetDefinition,
    /// Chart/body correspondence provenance; truth/scope validation is a later obligation.
    pub provenance: EntityId,
}
/// Immutable custom-member-of-one body chart with explicit structural coverage queries.
/// ```compile_fail
/// fn retarget(chart: &mut sc_measure::MtmChart) { chart.definition.member = sc_core::ontology::EntityId::from_bits(9); }
/// ```
/// Body and Ease inputs do not become a garment numeric result through this API.
/// ```compile_fail
/// fn unconverted(chart: &sc_measure::MtmChart, context: &sc_measure::MtmChartContext<'_>, pom: sc_core::ontology::EntityId) {
///     let _ = chart.authored_value(pom, context);
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MtmChart {
    definition: MtmChartDefinition,
}
/// Unambiguous borrowed membership, current Ease sets and their canonical target context.
#[derive(Debug)]
pub struct MtmChartContext<'a> {
    membership: &'a SizeMembership,
    sets: BTreeMap<EntityId, &'a EaseSet>,
    eases: &'a EaseSetContext<'a>,
}
impl<'a> MtmChartContext<'a> {
    /// Refuse ambiguous set/member/Ease-set/target identities before lookup.
    /// Does not validate unrelated Ease-set contents or current global Design revision.
    /// # Errors
    /// Returns [`MtmChartError::DuplicateIdentity`] for a duplicate or cross-kind identity.
    pub fn new(
        membership: &'a SizeMembership,
        sets: &'a [EaseSet],
        eases: &'a EaseSetContext<'a>,
    ) -> Result<Self, MtmChartError> {
        let mut identities = BTreeSet::from([membership.reference().id]);
        identities.extend(
            membership
                .definition()
                .members
                .iter()
                .map(|member| member.id),
        );
        for &id in &identities {
            if eases.contains(id) {
                return Err(MtmChartError::DuplicateIdentity(id));
            }
        }
        let mut inventory = BTreeMap::new();
        for set in sets {
            if eases.contains(set.id()) || !identities.insert(set.id()) {
                return Err(MtmChartError::DuplicateIdentity(set.id()));
            }
            inventory.insert(set.id(), set);
        }
        Ok(Self {
            membership,
            sets: inventory,
            eases,
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
            || self.sets.contains_key(&id)
            || self.eases.contains(id)
    }
}
/// Scoped membership, canonical correspondence, numeric-input or path refusal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MtmChartError {
    /// An identity repeats or aliases a different supplied semantic object.
    DuplicateIdentity(EntityId),
    /// MTM requires custom designation intent.
    WrongSystem {
        /// Selected chart.
        chart: EntityId,
        /// Refused designation system.
        actual: SizeSystem,
    },
    /// MTM requires exactly one existing member, whose base identity is that member.
    WrongMemberCount {
        /// Selected chart.
        chart: EntityId,
        /// Actual supplied member count.
        actual: usize,
    },
    /// Supplied owning membership differs from the chart's pinned reference.
    MembershipMismatch {
        /// Selected chart.
        chart: EntityId,
        /// Authored reference.
        expected: SizeSetReference,
        /// Supplied reference.
        actual: SizeSetReference,
    },
    /// Authored sole member identity is absent; labels never substitute for it.
    InvalidMember {
        /// Selected chart.
        chart: EntityId,
        /// Exact underlying lookup refusal.
        issue: SizeMembershipError,
    },
    /// Canonical Ease-set identity is absent; same-content peers never replace it.
    MissingEaseSet {
        /// Selected chart.
        chart: EntityId,
        /// Missing canonical set.
        ease_set: EntityId,
    },
    /// Current Ease set retargeted a table, mapping, token, binding, amount or authored order.
    ReassignedEaseSet {
        /// Selected chart.
        chart: EntityId,
        /// Authored reference targets.
        expected: Box<EaseSetDefinition>,
        /// Current canonical targets.
        actual: Box<EaseSetDefinition>,
    },
    /// Current selected/all Ease-set bindings or table memberships are invalid.
    InvalidEaseSet {
        /// Selected chart.
        chart: EntityId,
        /// Canonical set.
        ease_set: EntityId,
        /// Exact mapping/set refusal.
        issue: Box<EaseSetError>,
    },
    /// Current selected mapping metadata, amount or compression permission is invalid.
    InvalidEase {
        /// Selected chart.
        chart: EntityId,
        /// Selected mapping.
        ease: EntityId,
        /// Exact mapping refusal.
        issue: Box<EaseError>,
    },
    /// Current Design table is not wholly valid for the completeness query.
    InvalidDesignTable {
        /// Selected chart.
        chart: EntityId,
        /// Exact table/metadata refusal.
        issue: Box<MeasurementTableError>,
    },
    /// Body input needs observation/evaluation; no garment value or zero substitutes for it.
    UnavailableBodyValue {
        /// Selected chart.
        chart: EntityId,
        /// Target garment POM whose body input is unresolved.
        pom: EntityId,
        /// Exact unresolved body input.
        issue: LengthValueError,
    },
    /// An empty mapping inventory cannot establish a complete MTM input chart.
    EmptyMappings(EntityId),
    /// An MTM chart of one is a regeneration input and cannot supply grade rules.
    GradeRulesUnsupported {
        /// Authored chart.
        chart: EntityId,
        /// Authored sole member.
        member: EntityId,
    },
}
impl fmt::Display for MtmChartError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateIdentity(id) => write!(f, "MTM chart/context reuses identity {id}"),
            Self::WrongSystem { chart, actual } => {
                write!(f, "MTM chart {chart} requires Custom, found {actual:?}")
            }
            Self::WrongMemberCount { chart, actual } => {
                write!(f, "MTM chart {chart} requires one member, found {actual}")
            }
            Self::MembershipMismatch {
                chart,
                expected,
                actual,
            } => write!(
                f,
                "MTM chart {chart} expects membership {expected:?}, found {actual:?}"
            ),
            Self::InvalidMember { chart, issue } => write!(f, "MTM chart {chart}: {issue}"),
            Self::MissingEaseSet { chart, ease_set } => {
                write!(f, "MTM chart {chart} Ease set {ease_set} is missing")
            }
            Self::ReassignedEaseSet {
                chart,
                expected,
                actual,
            } => write!(
                f,
                "MTM chart {chart} Ease set reassigned: {expected:?} -> {actual:?}"
            ),
            Self::InvalidEaseSet {
                chart,
                ease_set,
                issue,
            } => write!(f, "MTM chart {chart} Ease set {ease_set}: {issue}"),
            Self::InvalidEase { chart, ease, issue } => {
                write!(f, "MTM chart {chart} Ease {ease}: {issue}")
            }
            Self::InvalidDesignTable { chart, issue } => {
                write!(f, "MTM chart {chart} Design table: {issue}")
            }
            Self::UnavailableBodyValue { chart, pom, issue } => {
                write!(f, "MTM chart {chart} POM {pom} body: {issue}")
            }
            Self::EmptyMappings(chart) => write!(
                f,
                "MTM chart {chart} needs body/Ease mappings for completeness"
            ),
            Self::GradeRulesUnsupported { chart, member } => write!(
                f,
                "MTM chart {chart} member {member} cannot supply grade rules"
            ),
        }
    }
}
impl std::error::Error for MtmChartError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidMember { issue, .. } => Some(issue),
            Self::InvalidEaseSet { issue, .. } => Some(issue.as_ref()),
            Self::InvalidEase { issue, .. } => Some(issue.as_ref()),
            Self::InvalidDesignTable { issue, .. } => Some(issue.as_ref()),
            Self::UnavailableBodyValue { issue, .. } => Some(issue),
            _ => None,
        }
    }
}
impl MtmChart {
    /// Validate custom single-member scope and every authored current body/Ease correspondence.
    /// Empty/partial and unresolved drafts remain inspectable; completeness is an explicit query.
    /// # Errors
    /// Returns [`MtmChartError`] for identity, member/system/reference or current mapping failures.
    pub fn new(
        definition: MtmChartDefinition,
        context: &MtmChartContext<'_>,
    ) -> Result<Self, MtmChartError> {
        let chart = Self { definition };
        chart.validate_current(context)?;
        Ok(chart)
    }
    /// Stable chart identity.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        self.definition.id
    }
    /// Borrow saved reference expectations and chart provenance without unchecked mutation.
    #[must_use]
    pub const fn definition(&self) -> &MtmChartDefinition {
        &self.definition
    }
    /// Borrow the exact sole/base member after identity, revision, system and cardinality checks.
    /// Does not validate the Ease set or source truth.
    /// # Errors
    /// Returns [`MtmChartError`] for a collision, different reference/system or missing sole member.
    pub fn member<'a>(
        &self,
        context: &MtmChartContext<'a>,
    ) -> Result<&'a SizeMember, MtmChartError> {
        if context.contains(self.id()) {
            return Err(MtmChartError::DuplicateIdentity(self.id()));
        }
        if self.definition.membership != context.membership.reference() {
            return Err(MtmChartError::MembershipMismatch {
                chart: self.id(),
                expected: self.definition.membership,
                actual: context.membership.reference(),
            });
        }
        let membership = context.membership.definition();
        if membership.system != SizeSystem::Custom {
            return Err(MtmChartError::WrongSystem {
                chart: self.id(),
                actual: membership.system,
            });
        }
        if membership.members.len() != 1 {
            return Err(MtmChartError::WrongMemberCount {
                chart: self.id(),
                actual: membership.members.len(),
            });
        }
        context
            .membership
            .member(self.definition.member)
            .map_err(|issue| MtmChartError::InvalidMember {
                chart: self.id(),
                issue,
            })
    }
    /// Borrow the exact canonical Ease set after member and saved target-snapshot checks.
    /// Does not validate every mapping; selected/full validation is separate.
    /// # Errors
    /// Returns [`MtmChartError`] for scope, missing set or retargeted set/table/mapping expectations.
    pub fn ease_set<'a>(
        &self,
        context: &MtmChartContext<'a>,
    ) -> Result<&'a EaseSet, MtmChartError> {
        self.member(context)?;
        let set = context
            .sets
            .get(&self.definition.ease_set.id)
            .copied()
            .ok_or(MtmChartError::MissingEaseSet {
                chart: self.id(),
                ease_set: self.definition.ease_set.id,
            })?;
        if set.definition() != &self.definition.ease_set {
            return Err(MtmChartError::ReassignedEaseSet {
                chart: self.id(),
                expected: Box::new(self.definition.ease_set.clone()),
                actual: Box::new(set.definition().clone()),
            });
        }
        Ok(set)
    }
    fn invalid_set(&self, issue: EaseSetError) -> MtmChartError {
        MtmChartError::InvalidEaseSet {
            chart: self.id(),
            ease_set: self.definition.ease_set.id,
            issue: Box::new(issue),
        }
    }
    fn invalid_ease(&self, ease: EntityId, issue: EaseError) -> MtmChartError {
        MtmChartError::InvalidEase {
            chart: self.id(),
            ease,
            issue: Box::new(issue),
        }
    }
    /// Validate every authored current mapping/amount, without certifying omitted POM coverage.
    /// # Errors
    /// Returns [`MtmChartError`] for scope, reassignment or required current mapping/metadata failures.
    pub fn validate_current(&self, context: &MtmChartContext<'_>) -> Result<(), MtmChartError> {
        self.ease_set(context)?
            .validate_current(context.eases)
            .map_err(|issue| self.invalid_set(issue))
    }
    /// Check nonempty mappings covering every garment POM in the valid current Design table.
    /// Numeric availability, body-plus-Ease evaluation, physical fit and path execution are separate.
    /// # Errors
    /// Returns [`MtmChartError`] for current failures, empty mappings or unmapped Design POMs.
    pub fn validate_complete(&self, context: &MtmChartContext<'_>) -> Result<(), MtmChartError> {
        self.validate_current(context)?;
        let set = self.ease_set(context)?;
        if set.definition().entries.is_empty() {
            return Err(MtmChartError::EmptyMappings(self.id()));
        }
        let table = context
            .eases
            .current_table(set.definition().garment_table)
            .ok_or_else(|| {
                self.invalid_set(EaseSetError::MissingTable {
                    set: set.id(),
                    side: EaseSide::Garment,
                    table: set.definition().garment_table,
                })
            })?;
        table
            .validate_current(context.eases.measurements())
            .map_err(|issue| MtmChartError::InvalidDesignTable {
                chart: self.id(),
                issue: Box::new(issue),
            })?;
        for binding in &table.definition().entries {
            if binding.kind == crate::MeasurementKind::Garment {
                self.ease_for_pom(binding.measurement, context)?;
            }
        }
        Ok(())
    }
    /// Borrow one exact current body-to-garment mapping and validate both table memberships/amount.
    /// Does not certify other mappings or complete Design coverage.
    /// # Errors
    /// Returns [`MtmChartError`] without selecting a peer or inventing an unmapped POM's inputs.
    pub fn ease_for_pom<'a>(
        &self,
        pom: EntityId,
        context: &MtmChartContext<'a>,
    ) -> Result<&'a Ease, MtmChartError> {
        self.ease_set(context)?
            .ease_for_pom(pom, context.eases)
            .map_err(|issue| self.invalid_set(issue))
    }
    /// Borrow the logical garment target metadata; it is not an evaluated garment result.
    /// # Errors
    /// Returns [`MtmChartError`] for the selected current correspondence or metadata failure.
    pub fn garment_pom<'a>(
        &self,
        pom: EntityId,
        context: &MtmChartContext<'a>,
    ) -> Result<&'a Measurement, MtmChartError> {
        let ease = self.ease_for_pom(pom, context)?;
        ease.measurement(EaseSide::Garment, context.eases.measurements())
            .map_err(|issue| self.invalid_ease(ease.id(), issue))
    }
    /// Borrow the selected canonical body value/state/source, retaining Body metadata semantics.
    /// # Errors
    /// Returns [`MtmChartError`] for current scope, correspondence, membership or metadata failures.
    pub fn body_declaration<'a>(
        &self,
        pom: EntityId,
        context: &MtmChartContext<'a>,
    ) -> Result<&'a LengthDeclaration, MtmChartError> {
        let ease = self.ease_for_pom(pom, context)?;
        let measurement = ease
            .measurement(EaseSide::Body, context.eases.measurements())
            .map_err(|issue| self.invalid_ease(ease.id(), issue))?;
        measurement
            .declaration(context.eases.measurements().records())
            .map_err(|issue| {
                self.invalid_ease(
                    ease.id(),
                    EaseError::InvalidMeasurement {
                        ease: ease.id(),
                        side: EaseSide::Body,
                        issue,
                    },
                )
            })
    }
    /// Borrow the current signed Ease amount/state/source separately from the body value.
    /// # Errors
    /// Returns [`MtmChartError`] for current mapping/reference or compression permission failures.
    pub fn ease_declaration<'a>(
        &self,
        pom: EntityId,
        context: &MtmChartContext<'a>,
    ) -> Result<&'a LengthDeclaration, MtmChartError> {
        let ease = self.ease_for_pom(pom, context)?;
        ease.declaration(context.eases.measurements())
            .map_err(|issue| self.invalid_ease(ease.id(), issue))
    }
    /// Read only a present authored body input, without converting it to a garment value.
    /// # Errors
    /// Returns [`MtmChartError`] for current references or required body observation/evaluation.
    pub fn body_value(
        &self,
        pom: EntityId,
        context: &MtmChartContext<'_>,
    ) -> Result<Length, MtmChartError> {
        self.body_declaration(pom, context)?
            .authored_value()
            .map_err(|issue| MtmChartError::UnavailableBodyValue {
                chart: self.id(),
                pom,
                issue,
            })
    }
    /// Read only a present signed Ease input, without evaluating body-plus-Ease or geometry.
    /// # Errors
    /// Returns [`MtmChartError`] for current references, unresolved amount or compression refusal.
    pub fn ease_value(
        &self,
        pom: EntityId,
        context: &MtmChartContext<'_>,
    ) -> Result<Length, MtmChartError> {
        let ease = self.ease_for_pom(pom, context)?;
        ease.authored_value(context.eases.measurements())
            .map_err(|issue| self.invalid_ease(ease.id(), issue))
    }
    /// Explicitly refuse grade-rule input for this authored MTM chart; no currentness is certified.
    /// Regeneration must separately validate coverage, resolve inputs and evaluate the recipe.
    /// # Errors
    /// Always returns [`MtmChartError::GradeRulesUnsupported`].
    pub const fn validate_grade_rule_input(&self) -> Result<(), MtmChartError> {
        Err(MtmChartError::GradeRulesUnsupported {
            chart: self.id(),
            member: self.definition.member,
        })
    }
}
