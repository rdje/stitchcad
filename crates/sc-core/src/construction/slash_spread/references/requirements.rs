//! Whole held-consumer evidence requests;registration coverage does not accept provider evidence.
use super::BoundSlashSpreadRegistrationPlan;
use crate::ontology::EntityId;

/// Mandatory distinct evidence scopes,including consumers absent from the registration subset.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlashSpreadConsumerEvidenceKind {
    /// Whole affected-source use across actual structural/semantic holders and physical copies.
    WholeSourceConsumers,
    /// Current Design/garment/Piece/frame/liveness and semantic-position ownership.
    CurrentOwnership,
    /// Actual source/output point and interval correspondence,with explicit ambiguous selections.
    Correspondence,
    /// Complete visible gaps/ambiguities/repairs and legitimate coordinated withdrawal evidence.
    VisibleRepairs,
    /// Actual authority for global edits,selected-holder repairs and canonical object replacement.
    AuthoringAuthority,
}
const KINDS: [SlashSpreadConsumerEvidenceKind; 5] = [
    SlashSpreadConsumerEvidenceKind::WholeSourceConsumers,
    SlashSpreadConsumerEvidenceKind::CurrentOwnership,
    SlashSpreadConsumerEvidenceKind::Correspondence,
    SlashSpreadConsumerEvidenceKind::VisibleRepairs,
    SlashSpreadConsumerEvidenceKind::AuthoringAuthority,
];
/// Five explicit provider/source identities to request,not registry-resolved evidence or approval.
/// There is no default,optional scope or automatic evidence acceptance.
/// ```compile_fail
/// let request:sc_core::construction::SlashSpreadConsumerEvidenceSources=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SlashSpreadConsumerEvidenceSources {
    whole_source_consumers: EntityId,
    current_ownership: EntityId,
    correspondence: EntityId,
    visible_repairs: EntityId,
    authoring_authority: EntityId,
}
impl SlashSpreadConsumerEvidenceSources {
    /// Retain all original requested source identities. One provider may serve multiple scopes;
    /// equal identities never collapse those obligations. No source existence is inferred.
    #[must_use]
    pub const fn new(
        whole_source_consumers: EntityId,
        current_ownership: EntityId,
        correspondence: EntityId,
        visible_repairs: EntityId,
        authoring_authority: EntityId,
    ) -> Self {
        Self {
            whole_source_consumers,
            current_ownership,
            correspondence,
            visible_repairs,
            authoring_authority,
        }
    }
    /// Actual original source field for the required scope,without provider lookup or evidence.
    #[must_use]
    pub const fn source(&self, kind: SlashSpreadConsumerEvidenceKind) -> &EntityId {
        match kind {
            SlashSpreadConsumerEvidenceKind::WholeSourceConsumers => &self.whole_source_consumers,
            SlashSpreadConsumerEvidenceKind::CurrentOwnership => &self.current_ownership,
            SlashSpreadConsumerEvidenceKind::Correspondence => &self.correspondence,
            SlashSpreadConsumerEvidenceKind::VisibleRepairs => &self.visible_repairs,
            SlashSpreadConsumerEvidenceKind::AuthoringAuthority => &self.authoring_authority,
        }
    }
}
impl<'s> BoundSlashSpreadRegistrationPlan<'s> {
    /// Require all five whole-consumer evidence scopes for this actual complete registered-source
    /// context,including when it contains zero registrations or all global sources are preserved.
    /// Fixed five-item traversal allocates nothing and does not scan or fold consumers. No evidence,
    /// provider existence,currentness,correspondence,repair discharge or authority is accepted here.
    #[must_use]
    pub fn require_consumer_evidence<'a>(
        &'a self,
        evidence: &'a SlashSpreadConsumerEvidenceSources,
    ) -> RequiredSlashSpreadConsumerEvidence<'a>
    where
        's: 'a,
    {
        RequiredSlashSpreadConsumerEvidence {
            registrations: self,
            evidence,
        }
    }
}
/// Authentic whole context with mandatory evidence-source requests;no execution readiness verdict.
#[derive(Clone, Copy, Debug)]
pub struct RequiredSlashSpreadConsumerEvidence<'a> {
    registrations: &'a BoundSlashSpreadRegistrationPlan<'a>,
    evidence: &'a SlashSpreadConsumerEvidenceSources,
}
impl<'a> RequiredSlashSpreadConsumerEvidence<'a> {
    /// Actual complete registered-source plan,with its original source/contour/ledger owners.
    #[must_use]
    pub const fn registration_plan(&self) -> &'a BoundSlashSpreadRegistrationPlan<'a> {
        self.registrations
    }
    /// Actual original caller-owned evidence-source request.
    #[must_use]
    pub const fn evidence_sources(&self) -> &'a SlashSpreadConsumerEvidenceSources {
        self.evidence
    }
    /// All five distinct obligations in whole-consumer/ownership/correspondence/repair/authority
    /// order. Shared source identities do not merge scopes;zero registrations omit no requirement.
    pub fn requirements(
        &self,
    ) -> impl DoubleEndedIterator<Item = SlashSpreadConsumerEvidenceRequirement<'_>> + ExactSizeIterator
    {
        KINDS
            .into_iter()
            .map(|kind| SlashSpreadConsumerEvidenceRequirement {
                context: self,
                kind,
                source: self.evidence.source(kind),
            })
    }
}
/// Private association of an evidence scope and original requested source with the whole context.
/// ```compile_fail
/// fn forge<'a>(mut item:sc_core::construction::SlashSpreadConsumerEvidenceRequirement<'a>,other:&'a sc_core::construction::RequiredSlashSpreadConsumerEvidence<'a>) {
///     item.context=other;
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct SlashSpreadConsumerEvidenceRequirement<'a> {
    context: &'a RequiredSlashSpreadConsumerEvidence<'a>,
    kind: SlashSpreadConsumerEvidenceKind,
    source: &'a EntityId,
}
impl<'a> SlashSpreadConsumerEvidenceRequirement<'a> {
    /// Actual complete evidence-request and registered-source context.
    #[must_use]
    pub const fn context(self) -> &'a RequiredSlashSpreadConsumerEvidence<'a> {
        self.context
    }
    /// Distinct required evidence scope,not evidence acceptance.
    #[must_use]
    pub const fn kind(self) -> SlashSpreadConsumerEvidenceKind {
        self.kind
    }
    /// Actual original requested source field,not an approved/resolved provider.
    #[must_use]
    pub const fn source(self) -> &'a EntityId {
        self.source
    }
}
