//! Proposed original-edge attribution,not complete source coverage or executed reflection.
use super::DeclaredMirrorPieceIdentity;
use crate::{
    construction::ActiveEdgeOutputRole,
    ontology::{EdgeRef, EntityId, LocalTag, MaterialAssignment},
};
use core::fmt;
use std::collections::BTreeMap;
mod coverage;
pub use coverage::{
    MirrorPieceEdgeCoverage, MirrorPieceEdgeCoverageError, MirrorPieceEdgeCoverageLimits,
    MirrorPieceEdgeCoverageRefusal, MirrorPieceMappedEdgeOccurrence,
};
/// Original unadmitted edge-role/source declaration;neither field supplies geometry evidence.
/// ```compile_fail
/// use sc_core::{construction::MirrorPieceEdgeCandidate,ontology::*};
/// MirrorPieceEdgeCandidate::new(LocalTag::new(9),
///     PointRef::new(EntityId::from_bits(40),LocalTag::FIRST));
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MirrorPieceEdgeCandidate {
    tag: LocalTag,
    source: EdgeRef,
}
impl MirrorPieceEdgeCandidate {
    /// Retain explicit persistent role tag and exact original EdgeRef,without normalizing either.
    #[must_use]
    pub const fn new(tag: LocalTag, source: EdgeRef) -> Self {
        Self { tag, source }
    }
    /// Original explicit output role,not a vector index.
    #[must_use]
    pub const fn tag(&self) -> LocalTag {
        self.tag
    }
    /// Actual original source identity/tag,without source existence/frame/membership proof.
    #[must_use]
    pub const fn source(&self) -> &EdgeRef {
        &self.source
    }
}
/// Explicit complete candidate and peak index/association entry allowances,without defaults.
/// ```compile_fail
/// let limits:sc_core::construction::MirrorPieceEdgeManifestLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MirrorPieceEdgeManifestLimits {
    candidates: usize,
    workspace_entries: usize,
}
impl MirrorPieceEdgeManifestLimits {
    /// Select whole N and checked 2N-entry workspace allowances before indexing or source scans.
    #[must_use]
    pub const fn new(candidates: usize, workspace_entries: usize) -> Self {
        Self {
            candidates,
            workspace_entries,
        }
    }
    /// Complete candidate population allowance.
    #[must_use]
    pub const fn candidates(self) -> usize {
        self.candidates
    }
    /// Complete index/association entry allowance,not allocation bytes.
    #[must_use]
    pub const fn workspace_entries(self) -> usize {
        self.workspace_entries
    }
}
/// First whole-resource,owner,original role or definite typed-source failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MirrorPieceEdgeManifestRefusal {
    /// The complete 2N workspace population cannot be represented.
    WorkspaceOverflow,
    /// Complete candidate count exceeds the explicit allowance.
    Candidates {
        /// Actual whole population.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete index/association population exceeds policy.
    Workspace {
        /// Actual required entries.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Raw manifest operation differs from the actual bound mirror input.
    Operation,
    /// An explicit output role repeats in original candidate order.
    DuplicateTag {
        /// First original candidate.
        first: usize,
        /// Repeated candidate.
        second: usize,
        /// Actual repeated persistent tag.
        tag: LocalTag,
    },
    /// Candidate names no supplied edge-role definition.
    UndefinedRole {
        /// Original candidate.
        index: usize,
        /// Actual original tag.
        tag: LocalTag,
    },
    /// Candidate names an issued but canonically inactive edge role.
    InactiveRole {
        /// Original candidate.
        index: usize,
        /// Actual original tag.
        tag: LocalTag,
    },
    /// Required original edge would be supplied by this same mirror operation.
    SelfSource {
        /// Original candidate.
        index: usize,
        /// Actual original source field.
        source: EdgeRef,
    },
    /// Required edge creator aliases an actual source/result Piece owner.
    PieceSource {
        /// Original candidate.
        index: usize,
        /// Actual original source field.
        source: EdgeRef,
        /// Actual source/result Piece identity.
        piece: EntityId,
    },
    /// Required edge creator aliases an explicitly assigned source/result Material.
    MaterialSource {
        /// Original candidate.
        index: usize,
        /// Actual original source field.
        source: EdgeRef,
        /// Actual assigned Material identity.
        material: EntityId,
    },
}
/// Actual complete manifest,typed parents and explicit policy survive every refusal.
pub struct MirrorPieceEdgeManifestError<'a> {
    manifest: &'a MirrorPieceEdgeManifest,
    identity: &'a DeclaredMirrorPieceIdentity<'a>,
    limits: MirrorPieceEdgeManifestLimits,
    refusal: MirrorPieceEdgeManifestRefusal,
}
impl<'a> MirrorPieceEdgeManifestError<'a> {
    /// Actual complete original unadmitted declaration,including every unaccepted candidate.
    #[must_use]
    pub const fn manifest(&self) -> &'a MirrorPieceEdgeManifest {
        self.manifest
    }
    /// Actual original typed role/identity/source/metadata parents.
    #[must_use]
    pub const fn identity(&self) -> &'a DeclaredMirrorPieceIdentity<'a> {
        self.identity
    }
    /// Original explicit whole-resource policy.
    #[must_use]
    pub const fn limits(&self) -> MirrorPieceEdgeManifestLimits {
        self.limits
    }
    /// First original resource/owner/role/source cause.
    #[must_use]
    pub const fn refusal(&self) -> MirrorPieceEdgeManifestRefusal {
        self.refusal
    }
}
impl fmt::Debug for MirrorPieceEdgeManifestError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MirrorPieceEdgeManifestError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for MirrorPieceEdgeManifestError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mirror edge candidate manifest: {:?}", self.refusal)
    }
}
impl std::error::Error for MirrorPieceEdgeManifestError<'_> {}

