//! Complete explicit original global-edge identity requirements,not live-state or journal effects.
use super::{
    BoundJoinSourcePieces, JoinConsumerEvidenceKind, JoinSourceOccurrence, RequiredJoinConsumers,
};
use crate::ontology::{EdgeRef, EntityId};
use core::fmt;
use std::collections::BTreeMap;

/// Requested fate of the entire original global EdgeRef,not just its selected Piece-local range.
/// ```compile_fail
/// let intent:sc_core::construction::JoinSourceIdentityIntent=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinSourceIdentityIntent {
    /// Require retained global identity;this does not copy geometry or retarget a held consumer.
    RetainOriginal,
    /// Require global retirement with actual history/correspondence and mapped-or-visible repairs.
    /// Stored old references may remain and resolve through the journal;their presence or absence
    /// alone decides neither liveness nor retirement permission. This is no accepted authority.
    RetireOriginal,
}
/// One explicit original global identity selector and nondefault disposition requirement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinSourceIdentityAssignment {
    edge: EdgeRef,
    intent: JoinSourceIdentityIntent,
}
impl JoinSourceIdentityAssignment {
    /// Retain exact original selector/intent fields without alias projection or mutation.
    #[must_use]
    pub const fn new(edge: EdgeRef, intent: JoinSourceIdentityIntent) -> Self {
        Self { edge, intent }
    }
    /// Actual authored literal source selector,not a reconstructed source field.
    #[must_use]
    pub const fn edge(&self) -> &EdgeRef {
        &self.edge
    }
    /// Actual authored global disposition requirement,not a state or effect verdict.
    #[must_use]
    pub const fn intent(&self) -> &JoinSourceIdentityIntent {
        &self.intent
    }
}
/// Whole original A/B occurrences S,raw rows M and checked S+2M new index/association allowances.
/// ```compile_fail
/// let limits:sc_core::construction::JoinSourceIdentityLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinSourceIdentityLimits {
    sources: usize,
    assignments: usize,
    workspace_entries: usize,
}
impl JoinSourceIdentityLimits {
    /// Count every boundary/hole/construction/separate fold occurrence before fields/allocation.
    #[must_use]
    pub const fn new(sources: usize, assignments: usize, workspace_entries: usize) -> Self {
        Self {
            sources,
            assignments,
            workspace_entries,
        }
    }
    /// Every original A/B field occurrence,including repeated global identities.
    #[must_use]
    pub const fn sources(self) -> usize {
        self.sources
    }
    /// Every authored raw disposition row.
    #[must_use]
    pub const fn assignments(self) -> usize {
        self.assignments
    }
    /// Complete conservative S+2M logical entries,not bytes or earlier immutable storage.
    #[must_use]
    pub const fn workspace_entries(self) -> usize {
        self.workspace_entries
    }
}
/// First whole-resource/arithmetic,operation,count or original-row identity cause.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinSourceIdentityRefusal {
    /// Whole original occurrences exceed policy before field inspection.
    Sources {
        /// Actual complete S.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Whole raw rows exceed policy before field inspection.
    Assignments {
        /// Actual complete M.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Complete checked S+2M cannot be represented.
    WorkspaceOverflow,
    /// Whole index/association reservation exceeds policy before fields/allocation.
    Workspace {
        /// Complete required entries.
        required: usize,
        /// Explicit allowance.
        allowed: usize,
    },
    /// Manifest names another join operation.
    Operation,
    /// Exactly one row is required per distinct literal original A/B EdgeRef.
    Count {
        /// Actual distinct source global identities K.
        expected: usize,
        /// Complete authored rows M.
        actual: usize,
    },
    /// Original global source selector repeats.
    Duplicate {
        /// First original row.
        first: usize,
        /// Repeated original row.
        second: usize,
    },
    /// Original raw selector names no literal held original A/B source field.
    MissingSource {
        /// First original row.
        index: usize,
    },
}
fn original_sources<'a>(consumers: &'a RequiredJoinConsumers<'a>) -> &'a BoundJoinSourcePieces<'a> {
    consumers
        .inventory()
        .plan()
        .correspondence()
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
}
/// Actual whole manifest/consumer contexts,original limits and first refusal;no accepted prefix.
pub struct JoinSourceIdentityError<'a> {
    manifest: &'a JoinSourceIdentityManifest,
    consumers: &'a RequiredJoinConsumers<'a>,
    limits: JoinSourceIdentityLimits,
    refusal: JoinSourceIdentityRefusal,
}
impl<'a> JoinSourceIdentityError<'a> {
    /// Actual complete original disposition/history request.
    #[must_use]
    pub const fn manifest(&self) -> &'a JoinSourceIdentityManifest {
        self.manifest
    }
    /// Actual whole original/candidate/copy/repair/consumer requirement context.
    #[must_use]
    pub const fn consumers(&self) -> &'a RequiredJoinConsumers<'a> {
        self.consumers
    }
    /// Original complete-resource policy.
    #[must_use]
    pub const fn limits(&self) -> JoinSourceIdentityLimits {
        self.limits
    }
    /// First whole or original-row cause,not global incompatibility or provider classification.
    #[must_use]
    pub const fn refusal(&self) -> JoinSourceIdentityRefusal {
        self.refusal
    }
    /// Actual refused original raw row;whole resource/operation/count causes return None.
    #[must_use]
    pub fn refused_assignment(&self) -> Option<&'a JoinSourceIdentityAssignment> {
        let index = match self.refusal {
            JoinSourceIdentityRefusal::Duplicate { second, .. } => second,
            JoinSourceIdentityRefusal::MissingSource { index } => index,
            _ => return None,
        };
        self.manifest.assignments.get(index)
    }
    /// First authentic original source occurrence for a known refused selector. This diagnostic
    /// scans the already admitted original H/S fields in O(H+S+1),without allocation or projection.
    #[must_use]
    pub fn first_source(&self) -> Option<JoinSourceOccurrence<'a>> {
        let edge = *self.refused_assignment()?.edge();
        original_sources(self.consumers)
            .occurrences()
            .find(|field| *field.field().edge() == edge)
    }
}
impl fmt::Debug for JoinSourceIdentityError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JoinSourceIdentityError")
            .field("limits", &self.limits)
            .field("refusal", &self.refusal)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for JoinSourceIdentityError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "join original source identities: {:?}", self.refusal)
    }
}
impl std::error::Error for JoinSourceIdentityError<'_> {}

