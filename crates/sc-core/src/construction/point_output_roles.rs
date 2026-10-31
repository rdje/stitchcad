//! Point-typed catalogue ownership over shared raw tag/key metadata and admission arithmetic.
//! No public point catalogue or membership view converts to an admitted edge-role proof.
use super::edge_output_roles::{
    ActiveEdgeOutputRole, AdmittedEdgeOutputRoleCatalog, EdgeOutputRoleCatalog,
};
use crate::ontology::{EntityId, LocalTag};
use core::fmt;
mod transition;
pub use transition::{
    AdmittedPointOutputRoleTransition, PointOutputRoleTransitionError,
    PointOutputRoleTransitionLimits, PointOutputRoleTransitionRefusal,
};

/// Raw tag/key records and count/refusal data carry no geometric membership proof themselves.
/// Reusing them does not make point and edge catalogue/admitted membership types interchangeable.
pub use super::edge_output_roles::{
    EdgeOutputRoleCatalogLimits as PointOutputRoleCatalogLimits,
    EdgeOutputRoleCatalogRefusal as PointOutputRoleCatalogRefusal,
    EdgeOutputRoleDefinition as PointOutputRoleDefinition,
};

/// Original point catalogue,explicit policy and first shared metadata cause remain inspectable.
pub struct PointOutputRoleCatalogError<'a> {
    catalog: &'a PointOutputRoleCatalog,
    limits: PointOutputRoleCatalogLimits,
    refusal: PointOutputRoleCatalogRefusal,
}
impl<'a> PointOutputRoleCatalogError<'a> {
    /// Actual original point owner,not an edge catalogue or a copied field set.
    #[must_use]
    pub const fn catalog(&self) -> &'a PointOutputRoleCatalog {
        self.catalog
    }
    /// Original explicit whole metadata policy.
    #[must_use]
    pub const fn limits(&self) -> PointOutputRoleCatalogLimits {
        self.limits
    }
    /// First original shared quota/duplicate/undefined cause with unchanged field positions.
    #[must_use]
    pub const fn refusal(&self) -> PointOutputRoleCatalogRefusal {
        self.refusal
    }
}
impl fmt::Debug for PointOutputRoleCatalogError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PointOutputRoleCatalogError")
            .field("operation", &self.catalog.operation())
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for PointOutputRoleCatalogError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "point output role catalogue admission: {:?}",
            self.refusal
        )
    }
}
impl std::error::Error for PointOutputRoleCatalogError<'_> {}

/// Explicit point-role inventory and canonical active membership,with a distinct typed owner.
/// Private composition reuses existing metadata admission,not edge geometry or a PointRef registry.
/// No identity generation,source lookup,history completeness or current-frame proof follows.
/// ```compile_fail
/// let limits:sc_core::construction::PointOutputRoleCatalogLimits=Default::default();
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PointOutputRoleCatalog {
    inner: EdgeOutputRoleCatalog,
}
impl PointOutputRoleCatalog {
    /// Move explicit original metadata into the point domain without cloning,admission or issuance.
    /// All issued definitions,including inactive ones,must be supplied;actual completeness is later.
    #[must_use]
    pub const fn new(
        operation: EntityId,
        definitions: Vec<PointOutputRoleDefinition>,
        active: Vec<LocalTag>,
    ) -> Self {
        Self {
            inner: EdgeOutputRoleCatalog::new(operation, definitions, active),
        }
    }
    /// Actual declared canonical operation,without current kind/ownership proof.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        self.inner.operation()
    }
    /// Every actual raw point-role definition in original order,including inactive definitions.
    #[must_use]
    pub fn definitions(&self) -> &[PointOutputRoleDefinition] {
        self.inner.definitions()
    }
    /// Every actual original active point tag,not evaluated-instance presence.
    #[must_use]
    pub fn active_tags(&self) -> &[LocalTag] {
        self.inner.active_tags()
    }
    /// Admit complete D/A populations and 2D+2A workspace counts,then D/A/key-byte quotas before
    /// indexes. Shared metadata logic retains tag-before-key and duplicate-before-undefined order.
    /// No accepted prefix escapes. Counts are index entries,not prior allocation or memory bytes.
    /// # Errors
    /// Complete count/quota/text refusal,then first original duplicate/undefined metadata cause.
    pub fn admit(
        &self,
        limits: PointOutputRoleCatalogLimits,
    ) -> Result<AdmittedPointOutputRoleCatalog<'_>, PointOutputRoleCatalogError<'_>> {
        let inner = self
            .inner
            .admit(limits)
            .map_err(|error| PointOutputRoleCatalogError {
                catalog: self,
                limits,
                refusal: error.refusal(),
            })?;
        Ok(AdmittedPointOutputRoleCatalog {
            catalog: self,
            inner,
        })
    }
}

