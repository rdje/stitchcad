//! Explicit guarded companion-reference replacement requirements,not accepted consumer/evidence truth.
use super::BoundJoinInventoryFields;
use crate::ontology::{EntityId, Mirroring, Piece, SeamSideId};
use core::fmt;
use std::collections::BTreeMap;

/// One explicit kept original member,expected whole mirroring field,new companion and evidence request.
/// No handedness is inferred from names,copy reflection,side order or geometry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinCompanionRepairRequest {
    piece: EntityId,
    expected: Mirroring,
    companion: EntityId,
    evidence_source: EntityId,
}
impl JoinCompanionRepairRequest {
    /// Retain every original field,including unresolved or reused evidence ids,without mutation.
    #[must_use]
    pub const fn new(
        piece: EntityId,
        expected: Mirroring,
        companion: EntityId,
        evidence_source: EntityId,
    ) -> Self {
        Self {
            piece,
            expected,
            companion,
            evidence_source,
        }
    }
    /// Actual authored original retained Piece selector.
    #[must_use]
    pub const fn piece(&self) -> &EntityId {
        &self.piece
    }
    /// Actual expected whole original mirrored-member field,not a replacement source.
    #[must_use]
    pub const fn expected(&self) -> &Mirroring {
        &self.expected
    }
    /// Actual explicitly requested new companion identity,not an ordinal or inferred mate.
    #[must_use]
    pub const fn companion(&self) -> &EntityId {
        &self.companion
    }
    /// Actual authored consumer/reference justification request,no accepted evidence or authority.
    #[must_use]
    pub const fn evidence_source(&self) -> &EntityId {
        &self.evidence_source
    }
}
/// Complete raw repair M and checked2M index/association allowances before fields or allocation.
/// ```compile_fail
/// let limits:sc_core::construction::JoinCompanionRepairLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinCompanionRepairLimits {
    repairs: usize,
    workspace_entries: usize,
}
impl JoinCompanionRepairLimits {
    /// Earlier complete I/J/H/R/B and copy-plan resources stay separate and immutable.
    #[must_use]
    pub const fn new(repairs: usize, workspace_entries: usize) -> Self {
        Self {
            repairs,
            workspace_entries,
        }
    }
    /// Every original raw repair request.
    #[must_use]
    pub const fn repairs(self) -> usize {
        self.repairs
    }
    /// Complete2M new source-index/request associations,not bytes or prior storage.
    #[must_use]
    pub const fn workspace_entries(self) -> usize {
        self.workspace_entries
    }
}
/// First whole arithmetic/resource,operation or original guarded reference cause.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinCompanionRepairRefusal {
    /// Checked complete2M cannot be represented.
    WorkspaceOverflow,
    /// Whole raw requests exceed policy.
    Repairs {
        /// Actual whole M.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete2M new entries exceed policy.
    Workspace {
        /// Actual required entries.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Raw manifest names another operation.
    Operation,
    /// Original retained Piece selector repeats.
    Duplicate {
        /// First original row.
        first: usize,
        /// Repeated original row.
        second: usize,
    },
    /// Selector names no actual original Piece in the full supplied source inventory.
    MissingSource {
        /// Original request.
        index: usize,
    },
    /// Expected whole mirroring field differs from the actual original field.
    SourceField {
        /// Original request.
        index: usize,
    },
    /// Actual original field is not a separately identified PairMember.
    NotPairMember {
        /// Original request.
        index: usize,
    },
    /// This request requires an actual retained original member,not a fully consumed source.
    SourceNotRetained {
        /// Original request.
        index: usize,
    },
    /// Old companion is not one of the actual A/B source Pieces being joined.
    CompanionNotSource {
        /// Original request.
        index: usize,
    },
    /// Old companion still has actual retained copies and cannot be silently replaced.
    CompanionNotConsumed {
        /// Original request.
        index: usize,
    },
    /// Explicit new companion is not the actual declared joined result Piece.
    ResultIdentity {
        /// Original request.
        index: usize,
    },
    /// Authored result mirroring does not explicitly reciprocate this member with opposite hand.
    ResultPairing {
        /// Original request.
        index: usize,
    },
    /// Actual candidate original member does not retain its hand and name that exact new companion.
    CandidateField {
        /// Original request.
        index: usize,
    },
    /// Internal fault:closed retained-copy/canonical membership lost the candidate original Piece.
    CandidateInvariant {
        /// Original request exposing the fault.
        index: usize,
    },
    /// Internal fault:closed original reciprocal inventory lost the old companion.
    SourceInvariant {
        /// Original request exposing the fault.
        index: usize,
    },
}
/// Whole original requests,actual field/inventory parents,policy and original first cause survive.
pub struct JoinCompanionRepairError<'a> {
    manifest: &'a JoinCompanionRepairManifest,
    fields: &'a BoundJoinInventoryFields<'a>,
    limits: JoinCompanionRepairLimits,
    refusal: JoinCompanionRepairRefusal,
}
impl<'a> JoinCompanionRepairError<'a> {
    /// Actual complete original request/evidence proposal.
    #[must_use]
    pub const fn manifest(&self) -> &'a JoinCompanionRepairManifest {
        self.manifest
    }
    /// Actual complete admitted original/candidate body/copy/identity contexts.
    #[must_use]
    pub const fn fields(&self) -> &'a BoundJoinInventoryFields<'a> {
        self.fields
    }
    /// Original explicit whole-resource policy.
    #[must_use]
    pub const fn limits(&self) -> JoinCompanionRepairLimits {
        self.limits
    }
    /// First whole resource,operation or original guard/reference cause.
    #[must_use]
    pub const fn refusal(&self) -> JoinCompanionRepairRefusal {
        self.refusal
    }
    /// Actual original refused raw request;whole resources/operation return None.
    #[must_use]
    pub fn refused_request(&self) -> Option<&'a JoinCompanionRepairRequest> {
        use JoinCompanionRepairRefusal as R;
        let index = match self.refusal {
            R::Duplicate { second, .. } => second,
            R::MissingSource { index }
            | R::SourceField { index }
            | R::NotPairMember { index }
            | R::SourceNotRetained { index }
            | R::CompanionNotSource { index }
            | R::CompanionNotConsumed { index }
            | R::ResultIdentity { index }
            | R::ResultPairing { index }
            | R::CandidateField { index }
            | R::CandidateInvariant { index }
            | R::SourceInvariant { index } => index,
            _ => return None,
        };
        self.manifest.requests.get(index)
    }
    /// Actual known original source for a row cause,if that persistent selector exists.
    #[must_use]
    pub fn original_piece(&self) -> Option<&'a Piece> {
        self.fields
            .plan()
            .correspondence()
            .sources()
            .results()
            .source_piece(*self.refused_request()?.piece())
    }
    /// Actual known candidate Piece for a row cause,if that same retained selector exists.
    #[must_use]
    pub fn candidate_piece(&self) -> Option<&'a Piece> {
        self.fields.plan().piece(*self.refused_request()?.piece())
    }
}
impl fmt::Debug for JoinCompanionRepairError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JoinCompanionRepairError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for JoinCompanionRepairError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "join companion repair requirements: {:?}", self.refusal)
    }
}
impl std::error::Error for JoinCompanionRepairError<'_> {}
/// Original operation and explicit ordered requests;empty means no companion exceptions requested.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JoinCompanionRepairManifest {
    operation: EntityId,
    requests: Vec<JoinCompanionRepairRequest>,
}
impl JoinCompanionRepairManifest {
    /// Retain all original guards/targets/evidence requests without sorting or source replacement.
    #[must_use]
    pub const fn new(operation: EntityId, requests: Vec<JoinCompanionRepairRequest>) -> Self {
        Self {
            operation,
            requests,
        }
    }
    /// Actual original operation.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        self.operation
    }
    /// Complete original raw companion requirements.
    #[must_use]
    pub fn requests(&self) -> &[JoinCompanionRepairRequest] {
        &self.requests
    }
    /// Admit complete M/checked2M before operation/fields/allocation. Each unique original source
    /// must match its expected PairMember field and retain copies. Its old companion must be an
    /// actual fully consumed A/B source. The explicit new companion must be the declared result;
    /// the authored result population must explicitly name this member with opposite handedness.
    /// Actual candidate member must preserve its original hand and name that same new result.
    /// Work O(M log(M+I+J+2)+1),2M new entries,earlier whole immutable I/J/H/R/B/resources remain.
    /// No name/parity/geometry inference,provider/evidence body lookup,currentness or consumer
    /// approval occurs. Evidence ids remain outstanding requests. Original field matching still must
    /// reject every uncovered mirroring change and preserve all other fields except derived quantity.
    /// This is a visible reference-repair requirement,not permission to reassign historical consumers.
    /// # Errors
    /// Whole M/2M,operation;first original duplicate/missing/expected-kind/retention/old-target/new-
    /// target/requested/candidate field cause. Internal invariants require diagnosis,no accepted prefix.
    pub fn admit<'a>(
        &'a self,
        fields: &'a BoundJoinInventoryFields<'a>,
        limits: JoinCompanionRepairLimits,
    ) -> Result<RequiredJoinCompanionRepairs<'a>, JoinCompanionRepairError<'a>> {
        use JoinCompanionRepairRefusal as R;
        let refuse = |refusal| JoinCompanionRepairError {
            manifest: self,
            fields,
            limits,
            refusal,
        };
        let m = self.requests.len();
        if m > limits.repairs {
            return Err(refuse(R::Repairs {
                required: m,
                allowed: limits.repairs,
            }));
        }
        let workspace = m
            .checked_mul(2)
            .ok_or_else(|| refuse(R::WorkspaceOverflow))?;
        if workspace > limits.workspace_entries {
            return Err(refuse(R::Workspace {
                required: workspace,
                allowed: limits.workspace_entries,
            }));
        }
        let results = fields.plan().correspondence().sources().results();
        if self.operation != results.manifest().operation() {
            return Err(refuse(R::Operation));
        }
        let sources = results
            .folds()
            .fold()
            .construction()
            .targets()
            .coverage()
            .groups()
            .sources()
            .uses()
            .connectivity()
            .contours()
            .directions()
            .coverage()
            .edges()
            .identity()
            .roles()
            .banks()
            .sources();
        let a = sources.piece(SeamSideId::A).id();
        let b = sources.piece(SeamSideId::B).id();
        let result_id = fields.plan().joined_piece().id();
        let mut by_piece = BTreeMap::new();
        let mut repairs = Vec::with_capacity(m);
        for (index, row) in self.requests.iter().enumerate() {
            if let Some(first) = by_piece.insert(row.piece, index) {
                return Err(refuse(R::Duplicate {
                    first,
                    second: index,
                }));
            }
            let original = results
                .source_piece(row.piece)
                .ok_or_else(|| refuse(R::MissingSource { index }))?;
            if original.definition().mirroring != row.expected {
                return Err(refuse(R::SourceField { index }));
            }
            let Mirroring::PairMember {
                handedness,
                companion,
            } = original.definition().mirroring
            else {
                return Err(refuse(R::NotPairMember { index }));
            };
            let retained = fields
                .plan()
                .original_retained_count(row.piece)
                .ok_or_else(|| refuse(R::SourceInvariant { index }))?;
            if retained == 0 {
                return Err(refuse(R::SourceNotRetained { index }));
            }
            if companion != a && companion != b {
                return Err(refuse(R::CompanionNotSource { index }));
            }
            let remaining = fields
                .plan()
                .original_retained_count(companion)
                .ok_or_else(|| refuse(R::SourceInvariant { index }))?;
            if remaining != 0 {
                return Err(refuse(R::CompanionNotConsumed { index }));
            }
            if row.companion != result_id {
                return Err(refuse(R::ResultIdentity { index }));
            }
            if !matches!(sources.population().mirroring(),Mirroring::PairMember{handedness:requested,companion:mate} if *requested!=handedness && *mate==row.piece)
            {
                return Err(refuse(R::ResultPairing { index }));
            }
            let candidate = fields
                .plan()
                .piece(row.piece)
                .ok_or_else(|| refuse(R::CandidateInvariant { index }))?;
            if candidate.definition().mirroring
                != (Mirroring::PairMember {
                    handedness,
                    companion: row.companion,
                })
            {
                return Err(refuse(R::CandidateField { index }));
            }
            let retired = results
                .source_piece(companion)
                .ok_or_else(|| refuse(R::SourceInvariant { index }))?;
            repairs.push(CompanionAssociation {
                index,
                row,
                original,
                candidate,
                retired,
            });
        }
        Ok(RequiredJoinCompanionRepairs {
            manifest: self,
            fields,
            limits,
            workspace,
            by_piece,
            repairs,
        })
    }
}
#[derive(Debug)]
struct CompanionAssociation<'a> {
    index: usize,
    row: &'a JoinCompanionRepairRequest,
    original: &'a Piece,
    candidate: &'a Piece,
    retired: &'a Piece,
}
/// Authentic explicit counterpart requirements;not complete original preservation or repair approval.
/// ```compile_fail
/// fn forge<'a>(mut repairs:sc_core::construction::RequiredJoinCompanionRepairs<'a>,other:&'a sc_core::construction::JoinCompanionRepairManifest) {repairs.manifest=other;}
/// ```
/// ```compile_fail
/// fn escape(raw:sc_core::construction::JoinCompanionRepairManifest,fields:sc_core::construction::BoundJoinInventoryFields<'_>)->sc_core::construction::RequiredJoinCompanionRepairs<'static> {
///     raw.admit(&fields,sc_core::construction::JoinCompanionRepairLimits::new(0,0)).unwrap()
/// }
#[derive(Debug)]
pub struct RequiredJoinCompanionRepairs<'a> {
    manifest: &'a JoinCompanionRepairManifest,
    fields: &'a BoundJoinInventoryFields<'a>,
    limits: JoinCompanionRepairLimits,
    workspace: usize,
    by_piece: BTreeMap<EntityId, usize>,
    repairs: Vec<CompanionAssociation<'a>>,
}
impl<'a> RequiredJoinCompanionRepairs<'a> {
    /// Actual complete original ordered repair/evidence proposal.
    #[must_use]
    pub const fn manifest(&self) -> &'a JoinCompanionRepairManifest {
        self.manifest
    }
    /// Actual whole admitted original/candidate resources and copy/identity contexts.
    #[must_use]
    pub const fn fields(&self) -> &'a BoundJoinInventoryFields<'a> {
        self.fields
    }
    /// Original complete-resource policy.
    #[must_use]
    pub const fn limits(&self) -> JoinCompanionRepairLimits {
        self.limits
    }
    /// Complete2M new index/association entries,not bytes or earlier immutable contexts.
    #[must_use]
    pub const fn workspace_entries(&self) -> usize {
        self.workspace
    }
    /// Complete explicitly requested repair population;not inferred changed-field coverage.
    #[must_use]
    pub fn repair_count(&self) -> usize {
        self.repairs.len()
    }
    /// Authentic requirements in original request order,and reverse.
    pub fn repairs(
        &self,
    ) -> impl ExactSizeIterator<Item = JoinCompanionRepair<'_>> + DoubleEndedIterator {
        self.repairs
            .iter()
            .map(move |association| JoinCompanionRepair {
                context: self,
                association,
            })
    }
    /// Authentic requested repair by original retained Piece identity.
    #[must_use]
    pub fn repair(&self, piece: EntityId) -> Option<JoinCompanionRepair<'_>> {
        let i = self.by_piece.get(&piece)?;
        self.repairs.get(*i).map(|association| JoinCompanionRepair {
            context: self,
            association,
        })
    }
}
/// One authentic original/current companion-field request and its unaccepted evidence source.
#[derive(Clone, Copy, Debug)]
pub struct JoinCompanionRepair<'a> {
    context: &'a RequiredJoinCompanionRepairs<'a>,
    association: &'a CompanionAssociation<'a>,
}
impl<'a> JoinCompanionRepair<'a> {
    /// Actual whole requirement owner.
    #[must_use]
    pub const fn context(self) -> &'a RequiredJoinCompanionRepairs<'a> {
        self.context
    }
    /// Original raw request ordinal,not a Piece identity.
    #[must_use]
    pub const fn index(self) -> usize {
        self.association.index
    }
    /// Actual original guard/target/evidence fields.
    #[must_use]
    pub const fn request(self) -> &'a JoinCompanionRepairRequest {
        self.association.row
    }
    /// Actual retained original member,not a reconstructed expected value.
    #[must_use]
    pub const fn original(self) -> &'a Piece {
        self.association.original
    }
    /// Actual candidate member retaining its hand and explicitly naming the new result.
    #[must_use]
    pub const fn candidate(self) -> &'a Piece {
        self.association.candidate
    }
    /// Actual original fully consumed A/B companion;no retirement effect is performed.
    #[must_use]
    pub const fn retired_companion(self) -> &'a Piece {
        self.association.retired
    }
    /// Actual original old companion/hand field.
    #[must_use]
    pub fn original_mirroring(self) -> &'a Mirroring {
        &self.association.original.definition().mirroring
    }
    /// Actual candidate requested new companion/same hand field,not geometric mirroring evidence.
    #[must_use]
    pub fn candidate_mirroring(self) -> &'a Mirroring {
        &self.association.candidate.definition().mirroring
    }
    /// Actual authored evidence source id field,no kind/state/currentness/truth/approval implied.
    #[must_use]
    pub const fn evidence_source(self) -> &'a EntityId {
        self.association.row.evidence_source()
    }
}
