//! Complete original walk occurrence-to-role attribution;grouping proposals are not semantic merges.
use super::{BoundWalkNotchRoles, WalkBalanceNotch, WalkBalancePair};
use crate::{
    construction::ActiveNotchOutputRole,
    ontology::{CutCopy, EntityId, LocalTag, Piece, SeamSideId},
};
use core::fmt;
use std::collections::BTreeMap;
/// One explicit original pair/A/B occurrence and proposed active output role. Pair position is input
/// attribution only,not a persistent semantic id or recipe statement ordinal. Repeated roles may propose
/// grouping;this raw entry alone supplies no catalogue membership,sharing or semantic identity proof.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WalkNotchRoleAssignment {
    pair: usize,
    side: SeamSideId,
    role: LocalTag,
}
impl WalkNotchRoleAssignment {
    /// Retain the explicit original occurrence and proposed local role without inference.
    #[must_use]
    pub const fn new(pair: usize, side: SeamSideId, role: LocalTag) -> Self {
        Self { pair, side, role }
    }
    /// Original zero-based input pair position,not a semantic id.
    #[must_use]
    pub const fn pair(self) -> usize {
        self.pair
    }
    /// Actual A/B request field.
    #[must_use]
    pub const fn side(self) -> SeamSideId {
        self.side
    }
    /// Actual original proposed local role field,not a Notch EntityId.
    #[must_use]
    pub const fn role(&self) -> &LocalTag {
        &self.role
    }
}
/// Complete raw ordered attribution proposal,not accepted output/sharing/identity metadata.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WalkNotchAssignmentManifest {
    operation: EntityId,
    assignments: Vec<WalkNotchRoleAssignment>,
}
impl WalkNotchAssignmentManifest {
    /// Own original operation and assignment order without admission or earlier-allocation proof.
    #[must_use]
    pub const fn new(operation: EntityId, assignments: Vec<WalkNotchRoleAssignment>) -> Self {
        Self {
            operation,
            assignments,
        }
    }
    /// Explicit original operation,not current-kind or global namespace proof.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        self.operation
    }
    /// All original proposed occurrences,in authored order.
    #[must_use]
    pub fn assignments(&self) -> &[WalkNotchRoleAssignment] {
        &self.assignments
    }
}
/// Explicit whole original pair/assignment/workspace-entry allowances.
/// ```compile_fail
/// let limits:sc_core::construction::WalkNotchAssignmentLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WalkNotchAssignmentLimits {
    pairs: usize,
    assignments: usize,
    workspace_entries: usize,
}
impl WalkNotchAssignmentLimits {
    /// Select whole N/M and checked2M index/association bounds,not bytes or earlier storage.
    #[must_use]
    pub const fn new(pairs: usize, assignments: usize, workspace_entries: usize) -> Self {
        Self {
            pairs,
            assignments,
            workspace_entries,
        }
    }
    /// Complete original input pair allowance.
    #[must_use]
    pub const fn pairs(self) -> usize {
        self.pairs
    }
    /// Complete original manifest population allowance.
    #[must_use]
    pub const fn assignments(self) -> usize {
        self.assignments
    }
    /// Peak new logical index plus association entry allowance.
    #[must_use]
    pub const fn workspace_entries(self) -> usize {
        self.workspace_entries
    }
}
/// Whole resource,operation,original proposal or complete source-occurrence refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WalkNotchAssignmentRefusal {
    /// Checked2M cannot be represented,before any scan or allocation.
    WorkspaceOverflow,
    /// Complete actual input pair population exceeds policy.
    Pairs {
        /// Complete actual pairs.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete manifest population exceeds policy.
    Assignments {
        /// Complete actual entries.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete2M new workspace exceeds policy.
    Workspace {
        /// Checked complete bound.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Manifest operation differs from the actual bound walk operation.
    Operation,
    /// First original proposal names no actual original input pair.
    Pair {
        /// Original manifest position.
        index: usize,
        /// Requested original pair position.
        pair: usize,
        /// Complete actual input pairs.
        available: usize,
    },
    /// Original pair/side already has a proposal,even when the second role differs.
    DuplicateOccurrence {
        /// First original manifest position.
        first: usize,
        /// Repeated original manifest position.
        second: usize,
        /// Actual original pair position.
        pair: usize,
        /// Actual A/B occurrence.
        side: SeamSideId,
    },
    /// First original proposal names no supplied role definition.
    UndefinedRole {
        /// Original manifest position.
        index: usize,
        /// Required explicit role.
        role: LocalTag,
    },
    /// First original proposal names an inactive supplied role.
    InactiveRole {
        /// Original manifest position.
        index: usize,
        /// Inactive explicit role.
        role: LocalTag,
    },
    /// First original source pair/A/B occurrence lacks any assignment.
    MissingOccurrence {
        /// Original source pair position.
        pair: usize,
        /// Actual missing A/B field.
        side: SeamSideId,
    },
}
/// Actual whole raw manifest,complete walk/role context,policy and first refusal.
pub struct WalkNotchAssignmentError<'a> {
    manifest: &'a WalkNotchAssignmentManifest,
    roles: &'a BoundWalkNotchRoles<'a>,
    limits: WalkNotchAssignmentLimits,
    refusal: WalkNotchAssignmentRefusal,
}
impl<'a> WalkNotchAssignmentError<'a> {
    /// Complete original raw proposal,not a prefix or reconstructed error payload.
    #[must_use]
    pub const fn manifest(&self) -> &'a WalkNotchAssignmentManifest {
        self.manifest
    }
    /// Actual complete walk/copy/pattern/typed-role context.
    #[must_use]
    pub const fn roles(&self) -> &'a BoundWalkNotchRoles<'a> {
        self.roles
    }
    /// Original explicit whole-resource policy.
    #[must_use]
    pub const fn limits(&self) -> WalkNotchAssignmentLimits {
        self.limits
    }
    /// First whole resource,original proposal or complete source cause.
    #[must_use]
    pub const fn refusal(&self) -> WalkNotchAssignmentRefusal {
        self.refusal
    }
}
impl fmt::Debug for WalkNotchAssignmentError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WalkNotchAssignmentError")
            .field("operation", &self.manifest.operation)
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for WalkNotchAssignmentError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "walk notch assignments: {:?}", self.refusal)
    }
}
impl std::error::Error for WalkNotchAssignmentError<'_> {}
fn key(pair: usize, side: SeamSideId) -> (usize, bool) {
    (pair, matches!(side, SeamSideId::B))
}
#[derive(Debug)]
struct Association<'a> {
    index: usize,
    assignment: &'a WalkNotchRoleAssignment,
    pair: &'a WalkBalancePair,
    role: ActiveNotchOutputRole<'a>,
}
impl WalkNotchAssignmentManifest {
    /// Admit complete original attribution only,not shared-mark semantics or EntityId assignment.
    /// Checked2M,N/M/workspace quotas and operation precede all scans/allocation. Each original entry
    /// checks pair bounds,duplicate occurrence,then undefined/inactive role. Complete source coverage
    /// follows input pair/A/B order. Repeated roles are grouping proposals;unused active metadata is
    /// legal. Work is O((M+N)log(M+D+2)),for D admitted definitions,with2M new logical entries.
    /// # Errors
    /// Whole resource/operation failure;first original entry failure;then first unassigned source field.
    pub fn admit<'a>(
        &'a self,
        roles: &'a BoundWalkNotchRoles<'a>,
        limits: WalkNotchAssignmentLimits,
    ) -> Result<AdmittedWalkNotchAssignments<'a>, WalkNotchAssignmentError<'a>> {
        use WalkNotchAssignmentRefusal as R;
        let refuse = |refusal| WalkNotchAssignmentError {
            manifest: self,
            roles,
            limits,
            refusal,
        };
        let pairs = roles.copies().source().admitted().edit().pairs();
        let n = pairs.len();
        let m = self.assignments.len();
        let workspace = m
            .checked_mul(2)
            .ok_or_else(|| refuse(R::WorkspaceOverflow))?;
        if n > limits.pairs {
            return Err(refuse(R::Pairs {
                required: n,
                allowed: limits.pairs,
            }));
        }
        if m > limits.assignments {
            return Err(refuse(R::Assignments {
                required: m,
                allowed: limits.assignments,
            }));
        }
        if workspace > limits.workspace_entries {
            return Err(refuse(R::Workspace {
                required: workspace,
                allowed: limits.workspace_entries,
            }));
        }
        if self.operation != roles.copies().source().admitted().edit().operation() {
            return Err(refuse(R::Operation));
        }
        let mut by_occurrence = BTreeMap::new();
        let mut associations = Vec::with_capacity(m);
        for (index, assignment) in self.assignments.iter().enumerate() {
            let pair = pairs.get(assignment.pair).ok_or_else(|| {
                refuse(R::Pair {
                    index,
                    pair: assignment.pair,
                    available: n,
                })
            })?;
            if let Some(first) = by_occurrence.insert(key(assignment.pair, assignment.side), index)
            {
                return Err(refuse(R::DuplicateOccurrence {
                    first,
                    second: index,
                    pair: assignment.pair,
                    side: assignment.side,
                }));
            }
            if roles.catalog().definition(assignment.role).is_none() {
                return Err(refuse(R::UndefinedRole {
                    index,
                    role: assignment.role,
                }));
            }
            let role = roles
                .catalog()
                .active_role(assignment.role)
                .ok_or_else(|| {
                    refuse(R::InactiveRole {
                        index,
                        role: assignment.role,
                    })
                })?;
            associations.push(Association {
                index,
                assignment,
                pair,
                role,
            });
        }
        for pair in 0..n {
            for side in [SeamSideId::A, SeamSideId::B] {
                if !by_occurrence.contains_key(&key(pair, side)) {
                    return Err(refuse(R::MissingOccurrence { pair, side }));
                }
            }
        }
        Ok(AdmittedWalkNotchAssignments {
            manifest: self,
            roles,
            limits,
            workspace,
            by_occurrence,
            associations,
        })
    }
}
/// Complete authentic original attribution;no accepted semantic coalescing or output entity follows.
/// ```compile_fail
/// fn reparent<'a>(mut assignments:sc_core::construction::AdmittedWalkNotchAssignments<'a>,
///     other:&'a sc_core::construction::BoundWalkNotchRoles<'a>) {assignments.roles=other;}
/// ```
/// ```compile_fail
/// fn escape(roles:sc_core::construction::BoundWalkNotchRoles<'_>,raw:sc_core::construction::WalkNotchAssignmentManifest)
///     ->sc_core::construction::AdmittedWalkNotchAssignments<'static> {
///     raw.admit(&roles,sc_core::construction::WalkNotchAssignmentLimits::new(0,0,0)).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct AdmittedWalkNotchAssignments<'a> {
    manifest: &'a WalkNotchAssignmentManifest,
    roles: &'a BoundWalkNotchRoles<'a>,
    limits: WalkNotchAssignmentLimits,
    workspace: usize,
    by_occurrence: BTreeMap<(usize, bool), usize>,
    associations: Vec<Association<'a>>,
}
impl<'a> AdmittedWalkNotchAssignments<'a> {
    /// Actual whole original proposal and authored assignment order.
    #[must_use]
    pub const fn manifest(&self) -> &'a WalkNotchAssignmentManifest {
        self.manifest
    }
    /// Actual complete bound walk/copy/pattern/notch-role context.
    #[must_use]
    pub const fn roles(&self) -> &'a BoundWalkNotchRoles<'a> {
        self.roles
    }
    /// Original explicitly selected whole-resource policy.
    #[must_use]
    pub const fn limits(&self) -> WalkNotchAssignmentLimits {
        self.limits
    }
    /// Complete2M index/association entry count,not bytes or prior storage.
    #[must_use]
    pub const fn workspace_entries(&self) -> usize {
        self.workspace
    }
    /// Complete original requested occurrences,not newly issued semantic identities.
    #[must_use]
    pub fn request_count(&self) -> usize {
        self.roles
            .copies()
            .source()
            .admitted()
            .notch_request_count()
    }
    /// Complete admitted original assignment population,not a semantic output count.
    #[must_use]
    pub fn assignment_count(&self) -> usize {
        self.associations.len()
    }
    /// Every original assignment in authored manifest order,with actual original fields/parents.
    pub fn assignments(
        &self,
    ) -> impl ExactSizeIterator<Item = BoundWalkNotchAssignment<'_>> + DoubleEndedIterator {
        self.associations
            .iter()
            .map(|association| BoundWalkNotchAssignment {
                owner: self,
                association,
            })
    }
    /// Actual original occurrence association selected in O(log(M+2)) work;positions are input
    /// attribution only,not persistent output identities or recipe statement positions.
    #[must_use]
    pub fn assignment(
        &self,
        pair: usize,
        side: SeamSideId,
    ) -> Option<BoundWalkNotchAssignment<'_>> {
        self.by_occurrence
            .get(&key(pair, side))
            .and_then(|index| self.associations.get(*index))
            .map(|association| BoundWalkNotchAssignment {
                owner: self,
                association,
            })
    }
    /// Every complete original request in source pair/A/B order,including repeated fields and roles.
    /// Reverse iteration reverses that order. Each traversal takes O(N log(M+2)) work,no allocation.
    pub fn requests(&self) -> impl DoubleEndedIterator<Item = BoundWalkNotchAssignment<'_>> {
        (0..self.request_count()).filter_map(|occurrence| {
            self.assignment(
                occurrence / 2,
                if occurrence % 2 == 0 {
                    SeamSideId::A
                } else {
                    SeamSideId::B
                },
            )
        })
    }
}
/// Authentic original input field and active-role proposal with the complete admitted owner.
/// ```compile_fail
/// fn reparent<'a>(mut item:sc_core::construction::BoundWalkNotchAssignment<'a>,
///     other:&'a sc_core::construction::AdmittedWalkNotchAssignments<'a>) {item.owner=other;}
/// ```
#[derive(Clone, Copy, Debug)]
pub struct BoundWalkNotchAssignment<'a> {
    owner: &'a AdmittedWalkNotchAssignments<'a>,
    association: &'a Association<'a>,
}
impl<'a> BoundWalkNotchAssignment<'a> {
    /// Actual complete original assignment and walk/copy/role owner.
    #[must_use]
    pub const fn owner(self) -> &'a AdmittedWalkNotchAssignments<'a> {
        self.owner
    }
    /// Original manifest position,not a semantic id.
    #[must_use]
    pub const fn index(self) -> usize {
        self.association.index
    }
    /// Actual original raw occurrence/role fields.
    #[must_use]
    pub const fn assignment(self) -> &'a WalkNotchRoleAssignment {
        self.association.assignment
    }
    /// Actual complete original A/B pair field.
    #[must_use]
    pub const fn pair(self) -> &'a WalkBalancePair {
        self.association.pair
    }
    /// Actual original anchor and all symbolic profile fields for this requested side.
    #[must_use]
    pub const fn requested(self) -> &'a WalkBalanceNotch {
        self.association.pair.mark(self.association.assignment.side)
    }
    /// Authentic active membership in the actual notch catalogue,not a semantic identity.
    #[must_use]
    pub const fn role(self) -> ActiveNotchOutputRole<'a> {
        self.association.role
    }
    /// Actual cached physical copy for the original requested side.
    #[must_use]
    pub const fn copy(self) -> &'a CutCopy {
        self.owner
            .roles
            .copies()
            .copy(self.association.assignment.side)
    }
    /// Actual cached pattern owner for the original requested side.
    #[must_use]
    pub const fn piece(self) -> &'a Piece {
        self.owner
            .roles
            .copies()
            .piece(self.association.assignment.side)
    }
}
