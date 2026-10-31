//! Complete explicit host identity/retention proposals;no Notch fields or global issuance are derived.
use super::BoundWalkNotchIdentityExclusions;
use crate::{
    construction::ActiveNotchOutputRole,
    ontology::{EntityId, LocalTag, Notch, SeamSideId},
};
use core::fmt;
use std::collections::BTreeMap;
/// Explicit semantic identity choice for one actually present role,not a client mutation request.
/// Retain keeps an existing identity with its actual prior object as context. It grants neither
/// replacement authority nor an unchanged-result certificate;field policy/sharing/effects follow.
/// Propose is an explicit trusted-host reserved-id proposal,not actual reservation or issuance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WalkNotchIdentityIntent {
    /// Retain the explicit supplied semantic identity and actual prior canonical object.
    Retain {
        /// Present declared output role.
        role: LocalTag,
        /// Actual supplied prior semantic identity.
        notch: EntityId,
    },
    /// Propose an explicit host identity for this present role,subject to all global checks.
    Propose {
        /// Present declared output role.
        role: LocalTag,
        /// Original host identity proposal,not a generated id.
        id: EntityId,
    },
}
impl WalkNotchIdentityIntent {
    /// Actual original local role field,not a semantic EntityId.
    #[must_use]
    pub const fn role(&self) -> &LocalTag {
        match self {
            Self::Retain { role, .. } | Self::Propose { role, .. } => role,
        }
    }
    /// Actual original retained/proposed semantic id field,without allocation or conversion.
    #[must_use]
    pub const fn id(&self) -> &EntityId {
        match self {
            Self::Retain { notch, .. } | Self::Propose { id: notch, .. } => notch,
        }
    }
}
/// Complete ordered identity proposal,not an applied edit or a new-id-bearing client request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WalkNotchIdentityManifest {
    operation: EntityId,
    intents: Vec<WalkNotchIdentityIntent>,
}
impl WalkNotchIdentityManifest {
    /// Own explicit original host plan fields,without acceptance or earlier-allocation guarantees.
    #[must_use]
    pub const fn new(operation: EntityId, intents: Vec<WalkNotchIdentityIntent>) -> Self {
        Self { operation, intents }
    }
    /// Explicit original operation,not a current-kind certificate.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        self.operation
    }
    /// All original ordered identity/retention proposals.
    #[must_use]
    pub fn intents(&self) -> &[WalkNotchIdentityIntent] {
        &self.intents
    }
}
/// Explicit complete identity-entry,original-assignment and new logical-workspace allowances.
/// ```compile_fail
/// let limits:sc_core::construction::WalkNotchIdentityLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WalkNotchIdentityLimits {
    entries: usize,
    assignments: usize,
    workspace_entries: usize,
}
impl WalkNotchIdentityLimits {
    /// Select complete E/M and checked3E+M upper bounds. These are logical entries,not bytes,
    /// earlier source/index storage,identity reservation or a whole execution budget.
    #[must_use]
    pub const fn new(entries: usize, assignments: usize, workspace_entries: usize) -> Self {
        Self {
            entries,
            assignments,
            workspace_entries,
        }
    }
    /// Complete original identity-plan population allowance.
    #[must_use]
    pub const fn entries(self) -> usize {
        self.entries
    }
    /// Complete actual original occurrence-attribution population allowance.
    #[must_use]
    pub const fn assignments(self) -> usize {
        self.assignments
    }
    /// Peak new logical identity/role/association/source-role upper bound allowance.
    #[must_use]
    pub const fn workspace_entries(self) -> usize {
        self.workspace_entries
    }
}
/// Whole resource,operation,original identity-entry or complete present-role coverage refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WalkNotchIdentityRefusal {
    /// Checked3E+M cannot be represented,before scans or allocation.
    WorkspaceOverflow,
    /// Whole actual original assignments exceed policy.
    Assignments {
        /// Complete actual count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole original identity plan exceeds policy.
    Entries {
        /// Complete actual count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete new logical workspace upper bound exceeds policy.
    Workspace {
        /// Checked complete bound.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Proposed operation differs from the actual original walk operation.
    Operation,
    /// A present role has two original identity proposals.
    DuplicateRole {
        /// Original first entry.
        first: usize,
        /// Original repeated entry.
        second: usize,
        /// Repeated local role.
        role: LocalTag,
    },
    /// Original entry names no role used by any actual assigned occurrence.
    UnassignedRole {
        /// Original entry position.
        index: usize,
        /// Unused/unknown/inactive local role.
        role: LocalTag,
    },
    /// Two identity choices use the same semantic id,including across both choice kinds.
    DuplicateId {
        /// Original first entry.
        first: usize,
        /// Original repeated entry.
        second: usize,
        /// Repeated semantic id.
        id: EntityId,
    },
    /// Retain names no supplied existing canonical Notch;not global absence proof.
    MissingExisting {
        /// Original entry position.
        index: usize,
        /// Required provided identity.
        id: EntityId,
    },
    /// Proposed new identity names an explicit known inspected reference,including old Notches.
    KnownProposalId {
        /// Original entry position.
        index: usize,
        /// Known conflicting identity.
        id: EntityId,
    },
    /// First source pair/A/B role has no original identity proposal.
    MissingRole {
        /// Actual original input pair position.
        pair: usize,
        /// Actual A/B request field.
        side: SeamSideId,
        /// Required present output role.
        role: LocalTag,
    },
}
/// Whole original plan,actual existing/exclusion/assignment/source context,policy and first refusal.
pub struct WalkNotchIdentityError<'a> {
    manifest: &'a WalkNotchIdentityManifest,
    known: &'a BoundWalkNotchIdentityExclusions<'a>,
    limits: WalkNotchIdentityLimits,
    refusal: WalkNotchIdentityRefusal,
}
impl<'a> WalkNotchIdentityError<'a> {
    /// Actual complete original host proposal,not a prefix.
    #[must_use]
    pub const fn manifest(&self) -> &'a WalkNotchIdentityManifest {
        self.manifest
    }
    /// Actual complete scoped exclusions,provided Notches and original attribution/source parents.
    #[must_use]
    pub const fn known(&self) -> &'a BoundWalkNotchIdentityExclusions<'a> {
        self.known
    }
    /// Original explicit whole-resource policy.
    #[must_use]
    pub const fn limits(&self) -> WalkNotchIdentityLimits {
        self.limits
    }
    /// First whole resource,original entry or complete present-role cause.
    #[must_use]
    pub const fn refusal(&self) -> WalkNotchIdentityRefusal {
        self.refusal
    }
}
impl fmt::Debug for WalkNotchIdentityError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WalkNotchIdentityError")
            .field("operation", &self.manifest.operation)
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for WalkNotchIdentityError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "walk semantic identity proposals: {:?}", self.refusal)
    }
}
impl std::error::Error for WalkNotchIdentityError<'_> {}
/// Borrowed data only;this enum alone proves no membership,identity-plan association or field effect.
#[derive(Clone, Copy, Debug)]
pub enum WalkNotchIdentitySource<'a> {
    /// Actual provided prior Notch;retained identity does not authorize changing its held fields.
    Existing(&'a Notch),
    /// Actual original host identity proposal;no immutable Notch is manufactured.
    Proposed(&'a EntityId),
}
#[derive(Debug)]
struct Association<'a> {
    index: usize,
    intent: &'a WalkNotchIdentityIntent,
    role: ActiveNotchOutputRole<'a>,
    source: WalkNotchIdentitySource<'a>,
}
impl WalkNotchIdentityManifest {
    /// Bind every actually present role to one explicit retained/proposed distinct semantic id.
    /// Checked3E+M,whole M/E/workspace quotas and operation precede all scans/allocation. An M-entry
    /// source-role map,up to E role/id entries each and E associations fit the bound. Original entry
    /// order checks duplicate role,actual assigned-role membership,duplicate id,then supplied existing
    /// object or scoped known-id exclusion. Complete coverage follows original input pair/A/B order.
    /// No Notch fields are derived and no semantic sharing/currentness/reservation/issuance/effect is
    /// accepted. Unmentioned supplied objects stay untouched;absence retires no entity or role.
    /// # Errors
    /// Whole resource/operation;first original entry;then first present source role lacking a choice.
    pub fn admit<'a>(
        &'a self,
        known: &'a BoundWalkNotchIdentityExclusions<'a>,
        limits: WalkNotchIdentityLimits,
    ) -> Result<AdmittedWalkNotchIdentities<'a>, WalkNotchIdentityError<'a>> {
        use WalkNotchIdentityRefusal as R;
        let refuse = |refusal| WalkNotchIdentityError {
            manifest: self,
            known,
            limits,
            refusal,
        };
        let assignments = known.existing().assignments();
        let m = assignments.assignment_count();
        let e = self.intents.len();
        let workspace = e
            .checked_mul(3)
            .and_then(|count| count.checked_add(m))
            .ok_or_else(|| refuse(R::WorkspaceOverflow))?;
        if m > limits.assignments {
            return Err(refuse(R::Assignments {
                required: m,
                allowed: limits.assignments,
            }));
        }
        if e > limits.entries {
            return Err(refuse(R::Entries {
                required: e,
                allowed: limits.entries,
            }));
        }
        if workspace > limits.workspace_entries {
            return Err(refuse(R::Workspace {
                required: workspace,
                allowed: limits.workspace_entries,
            }));
        }
        if self.operation
            != assignments
                .roles()
                .copies()
                .source()
                .admitted()
                .edit()
                .operation()
        {
            return Err(refuse(R::Operation));
        }
        let mut source_roles = BTreeMap::new();
        for item in assignments.assignments() {
            source_roles
                .entry(*item.role().tag())
                .or_insert(item.role());
        }
        let mut by_role = BTreeMap::new();
        let mut by_id = BTreeMap::new();
        let mut associations = Vec::with_capacity(e);
        let mut retained = 0usize;
        let mut proposed = 0usize;
        for (index, intent) in self.intents.iter().enumerate() {
            if let Some(first) = by_role.insert(*intent.role(), index) {
                return Err(refuse(R::DuplicateRole {
                    first,
                    second: index,
                    role: *intent.role(),
                }));
            }
            let role = source_roles.get(intent.role()).copied().ok_or_else(|| {
                refuse(R::UnassignedRole {
                    index,
                    role: *intent.role(),
                })
            })?;
            if let Some(first) = by_id.insert(*intent.id(), index) {
                return Err(refuse(R::DuplicateId {
                    first,
                    second: index,
                    id: *intent.id(),
                }));
            }
            let source = match intent {
                WalkNotchIdentityIntent::Retain { notch, .. } => {
                    let actual = known
                        .existing()
                        .notch(*notch)
                        .ok_or_else(|| refuse(R::MissingExisting { index, id: *notch }))?;
                    retained += 1;
                    WalkNotchIdentitySource::Existing(actual)
                }
                WalkNotchIdentityIntent::Propose { id, .. } => {
                    if known.excludes_new_notch(*id) {
                        return Err(refuse(R::KnownProposalId { index, id: *id }));
                    }
                    proposed += 1;
                    WalkNotchIdentitySource::Proposed(id)
                }
            };
            associations.push(Association {
                index,
                intent,
                role,
                source,
            });
        }
        for request in assignments.requests() {
            if !by_role.contains_key(request.role().tag()) {
                return Err(refuse(R::MissingRole {
                    pair: request.assignment().pair(),
                    side: request.assignment().side(),
                    role: *request.role().tag(),
                }));
            }
        }
        Ok(AdmittedWalkNotchIdentities {
            manifest: self,
            known,
            limits,
            workspace,
            by_role,
            associations,
            retained,
            proposed,
        })
    }
}
/// Complete identity-proposal metadata with authentic original parents,not applied semantic effects.
/// ```compile_fail
/// fn reparent<'a>(mut identities:sc_core::construction::AdmittedWalkNotchIdentities<'a>,
///     other:&'a sc_core::construction::BoundWalkNotchIdentityExclusions<'a>) {identities.known=other;}
/// ```
/// ```compile_fail
/// fn escape(known:sc_core::construction::BoundWalkNotchIdentityExclusions<'_>,raw:sc_core::construction::WalkNotchIdentityManifest)
///     ->sc_core::construction::AdmittedWalkNotchIdentities<'static> {
///     raw.admit(&known,sc_core::construction::WalkNotchIdentityLimits::new(0,0,0)).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct AdmittedWalkNotchIdentities<'a> {
    manifest: &'a WalkNotchIdentityManifest,
    known: &'a BoundWalkNotchIdentityExclusions<'a>,
    limits: WalkNotchIdentityLimits,
    workspace: usize,
    by_role: BTreeMap<LocalTag, usize>,
    associations: Vec<Association<'a>>,
    retained: usize,
    proposed: usize,
}
impl<'a> AdmittedWalkNotchIdentities<'a> {
    /// Actual complete original explicit host identity/retention plan.
    #[must_use]
    pub const fn manifest(&self) -> &'a WalkNotchIdentityManifest {
        self.manifest
    }
    /// Actual complete known/existing/attribution/copy/role/source context.
    #[must_use]
    pub const fn known(&self) -> &'a BoundWalkNotchIdentityExclusions<'a> {
        self.known
    }
    /// Original explicit whole-resource policy.
    #[must_use]
    pub const fn limits(&self) -> WalkNotchIdentityLimits {
        self.limits
    }
    /// Checked3E+M temporary logical upper bound,not bytes or prior storage.
    #[must_use]
    pub const fn workspace_bound(&self) -> usize {
        self.workspace
    }
    /// Complete present role groups,not newly issued Notches or accepted shared marks.
    #[must_use]
    pub fn group_count(&self) -> usize {
        self.associations.len()
    }
    /// Complete retained semantic-id choices,not applied retention or unchanged-field proof.
    #[must_use]
    pub const fn retention_count(&self) -> usize {
        self.retained
    }
    /// Complete new host-id proposals,not reserved/issued identities or immutable Notches.
    #[must_use]
    pub const fn proposal_count(&self) -> usize {
        self.proposed
    }
    /// Every original identity entry in manifest order,including reverse traversal.
    pub fn actions(
        &self,
    ) -> impl ExactSizeIterator<Item = WalkNotchIdentityAction<'_>> + DoubleEndedIterator {
        self.associations
            .iter()
            .map(|association| WalkNotchIdentityAction {
                owner: self,
                association,
            })
    }
    /// Authentic original choice for a present local role,in O(log(E+2)) work.
    #[must_use]
    pub fn action(&self, role: LocalTag) -> Option<WalkNotchIdentityAction<'_>> {
        self.by_role
            .get(&role)
            .and_then(|index| self.associations.get(*index))
            .map(|association| WalkNotchIdentityAction {
                owner: self,
                association,
            })
    }
}
/// Authentic original host choice,actual active role and actual prior/proposed source data.
/// ```compile_fail
/// fn reparent<'a>(mut action:sc_core::construction::WalkNotchIdentityAction<'a>,
///     other:&'a sc_core::construction::AdmittedWalkNotchIdentities<'a>) {action.owner=other;}
/// ```
#[derive(Clone, Copy, Debug)]
pub struct WalkNotchIdentityAction<'a> {
    owner: &'a AdmittedWalkNotchIdentities<'a>,
    association: &'a Association<'a>,
}
impl<'a> WalkNotchIdentityAction<'a> {
    /// Actual complete original identity/attribution/source owner.
    #[must_use]
    pub const fn owner(self) -> &'a AdmittedWalkNotchIdentities<'a> {
        self.owner
    }
    /// Original host manifest position,not a semantic identity.
    #[must_use]
    pub const fn index(self) -> usize {
        self.association.index
    }
    /// Actual original explicit role/retained-or-proposed id fields.
    #[must_use]
    pub const fn intent(self) -> &'a WalkNotchIdentityIntent {
        self.association.intent
    }
    /// Actual original retained/proposed id field,not a generated or reserved value.
    #[must_use]
    pub const fn id(self) -> &'a EntityId {
        self.association.intent.id()
    }
    /// Authentic active membership in the actual source notch catalogue.
    #[must_use]
    pub const fn role(self) -> ActiveNotchOutputRole<'a> {
        self.association.role
    }
    /// Actual provided prior object or raw host id proposal;no field effect or issuance is accepted.
    #[must_use]
    pub const fn source(self) -> WalkNotchIdentitySource<'a> {
        self.association.source
    }
}
