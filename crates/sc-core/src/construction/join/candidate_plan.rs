//! Complete actual candidate copy-plan binding,not deep Piece-field preservation or physical proof.
use super::{
    JoinCopySourceAction, JoinCopySourceIntent, JoinResultCopyRequest,
    RequiredJoinCopyCorrespondence,
};
use crate::ontology::{CutCopy, CutPlan, CutPlanError, EntityId, Piece, PieceInventory};
use core::fmt;
use std::collections::BTreeMap;

/// Whole original C/I,result Q,candidate J/D and peak max(J+3D,I+J+D) entry allowances.
/// ```compile_fail
/// let limits:sc_core::construction::JoinCandidatePlanLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinCandidatePlanLimits {
    source_copies: usize,
    source_pieces: usize,
    results: usize,
    pieces: usize,
    copies: usize,
    workspace_entries: usize,
}
impl JoinCandidatePlanLimits {
    /// Admit every actual original/result/candidate member before fields or allocation.
    /// Workspace separately covers canonical-plan temporary validation and later persistent indexes.
    #[must_use]
    pub const fn new(
        source_copies: usize,
        source_pieces: usize,
        results: usize,
        pieces: usize,
        copies: usize,
        workspace_entries: usize,
    ) -> Self {
        Self {
            source_copies,
            source_pieces,
            results,
            pieces,
            copies,
            workspace_entries,
        }
    }
    /// Complete actual original copy/action population C.
    #[must_use]
    pub const fn source_copies(self) -> usize {
        self.source_copies
    }
    /// Complete actual original Piece population I,including unrelated/companion members.
    #[must_use]
    pub const fn source_pieces(self) -> usize {
        self.source_pieces
    }
    /// Complete actual explicitly requested result copies Q.
    #[must_use]
    pub const fn results(self) -> usize {
        self.results
    }
    /// Complete supplied candidate Piece population J.
    #[must_use]
    pub const fn pieces(self) -> usize {
        self.pieces
    }
    /// Complete supplied candidate physical-copy population D.
    #[must_use]
    pub const fn copies(self) -> usize {
        self.copies
    }
    /// Peak logical validation/index entries,not bytes or earlier immutable storage.
    #[must_use]
    pub const fn workspace_entries(self) -> usize {
        self.workspace_entries
    }
}
/// First whole resource/canonical-plan/count or exact retained/result copy-definition cause.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JoinCandidatePlanRefusal {
    /// Checked validation/persistent workspace cannot be represented.
    WorkspaceOverflow,
    /// Whole original C exceeds policy.
    SourceCopies {
        /// Actual whole C.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole original I exceeds policy.
    SourcePieces {
        /// Actual whole I.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole actual requested Q exceeds policy.
    Results {
        /// Actual whole Q.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole supplied candidate J exceeds policy.
    Pieces {
        /// Actual whole J.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole supplied candidate D exceeds policy.
    Copies {
        /// Actual whole D.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete peak new workspace exceeds policy.
    Workspace {
        /// Actual required entries.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Actual canonical CutPlan validation against these exact supplied candidate Pieces refused.
    Canonical(Box<CutPlanError>),
    /// Complete expected retained-original plus requested-result count cannot be represented.
    CountOverflow,
    /// Candidate plan must contain exactly retained original and explicitly requested result copies.
    Count {
        /// Complete expected retained+result copies.
        expected: usize,
        /// Complete actual candidate D.
        actual: usize,
    },
    /// Candidate plan lacks an actual original RetainOriginal copy.
    MissingRetained {
        /// Original source action row.
        source: usize,
    },
    /// Candidate retained copy changes original id/Piece/orientation definition.
    RetainedField {
        /// Original source action row.
        source: usize,
    },
    /// Candidate plan lacks an explicitly requested result copy.
    MissingResult {
        /// Actual stable requested result id.
        copy: EntityId,
    },
    /// Actual candidate result copy differs from its explicit id/Piece/orientation request.
    ResultField {
        /// Actual stable requested result id.
        copy: EntityId,
    },
    /// Internal fault:closed original-copy ownership lost its original Piece count slot.
    SourceInvariant {
        /// Actual original source copy id.
        copy: EntityId,
    },
    /// Internal fault:complete matched canonical result copies lost their declared result Piece.
    ResultInvariant,
}
/// Actual whole supplied candidate inventory/plan,original correspondence,policy and first cause.
pub struct JoinCandidatePlanError<'a> {
    correspondence: &'a RequiredJoinCopyCorrespondence<'a>,
    inventory: &'a PieceInventory,
    plan: &'a CutPlan,
    limits: JoinCandidatePlanLimits,
    refusal: JoinCandidatePlanRefusal,
    candidate_copy: Option<&'a CutCopy>,
}
impl<'a> JoinCandidatePlanError<'a> {
    /// Actual complete native copy correspondence and original contexts.
    #[must_use]
    pub const fn correspondence(&self) -> &'a RequiredJoinCopyCorrespondence<'a> {
        self.correspondence
    }
    /// Actual complete supplied candidate Piece inventory,without replacement values.
    #[must_use]
    pub const fn inventory(&self) -> &'a PieceInventory {
        self.inventory
    }
    /// Actual complete supplied candidate physical-copy plan,including refused suffixes.
    #[must_use]
    pub const fn plan(&self) -> &'a CutPlan {
        self.plan
    }
    /// Original complete-resource policy.
    #[must_use]
    pub const fn limits(&self) -> JoinCandidatePlanLimits {
        self.limits
    }
    /// First whole resource,canonical or exact definition cause.
    #[must_use]
    pub const fn refusal(&self) -> &JoinCandidatePlanRefusal {
        &self.refusal
    }
    /// Actual canonical cause only for Canonical;no rewritten incompatibility classification.
    #[must_use]
    pub fn canonical_cause(&self) -> Option<&CutPlanError> {
        match &self.refusal {
            JoinCandidatePlanRefusal::Canonical(cause) => Some(cause),
            _ => None,
        }
    }
    /// Actual original retained source action for missing/changed-retained causes;other causes None.
    #[must_use]
    pub fn refused_source(&self) -> Option<JoinCopySourceAction<'a>> {
        let index = match self.refusal {
            JoinCandidatePlanRefusal::MissingRetained { source }
            | JoinCandidatePlanRefusal::RetainedField { source } => source,
            _ => return None,
        };
        self.correspondence.sources().assignments().nth(index)
    }
    /// Actual explicit result request for missing/changed-result causes;other causes None.
    #[must_use]
    pub fn refused_result(&self) -> Option<JoinResultCopyRequest<'a>> {
        let id = match self.refusal {
            JoinCandidatePlanRefusal::MissingResult { copy }
            | JoinCandidatePlanRefusal::ResultField { copy } => copy,
            _ => return None,
        };
        self.correspondence.sources().results().copy(id)
    }
    /// Actual candidate copy only for retained/result field mismatch;missing/whole causes None.
    #[must_use]
    pub const fn candidate_copy(&self) -> Option<&'a CutCopy> {
        self.candidate_copy
    }
}
impl fmt::Debug for JoinCandidatePlanError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JoinCandidatePlanError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for JoinCandidatePlanError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "join candidate copy plan: {:?}", self.refusal)
    }
}
impl std::error::Error for JoinCandidatePlanError<'_> {}
impl<'a> RequiredJoinCopyCorrespondence<'a> {
    /// Bind a complete actual candidate CutPlan to the same supplied actual Piece inventory and
    /// exact retained-original/explicit-result definitions. Whole C/I/Q/J/D and checked peak
    /// max(J+3D,I+J+D) precede fields/allocation. Revalidate canonical CutPlan against these exact
    /// Pieces,then drop all validation temporaries before allocating the persistent I/J/D indexes.
    /// Tally every original Piece's retained-copy count from complete guarded original actions;
    /// zero is derived from closed coverage,not an unknown/default authored quantity. Require exact
    /// candidate D=retained+Q,then original retained definitions in authored action order and result
    /// definitions in requested order. Exact disjoint membership/count excludes all hidden copies.
    /// Work O((C+I+Q+J+D) log(C+I+Q+J+D+2)+1),peak as above,successful I+J+D new entries;
    /// earlier immutable context/storage remains separate. No labels,material reasons,contours or
    /// provider/evidence/geometry bodies are scanned. Deep whole Piece-field resources,original
    /// preservation and joined-result fields remain mandatory next,as do per-pair physical proofs,
    /// consumers,currentness,identity/retirement and atomic effects. This is not full result approval.
    /// # Errors
    /// Whole quotas,workspace arithmetic/quota,actual canonical cause,retained/result count,
    /// original retained membership/fields,result membership/fields. Invariants are internal faults
    /// requiring diagnosis;no accepted candidate prefix or reconstructed original object escapes.
    pub fn bind_candidate_plan(
        &'a self,
        inventory: &'a PieceInventory,
        plan: &'a CutPlan,
        limits: JoinCandidatePlanLimits,
    ) -> Result<BoundJoinCandidatePlan<'a>, JoinCandidatePlanError<'a>> {
        use JoinCandidatePlanRefusal as R;
        let refuse = |refusal| JoinCandidatePlanError {
            correspondence: self,
            inventory,
            plan,
            limits,
            refusal,
            candidate_copy: None,
        };
        let results = self.sources().results();
        let c = self.sources().copy_count();
        let i = results.source_piece_count();
        let q = results.copy_count();
        let j = inventory.pieces().len();
        let d = plan.copies().len();
        if c > limits.source_copies {
            return Err(refuse(R::SourceCopies {
                required: c,
                allowed: limits.source_copies,
            }));
        }
        if i > limits.source_pieces {
            return Err(refuse(R::SourcePieces {
                required: i,
                allowed: limits.source_pieces,
            }));
        }
        if q > limits.results {
            return Err(refuse(R::Results {
                required: q,
                allowed: limits.results,
            }));
        }
        if j > limits.pieces {
            return Err(refuse(R::Pieces {
                required: j,
                allowed: limits.pieces,
            }));
        }
        if d > limits.copies {
            return Err(refuse(R::Copies {
                required: d,
                allowed: limits.copies,
            }));
        }
        let validation = d
            .checked_mul(3)
            .and_then(|n| n.checked_add(j))
            .ok_or_else(|| refuse(R::WorkspaceOverflow))?;
        let persistent = i
            .checked_add(j)
            .and_then(|n| n.checked_add(d))
            .ok_or_else(|| refuse(R::WorkspaceOverflow))?;
        let workspace = validation.max(persistent);
        if workspace > limits.workspace_entries {
            return Err(refuse(R::Workspace {
                required: workspace,
                allowed: limits.workspace_entries,
            }));
        }
        let definitions = plan
            .copies()
            .iter()
            .map(|copy| *copy.definition())
            .collect();
        // A canonical error is boxed only after CutPlan::new has dropped its temporaries.
        // Its one diagnostic entry fits I+J+D:closed distinct A/B source inventory has I>=2.
        drop(
            CutPlan::new(definitions, inventory.pieces())
                .map_err(|cause| refuse(R::Canonical(Box::new(cause))))?,
        );
        let by_piece: BTreeMap<_, _> = inventory
            .pieces()
            .iter()
            .map(|piece| (piece.id(), piece))
            .collect();
        let by_copy: BTreeMap<_, _> = plan.copies().iter().map(|copy| (copy.id(), copy)).collect();
        let original_inventory = results
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
            .sources()
            .population()
            .metadata()
            .splice()
            .inventory()
            .inventory();
        let mut retained_by_piece: BTreeMap<_, _> = original_inventory
            .pieces()
            .iter()
            .map(|piece| (piece.id(), 0usize))
            .collect();
        let mut retained = 0usize;
        for source in self.sources().assignments() {
            if matches!(source.intent(), JoinCopySourceIntent::RetainOriginal) {
                let count = retained_by_piece
                    .get_mut(&source.source().piece())
                    .ok_or_else(|| {
                        refuse(R::SourceInvariant {
                            copy: source.source().id(),
                        })
                    })?;
                *count += 1;
                retained += 1; // Both counters are bounded by admitted complete C.
            }
        }
        let expected = retained
            .checked_add(q)
            .ok_or_else(|| refuse(R::CountOverflow))?;
        if d != expected {
            return Err(refuse(R::Count {
                expected,
                actual: d,
            }));
        }
        for source in self.sources().assignments() {
            if matches!(source.intent(), JoinCopySourceIntent::RetainOriginal) {
                let copy = by_copy.get(&source.source().id()).copied().ok_or_else(|| {
                    refuse(R::MissingRetained {
                        source: source.index(),
                    })
                })?;
                if copy.definition() != source.source_definition() {
                    let mut error = refuse(R::RetainedField {
                        source: source.index(),
                    });
                    error.candidate_copy = Some(copy);
                    return Err(error);
                }
            }
        }
        for result in results.copies() {
            let copy = by_copy
                .get(&result.id())
                .copied()
                .ok_or_else(|| refuse(R::MissingResult { copy: result.id() }))?;
            if copy.definition() != result.definition() {
                let mut error = refuse(R::ResultField { copy: result.id() });
                error.candidate_copy = Some(copy);
                return Err(error);
            }
        }
        let identity = results
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
            .identity();
        let joined = by_piece
            .get(identity.piece_id())
            .copied()
            .ok_or_else(|| refuse(R::ResultInvariant))?;
        Ok(BoundJoinCandidatePlan {
            correspondence: self,
            inventory,
            plan,
            limits,
            workspace,
            retained,
            retained_by_piece,
            by_piece,
            by_copy,
            joined,
        })
    }
}
/// Authentic complete candidate copy plan/counts,not deep original/result Piece-field preservation.
/// ```compile_fail
/// fn forge<'a>(mut plan:sc_core::construction::BoundJoinCandidatePlan<'a>,other:&'a sc_core::ontology::PieceInventory) {plan.inventory=other;}
/// ```
/// ```compile_fail
/// fn escape(pairs:sc_core::construction::RequiredJoinCopyCorrespondence<'_>,inventory:sc_core::ontology::PieceInventory,plan:sc_core::ontology::CutPlan)->sc_core::construction::BoundJoinCandidatePlan<'static> {
///     pairs.bind_candidate_plan(&inventory,&plan,sc_core::construction::JoinCandidatePlanLimits::new(0,0,0,0,0,0)).unwrap()
/// }
#[derive(Debug)]
pub struct BoundJoinCandidatePlan<'a> {
    correspondence: &'a RequiredJoinCopyCorrespondence<'a>,
    inventory: &'a PieceInventory,
    plan: &'a CutPlan,
    limits: JoinCandidatePlanLimits,
    workspace: usize,
    retained: usize,
    retained_by_piece: BTreeMap<EntityId, usize>,
    by_piece: BTreeMap<EntityId, &'a Piece>,
    by_copy: BTreeMap<EntityId, &'a CutCopy>,
    joined: &'a Piece,
}
impl<'a> BoundJoinCandidatePlan<'a> {
    /// Actual whole native original-copy correspondence/source/result parents.
    #[must_use]
    pub const fn correspondence(&self) -> &'a RequiredJoinCopyCorrespondence<'a> {
        self.correspondence
    }
    /// Actual complete original inventory,including fully consumed and unrelated members.
    #[must_use]
    pub fn original_inventory(&self) -> &'a PieceInventory {
        self.correspondence
            .sources()
            .results()
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
            .sources()
            .population()
            .metadata()
            .splice()
            .inventory()
            .inventory()
    }
    /// Actual complete supplied candidate Piece inventory.
    #[must_use]
    pub const fn inventory(&self) -> &'a PieceInventory {
        self.inventory
    }
    /// Actual complete supplied candidate CutPlan,revalidated against these same actual Pieces.
    #[must_use]
    pub const fn plan(&self) -> &'a CutPlan {
        self.plan
    }
    /// Original complete-resource policy.
    #[must_use]
    pub const fn limits(&self) -> JoinCandidatePlanLimits {
        self.limits
    }
    /// Peak max(J+3D,I+J+D) new logical entries;successful retained storage I+J+D.
    #[must_use]
    pub const fn workspace_entries(&self) -> usize {
        self.workspace
    }
    /// Actual complete candidate Piece population J.
    #[must_use]
    pub fn piece_count(&self) -> usize {
        self.by_piece.len()
    }
    /// Actual complete candidate physical-copy population D.
    #[must_use]
    pub fn copy_count(&self) -> usize {
        self.by_copy.len()
    }
    /// Complete actual original physical copies explicitly retained.
    #[must_use]
    pub const fn retained_copy_count(&self) -> usize {
        self.retained
    }
    /// Derived retained count for an actual original Piece,including known zero;unknown id None.
    #[must_use]
    pub fn original_retained_count(&self, id: EntityId) -> Option<usize> {
        self.retained_by_piece.get(&id).copied()
    }
    /// Actual candidate Piece by stable identity;deep field matching is not yet certified.
    #[must_use]
    pub fn piece(&self, id: EntityId) -> Option<&'a Piece> {
        self.by_piece.get(&id).copied()
    }
    /// Actual candidate copy by stable identity,without reconstruction or transforms.
    #[must_use]
    pub fn copy(&self, id: EntityId) -> Option<&'a CutCopy> {
        self.by_copy.get(&id).copied()
    }
    /// Actual declared-result Piece in this canonical candidate plan;deep authored fields still pending.
    #[must_use]
    pub const fn joined_piece(&self) -> &'a Piece {
        self.joined
    }
}
