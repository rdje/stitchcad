//! Original join Edge-role/range/occurrence attribution,not complete surviving-source coverage.
use super::{DeclaredJoinPieceIdentity, JoinSourceOccurrence, JoinSourceOccurrenceLocation};
use crate::{
    construction::ActiveEdgeOutputRole,
    ontology::{EdgeRange, EdgeRef, EntityId, LocalTag, SeamSideId},
};
use core::fmt;
use std::collections::BTreeMap;

/// Original explicit role,source role/location and positive held range. Locations select source
/// fields only;the output identity always uses its explicit persistent tag.
/// ```compile_fail
/// use sc_core::{construction::*,ontology::*};
/// JoinEdgeCandidate::new(LocalTag::FIRST,SeamSideId::A,JoinSourceOccurrenceLocation::Boundary(0),
///     PointRef::new(EntityId::from_bits(40),LocalTag::FIRST));
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinEdgeCandidate {
    tag: LocalTag,
    side: SeamSideId,
    location: JoinSourceOccurrenceLocation,
    source: EdgeRange,
}
impl JoinEdgeCandidate {
    /// Retain exact authored fields;no source membership,currentness,coverage or map is inferred.
    #[must_use]
    pub const fn new(
        tag: LocalTag,
        side: SeamSideId,
        location: JoinSourceOccurrenceLocation,
        source: EdgeRange,
    ) -> Self {
        Self {
            tag,
            side,
            location,
            source,
        }
    }
    /// Explicit persistent output role,not a source location or candidate ordinal.
    #[must_use]
    pub const fn tag(&self) -> LocalTag {
        self.tag
    }
    /// Explicit original A/B role.
    #[must_use]
    pub const fn side(&self) -> SeamSideId {
        self.side
    }
    /// Explicit original source field location,not an output identity.
    #[must_use]
    pub const fn location(&self) -> JoinSourceOccurrenceLocation {
        self.location
    }
    /// Actual original positive held source-range field.
    #[must_use]
    pub const fn source(&self) -> &EdgeRange {
        &self.source
    }
}
/// Whole candidate and checked index/association entry allowances before fields/allocation.
/// ```compile_fail
/// let limits:sc_core::construction::JoinEdgeManifestLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinEdgeManifestLimits {
    candidates: usize,
    workspace_entries: usize,
}
impl JoinEdgeManifestLimits {
    /// Bound complete N and checked2N entries;earlier original storage/context policies remain.
    #[must_use]
    pub const fn new(candidates: usize, workspace_entries: usize) -> Self {
        Self {
            candidates,
            workspace_entries,
        }
    }
    /// Complete original candidate allowance.
    #[must_use]
    pub const fn candidates(self) -> usize {
        self.candidates
    }
    /// Complete index-plus-association entry allowance,not bytes.
    #[must_use]
    pub const fn workspace_entries(self) -> usize {
        self.workspace_entries
    }
}
/// First whole-resource,operation,role or literal original-source attribution refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinEdgeManifestRefusal {
    /// Checked complete2N cannot be represented.
    WorkspaceOverflow,
    /// Whole N exceeds policy.
    Candidates {
        /// Complete original count.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole index-plus-association population exceeds policy.
    Workspace {
        /// Complete required entries.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Manifest operation differs from the actual join operation.
    Operation,
    /// An explicit persistent tag repeats in original candidate order.
    DuplicateTag {
        /// First original candidate.
        first: usize,
        /// Repeated original candidate.
        second: usize,
        /// Actual repeated tag.
        tag: LocalTag,
    },
    /// Candidate has no definition in the actual complete Edge catalogue.
    UndefinedRole {
        /// Original candidate index.
        index: usize,
        /// Actual explicit tag.
        tag: LocalTag,
    },
    /// Candidate names an issued but inactive role in the actual catalogue.
    InactiveRole {
        /// Original candidate index.
        index: usize,
        /// Actual explicit tag.
        tag: LocalTag,
    },
    /// This original A/B Piece has no field at the requested family/location.
    MissingOccurrence {
        /// Original candidate index.
        index: usize,
    },
    /// Original requested range edge differs from the actual original field at that location.
    SourceEdge {
        /// Original candidate index.
        index: usize,
    },
}
/// Whole original manifest,identity/context,policy and first cause survive every refusal.
pub struct JoinEdgeManifestError<'a> {
    manifest: &'a JoinEdgeManifest,
    identity: &'a DeclaredJoinPieceIdentity<'a>,
    limits: JoinEdgeManifestLimits,
    refusal: JoinEdgeManifestRefusal,
}
impl<'a> JoinEdgeManifestError<'a> {
    /// Actual complete original proposal,including unaccepted suffixes and fields.
    #[must_use]
    pub const fn manifest(&self) -> &'a JoinEdgeManifest {
        self.manifest
    }
    /// Actual identity and complete original role/bank/source/population parents.
    #[must_use]
    pub const fn identity(&self) -> &'a DeclaredJoinPieceIdentity<'a> {
        self.identity
    }
    /// Original whole-resource policy.
    #[must_use]
    pub const fn limits(&self) -> JoinEdgeManifestLimits {
        self.limits
    }
    /// First original resource/operation/role/source cause.
    #[must_use]
    pub const fn refusal(&self) -> JoinEdgeManifestRefusal {
        self.refusal
    }
    /// Actual first refused candidate;whole-resource/operation causes return None.
    #[must_use]
    pub fn refused_candidate(&self) -> Option<&'a JoinEdgeCandidate> {
        use JoinEdgeManifestRefusal as R;
        let index = match self.refusal {
            R::DuplicateTag { second, .. } => second,
            R::UndefinedRole { index, .. }
            | R::InactiveRole { index, .. }
            | R::MissingOccurrence { index }
            | R::SourceEdge { index } => index,
            R::WorkspaceOverflow | R::Candidates { .. } | R::Workspace { .. } | R::Operation => {
                return None
            }
        };
        self.manifest.candidates.get(index)
    }
    /// Actual original source occurrence only for SourceEdge;other causes return None.
    /// Constant direct field lookup;no detached/copy-normalized replacement is returned.
    #[must_use]
    pub fn refused_occurrence(&self) -> Option<JoinSourceOccurrence<'a>> {
        let JoinEdgeManifestRefusal::SourceEdge { index } = self.refusal else {
            return None;
        };
        let candidate = self.manifest.candidates.get(index)?;
        self.identity
            .roles()
            .banks()
            .sources()
            .occurrence(candidate.side, candidate.location)
    }
}
impl fmt::Debug for JoinEdgeManifestError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JoinEdgeManifestError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for JoinEdgeManifestError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "join edge candidate manifest: {:?}", self.refusal)
    }
}
impl std::error::Error for JoinEdgeManifestError<'_> {}
/// Original readonly join Edge proposal. Empty is presence metadata,not a joined result/no-op.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JoinEdgeManifest {
    operation: EntityId,
    candidates: Vec<JoinEdgeCandidate>,
}
impl JoinEdgeManifest {
    /// Retain exact authored operation/candidate order;issue no identities.
    #[must_use]
    pub const fn new(operation: EntityId, candidates: Vec<JoinEdgeCandidate>) -> Self {
        Self {
            operation,
            candidates,
        }
    }
    /// Actual original operation declaration.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        self.operation
    }
    /// Complete unmodified candidates,including repeated sources and invalid fields.
    #[must_use]
    pub fn candidates(&self) -> &[JoinEdgeCandidate] {
        &self.candidates
    }
    /// Admit whole N/checked2N before any field inspection or index/association allocation.
    /// Then operation,each original tag duplicate/undefined/inactive,source location,source-edge
    /// equality. Actual original occurrences are lent from the immutable complete H/R parent.
    /// Work O(N log(N+D+2)),workspace2N entries;no key/source scan,registry/provider or geometry call.
    /// Repeated sources with distinct tags/locations remain distinct. Unused active roles do not
    /// retire;empty metadata proves no empty result. Selected-bank survival/outside coverage,
    /// full contour/consumer mapping,currentness,global kind/freshness and publication stay later.
    /// # Errors
    /// Whole arithmetic/quotas,operation,then first original role/location/literal-edge cause.
    pub fn admit<'a>(
        &'a self,
        identity: &'a DeclaredJoinPieceIdentity<'a>,
        limits: JoinEdgeManifestLimits,
    ) -> Result<AdmittedJoinEdges<'a>, JoinEdgeManifestError<'a>> {
        use JoinEdgeManifestRefusal as R;
        let refuse = |refusal| JoinEdgeManifestError {
            manifest: self,
            identity,
            limits,
            refusal,
        };
        let n = self.candidates.len();
        let workspace_entries = n
            .checked_mul(2)
            .ok_or_else(|| refuse(R::WorkspaceOverflow))?;
        if n > limits.candidates {
            return Err(refuse(R::Candidates {
                required: n,
                allowed: limits.candidates,
            }));
        }
        if workspace_entries > limits.workspace_entries {
            return Err(refuse(R::Workspace {
                required: workspace_entries,
                allowed: limits.workspace_entries,
            }));
        }
        let roles = identity.roles();
        if self.operation != roles.catalog().catalog().operation() {
            return Err(refuse(R::Operation));
        }
        let mut by_tag = BTreeMap::new();
        let mut edges = Vec::with_capacity(n);
        for (index, candidate) in self.candidates.iter().enumerate() {
            let tag = candidate.tag;
            if let Some(first) = by_tag.insert(tag, index) {
                return Err(refuse(R::DuplicateTag {
                    first,
                    second: index,
                    tag,
                }));
            }
            if roles.catalog().definition(tag).is_none() {
                return Err(refuse(R::UndefinedRole { index, tag }));
            }
            let role = roles
                .catalog()
                .active_role(tag)
                .ok_or_else(|| refuse(R::InactiveRole { index, tag }))?;
            let occurrence = roles
                .banks()
                .sources()
                .occurrence(candidate.side, candidate.location)
                .ok_or_else(|| refuse(R::MissingOccurrence { index }))?;
            if *occurrence.field().edge() != candidate.source.edge() {
                return Err(refuse(R::SourceEdge { index }));
            }
            edges.push(EdgeAssociation {
                index,
                candidate,
                role,
                occurrence,
            });
        }
        Ok(AdmittedJoinEdges {
            manifest: self,
            identity,
            limits,
            workspace_entries,
            by_tag,
            edges,
        })
    }
}
#[derive(Debug)]
struct EdgeAssociation<'a> {
    index: usize,
    candidate: &'a JoinEdgeCandidate,
    role: ActiveEdgeOutputRole<'a>,
    occurrence: JoinSourceOccurrence<'a>,
}
/// Whole local Edge-role/source attribution,not coverage,executed geometry or an issued Piece.
/// ```compile_fail
/// fn forge<'a>(mut edges:sc_core::construction::AdmittedJoinEdges<'a>,other:&'a sc_core::construction::JoinEdgeManifest) {edges.manifest=other;}
/// ```
/// ```compile_fail
/// fn escape(manifest:sc_core::construction::JoinEdgeManifest,identity:sc_core::construction::DeclaredJoinPieceIdentity<'_>)
///     ->sc_core::construction::AdmittedJoinEdges<'static> {
///     manifest.admit(&identity,sc_core::construction::JoinEdgeManifestLimits::new(0,0)).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct AdmittedJoinEdges<'a> {
    manifest: &'a JoinEdgeManifest,
    identity: &'a DeclaredJoinPieceIdentity<'a>,
    limits: JoinEdgeManifestLimits,
    workspace_entries: usize,
    by_tag: BTreeMap<LocalTag, usize>,
    edges: Vec<EdgeAssociation<'a>>,
}
impl<'a> AdmittedJoinEdges<'a> {
    /// Actual original complete manifest.
    #[must_use]
    pub const fn manifest(&self) -> &'a JoinEdgeManifest {
        self.manifest
    }
    /// Actual original identity and complete typed/context parents.
    #[must_use]
    pub const fn identity(&self) -> &'a DeclaredJoinPieceIdentity<'a> {
        self.identity
    }
    /// Original complete-resource policy.
    #[must_use]
    pub const fn limits(&self) -> JoinEdgeManifestLimits {
        self.limits
    }
    /// Complete admitted index-plus-association count,not allocation bytes.
    #[must_use]
    pub const fn workspace_entries(&self) -> usize {
        self.workspace_entries
    }
    /// Complete admitted candidate count,including repeated original sources.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
    /// Actual attributed output for an explicit persistent tag;no ordinal/default role selection.
    #[must_use]
    pub fn edge(&self, tag: LocalTag) -> Option<JoinOutputEdge<'_>> {
        self.by_tag
            .get(&tag)
            .and_then(|i| self.edges.get(*i))
            .map(|association| JoinOutputEdge {
                owner: self,
                association,
            })
    }
    /// Authentic candidates in original declaration order,including reverse iteration.
    pub fn edges(&self) -> impl ExactSizeIterator<Item = JoinOutputEdge<'_>> + DoubleEndedIterator {
        self.edges.iter().map(move |association| JoinOutputEdge {
            owner: self,
            association,
        })
    }
}
/// Actual whole-owner/candidate/role/original-occurrence association;logical output is not issuance.
/// ```compile_fail
/// fn forge<'a>(mut edge:sc_core::construction::JoinOutputEdge<'a>,other:&'a sc_core::construction::AdmittedJoinEdges<'a>) {edge.owner=other;}
/// ```
#[derive(Clone, Copy, Debug)]
pub struct JoinOutputEdge<'a> {
    owner: &'a AdmittedJoinEdges<'a>,
    association: &'a EdgeAssociation<'a>,
}
impl<'a> JoinOutputEdge<'a> {
    /// Actual whole admitted manifest/context owner.
    #[must_use]
    pub const fn owner(self) -> &'a AdmittedJoinEdges<'a> {
        self.owner
    }
    /// Original candidate position,not its persistent output identity.
    #[must_use]
    pub const fn index(self) -> usize {
        self.association.index
    }
    /// Actual original unmodified candidate field.
    #[must_use]
    pub const fn candidate(self) -> &'a JoinEdgeCandidate {
        self.association.candidate
    }
    /// Actual active Edge role from the original admitted catalogue.
    #[must_use]
    pub const fn role(self) -> ActiveEdgeOutputRole<'a> {
        self.association.role
    }
    /// Actual original source occurrence/context,including unchanged traversal if present.
    #[must_use]
    pub const fn occurrence(self) -> JoinSourceOccurrence<'a> {
        self.association.occurrence
    }
    /// Actual original positive held source-range field.
    #[must_use]
    pub const fn source(self) -> &'a EdgeRange {
        self.association.candidate.source()
    }
    /// Declared stable output tuple from actual operation/explicit role;not a minted entity.
    #[must_use]
    pub fn logical_output(self) -> EdgeRef {
        EdgeRef::new(
            self.owner.identity.roles().catalog().catalog().operation(),
            *self.association.role.tag(),
        )
    }
}
