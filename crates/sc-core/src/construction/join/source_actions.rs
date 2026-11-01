//! Complete explicit original construction retention/omission requests,not target mappings/evidence.
use super::{BoundJoinSourcePieces, DeclaredJoinConstructionUses};
use crate::ontology::{CutCopy, DirectedEdge, EntityId, Piece, SeamSideId};
use core::fmt;
use std::collections::BTreeMap;
/// Explicit original-field intent;neither variant performs an effect or supplies accepted evidence.
/// ```compile_fail
/// let intent:sc_core::construction::JoinConstructionSourceIntent=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinConstructionSourceIntent {
    /// Require complete explicit retained output mapping for this original occurrence later.
    Retain,
    /// Request omission justification/consumer repair from this explicit source,not approval.
    Omit {
        /// Original requested source id;zero/shared values remain ordinary requests.
        evidence_source: EntityId,
    },
}
/// Original A/B/snapshot location,expected persistent edge/traversal and explicit action.
/// Indices locate fields in the actual immutable parent;they are never persistent geometry ids.
/// ```compile_fail
/// use sc_core::{construction::*,ontology::*};
/// JoinConstructionSourceAssignment::new(SeamSideId::A,0,
///     PointRef::new(EntityId::from_bits(1),LocalTag::FIRST),JoinConstructionSourceIntent::Retain);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinConstructionSourceAssignment {
    side: SeamSideId,
    source_index: usize,
    expected: DirectedEdge,
    intent: JoinConstructionSourceIntent,
}
impl JoinConstructionSourceAssignment {
    /// Retain all explicit fields;expected EdgeRef/traversal guards the snapshot locator.
    #[must_use]
    pub const fn new(
        side: SeamSideId,
        source_index: usize,
        expected: DirectedEdge,
        intent: JoinConstructionSourceIntent,
    ) -> Self {
        Self {
            side,
            source_index,
            expected,
            intent,
        }
    }
    /// Original A/B source role,not inferred from an edge id or Piece ordering.
    #[must_use]
    pub const fn side(&self) -> SeamSideId {
        self.side
    }
    /// Original snapshot construction-field location,not a persistent identity.
    #[must_use]
    pub const fn source_index(&self) -> usize {
        self.source_index
    }
    /// Actual original expected persistent edge/traversal field,without currentness proof.
    #[must_use]
    pub const fn expected(&self) -> &DirectedEdge {
        &self.expected
    }
    /// Actual original explicit retention/omission request.
    #[must_use]
    pub const fn intent(&self) -> &JoinConstructionSourceIntent {
        &self.intent
    }
    /// Actual requested omission evidence field;Retain returns None. No registry fact follows.
    #[must_use]
    pub const fn evidence_source(&self) -> Option<&EntityId> {
        match &self.intent {
            JoinConstructionSourceIntent::Retain => None,
            JoinConstructionSourceIntent::Omit { evidence_source } => Some(evidence_source),
        }
    }
}
/// Whole original source R,raw assignment M and checked2M index/association allowances.
/// ```compile_fail
/// let limits:sc_core::construction::JoinConstructionSourceLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinConstructionSourceLimits {
    source_occurrences: usize,
    assignments: usize,
    workspace_entries: usize,
}
impl JoinConstructionSourceLimits {
    /// Bound all original cut/construction/fold occurrences,all raw actions and checked2M entries
    /// before source fields/allocation. Entry counts do not measure allocator/input-storage bytes.
    #[must_use]
    pub const fn new(
        source_occurrences: usize,
        assignments: usize,
        workspace_entries: usize,
    ) -> Self {
        Self {
            source_occurrences,
            assignments,
            workspace_entries,
        }
    }
    /// Complete original A+B occurrence allowance,including unrelated cut/fold fields.
    #[must_use]
    pub const fn source_occurrences(self) -> usize {
        self.source_occurrences
    }
    /// Complete raw assignment allowance,including invalid/duplicate fields.
    #[must_use]
    pub const fn assignments(self) -> usize {
        self.assignments
    }
    /// Complete2M index/association allowance,not bytes or prior storage.
    #[must_use]
    pub const fn workspace_entries(self) -> usize {
        self.workspace_entries
    }
}
/// First whole resource,owner/count or original source-assignment refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinConstructionSourceRefusal {
    /// Complete original construction-field count cannot be represented.
    SourceCountOverflow,
    /// Checked complete2M cannot be represented.
    WorkspaceOverflow,
    /// Whole original R exceeds policy.
    SourceOccurrences {
        /// Actual whole R.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole raw M exceeds policy.
    Assignments {
        /// Actual whole M.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole2M index/association population exceeds policy.
    Workspace {
        /// Required complete entries.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Raw operation differs from the actual original join operation.
    Operation,
    /// One explicit action per actual construction field is required.
    Count {
        /// Complete actual original construction count.
        expected: usize,
        /// Complete raw action count.
        actual: usize,
    },
    /// One original source occurrence repeats in original assignment order.
    DuplicateSource {
        /// First raw assignment.
        first: usize,
        /// Repeated raw assignment.
        second: usize,
        /// Actual repeated A/B role.
        side: SeamSideId,
        /// Original construction-field location.
        source_index: usize,
    },
    /// No original construction field exists at this snapshot locator.
    MissingSource {
        /// Original raw assignment.
        assignment: usize,
    },
    /// Actual original persistent edge or traversal differs from the raw expected field.
    SourceField {
        /// Original raw assignment.
        assignment: usize,
    },
}
/// Whole raw actions,actual proposed-use/source context,policy and first cause survive refusal.
pub struct JoinConstructionSourceError<'a> {
    manifest: &'a JoinConstructionSourceManifest,
    uses: &'a DeclaredJoinConstructionUses<'a>,
    limits: JoinConstructionSourceLimits,
    refusal: JoinConstructionSourceRefusal,
}
fn sources<'a>(uses: &DeclaredJoinConstructionUses<'a>) -> &'a BoundJoinSourcePieces<'a> {
    uses.connectivity()
        .contours()
        .directions()
        .coverage()
        .edges()
        .identity()
        .roles()
        .banks()
        .sources()
}
fn key(side: SeamSideId, index: usize) -> (bool, usize) {
    (side == SeamSideId::B, index)
}
impl<'a> JoinConstructionSourceError<'a> {
    /// Actual complete original action declaration,including unaccepted suffixes.
    #[must_use]
    pub const fn manifest(&self) -> &'a JoinConstructionSourceManifest {
        self.manifest
    }
    /// Actual complete proposed construction uses/cut-path/original source parents.
    #[must_use]
    pub const fn uses(&self) -> &'a DeclaredJoinConstructionUses<'a> {
        self.uses
    }
    /// Original explicit whole-resource policy.
    #[must_use]
    pub const fn limits(&self) -> JoinConstructionSourceLimits {
        self.limits
    }
    /// First complete resource,owner/count or original source cause.
    #[must_use]
    pub const fn refusal(&self) -> JoinConstructionSourceRefusal {
        self.refusal
    }
    /// Actual first refused assignment;whole-resource/operation/count causes return None.
    #[must_use]
    pub fn refused_assignment(&self) -> Option<&'a JoinConstructionSourceAssignment> {
        use JoinConstructionSourceRefusal as R;
        let index = match self.refusal {
            R::DuplicateSource { second, .. } => second,
            R::MissingSource { assignment } | R::SourceField { assignment } => assignment,
            _ => return None,
        };
        self.manifest.assignments.get(index)
    }
    /// Actual original mismatched field only for SourceField;other causes return None.
    #[must_use]
    pub fn refused_source_field(&self) -> Option<&'a DirectedEdge> {
        let JoinConstructionSourceRefusal::SourceField { .. } = self.refusal else {
            return None;
        };
        let row = self.refused_assignment()?;
        sources(self.uses)
            .piece(row.side)
            .definition()
            .construction_lines
            .get(row.source_index)
    }
}
impl fmt::Debug for JoinConstructionSourceError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JoinConstructionSourceError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for JoinConstructionSourceError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "join original construction actions: {:?}", self.refusal)
    }
}
impl std::error::Error for JoinConstructionSourceError<'_> {}
/// Explicit original operation and ordered complete construction-field action requests.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JoinConstructionSourceManifest {
    operation: EntityId,
    assignments: Vec<JoinConstructionSourceAssignment>,
}
impl JoinConstructionSourceManifest {
    /// Retain exact original operation/order/invalid fields without implicit actions or id issuance.
    #[must_use]
    pub const fn new(
        operation: EntityId,
        assignments: Vec<JoinConstructionSourceAssignment>,
    ) -> Self {
        Self {
            operation,
            assignments,
        }
    }
    /// Actual original declared operation.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        self.operation
    }
    /// Complete original assignments,including duplicate/unknown/mismatched fields.
    #[must_use]
    pub fn assignments(&self) -> &[JoinConstructionSourceAssignment] {
        &self.assignments
    }
    /// Admit whole R,checked original K/raw2M and M/workspace before fields or allocation.
    /// Then operation,exact M=K,and each original row's duplicate source,missing field or expected
    /// EdgeRef/traversal mismatch. Unique actual construction membership plus M=K declares every
    /// original A/B occurrence,including repeated geometry. Snapshot locations alone cannot rebind.
    /// O(M log(M+2)+1),2M new entries;no source/key scan,registry/provider/geometry or evidence call.
    /// Retain requires target mapping later;Omit carries only its original requested evidence id.
    /// Zero/shared requested ids are neither defaults nor accepted evidence/kind/authority facts.
    /// This proves complete action requests only,not target use/whole-range/multiplicity coverage,
    /// justified omission,visible consumer repair,currentness,retirement or published effects.
    /// # Errors
    /// Whole R quota,K/2M arithmetic,M/workspace quotas,operation,count;then original source causes.
    pub fn admit<'a>(
        &'a self,
        uses: &'a DeclaredJoinConstructionUses<'a>,
        limits: JoinConstructionSourceLimits,
    ) -> Result<DeclaredJoinConstructionSources<'a>, JoinConstructionSourceError<'a>> {
        use JoinConstructionSourceRefusal as R;
        let refuse = |refusal| JoinConstructionSourceError {
            manifest: self,
            uses,
            limits,
            refusal,
        };
        let sources = sources(uses);
        let r = sources.occurrence_count();
        let m = self.assignments.len();
        if r > limits.source_occurrences {
            return Err(refuse(R::SourceOccurrences {
                required: r,
                allowed: limits.source_occurrences,
            }));
        }
        let k = sources
            .piece(SeamSideId::A)
            .definition()
            .construction_lines
            .len()
            .checked_add(
                sources
                    .piece(SeamSideId::B)
                    .definition()
                    .construction_lines
                    .len(),
            )
            .ok_or_else(|| refuse(R::SourceCountOverflow))?;
        let workspace = m
            .checked_mul(2)
            .ok_or_else(|| refuse(R::WorkspaceOverflow))?;
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
        if self.operation
            != sources
                .population()
                .metadata()
                .splice()
                .source()
                .edit()
                .operation()
        {
            return Err(refuse(R::Operation));
        }
        if m != k {
            return Err(refuse(R::Count {
                expected: k,
                actual: m,
            }));
        }
        let mut by_source = BTreeMap::new();
        let mut assignments = Vec::with_capacity(m);
        for (index, row) in self.assignments.iter().enumerate() {
            if let Some(first) = by_source.insert(key(row.side, row.source_index), index) {
                return Err(refuse(R::DuplicateSource {
                    first,
                    second: index,
                    side: row.side,
                    source_index: row.source_index,
                }));
            }
            let source = sources
                .piece(row.side)
                .definition()
                .construction_lines
                .get(row.source_index)
                .ok_or_else(|| refuse(R::MissingSource { assignment: index }))?;
            if *source != row.expected {
                return Err(refuse(R::SourceField { assignment: index }));
            }
            assignments.push(SourceAssociation { index, row, source });
        }
        Ok(DeclaredJoinConstructionSources {
            manifest: self,
            uses,
            limits,
            source_count: k,
            workspace,
            by_source,
            assignments,
        })
    }
}
#[derive(Debug)]
struct SourceAssociation<'a> {
    index: usize,
    row: &'a JoinConstructionSourceAssignment,
    source: &'a DirectedEdge,
}
/// Complete explicit source action requests,not retained target maps or accepted omission evidence.
/// ```compile_fail
/// fn forge<'a>(mut actions:sc_core::construction::DeclaredJoinConstructionSources<'a>,other:&'a sc_core::construction::JoinConstructionSourceManifest) {actions.manifest=other;}
/// ```
/// ```compile_fail
/// fn escape(raw:sc_core::construction::JoinConstructionSourceManifest,uses:sc_core::construction::DeclaredJoinConstructionUses<'_>)
///     ->sc_core::construction::DeclaredJoinConstructionSources<'static> {
///     raw.admit(&uses,sc_core::construction::JoinConstructionSourceLimits::new(0,0,0)).unwrap()
/// }
/// ```
pub struct DeclaredJoinConstructionSources<'a> {
    manifest: &'a JoinConstructionSourceManifest,
    uses: &'a DeclaredJoinConstructionUses<'a>,
    limits: JoinConstructionSourceLimits,
    source_count: usize,
    workspace: usize,
    by_source: BTreeMap<(bool, usize), usize>,
    assignments: Vec<SourceAssociation<'a>>,
}
impl fmt::Debug for DeclaredJoinConstructionSources<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeclaredJoinConstructionSources")
            .field("operation", &self.manifest.operation)
            .field("sources", &self.source_count)
            .field("limits", &self.limits)
            .finish_non_exhaustive()
    }
}
impl<'a> DeclaredJoinConstructionSources<'a> {
    /// Actual complete original action proposal.
    #[must_use]
    pub const fn manifest(&self) -> &'a JoinConstructionSourceManifest {
        self.manifest
    }
    /// Actual whole proposed construction/cut-path/original source parents.
    #[must_use]
    pub const fn uses(&self) -> &'a DeclaredJoinConstructionUses<'a> {
        self.uses
    }
    /// Original explicit full-resource policy.
    #[must_use]
    pub const fn limits(&self) -> JoinConstructionSourceLimits {
        self.limits
    }
    /// Complete original A+B construction occurrence count,including repeated geometry.
    #[must_use]
    pub const fn source_count(&self) -> usize {
        self.source_count
    }
    /// Complete new2M source-index/association entries,not bytes or previous storage.
    #[must_use]
    pub const fn workspace_entries(&self) -> usize {
        self.workspace
    }
    /// Authentic assignments in original authored order,and reverse;scopes are not merged.
    pub fn assignments(
        &self,
    ) -> impl ExactSizeIterator<Item = JoinConstructionSourceAction<'_>> + DoubleEndedIterator {
        self.assignments
            .iter()
            .map(move |association| JoinConstructionSourceAction {
                context: self,
                association,
            })
    }
    /// Authentic action for one actual A/B/original snapshot construction location.
    #[must_use]
    pub fn assignment(
        &self,
        side: SeamSideId,
        source_index: usize,
    ) -> Option<JoinConstructionSourceAction<'_>> {
        self.by_source
            .get(&key(side, source_index))
            .and_then(|i| self.assignments.get(*i))
            .map(|association| JoinConstructionSourceAction {
                context: self,
                association,
            })
    }
}
/// One authentic original assignment,source field and action request;not evidence acceptance.
/// ```compile_fail
/// fn forge<'a>(mut action:sc_core::construction::JoinConstructionSourceAction<'a>,other:&'a sc_core::construction::DeclaredJoinConstructionSources<'a>) {action.context=other;}
/// ```
#[derive(Clone, Copy, Debug)]
pub struct JoinConstructionSourceAction<'a> {
    context: &'a DeclaredJoinConstructionSources<'a>,
    association: &'a SourceAssociation<'a>,
}
impl<'a> JoinConstructionSourceAction<'a> {
    /// Actual whole complete action context.
    #[must_use]
    pub const fn context(self) -> &'a DeclaredJoinConstructionSources<'a> {
        self.context
    }
    /// Original assignment ordinal,not a source/geometry identity.
    #[must_use]
    pub const fn index(self) -> usize {
        self.association.index
    }
    /// Actual original raw assignment,including expected source/traversal and requested intent.
    #[must_use]
    pub const fn assignment(self) -> &'a JoinConstructionSourceAssignment {
        self.association.row
    }
    /// Actual original source field,not a copied expected replacement.
    #[must_use]
    pub const fn source_field(self) -> &'a DirectedEdge {
        self.association.source
    }
    /// Actual original source Piece for this explicitly validated A/B role.
    #[must_use]
    pub fn piece(self) -> &'a Piece {
        sources(self.context.uses).piece(self.association.row.side)
    }
    /// Actual selected physical copy for this original A/B source role.
    #[must_use]
    pub fn copy(self) -> &'a CutCopy {
        sources(self.context.uses)
            .population()
            .metadata()
            .splice()
            .source()
            .side(self.association.row.side)
            .copy()
    }
    /// Actual original explicit retention/omission intent field.
    #[must_use]
    pub const fn intent(self) -> &'a JoinConstructionSourceIntent {
        self.association.row.intent()
    }
    /// Actual original requested omission evidence field;Retain returns None,no registry lookup.
    #[must_use]
    pub const fn evidence_source(self) -> Option<&'a EntityId> {
        self.association.row.evidence_source()
    }
}
