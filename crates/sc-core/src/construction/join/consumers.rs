//! Explicit original/result semantic consumer evidence requests,not a consumer census or repair verdict.
use super::MatchedJoinPieceInventory;
use crate::ontology::EntityId;

/// Distinct mandatory scopes;equal or zero source ids never merge or discharge obligations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinConsumerEvidenceKind {
    /// Complete current original/affected result structural,mark,grainline,sewing and other consumers,
    /// including shared edges,unselected/retained copies and separately identified companions.
    WholeSourceConsumers,
    /// Every original holder/field/held key reconciles with the explicit copy/field dispositions.
    /// Construction/fold omissions,additional content/new fold axes and companion changes require
    /// actual justification;historical objects/references are not rewritten by this association.
    SourcePreservation,
    /// Actual current Design/revision,graph,plan,Piece/frame/copy/material/semantic owner registries.
    CurrentOwnership,
    /// Complete result marks/grainlines/seams/other semantics have explicit canonical owners and
    /// mappings;no cloning,quantity,orientation or list position automatically creates semantic ids.
    ResultSemantics,
    /// Every held point/range and registration has explicit mapping/ambiguity/gap or visible repair.
    /// Ledger registration repair alone does not replace an immutable semantic holder's old field.
    VisibleRepairs,
}
const KINDS: [JoinConsumerEvidenceKind; 5] = [
    JoinConsumerEvidenceKind::WholeSourceConsumers,
    JoinConsumerEvidenceKind::SourcePreservation,
    JoinConsumerEvidenceKind::CurrentOwnership,
    JoinConsumerEvidenceKind::ResultSemantics,
    JoinConsumerEvidenceKind::VisibleRepairs,
];

/// Five explicit original evidence-source requests;no optional/defaulted scope or accepted verdict.
/// ```compile_fail
/// let sources:sc_core::construction::JoinConsumerSources=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinConsumerSources {
    whole_source_consumers: EntityId,
    source_preservation: EntityId,
    current_ownership: EntityId,
    result_semantics: EntityId,
    visible_repairs: EntityId,
}
impl JoinConsumerSources {
    /// Retain every original source id,including zero/shared ids,without lookup,truth or approval.
    #[must_use]
    pub const fn new(
        whole_source_consumers: EntityId,
        source_preservation: EntityId,
        current_ownership: EntityId,
        result_semantics: EntityId,
        visible_repairs: EntityId,
    ) -> Self {
        Self {
            whole_source_consumers,
            source_preservation,
            current_ownership,
            result_semantics,
            visible_repairs,
        }
    }
    /// Actual original source field for this distinct scope,not a resolved provider/evidence object.
    #[must_use]
    pub const fn source(&self, kind: JoinConsumerEvidenceKind) -> &EntityId {
        match kind {
            JoinConsumerEvidenceKind::WholeSourceConsumers => &self.whole_source_consumers,
            JoinConsumerEvidenceKind::SourcePreservation => &self.source_preservation,
            JoinConsumerEvidenceKind::CurrentOwnership => &self.current_ownership,
            JoinConsumerEvidenceKind::ResultSemantics => &self.result_semantics,
            JoinConsumerEvidenceKind::VisibleRepairs => &self.visible_repairs,
        }
    }
}
impl<'s> MatchedJoinPieceInventory<'s> {
    /// Require every semantic consumer scope on these actual complete candidate/original/copy/repair
    /// parents. Fixed borrowed association/five-item traversal takes constant work/space,allocates
    /// nothing and scans no registry,consumer,provider,text or geometry body. Earlier complete field/
    /// proposal/copy resources remain separate. Neither empty registries nor shared ids imply absence.
    /// Original held references/owners remain unchanged;mapping/visible repair truth,currentness,
    /// canonical intake/sewing/copy effects,global identity and atomic authority remain execution and
    /// later explicit effect/lineage requirements. This association approves no omission or new axis.
    #[must_use]
    pub fn require_consumers<'a>(
        &'a self,
        sources: &'a JoinConsumerSources,
    ) -> RequiredJoinConsumers<'a>
    where
        's: 'a,
    {
        RequiredJoinConsumers {
            inventory: self,
            sources,
        }
    }
}
/// Authentic complete inventory/source request association,not accepted consumer coverage or repair.
/// ```compile_fail
/// fn forge<'a>(mut proof:sc_core::construction::RequiredJoinConsumers<'a>,other:&'a sc_core::construction::MatchedJoinPieceInventory<'a>) {proof.inventory=other;}
/// ```
/// ```compile_fail
/// fn escape<'a>(inventory:&'a sc_core::construction::MatchedJoinPieceInventory<'a>)->sc_core::construction::RequiredJoinConsumers<'a> {
///     use sc_core::{construction::*,ontology::EntityId};let id=EntityId::from_bits(0);
///     let sources=JoinConsumerSources::new(id,id,id,id,id);inventory.require_consumers(&sources)
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct RequiredJoinConsumers<'a> {
    inventory: &'a MatchedJoinPieceInventory<'a>,
    sources: &'a JoinConsumerSources,
}
impl<'a> RequiredJoinConsumers<'a> {
    /// Actual whole matched candidate/source fields,copies,omissions,companions and evidence requests.
    #[must_use]
    pub const fn inventory(&self) -> &'a MatchedJoinPieceInventory<'a> {
        self.inventory
    }
    /// Actual whole original five-field source request.
    #[must_use]
    pub const fn sources(&self) -> &'a JoinConsumerSources {
        self.sources
    }
    /// All five mandatory scopes in canonical whole-consumer,preservation,current-owner,result,
    /// visible-repair order,and reverse. No id-based deduplication or inferred empty-consumer skip.
    pub fn requirements(
        &self,
    ) -> impl DoubleEndedIterator<Item = JoinConsumerRequirement<'_>> + ExactSizeIterator {
        KINDS.into_iter().map(|kind| self.requirement(kind))
    }
    /// Authentic distinct obligation and actual original source field for any mandatory kind.
    #[must_use]
    pub fn requirement(&self, kind: JoinConsumerEvidenceKind) -> JoinConsumerRequirement<'_> {
        JoinConsumerRequirement {
            context: self,
            kind,
            source: self.sources.source(kind),
        }
    }
}
/// One authentic mandatory scope/source-field loan with its whole original/candidate context.
/// ```compile_fail
/// fn forge<'a>(mut item:sc_core::construction::JoinConsumerRequirement<'a>,other:&'a sc_core::construction::RequiredJoinConsumers<'a>) {item.context=other;}
/// ```
#[derive(Clone, Copy, Debug)]
pub struct JoinConsumerRequirement<'a> {
    context: &'a RequiredJoinConsumers<'a>,
    kind: JoinConsumerEvidenceKind,
    source: &'a EntityId,
}
impl<'a> JoinConsumerRequirement<'a> {
    /// Actual complete original/candidate inventory and explicit source request owner.
    #[must_use]
    pub const fn context(self) -> &'a RequiredJoinConsumers<'a> {
        self.context
    }
    /// Distinct unresolved obligation,not accepted evidence or a provider classification.
    #[must_use]
    pub const fn kind(self) -> JoinConsumerEvidenceKind {
        self.kind
    }
    /// Actual original requested source id field,including zero/reused values without normalization.
    #[must_use]
    pub const fn source(self) -> &'a EntityId {
        self.source
    }
}
