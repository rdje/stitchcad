//! Complete mandatory evidence requests over actual truing locks;no provider verdicts.
use super::RequiredTruingLocks;
use crate::ontology::EntityId;

/// Distinct mandatory scopes;shared source ids never merge or discharge obligations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TruingEvidenceKind {
    /// Complete current Design/revision,graph,plan,Pieces,ledger and semantic/source registries.
    CurrentOwnership,
    /// Global issued/nonreused identities,actual reservations and retained role/entity history.
    IdentityHistory,
    /// All held references,explicit ambiguity choices,gaps,retirement and visible semantic repairs.
    VisibleRepairs,
    /// Deterministic curve/refit selection and its actual supported domain;locks alone select none.
    DeterministicCurve,
    /// Achieved explicit preservation/refit over the complete Piece,including outside source portions.
    Outcome,
    /// Actual curve lengths and achieved fit under the original sourced T2/error budget.
    LengthFit,
    /// Signed A-minus-B differential,original allocated ease and every unexplained residual.
    DifferentialAndEase,
    /// Both original selected endpoint positions and every original span-stop correspondence.
    EndpointAndStopCorrespondence,
    /// Actual profile/material/source truth,requirement derivation and any explicit approved loosening.
    Profiles,
    /// Every physical copy,shared-Piece counterpart and semantic/sewing observer and its effects.
    AllCopyEffects,
    /// Actual authoring authority and complete atomic repair/publication/replay/history.
    AuthoringAuthority,
}
const KINDS: [TruingEvidenceKind; 11] = [
    TruingEvidenceKind::CurrentOwnership,
    TruingEvidenceKind::IdentityHistory,
    TruingEvidenceKind::VisibleRepairs,
    TruingEvidenceKind::DeterministicCurve,
    TruingEvidenceKind::Outcome,
    TruingEvidenceKind::LengthFit,
    TruingEvidenceKind::DifferentialAndEase,
    TruingEvidenceKind::EndpointAndStopCorrespondence,
    TruingEvidenceKind::Profiles,
    TruingEvidenceKind::AllCopyEffects,
    TruingEvidenceKind::AuthoringAuthority,
];

