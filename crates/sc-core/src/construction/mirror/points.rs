//! Proposed original-point attribution,not source membership,coordinates or executed reflection.
use super::BoundMirrorPiecePointRoles;
use crate::{
    construction::ActivePointOutputRole,
    ontology::{EntityId, LocalTag, MaterialAssignment, PointRef},
};
use core::fmt;
use std::collections::BTreeMap;

/// Original unadmitted point-role/source declaration;neither field supplies geometry evidence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MirrorPiecePointCandidate {
    tag: LocalTag,
    source: PointRef,
}
impl MirrorPiecePointCandidate {
    /// Retain explicit persistent role tag and exact original PointRef,without normalizing either.
    #[must_use]
    pub const fn new(tag: LocalTag, source: PointRef) -> Self {
        Self { tag, source }
    }
    /// Original explicit output role,not a vector index.
    #[must_use]
    pub const fn tag(&self) -> LocalTag {
        self.tag
    }
    /// Actual original source identity/tag,without source existence/frame/membership proof.
    #[must_use]
    pub const fn source(&self) -> &PointRef {
        &self.source
    }
}
/// Explicit complete candidate and peak index/association entry allowances,without defaults.
/// ```compile_fail
/// let limits:sc_core::construction::MirrorPiecePointManifestLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MirrorPiecePointManifestLimits {
    candidates: usize,
    workspace_entries: usize,
}
impl MirrorPiecePointManifestLimits {
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
pub enum MirrorPiecePointManifestRefusal {
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
    /// Candidate names no supplied point-role definition.
    UndefinedRole {
        /// Original candidate.
        index: usize,
        /// Actual original tag.
        tag: LocalTag,
    },
    /// Candidate names an issued but canonically inactive point role.
    InactiveRole {
        /// Original candidate.
        index: usize,
        /// Actual original tag.
        tag: LocalTag,
    },
    /// Required original point would be supplied by this same mirror operation.
    SelfSource {
        /// Original candidate.
        index: usize,
        /// Actual original source field.
        source: PointRef,
    },
    /// Required point creator aliases an actual source/result Piece owner.
    PieceSource {
        /// Original candidate.
        index: usize,
        /// Actual original source field.
        source: PointRef,
        /// Actual source/result Piece identity.
        piece: EntityId,
    },
    /// Required point creator aliases an explicitly assigned source/result Material.
    MaterialSource {
        /// Original candidate.
        index: usize,
        /// Actual original source field.
        source: PointRef,
        /// Actual assigned Material identity.
        material: EntityId,
    },
}
/// Actual complete manifest,typed parents and explicit policy survive every refusal.
pub struct MirrorPiecePointManifestError<'a> {
    manifest: &'a MirrorPiecePointManifest,
    roles: &'a BoundMirrorPiecePointRoles<'a>,
    limits: MirrorPiecePointManifestLimits,
    refusal: MirrorPiecePointManifestRefusal,
}
impl<'a> MirrorPiecePointManifestError<'a> {
    /// Actual complete original unadmitted declaration,including every unaccepted candidate.
    #[must_use]
    pub const fn manifest(&self) -> &'a MirrorPiecePointManifest {
        self.manifest
    }
    /// Actual original typed role/identity/source/metadata parents.
    #[must_use]
    pub const fn roles(&self) -> &'a BoundMirrorPiecePointRoles<'a> {
        self.roles
    }
    /// Original explicit whole-resource policy.
    #[must_use]
    pub const fn limits(&self) -> MirrorPiecePointManifestLimits {
        self.limits
    }
    /// First original resource/owner/role/source cause.
    #[must_use]
    pub const fn refusal(&self) -> MirrorPiecePointManifestRefusal {
        self.refusal
    }
}
impl fmt::Debug for MirrorPiecePointManifestError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MirrorPiecePointManifestError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for MirrorPiecePointManifestError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mirror point candidate manifest: {:?}", self.refusal)
    }
}
impl std::error::Error for MirrorPiecePointManifestError<'_> {}

