//! Original role/source-range candidate attribution,not source coverage or an executed refit.
use super::BoundTruingEdgeRoles;
use crate::{
    construction::ActiveEdgeOutputRole,
    ontology::{EdgeRange, EdgeRef, EntityId, LocalTag, MaterialAssignment},
};
use core::fmt;
use std::collections::BTreeMap;
/// Original unadmitted edge-role/source-range declaration;neither field supplies geometry evidence.
/// ```compile_fail
/// use sc_core::{construction::TruingEdgeCandidate,ontology::*};
/// TruingEdgeCandidate::new(LocalTag::new(9),
///     PointRef::new(EntityId::from_bits(40),LocalTag::FIRST));
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TruingEdgeCandidate {
    tag: LocalTag,
    source: EdgeRange,
}
impl TruingEdgeCandidate {
    /// Retain explicit persistent role tag and exact original EdgeRange,without normalizing either.
    #[must_use]
    pub const fn new(tag: LocalTag, source: EdgeRange) -> Self {
        Self { tag, source }
    }
    /// Original explicit output role,not a vector index.
    #[must_use]
    pub const fn tag(&self) -> LocalTag {
        self.tag
    }
    /// Actual original held source range,without source existence/frame/membership proof.
    #[must_use]
    pub const fn source(&self) -> &EdgeRange {
        &self.source
    }
}
/// Explicit complete candidate and peak index/association entry allowances,without defaults.
/// ```compile_fail
/// let limits:sc_core::construction::TruingEdgeManifestLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TruingEdgeManifestLimits {
    candidates: usize,
    workspace_entries: usize,
}
impl TruingEdgeManifestLimits {
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
pub enum TruingEdgeManifestRefusal {
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
    /// Raw manifest operation differs from the actual bound truing input.
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
    /// Required source edge would be supplied by this same truing operation.
    SelfSource {
        /// Original candidate.
        index: usize,
        /// Actual offending source edge;the full original range remains in the manifest.
        source: EdgeRef,
    },
    /// Required edge creator aliases an actual selected/counterpart Piece owner.
    PieceSource {
        /// Original candidate.
        index: usize,
        /// Actual offending source edge;the full original range remains in the manifest.
        source: EdgeRef,
        /// Actual selected/counterpart Piece identity.
        piece: EntityId,
    },
    /// Required edge creator aliases an explicitly assigned selected/counterpart Material.
    MaterialSource {
        /// Original candidate.
        index: usize,
        /// Actual offending source edge;the full original range remains in the manifest.
        source: EdgeRef,
        /// Actual assigned Material identity.
        material: EntityId,
    },
}
/// Actual complete manifest,typed parents and explicit policy survive every refusal.
pub struct TruingEdgeManifestError<'a> {
    manifest: &'a TruingEdgeManifest,
    roles: &'a BoundTruingEdgeRoles<'a>,
    limits: TruingEdgeManifestLimits,
    refusal: TruingEdgeManifestRefusal,
}
impl<'a> TruingEdgeManifestError<'a> {
    /// Actual complete original unadmitted declaration,including every unaccepted candidate.
    #[must_use]
    pub const fn manifest(&self) -> &'a TruingEdgeManifest {
        self.manifest
    }
    /// Actual typed role and complete copy/Piece/source/provider parents.
    #[must_use]
    pub const fn roles(&self) -> &'a BoundTruingEdgeRoles<'a> {
        self.roles
    }
    /// Original explicit whole-resource policy.
    #[must_use]
    pub const fn limits(&self) -> TruingEdgeManifestLimits {
        self.limits
    }
    /// First original resource/owner/role/source cause.
    #[must_use]
    pub const fn refusal(&self) -> TruingEdgeManifestRefusal {
        self.refusal
    }
}
impl fmt::Debug for TruingEdgeManifestError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TruingEdgeManifestError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for TruingEdgeManifestError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "truing edge candidate manifest: {:?}", self.refusal)
    }
}
impl std::error::Error for TruingEdgeManifestError<'_> {}