/// Authentic point-domain metadata admission;no conversion to an admitted edge-role proof exists.
/// ```compile_fail
/// fn edge<'a>(points:&'a sc_core::construction::AdmittedPointOutputRoleCatalog<'a>)
///     ->&'a sc_core::construction::AdmittedEdgeOutputRoleCatalog<'a> { points }
/// ```
/// ```compile_fail
/// fn forge<'a>(catalog:&'a sc_core::construction::PointOutputRoleCatalog,
///     inner:sc_core::construction::AdmittedEdgeOutputRoleCatalog<'a>)
///     ->sc_core::construction::AdmittedPointOutputRoleCatalog<'a> {
///     sc_core::construction::AdmittedPointOutputRoleCatalog { catalog,inner }
/// }
/// ```
/// ```compile_fail
/// fn escape(catalog:sc_core::construction::PointOutputRoleCatalog)
///     ->sc_core::construction::AdmittedPointOutputRoleCatalog<'static> {
///     catalog.admit(sc_core::construction::PointOutputRoleCatalogLimits::new(0,0,0)).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct AdmittedPointOutputRoleCatalog<'a> {
    catalog: &'a PointOutputRoleCatalog,
    inner: AdmittedEdgeOutputRoleCatalog<'a>,
}
impl<'a> AdmittedPointOutputRoleCatalog<'a> {
    /// Actual complete original point catalogue and both authored field orders.
    #[must_use]
    pub const fn catalog(&self) -> &'a PointOutputRoleCatalog {
        self.catalog
    }
    /// Original explicit admission policy.
    #[must_use]
    pub const fn limits(&self) -> PointOutputRoleCatalogLimits {
        self.inner.limits()
    }
    /// Complete actual machine-key bytes,not an allocation footprint.
    #[must_use]
    pub const fn key_bytes(&self) -> usize {
        self.inner.key_bytes()
    }
    /// Complete peak index/membership entry counts,not allocation bytes.
    #[must_use]
    pub const fn workspace_entries(&self) -> usize {
        self.inner.workspace_entries()
    }
    /// Actual original definition for an explicit tag,including inactive point roles.
    #[must_use]
    pub fn definition(&self, tag: LocalTag) -> Option<&'a PointOutputRoleDefinition> {
        self.inner.definition(tag)
    }
    /// Declared canonical point membership only,not evaluated presence or source liveness.
    #[must_use]
    pub fn is_active(&self, tag: LocalTag) -> bool {
        self.inner.is_active(tag)
    }
    /// Authentic original point membership and definition when canonically active.
    #[must_use]
    pub fn active_role(&self, tag: LocalTag) -> Option<ActivePointOutputRole<'_>> {
        self.inner
            .active_role(tag)
            .map(|inner| ActivePointOutputRole {
                catalog: self,
                inner,
            })
    }
    /// Every actual active point field in original active-list order,including reverse iteration.
    pub fn active_roles(
        &self,
    ) -> impl ExactSizeIterator<Item = ActivePointOutputRole<'_>> + DoubleEndedIterator {
        self.inner
            .active_roles()
            .map(move |inner| ActivePointOutputRole {
                catalog: self,
                inner,
            })
    }
}

/// Authentic membership in its actual admitted point catalogue;it is not an edge-role witness.
/// ```compile_fail
/// fn edge<'a>(role:sc_core::construction::ActivePointOutputRole<'a>)
///     ->sc_core::construction::ActiveEdgeOutputRole<'a> { role }
/// ```
/// ```compile_fail
/// fn reparent<'a>(mut role:sc_core::construction::ActivePointOutputRole<'a>,
///     other:&'a sc_core::construction::AdmittedPointOutputRoleCatalog<'a>) {
///     role.catalog=other;
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct ActivePointOutputRole<'a> {
    catalog: &'a AdmittedPointOutputRoleCatalog<'a>,
    inner: ActiveEdgeOutputRole<'a>,
}
impl<'a> ActivePointOutputRole<'a> {
    /// Actual complete admitted point catalogue supplying this field.
    #[must_use]
    pub const fn catalog(self) -> &'a AdmittedPointOutputRoleCatalog<'a> {
        self.catalog
    }
    /// Original active-list position,not a stable identity or recipe ordinal.
    #[must_use]
    pub const fn index(self) -> usize {
        self.inner.index()
    }
    /// Actual original point active-tag field.
    #[must_use]
    pub const fn tag(self) -> &'a LocalTag {
        self.inner.tag()
    }
    /// Actual original raw definition;this record alone supplies no namespace membership proof.
    #[must_use]
    pub const fn definition(self) -> &'a PointOutputRoleDefinition {
        self.inner.definition()
    }
}