/// Original readonly candidate proposal. Empty is explicit presence metadata,not a complete copy/no-op.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MirrorPieceEdgeManifest {
    operation: EntityId,
    candidates: Vec<MirrorPieceEdgeCandidate>,
}
impl MirrorPieceEdgeManifest {
    /// Retain exact original operation and candidate order without issuing identities.
    #[must_use]
    pub const fn new(operation: EntityId, candidates: Vec<MirrorPieceEdgeCandidate>) -> Self {
        Self {
            operation,
            candidates,
        }
    }
    /// Original declared operation,which admission compares to its actual parent.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        self.operation
    }
    /// Complete original candidates,including repeated sources and invalid unadmitted fields.
    #[must_use]
    pub fn candidates(&self) -> &[MirrorPieceEdgeCandidate] {
        &self.candidates
    }
    /// Admit whole N/2N before owner/role/source inspection and index allocation.
    /// Repeated sources with distinct roles remain distinct;active roles absent here are not retired.
    /// This proves local attribution only,not actual source existence,Piece membership,common frame,
    /// reflection,complete source/consumer coverage,issued history or logical-edge publication.
    /// # Errors
    /// Workspace overflow,N quota,workspace quota,operation;then each original candidate's duplicate
    /// tag,undefined/inactive role,self-source,known Piece and Material source aliases,in that order.
    pub fn admit<'a>(
        &'a self,
        identity: &'a DeclaredMirrorPieceIdentity<'a>,
        limits: MirrorPieceEdgeManifestLimits,
    ) -> Result<AdmittedMirrorPieceEdges<'a>, MirrorPieceEdgeManifestError<'a>> {
        let refuse = |refusal| MirrorPieceEdgeManifestError {
            manifest: self,
            identity,
            limits,
            refusal,
        };
        let n = self.candidates.len();
        let workspace_entries = n
            .checked_mul(2)
            .ok_or_else(|| refuse(MirrorPieceEdgeManifestRefusal::WorkspaceOverflow))?;
        if n > limits.candidates {
            return Err(refuse(MirrorPieceEdgeManifestRefusal::Candidates {
                required: n,
                allowed: limits.candidates,
            }));
        }
        if workspace_entries > limits.workspace_entries {
            return Err(refuse(MirrorPieceEdgeManifestRefusal::Workspace {
                required: workspace_entries,
                allowed: limits.workspace_entries,
            }));
        }
        let metadata = identity.roles().metadata();
        let original = metadata.cut().source();
        let operation = original.input().operation();
        if self.operation != operation {
            return Err(refuse(MirrorPieceEdgeManifestRefusal::Operation));
        }
        let mut by_tag = BTreeMap::new();
        let mut edges = Vec::with_capacity(n);
        for (index, candidate) in self.candidates.iter().enumerate() {
            let tag = candidate.tag;
            if let Some(first) = by_tag.insert(tag, index) {
                return Err(refuse(MirrorPieceEdgeManifestRefusal::DuplicateTag {
                    first,
                    second: index,
                    tag,
                }));
            }
            if identity.roles().catalog().definition(tag).is_none() {
                return Err(refuse(MirrorPieceEdgeManifestRefusal::UndefinedRole {
                    index,
                    tag,
                }));
            }
            let role = identity.roles().catalog().active_role(tag).ok_or_else(|| {
                refuse(MirrorPieceEdgeManifestRefusal::InactiveRole { index, tag })
            })?;
            let source = candidate.source;
            if source.creator() == operation {
                return Err(refuse(MirrorPieceEdgeManifestRefusal::SelfSource {
                    index,
                    source,
                }));
            }
            for piece in [original.piece().id(), *identity.piece_id()] {
                if source.creator() == piece {
                    return Err(refuse(MirrorPieceEdgeManifestRefusal::PieceSource {
                        index,
                        source,
                        piece,
                    }));
                }
            }
            for material in [&original.piece().definition().material, metadata.material()] {
                if let MaterialAssignment::Assigned(material) = material {
                    if source.creator() == *material {
                        return Err(refuse(MirrorPieceEdgeManifestRefusal::MaterialSource {
                            index,
                            source,
                            material: *material,
                        }));
                    }
                }
            }
            edges.push(EdgeAssociation {
                index,
                candidate,
                role,
            });
        }
        Ok(AdmittedMirrorPieceEdges {
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
    candidate: &'a MirrorPieceEdgeCandidate,
    role: ActiveEdgeOutputRole<'a>,
}

/// Whole local edge-candidate admission,not executed coordinates or a complete reflected Piece.
/// ```compile_fail
/// fn forge<'a>(mut edges:sc_core::construction::AdmittedMirrorPieceEdges<'a>,
///     other:&'a sc_core::construction::MirrorPieceEdgeManifest) { edges.manifest=other; }
/// ```
/// ```compile_fail
/// fn escape(manifest:sc_core::construction::MirrorPieceEdgeManifest,
///     identity:sc_core::construction::DeclaredMirrorPieceIdentity<'_>)
///     ->sc_core::construction::AdmittedMirrorPieceEdges<'static> {
///     manifest.admit(&identity,sc_core::construction::MirrorPieceEdgeManifestLimits::new(0,0)).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct AdmittedMirrorPieceEdges<'a> {
    manifest: &'a MirrorPieceEdgeManifest,
    identity: &'a DeclaredMirrorPieceIdentity<'a>,
    limits: MirrorPieceEdgeManifestLimits,
    workspace_entries: usize,
    by_tag: BTreeMap<LocalTag, usize>,
    edges: Vec<EdgeAssociation<'a>>,
}
impl<'a> AdmittedMirrorPieceEdges<'a> {
    /// Actual complete original candidate declaration.
    #[must_use]
    pub const fn manifest(&self) -> &'a MirrorPieceEdgeManifest {
        self.manifest
    }
    /// Actual original typed edge roles and all prior identity/source/metadata parents.
    #[must_use]
    pub const fn identity(&self) -> &'a DeclaredMirrorPieceIdentity<'a> {
        self.identity
    }
    /// Original explicit whole-resource limits.
    #[must_use]
    pub const fn limits(&self) -> MirrorPieceEdgeManifestLimits {
        self.limits
    }
    /// Complete admitted index/association entry count,not allocation bytes.
    #[must_use]
    pub const fn workspace_entries(&self) -> usize {
        self.workspace_entries
    }
    /// Complete admitted candidate count,including repeated original sources.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
    /// Actual admitted edge for an explicit persistent tag,without prefix/default selection.
    #[must_use]
    pub fn edge(&self, tag: LocalTag) -> Option<MirrorPieceOutputEdge<'_>> {
        self.by_tag
            .get(&tag)
            .and_then(|index| self.edges.get(*index))
            .map(|association| MirrorPieceOutputEdge {
                owner: self,
                association,
            })
    }
    /// Authentic candidates in original declaration order,including reverse iteration.
    pub fn edges(
        &self,
    ) -> impl ExactSizeIterator<Item = MirrorPieceOutputEdge<'_>> + DoubleEndedIterator {
        self.edges
            .iter()
            .map(move |association| MirrorPieceOutputEdge {
                owner: self,
                association,
            })
    }
}
/// Actual original candidate and edge-role attribution;logical output is a declaration,not issuance.
#[derive(Clone, Copy, Debug)]
pub struct MirrorPieceOutputEdge<'a> {
    owner: &'a AdmittedMirrorPieceEdges<'a>,
    association: &'a EdgeAssociation<'a>,
}
impl<'a> MirrorPieceOutputEdge<'a> {
    /// Actual whole admitted owner.
    #[must_use]
    pub const fn owner(self) -> &'a AdmittedMirrorPieceEdges<'a> {
        self.owner
    }
    /// Actual original candidate position,not a persistent identity.
    #[must_use]
    pub const fn index(self) -> usize {
        self.association.index
    }
    /// Actual original unmodified candidate field.
    #[must_use]
    pub const fn candidate(self) -> &'a MirrorPieceEdgeCandidate {
        self.association.candidate
    }
    /// Actual admitted edge role from its original edge catalogue.
    #[must_use]
    pub const fn role(self) -> ActiveEdgeOutputRole<'a> {
        self.association.role
    }
    /// Actual original EdgeRef source field,without currentness/geometry/membership proof.
    #[must_use]
    pub const fn source(self) -> &'a EdgeRef {
        self.association.candidate.source()
    }
    /// Declared persistent output tuple from actual operation and canonical tag;not a minted entity.
    #[must_use]
    pub fn logical_output(self) -> EdgeRef {
        EdgeRef::new(
            self.owner.identity.roles().catalog().catalog().operation(),
            *self.association.role.tag(),
        )
    }
}