/// Original readonly candidate proposal. Empty is explicit presence metadata,not a complete copy/no-op.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MirrorPiecePointManifest {
    operation: EntityId,
    candidates: Vec<MirrorPiecePointCandidate>,
}
impl MirrorPiecePointManifest {
    /// Retain exact original operation and candidate order without issuing identities.
    #[must_use]
    pub const fn new(operation: EntityId, candidates: Vec<MirrorPiecePointCandidate>) -> Self {
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
    pub fn candidates(&self) -> &[MirrorPiecePointCandidate] {
        &self.candidates
    }
    /// Admit whole N/2N before owner/role/source inspection and index allocation.
    /// Repeated sources with distinct roles remain distinct;active roles absent here are not retired.
    /// This proves local attribution only,not actual source existence,Piece membership,common frame,
    /// reflection,whole source/consumer coverage,issued history or logical-point publication.
    /// # Errors
    /// Workspace overflow,N quota,workspace quota,operation;then each original candidate's duplicate
    /// tag,undefined/inactive role,self-source,known Piece and Material source aliases,in that order.
    pub fn admit<'a>(
        &'a self,
        roles: &'a BoundMirrorPiecePointRoles<'a>,
        limits: MirrorPiecePointManifestLimits,
    ) -> Result<AdmittedMirrorPiecePoints<'a>, MirrorPiecePointManifestError<'a>> {
        let refuse = |refusal| MirrorPiecePointManifestError {
            manifest: self,
            roles,
            limits,
            refusal,
        };
        let n = self.candidates.len();
        let workspace_entries = n
            .checked_mul(2)
            .ok_or_else(|| refuse(MirrorPiecePointManifestRefusal::WorkspaceOverflow))?;
        if n > limits.candidates {
            return Err(refuse(MirrorPiecePointManifestRefusal::Candidates {
                required: n,
                allowed: limits.candidates,
            }));
        }
        if workspace_entries > limits.workspace_entries {
            return Err(refuse(MirrorPiecePointManifestRefusal::Workspace {
                required: workspace_entries,
                allowed: limits.workspace_entries,
            }));
        }
        let identity = roles.identity();
        let metadata = identity.roles().metadata();
        let original = metadata.cut().source();
        let operation = original.input().operation();
        if self.operation != operation {
            return Err(refuse(MirrorPiecePointManifestRefusal::Operation));
        }
        let mut by_tag = BTreeMap::new();
        let mut points = Vec::with_capacity(n);
        for (index, candidate) in self.candidates.iter().enumerate() {
            let tag = candidate.tag;
            if let Some(first) = by_tag.insert(tag, index) {
                return Err(refuse(MirrorPiecePointManifestRefusal::DuplicateTag {
                    first,
                    second: index,
                    tag,
                }));
            }
            if roles.catalog().definition(tag).is_none() {
                return Err(refuse(MirrorPiecePointManifestRefusal::UndefinedRole {
                    index,
                    tag,
                }));
            }
            let role = roles.catalog().active_role(tag).ok_or_else(|| {
                refuse(MirrorPiecePointManifestRefusal::InactiveRole { index, tag })
            })?;
            let source = candidate.source;
            if source.creator() == operation {
                return Err(refuse(MirrorPiecePointManifestRefusal::SelfSource {
                    index,
                    source,
                }));
            }
            for piece in [original.piece().id(), *identity.piece_id()] {
                if source.creator() == piece {
                    return Err(refuse(MirrorPiecePointManifestRefusal::PieceSource {
                        index,
                        source,
                        piece,
                    }));
                }
            }
            for material in [&original.piece().definition().material, metadata.material()] {
                if let MaterialAssignment::Assigned(material) = material {
                    if source.creator() == *material {
                        return Err(refuse(MirrorPiecePointManifestRefusal::MaterialSource {
                            index,
                            source,
                            material: *material,
                        }));
                    }
                }
            }
            points.push(PointAssociation {
                index,
                candidate,
                role,
            });
        }
        Ok(AdmittedMirrorPiecePoints {
            manifest: self,
            roles,
            limits,
            workspace_entries,
            by_tag,
            points,
        })
    }
}
#[derive(Debug)]
struct PointAssociation<'a> {
    index: usize,
    candidate: &'a MirrorPiecePointCandidate,
    role: ActivePointOutputRole<'a>,
}