/// Complete operation/global identity-history source and explicit original-edge disposition rows.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JoinSourceIdentityManifest {
    operation: EntityId,
    identity_history: EntityId,
    assignments: Vec<JoinSourceIdentityAssignment>,
}
impl JoinSourceIdentityManifest {
    /// Require history/nonreuse/issuance/copy-entity disposition evidence plus every raw edge intent.
    /// Zero/reused source ids remain ordinary unaccepted requests,not live-state certificates.
    #[must_use]
    pub const fn new(
        operation: EntityId,
        identity_history: EntityId,
        assignments: Vec<JoinSourceIdentityAssignment>,
    ) -> Self {
        Self {
            operation,
            identity_history,
            assignments,
        }
    }
    /// Actual authored operation.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        self.operation
    }
    /// Actual original global identity/history source id field,not accepted or looked up evidence.
    #[must_use]
    pub const fn identity_history(&self) -> &EntityId {
        &self.identity_history
    }
    /// Complete original raw global-edge requirements in authored order.
    #[must_use]
    pub fn assignments(&self) -> &[JoinSourceIdentityAssignment] {
        &self.assignments
    }
    /// Admit whole S/M/checked S+2M before operation,fields or allocation. Index all original A/B
    /// boundary/hole/construction/fold fields by literal global EdgeRef,then require exactly K rows,
    /// each distinct and actually held. Deduplication selects one global fate,never erases source
    /// frames or repeated original fields. Both intents remain requests even when a candidate holds
    /// old references:journal correspondence may resolve them;local absence supplies no permission.
    /// Work O((H+S+M) log(S+M+2)+1),peak S+2M new logical entries,successful2M. Earlier whole
    /// source/candidate/copy/proposal policies stay separate. No provider/registry/geometry query,
    /// currentness,identity issuance,history event,retirement or semantic/registration repair occurs.
    /// # Errors
    /// Whole S/M/arithmetic/workspace,operation,complete distinct-source count,then first original
    /// duplicate or unknown literal source selector. No accepted prefix or history effect escapes.
    pub fn admit<'a>(
        &'a self,
        consumers: &'a RequiredJoinConsumers<'a>,
        limits: JoinSourceIdentityLimits,
    ) -> Result<RequiredJoinSourceIdentities<'a>, JoinSourceIdentityError<'a>> {
        use JoinSourceIdentityRefusal as R;
        let refuse = |refusal| JoinSourceIdentityError {
            manifest: self,
            consumers,
            limits,
            refusal,
        };
        let source = original_sources(consumers);
        let s = source.occurrence_count();
        let m = self.assignments.len();
        if s > limits.sources {
            return Err(refuse(R::Sources {
                required: s,
                allowed: limits.sources,
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
            .and_then(|rows| s.checked_add(rows))
            .ok_or_else(|| refuse(R::WorkspaceOverflow))?;
        if workspace > limits.workspace_entries {
            return Err(refuse(R::Workspace {
                required: workspace,
                allowed: limits.workspace_entries,
            }));
        }
        if self.operation
            != consumers
                .inventory()
                .plan()
                .correspondence()
                .sources()
                .results()
                .manifest()
                .operation()
        {
            return Err(refuse(R::Operation));
        }
        let mut originals = BTreeMap::new();
        for field in source.occurrences() {
            originals.entry(*field.field().edge()).or_insert(field);
        }
        let k = originals.len();
        if m != k {
            return Err(refuse(R::Count {
                expected: k,
                actual: m,
            }));
        }
        let mut by_edge = BTreeMap::new();
        let mut assignments = Vec::with_capacity(m);
        let mut retirements = 0;
        for (index, row) in self.assignments.iter().enumerate() {
            if let Some(first) = by_edge.insert(row.edge, index) {
                return Err(refuse(R::Duplicate {
                    first,
                    second: index,
                }));
            }
            let first_source = *originals
                .get(&row.edge)
                .ok_or_else(|| refuse(R::MissingSource { index }))?;
            if row.intent == JoinSourceIdentityIntent::RetireOriginal {
                retirements += 1;
            }
            assignments.push(SourceIdentityAssociation {
                index,
                row,
                first_source,
            });
        }
        Ok(RequiredJoinSourceIdentities {
            manifest: self,
            consumers,
            source,
            limits,
            workspace,
            retirements,
            by_edge,
            assignments,
        })
    }
}
#[derive(Debug)]
struct SourceIdentityAssociation<'a> {
    index: usize,
    row: &'a JoinSourceIdentityAssignment,
    first_source: JoinSourceOccurrence<'a>,
}
/// Authentic complete global-edge disposition/history requirements,not approved identity effects.
/// ```compile_fail
/// fn forge<'a>(mut proof:sc_core::construction::RequiredJoinSourceIdentities<'a>,other:&'a sc_core::construction::JoinSourceIdentityManifest) {proof.manifest=other;}
/// ```
/// ```compile_fail
/// fn escape(raw:sc_core::construction::JoinSourceIdentityManifest,consumers:sc_core::construction::RequiredJoinConsumers<'_>)->sc_core::construction::RequiredJoinSourceIdentities<'static> {
///     raw.admit(&consumers,sc_core::construction::JoinSourceIdentityLimits::new(10,4,18)).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct RequiredJoinSourceIdentities<'a> {
    manifest: &'a JoinSourceIdentityManifest,
    consumers: &'a RequiredJoinConsumers<'a>,
    source: &'a BoundJoinSourcePieces<'a>,
    limits: JoinSourceIdentityLimits,
    workspace: usize,
    retirements: usize,
    by_edge: BTreeMap<EdgeRef, usize>,
    assignments: Vec<SourceIdentityAssociation<'a>>,
}
impl<'a> RequiredJoinSourceIdentities<'a> {
    /// Actual whole original global identity/history proposal.
    #[must_use]
    pub const fn manifest(&self) -> &'a JoinSourceIdentityManifest {
        self.manifest
    }
    /// Actual whole matched inventory/copy/consumer/repair context.
    #[must_use]
    pub const fn consumers(&self) -> &'a RequiredJoinConsumers<'a> {
        self.consumers
    }
    /// Actual original A/B occurrence context,including every original frame and repeated field.
    #[must_use]
    pub const fn source_pieces(&self) -> &'a BoundJoinSourcePieces<'a> {
        self.source
    }
    /// Original complete-resource policy.
    #[must_use]
    pub const fn limits(&self) -> JoinSourceIdentityLimits {
        self.limits
    }
    /// Conservative S+2M peak new entries,not bytes or earlier storage.
    #[must_use]
    pub const fn workspace_entries(&self) -> usize {
        self.workspace
    }
    /// Complete distinct original global identity count K,one explicit request each.
    #[must_use]
    pub fn source_edge_count(&self) -> usize {
        self.assignments.len()
    }
    /// Explicit retirement requests,not retired entities or approved permissions.
    #[must_use]
    pub const fn retirement_count(&self) -> usize {
        self.retirements
    }
    /// Explicit retention requests,not current global liveness certificates.
    #[must_use]
    pub fn retention_count(&self) -> usize {
        self.assignments.len() - self.retirements
    }
    /// Actual authored global history/nonreuse/issuance and consumed/retained entity evidence request.
    #[must_use]
    pub const fn history_source(&self) -> &'a EntityId {
        self.manifest.identity_history()
    }
    /// Actual mandatory original consumer/registration/semantic repair source field;no duplicate input.
    #[must_use]
    pub fn repair_source(&self) -> &'a EntityId {
        self.consumers
            .requirement(JoinConsumerEvidenceKind::VisibleRepairs)
            .source()
    }
    /// Authentic assignments in original raw order,and reverse.
    pub fn assignments(
        &self,
    ) -> impl ExactSizeIterator<Item = JoinSourceIdentity<'_>> + DoubleEndedIterator {
        self.assignments
            .iter()
            .map(move |association| JoinSourceIdentity {
                context: self,
                association,
            })
    }
    /// Authentic requested disposition by literal original global EdgeRef;no alias projection.
    #[must_use]
    pub fn assignment(&self, edge: EdgeRef) -> Option<JoinSourceIdentity<'_>> {
        self.assignments
            .get(*self.by_edge.get(&edge)?)
            .map(|association| JoinSourceIdentity {
                context: self,
                association,
            })
    }
}
/// One authentic original global edge request,with every original field/frame still inspectable.
#[derive(Clone, Copy, Debug)]
pub struct JoinSourceIdentity<'a> {
    context: &'a RequiredJoinSourceIdentities<'a>,
    association: &'a SourceIdentityAssociation<'a>,
}
impl<'a> JoinSourceIdentity<'a> {
    /// Actual complete original/global identity requirement owner.
    #[must_use]
    pub const fn context(self) -> &'a RequiredJoinSourceIdentities<'a> {
        self.context
    }
    /// Original raw ordinal,not a persistent edge identity.
    #[must_use]
    pub const fn index(self) -> usize {
        self.association.index
    }
    /// Actual original selector/intent proposal.
    #[must_use]
    pub const fn definition(self) -> &'a JoinSourceIdentityAssignment {
        self.association.row
    }
    /// Actual first original source occurrence in A/B and field order.
    #[must_use]
    pub const fn first_source(self) -> JoinSourceOccurrence<'a> {
        self.association.first_source
    }
    /// Actual first held source EdgeRef field,not the detached equal selector.
    #[must_use]
    pub const fn source_edge(self) -> &'a EdgeRef {
        self.association.first_source.field().edge()
    }
    /// Actual original nondefault global disposition request.
    #[must_use]
    pub const fn intent(self) -> &'a JoinSourceIdentityIntent {
        self.association.row.intent()
    }
    /// Every authentic matching original occurrence/frame and reverse,in O(H+S+1),constant space.
    /// One global disposition never collapses distinct A/B frames or repeated construction/fold fields.
    pub fn sources(self) -> impl DoubleEndedIterator<Item = JoinSourceOccurrence<'a>> {
        let edge = *self.source_edge();
        self.context
            .source
            .occurrences()
            .filter(move |field| *field.field().edge() == edge)
    }
}
