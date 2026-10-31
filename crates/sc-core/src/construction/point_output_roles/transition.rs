//! Point-typed history intent over unchanged shared metadata preservation;no undo authority.
use super::{ActivePointOutputRole, AdmittedPointOutputRoleCatalog, PointOutputRoleDefinition};
use crate::construction::AdmittedEdgeOutputRoleTransition;
use core::fmt;

/// Combined count/quota/refusal records are shared data,not catalogue or transition proofs.
pub use crate::construction::{
    EdgeOutputRoleTransitionLimits as PointOutputRoleTransitionLimits,
    EdgeOutputRoleTransitionRefusal as PointOutputRoleTransitionRefusal,
};

/// Actual point before/after parents and original metadata policy/cause survive every refusal.
pub struct PointOutputRoleTransitionError<'a> {
    before: &'a AdmittedPointOutputRoleCatalog<'a>,
    after: &'a AdmittedPointOutputRoleCatalog<'a>,
    limits: PointOutputRoleTransitionLimits,
    refusal: PointOutputRoleTransitionRefusal,
}
impl<'a> PointOutputRoleTransitionError<'a> {
    /// Actual complete point source,including inactive definitions.
    #[must_use]
    pub const fn before(&self) -> &'a AdmittedPointOutputRoleCatalog<'a> {
        self.before
    }
    /// Actual complete proposed point metadata.
    #[must_use]
    pub const fn after(&self) -> &'a AdmittedPointOutputRoleCatalog<'a> {
        self.after
    }
    /// Original explicit combined work policy.
    #[must_use]
    pub const fn limits(&self) -> PointOutputRoleTransitionLimits {
        self.limits
    }
    /// First original complete quota/owner/definition cause.
    #[must_use]
    pub const fn refusal(&self) -> PointOutputRoleTransitionRefusal {
        self.refusal
    }
}
impl fmt::Debug for PointOutputRoleTransitionError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PointOutputRoleTransitionError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for PointOutputRoleTransitionError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "point output role transition: {:?}", self.refusal)
    }
}
impl std::error::Error for PointOutputRoleTransitionError<'_> {}

impl<'s> AdmittedPointOutputRoleCatalog<'s> {
    /// Preserve complete supplied point definitions before changing explicit active membership.
    /// Private composition reuses the published metadata algorithm without allocating another index.
    /// Actual issued-history completeness,recorded undo authority,currentness and publication remain
    /// separate obligations. Neither parent changes and no PointRef or canonical role is issued here.
    /// # Errors
    /// Combined count overflow,D/A/key-byte quotas,owner,then first forgotten/reassigned definition.
    pub fn admit_transition<'a>(
        &'a self,
        after: &'a AdmittedPointOutputRoleCatalog<'a>,
        limits: PointOutputRoleTransitionLimits,
    ) -> Result<AdmittedPointOutputRoleTransition<'a>, PointOutputRoleTransitionError<'a>>
    where
        's: 'a,
    {
        let before: &AdmittedPointOutputRoleCatalog<'a> = self;
        let inner = before
            .inner
            .admit_transition(&after.inner, limits)
            .map_err(|error| PointOutputRoleTransitionError {
                before,
                after,
                limits,
                refusal: error.refusal(),
            })?;
        Ok(AdmittedPointOutputRoleTransition {
            before,
            after,
            inner,
        })
    }
}

/// Authentic point-typed structural transition;shared data does not convert this to an edge proof.
/// ```compile_fail
/// let limits:sc_core::construction::PointOutputRoleTransitionLimits=Default::default();
/// ```
/// ```compile_fail
/// fn edge<'a>(point:sc_core::construction::AdmittedPointOutputRoleTransition<'a>)
///     ->sc_core::construction::AdmittedEdgeOutputRoleTransition<'a> { point }
/// ```
/// ```compile_fail
/// fn replace<'a>(mut transition:sc_core::construction::AdmittedPointOutputRoleTransition<'a>,
///     after:&'a sc_core::construction::AdmittedPointOutputRoleCatalog<'a>) {
///     transition.after=after;
/// }
/// ```
/// ```compile_fail
/// fn escape(before:sc_core::construction::AdmittedPointOutputRoleCatalog<'_>,
///     after:sc_core::construction::AdmittedPointOutputRoleCatalog<'_>)
///     ->sc_core::construction::AdmittedPointOutputRoleTransition<'static> {
///     before.admit_transition(&after,sc_core::construction::PointOutputRoleTransitionLimits::new(0,0,0)).unwrap()
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct AdmittedPointOutputRoleTransition<'a> {
    before: &'a AdmittedPointOutputRoleCatalog<'a>,
    after: &'a AdmittedPointOutputRoleCatalog<'a>,
    inner: AdmittedEdgeOutputRoleTransition<'a>,
}
impl<'a> AdmittedPointOutputRoleTransition<'a> {
    /// Actual complete original point source.
    #[must_use]
    pub const fn before(self) -> &'a AdmittedPointOutputRoleCatalog<'a> {
        self.before
    }
    /// Actual complete proposed point source.
    #[must_use]
    pub const fn after(self) -> &'a AdmittedPointOutputRoleCatalog<'a> {
        self.after
    }
    /// Original explicit work policy.
    #[must_use]
    pub const fn limits(self) -> PointOutputRoleTransitionLimits {
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
    ) -> impl DoubleEndedIterator<Item = (usize, &PointOutputRoleDefinition)> {
        self.inner.introduced()
    }
    /// Actual newly active point membership in after order,without reactivation authority.
    pub fn activated(&self) -> impl DoubleEndedIterator<Item = ActivePointOutputRole<'_>> {
        self.inner
            .activated()
            .map(move |inner| ActivePointOutputRole {
                catalog: self.after,
                inner,
            })
    }
    /// Actual before memberships becoming inactive,in original before order.
    pub fn deactivated(&self) -> impl DoubleEndedIterator<Item = ActivePointOutputRole<'_>> {
        self.inner
            .deactivated()
            .map(move |inner| ActivePointOutputRole {
                catalog: self.before,
                inner,
            })
    }
}
