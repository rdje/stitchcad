//! Structural role-history preservation only;not command/history authority or geometric continuity.
use super::{ActiveEdgeOutputRole, AdmittedEdgeOutputRoleCatalog, EdgeOutputRoleDefinition};
use crate::ontology::LocalTag;
use core::fmt;

/// Explicit combined metadata work allowances across both original catalogues.
/// ```compile_fail
/// let limits:sc_core::construction::EdgeOutputRoleTransitionLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EdgeOutputRoleTransitionLimits {
    definitions: usize,
    active: usize,
    key_bytes: usize,
}
impl EdgeOutputRoleTransitionLimits {
    /// Bound combined definitions,active occurrences and key bytes before content comparisons.
    #[must_use]
    pub const fn new(definitions: usize, active: usize, key_bytes: usize) -> Self {
        Self {
            definitions,
            active,
            key_bytes,
        }
    }
    /// Complete combined definition allowance.
    #[must_use]
    pub const fn definitions(self) -> usize {
        self.definitions
    }
    /// Complete combined active-occurrence allowance.
    #[must_use]
    pub const fn active(self) -> usize {
        self.active
    }
    /// Complete combined actual machine-key byte allowance.
    #[must_use]
    pub const fn key_bytes(self) -> usize {
        self.key_bytes
    }
}
/// Complete resource admission or first original owner/definition preservation cause.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdgeOutputRoleTransitionRefusal {
    /// Complete combined populations cannot be counted.
    CountOverflow,
    /// Complete combined definitions exceed policy.
    Definitions {
        /// Complete required population.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete combined active occurrences exceed policy.
    Active {
        /// Complete required population.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete combined key bytes exceed policy.
    KeyBytes {
        /// Complete required byte count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Before/after catalogue operation identities differ.
    Owner,
    /// A prior issued definition is missing,including an inactive one.
    Forgotten {
        /// Actual original before-definition index.
        index: usize,
        /// Actual prior explicit tag.
        tag: LocalTag,
    },
    /// A prior tag has been assigned another local machine key.
    ReassignedKey {
        /// Actual original before-definition index.
        index: usize,
        /// Actual prior explicit tag.
        tag: LocalTag,
    },
}
/// Actual admitted before/after catalogues,policy and first preservation refusal.
pub struct EdgeOutputRoleTransitionError<'a> {
    before: &'a AdmittedEdgeOutputRoleCatalog<'a>,
    after: &'a AdmittedEdgeOutputRoleCatalog<'a>,
    limits: EdgeOutputRoleTransitionLimits,
    refusal: EdgeOutputRoleTransitionRefusal,
}
impl<'a> EdgeOutputRoleTransitionError<'a> {
    /// Actual complete before catalogue.
    #[must_use]
    pub const fn before(&self) -> &'a AdmittedEdgeOutputRoleCatalog<'a> {
        self.before
    }
    /// Actual complete proposed after catalogue.
    #[must_use]
    pub const fn after(&self) -> &'a AdmittedEdgeOutputRoleCatalog<'a> {
        self.after
    }
    /// Original explicit combined resource policy.
    #[must_use]
    pub const fn limits(&self) -> EdgeOutputRoleTransitionLimits {
        self.limits
    }
    /// Complete quota or first original owner/definition cause.
    #[must_use]
    pub const fn refusal(&self) -> EdgeOutputRoleTransitionRefusal {
        self.refusal
    }
}
impl fmt::Debug for EdgeOutputRoleTransitionError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EdgeOutputRoleTransitionError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for EdgeOutputRoleTransitionError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "edge output role transition: {:?}", self.refusal)
    }
}
impl std::error::Error for EdgeOutputRoleTransitionError<'_> {}

