//! Complete original physical-copy requests,not selected-source consumption or exact no-fork pairing.
use super::{DeclaredJoinResultCopies, JoinResultCopyRequest};
use crate::ontology::{CutCopy, CutCopyDefinition, EntityId, SeamSideId};
use core::fmt;
use std::collections::BTreeMap;

/// Explicit original-copy preservation or contribution to a named actual result-copy request.
/// ```compile_fail
/// let intent:sc_core::construction::JoinCopySourceIntent=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinCopySourceIntent {
    /// Preserve the exact original id/Piece/orientation fields;no reassignment or silent deletion.
    RetainOriginal,
    /// Request consumption of this actual A/B source copy into a named actual result request.
    /// Exact selected-copy correspondence and one A+B pair per result remain mandatory next.
    JoinTo {
        /// Original explicit persistent requested-result copy identity,not a copy ordinal.
        result_copy: EntityId,
    },
}
/// Original complete expected physical-copy fields plus explicit Retain/JoinTo intent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinCopySourceAssignment {
    expected: CutCopyDefinition,
    intent: JoinCopySourceIntent,
}
impl JoinCopySourceAssignment {
    /// Retain all original expected id/Piece/orientation fields;the id selects the actual source copy.
    #[must_use]
    pub const fn new(expected: CutCopyDefinition, intent: JoinCopySourceIntent) -> Self {
        Self { expected, intent }
    }
    /// Actual original expected whole source definition,not a replacement current copy.
    #[must_use]
    pub const fn expected(&self) -> &CutCopyDefinition {
        &self.expected
    }
    /// Actual original explicit source-copy action.
    #[must_use]
    pub const fn intent(&self) -> &JoinCopySourceIntent {
        &self.intent
    }
    /// Actual authored result id field for JoinTo,or None for RetainOriginal.
    #[must_use]
    pub const fn result_copy(&self) -> Option<&EntityId> {
        match &self.intent {
            JoinCopySourceIntent::RetainOriginal => None,
            JoinCopySourceIntent::JoinTo { result_copy } => Some(result_copy),
        }
    }
}
/// Complete actual original C,raw action M and checked2M new entry allowances.
/// ```compile_fail
/// let limits:sc_core::construction::JoinCopySourceLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinCopySourceLimits {
    copies: usize,
    assignments: usize,
    workspace_entries: usize,
}
impl JoinCopySourceLimits {
    /// Admit every original copy and every raw row/checked2M before fields or allocation.
    /// Earlier immutable original/result inventory and fold/construction resources remain separate.
    #[must_use]
    pub const fn new(copies: usize, assignments: usize, workspace_entries: usize) -> Self {
        Self {
            copies,
            assignments,
            workspace_entries,
        }
    }
    /// Every original copy,including unselected and unrelated members.
    #[must_use]
    pub const fn copies(self) -> usize {
        self.copies
    }
    /// Every original raw action row.
    #[must_use]
    pub const fn assignments(self) -> usize {
        self.assignments
    }
    /// Complete2M new source-index/association entries,not bytes or earlier storage.
    #[must_use]
    pub const fn workspace_entries(self) -> usize {
        self.workspace_entries
    }
}
/// First complete arithmetic/resource,owner/count or original source/intent cause.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinCopySourceRefusal {
    /// Checked complete2M cannot be represented.
    WorkspaceOverflow,
    /// Whole original copy population exceeds policy.
    Copies {
        /// Actual whole C.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole raw action population exceeds policy.
    Assignments {
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
    /// Raw operation differs from the actual result/source context.
    Operation,
    /// One unique guarded action per complete original copy is required.
    Count {
        /// Actual complete original copies.
        expected: usize,
        /// Raw original actions.
        actual: usize,
    },
    /// Original physical-copy id repeats across action rows.
    DuplicateCopy {
        /// First original row.
        first: usize,
        /// Repeated original row.
        second: usize,
    },
    /// Expected id names no actual original physical copy.
    MissingCopy {
        /// Original row.
        index: usize,
    },
    /// Expected whole original Piece/orientation definition differs from the actual copy.
    SourceField {
        /// Original row.
        index: usize,
    },
    /// Only physical copies of actual original A/B Pieces may contribute to this join.
    ForeignPiece {
        /// Original row.
        index: usize,
    },
    /// JoinTo names no actual explicit result-copy request.
    UnknownResult {
        /// Original row.
        index: usize,
    },
}
/// Whole original proposal/result/source parents,policy and original first refusal.
pub struct JoinCopySourceError<'a> {
    manifest: &'a JoinCopySourceManifest,
    results: &'a DeclaredJoinResultCopies<'a>,
    limits: JoinCopySourceLimits,
    refusal: JoinCopySourceRefusal,
}
impl<'a> JoinCopySourceError<'a> {
    /// Actual complete original action proposal,including invalid suffixes.
    #[must_use]
    pub const fn manifest(&self) -> &'a JoinCopySourceManifest {
        self.manifest
    }
    /// Actual complete result requests/original inventory/fold/construction parents.
    #[must_use]
    pub const fn results(&self) -> &'a DeclaredJoinResultCopies<'a> {
        self.results
    }
    /// Original complete-resource policy.
    #[must_use]
    pub const fn limits(&self) -> JoinCopySourceLimits {
        self.limits
    }
    /// First whole resource,owner/count or original field cause.
    #[must_use]
    pub const fn refusal(&self) -> JoinCopySourceRefusal {
        self.refusal
    }
    /// Actual original refused raw row;whole resource/operation/count causes None.
    #[must_use]
    pub fn refused_assignment(&self) -> Option<&'a JoinCopySourceAssignment> {
        use JoinCopySourceRefusal as R;
        let index = match self.refusal {
            R::DuplicateCopy { second, .. } => second,
            R::MissingCopy { index }
            | R::SourceField { index }
            | R::ForeignPiece { index }
            | R::UnknownResult { index } => index,
            _ => return None,
        };
        self.manifest.assignments.get(index)
    }
    /// Actual known original copy for a row cause,if that persistent selector exists.
    #[must_use]
    pub fn refused_copy(&self) -> Option<&'a CutCopy> {
        self.results
            .source_copy(self.refused_assignment()?.expected.id)
    }
    /// Actual original authored result id only for UnknownResult;other causes None.
    #[must_use]
    pub fn refused_result(&self) -> Option<&'a EntityId> {
        if !matches!(self.refusal, JoinCopySourceRefusal::UnknownResult { .. }) {
            return None;
        }
        self.refused_assignment()?.result_copy()
    }
}
impl fmt::Debug for JoinCopySourceError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JoinCopySourceError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for JoinCopySourceError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "join original copy actions: {:?}", self.refusal)
    }
}
impl std::error::Error for JoinCopySourceError<'_> {}
/// Original operation and complete ordered original physical-copy actions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JoinCopySourceManifest {
    operation: EntityId,
    assignments: Vec<JoinCopySourceAssignment>,
}
impl JoinCopySourceManifest {
    /// Retain original operation/expected source/action fields without defaults or replacement copies.
    #[must_use]
    pub const fn new(operation: EntityId, assignments: Vec<JoinCopySourceAssignment>) -> Self {
        Self {
            operation,
            assignments,
        }
    }
    /// Original explicit operation.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        self.operation
    }
    /// Complete original raw expected-copy/action rows.
    #[must_use]
    pub fn assignments(&self) -> &[JoinCopySourceAssignment] {
        &self.assignments
    }
    /// Admit complete actual C/raw M/checked2M before fields/allocation. Operation/exact M=C precede
    /// original duplicate/missing/whole-definition guards. Unique actual membership plus exact count
    /// covers all original copies,including unrelated and unselected members. RetainOriginal preserves
    /// exact actual id/Piece/orientation;JoinTo requires actual source A/B ownership and actual result
    /// request membership. There is no implicit discard,copy reassignment,orientation or pairing.
    /// Work O(M log(M+C+Q+2)+1),2M new entries for Q actual result requests. Earlier complete
    /// immutable inventories/resource policies remain. Selected copies may still request Retain here;
    /// exact no-fork pairing must require actual selected A/B in the same result and one A+B per result.
    /// No provider,geometry,copy transform,global currentness,id issuance or atomic effect follows.
    /// # Errors
    /// Whole C/M,2M arithmetic/quota,operation,count;first original duplicate/missing/guard/foreign
    /// source/unknown result cause. No accepted prefix or replacement original copy escapes.
    pub fn admit<'a>(
        &'a self,
        results: &'a DeclaredJoinResultCopies<'a>,
        limits: JoinCopySourceLimits,
    ) -> Result<DeclaredJoinCopySources<'a>, JoinCopySourceError<'a>> {
        use JoinCopySourceRefusal as R;
        let refuse = |refusal| JoinCopySourceError {
            manifest: self,
            results,
            limits,
            refusal,
        };
        let c = results.source_copy_count();
        let m = self.assignments.len();
        if c > limits.copies {
            return Err(refuse(R::Copies {
                required: c,
                allowed: limits.copies,
            }));
        }
        if m > limits.assignments {
            return Err(refuse(R::Assignments {
                required: m,
                allowed: limits.assignments,
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
        if self.operation != results.manifest().operation() {
            return Err(refuse(R::Operation));
        }
        if m != c {
            return Err(refuse(R::Count {
                expected: c,
                actual: m,
            }));
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
        let mut by_copy = BTreeMap::new();
        let mut actions = Vec::with_capacity(m);
        for (index, row) in self.assignments.iter().enumerate() {
            if let Some(first) = by_copy.insert(row.expected.id, index) {
                return Err(refuse(R::DuplicateCopy {
                    first,
                    second: index,
                }));
            }
            let original = results
                .source_copy(row.expected.id)
                .ok_or_else(|| refuse(R::MissingCopy { index }))?;
            if *original.definition() != row.expected {
                return Err(refuse(R::SourceField { index }));
            }
            let side = if original.piece() == a {
                Some(SeamSideId::A)
            } else if original.piece() == b {
                Some(SeamSideId::B)
            } else {
                None
            };
            let result = match row.intent {
                JoinCopySourceIntent::RetainOriginal => None,
                JoinCopySourceIntent::JoinTo { result_copy } => {
                    if side.is_none() {
                        return Err(refuse(R::ForeignPiece { index }));
                    }
                    Some(
                        results
                            .copy(result_copy)
                            .ok_or_else(|| refuse(R::UnknownResult { index }))?,
                    )
                }
            };
            actions.push(CopyAssociation {
                index,
                row,
                original,
                side,
                result,
            });
        }
        Ok(DeclaredJoinCopySources {
            manifest: self,
            results,
            limits,
            workspace,
            by_copy,
            actions,
        })
    }
}
#[derive(Debug)]
struct CopyAssociation<'a> {
    index: usize,
    row: &'a JoinCopySourceAssignment,
    original: &'a CutCopy,
    side: Option<SeamSideId>,
    result: Option<JoinResultCopyRequest<'a>>,
}
/// Complete guarded original-copy action requests;exact selected pairing/physical effects pending.
/// ```compile_fail
/// fn forge<'a>(mut copies:sc_core::construction::DeclaredJoinCopySources<'a>,other:&'a sc_core::construction::JoinCopySourceManifest) {copies.manifest=other;}
/// ```
/// ```compile_fail
/// fn escape(raw:sc_core::construction::JoinCopySourceManifest,results:sc_core::construction::DeclaredJoinResultCopies<'_>)->sc_core::construction::DeclaredJoinCopySources<'static> {
///     raw.admit(&results,sc_core::construction::JoinCopySourceLimits::new(0,0,0)).unwrap()
/// }
#[derive(Debug)]
pub struct DeclaredJoinCopySources<'a> {
    manifest: &'a JoinCopySourceManifest,
    results: &'a DeclaredJoinResultCopies<'a>,
    limits: JoinCopySourceLimits,
    workspace: usize,
    by_copy: BTreeMap<EntityId, usize>,
    actions: Vec<CopyAssociation<'a>>,
}
impl<'a> DeclaredJoinCopySources<'a> {
    /// Actual complete original expected-copy/action proposal.
    #[must_use]
    pub const fn manifest(&self) -> &'a JoinCopySourceManifest {
        self.manifest
    }
    /// Actual complete requested copies/original inventory/fold/construction parents.
    #[must_use]
    pub const fn results(&self) -> &'a DeclaredJoinResultCopies<'a> {
        self.results
    }
    /// Original complete-resource policy.
    #[must_use]
    pub const fn limits(&self) -> JoinCopySourceLimits {
        self.limits
    }
    /// Complete2M new source-index/action entries,not bytes or earlier immutable storage.
    #[must_use]
    pub const fn workspace_entries(&self) -> usize {
        self.workspace
    }
    /// Complete actual original physical-copy action population.
    #[must_use]
    pub fn copy_count(&self) -> usize {
        self.actions.len()
    }
    /// Authentic original actions in authored order,and reverse.
    pub fn assignments(
        &self,
    ) -> impl ExactSizeIterator<Item = JoinCopySourceAction<'_>> + DoubleEndedIterator {
        self.actions
            .iter()
            .map(move |association| JoinCopySourceAction {
                context: self,
                association,
            })
    }
    /// Authentic original action by stable original physical-copy id.
    #[must_use]
    pub fn assignment(&self, id: EntityId) -> Option<JoinCopySourceAction<'_>> {
        let i = self.by_copy.get(&id)?;
        self.actions
            .get(*i)
            .map(|association| JoinCopySourceAction {
                context: self,
                association,
            })
    }
}
/// One authentic original guarded action,actual physical copy/role and actual requested result.
#[derive(Clone, Copy, Debug)]
pub struct JoinCopySourceAction<'a> {
    context: &'a DeclaredJoinCopySources<'a>,
    association: &'a CopyAssociation<'a>,
}
impl<'a> JoinCopySourceAction<'a> {
    /// Actual complete source/result action owner.
    #[must_use]
    pub const fn context(self) -> &'a DeclaredJoinCopySources<'a> {
        self.context
    }
    /// Original action row ordinal,not a physical-copy identity.
    #[must_use]
    pub const fn index(self) -> usize {
        self.association.index
    }
    /// Actual original expected-copy/intent fields.
    #[must_use]
    pub const fn assignment(self) -> &'a JoinCopySourceAssignment {
        self.association.row
    }
    /// Actual canonical immutable original physical copy,not expected-field reconstruction.
    #[must_use]
    pub const fn source(self) -> &'a CutCopy {
        self.association.original
    }
    /// Actual original copy definition field loan,including persistent id/Piece/orientation.
    #[must_use]
    pub const fn source_definition(self) -> &'a CutCopyDefinition {
        self.association.original.definition()
    }
    /// Actual original source A/B Piece role,or None for an unrelated retained copy.
    #[must_use]
    pub const fn side(self) -> Option<SeamSideId> {
        self.association.side
    }
    /// Actual original explicit source action field.
    #[must_use]
    pub const fn intent(self) -> &'a JoinCopySourceIntent {
        self.association.row.intent()
    }
    /// Actual requested result only for JoinTo;RetainOriginal never implies a replacement target.
    #[must_use]
    pub const fn result(self) -> Option<JoinResultCopyRequest<'a>> {
        self.association.result
    }
}
