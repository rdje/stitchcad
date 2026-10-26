//! Distinct regeneration and grade-rule inputs; these references do not validate or execute a path.
use super::{CommandId, CommandSnapshotContext};
use crate::ontology::{EntityId, SelectedProfilePin};
use sc_units::Count;

/// Exact requested member of a particular SizeSet revision, independent of labels or array positions.
/// Execution must resolve the actual membership and its current chart/axis/break bindings.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SizeMemberTarget {
    size_set: EntityId,
    revision: Count,
    member: EntityId,
}
impl SizeMemberTarget {
    /// Retain the actual set, its canonical Count revision and a member identity without lookup.
    #[must_use]
    pub const fn new(size_set: EntityId, revision: Count, member: EntityId) -> Self {
        Self {
            size_set,
            revision,
            member,
        }
    }
    /// Requested set identity; a resolved set is named explicitly rather than inferred from a profile.
    #[must_use]
    pub const fn size_set(self) -> EntityId {
        self.size_set
    }
    /// Exact set revision; this is separate from the command's Design revision.
    #[must_use]
    pub const fn revision(self) -> Count {
        self.revision
    }
    /// Requested member identity; no label parsing, base selection or member allocation.
    #[must_use]
    pub const fn member(self) -> EntityId {
        self.member
    }
}

/// Explicit canonical chart family whose existing bindings supply measurements and, for MTM, Ease.
/// A label, singleton membership or numeric value never selects a family on the caller's behalf.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegenerationChart {
    /// Authored garment SizeChart collection and its current member-to-POM correspondences.
    Garment(EntityId),
    /// Authored MtmChart collection and its current body/Ease/garment correspondences.
    Mtm(EntityId),
}

/// Explicit logical evaluation or a reference to selected factory parameters already resolved by G4.
/// Both are caller claims: execution checks required quantities, exact pins and result provenance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EvaluationProfile {
    /// Evaluate the Design's logical inputs without selecting a factory or inventing physical defaults.
    /// Any operation requiring an unbound factory quantity must still refuse it.
    Logical,
    /// Use the named resolved parameter result under this exact profile/schema/content selection.
    FactoryResolved {
        /// Explicit selected labels; presence is not registry or constraint validation.
        selected: SelectedProfilePin,
        /// Immutable resolved parameter-result identity, not an authored profile or a truth certificate.
        parameters: EntityId,
    },
}

/// Regenerate one member by evaluating the Design's recipe against its actual chart inputs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EvaluateInstance {
    target: SizeMemberTarget,
    chart: RegenerationChart,
    profile: EvaluationProfile,
}
impl EvaluateInstance {
    /// Preserve all input selections. No chart values, Ease mappings or uncertainty states are copied.
    /// Execution validates actual family/coverage/bindings/profile provenance before recipe evaluation.
    #[must_use]
    pub const fn new(
        target: SizeMemberTarget,
        chart: RegenerationChart,
        profile: EvaluationProfile,
    ) -> Self {
        Self {
            target,
            chart,
            profile,
        }
    }
    /// Exact target member and set snapshot.
    #[must_use]
    pub const fn target(&self) -> SizeMemberTarget {
        self.target
    }
    /// Explicit current chart selection.
    #[must_use]
    pub const fn chart(&self) -> RegenerationChart {
        self.chart
    }
    /// Original logical/factory selection, with no implicit latest profile or resolution.
    #[must_use]
    pub const fn profile(&self) -> &EvaluationProfile {
        &self.profile
    }
}

/// Instantiate one member from a base Instance, a grade-rule table and the target set's actual breaks.
/// This path does not replace its base/rules by evaluating the recipe or deriving chart differences.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GradeSize {
    base_instance: EntityId,
    rule_table: EntityId,
    target: SizeMemberTarget,
}
impl GradeSize {
    /// Retain the actual base/rule targets; execution must check base provenance and break coverage.
    /// Imported base instances are allowed; no recipe-derived base or rule table is silently supplied.
    #[must_use]
    pub const fn new(
        base_instance: EntityId,
        rule_table: EntityId,
        target: SizeMemberTarget,
    ) -> Self {
        Self {
            base_instance,
            rule_table,
            target,
        }
    }
    /// Explicit base Instance identity, whose provenance must agree with this target set and base.
    #[must_use]
    pub const fn base_instance(&self) -> EntityId {
        self.base_instance
    }
    /// Explicit table carrying stable grade-point references and declared X/Y deltas/attributes.
    #[must_use]
    pub const fn rule_table(&self) -> EntityId {
        self.rule_table
    }
    /// Target member and exact set snapshot owning the required authored breaks.
    #[must_use]
    pub const fn target(&self) -> SizeMemberTarget {
        self.target
    }
}

/// Implemented evaluation input schemas; profile-resolution selection follows its own contract.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EvaluationPayload {
    /// Measurement-driven recipe regeneration.
    EvaluateInstance(EvaluateInstance),
    /// Base-instance and grade-rule instantiation.
    GradeSize(GradeSize),
}
impl EvaluationPayload {
    /// Concrete payload determines identity and the existing evaluation/generate/discarded metadata.
    #[must_use]
    pub const fn command_id(&self) -> CommandId {
        match self {
            Self::EvaluateInstance(_) => CommandId::EvaluateInstance,
            Self::GradeSize(_) => CommandId::GradeSize,
        }
    }
}

/// Immutable derived-result request against an exact Design snapshot, before auth or execution.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandEvaluation {
    context: CommandSnapshotContext,
    payload: EvaluationPayload,
}
impl CommandEvaluation {
    /// Bind explicit inputs; no result identity, mutation retry key or undo group is allocated.
    #[must_use]
    pub const fn new(context: CommandSnapshotContext, payload: EvaluationPayload) -> Self {
        Self { context, payload }
    }
    /// Original Design/revision/actor claims.
    #[must_use]
    pub const fn context(&self) -> &CommandSnapshotContext {
        &self.context
    }
    /// Distinct actual inputs for the requested instantiation path.
    #[must_use]
    pub const fn payload(&self) -> &EvaluationPayload {
        &self.payload
    }
    /// Actual command identity; no independent caller assertion can change its class.
    #[must_use]
    pub const fn command_id(&self) -> CommandId {
        self.payload.command_id()
    }
}
