//! Explicit complete mirror semantic/sewing evidence requests,not accepted provider effects.
use super::{BoundMirrorPieceCopyPlan, RequiredMirrorPieceCorrespondence};
use crate::ontology::EntityId;

/// Explicit result semantic intent;source holders retain their original canonical owners in both modes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MirrorPieceSemanticIntent {
    /// Request no result semantic copies or new sewing relations from this operation. Evidence must
    /// justify the complete result and reconcile any existing result consumers;absence is not inferred.
    SourceOnly,
    /// Require explicit complete result semantic/mark/reference/sewing mappings. Source object ids,
    /// held references and physical-copy neighbours cannot be reassigned by copying or list position.
    ExplicitResultMappings,
}
/// Mandatory distinct original evidence scopes;equal provider ids never merge obligations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MirrorPieceSemanticEvidenceKind {
    /// Complete current structural/semantic/reference consumers of source and affected result objects.
    WholeSourceConsumers,
    /// Original canonical source holders,fields,held references and ownership remain unchanged.
    SourcePreservation,
    /// Actual current Design/revision,Piece/frame/copy/material ownership and identity history.
    CurrentOwnership,
    /// Complete result semantics match the explicit intent;new objects use distinct globally issued ids.
    ResultSemantics,
    /// Explicit held point/interval mapping,ambiguous choices,gaps and visible repairs without retargeting.
    VisibleRepairs,
    /// Whole canonical Dart/Tuck/Pleat/Gather intake/closing provenance and conservation obligations.
    CanonicalIntakes,
    /// Actual copy-addressed spans,directions,ease,stops and neighbours reconcile without automatic copying.
    Sewing,
    /// Every retained/retired/proposed and affected companion copy's semantic/physical effects.
    AllCopyEffects,
    /// Actual authority for global/canonical changes,selected-holder repair and atomic publication.
    AuthoringAuthority,
}
const KINDS: [MirrorPieceSemanticEvidenceKind; 9] = [
    MirrorPieceSemanticEvidenceKind::WholeSourceConsumers,
    MirrorPieceSemanticEvidenceKind::SourcePreservation,
    MirrorPieceSemanticEvidenceKind::CurrentOwnership,
    MirrorPieceSemanticEvidenceKind::ResultSemantics,
    MirrorPieceSemanticEvidenceKind::VisibleRepairs,
    MirrorPieceSemanticEvidenceKind::CanonicalIntakes,
    MirrorPieceSemanticEvidenceKind::Sewing,
    MirrorPieceSemanticEvidenceKind::AllCopyEffects,
    MirrorPieceSemanticEvidenceKind::AuthoringAuthority,
];
/// Five explicit whole-consumer/owner/preservation/result/repair evidence-source fields.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MirrorPieceSemanticConsumerSources {
    whole_source_consumers: EntityId,
    source_preservation: EntityId,
    current_ownership: EntityId,
    result_semantics: EntityId,
    visible_repairs: EntityId,
}
impl MirrorPieceSemanticConsumerSources {
    /// Retain all five explicit requested source identities. Zero/shared values remain ordinary
    /// requests;no registry existence,accepted evidence or human approval follows.
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
}
/// Four explicit canonical-intake,sewing,all-copy-effect and authoring-authority source fields.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MirrorPieceSemanticEffectSources {
    canonical_intakes: EntityId,
    sewing: EntityId,
    all_copy_effects: EntityId,
    authoring_authority: EntityId,
}
impl MirrorPieceSemanticEffectSources {
    /// Retain every distinct mandatory scope even when one requested provider serves them all.
    #[must_use]
    pub const fn new(
        canonical_intakes: EntityId,
        sewing: EntityId,
        all_copy_effects: EntityId,
        authoring_authority: EntityId,
    ) -> Self {
        Self {
            canonical_intakes,
            sewing,
            all_copy_effects,
            authoring_authority,
        }
    }
}
/// Explicit semantic intent and nine mandatory source requests;no copied amounts or default policy.
/// ```compile_fail
/// let policy:sc_core::construction::MirrorPieceSemanticPolicy=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MirrorPieceSemanticPolicy {
    intent: MirrorPieceSemanticIntent,
    consumers: MirrorPieceSemanticConsumerSources,
    effects: MirrorPieceSemanticEffectSources,
}
impl MirrorPieceSemanticPolicy {
    /// Require explicit result intent and every original semantic/sewing evidence source.
    #[must_use]
    pub const fn new(
        intent: MirrorPieceSemanticIntent,
        consumers: MirrorPieceSemanticConsumerSources,
        effects: MirrorPieceSemanticEffectSources,
    ) -> Self {
        Self {
            intent,
            consumers,
            effects,
        }
    }
    /// Actual original caller-authored intent,not an inferred empty-consumer policy.
    #[must_use]
    pub const fn intent(&self) -> &MirrorPieceSemanticIntent {
        &self.intent
    }
    /// Actual embedded five-field whole-consumer source request.
    #[must_use]
    pub const fn consumer_sources(&self) -> &MirrorPieceSemanticConsumerSources {
        &self.consumers
    }
    /// Actual embedded four-field semantic/sewing effect source request.
    #[must_use]
    pub const fn effect_sources(&self) -> &MirrorPieceSemanticEffectSources {
        &self.effects
    }
    /// Actual original source field for the named scope,not a resolved provider or accepted evidence.
    #[must_use]
    pub const fn source(&self, kind: MirrorPieceSemanticEvidenceKind) -> &EntityId {
        use MirrorPieceSemanticEvidenceKind as K;
        match kind {
            K::WholeSourceConsumers => &self.consumers.whole_source_consumers,
            K::SourcePreservation => &self.consumers.source_preservation,
            K::CurrentOwnership => &self.consumers.current_ownership,
            K::ResultSemantics => &self.consumers.result_semantics,
            K::VisibleRepairs => &self.consumers.visible_repairs,
            K::CanonicalIntakes => &self.effects.canonical_intakes,
            K::Sewing => &self.effects.sewing,
            K::AllCopyEffects => &self.effects.all_copy_effects,
            K::AuthoringAuthority => &self.effects.authoring_authority,
        }
    }
}
impl<'s> BoundMirrorPieceCopyPlan<'s> {
    /// Require all semantic/sewing scopes on this actual complete candidate context. Fixed borrowed
    /// association and nine-item traversal allocate nothing,scan no registries/consumers and take
    /// constant work/space. Neither SourceOnly nor shared/zero provider ids omit an obligation.
    /// Source canonical holders remain unchanged;actual result mappings,currentness,global issuance,
    /// visible repair,physical/copy effects,geometry and authority must be established by execution.
    #[must_use]
    pub fn require_semantics<'a>(
        &'a self,
        policy: &'a MirrorPieceSemanticPolicy,
    ) -> RequiredMirrorPieceSemantics<'a>
    where
        's: 'a,
    {
        RequiredMirrorPieceSemantics { plan: self, policy }
    }
}
/// Authentic complete plan/policy/correspondence association,not semantic execution readiness.
/// ```compile_fail
/// fn reparent<'a>(mut required:sc_core::construction::RequiredMirrorPieceSemantics<'a>,
///     other:&'a sc_core::construction::BoundMirrorPieceCopyPlan<'a>) {required.plan=other;}
/// ```
/// ```compile_fail
/// fn escape<'a>(plan:&'a sc_core::construction::BoundMirrorPieceCopyPlan<'a>)
///     ->sc_core::construction::RequiredMirrorPieceSemantics<'a> {
///     use sc_core::{construction::*,ontology::EntityId};let id=EntityId::from_bits(0);
///     let policy=MirrorPieceSemanticPolicy::new(MirrorPieceSemanticIntent::SourceOnly,
///         MirrorPieceSemanticConsumerSources::new(id,id,id,id,id),
///         MirrorPieceSemanticEffectSources::new(id,id,id,id));plan.require_semantics(&policy)
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct RequiredMirrorPieceSemantics<'a> {
    plan: &'a BoundMirrorPieceCopyPlan<'a>,
    policy: &'a MirrorPieceSemanticPolicy,
}
impl<'a> RequiredMirrorPieceSemantics<'a> {
    /// Actual complete candidate plan/inventory/native source/result/copy context.
    #[must_use]
    pub const fn plan(&self) -> &'a BoundMirrorPieceCopyPlan<'a> {
        self.plan
    }
    /// Actual original complete semantic policy and source fields.
    #[must_use]
    pub const fn policy(&self) -> &'a MirrorPieceSemanticPolicy {
        self.policy
    }
    /// Actual original semantic result intent,without duplicate policy authority.
    #[must_use]
    pub const fn intent(&self) -> &'a MirrorPieceSemanticIntent {
        self.policy.intent()
    }
    /// Original actual edge/point/axis/frame/contour/accuracy correspondence request remains mandatory.
    #[must_use]
    pub fn correspondence(&self) -> &'a RequiredMirrorPieceCorrespondence<'a> {
        self.plan
            .inventory()
            .copies()
            .identities()
            .source()
            .context()
    }
    /// All nine original obligations in canonical consumer,preservation,ownership,result,repair,
    /// intakes,sewing,all-copy,authority order. Empty registries or repeated provider ids skip none.
    pub fn requirements(
        &self,
    ) -> impl DoubleEndedIterator<Item = MirrorPieceSemanticEvidenceRequirement<'_>> + ExactSizeIterator
    {
        KINDS
            .into_iter()
            .map(|kind| MirrorPieceSemanticEvidenceRequirement {
                context: self,
                kind,
                source: self.policy.source(kind),
            })
    }
}
/// Authentic scope/original-source field with the actual complete semantic context.
/// ```compile_fail
/// fn forge<'a>(mut item:sc_core::construction::MirrorPieceSemanticEvidenceRequirement<'a>,
///     other:&'a sc_core::construction::RequiredMirrorPieceSemantics<'a>) {item.context=other;}
/// ```
#[derive(Clone, Copy, Debug)]
pub struct MirrorPieceSemanticEvidenceRequirement<'a> {
    context: &'a RequiredMirrorPieceSemantics<'a>,
    kind: MirrorPieceSemanticEvidenceKind,
    source: &'a EntityId,
}
impl<'a> MirrorPieceSemanticEvidenceRequirement<'a> {
    /// Actual complete semantic policy and candidate/source/correspondence context.
    #[must_use]
    pub const fn context(self) -> &'a RequiredMirrorPieceSemantics<'a> {
        self.context
    }
    /// Distinct required obligation,not accepted evidence.
    #[must_use]
    pub const fn kind(self) -> MirrorPieceSemanticEvidenceKind {
        self.kind
    }
    /// Actual original requested source field,without registry lookup or approval.
    #[must_use]
    pub const fn source(self) -> &'a EntityId {
        self.source
    }
}
