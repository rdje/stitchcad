//! Complete mandatory evidence requests over actual walk field proposals;no provider verdicts.
use super::AdmittedWalkNotchFields;
use crate::ontology::EntityId;
/// Distinct mandatory scopes;shared provider identities never merge or discharge obligations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WalkNotchEvidenceKind {
    /// Actual complete current Design/revision,graph,plan,Pieces,ledger and semantic registries.
    CurrentOwnership,
    /// Global issued/nonreused identities,actual private reservation and retained role/entity history.
    IdentityHistory,
    /// Canonical Notch birth/revision,prior holder preservation and complete result semantics.
    CanonicalNotches,
    /// All affected consumers,held references,explicit ambiguity choices,gaps and visible repairs.
    VisibleRepairs,
    /// Actual paired source positions and physical sewing correspondence,not coordinate coincidence.
    Correspondence,
    /// Measured actual source lengths and signed differential under declared numerical error budgets.
    Differential,
    /// Original copy-addressed direction,ease distribution,stop/turn constraints and conservation.
    EaseAndStops,
    /// Actual symbolic style,sample/production dimensions and encoding resolution with evidence.
    Profiles,
    /// Every physical copy of affected pattern Pieces and its semantic/sewing neighbours and effects.
    AllCopyEffects,
    /// Actual authoring authority,selected-holder repair and complete atomic publication/history.
    AuthoringAuthority,
}
const KINDS: [WalkNotchEvidenceKind; 10] = [
    WalkNotchEvidenceKind::CurrentOwnership,
    WalkNotchEvidenceKind::IdentityHistory,
    WalkNotchEvidenceKind::CanonicalNotches,
    WalkNotchEvidenceKind::VisibleRepairs,
    WalkNotchEvidenceKind::Correspondence,
    WalkNotchEvidenceKind::Differential,
    WalkNotchEvidenceKind::EaseAndStops,
    WalkNotchEvidenceKind::Profiles,
    WalkNotchEvidenceKind::AllCopyEffects,
    WalkNotchEvidenceKind::AuthoringAuthority,
];
/// Four explicit current ownership,identity history,canonical Notch and visible-repair sources.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WalkNotchContextSources {
    current_ownership: EntityId,
    identity_history: EntityId,
    canonical_notches: EntityId,
    visible_repairs: EntityId,
}
impl WalkNotchContextSources {
    /// Retain every original requested source identity,without registry lookup or accepted evidence.
    #[must_use]
    pub const fn new(
        current_ownership: EntityId,
        identity_history: EntityId,
        canonical_notches: EntityId,
        visible_repairs: EntityId,
    ) -> Self {
        Self {
            current_ownership,
            identity_history,
            canonical_notches,
            visible_repairs,
        }
    }
}
/// Six explicit correspondence,differential,ease,profile,all-copy and authority evidence sources.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WalkNotchEffectSources {
    correspondence: EntityId,
    differential: EntityId,
    ease_and_stops: EntityId,
    profiles: EntityId,
    all_copy_effects: EntityId,
    authoring_authority: EntityId,
}
impl WalkNotchEffectSources {
    /// Retain every distinct requested scope,even when source values are equal or zero.
    #[must_use]
    pub const fn new(
        correspondence: EntityId,
        differential: EntityId,
        ease_and_stops: EntityId,
        profiles: EntityId,
        all_copy_effects: EntityId,
        authoring_authority: EntityId,
    ) -> Self {
        Self {
            correspondence,
            differential,
            ease_and_stops,
            profiles,
            all_copy_effects,
            authoring_authority,
        }
    }
}
/// All ten mandatory original evidence-source fields,without optional/defaulted scopes.
/// ```compile_fail
/// let policy:sc_core::construction::WalkNotchEvidencePolicy=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WalkNotchEvidencePolicy {
    context: WalkNotchContextSources,
    effects: WalkNotchEffectSources,
}
impl WalkNotchEvidencePolicy {
    /// Require every context and effect source explicitly;this records no provider verdict.
    #[must_use]
    pub const fn new(context: WalkNotchContextSources, effects: WalkNotchEffectSources) -> Self {
        Self { context, effects }
    }
    /// Actual embedded complete four-field context source request.
    #[must_use]
    pub const fn context_sources(&self) -> &WalkNotchContextSources {
        &self.context
    }
    /// Actual embedded complete six-field effect source request.
    #[must_use]
    pub const fn effect_sources(&self) -> &WalkNotchEffectSources {
        &self.effects
    }
    /// Actual original requested source field for a scope,not resolved evidence or authority.
    #[must_use]
    pub const fn source(&self, kind: WalkNotchEvidenceKind) -> &EntityId {
        use WalkNotchEvidenceKind as K;
        match kind {
            K::CurrentOwnership => &self.context.current_ownership,
            K::IdentityHistory => &self.context.identity_history,
            K::CanonicalNotches => &self.context.canonical_notches,
            K::VisibleRepairs => &self.context.visible_repairs,
            K::Correspondence => &self.effects.correspondence,
            K::Differential => &self.effects.differential,
            K::EaseAndStops => &self.effects.ease_and_stops,
            K::Profiles => &self.effects.profiles,
            K::AllCopyEffects => &self.effects.all_copy_effects,
            K::AuthoringAuthority => &self.effects.authoring_authority,
        }
    }
}
impl<'s> AdmittedWalkNotchFields<'s> {
    /// Require all ten scopes on this actual complete field/identity/assignment/source context.
    /// Fixed borrowing and ten-item traversal take constant work/space and allocate nothing.
    /// Source ids remain ordinary requested references,including shared/zero values. No registry
    /// is scanned,no provider is called and no currentness,geometry,effect or authority is accepted.
    #[must_use]
    pub fn require_evidence<'a>(
        &'a self,
        policy: &'a WalkNotchEvidencePolicy,
    ) -> RequiredWalkNotchEvidence<'a>
    where
        's: 'a,
    {
        RequiredWalkNotchEvidence {
            fields: self,
            policy,
        }
    }
}
/// Authentic complete actual walk proposal and all evidence requests,not execution readiness.
/// ```compile_fail
/// fn reparent<'a>(mut required:sc_core::construction::RequiredWalkNotchEvidence<'a>,
///     other:&'a sc_core::construction::AdmittedWalkNotchFields<'a>) {required.fields=other;}
/// ```
/// ```compile_fail
/// fn escape<'a>(fields:&'a sc_core::construction::AdmittedWalkNotchFields<'a>)
///     ->sc_core::construction::RequiredWalkNotchEvidence<'a> {
///     use sc_core::{construction::*,ontology::EntityId};let id=EntityId::from_bits(0);
///     let policy=WalkNotchEvidencePolicy::new(WalkNotchContextSources::new(id,id,id,id),
///         WalkNotchEffectSources::new(id,id,id,id,id,id));fields.require_evidence(&policy)
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct RequiredWalkNotchEvidence<'a> {
    fields: &'a AdmittedWalkNotchFields<'a>,
    policy: &'a WalkNotchEvidencePolicy,
}
impl<'a> RequiredWalkNotchEvidence<'a> {
    /// Actual complete field/identity/attribution/copy/role/source proposal.
    #[must_use]
    pub const fn fields(&self) -> &'a AdmittedWalkNotchFields<'a> {
        self.fields
    }
    /// Actual original complete mandatory source policy.
    #[must_use]
    pub const fn policy(&self) -> &'a WalkNotchEvidencePolicy {
        self.policy
    }
    /// All ten requirements in context/history/canonical/repair/correspondence/differential/ease/
    /// profiles/all-copy/authority order. Repeated providers or fewer result groups skip none.
    pub fn requirements(
        &self,
    ) -> impl ExactSizeIterator<Item = WalkNotchEvidenceRequirement<'_>> + DoubleEndedIterator {
        KINDS.into_iter().map(|kind| WalkNotchEvidenceRequirement {
            context: self,
            kind,
            source: self.policy.source(kind),
        })
    }
}
/// Authentic distinct scope and actual original source field with complete walk proposal context.
/// ```compile_fail
/// fn forge<'a>(mut item:sc_core::construction::WalkNotchEvidenceRequirement<'a>,
///     other:&'a sc_core::construction::RequiredWalkNotchEvidence<'a>) {item.context=other;}
/// ```
#[derive(Clone, Copy, Debug)]
pub struct WalkNotchEvidenceRequirement<'a> {
    context: &'a RequiredWalkNotchEvidence<'a>,
    kind: WalkNotchEvidenceKind,
    source: &'a EntityId,
}
impl<'a> WalkNotchEvidenceRequirement<'a> {
    /// Actual complete walk proposal and evidence-source owner.
    #[must_use]
    pub const fn context(self) -> &'a RequiredWalkNotchEvidence<'a> {
        self.context
    }
    /// Distinct mandatory scope,not accepted evidence.
    #[must_use]
    pub const fn kind(self) -> WalkNotchEvidenceKind {
        self.kind
    }
    /// Actual original requested source field,without provider lookup or approval.
    #[must_use]
    pub const fn source(self) -> &'a EntityId {
        self.source
    }
}
