//! Notch-typed catalogue ownership over shared raw tag/key metadata and admission arithmetic.
//! No public notch catalogue/member converts to point/edge membership or issues semantic EntityIds.
use super::edge_output_roles::{
    ActiveEdgeOutputRole, AdmittedEdgeOutputRoleCatalog, EdgeOutputRoleCatalog,
};
/// Raw tag/key records and count/refusal data carry no geometric membership proof themselves.
/// Reusing them does not make notch and edge catalogue/admitted membership types interchangeable.
pub use super::edge_output_roles::{
    EdgeOutputRoleCatalogLimits as NotchOutputRoleCatalogLimits,
    EdgeOutputRoleCatalogRefusal as NotchOutputRoleCatalogRefusal,
    EdgeOutputRoleDefinition as NotchOutputRoleDefinition,
};
use crate::ontology::{EntityId, LocalTag};
use core::fmt;
mod transition;
pub use transition::{
    AdmittedNotchOutputRoleTransition, NotchOutputRoleTransitionError,
    NotchOutputRoleTransitionLimits, NotchOutputRoleTransitionRefusal,
};

/// Original notch catalogue,explicit policy and first shared metadata cause remain inspectable.
pub struct NotchOutputRoleCatalogError<'a> {
    catalog: &'a NotchOutputRoleCatalog,
    limits: NotchOutputRoleCatalogLimits,
    refusal: NotchOutputRoleCatalogRefusal,
}
impl<'a> NotchOutputRoleCatalogError<'a> {
    /// Actual original notch owner,not an edge catalogue or a copied field set.
    #[must_use]
    pub const fn catalog(&self) -> &'a NotchOutputRoleCatalog {
        self.catalog
    }
    /// Original explicit whole metadata policy.
    #[must_use]
    pub const fn limits(&self) -> NotchOutputRoleCatalogLimits {
        self.limits
    }
    /// First original shared quota/duplicate/undefined cause with unchanged field positions.
    #[must_use]
    pub const fn refusal(&self) -> NotchOutputRoleCatalogRefusal {
        self.refusal
    }
}
impl fmt::Debug for NotchOutputRoleCatalogError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NotchOutputRoleCatalogError")
            .field("operation", &self.catalog.operation())
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for NotchOutputRoleCatalogError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "notch output role catalogue admission: {:?}",
            self.refusal
        )
    }
}
impl std::error::Error for NotchOutputRoleCatalogError<'_> {}

/// Explicit notch-role inventory and canonical active membership,with a distinct typed owner.
/// Private composition reuses bounded raw metadata admission,not geometric membership or a Notch registry.
/// Local tags name semantic output roles,not Notch EntityIds,PointRefs or EdgeRefs. No identity
/// generation,source lookup,history completeness or current-frame proof follows.
/// ```compile_fail
/// let limits:sc_core::construction::NotchOutputRoleCatalogLimits=Default::default();
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NotchOutputRoleCatalog {
    inner: EdgeOutputRoleCatalog,
}
impl NotchOutputRoleCatalog {
    /// Move explicit original metadata into the notch domain without cloning,admission or issuance.
    /// All issued definitions,including inactive ones,must be supplied;actual completeness is later.
    #[must_use]
    pub const fn new(
        operation: EntityId,
        definitions: Vec<NotchOutputRoleDefinition>,
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
    /// Every actual raw notch-role definition in original order,including inactive definitions.
    #[must_use]
    pub fn definitions(&self) -> &[NotchOutputRoleDefinition] {
        self.inner.definitions()
    }
    /// Every actual original active notch tag,not evaluated-instance presence.
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
        limits: NotchOutputRoleCatalogLimits,
    ) -> Result<AdmittedNotchOutputRoleCatalog<'_>, NotchOutputRoleCatalogError<'_>> {
        let inner = self
            .inner
            .admit(limits)
            .map_err(|error| NotchOutputRoleCatalogError {
                catalog: self,
                limits,
                refusal: error.refusal(),
            })?;
        Ok(AdmittedNotchOutputRoleCatalog {
            catalog: self,
            inner,
        })
    }
}

