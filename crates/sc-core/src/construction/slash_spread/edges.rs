//! Original source/motion and bridge provenance with admitted canonical edge roles;no geometry.
use super::{BoundSlashSpreadEdgeRoles, SlashSpreadBoundaryPolicy};
use crate::{
    construction::{ActiveEdgeOutputRole, EdgeOutputRoleCatalogLimits},
    ontology::{DirectedRange, EdgeRef, EntityId, LocalTag},
};
use core::fmt;
use std::collections::BTreeMap;

/// Declared transform of an original source range,not evaluated coordinates or region membership.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlashSpreadSourceMotion {
    /// Retain the stationary bank's original geometry.
    Stationary,
    /// Apply this operation's explicit rotation or directed separation to the moving bank.
    Moving,
}
/// Cut end in the original edge parameter frame,independent of cut traversal direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlashSpreadCutEnd {
    /// The original cut range's lower parameter bound.
    Lower,
    /// The original cut range's upper parameter bound.
    Upper,
}
/// Explicit bridge traversal between corresponding original cut-end bank occurrences.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlashSpreadBridgeDirection {
    /// From the stationary endpoint to its transformed moving-bank counterpart.
    StationaryToMoving,
    /// From the transformed moving-bank endpoint to its stationary counterpart.
    MovingToStationary,
}
/// Raw original-source or connector provenance;actual geometry/ownership/closure remains unproved.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlashSpreadEdgeOrigin {
    /// Retained positive source portion with explicit original traversal and bank motion.
    Source {
        /// Actual original source frame and bounds;not a current-edge alias.
        range: DirectedRange,
        /// Required stationary or operation-specific moving transform.
        motion: SlashSpreadSourceMotion,
    },
    /// Straight connector at one corresponding cut end,without a computed segment or issued point.
    StraightBridge {
        /// Original lower/upper cut bound;Reversed cut traversal does not swap these names.
        end: SlashSpreadCutEnd,
        /// Explicit output traversal between the two bank occurrences.
        direction: SlashSpreadBridgeDirection,
    },
}
/// One explicit persistent edge tag with original provenance,not a computed or issued output.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SlashSpreadEdgeCandidate {
    tag: LocalTag,
    origin: SlashSpreadEdgeOrigin,
}
impl SlashSpreadEdgeCandidate {
    /// Retain the authored tag and complete provenance without geometry evaluation.
    #[must_use]
    pub const fn new(tag: LocalTag, origin: SlashSpreadEdgeOrigin) -> Self {
        Self { tag, origin }
    }
    /// Explicit persistent tag,not a vector position.
    #[must_use]
    pub const fn tag(&self) -> LocalTag {
        self.tag
    }
    /// Actual original provenance field.
    #[must_use]
    pub const fn origin(&self) -> &SlashSpreadEdgeOrigin {
        &self.origin
    }
}
/// Explicit complete candidate and catalogue allowances for this admission stage.
/// ```compile_fail
/// let limits:sc_core::construction::SlashSpreadEdgeManifestLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SlashSpreadEdgeManifestLimits {
    candidates: usize,
    catalog: EdgeOutputRoleCatalogLimits,
}
impl SlashSpreadEdgeManifestLimits {
    /// Select complete candidate and definition/active/key-byte allowances,never a prefix.
    #[must_use]
    pub const fn new(candidates: usize, catalog: EdgeOutputRoleCatalogLimits) -> Self {
        Self {
            candidates,
            catalog,
        }
    }
    /// Complete candidate occurrence allowance.
    #[must_use]
    pub const fn candidates(self) -> usize {
        self.candidates
    }
    /// Complete supplied catalogue population/text allowances for this stage.
    #[must_use]
    pub const fn catalog(self) -> EdgeOutputRoleCatalogLimits {
        self.catalog
    }
}
/// Whole-resource,operation or first original candidate cause.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlashSpreadEdgeManifestRefusal {
    /// Two complete N-entry work populations cannot be counted.
    WorkspaceOverflow,
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
    /// Complete key bytes exceed policy.
    KeyBytes {
        /// Complete required byte count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Ordinary admit requires a nonempty declaration;no fallback output is fabricated.
    Empty,
    /// Explicit no-new-edge admission received candidates;none is silently discarded.
    CandidatesPresent,
    /// Manifest operation differs from the actual bound slash intent.
    Operation,
    /// Explicit tag repeated in original candidate order.
    DuplicateTag {
        /// Original first occurrence index.
        first: usize,
        /// Original repeated occurrence index.
        second: usize,
    },
    /// Original candidate has no catalogue definition.
    UndefinedRole {
        /// Original candidate index.
        candidate: usize,
        /// Original explicit tag.
        tag: LocalTag,
    },
    /// Original candidate is defined but not canonically active.
    InactiveRole {
        /// Original candidate index.
        candidate: usize,
        /// Original explicit tag.
        tag: LocalTag,
    },
    /// Original source is supplied by this same creating operation.
    SelfSource {
        /// Original candidate index.
        candidate: usize,
    },
    /// A TrimOverlaps request cannot silently acquire a straight bridge.
    BridgePolicy {
        /// Original candidate index.
        candidate: usize,
    },
}
/// Actual whole original manifest,bound role/input owners,limits and first cause survive refusal.
pub struct SlashSpreadEdgeManifestError<'a> {
    manifest: &'a SlashSpreadEdgeManifest,
    bound: &'a BoundSlashSpreadEdgeRoles<'a>,
    limits: SlashSpreadEdgeManifestLimits,
    refusal: SlashSpreadEdgeManifestRefusal,
}
impl<'a> SlashSpreadEdgeManifestError<'a> {
    /// Actual whole original manifest,including every unaccepted candidate.
    #[must_use]
    pub const fn manifest(&self) -> &'a SlashSpreadEdgeManifest {
        self.manifest
    }
    /// Actual complete slash intent/catalogue association.
    #[must_use]
    pub const fn bound_roles(&self) -> &'a BoundSlashSpreadEdgeRoles<'a> {
        self.bound
    }
    /// Original explicit admission policy.
    #[must_use]
    pub const fn limits(&self) -> SlashSpreadEdgeManifestLimits {
        self.limits
    }
    /// First complete-resource,owner or original candidate cause.
    #[must_use]
    pub const fn refusal(&self) -> SlashSpreadEdgeManifestRefusal {
        self.refusal
    }
}
impl fmt::Debug for SlashSpreadEdgeManifestError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SlashSpreadEdgeManifestError")
            .field("operation", &self.manifest.operation)
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for SlashSpreadEdgeManifestError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "slash/spread edge manifest admission: {:?}",
            self.refusal
        )
    }
}
impl std::error::Error for SlashSpreadEdgeManifestError<'_> {}