/// Whole local point-candidate admission,not executed coordinates or a complete reflected Piece.
/// ```compile_fail
/// fn forge<'a>(mut points:sc_core::construction::AdmittedMirrorPiecePoints<'a>,
///     other:&'a sc_core::construction::MirrorPiecePointManifest) { points.manifest=other; }
/// ```
/// ```compile_fail
/// fn escape(manifest:sc_core::construction::MirrorPiecePointManifest,
///     roles:sc_core::construction::BoundMirrorPiecePointRoles<'_>)
///     ->sc_core::construction::AdmittedMirrorPiecePoints<'static> {
///     manifest.admit(&roles,sc_core::construction::MirrorPiecePointManifestLimits::new(0,0)).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct AdmittedMirrorPiecePoints<'a> {
    manifest: &'a MirrorPiecePointManifest,
    roles: &'a BoundMirrorPiecePointRoles<'a>,
    limits: MirrorPiecePointManifestLimits,
    workspace_entries: usize,
    by_tag: BTreeMap<LocalTag, usize>,
    points: Vec<PointAssociation<'a>>,
}
impl<'a> AdmittedMirrorPiecePoints<'a> {
    /// Actual complete original candidate declaration.
    #[must_use]
    pub const fn manifest(&self) -> &'a MirrorPiecePointManifest {
        self.manifest
    }
    /// Actual original typed point roles and all prior identity/source/metadata parents.
    #[must_use]
    pub const fn roles(&self) -> &'a BoundMirrorPiecePointRoles<'a> {
        self.roles
    }
    /// Original explicit whole-resource limits.
    #[must_use]
    pub const fn limits(&self) -> MirrorPiecePointManifestLimits {
        self.limits
    }
    /// Complete admitted index/association entry count,not allocation bytes.
    #[must_use]
    pub const fn workspace_entries(&self) -> usize {
        self.workspace_entries
    }
    /// Complete admitted candidate count,including repeated original sources.
    #[must_use]
    pub fn point_count(&self) -> usize {
        self.points.len()
    }
    /// Actual admitted point for an explicit persistent tag,without prefix/default selection.
    #[must_use]
    pub fn point(&self, tag: LocalTag) -> Option<MirrorPiecePoint<'_>> {
        self.by_tag
            .get(&tag)
            .and_then(|index| self.points.get(*index))
            .map(|association| MirrorPiecePoint {
                owner: self,
                association,
            })
    }
    /// Authentic candidates in original declaration order,including reverse iteration.
    pub fn points(
        &self,
    ) -> impl ExactSizeIterator<Item = MirrorPiecePoint<'_>> + DoubleEndedIterator {
        self.points.iter().map(move |association| MirrorPiecePoint {
            owner: self,
            association,
        })
    }
}
/// Actual original candidate and point-role attribution;logical output is a declaration,not issuance.
#[derive(Clone, Copy, Debug)]
pub struct MirrorPiecePoint<'a> {
    owner: &'a AdmittedMirrorPiecePoints<'a>,
    association: &'a PointAssociation<'a>,
}
impl<'a> MirrorPiecePoint<'a> {
    /// Actual whole admitted owner.
    #[must_use]
    pub const fn owner(self) -> &'a AdmittedMirrorPiecePoints<'a> {
        self.owner
    }
    /// Actual original candidate position,not a persistent identity.
    #[must_use]
    pub const fn index(self) -> usize {
        self.association.index
    }
    /// Actual original unmodified candidate field.
    #[must_use]
    pub const fn candidate(self) -> &'a MirrorPiecePointCandidate {
        self.association.candidate
    }
    /// Actual admitted point role from its original point catalogue.
    #[must_use]
    pub const fn role(self) -> ActivePointOutputRole<'a> {
        self.association.role
    }
    /// Actual original PointRef source field,without currentness/geometry/membership proof.
    #[must_use]
    pub const fn source(self) -> &'a PointRef {
        self.association.candidate.source()
    }
    /// Declared persistent output tuple from actual operation and canonical tag;not a minted entity.
    #[must_use]
    pub fn logical_output(self) -> PointRef {
        PointRef::new(
            self.owner.roles.catalog().catalog().operation(),
            *self.association.role.tag(),
        )
    }
}
