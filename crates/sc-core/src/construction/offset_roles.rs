//! Declared local role identities and canonical membership;not geometric matching or issuance authority.
use crate::{
    name::MachineToken,
    ontology::{EntityId, LocalTag},
};
use core::fmt;
use std::collections::BTreeMap;
mod transition;
pub use transition::{
    AdmittedOffsetRoleTransition, OffsetRoleTransitionError, OffsetRoleTransitionLimits,
    OffsetRoleTransitionRefusal,
};

/// One explicit local machine key/tag declaration,not a curve classifier or allocated identity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OffsetRoleDefinition {
    tag: LocalTag,
    key: MachineToken,
}
impl OffsetRoleDefinition {
    /// Retain the actual validated machine key and explicit local tag without normalization.
    #[must_use]
    pub const fn new(tag: LocalTag, key: MachineToken) -> Self {
        Self { tag, key }
    }
    /// Explicit local tag,not a vector position.
    #[must_use]
    pub const fn tag(&self) -> LocalTag {
        self.tag
    }
    /// Actual authored local machine key;not a global recipe binding or geometric matching rule.
    #[must_use]
    pub const fn key(&self) -> &MachineToken {
        &self.key
    }
}
/// Explicit complete catalogue allowances,without default admission or prior-allocation guarantees.
/// ```compile_fail
/// let limits:sc_core::construction::OffsetRoleCatalogLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OffsetRoleCatalogLimits {
    definitions: usize,
    active: usize,
    key_bytes: usize,
}
impl OffsetRoleCatalogLimits {
    /// Select complete definition,active-occurrence and total machine-key byte allowances.
    #[must_use]
    pub const fn new(definitions: usize, active: usize, key_bytes: usize) -> Self {
        Self {
            definitions,
            active,
            key_bytes,
        }
    }
    /// Complete definition allowance,including inactive issued-role declarations.
    #[must_use]
    pub const fn definitions(self) -> usize {
        self.definitions
    }
    /// Complete supplied active-id occurrence allowance.
    #[must_use]
    pub const fn active(self) -> usize {
        self.active
    }
    /// Complete actual machine-key byte allowance.
    #[must_use]
    pub const fn key_bytes(self) -> usize {
        self.key_bytes
    }
}
/// Complete count/text admission or first original declaration/membership cause.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OffsetRoleCatalogRefusal {
    /// The two N-entry plus two A-entry workspace populations cannot be counted.
    WorkspaceOverflow,
    /// Complete definition population exceeds policy.
    Definitions {
        /// Complete required population.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete active-id population exceeds policy.
    Active {
        /// Complete required population.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete machine-key bytes cannot be counted.
    KeyBytesOverflow,
    /// Complete actual key bytes exceed policy.
    KeyBytes {
        /// Complete required byte count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Two original definitions use the same explicit tag.
    DuplicateTag {
        /// Original first definition index.
        first: usize,
        /// Original repeated definition index.
        second: usize,
    },
    /// Two original definitions use the same exact local machine key.
    DuplicateKey {
        /// Original first definition index.
        first: usize,
        /// Original repeated definition index.
        second: usize,
    },
    /// Active membership repeats an original id occurrence.
    DuplicateActive {
        /// Original first active-list index.
        first: usize,
        /// Original repeated active-list index.
        second: usize,
    },
    /// Active membership names no supplied definition.
    UndefinedActive {
        /// Actual original active-list index.
        index: usize,
        /// Actual required explicit tag.
        tag: LocalTag,
    },
}
/// Every original input,selected policy and typed cause remain inspectable after refusal.
pub struct OffsetRoleCatalogError<'a> {
    catalog: &'a OffsetRoleCatalog,
    limits: OffsetRoleCatalogLimits,
    refusal: OffsetRoleCatalogRefusal,
}
impl<'a> OffsetRoleCatalogError<'a> {
    /// Actual complete original catalogue,including all unaccepted fields.
    #[must_use]
    pub const fn catalog(&self) -> &'a OffsetRoleCatalog {
        self.catalog
    }
    /// Original explicit resource policy.
    #[must_use]
    pub const fn limits(&self) -> OffsetRoleCatalogLimits {
        self.limits
    }
    /// Complete resource or first original metadata cause.
    #[must_use]
    pub const fn refusal(&self) -> OffsetRoleCatalogRefusal {
        self.refusal
    }
}
impl fmt::Debug for OffsetRoleCatalogError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OffsetRoleCatalogError")
            .field("operation", &self.catalog.operation)
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for OffsetRoleCatalogError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "offset role catalogue admission: {:?}", self.refusal)
    }
}
impl std::error::Error for OffsetRoleCatalogError<'_> {}