/// Authentic notch-domain metadata admission;no conversion to point/edge membership exists.
/// ```compile_fail
/// fn edge<'a>(notches:&'a sc_core::construction::AdmittedNotchOutputRoleCatalog<'a>)
///     ->&'a sc_core::construction::AdmittedEdgeOutputRoleCatalog<'a> { notches }
/// ```
/// ```compile_fail
/// fn point<'a>(notches:&'a sc_core::construction::AdmittedNotchOutputRoleCatalog<'a>)
///     ->&'a sc_core::construction::AdmittedPointOutputRoleCatalog<'a> { notches }
/// ```
/// ```compile_fail
/// fn forge<'a>(catalog:&'a sc_core::construction::NotchOutputRoleCatalog,
///     inner:sc_core::construction::AdmittedEdgeOutputRoleCatalog<'a>)
///     ->sc_core::construction::AdmittedNotchOutputRoleCatalog<'a> {
///     sc_core::construction::AdmittedNotchOutputRoleCatalog { catalog,inner }
/// }
/// ```
/// ```compile_fail
/// fn escape(catalog:sc_core::construction::NotchOutputRoleCatalog)
///     ->sc_core::construction::AdmittedNotchOutputRoleCatalog<'static> {
///     catalog.admit(sc_core::construction::NotchOutputRoleCatalogLimits::new(0,0,0)).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct AdmittedNotchOutputRoleCatalog<'a> {
    catalog: &'a NotchOutputRoleCatalog,
    inner: AdmittedEdgeOutputRoleCatalog<'a>,
}
impl<'a> AdmittedNotchOutputRoleCatalog<'a> {
    /// Actual complete original notch catalogue and both authored field orders.
    #[must_use]
    pub const fn catalog(&self) -> &'a NotchOutputRoleCatalog {
        self.catalog
    }
    /// Original explicit admission policy.
    #[must_use]
    pub const fn limits(&self) -> NotchOutputRoleCatalogLimits {
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
    /// Actual original definition for an explicit tag,including inactive notch roles.
    #[must_use]
    pub fn definition(&self, tag: LocalTag) -> Option<&'a NotchOutputRoleDefinition> {
        self.inner.definition(tag)
    }
    /// Declared canonical notch membership only,not evaluated presence or source liveness.
    #[must_use]
    pub fn is_active(&self, tag: LocalTag) -> bool {
        self.inner.is_active(tag)
    }
    /// Authentic original notch membership and definition when canonically active.
    #[must_use]
    pub fn active_role(&self, tag: LocalTag) -> Option<ActiveNotchOutputRole<'_>> {
        self.inner
            .active_role(tag)
            .map(|inner| ActiveNotchOutputRole {
                catalog: self,
                inner,
            })
    }
    /// Every actual active notch field in original active-list order,including reverse iteration.
    pub fn active_roles(
        &self,
    ) -> impl ExactSizeIterator<Item = ActiveNotchOutputRole<'_>> + DoubleEndedIterator {
        self.inner
            .active_roles()
            .map(move |inner| ActiveNotchOutputRole {
                catalog: self,
                inner,
            })
    }
}

/// Authentic membership in its actual admitted notch catalogue;it is not an edge-role witness.
/// ```compile_fail
/// fn edge<'a>(role:sc_core::construction::ActiveNotchOutputRole<'a>)
///     ->sc_core::construction::ActiveEdgeOutputRole<'a> { role }
/// ```
/// ```compile_fail
/// fn point<'a>(role:sc_core::construction::ActiveNotchOutputRole<'a>)
///     ->sc_core::construction::ActivePointOutputRole<'a> { role }
/// ```
/// ```compile_fail
/// fn reparent<'a>(mut role:sc_core::construction::ActiveNotchOutputRole<'a>,
///     other:&'a sc_core::construction::AdmittedNotchOutputRoleCatalog<'a>) {
///     role.catalog=other;
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct ActiveNotchOutputRole<'a> {
    catalog: &'a AdmittedNotchOutputRoleCatalog<'a>,
    inner: ActiveEdgeOutputRole<'a>,
}
impl<'a> ActiveNotchOutputRole<'a> {
    /// Actual complete admitted notch catalogue supplying this field.
    #[must_use]
    pub const fn catalog(self) -> &'a AdmittedNotchOutputRoleCatalog<'a> {
        self.catalog
    }
    /// Original active-list position,not a stable identity or recipe ordinal.
    #[must_use]
    pub const fn index(self) -> usize {
        self.inner.index()
    }
    /// Actual original notch active-tag field.
    #[must_use]
    pub const fn tag(self) -> &'a LocalTag {
        self.inner.tag()
    }
    /// Actual original raw definition;this record alone supplies no namespace membership proof.
    #[must_use]
    pub const fn definition(self) -> &'a NotchOutputRoleDefinition {
        self.inner.definition()
    }
}
