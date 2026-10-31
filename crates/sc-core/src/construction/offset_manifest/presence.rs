//! Readonly association of candidate occurrence with canonical active metadata;no instance proof.
use super::{AdmittedOffsetCandidates, OffsetCandidateFragment};
use crate::{
    construction::{ActiveOffsetRole, AdmittedOffsetRoleCatalog},
    ontology::{EdgeRef, LocalTag},
};
use core::fmt;

/// Explicit whole candidate/catalogue populations for this association stage.
/// ```compile_fail
/// let limits:sc_core::construction::OffsetRolePresenceLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OffsetRolePresenceLimits {
    candidates: usize,
    definitions: usize,
    active: usize,
}
impl OffsetRolePresenceLimits {
    /// Select complete candidate,definition and active-id allowances;no prefix is accepted.
    #[must_use]
    pub const fn new(candidates: usize, definitions: usize, active: usize) -> Self {
        Self {
            candidates,
            definitions,
            active,
        }
    }
    /// Complete candidate occurrence allowance.
    #[must_use]
    pub const fn candidates(self) -> usize {
        self.candidates
    }
    /// Complete catalogue definition allowance,including inactive roles.
    #[must_use]
    pub const fn definitions(self) -> usize {
        self.definitions
    }
    /// Complete canonical active-id allowance,including roles absent from this manifest.
    #[must_use]
    pub const fn active(self) -> usize {
        self.active
    }
}
/// First resource,owner or original candidate's role-membership refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OffsetRolePresenceRefusal {
    /// Complete candidate population exceeds policy.
    Candidates {
        /// Complete required population.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
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
    /// Candidate operation differs from the actual catalogue owner.
    Operation,
    /// An original candidate tag has no declared definition.
    Undefined {
        /// Original candidate occurrence index.
        candidate: usize,
        /// Original explicit candidate tag.
        tag: LocalTag,
    },
    /// An original candidate tag is defined but not canonically active.
    Inactive {
        /// Original candidate occurrence index.
        candidate: usize,
        /// Original explicit candidate tag.
        tag: LocalTag,
    },
}
/// Actual whole admitted inputs,policy and original cause survive refusal.
pub struct OffsetRolePresenceError<'a> {
    admitted: &'a AdmittedOffsetCandidates<'a>,
    catalog: &'a AdmittedOffsetRoleCatalog<'a>,
    limits: OffsetRolePresenceLimits,
    refusal: OffsetRolePresenceRefusal,
}
impl<'a> OffsetRolePresenceError<'a> {
    /// Actual admitted candidate source.
    #[must_use]
    pub const fn admitted(&self) -> &'a AdmittedOffsetCandidates<'a> {
        self.admitted
    }
    /// Actual admitted catalogue source,including unused and inactive definitions.
    #[must_use]
    pub const fn catalog(&self) -> &'a AdmittedOffsetRoleCatalog<'a> {
        self.catalog
    }
    /// Original explicit association policy.
    #[must_use]
    pub const fn limits(&self) -> OffsetRolePresenceLimits {
        self.limits
    }
    /// First whole-resource,owner or original candidate cause.
    #[must_use]
    pub const fn refusal(&self) -> OffsetRolePresenceRefusal {
        self.refusal
    }
}
impl fmt::Debug for OffsetRolePresenceError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OffsetRolePresenceError")
            .field("operation", &self.admitted.manifest().operation())
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for OffsetRolePresenceError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "offset active-role association: {:?}", self.refusal)
    }
}
impl std::error::Error for OffsetRolePresenceError<'_> {}

impl AdmittedOffsetCandidates<'_> {
    /// Associate each candidate with actual active canonical metadata,without retiring absent roles.
    /// Check complete N candidates,D definitions,A active ids before owner/tag lookup or allocation.
    /// Retain N pairs and reuse existing catalogue indexes;no new index is allocated. These are
    /// population bounds,not earlier storage,key-byte,whole-memory or geometry/execution guarantees.
    /// # Errors
    /// N,D,A quota,owner,then first original candidate's undefined or inactive role. No prefix escapes.
    pub fn bind_active_roles<'a>(
        &'a self,
        catalog: &'a AdmittedOffsetRoleCatalog<'a>,
        limits: OffsetRolePresenceLimits,
    ) -> Result<ActiveOffsetCandidates<'a>, OffsetRolePresenceError<'a>> {
        let refuse = |refusal| OffsetRolePresenceError {
            admitted: self,
            catalog,
            limits,
            refusal,
        };
        let n = self.fragments().len();
        let d = catalog.catalog().definitions().len();
        let a = catalog.catalog().active_tags().len();
        if n > limits.candidates {
            return Err(refuse(OffsetRolePresenceRefusal::Candidates {
                required: n,
                allowed: limits.candidates,
            }));
        }
        if d > limits.definitions {
            return Err(refuse(OffsetRolePresenceRefusal::Definitions {
                required: d,
                allowed: limits.definitions,
            }));
        }
        if a > limits.active {
            return Err(refuse(OffsetRolePresenceRefusal::Active {
                required: a,
                allowed: limits.active,
            }));
        }
        if self.manifest().operation() != catalog.catalog().operation() {
            return Err(refuse(OffsetRolePresenceRefusal::Operation));
        }
        let mut pairs = Vec::with_capacity(n);
        for (candidate, fragment) in self.fragments().iter().enumerate() {
            let tag = fragment.tag();
            if catalog.definition(tag).is_none() {
                return Err(refuse(OffsetRolePresenceRefusal::Undefined {
                    candidate,
                    tag,
                }));
            }
            let role = catalog
                .active_role(tag)
                .ok_or_else(|| refuse(OffsetRolePresenceRefusal::Inactive { candidate, tag }))?;
            pairs.push((fragment, role));
        }
        Ok(ActiveOffsetCandidates {
            admitted: self,
            catalog,
            limits,
            pairs,
        })
    }
}