/// Owned unadmitted declared-role inventory and explicit canonical active membership.
/// All issued definitions,including inactive ones,belong here;cross-edit completeness and actual
/// issuance/history/current Design truth require their separate owners. Empty initial input is valid.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OffsetRoleCatalog {
    operation: EntityId,
    definitions: Vec<OffsetRoleDefinition>,
    active: Vec<LocalTag>,
}
impl OffsetRoleCatalog {
    /// Retain all original fields in both authored orders without admitting or issuing them.
    #[must_use]
    pub const fn new(
        operation: EntityId,
        definitions: Vec<OffsetRoleDefinition>,
        active: Vec<LocalTag>,
    ) -> Self {
        Self {
            operation,
            definitions,
            active,
        }
    }
    /// Required canonical owner identity,without global ownership/authority proof.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        self.operation
    }
    /// Complete original role definitions,including those not canonically active.
    #[must_use]
    pub fn definitions(&self) -> &[OffsetRoleDefinition] {
        &self.definitions
    }
    /// Complete original canonical active-id list,not evaluated-instance presence.
    #[must_use]
    pub fn active_tags(&self) -> &[LocalTag] {
        &self.active
    }
    /// Check complete N/A/workspace counts,definition/active quotas and total key bytes before indexes.
    /// Then inspect definitions in original order (tag before key),followed by original active ids
    /// (duplicate before undefined). No sorting,renaming,default role or accepted prefix escapes.
    /// Peak populations are two N-entry indexes and two A-entry membership collections;these counts
    /// grant no prior String/Vec allocation,whole-allocation-byte or geometry/execution certificate.
    /// # Errors
    /// Complete count/quota/text refusal,then first original duplicate/undefined metadata cause.
    pub fn admit(
        &self,
        limits: OffsetRoleCatalogLimits,
    ) -> Result<AdmittedOffsetRoleCatalog<'_>, OffsetRoleCatalogError<'_>> {
        let refuse = |refusal| OffsetRoleCatalogError {
            catalog: self,
            limits,
            refusal,
        };
        let n = self.definitions.len();
        let a = self.active.len();
        let workspace_entries = n
            .checked_add(a)
            .and_then(|total| total.checked_mul(2))
            .ok_or_else(|| refuse(OffsetRoleCatalogRefusal::WorkspaceOverflow))?;
        if n > limits.definitions {
            return Err(refuse(OffsetRoleCatalogRefusal::Definitions {
                required: n,
                allowed: limits.definitions,
            }));
        }
        if a > limits.active {
            return Err(refuse(OffsetRoleCatalogRefusal::Active {
                required: a,
                allowed: limits.active,
            }));
        }
        let key_bytes = self
            .definitions
            .iter()
            .try_fold(0usize, |sum, role| sum.checked_add(role.key.as_str().len()))
            .ok_or_else(|| refuse(OffsetRoleCatalogRefusal::KeyBytesOverflow))?;
        if key_bytes > limits.key_bytes {
            return Err(refuse(OffsetRoleCatalogRefusal::KeyBytes {
                required: key_bytes,
                allowed: limits.key_bytes,
            }));
        }
        let mut by_tag = BTreeMap::new();
        let mut by_key = BTreeMap::new();
        for (index, role) in self.definitions.iter().enumerate() {
            if let Some((first, _)) = by_tag.insert(role.tag, (index, role)) {
                return Err(refuse(OffsetRoleCatalogRefusal::DuplicateTag {
                    first,
                    second: index,
                }));
            }
            if let Some(first) = by_key.insert(role.key.as_str(), index) {
                return Err(refuse(OffsetRoleCatalogRefusal::DuplicateKey {
                    first,
                    second: index,
                }));
            }
        }
        let mut active_by_tag = BTreeMap::new();
        let mut active_roles = Vec::with_capacity(a);
        for (index, tag) in self.active.iter().enumerate() {
            if let Some(first) = active_by_tag.insert(*tag, index) {
                return Err(refuse(OffsetRoleCatalogRefusal::DuplicateActive {
                    first,
                    second: index,
                }));
            }
            let (_, definition) = by_tag.get(tag).copied().ok_or_else(|| {
                refuse(OffsetRoleCatalogRefusal::UndefinedActive { index, tag: *tag })
            })?;
            active_roles.push(ActiveAssociation {
                index,
                tag,
                definition,
            });
        }
        Ok(AdmittedOffsetRoleCatalog {
            catalog: self,
            limits,
            key_bytes,
            workspace_entries,
            by_tag,
            active_by_tag,
            active_roles,
        })
    }
}
#[derive(Debug)]
struct ActiveAssociation<'a> {
    index: usize,
    tag: &'a LocalTag,
    definition: &'a OffsetRoleDefinition,
}