/// Three explicit current ownership,identity history and visible-repair source requests.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TruingContextSources {
    current_ownership: EntityId,
    identity_history: EntityId,
    visible_repairs: EntityId,
}
impl TruingContextSources {
    /// Retain every original requested source id,without lookup or accepted evidence.
    #[must_use]
    pub const fn new(
        current_ownership: EntityId,
        identity_history: EntityId,
        visible_repairs: EntityId,
    ) -> Self {
        Self {
            current_ownership,
            identity_history,
            visible_repairs,
        }
    }
}
/// Four explicit curve-domain,outcome,length-fit and differential/ease source requests.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TruingFitSources {
    deterministic_curve: EntityId,
    outcome: EntityId,
    length_fit: EntityId,
    differential_and_ease: EntityId,
}
impl TruingFitSources {
    /// Retain original requested source ids;no threshold,target length or curve is substituted.
    #[must_use]
    pub const fn new(
        deterministic_curve: EntityId,
        outcome: EntityId,
        length_fit: EntityId,
        differential_and_ease: EntityId,
    ) -> Self {
        Self {
            deterministic_curve,
            outcome,
            length_fit,
            differential_and_ease,
        }
    }
}
/// Four explicit endpoint/stop,profile,all-copy and authoring/atomic-history source requests.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TruingEffectSources {
    endpoint_and_stop_correspondence: EntityId,
    profiles: EntityId,
    all_copy_effects: EntityId,
    authoring_authority: EntityId,
}
impl TruingEffectSources {
    /// Require every scope explicitly,even when ids are equal or zero;these are unresolved requests.
    #[must_use]
    pub const fn new(
        endpoint_and_stop_correspondence: EntityId,
        profiles: EntityId,
        all_copy_effects: EntityId,
        authoring_authority: EntityId,
    ) -> Self {
        Self {
            endpoint_and_stop_correspondence,
            profiles,
            all_copy_effects,
            authoring_authority,
        }
    }
}
/// All mandatory original evidence-source fields;no optional/defaulted scopes or verdicts.
/// ```compile_fail
/// let policy:sc_core::construction::TruingEvidencePolicy=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TruingEvidencePolicy {
    context: TruingContextSources,
    fit: TruingFitSources,
    effects: TruingEffectSources,
}
impl TruingEvidencePolicy {
    /// Retain every requested context,fit and effect source;this accepts no evidence or authority.
    #[must_use]
    pub const fn new(
        context: TruingContextSources,
        fit: TruingFitSources,
        effects: TruingEffectSources,
    ) -> Self {
        Self {
            context,
            fit,
            effects,
        }
    }
    /// Actual embedded context source request.
    #[must_use]
    pub const fn context_sources(&self) -> &TruingContextSources {
        &self.context
    }
    /// Actual embedded curve/outcome/fit/differential source request.
    #[must_use]
    pub const fn fit_sources(&self) -> &TruingFitSources {
        &self.fit
    }
    /// Actual embedded endpoint/stop/profile/copy/authority source request.
    #[must_use]
    pub const fn effect_sources(&self) -> &TruingEffectSources {
        &self.effects
    }
    /// Actual original requested source field for a scope,not resolved evidence or authority.
    #[must_use]
    pub const fn source(&self, kind: TruingEvidenceKind) -> &EntityId {
        use TruingEvidenceKind as K;
        match kind {
            K::CurrentOwnership => &self.context.current_ownership,
            K::IdentityHistory => &self.context.identity_history,
            K::VisibleRepairs => &self.context.visible_repairs,
            K::DeterministicCurve => &self.fit.deterministic_curve,
            K::Outcome => &self.fit.outcome,
            K::LengthFit => &self.fit.length_fit,
            K::DifferentialAndEase => &self.fit.differential_and_ease,
            K::EndpointAndStopCorrespondence => &self.effects.endpoint_and_stop_correspondence,
            K::Profiles => &self.effects.profiles,
            K::AllCopyEffects => &self.effects.all_copy_effects,
            K::AuthoringAuthority => &self.effects.authoring_authority,
        }
    }
}
impl<'s> RequiredTruingLocks<'s> {
    /// Require every evidence scope over the actual complete bounded locks/outcome/source parents.
    /// Fixed borrowing/traversal take constant work/space with no allocation or source/body scans.
    /// Both explicit outcomes require all scopes,including empty preservation/shared providers.
    /// No provider is called,no source id is resolved and no mapping,fit,currentness or authority
    /// is accepted. Actual providers retain whole input/resource/identity/verification obligations.
    #[must_use]
    pub fn require_evidence<'a>(
        &'a self,
        policy: &'a TruingEvidencePolicy,
    ) -> RequiredTruingEvidence<'a>
    where
        's: 'a,
    {
        RequiredTruingEvidence {
            locks: self,
            policy,
        }
    }
}
/// Authentic complete original lock proposal and evidence requests,not execution readiness.
/// ```compile_fail
/// fn forge<'a>(mut required:sc_core::construction::RequiredTruingEvidence<'a>,
///     other:&'a sc_core::construction::RequiredTruingLocks<'a>) {required.locks=other;}
/// ```
/// ```compile_fail
/// fn escape<'a>(locks:&'a sc_core::construction::RequiredTruingLocks<'a>)
///     ->sc_core::construction::RequiredTruingEvidence<'a> {
///     use sc_core::{construction::*,ontology::EntityId};let id=EntityId::from_bits(0);
///     let policy=TruingEvidencePolicy::new(TruingContextSources::new(id,id,id),
///         TruingFitSources::new(id,id,id,id),TruingEffectSources::new(id,id,id,id));
///     locks.require_evidence(&policy)
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct RequiredTruingEvidence<'a> {
    locks: &'a RequiredTruingLocks<'a>,
    policy: &'a TruingEvidencePolicy,
}
impl<'a> RequiredTruingEvidence<'a> {
    /// Actual complete bounded lock/outcome/membership/candidate/role/copy/Piece/source proposal.
    #[must_use]
    pub const fn locks(&self) -> &'a RequiredTruingLocks<'a> {
        self.locks
    }
    /// Actual original complete mandatory source policy.
    #[must_use]
    pub const fn policy(&self) -> &'a TruingEvidencePolicy {
        self.policy
    }
    /// Every scope in context,fit,effect order;shared ids and zero new edges skip none.
    pub fn requirements(
        &self,
    ) -> impl ExactSizeIterator<Item = TruingEvidenceRequirement<'_>> + DoubleEndedIterator {
        KINDS.into_iter().map(|kind| TruingEvidenceRequirement {
            context: self,
            kind,
            source: self.policy.source(kind),
        })
    }
}
/// Authentic distinct scope and actual original source field with complete truing proposal context.
/// ```compile_fail
/// fn forge<'a>(mut item:sc_core::construction::TruingEvidenceRequirement<'a>,
///     other:&'a sc_core::construction::RequiredTruingEvidence<'a>) {item.context=other;}
/// ```
#[derive(Clone, Copy, Debug)]
pub struct TruingEvidenceRequirement<'a> {
    context: &'a RequiredTruingEvidence<'a>,
    kind: TruingEvidenceKind,
    source: &'a EntityId,
}
impl<'a> TruingEvidenceRequirement<'a> {
    /// Actual complete truing proposal and original evidence-source owner.
    #[must_use]
    pub const fn context(self) -> &'a RequiredTruingEvidence<'a> {
        self.context
    }
    /// Distinct mandatory scope,not an accepted proof or authority.
    #[must_use]
    pub const fn kind(self) -> TruingEvidenceKind {
        self.kind
    }
    /// Actual original requested source field,without provider lookup or approval.
    #[must_use]
    pub const fn source(self) -> &'a EntityId {
        self.source
    }
}
