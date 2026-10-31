//! Explicit exact-field sharing and retained-identity policy,without creating or replacing Notches.
use super::{
    AdmittedWalkNotchIdentities, BoundWalkNotchAssignment, WalkNotchIdentityAction,
    WalkNotchIdentitySource,
};
use crate::ontology::{EdgeAnchor, EntityId, LocalTag, NotchProfileBindings, Piece, SeamSideId};
use core::fmt;
use std::collections::BTreeMap;

/// Explicit occurrence policy;equal coordinates or repeated roles never select it automatically.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WalkNotchSharingPolicy {
    /// Exactly one original occurrence must use this role.
    SingleOccurrence,
    /// One or more occurrences must have identical actual pattern owner,anchor and profile fields.
    CoalesceIdentical,
}
/// Explicit fields for the previously chosen semantic identity;no replacement authority is granted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WalkNotchFieldPolicy {
    /// Proposed new identity takes the common original requested fields.
    NewRequested,
    /// Retained identity must already have the exact requested owner,anchor and profiles.
    RetainUnchanged,
    /// Retained identity explicitly proposes requested anchor/profile revisions on its same Piece.
    ReviseRetained,
}
/// Original per-role sharing and identity-field choices,not an accepted semantic result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WalkNotchFieldIntent {
    role: LocalTag,
    sharing: WalkNotchSharingPolicy,
    fields: WalkNotchFieldPolicy,
}
impl WalkNotchFieldIntent {
    /// Retain explicit choices without defaults or validation.
    #[must_use]
    pub const fn new(
        role: LocalTag,
        sharing: WalkNotchSharingPolicy,
        fields: WalkNotchFieldPolicy,
    ) -> Self {
        Self {
            role,
            sharing,
            fields,
        }
    }
    /// Original role field.
    #[must_use]
    pub const fn role(&self) -> &LocalTag {
        &self.role
    }
    /// Explicit original occurrence policy.
    #[must_use]
    pub const fn sharing(&self) -> WalkNotchSharingPolicy {
        self.sharing
    }
    /// Explicit original identity-field policy.
    #[must_use]
    pub const fn fields(&self) -> WalkNotchFieldPolicy {
        self.fields
    }
}
/// Whole ordered sharing/field proposal for all present semantic identity roles.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WalkNotchFieldManifest {
    operation: EntityId,
    intents: Vec<WalkNotchFieldIntent>,
}
impl WalkNotchFieldManifest {
    /// Own original host choices,without accepted fields or effects.
    #[must_use]
    pub const fn new(operation: EntityId, intents: Vec<WalkNotchFieldIntent>) -> Self {
        Self { operation, intents }
    }
    /// Original operation field.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        self.operation
    }
    /// Complete original ordered choices.
    #[must_use]
    pub fn intents(&self) -> &[WalkNotchFieldIntent] {
        &self.intents
    }
}
/// Explicit complete identity,assignment,field-policy and temporary logical-workspace allowances.
/// ```compile_fail
/// let limits:sc_core::construction::WalkNotchFieldLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WalkNotchFieldLimits {
    identities: usize,
    assignments: usize,
    policies: usize,
    workspace_entries: usize,
}
impl WalkNotchFieldLimits {
    /// Select complete E/M/F and checked E+2F allowances;entries are not bytes or prior storage.
    #[must_use]
    pub const fn new(
        identities: usize,
        assignments: usize,
        policies: usize,
        workspace_entries: usize,
    ) -> Self {
        Self {
            identities,
            assignments,
            policies,
            workspace_entries,
        }
    }
    /// Complete actual identity choices.
    #[must_use]
    pub const fn identities(self) -> usize {
        self.identities
    }
    /// Complete actual original assignments.
    #[must_use]
    pub const fn assignments(self) -> usize {
        self.assignments
    }
    /// Complete original policy choices.
    #[must_use]
    pub const fn policies(self) -> usize {
        self.policies
    }
    /// Peak new logical-entry upper bound.
    #[must_use]
    pub const fn workspace_entries(self) -> usize {
        self.workspace_entries
    }
}
/// Exact canonical field that disagrees;no approximate geometric comparison is performed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WalkNotchField {
    /// Actual pattern Piece identity,not physical-copy identity.
    Piece,
    /// Exact original held edge reference and parameter.
    Anchor,
    /// Complete symbolic sample/production/style/encoding profile bindings.
    Profiles,
}
/// Whole resource,original policy or original source-order field/sharing refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WalkNotchFieldRefusal {
    /// E+2F cannot be represented.
    WorkspaceOverflow,
    /// Whole original assignments exceed the explicit allowance.
    Assignments {
        /// Complete actual count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole actual semantic identity choices exceed the allowance.
    Identities {
        /// Complete actual count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole original field policies exceed the allowance.
    Policies {
        /// Complete actual count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete logical workspace upper bound exceeds the allowance.
    Workspace {
        /// Checked complete bound.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Original policy operation differs from the actual walk operation.
    Operation,
    /// Two original choices name one role.
    DuplicateRole {
        /// First original position.
        first: usize,
        /// Repeated original position.
        second: usize,
        /// Repeated role.
        role: LocalTag,
    },
    /// Original choice names no present identity role.
    UnassignedRole {
        /// Original policy position.
        index: usize,
        /// Unassigned role.
        role: LocalTag,
    },
    /// New/retained field policy disagrees with the actual prior identity choice.
    IdentityKind {
        /// Original policy position.
        index: usize,
        /// Actual role.
        role: LocalTag,
    },
    /// First original source occurrence has no field policy.
    MissingRole {
        /// Original input pair.
        pair: usize,
        /// Original A/B field.
        side: SeamSideId,
        /// Required role.
        role: LocalTag,
    },
    /// Explicit single-occurrence policy encountered a second occurrence.
    RepeatedOccurrence {
        /// First original pair.
        first_pair: usize,
        /// First original side.
        first_side: SeamSideId,
        /// Repeated original pair.
        pair: usize,
        /// Repeated original side.
        side: SeamSideId,
        /// Actual role.
        role: LocalTag,
    },
    /// An explicitly coalesced group has unequal actual canonical fields.
    GroupFields {
        /// First original pair.
        first_pair: usize,
        /// First original side.
        first_side: SeamSideId,
        /// Conflicting original pair.
        pair: usize,
        /// Conflicting original side.
        side: SeamSideId,
        /// Actual role.
        role: LocalTag,
        /// First differing field in Piece/anchor/profile order.
        field: WalkNotchField,
    },
    /// Actual prior Notch violates the explicit unchanged policy or retained Piece owner.
    RetainedFields {
        /// First original pair for this role.
        pair: usize,
        /// First original side for this role.
        side: SeamSideId,
        /// Actual role.
        role: LocalTag,
        /// First differing field in Piece/anchor/profile order.
        field: WalkNotchField,
    },
}
/// Whole original field proposal,actual complete identity/source parents,policy and first refusal.
pub struct WalkNotchFieldError<'a> {
    manifest: &'a WalkNotchFieldManifest,
    identities: &'a AdmittedWalkNotchIdentities<'a>,
    limits: WalkNotchFieldLimits,
    refusal: WalkNotchFieldRefusal,
}
impl<'a> WalkNotchFieldError<'a> {
    /// Actual complete original field/sharing proposal.
    #[must_use]
    pub const fn manifest(&self) -> &'a WalkNotchFieldManifest {
        self.manifest
    }
    /// Actual complete identities and all original source parents.
    #[must_use]
    pub const fn identities(&self) -> &'a AdmittedWalkNotchIdentities<'a> {
        self.identities
    }
    /// Explicit original policy.
    #[must_use]
    pub const fn limits(&self) -> WalkNotchFieldLimits {
        self.limits
    }
    /// First whole resource,original policy or source-order cause.
    #[must_use]
    pub const fn refusal(&self) -> WalkNotchFieldRefusal {
        self.refusal
    }
}
impl fmt::Debug for WalkNotchFieldError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WalkNotchFieldError")
            .field("operation", &self.manifest.operation)
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for WalkNotchFieldError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "walk semantic field policy: {:?}", self.refusal)
    }
}
impl std::error::Error for WalkNotchFieldError<'_> {}
#[derive(Debug)]
struct Group<'a> {
    first: BoundWalkNotchAssignment<'a>,
    count: usize,
}
#[derive(Debug)]
struct Association<'a> {
    index: usize,
    intent: &'a WalkNotchFieldIntent,
    identity: WalkNotchIdentityAction<'a>,
    first: BoundWalkNotchAssignment<'a>,
    count: usize,
}
fn difference(
    a: BoundWalkNotchAssignment<'_>,
    b: BoundWalkNotchAssignment<'_>,
) -> Option<WalkNotchField> {
    if a.piece().id() != b.piece().id() {
        Some(WalkNotchField::Piece)
    } else if a.requested().anchor() != b.requested().anchor() {
        Some(WalkNotchField::Anchor)
    } else if a.requested().representation() != b.requested().representation() {
        Some(WalkNotchField::Profiles)
    } else {
        None
    }
}
impl WalkNotchFieldManifest {
    /// Check explicit field kinds,complete source coverage and exact permitted group agreement.
    /// E+2F and complete M/E/F/workspace quotas precede operation equality and all scans/allocation.
    /// Original policy checks precede source pair/A/B checks. Retained Piece owners cannot change;
    /// ReviseRetained proposes anchor/profile revisions only. No prior object is mutated or cloned.
    /// Work is O((M+E+F)log(M+E+F+2));workspace covers F policy-index,E group and F association entries.
    /// No geometry,currentness,all-copy suitability,identity reservation or replacement is accepted.
    /// # Errors
    /// Whole resource/operation;first original policy;then first source-order omission or disagreement.
    pub fn admit<'a>(
        &'a self,
        identities: &'a AdmittedWalkNotchIdentities<'a>,
        limits: WalkNotchFieldLimits,
    ) -> Result<AdmittedWalkNotchFields<'a>, WalkNotchFieldError<'a>> {
        use WalkNotchFieldRefusal as R;
        let refuse = |refusal| WalkNotchFieldError {
            manifest: self,
            identities,
            limits,
            refusal,
        };
        let assignments = identities.known().existing().assignments();
        let (e, m, f) = (
            identities.group_count(),
            assignments.assignment_count(),
            self.intents.len(),
        );
        let workspace = f
            .checked_mul(2)
            .and_then(|v| v.checked_add(e))
            .ok_or_else(|| refuse(R::WorkspaceOverflow))?;
        if m > limits.assignments {
            return Err(refuse(R::Assignments {
                required: m,
                allowed: limits.assignments,
            }));
        }
        if e > limits.identities {
            return Err(refuse(R::Identities {
                required: e,
                allowed: limits.identities,
            }));
        }
        if f > limits.policies {
            return Err(refuse(R::Policies {
                required: f,
                allowed: limits.policies,
            }));
        }
        if workspace > limits.workspace_entries {
            return Err(refuse(R::Workspace {
                required: workspace,
                allowed: limits.workspace_entries,
            }));
        }
        if self.operation != identities.manifest().operation() {
            return Err(refuse(R::Operation));
        }
        let mut by_role = BTreeMap::new();
        for (index, intent) in self.intents.iter().enumerate() {
            if let Some(first) = by_role.insert(intent.role, index) {
                return Err(refuse(R::DuplicateRole {
                    first,
                    second: index,
                    role: intent.role,
                }));
            }
            let identity = identities.action(intent.role).ok_or_else(|| {
                refuse(R::UnassignedRole {
                    index,
                    role: intent.role,
                })
            })?;
            if !matches!(
                (identity.source(), intent.fields),
                (
                    WalkNotchIdentitySource::Proposed(_),
                    WalkNotchFieldPolicy::NewRequested
                ) | (
                    WalkNotchIdentitySource::Existing(_),
                    WalkNotchFieldPolicy::RetainUnchanged | WalkNotchFieldPolicy::ReviseRetained
                )
            ) {
                return Err(refuse(R::IdentityKind {
                    index,
                    role: intent.role,
                }));
            }
        }
        let mut groups: BTreeMap<LocalTag, Group<'a>> = BTreeMap::new();
        for item in assignments.requests() {
            let role = *item.role().tag();
            let pair = item.assignment().pair();
            let side = item.assignment().side();
            let intent = by_role
                .get(&role)
                .and_then(|i| self.intents.get(*i))
                .ok_or_else(|| refuse(R::MissingRole { pair, side, role }))?;
            if let Some(group) = groups.get_mut(&role) {
                let first_pair = group.first.assignment().pair();
                let first_side = group.first.assignment().side();
                if intent.sharing == WalkNotchSharingPolicy::SingleOccurrence {
                    return Err(refuse(R::RepeatedOccurrence {
                        first_pair,
                        first_side,
                        pair,
                        side,
                        role,
                    }));
                }
                if let Some(field) = difference(group.first, item) {
                    return Err(refuse(R::GroupFields {
                        first_pair,
                        first_side,
                        pair,
                        side,
                        role,
                        field,
                    }));
                }
                group.count += 1;
            } else {
                let identity = identities
                    .action(role)
                    .ok_or_else(|| refuse(R::MissingRole { pair, side, role }))?;
                if let WalkNotchIdentitySource::Existing(prior) = identity.source() {
                    let field = if prior.piece() != item.piece().id() {
                        Some(WalkNotchField::Piece)
                    } else if intent.fields == WalkNotchFieldPolicy::RetainUnchanged {
                        if prior.definition().anchor != *item.requested().anchor() {
                            Some(WalkNotchField::Anchor)
                        } else if prior.definition().representation
                            != *item.requested().representation()
                        {
                            Some(WalkNotchField::Profiles)
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    if let Some(field) = field {
                        return Err(refuse(R::RetainedFields {
                            pair,
                            side,
                            role,
                            field,
                        }));
                    }
                }
                groups.insert(
                    role,
                    Group {
                        first: item,
                        count: 1,
                    },
                );
            }
        }
        let mut associations = Vec::with_capacity(f);
        for (index, intent) in self.intents.iter().enumerate() {
            let absent = || {
                refuse(R::UnassignedRole {
                    index,
                    role: intent.role,
                })
            };
            let group = groups.get(&intent.role).ok_or_else(absent)?;
            let identity = identities.action(intent.role).ok_or_else(absent)?;
            associations.push(Association {
                index,
                intent,
                identity,
                first: group.first,
                count: group.count,
            });
        }
        Ok(AdmittedWalkNotchFields {
            manifest: self,
            identities,
            limits,
            workspace,
            by_role,
            associations,
        })
    }
}
/// Complete exact field-policy agreement with actual parents;no immutable Notch or applied effect.
/// ```compile_fail
/// fn reparent<'a>(mut fields:sc_core::construction::AdmittedWalkNotchFields<'a>,
///     other:&'a sc_core::construction::AdmittedWalkNotchIdentities<'a>) {fields.identities=other;}
/// ```
/// ```compile_fail
/// fn escape(raw:sc_core::construction::WalkNotchFieldManifest,ids:sc_core::construction::AdmittedWalkNotchIdentities<'_>)
///     ->sc_core::construction::AdmittedWalkNotchFields<'static> {
///     raw.admit(&ids,sc_core::construction::WalkNotchFieldLimits::new(0,0,0,0)).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct AdmittedWalkNotchFields<'a> {
    manifest: &'a WalkNotchFieldManifest,
    identities: &'a AdmittedWalkNotchIdentities<'a>,
    limits: WalkNotchFieldLimits,
    workspace: usize,
    by_role: BTreeMap<LocalTag, usize>,
    associations: Vec<Association<'a>>,
}
impl<'a> AdmittedWalkNotchFields<'a> {
    /// Actual complete original field/sharing choices.
    #[must_use]
    pub const fn manifest(&self) -> &'a WalkNotchFieldManifest {
        self.manifest
    }
    /// Actual complete original identity/source parents.
    #[must_use]
    pub const fn identities(&self) -> &'a AdmittedWalkNotchIdentities<'a> {
        self.identities
    }
    /// Original explicit whole-resource policy.
    #[must_use]
    pub const fn limits(&self) -> WalkNotchFieldLimits {
        self.limits
    }
    /// Conservative E+2F temporary new logical-entry upper bound.
    #[must_use]
    pub const fn workspace_bound(&self) -> usize {
        self.workspace
    }
    /// Complete semantic field groups,not applied or physically suitable marks.
    #[must_use]
    pub fn group_count(&self) -> usize {
        self.associations.len()
    }
    /// Authentic field groups in original policy order,including reverse traversal.
    pub fn groups(
        &self,
    ) -> impl ExactSizeIterator<Item = WalkNotchFieldGroup<'_>> + DoubleEndedIterator {
        self.associations
            .iter()
            .map(|association| WalkNotchFieldGroup {
                owner: self,
                association,
            })
    }
    /// Authentic field group for a present role,in O(log(F+2)) work.
    #[must_use]
    pub fn group(&self, role: LocalTag) -> Option<WalkNotchFieldGroup<'_>> {
        self.by_role
            .get(&role)
            .and_then(|i| self.associations.get(*i))
            .map(|association| WalkNotchFieldGroup {
                owner: self,
                association,
            })
    }
}
/// Actual common field borrows after all group agreement succeeds,never an unchecked representative.
/// ```compile_fail
/// fn reparent<'a>(mut group:sc_core::construction::WalkNotchFieldGroup<'a>,
///     other:&'a sc_core::construction::AdmittedWalkNotchFields<'a>) {group.owner=other;}
/// ```
#[derive(Clone, Copy, Debug)]
pub struct WalkNotchFieldGroup<'a> {
    owner: &'a AdmittedWalkNotchFields<'a>,
    association: &'a Association<'a>,
}
impl<'a> WalkNotchFieldGroup<'a> {
    /// Actual complete accepted policy owner.
    #[must_use]
    pub const fn owner(self) -> &'a AdmittedWalkNotchFields<'a> {
        self.owner
    }
    /// Original policy position,not persistent semantic identity.
    #[must_use]
    pub const fn index(self) -> usize {
        self.association.index
    }
    /// Actual original explicit sharing and field choices.
    #[must_use]
    pub const fn intent(self) -> &'a WalkNotchFieldIntent {
        self.association.intent
    }
    /// Authentic actual retained/proposed identity choice.
    #[must_use]
    pub const fn identity(self) -> WalkNotchIdentityAction<'a> {
        self.association.identity
    }
    /// First original source occurrence after complete exact group agreement.
    #[must_use]
    pub const fn first(self) -> BoundWalkNotchAssignment<'a> {
        self.association.first
    }
    /// All original occurrences explicitly sharing these exact fields.
    #[must_use]
    pub const fn occurrence_count(self) -> usize {
        self.association.count
    }
    /// Actual common pattern owner,not a physical-copy or whole-copy-effect certificate.
    #[must_use]
    pub const fn piece(self) -> &'a Piece {
        self.association.first.piece()
    }
    /// Actual original common anchor field,not a reconstructed or geometric location.
    #[must_use]
    pub const fn anchor(self) -> &'a EdgeAnchor {
        self.association.first.requested().anchor()
    }
    /// All actual common symbolic profile fields,not resolved physical defaults.
    #[must_use]
    pub const fn representation(self) -> &'a NotchProfileBindings {
        self.association.first.requested().representation()
    }
}
