//! Detached approval intent references, without authentication, date creation or approval effects.
use crate::ontology::{EntityId, PackageIdentity};

/// Exact claimed immutable scope record; its tuple/package/evidence coverage requires actual lookup.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ApprovalScopeTarget(EntityId);
impl ApprovalScopeTarget {
    /// Retain the explicit scope identity, never substitute a Design or implicit whole-package scope.
    #[must_use]
    pub const fn from_entity_id(identity: EntityId) -> Self {
        Self(identity)
    }
    /// Original scope-record identity, not proof of existence, scope intersection or reviewer consent.
    #[must_use]
    pub const fn entity_id(self) -> EntityId {
        self.0
    }
}

/// Deliberate approval intent for an exact candidate identity and explicitly reviewed scope.
/// Actor claims grant no authority: execution must require an authenticated human with a scoped
/// approve grant and valid subject/evidence. The human-act date belongs to its detached output record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApprovePackage {
    package: PackageIdentity,
    scope: ApprovalScopeTarget,
}
impl ApprovePackage {
    /// Preserve exact target claims; creates no approval, human credential, date or manifest mutation.
    #[must_use]
    pub const fn new(package: PackageIdentity, scope: ApprovalScopeTarget) -> Self {
        Self { package, scope }
    }
    /// Exact candidate identity; no Design id, latest-package lookup or recomputed digest substitutes.
    #[must_use]
    pub const fn package_identity(&self) -> &PackageIdentity {
        &self.package
    }
    /// Original explicit reviewed scope target.
    #[must_use]
    pub const fn scope(&self) -> ApprovalScopeTarget {
        self.scope
    }
}