/// Complete structural catalogue admission,not issuance history,global namespace or geometric truth.
/// ```compile_fail
/// fn escape(catalog:sc_core::construction::OffsetRoleCatalog)->sc_core::construction::AdmittedOffsetRoleCatalog<'static> {
///     catalog.admit(sc_core::construction::OffsetRoleCatalogLimits::new(0,0,0)).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct AdmittedOffsetRoleCatalog<'a> {
    catalog: &'a OffsetRoleCatalog,
    limits: OffsetRoleCatalogLimits,
    key_bytes: usize,
    workspace_entries: usize,
    by_tag: BTreeMap<LocalTag, (usize, &'a OffsetRoleDefinition)>,
    active_by_tag: BTreeMap<LocalTag, usize>,
    active_roles: Vec<ActiveAssociation<'a>>,
}
impl<'a> AdmittedOffsetRoleCatalog<'a> {
    /// Actual complete original catalogue and both original field orders.
    #[must_use]
    pub const fn catalog(&self) -> &'a OffsetRoleCatalog {
        self.catalog
    }
    /// Selected original admission policy.
    #[must_use]
    pub const fn limits(&self) -> OffsetRoleCatalogLimits {
        self.limits
    }
    /// Complete actual machine-key bytes,not an approved memory footprint.
    #[must_use]
    pub const fn key_bytes(&self) -> usize {
        self.key_bytes
    }
    /// Complete peak index/membership entry populations,not allocation bytes.
    #[must_use]
    pub const fn workspace_entries(&self) -> usize {
        self.workspace_entries
    }
    /// Actual original definition for an explicit tag,including inactive definitions.
    #[must_use]
    pub fn definition(&self, tag: LocalTag) -> Option<&'a OffsetRoleDefinition> {
        self.by_tag.get(&tag).map(|(_, definition)| *definition)
    }
    /// Canonical active membership only,not presence or liveness in an evaluated geometry instance.
    #[must_use]
    pub fn is_active(&self, tag: LocalTag) -> bool {
        self.active_by_tag.contains_key(&tag)
    }
    /// Every actual active membership/definition pair,in original active-list order.
    pub fn active_roles(
        &self,
    ) -> impl ExactSizeIterator<Item = ActiveOffsetRole<'_>> + DoubleEndedIterator {
        self.active_roles
            .iter()
            .map(move |association| ActiveOffsetRole {
                catalog: self,
                association,
            })
    }
}
/// Authentic original membership tag and its actual definition,without cloned metadata.
/// ```compile_fail
/// fn reparent<'a>(mut role:sc_core::construction::ActiveOffsetRole<'a>,catalog:&'a sc_core::construction::AdmittedOffsetRoleCatalog<'a>) {
///     role.catalog=catalog;
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct ActiveOffsetRole<'a> {
    catalog: &'a AdmittedOffsetRoleCatalog<'a>,
    association: &'a ActiveAssociation<'a>,
}
impl<'a> ActiveOffsetRole<'a> {
    /// Actual complete admitted catalogue supplying this view.
    #[must_use]
    pub const fn catalog(self) -> &'a AdmittedOffsetRoleCatalog<'a> {
        self.catalog
    }
    /// Actual zero-based original active-list index,not a recipe ordinal or persistent identity.
    #[must_use]
    pub const fn index(self) -> usize {
        self.association.index
    }
    /// Actual original active-list tag field.
    #[must_use]
    pub const fn tag(self) -> &'a LocalTag {
        self.association.tag
    }
    /// Actual original definition field for that tag.
    #[must_use]
    pub const fn definition(self) -> &'a OffsetRoleDefinition {
        self.association.definition
    }
}
