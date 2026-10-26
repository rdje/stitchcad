//! Concrete artifact intents and shared snapshot binding, before generation or dependency traversal.
use super::{
    CommandId, CommandSnapshotContext, ExportPdf, ExportPlt, GenerateTechPack, GradedSizeSelection,
};
use crate::ontology::{EntityId, SelectedProfilePin};

/// Registered DXF dialect identities, not writer flags or evidence of receiver compatibility.
/// Each fixes its layer naming, DXF release and polyline-only entity policy in the contract.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DxfTarget {
    /// Named AAMA layers in the R12 carrier.
    AamaNamedR12,
    /// Numbered ASTM convention in the R13 carrier; no standards-conformance claim.
    AstmNumberedR13,
}
impl DxfTarget {
    /// Both declared DXF identities, independent of handler availability or G6 receiver evidence.
    pub const ALL: [Self; 2] = [Self::AamaNamedR12, Self::AstmNumberedR13];
    /// Exact registered target token.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::AamaNamedR12 => "dxf-aama-named",
            Self::AstmNumberedR13 => "dxf-astm-num-r13",
        }
    }
    /// Exact lookup, without trimming, case folding or a universal-target fallback.
    #[must_use]
    pub fn from_token(token: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|target| target.token() == token)
    }
}

/// Explicit carriage of a requested DXF; actual grade rules retain stable point references and states.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DxfGrading {
    /// Serialize the one supplied Instance; no rule table or size range is inferred.
    SingleSize,
    /// AAMA base geometry plus a separate rule table; the companion is part of the requested artifact.
    BasePlusRules {
        /// Actual grade-rule table identity, not deltas computed from chart subtraction.
        rule_table: EntityId,
        /// Explicit target members under one set snapshot.
        sizes: GradedSizeSelection,
    },
    /// ASTM base geometry with its named rule table embedded in the DXF.
    EmbeddedRules {
        /// Actual grade-rule table identity.
        rule_table: EntityId,
        /// Explicit target members under one set snapshot.
        sizes: GradedSizeSelection,
    },
}

/// DXF request with explicit carriage, selected profile and resolved parameter-result claims.
/// Actual result bindings supply units, line semantics and allowance policy without copied defaults.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExportDxf {
    instance: EntityId,
    target: DxfTarget,
    selected: SelectedProfilePin,
    parameters: EntityId,
    grading: DxfGrading,
}
impl ExportDxf {
    /// Request one complete Instance in a named single-size dialect; allocates no file or result.
    /// Execution checks Design/Instance provenance, selected result, all required axes and policy.
    #[must_use]
    pub const fn single_instance(
        instance: EntityId,
        target: DxfTarget,
        selected: SelectedProfilePin,
        parameters: EntityId,
    ) -> Self {
        Self {
            instance,
            target,
            selected,
            parameters,
            grading: DxfGrading::SingleSize,
        }
    }
    /// Request the registered AAMA/R12 base-plus-rules carriage, including its companion rule table.
    /// Execution must check actual base/member/rule/break provenance; no complete-range claim is inferred.
    #[must_use]
    pub const fn base_plus_rules(
        base_instance: EntityId,
        rule_table: EntityId,
        sizes: GradedSizeSelection,
        selected: SelectedProfilePin,
        parameters: EntityId,
    ) -> Self {
        Self {
            instance: base_instance,
            target: DxfTarget::AamaNamedR12,
            selected,
            parameters,
            grading: DxfGrading::BasePlusRules { rule_table, sizes },
        }
    }
    /// Request the registered ASTM/R13 embedded-rule carriage with an explicit base and size selection.
    #[must_use]
    pub const fn embedded_rules(
        base_instance: EntityId,
        rule_table: EntityId,
        sizes: GradedSizeSelection,
        selected: SelectedProfilePin,
        parameters: EntityId,
    ) -> Self {
        Self {
            instance: base_instance,
            target: DxfTarget::AstmNumberedR13,
            selected,
            parameters,
            grading: DxfGrading::EmbeddedRules { rule_table, sizes },
        }
    }
    /// Exact carriage, rule table and target size selection, before any serialization or traversal.
    #[must_use]
    pub const fn grading(&self) -> &DxfGrading {
        &self.grading
    }
    /// Supplied Instance: serialized alone or used as the explicit base of the selected graded carriage.
    #[must_use]
    pub const fn instance(&self) -> EntityId {
        self.instance
    }
    /// Registered dialect requested explicitly by this artifact.
    #[must_use]
    pub const fn target(&self) -> DxfTarget {
        self.target
    }
    /// Exact profile/schema/content labels expected on the resolved parameters and source provenance.
    #[must_use]
    pub const fn selected_profile(&self) -> &SelectedProfilePin {
        &self.selected
    }
    /// Named resolved result; constructing this reference proves neither existence nor constraint truth.
    #[must_use]
    pub const fn parameters(&self) -> EntityId {
        self.parameters
    }
}

/// Concrete artifact schemas; handlers, serialization and receiver validation remain separate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArtifactPayload {
    /// Explicit named-target DXF and any requested rule-table companion.
    ExportDxf(ExportDxf),
    /// Named plotter target and actual selected plotter parameter result.
    ExportPlt(ExportPlt),
    /// Exact vector-print settings and logical/factory source selection.
    ExportPdf(ExportPdf),
    /// Canonical structured tech-pack content from authored and evaluated sources.
    GenerateTechPack(GenerateTechPack),
}
impl ArtifactPayload {
    /// The typed artifact determines identity, never an independently asserted command name.
    #[must_use]
    pub const fn command_id(&self) -> CommandId {
        match self {
            Self::ExportDxf(_) => CommandId::ExportDxf,
            Self::ExportPlt(_) => CommandId::ExportPlt,
            Self::ExportPdf(_) => CommandId::ExportPdf,
            Self::GenerateTechPack(_) => CommandId::GenerateTechPack,
        }
    }
}

/// Immutable generation request against an exact Design snapshot; no observable effect at construction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandArtifact {
    context: CommandSnapshotContext,
    payload: ArtifactPayload,
}
impl CommandArtifact {
    /// Bind source/actor/revision claims to the complete requested artifact without generating it.
    #[must_use]
    pub const fn new(context: CommandSnapshotContext, payload: ArtifactPayload) -> Self {
        Self { context, payload }
    }
    /// Original exact Design snapshot and unauthenticated actor/principal claims.
    #[must_use]
    pub const fn context(&self) -> &CommandSnapshotContext {
        &self.context
    }
    /// Concrete inputs also usable unchanged for artifact-specific uncertainty inspection.
    #[must_use]
    pub const fn payload(&self) -> &ArtifactPayload {
        &self.payload
    }
    /// Actual artifact command identity and its existing generate permission metadata.
    #[must_use]
    pub const fn command_id(&self) -> CommandId {
        self.payload.command_id()
    }
}
