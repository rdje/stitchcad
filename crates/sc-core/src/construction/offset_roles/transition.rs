//! Structural role-history preservation only;not command/history authority or geometric continuity.
use super::{ActiveOffsetRole, AdmittedOffsetRoleCatalog, OffsetRoleDefinition};
use crate::ontology::LocalTag;
use core::fmt;

/// Explicit combined metadata work allowances across both original catalogues.
/// ```compile_fail
/// let limits:sc_core::construction::OffsetRoleTransitionLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OffsetRoleTransitionLimits {
    definitions: usize,
    active: usize,
    key_bytes: usize,
}
impl OffsetRoleTransitionLimits {
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
pub enum OffsetRoleTransitionRefusal {
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
pub struct OffsetRoleTransitionError<'a> {
    before: &'a AdmittedOffsetRoleCatalog<'a>,
    after: &'a AdmittedOffsetRoleCatalog<'a>,
    limits: OffsetRoleTransitionLimits,
    refusal: OffsetRoleTransitionRefusal,
}
impl<'a> OffsetRoleTransitionError<'a> {
    /// Actual complete before catalogue.
    #[must_use]
    pub const fn before(&self) -> &'a AdmittedOffsetRoleCatalog<'a> {
        self.before
    }
    /// Actual complete proposed after catalogue.
    #[must_use]
    pub const fn after(&self) -> &'a AdmittedOffsetRoleCatalog<'a> {
        self.after
    }
    /// Original explicit combined resource policy.
    #[must_use]
    pub const fn limits(&self) -> OffsetRoleTransitionLimits {
        self.limits
    }
    /// Complete quota or first original owner/definition cause.
    #[must_use]
    pub const fn refusal(&self) -> OffsetRoleTransitionRefusal {
        self.refusal
    }
}
impl fmt::Debug for OffsetRoleTransitionError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OffsetRoleTransitionError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for OffsetRoleTransitionError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "offset role transition: {:?}", self.refusal)
    }
}
impl std::error::Error for OffsetRoleTransitionError<'_> {}

/// Complete readonly structural transition intent;no catalogue,Design or history is mutated.
/// ```compile_fail
/// fn escape(before:sc_core::construction::AdmittedOffsetRoleCatalog<'_>,after:sc_core::construction::AdmittedOffsetRoleCatalog<'_>)
///     ->sc_core::construction::AdmittedOffsetRoleTransition<'static> {
///     before.admit_transition(&after,sc_core::construction::OffsetRoleTransitionLimits::new(0,0,0)).unwrap()
/// }
/// ```
/// ```compile_fail
/// fn replace<'a>(mut transition:sc_core::construction::AdmittedOffsetRoleTransition<'a>,after:&'a sc_core::construction::AdmittedOffsetRoleCatalog<'a>) {
///     transition.after=after;
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct AdmittedOffsetRoleTransition<'a> {
    before: &'a AdmittedOffsetRoleCatalog<'a>,
    after: &'a AdmittedOffsetRoleCatalog<'a>,
    limits: OffsetRoleTransitionLimits,
    definitions: usize,
    active: usize,
    key_bytes: usize,
}
impl<'a> AdmittedOffsetRoleTransition<'a> {
    /// Actual complete before source,including inactive issued definitions.
    #[must_use]
    pub const fn before(self) -> &'a AdmittedOffsetRoleCatalog<'a> {
        self.before
    }
    /// Actual complete proposed after source.
    #[must_use]
    pub const fn after(self) -> &'a AdmittedOffsetRoleCatalog<'a> {
        self.after
    }
    /// Selected original work policy.
    #[must_use]
    pub const fn limits(self) -> OffsetRoleTransitionLimits {
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
    pub fn introduced(&self) -> impl DoubleEndedIterator<Item = (usize, &OffsetRoleDefinition)> {
        self.after
            .catalog()
            .definitions()
            .iter()
            .enumerate()
            .filter(move |(_, role)| self.before.definition(role.tag()).is_none())
    }
    /// Actual after membership views newly active here,in original after active-list order.
    /// This describes intent,not authority to reactivate a role or geometric matching evidence.
    pub fn activated(&self) -> impl DoubleEndedIterator<Item = ActiveOffsetRole<'_>> {
        self.after
            .active_roles()
            .filter(move |role| !self.before.is_active(*role.tag()))
    }
    /// Actual before membership views becoming inactive,in original before active-list order.
    pub fn deactivated(&self) -> impl DoubleEndedIterator<Item = ActiveOffsetRole<'_>> {
        self.before
            .active_roles()
            .filter(move |role| !self.after.is_active(*role.tag()))
    }
}
impl<'s> AdmittedOffsetRoleCatalog<'s> {
    /// Admit complete combined counts/text before owner and all original definition checks.
    /// Every prior tag/key must remain,even if inactive;membership may change explicitly without
    /// rewinding definitions. Neither source is mutated and no new work index is allocated.
    /// This does not prove actual issuance history,recorded undo authority,current Design membership,
    /// geometric role continuity or publication permission. It validates supplied metadata intent only.
    /// # Errors
    /// Count overflow,definition quota,active quota,key-byte quota,owner,then first forgotten/reassigned role.
    pub fn admit_transition<'a>(
        &'a self,
        after: &'a AdmittedOffsetRoleCatalog<'a>,
        limits: OffsetRoleTransitionLimits,
    ) -> Result<AdmittedOffsetRoleTransition<'a>, OffsetRoleTransitionError<'a>>
    where
        's: 'a,
    {
        let before: &AdmittedOffsetRoleCatalog<'a> = self;
        let refuse = |refusal| OffsetRoleTransitionError {
            before,
            after,
            limits,
            refusal,
        };
        let overflow = || refuse(OffsetRoleTransitionRefusal::CountOverflow);
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
            return Err(refuse(OffsetRoleTransitionRefusal::Definitions {
                required: definitions,
                allowed: limits.definitions,
            }));
        }
        if active > limits.active {
            return Err(refuse(OffsetRoleTransitionRefusal::Active {
                required: active,
                allowed: limits.active,
            }));
        }
        if key_bytes > limits.key_bytes {
            return Err(refuse(OffsetRoleTransitionRefusal::KeyBytes {
                required: key_bytes,
                allowed: limits.key_bytes,
            }));
        }
        if before.catalog().operation() != after.catalog().operation() {
            return Err(refuse(OffsetRoleTransitionRefusal::Owner));
        }
        for (index, role) in before.catalog().definitions().iter().enumerate() {
            let Some(retained) = after.definition(role.tag()) else {
                return Err(refuse(OffsetRoleTransitionRefusal::Forgotten {
                    index,
                    tag: role.tag(),
                }));
            };
            if retained.key() != role.key() {
                return Err(refuse(OffsetRoleTransitionRefusal::ReassignedKey {
                    index,
                    tag: role.tag(),
                }));
            }
        }
        Ok(AdmittedOffsetRoleTransition {
            before,
            after,
            limits,
            definitions,
            active,
            key_bytes,
        })
    }
}