/// Complete readonly candidate-to-active-role association,not evaluated geometry or currentness.
/// ```compile_fail
/// fn escape<'a>(admitted:&'a sc_core::construction::AdmittedOffsetCandidates<'a>)
///     ->sc_core::construction::ActiveOffsetCandidates<'a> {
///     use sc_core::{construction::*,ontology::*};
///     let raw=OffsetRoleCatalog::new(EntityId::from_bits(10),vec![],vec![]);
///     let catalog=raw.admit(OffsetRoleCatalogLimits::new(0,0,0)).unwrap();
///     admitted.bind_active_roles(&catalog,OffsetRolePresenceLimits::new(1,0,0)).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct ActiveOffsetCandidates<'a> {
    admitted: &'a AdmittedOffsetCandidates<'a>,
    catalog: &'a AdmittedOffsetRoleCatalog<'a>,
    limits: OffsetRolePresenceLimits,
    pairs: Vec<(&'a OffsetCandidateFragment, ActiveOffsetRole<'a>)>,
}
impl<'a> ActiveOffsetCandidates<'a> {
    /// Actual admitted candidate owner.
    #[must_use]
    pub const fn admitted(&self) -> &'a AdmittedOffsetCandidates<'a> {
        self.admitted
    }
    /// Actual whole catalogue,including roles absent from this manifest.
    #[must_use]
    pub const fn catalog(&self) -> &'a AdmittedOffsetRoleCatalog<'a> {
        self.catalog
    }
    /// Original explicit association policy.
    #[must_use]
    pub const fn limits(&self) -> OffsetRolePresenceLimits {
        self.limits
    }
    /// N retained association entries,not allocation bytes or prior catalogue workspace.
    #[must_use]
    pub fn workspace_entries(&self) -> usize {
        self.pairs.len()
    }
    /// Every actual association in original candidate order;catalogue order may differ.
    pub fn outputs(
        &self,
    ) -> impl ExactSizeIterator<Item = ActiveOffsetCandidate<'_>> + DoubleEndedIterator {
        self.pairs
            .iter()
            .enumerate()
            .map(move |(index, (fragment, role))| ActiveOffsetCandidate {
                presence: self,
                index,
                fragment,
                role: *role,
            })
    }
}
/// Authentic candidate occurrence and canonical active field,borrowing the complete association.
/// ```compile_fail
/// fn reparent<'a>(mut output:sc_core::construction::ActiveOffsetCandidate<'a>,other:&'a sc_core::construction::ActiveOffsetCandidates<'a>) {
///     output.presence=other;
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct ActiveOffsetCandidate<'a> {
    presence: &'a ActiveOffsetCandidates<'a>,
    index: usize,
    fragment: &'a OffsetCandidateFragment,
    role: ActiveOffsetRole<'a>,
}
impl<'a> ActiveOffsetCandidate<'a> {
    /// Actual complete association supplying this view.
    #[must_use]
    pub const fn presence(self) -> &'a ActiveOffsetCandidates<'a> {
        self.presence
    }
    /// Actual original candidate occurrence index,not persistent identity.
    #[must_use]
    pub const fn index(self) -> usize {
        self.index
    }
    /// Actual original candidate field and source interval.
    #[must_use]
    pub const fn fragment(self) -> &'a OffsetCandidateFragment {
        self.fragment
    }
    /// Actual original canonical active-list field and its definition.
    #[must_use]
    pub const fn role(self) -> ActiveOffsetRole<'a> {
        self.role
    }
    /// Logical unissued edge role,without geometry/currentness/declaration or publication proof.
    #[must_use]
    pub fn logical_output(self) -> EdgeRef {
        EdgeRef::new(
            self.presence.admitted.manifest().operation(),
            self.fragment.tag(),
        )
    }
}
