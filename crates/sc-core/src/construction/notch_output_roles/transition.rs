//! Notch-typed history intent over unchanged shared metadata preservation;no undo authority.
use super::{ActiveNotchOutputRole, AdmittedNotchOutputRoleCatalog, NotchOutputRoleDefinition};
use crate::construction::AdmittedEdgeOutputRoleTransition;
use core::fmt;

/// Combined count/quota/refusal records are shared data,not catalogue or transition proofs.
pub use crate::construction::{
    EdgeOutputRoleTransitionLimits as NotchOutputRoleTransitionLimits,
    EdgeOutputRoleTransitionRefusal as NotchOutputRoleTransitionRefusal,
};

/// Actual notch before/after parents and original metadata policy/cause survive every refusal.
pub struct NotchOutputRoleTransitionError<'a> {
    before: &'a AdmittedNotchOutputRoleCatalog<'a>,
    after: &'a AdmittedNotchOutputRoleCatalog<'a>,
    limits: NotchOutputRoleTransitionLimits,
    refusal: NotchOutputRoleTransitionRefusal,
}
impl<'a> NotchOutputRoleTransitionError<'a> {
    /// Actual complete notch source,including inactive definitions.
    #[must_use]
    pub const fn before(&self) -> &'a AdmittedNotchOutputRoleCatalog<'a> {
        self.before
    }
    /// Actual complete proposed notch metadata.
    #[must_use]
    pub const fn after(&self) -> &'a AdmittedNotchOutputRoleCatalog<'a> {
        self.after
    }
    /// Original explicit combined work policy.
    #[must_use]
    pub const fn limits(&self) -> NotchOutputRoleTransitionLimits {
        self.limits
    }
    /// First original complete quota/owner/definition cause.
    #[must_use]
    pub const fn refusal(&self) -> NotchOutputRoleTransitionRefusal {
        self.refusal
    }
}
impl fmt::Debug for NotchOutputRoleTransitionError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NotchOutputRoleTransitionError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for NotchOutputRoleTransitionError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "notch output role transition: {:?}", self.refusal)
    }
}
impl std::error::Error for NotchOutputRoleTransitionError<'_> {}

impl<'s> AdmittedNotchOutputRoleCatalog<'s> {
    /// Preserve complete supplied notch definitions before changing explicit active membership.
    /// Private composition reuses the published metadata algorithm without allocating another index.
    /// Actual issued-history completeness,recorded undo authority,currentness and publication remain
    /// separate obligations. Neither parent changes and no semantic Notch identity or canonical role is issued here.
    /// # Errors
    /// Combined count overflow,D/A/key-byte quotas,owner,then first forgotten/reassigned definition.
    pub fn admit_transition<'a>(
        &'a self,
        after: &'a AdmittedNotchOutputRoleCatalog<'a>,
        limits: NotchOutputRoleTransitionLimits,
    ) -> Result<AdmittedNotchOutputRoleTransition<'a>, NotchOutputRoleTransitionError<'a>>
    where
        's: 'a,
    {
        let before: &AdmittedNotchOutputRoleCatalog<'a> = self;
        let inner = before
            .inner
            .admit_transition(&after.inner, limits)
            .map_err(|error| NotchOutputRoleTransitionError {
                before,
                after,
                limits,
                refusal: error.refusal(),
            })?;
        Ok(AdmittedNotchOutputRoleTransition {
            before,
            after,
            inner,
        })
    }
}

/// Authentic notch-typed structural transition;shared data does not convert this to an edge proof.
/// ```compile_fail
/// let limits:sc_core::construction::NotchOutputRoleTransitionLimits=Default::default();
/// ```
/// ```compile_fail
/// fn edge<'a>(notch:sc_core::construction::AdmittedNotchOutputRoleTransition<'a>)
///     ->sc_core::construction::AdmittedEdgeOutputRoleTransition<'a> { notch }
/// ```
/// ```compile_fail
/// fn point<'a>(notch:sc_core::construction::AdmittedNotchOutputRoleTransition<'a>)
///     ->sc_core::construction::AdmittedPointOutputRoleTransition<'a> { notch }
/// ```
/// ```compile_fail
/// fn replace<'a>(mut transition:sc_core::construction::AdmittedNotchOutputRoleTransition<'a>,
///     after:&'a sc_core::construction::AdmittedNotchOutputRoleCatalog<'a>) {
///     transition.after=after;
/// }
/// ```
/// ```compile_fail
/// fn escape(before:sc_core::construction::AdmittedNotchOutputRoleCatalog<'_>,
///     after:sc_core::construction::AdmittedNotchOutputRoleCatalog<'_>)
///     ->sc_core::construction::AdmittedNotchOutputRoleTransition<'static> {
///     before.admit_transition(&after,sc_core::construction::NotchOutputRoleTransitionLimits::new(0,0,0)).unwrap()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct AdmittedNotchOutputRoleTransition<'a> {
    before: &'a AdmittedNotchOutputRoleCatalog<'a>,
    after: &'a AdmittedNotchOutputRoleCatalog<'a>,
    inner: AdmittedEdgeOutputRoleTransition<'a>,
}
impl<'a> AdmittedNotchOutputRoleTransition<'a> {
    /// Actual complete original notch source.
    #[must_use]
    pub const fn before(self) -> &'a AdmittedNotchOutputRoleCatalog<'a> {
        self.before
    }
    /// Actual complete proposed notch source.
    #[must_use]
    pub const fn after(self) -> &'a AdmittedNotchOutputRoleCatalog<'a> {
        self.after
    }
    /// Original explicit work policy.
    #[must_use]
    pub const fn limits(self) -> NotchOutputRoleTransitionLimits {
        self.inner.limits()
    }
    /// Complete combined definition population.
    #[must_use]
    pub const fn definitions(self) -> usize {
        self.inner.definitions()
    }
    /// Complete combined active-occurrence population.
    #[must_use]
    pub const fn active(self) -> usize {
        self.inner.active()
    }
    /// Complete combined actual machine-key bytes.
    #[must_use]
    pub const fn key_bytes(self) -> usize {
        self.inner.key_bytes()
    }
    /// Actual introduced after-definition fields and original indices,in original after order.
    pub fn introduced(
        &self,
    ) -> impl DoubleEndedIterator<Item = (usize, &NotchOutputRoleDefinition)> {
        self.inner.introduced()
    }
    /// Actual newly active notch membership in after order,without reactivation authority.
    pub fn activated(&self) -> impl DoubleEndedIterator<Item = ActiveNotchOutputRole<'_>> {
        self.inner
            .activated()
            .map(move |inner| ActiveNotchOutputRole {
                catalog: self.after,
                inner,
            })
    }
    /// Actual before memberships becoming inactive,in original before order.
    pub fn deactivated(&self) -> impl DoubleEndedIterator<Item = ActiveNotchOutputRole<'_>> {
        self.inner
            .deactivated()
            .map(move |inner| ActiveNotchOutputRole {
                catalog: self.before,
                inner,
            })
    }
}