/// Complete readonly structural transition intent;no catalogue,Design or history is mutated.
/// ```compile_fail
/// fn escape(before:sc_core::construction::AdmittedEdgeOutputRoleCatalog<'_>,after:sc_core::construction::AdmittedEdgeOutputRoleCatalog<'_>)
///     ->sc_core::construction::AdmittedEdgeOutputRoleTransition<'static> {
///     before.admit_transition(&after,sc_core::construction::EdgeOutputRoleTransitionLimits::new(0,0,0)).unwrap()
/// }
/// ```
/// ```compile_fail
/// fn replace<'a>(mut transition:sc_core::construction::AdmittedEdgeOutputRoleTransition<'a>,after:&'a sc_core::construction::AdmittedEdgeOutputRoleCatalog<'a>) {
///     transition.after=after;
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct AdmittedEdgeOutputRoleTransition<'a> {
    before: &'a AdmittedEdgeOutputRoleCatalog<'a>,
    after: &'a AdmittedEdgeOutputRoleCatalog<'a>,
    limits: EdgeOutputRoleTransitionLimits,
    definitions: usize,
    active: usize,
    key_bytes: usize,
}
impl<'a> AdmittedEdgeOutputRoleTransition<'a> {
    /// Actual complete before source,including inactive issued definitions.
    #[must_use]
    pub const fn before(self) -> &'a AdmittedEdgeOutputRoleCatalog<'a> {
        self.before
    }
    /// Actual complete proposed after source.
    #[must_use]
    pub const fn after(self) -> &'a AdmittedEdgeOutputRoleCatalog<'a> {
        self.after
    }
    /// Selected original work policy.
    #[must_use]
    pub const fn limits(self) -> EdgeOutputRoleTransitionLimits {
        self.limits
    }
    /// Complete combined definition population,not an accepted prefix.
    #[must_use]
    pub const fn definitions(self) -> usize {
        self.definitions
    }
    /// Complete combined active-occurrence population.
    #[must_use]
    pub const fn active(self) -> usize {
        self.active
    }
    /// Complete combined actual key bytes.
    #[must_use]
    pub const fn key_bytes(self) -> usize {
        self.key_bytes
    }
    /// Actual newly introduced definition fields in original after order,with their real indices.
    pub fn introduced(
        &self,
    ) -> impl DoubleEndedIterator<Item = (usize, &EdgeOutputRoleDefinition)> {
        self.after
            .catalog()
            .definitions()
            .iter()
            .enumerate()
            .filter(move |(_, role)| self.before.definition(role.tag()).is_none())
    }
    /// Actual after membership views newly active here,in original after active-list order.
    /// This describes intent,not authority to reactivate a role or geometric matching evidence.
    pub fn activated(&self) -> impl DoubleEndedIterator<Item = ActiveEdgeOutputRole<'_>> {
        self.after
            .active_roles()
            .filter(move |role| !self.before.is_active(*role.tag()))
    }
    /// Actual before membership views becoming inactive,in original before active-list order.
    pub fn deactivated(&self) -> impl DoubleEndedIterator<Item = ActiveEdgeOutputRole<'_>> {
        self.before
            .active_roles()
            .filter(move |role| !self.after.is_active(*role.tag()))
    }
}
impl<'s> AdmittedEdgeOutputRoleCatalog<'s> {
    /// Admit complete combined counts/text before owner and all original definition checks.
    /// Every prior tag/key must remain,even if inactive;membership may change explicitly without
    /// rewinding definitions. Neither source is mutated and no new work index is allocated.
    /// This does not prove actual issuance history,recorded undo authority,current Design membership,
    /// geometric role continuity or publication permission. It validates supplied metadata intent only.
    /// # Errors
    /// Count overflow,definition quota,active quota,key-byte quota,owner,then first forgotten/reassigned role.
    pub fn admit_transition<'a>(
        &'a self,
        after: &'a AdmittedEdgeOutputRoleCatalog<'a>,
        limits: EdgeOutputRoleTransitionLimits,
    ) -> Result<AdmittedEdgeOutputRoleTransition<'a>, EdgeOutputRoleTransitionError<'a>>
    where
        's: 'a,
    {
        let before: &AdmittedEdgeOutputRoleCatalog<'a> = self;
        let refuse = |refusal| EdgeOutputRoleTransitionError {
            before,
            after,
            limits,
            refusal,
        };
        let overflow = || refuse(EdgeOutputRoleTransitionRefusal::CountOverflow);
        let definitions = before
            .catalog()
            .definitions()
            .len()
            .checked_add(after.catalog().definitions().len())
            .ok_or_else(overflow)?;
        let active = before
            .catalog()
            .active_tags()
            .len()
            .checked_add(after.catalog().active_tags().len())
            .ok_or_else(overflow)?;
        let key_bytes = before
            .key_bytes()
            .checked_add(after.key_bytes())
            .ok_or_else(overflow)?;
        if definitions > limits.definitions {
            return Err(refuse(EdgeOutputRoleTransitionRefusal::Definitions {
                required: definitions,
                allowed: limits.definitions,
            }));
        }
        if active > limits.active {
            return Err(refuse(EdgeOutputRoleTransitionRefusal::Active {
                required: active,
                allowed: limits.active,
            }));
        }
        if key_bytes > limits.key_bytes {
            return Err(refuse(EdgeOutputRoleTransitionRefusal::KeyBytes {
                required: key_bytes,
                allowed: limits.key_bytes,
            }));
        }
        if before.catalog().operation() != after.catalog().operation() {
            return Err(refuse(EdgeOutputRoleTransitionRefusal::Owner));
        }
        for (index, role) in before.catalog().definitions().iter().enumerate() {
            let Some(retained) = after.definition(role.tag()) else {
                return Err(refuse(EdgeOutputRoleTransitionRefusal::Forgotten {
                    index,
                    tag: role.tag(),
                }));
            };
            if retained.key() != role.key() {
                return Err(refuse(EdgeOutputRoleTransitionRefusal::ReassignedKey {
                    index,
                    tag: role.tag(),
                }));
            }
        }
        Ok(AdmittedEdgeOutputRoleTransition {
            before,
            after,
            limits,
            definitions,
            active,
            key_bytes,
        })
    }
}