/// Original readonly candidate proposal. Empty is explicit presence metadata,not a complete refit/no-op.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TruingEdgeManifest {
    operation: EntityId,
    candidates: Vec<TruingEdgeCandidate>,
}
impl TruingEdgeManifest {
    /// Retain exact original operation and candidate order without issuing topology identities.
    #[must_use]
    pub const fn new(operation: EntityId, candidates: Vec<TruingEdgeCandidate>) -> Self {
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
    pub fn candidates(&self) -> &[TruingEdgeCandidate] {
        &self.candidates
    }
    /// Admit whole N/2N before owner/role/source inspection and index allocation.
    /// Repeated sources with distinct roles remain distinct;active roles absent here are not retired.
    /// This proves local attribution only,not actual source existence,selected domain,Piece membership,common frame,
    /// refitting,complete source/consumer coverage,issued history or logical-edge publication.
    /// # Errors
    /// Workspace overflow,N quota,workspace quota,operation;then each original candidate's duplicate
    /// tag,undefined/inactive role,self-source,actual selected/counterpart Piece and Material aliases.
    /// Work is O(N log(N+D+2)),for D admitted roles;no key,geometry,registry or whole inventory scan.
    pub fn admit<'a>(
        &'a self,
        roles: &'a BoundTruingEdgeRoles<'a>,
        limits: TruingEdgeManifestLimits,
    ) -> Result<AdmittedTruingEdges<'a>, TruingEdgeManifestError<'a>> {
        let refuse = |refusal| TruingEdgeManifestError {
            manifest: self,
            roles,
            limits,
            refusal,
        };
        let n = self.candidates.len();
        let workspace_entries = n
            .checked_mul(2)
            .ok_or_else(|| refuse(TruingEdgeManifestRefusal::WorkspaceOverflow))?;
        if n > limits.candidates {
            return Err(refuse(TruingEdgeManifestRefusal::Candidates {
                required: n,
                allowed: limits.candidates,
            }));
        }
        if workspace_entries > limits.workspace_entries {
            return Err(refuse(TruingEdgeManifestRefusal::Workspace {
                required: workspace_entries,
                allowed: limits.workspace_entries,
            }));
        }
        let copies = roles.copies();
        let original = copies.source();
        let operation = original.edit().operation();
        if self.operation != operation {
            return Err(refuse(TruingEdgeManifestRefusal::Operation));
        }
        let mut by_tag = BTreeMap::new();
        let mut edges = Vec::with_capacity(n);
        for (index, candidate) in self.candidates.iter().enumerate() {
            let tag = candidate.tag;
            if let Some(first) = by_tag.insert(tag, index) {
                return Err(refuse(TruingEdgeManifestRefusal::DuplicateTag {
                    first,
                    second: index,
                    tag,
                }));
            }
            if roles.catalog().definition(tag).is_none() {
                return Err(refuse(TruingEdgeManifestRefusal::UndefinedRole {
                    index,
                    tag,
                }));
            }
            let role = roles
                .catalog()
                .active_role(tag)
                .ok_or_else(|| refuse(TruingEdgeManifestRefusal::InactiveRole { index, tag }))?;
            let source = candidate.source.edge();
            if source.creator() == operation {
                return Err(refuse(TruingEdgeManifestRefusal::SelfSource {
                    index,
                    source,
                }));
            }
            for piece in [
                copies.selected_piece().id(),
                copies.counterpart_piece().id(),
            ] {
                if source.creator() == piece {
                    return Err(refuse(TruingEdgeManifestRefusal::PieceSource {
                        index,
                        source,
                        piece,
                    }));
                }
            }
            for material in [
                &copies.selected_piece().definition().material,
                &copies.counterpart_piece().definition().material,
            ] {
                if let MaterialAssignment::Assigned(material) = material {
                    if source.creator() == *material {
                        return Err(refuse(TruingEdgeManifestRefusal::MaterialSource {
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
        Ok(AdmittedTruingEdges {
            manifest: self,
            roles,
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
    candidate: &'a TruingEdgeCandidate,
    role: ActiveEdgeOutputRole<'a>,
}

/// Whole local edge-candidate admission,not executed coordinates or a complete refitted Piece.
/// ```compile_fail
/// fn forge<'a>(mut edges:sc_core::construction::AdmittedTruingEdges<'a>,
///     other:&'a sc_core::construction::TruingEdgeManifest) { edges.manifest=other; }
/// ```
/// ```compile_fail
/// fn escape(manifest:sc_core::construction::TruingEdgeManifest,
///     roles:sc_core::construction::BoundTruingEdgeRoles<'_>)
///     ->sc_core::construction::AdmittedTruingEdges<'static> {
///     manifest.admit(&roles,sc_core::construction::TruingEdgeManifestLimits::new(0,0)).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct AdmittedTruingEdges<'a> {
    manifest: &'a TruingEdgeManifest,
    roles: &'a BoundTruingEdgeRoles<'a>,
    limits: TruingEdgeManifestLimits,
    workspace_entries: usize,
    by_tag: BTreeMap<LocalTag, usize>,
    edges: Vec<EdgeAssociation<'a>>,
}
impl<'a> AdmittedTruingEdges<'a> {
    /// Actual complete original candidate declaration.
    #[must_use]
    pub const fn manifest(&self) -> &'a TruingEdgeManifest {
        self.manifest
    }
    /// Actual original typed edge roles and actual copy/Piece/source/provider parents.
    #[must_use]
    pub const fn roles(&self) -> &'a BoundTruingEdgeRoles<'a> {
        self.roles
    }
    /// Original explicit whole-resource limits.
    #[must_use]
    pub const fn limits(&self) -> TruingEdgeManifestLimits {
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
    pub fn edge(&self, tag: LocalTag) -> Option<TruingOutputEdge<'_>> {
        self.by_tag
            .get(&tag)
            .and_then(|index| self.edges.get(*index))
            .map(|association| TruingOutputEdge {
                owner: self,
                association,
            })
    }
    /// Authentic candidates in original declaration order,including reverse iteration.
    pub fn edges(
        &self,
    ) -> impl ExactSizeIterator<Item = TruingOutputEdge<'_>> + DoubleEndedIterator {
        self.edges.iter().map(move |association| TruingOutputEdge {
            owner: self,
            association,
        })
    }
}
/// Actual original candidate and edge-role attribution;logical output is a declaration,not issuance.
/// ```compile_fail
/// fn reparent<'a>(mut edge:sc_core::construction::TruingOutputEdge<'a>,
///     other:&'a sc_core::construction::AdmittedTruingEdges<'a>) {edge.owner=other;}
/// ```
#[derive(Clone, Copy, Debug)]
pub struct TruingOutputEdge<'a> {
    owner: &'a AdmittedTruingEdges<'a>,
    association: &'a EdgeAssociation<'a>,
}
impl<'a> TruingOutputEdge<'a> {
    /// Actual whole admitted owner.
    #[must_use]
    pub const fn owner(self) -> &'a AdmittedTruingEdges<'a> {
        self.owner
    }
    /// Actual original candidate position,not a persistent identity.
    #[must_use]
    pub const fn index(self) -> usize {
        self.association.index
    }
    /// Actual original unmodified candidate field.
    #[must_use]
    pub const fn candidate(self) -> &'a TruingEdgeCandidate {
        self.association.candidate
    }
    /// Actual admitted edge role from its original edge catalogue.
    #[must_use]
    pub const fn role(self) -> ActiveEdgeOutputRole<'a> {
        self.association.role
    }
    /// Actual original EdgeRange source field,without currentness/domain/geometry/membership proof.
    #[must_use]
    pub const fn source(self) -> &'a EdgeRange {
        self.association.candidate.source()
    }
    /// Declared persistent output tuple from actual operation and canonical tag;not a minted entity.
    #[must_use]
    pub fn logical_output(self) -> EdgeRef {
        EdgeRef::new(
            self.owner.roles.catalog().catalog().operation(),
            *self.association.role.tag(),
        )
    }
}