/// Owned raw edge candidates and operation identity;no contour completeness or geometric result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SlashSpreadEdgeManifest {
    operation: EntityId,
    candidates: Vec<SlashSpreadEdgeCandidate>,
}
enum CandidatePopulation {
    NewEdges,
    NoNewEdges,
}
impl SlashSpreadEdgeManifest {
    /// Own all original candidates in authored order without accepting or issuing them.
    #[must_use]
    pub const fn new(operation: EntityId, candidates: Vec<SlashSpreadEdgeCandidate>) -> Self {
        Self {
            operation,
            candidates,
        }
    }
    /// Original required operation identity,without authority or global namespace proof.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        self.operation
    }
    /// All actual original candidate fields,without sorting or deduplication.
    #[must_use]
    pub fn candidates(&self) -> &[SlashSpreadEdgeCandidate] {
        &self.candidates
    }
    /// Admit complete 2N workspace,N/D/A/key-byte quotas before empty,operation and candidate checks.
    /// Then inspect original tag duplication,defined/active role,local source or bridge policy.
    /// Peak work is one N-entry temporary tag index plus N retained actual candidate/role pairs.
    /// Earlier storage/catalogue allocation,whole memory,geometry and execution resources remain owned.
    /// # Errors
    /// Complete resource,empty,owner,then first original candidate cause. No accepted prefix escapes.
    pub fn admit<'a>(
        &'a self,
        bound: &'a BoundSlashSpreadEdgeRoles<'a>,
        limits: SlashSpreadEdgeManifestLimits,
    ) -> Result<AdmittedSlashSpreadEdges<'a>, SlashSpreadEdgeManifestError<'a>> {
        self.admit_population(bound, limits, CandidatePopulation::NewEdges)
    }
    /// Explicitly declare no new edges using the actual empty manifest and bound role owners.
    /// Complete 2N/N/D/A/key-byte quotas precede the required empty population and matching operation.
    /// Active/issued roles are neither retired nor deactivated. No candidate,index entry or identity
    /// is fabricated. This does not prove unchanged contours,effect,currentness or geometric validity.
    /// # Errors
    /// Complete resource,CandidatesPresent,then Operation. Ordinary admit still refuses empty input.
    pub fn admit_no_new_edges<'a>(
        &'a self,
        bound: &'a BoundSlashSpreadEdgeRoles<'a>,
        limits: SlashSpreadEdgeManifestLimits,
    ) -> Result<AdmittedSlashSpreadEdges<'a>, SlashSpreadEdgeManifestError<'a>> {
        self.admit_population(bound, limits, CandidatePopulation::NoNewEdges)
    }
    fn admit_population<'a>(
        &'a self,
        bound: &'a BoundSlashSpreadEdgeRoles<'a>,
        limits: SlashSpreadEdgeManifestLimits,
        population: CandidatePopulation,
    ) -> Result<AdmittedSlashSpreadEdges<'a>, SlashSpreadEdgeManifestError<'a>> {
        let refuse = |refusal| SlashSpreadEdgeManifestError {
            manifest: self,
            bound,
            limits,
            refusal,
        };
        let n = self.candidates.len();
        let workspace_entries = n
            .checked_mul(2)
            .ok_or_else(|| refuse(SlashSpreadEdgeManifestRefusal::WorkspaceOverflow))?;
        let catalog = bound.catalog();
        let d = catalog.catalog().definitions().len();
        let a = catalog.catalog().active_tags().len();
        if n > limits.candidates {
            return Err(refuse(SlashSpreadEdgeManifestRefusal::Candidates {
                required: n,
                allowed: limits.candidates,
            }));
        }
        if d > limits.catalog.definitions() {
            return Err(refuse(SlashSpreadEdgeManifestRefusal::Definitions {
                required: d,
                allowed: limits.catalog.definitions(),
            }));
        }
        if a > limits.catalog.active() {
            return Err(refuse(SlashSpreadEdgeManifestRefusal::Active {
                required: a,
                allowed: limits.catalog.active(),
            }));
        }
        if catalog.key_bytes() > limits.catalog.key_bytes() {
            return Err(refuse(SlashSpreadEdgeManifestRefusal::KeyBytes {
                required: catalog.key_bytes(),
                allowed: limits.catalog.key_bytes(),
            }));
        }
        match population {
            CandidatePopulation::NewEdges if n == 0 => {
                return Err(refuse(SlashSpreadEdgeManifestRefusal::Empty));
            }
            CandidatePopulation::NoNewEdges if n != 0 => {
                return Err(refuse(SlashSpreadEdgeManifestRefusal::CandidatesPresent));
            }
            _ => {}
        }
        if self.operation != bound.intent().input().operation() {
            return Err(refuse(SlashSpreadEdgeManifestRefusal::Operation));
        }
        let mut tags = BTreeMap::new();
        let mut pairs = Vec::with_capacity(n);
        for (candidate, edge) in self.candidates.iter().enumerate() {
            let tag = edge.tag;
            if let Some(first) = tags.insert(tag, candidate) {
                return Err(refuse(SlashSpreadEdgeManifestRefusal::DuplicateTag {
                    first,
                    second: candidate,
                }));
            }
            if catalog.definition(tag).is_none() {
                return Err(refuse(SlashSpreadEdgeManifestRefusal::UndefinedRole {
                    candidate,
                    tag,
                }));
            }
            let role = catalog.active_role(tag).ok_or_else(|| {
                refuse(SlashSpreadEdgeManifestRefusal::InactiveRole { candidate, tag })
            })?;
            match edge.origin {
                SlashSpreadEdgeOrigin::Source { range, .. }
                    if range.range.edge().creator() == self.operation =>
                {
                    return Err(refuse(SlashSpreadEdgeManifestRefusal::SelfSource {
                        candidate,
                    }))
                }
                SlashSpreadEdgeOrigin::StraightBridge { .. }
                    if *bound.intent().boundary_policy()
                        != SlashSpreadBoundaryPolicy::StraightBridges =>
                {
                    return Err(refuse(SlashSpreadEdgeManifestRefusal::BridgePolicy {
                        candidate,
                    }))
                }
                _ => {}
            }
            pairs.push((edge, role));
        }
        Ok(AdmittedSlashSpreadEdges {
            manifest: self,
            bound,
            limits,
            workspace_entries,
            pairs,
        })
    }
}
/// Admitted role/provenance metadata,not a complete boundary or accepted geometry/currentness proof.
/// ```compile_fail
/// fn forge<'a>(manifest:&'a sc_core::construction::SlashSpreadEdgeManifest,bound:&'a sc_core::construction::BoundSlashSpreadEdgeRoles<'a>,limits:sc_core::construction::SlashSpreadEdgeManifestLimits)
///     ->sc_core::construction::AdmittedSlashSpreadEdges<'a> {
///     sc_core::construction::AdmittedSlashSpreadEdges { manifest,bound,limits,workspace_entries:0,pairs:vec![] }
/// }
/// ```
/// ```compile_fail
/// fn escape(manifest:sc_core::construction::SlashSpreadEdgeManifest,bound:sc_core::construction::BoundSlashSpreadEdgeRoles<'_>)
///     ->sc_core::construction::AdmittedSlashSpreadEdges<'static> {
///     manifest.admit(&bound,sc_core::construction::SlashSpreadEdgeManifestLimits::new(0,sc_core::construction::EdgeOutputRoleCatalogLimits::new(0,0,0))).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct AdmittedSlashSpreadEdges<'a> {
    manifest: &'a SlashSpreadEdgeManifest,
    bound: &'a BoundSlashSpreadEdgeRoles<'a>,
    limits: SlashSpreadEdgeManifestLimits,
    workspace_entries: usize,
    pairs: Vec<(&'a SlashSpreadEdgeCandidate, ActiveEdgeOutputRole<'a>)>,
}
impl<'a> AdmittedSlashSpreadEdges<'a> {
    /// Actual whole original candidate manifest.
    #[must_use]
    pub const fn manifest(&self) -> &'a SlashSpreadEdgeManifest {
        self.manifest
    }
    /// Actual complete slash intent/catalogue association.
    #[must_use]
    pub const fn bound_roles(&self) -> &'a BoundSlashSpreadEdgeRoles<'a> {
        self.bound
    }
    /// Original explicit admission policy.
    #[must_use]
    pub const fn limits(&self) -> SlashSpreadEdgeManifestLimits {
        self.limits
    }
    /// Complete peak 2N entry population,not bytes or previous catalogue workspace.
    #[must_use]
    pub const fn workspace_entries(&self) -> usize {
        self.workspace_entries
    }
    /// All actual candidate/active-role pairs in original candidate order.
    pub fn outputs(
        &self,
    ) -> impl ExactSizeIterator<Item = SlashSpreadEdge<'_>> + DoubleEndedIterator {
        self.pairs
            .iter()
            .enumerate()
            .map(move |(index, (candidate, role))| SlashSpreadEdge {
                admitted: self,
                index,
                candidate,
                role: *role,
            })
    }
}
/// Authentic original candidate and active-role fields,borrowing the whole admitted manifest.
/// ```compile_fail
/// fn reparent<'a>(mut output:sc_core::construction::SlashSpreadEdge<'a>,other:&'a sc_core::construction::AdmittedSlashSpreadEdges<'a>) {
///     output.admitted=other;
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct SlashSpreadEdge<'a> {
    admitted: &'a AdmittedSlashSpreadEdges<'a>,
    index: usize,
    candidate: &'a SlashSpreadEdgeCandidate,
    role: ActiveEdgeOutputRole<'a>,
}
impl<'a> SlashSpreadEdge<'a> {
    /// Actual complete admitted owner.
    #[must_use]
    pub const fn admitted(self) -> &'a AdmittedSlashSpreadEdges<'a> {
        self.admitted
    }
    /// Actual original candidate occurrence index,not persistent identity.
    #[must_use]
    pub const fn index(self) -> usize {
        self.index
    }
    /// Actual original candidate and complete provenance field.
    #[must_use]
    pub const fn candidate(self) -> &'a SlashSpreadEdgeCandidate {
        self.candidate
    }
    /// Authentic original catalogue active-list/definition fields.
    #[must_use]
    pub const fn role(self) -> ActiveEdgeOutputRole<'a> {
        self.role
    }
    /// Logical unissued edge reference,without ledger registration or actual geometry.
    #[must_use]
    pub fn logical_output(self) -> EdgeRef {
        EdgeRef::new(self.admitted.manifest.operation, self.candidate.tag)
    }
}
