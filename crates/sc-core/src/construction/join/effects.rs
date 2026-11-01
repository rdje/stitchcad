//! Explicit complete intake/sewing/copy/authoring effect requests,not accepted geometry or effects.
use super::{
    RequiredJoinCopyCorrespondence, RequiredJoinExactSplice, RequiredJoinSourceIdentities,
};
use crate::ontology::EntityId;

/// Four mandatory effect scopes;shared source ids never merge or discharge obligations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinEffectEvidenceKind {
    /// Whole canonical Dart/Tuck/Pleat/Gather closing/intake provenance and conservation;joining
    /// cannot duplicate/rebase source amounts or silently withdraw canonical consumers.
    CanonicalIntakes,
    /// Actual copy-addressed spans/directions,original full/portion ease,all stops and neighbours.
    /// Consumed copies never automatically reassign sewing/marks or copy declared ease twice.
    Sewing,
    /// Every retained/consumed/requested/companion/otherwise affected copy in its actual frame.
    /// Each explicit A/B pair needs the original exact-splice domain:actual zero allocated ease,
    /// declared pose/parity,whole-bank coincidence and valid complete topology/error evidence.
    /// Selected-span evidence cannot certify another pair or fabricate an available result.
    AllCopyEffects,
    /// Actual global/canonical authoring authority and complete atomic history/repair/publication,
    /// replay and undo obligations. A requested source id supplies no permission or human approval.
    AuthoringAuthority,
}
const KINDS: [JoinEffectEvidenceKind; 4] = [
    JoinEffectEvidenceKind::CanonicalIntakes,
    JoinEffectEvidenceKind::Sewing,
    JoinEffectEvidenceKind::AllCopyEffects,
    JoinEffectEvidenceKind::AuthoringAuthority,
];
/// Every original effect-source request is explicit,with no optional/defaulted evidence scope.
/// ```compile_fail
/// let sources:sc_core::construction::JoinEffectSources=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinEffectSources {
    canonical_intakes: EntityId,
    sewing: EntityId,
    all_copy_effects: EntityId,
    authoring_authority: EntityId,
}
impl JoinEffectSources {
    /// Retain all actual authored source ids,including zero/shared values,without lookup or truth.
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
    /// Actual original field for this distinct mandatory scope,not a provider/evidence verdict.
    #[must_use]
    pub const fn source(&self, kind: JoinEffectEvidenceKind) -> &EntityId {
        match kind {
            JoinEffectEvidenceKind::CanonicalIntakes => &self.canonical_intakes,
            JoinEffectEvidenceKind::Sewing => &self.sewing,
            JoinEffectEvidenceKind::AllCopyEffects => &self.all_copy_effects,
            JoinEffectEvidenceKind::AuthoringAuthority => &self.authoring_authority,
        }
    }
}
impl<'s> RequiredJoinSourceIdentities<'s> {
    /// Bind every effect request to the actual complete identity/consumer/copy/field/splice parents.
    /// Fixed association/four-item traversal has constant work/space,no allocation,quota,registry,
    /// provider,text or geometry scan. Original history,global edge dispositions,five consumer scopes,
    /// omissions,companions and source frames remain explicit unresolved requirements.
    /// Actual geometry/currentness/effect/repair/authority truth must come from execution. Unknown,
    /// unsupported,uncertified,resource and cancel causes cannot become invented geometry or proof
    /// of incompatibility. No amount,pose,copy identity,semantic holder or ledger is changed here.
    #[must_use]
    pub fn require_effects<'a>(&'a self, sources: &'a JoinEffectSources) -> RequiredJoinEffects<'a>
    where
        's: 'a,
    {
        RequiredJoinEffects {
            identities: self,
            sources,
        }
    }
}
/// Authentic complete original/final requirement context,not execution or publication readiness.
/// ```compile_fail
/// fn forge<'a>(mut required:sc_core::construction::RequiredJoinEffects<'a>,other:&'a sc_core::construction::RequiredJoinSourceIdentities<'a>) {required.identities=other;}
/// ```
/// ```compile_fail
/// fn escape<'a>(identities:&'a sc_core::construction::RequiredJoinSourceIdentities<'a>)->sc_core::construction::RequiredJoinEffects<'a> {
///     use sc_core::{construction::*,ontology::EntityId};let id=EntityId::from_bits(0);
///     let sources=JoinEffectSources::new(id,id,id,id);identities.require_effects(&sources)
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct RequiredJoinEffects<'a> {
    identities: &'a RequiredJoinSourceIdentities<'a>,
    sources: &'a JoinEffectSources,
}
impl<'a> RequiredJoinEffects<'a> {
    /// Actual whole original global-edge/history/consumer/inventory requirements.
    #[must_use]
    pub const fn identities(&self) -> &'a RequiredJoinSourceIdentities<'a> {
        self.identities
    }
    /// Actual whole original four-field effect-source request.
    #[must_use]
    pub const fn sources(&self) -> &'a JoinEffectSources {
        self.sources
    }
    /// Actual complete requested physical-copy A/B pairing,not selected-span evidence reused as proof.
    #[must_use]
    pub fn correspondence(&self) -> &'a RequiredJoinCopyCorrespondence<'a> {
        self.identities
            .consumers()
            .inventory()
            .plan()
            .correspondence()
    }
    /// Actual original placement/endpoint/ease/exact-bank requirement,not an evaluated transform.
    #[must_use]
    pub fn splice(&self) -> &'a RequiredJoinExactSplice<'a> {
        self.identities
            .source_pieces()
            .population()
            .metadata()
            .splice()
    }
    /// All four mandatory original scopes in canonical intake,sewing,all-copy,authoring order and
    /// reverse. No empty-result,shared-id or requested-retention branch skips an obligation.
    pub fn requirements(
        &self,
    ) -> impl DoubleEndedIterator<Item = JoinEffectRequirement<'_>> + ExactSizeIterator {
        KINDS.into_iter().map(|kind| self.requirement(kind))
    }
    /// Authentic distinct original obligation and original source-field loan.
    #[must_use]
    pub fn requirement(&self, kind: JoinEffectEvidenceKind) -> JoinEffectRequirement<'_> {
        JoinEffectRequirement {
            context: self,
            kind,
            source: self.sources.source(kind),
        }
    }
}
/// One authentic effect scope/source-field loan with the whole original/final native context.
/// ```compile_fail
/// fn forge<'a>(mut item:sc_core::construction::JoinEffectRequirement<'a>,other:&'a sc_core::construction::RequiredJoinEffects<'a>) {item.context=other;}
/// ```
#[derive(Clone, Copy, Debug)]
pub struct JoinEffectRequirement<'a> {
    context: &'a RequiredJoinEffects<'a>,
    kind: JoinEffectEvidenceKind,
    source: &'a EntityId,
}
impl<'a> JoinEffectRequirement<'a> {
    /// Actual whole original effect/identity/consumer/copy/splice requirement owner.
    #[must_use]
    pub const fn context(self) -> &'a RequiredJoinEffects<'a> {
        self.context
    }
    /// Distinct mandatory scope,not accepted evidence or an availability classification.
    #[must_use]
    pub const fn kind(self) -> JoinEffectEvidenceKind {
        self.kind
    }
    /// Actual authored source id field,not a lookup,copy or accepted authority record.
    #[must_use]
    pub const fn source(self) -> &'a EntityId {
        self.source
    }
}
