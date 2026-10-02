//! Ordered per-POM Ease namespaces over borrowed current canonical inventories.
use crate::{
    Ease, EaseError, EaseSide, MeasurementBinding, MeasurementTable, MeasurementTableContext,
    MeasurementTableError,
};
use core::fmt;
use sc_core::{name::MachineToken, ontology::EntityId};
use std::collections::{BTreeMap, BTreeSet};

/// Saved mapping targets and set-scoped formula spelling; no numeric/state/fit cache.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EaseBinding {
    /// Stable mapping identity.
    pub ease: EntityId,
    /// Exact machine spelling, unique within this set.
    pub token: MachineToken,
    /// Expected body measurement binding.
    pub body: MeasurementBinding,
    /// Expected garment POM binding.
    pub garment: MeasurementBinding,
    /// Sole canonical signed amount identity.
    pub declaration: EntityId,
}
impl EaseBinding {
    /// Capture stable targets from an immutable mapping with an explicitly supplied set token.
    #[must_use]
    pub fn new(token: MachineToken, ease: &Ease) -> Self {
        Self {
            ease: ease.id(),
            token,
            body: ease.definition().body.clone(),
            garment: ease.definition().garment.clone(),
            declaration: ease.definition().declaration,
        }
    }
}
/// Authored order and selected table identities; both sides may use the same mixed table.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EaseSetDefinition {
    /// Stable set identity.
    pub id: EntityId,
    /// Current table owning the mapped body inputs.
    pub body_table: EntityId,
    /// Current table owning the mapped garment POMs.
    pub garment_table: EntityId,
    /// Ordered unique mapping/token/POM inventory; shared body sources are legal.
    pub entries: Vec<EaseBinding>,
}
/// Immutable per-POM mapping namespace; empty drafts do not imply complete POM coverage.
/// ```compile_fail
/// fn change(set: &mut sc_measure::EaseSet) { set.definition.entries.clear(); }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EaseSet {
    definition: EaseSetDefinition,
}
/// Borrowed unambiguous canonical mapping/table inventory and current metadata targets.
/// A supplied inventory is not a current Design-revision certificate or evidence registry.
#[derive(Debug)]
pub struct EaseSetContext<'a> {
    tables: BTreeMap<EntityId, &'a MeasurementTable>,
    eases: BTreeMap<EntityId, &'a Ease>,
    measurements: &'a MeasurementTableContext<'a>,
}
impl<'a> EaseSetContext<'a> {
    /// Reject duplicate or cross-kind record identities before selected lookup.
    /// Does not validate unrelated table contents or mapping targets.
    /// # Errors
    /// Returns [`EaseSetError::DuplicateIdentity`] for any supplied identity collision.
    pub fn new(
        tables: &'a [MeasurementTable],
        eases: &'a [Ease],
        measurements: &'a MeasurementTableContext<'a>,
    ) -> Result<Self, EaseSetError> {
        let mut seen = BTreeSet::new();
        let mut table_inventory = BTreeMap::new();
        let mut ease_inventory = BTreeMap::new();
        for table in tables {
            if measurements.contains(table.id()) || !seen.insert(table.id()) {
                return Err(EaseSetError::DuplicateIdentity(table.id()));
            }
            table_inventory.insert(table.id(), table);
        }
        for ease in eases {
            if measurements.contains(ease.id()) || !seen.insert(ease.id()) {
                return Err(EaseSetError::DuplicateIdentity(ease.id()));
            }
            ease_inventory.insert(ease.id(), ease);
        }
        Ok(Self {
            tables: table_inventory,
            eases: ease_inventory,
            measurements,
        })
    }
    /// Borrow the exact metadata context used for canonical amount queries/evaluation integration.
    #[must_use]
    pub const fn measurements(&self) -> &'a MeasurementTableContext<'a> {
        self.measurements
    }
    pub(crate) fn contains(&self, id: EntityId) -> bool {
        self.tables.contains_key(&id)
            || self.eases.contains_key(&id)
            || self.measurements.contains(id)
    }
    pub(crate) fn current_table(&self, id: EntityId) -> Option<&'a MeasurementTable> {
        self.tables.get(&id).copied()
    }
}
/// Scoped ambiguity, current target or membership refusal; no missing mapping is defaulted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EaseSetError {
    /// Set or inventory identity is reused.
    DuplicateIdentity(EntityId),
    /// A set repeats a machine spelling.
    DuplicateToken {
        /// Selected set.
        set: EntityId,
        /// Repeated token.
        token: MachineToken,
    },
    /// More than one mapping claims the same POM.
    DuplicatePom {
        /// Selected set.
        set: EntityId,
        /// Ambiguous garment measurement identity.
        pom: EntityId,
    },
    /// Selected table identity is absent.
    MissingTable {
        /// Selected set.
        set: EntityId,
        /// Body or garment ownership.
        side: EaseSide,
        /// Missing table identity.
        table: EntityId,
    },
    /// Saved canonical mapping is absent; same-token/POM peers cannot replace it.
    MissingEase {
        /// Selected set.
        set: EntityId,
        /// Missing mapping identity.
        ease: EntityId,
    },
    /// Requested POM has no authored mapping.
    MissingPom {
        /// Selected set.
        set: EntityId,
        /// Unmapped POM identity.
        pom: EntityId,
    },
    /// Requested spelling is not in this set's namespace.
    MissingToken {
        /// Selected set.
        set: EntityId,
        /// Unbound spelling.
        token: MachineToken,
    },
    /// Requested mapping identity is not bound by this set.
    MissingBinding {
        /// Selected set.
        set: EntityId,
        /// Unbound mapping identity.
        ease: EntityId,
    },
    /// Same-id mapping retargeted a measurement binding or its amount declaration.
    ReassignedEase {
        /// Selected set.
        set: EntityId,
        /// Current mapping identity.
        ease: EntityId,
        /// Authored target expectations.
        expected: Box<EaseBinding>,
        /// Current targets, using the same set-owned token.
        actual: Box<EaseBinding>,
    },
    /// Mapped measurement is not a valid current member of its selected table.
    InvalidTableMember {
        /// Selected set.
        set: EntityId,
        /// Selected mapping identity.
        ease: EntityId,
        /// Body or garment table scope.
        side: EaseSide,
        /// Original table/current metadata refusal.
        issue: Box<MeasurementTableError>,
    },
    /// Current canonical mapping fails its own target/amount/permission contract.
    InvalidEase {
        /// Selected set.
        set: EntityId,
        /// Selected mapping identity.
        ease: EntityId,
        /// Exact underlying mapping refusal.
        issue: Box<EaseError>,
    },
}
impl fmt::Display for EaseSetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateIdentity(id) => write!(f, "ease set/context reuses identity {id}"),
            Self::DuplicateToken { set, token } => {
                write!(f, "ease set {set} repeats token {token}")
            }
            Self::DuplicatePom { set, pom } => write!(f, "ease set {set} repeats POM {pom}"),
            Self::MissingTable { set, side, table } => {
                write!(f, "ease set {set} {side:?} table {table} is missing")
            }
            Self::MissingEase { set, ease } => {
                write!(f, "ease set {set} mapping {ease} is missing")
            }
            Self::MissingPom { set, pom } => {
                write!(f, "ease set {set} has no mapping for POM {pom}")
            }
            Self::MissingToken { set, token } => write!(f, "ease set {set} has no token {token}"),
            Self::MissingBinding { set, ease } => {
                write!(f, "ease set {set} has no binding for mapping {ease}")
            }
            Self::ReassignedEase {
                set,
                ease,
                expected,
                actual,
            } => write!(
                f,
                "ease set {set} mapping {ease} reassigned: {expected:?} -> {actual:?}"
            ),
            Self::InvalidTableMember {
                set,
                ease,
                side,
                issue,
            } => write!(f, "ease set {set} mapping {ease} {side:?}: {issue}"),
            Self::InvalidEase { set, ease, issue } => {
                write!(f, "ease set {set} mapping {ease}: {issue}")
            }
        }
    }
}
impl std::error::Error for EaseSetError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidTableMember { issue, .. } => Some(issue.as_ref()),
            Self::InvalidEase { issue, .. } => Some(issue.as_ref()),
            _ => None,
        }
    }
}
impl EaseSet {
    /// Validate unique identities/tokens/POMs, selected tables and all current mapped targets.
    /// # Errors
    /// Returns [`EaseSetError`] for ambiguity, current reassignment or missing/invalid membership.
    pub fn new(
        definition: EaseSetDefinition,
        context: &EaseSetContext<'_>,
    ) -> Result<Self, EaseSetError> {
        let mut identities = BTreeSet::from([definition.id]);
        let mut tokens = BTreeSet::new();
        let mut poms = BTreeSet::new();
        for entry in &definition.entries {
            if !identities.insert(entry.ease) {
                return Err(EaseSetError::DuplicateIdentity(entry.ease));
            }
            if !tokens.insert(&entry.token) {
                return Err(EaseSetError::DuplicateToken {
                    set: definition.id,
                    token: entry.token.clone(),
                });
            }
            if !poms.insert(entry.garment.measurement) {
                return Err(EaseSetError::DuplicatePom {
                    set: definition.id,
                    pom: entry.garment.measurement,
                });
            }
        }
        let set = Self { definition };
        set.validate_current(context)?;
        Ok(set)
    }
    /// Stable set identity.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        self.definition.id
    }
    /// Borrow authored order and targets without unchecked mutation or value/state caches.
    #[must_use]
    pub const fn definition(&self) -> &EaseSetDefinition {
        &self.definition
    }
    fn table<'a>(
        &self,
        side: EaseSide,
        context: &EaseSetContext<'a>,
    ) -> Result<&'a MeasurementTable, EaseSetError> {
        let table = match side {
            EaseSide::Body => self.definition.body_table,
            EaseSide::Garment => self.definition.garment_table,
        };
        context
            .tables
            .get(&table)
            .copied()
            .ok_or(EaseSetError::MissingTable {
                set: self.id(),
                side,
                table,
            })
    }
    fn validate_tables(&self, context: &EaseSetContext<'_>) -> Result<(), EaseSetError> {
        if context.contains(self.id()) {
            return Err(EaseSetError::DuplicateIdentity(self.id()));
        }
        self.table(EaseSide::Body, context)?;
        self.table(EaseSide::Garment, context)?;
        Ok(())
    }
    /// Validate selected table existence and every saved mapped target/current membership.
    /// Empty sets still require their table references; no complete size-chart coverage is inferred.
    /// # Errors
    /// Returns [`EaseSetError`] for current collision, missing/reassigned or invalid targets.
    pub fn validate_current(&self, context: &EaseSetContext<'_>) -> Result<(), EaseSetError> {
        self.validate_tables(context)?;
        for entry in &self.definition.entries {
            self.resolve(entry, context)?;
        }
        Ok(())
    }
    /// Borrow the canonical current mapping for an exact saved POM identity.
    /// Validates the selected mapping/memberships, not unrelated entries or a Design revision.
    /// # Errors
    /// Returns [`EaseSetError`] without a default mapping or peer replacement.
    pub fn ease_for_pom<'a>(
        &self,
        pom: EntityId,
        context: &EaseSetContext<'a>,
    ) -> Result<&'a Ease, EaseSetError> {
        let entry = self
            .definition
            .entries
            .iter()
            .find(|entry| entry.garment.measurement == pom)
            .ok_or(EaseSetError::MissingPom {
                set: self.id(),
                pom,
            })?;
        self.resolve(entry, context)
    }
    /// Borrow the canonical current mapping for an exact set-owned machine token.
    /// # Errors
    /// Returns [`EaseSetError`] for an unbound token or invalid selected current targets.
    pub fn ease_by_token<'a>(
        &self,
        token: &MachineToken,
        context: &EaseSetContext<'a>,
    ) -> Result<&'a Ease, EaseSetError> {
        let entry = self
            .definition
            .entries
            .iter()
            .find(|entry| &entry.token == token)
            .ok_or_else(|| EaseSetError::MissingToken {
                set: self.id(),
                token: token.clone(),
            })?;
        self.resolve(entry, context)
    }
    /// Borrow the canonical current mapping for an exact saved Ease identity.
    /// # Errors
    /// Returns [`EaseSetError`] for an unbound mapping or invalid selected current targets.
    pub fn ease_by_id<'a>(
        &self,
        ease: EntityId,
        context: &EaseSetContext<'a>,
    ) -> Result<&'a Ease, EaseSetError> {
        let entry = self
            .definition
            .entries
            .iter()
            .find(|entry| entry.ease == ease)
            .ok_or(EaseSetError::MissingBinding {
                set: self.id(),
                ease,
            })?;
        self.resolve(entry, context)
    }
    fn resolve<'a>(
        &self,
        entry: &EaseBinding,
        context: &EaseSetContext<'a>,
    ) -> Result<&'a Ease, EaseSetError> {
        self.validate_tables(context)?;
        let ease = context
            .eases
            .get(&entry.ease)
            .copied()
            .ok_or(EaseSetError::MissingEase {
                set: self.id(),
                ease: entry.ease,
            })?;
        let actual = EaseBinding::new(entry.token.clone(), ease);
        if &actual != entry {
            return Err(EaseSetError::ReassignedEase {
                set: self.id(),
                ease: ease.id(),
                expected: Box::new(entry.clone()),
                actual: Box::new(actual),
            });
        }
        for (side, binding) in [
            (EaseSide::Body, &entry.body),
            (EaseSide::Garment, &entry.garment),
        ] {
            self.table(side, context)?
                .measurement_by_id(binding.measurement, context.measurements)
                .map_err(|issue| EaseSetError::InvalidTableMember {
                    set: self.id(),
                    ease: ease.id(),
                    side,
                    issue: Box::new(issue),
                })?;
        }
        ease.validate_current(context.measurements)
            .map_err(|issue| EaseSetError::InvalidEase {
                set: self.id(),
                ease: ease.id(),
                issue: Box::new(issue),
            })?;
        Ok(ease)
    }
}
