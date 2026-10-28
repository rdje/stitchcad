//! Explicit metadata input admission and data-only progress/cancellation contracts.
use super::{CommandId, CommandMutation, CommandMutationGroup, MutationPayload};
use crate::{ontology::MaterialAssignment, value::LengthState};
use core::{fmt, num::NonZeroUsize};

/// Explicit host-chosen positive limits for declared metadata input work, without hidden defaults.
/// These bound submitted members, primary metadata targets and cumulative proposed assignment-reason
/// bytes, plus proposed Known-state Measurement evidence references. They do not measure CPU time,
/// total model memory, numeric complexity or allocation peaks.
/// Admission grants no authority, target validity, handler availability or execution capability.
/// ```compile_fail
/// use sc_core::command::CommandOperationLimits;
/// use core::num::NonZeroUsize;
/// fn widen(limits: &mut CommandOperationLimits, maximum: NonZeroUsize) {
///     limits.members = maximum;
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandOperationLimits {
    members: NonZeroUsize,
    targets: NonZeroUsize,
    reason_bytes: NonZeroUsize,
    evidence_references: NonZeroUsize,
}
impl CommandOperationLimits {
    /// Preserve explicit finite positive host policy; no value is guessed or minted here.
    #[must_use]
    pub const fn new(
        members: NonZeroUsize,
        targets: NonZeroUsize,
        reason_bytes: NonZeroUsize,
        evidence_references: NonZeroUsize,
    ) -> Self {
        Self {
            members,
            targets,
            reason_bytes,
            evidence_references,
        }
    }
    /// Maximum original submitted members, including exact retry aliases.
    #[must_use]
    pub const fn members(&self) -> NonZeroUsize {
        self.members
    }
    /// Maximum primary metadata target selections across all submitted members.
    #[must_use]
    pub const fn targets(&self) -> NonZeroUsize {
        self.targets
    }
    /// Maximum cumulative UTF-8 reason bytes proposed on selected Piece assignments.
    #[must_use]
    pub const fn reason_bytes(&self) -> NonZeroUsize {
        self.reason_bytes
    }
    /// Maximum cumulative Known-state evidence references in submitted Measurement replacements.
    #[must_use]
    pub const fn evidence_references(&self) -> NonZeroUsize {
        self.evidence_references
    }
    /// Admit one original mutation's declared metadata input without resolving a model or grant.
    /// # Errors
    /// Returns the first member/target/assignment-byte/evidence excess, preserving exact input components.
    pub fn check_mutation(
        &self,
        mutation: &CommandMutation,
    ) -> Result<CommandOperationDemand, CommandOperationLimitError> {
        self.check_members(core::slice::from_ref(mutation))
    }
    /// Admit the complete original group, counting submitted aliases rather than silently deduping.
    /// Target demand counts one Measurement, selected Material Pieces or one Notch-owning Piece.
    /// Unavailable SplitSeamSpan/CloseDart handlers contribute no implemented metadata target demand;
    /// their submitted member still counts, and admission supplies no handler availability.
    /// # Errors
    /// Returns member-count excess first, then target/assignment-byte/evidence excess per original member.
    pub fn check_group(
        &self,
        group: &CommandMutationGroup,
    ) -> Result<CommandOperationDemand, CommandOperationLimitError> {
        self.check_members(group.mutations())
    }
    fn check_members(
        &self,
        members: &[CommandMutation],
    ) -> Result<CommandOperationDemand, CommandOperationLimitError> {
        if members.len() > self.members.get() {
            return Err(CommandOperationLimitError::Members {
                maximum: self.members.get(),
                requested: members.len(),
            });
        }
        let mut demand = CommandOperationDemand {
            members: members.len(),
            targets: 0,
            reason_bytes: 0,
            evidence_references: 0,
        };
        for (index, member) in members.iter().enumerate() {
            let command = member.command_id();
            let (targets, copies, bytes_per_copy, evidence_references) = match member.payload() {
                MutationPayload::ChangeMeasurement(change) => {
                    let evidence = match change.replacement().state() {
                        LengthState::Known { evidence, .. } => evidence.len(),
                        LengthState::Assumed { .. }
                        | LengthState::Unknown { .. }
                        | LengthState::Preference { .. }
                        | LengthState::Derived { .. } => 0,
                    };
                    (1, 0, 0, evidence)
                }
                MutationPayload::AddNotch(_) => (1, 0, 0, 0),
                MutationPayload::AssignMaterial(assignment) => {
                    let targets = assignment.pieces().pieces().len();
                    match assignment.material() {
                        MaterialAssignment::Assigned(_) => (targets, 0, 0, 0),
                        MaterialAssignment::Unresolved(reason) => {
                            (targets, targets, reason.len(), 0)
                        }
                    }
                }
                MutationPayload::SplitSeamSpan(_) | MutationPayload::CloseDart(_) => (0, 0, 0, 0),
            };
            // Demand is always at most the limit. Compare against remaining room before adding
            // or multiplying, retaining the exact components rather than wrapping or saturating.
            if targets > self.targets.get() - demand.targets {
                return Err(CommandOperationLimitError::Targets {
                    maximum: self.targets.get(),
                    consumed: demand.targets,
                    requested: targets,
                    index,
                    command,
                });
            }
            demand.targets += targets;
            let remaining = self.reason_bytes.get() - demand.reason_bytes;
            if bytes_per_copy != 0 && copies > remaining / bytes_per_copy {
                return Err(CommandOperationLimitError::ReasonBytes {
                    maximum: self.reason_bytes.get(),
                    consumed: demand.reason_bytes,
                    copies,
                    bytes_per_copy,
                    index,
                    command,
                });
            }
            demand.reason_bytes += copies * bytes_per_copy;
            if evidence_references > self.evidence_references.get() - demand.evidence_references {
                return Err(CommandOperationLimitError::EvidenceReferences {
                    maximum: self.evidence_references.get(),
                    consumed: demand.evidence_references,
                    requested: evidence_references,
                    index,
                    command,
                });
            }
            demand.evidence_references += evidence_references;
        }
        Ok(demand)
    }
}

/// Admitted declared input demand, not observed CPU work or a total-memory certificate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandOperationDemand {
    members: usize,
    targets: usize,
    reason_bytes: usize,
    evidence_references: usize,
}
impl CommandOperationDemand {
    /// Original submitted member count, including aliases.
    #[must_use]
    pub const fn members(self) -> usize {
        self.members
    }
    /// Original primary metadata target selections, without registry lookup.
    #[must_use]
    pub const fn targets(self) -> usize {
        self.targets
    }
    /// Cumulative proposed reason bytes over selected assignments, counting UTF-8 bytes.
    #[must_use]
    pub const fn reason_bytes(self) -> usize {
        self.reason_bytes
    }
    /// Cumulative submitted Known-state evidence references, including original exact aliases.
    #[must_use]
    pub const fn evidence_references(self) -> usize {
        self.evidence_references
    }
}

/// Original metadata input-demand refusal with exact counts, units and submitted member context.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandOperationLimitError {
    /// Submitted group exceeds the explicit original-member bound.
    Members {
        /// Explicit configured maximum members.
        maximum: usize,
        /// Original submitted member count.
        requested: usize,
    },
    /// This original member's primary metadata targets exceed remaining room.
    Targets {
        /// Explicit total target-selection bound.
        maximum: usize,
        /// Actual preceding submitted target count.
        consumed: usize,
        /// This member's original target count.
        requested: usize,
        /// Zero-based original member position.
        index: usize,
        /// Payload-derived command identity.
        command: CommandId,
    },
    /// This original assignment's proposed UTF-8 reason fields exceed remaining byte room.
    ReasonBytes {
        /// Explicit total proposed assignment-reason byte bound.
        maximum: usize,
        /// Actual preceding proposed reason bytes.
        consumed: usize,
        /// Selected Pieces receiving this exact reason.
        copies: usize,
        /// Exact original UTF-8 reason bytes per selected Piece.
        bytes_per_copy: usize,
        /// Zero-based original member position.
        index: usize,
        /// Payload-derived command identity.
        command: CommandId,
    },
    /// This original Measurement replacement exceeds remaining evidence-reference room.
    EvidenceReferences {
        /// Explicit cumulative submitted evidence-reference bound.
        maximum: usize,
        /// Actual preceding submitted reference count, including aliases.
        consumed: usize,
        /// This member's exact Known-state evidence-reference count.
        requested: usize,
        /// Zero-based original member position.
        index: usize,
        /// Payload-derived command identity.
        command: CommandId,
    },
}
impl fmt::Display for CommandOperationLimitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "metadata input limit: {self:?}")
    }
}
impl std::error::Error for CommandOperationLimitError {}

/// Declared pre-effect step; success itself acknowledges the completed operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandProgressPhase {
    /// Current authority has been checked.
    Authorized,
    /// Actual requested snapshot has been bound.
    SnapshotBound,
    /// Actual complete private retained-content admission before preparation or history traversal.
    RetainedValidation,
    /// Original submitted members are being prepared or resolved as exact aliases.
    Preparing,
    /// Actual current model/source validation.
    CurrentValidation,
    /// Actual private candidate/alternative validation.
    CandidateValidation,
    /// Actual record/reserve/stack allocation preflight.
    AllocationPreflight,
    /// All preparation is complete; no model has been published at this step.
    ReadyToPublish,
    /// An exact original accepted result has been found for authorized replay.
    Replaying,
}

/// Typed step fraction and original member context, containing no model or authority capability.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandProgress {
    phase: CommandProgressPhase,
    completed: usize,
    total: NonZeroUsize,
    member: Option<(usize, CommandId)>,
}
impl CommandProgress {
    /// Validate a declared phase-local fraction; this data constructor performs no operation.
    /// # Errors
    /// Returns the exact completed/total counts when completed exceeds total. Zero total cannot be supplied.
    pub fn new(
        phase: CommandProgressPhase,
        completed: usize,
        total: NonZeroUsize,
        member: Option<(usize, CommandId)>,
    ) -> Result<Self, CommandProgressError> {
        if completed > total.get() {
            return Err(CommandProgressError { completed, total });
        }
        Ok(Self {
            phase,
            completed,
            total,
            member,
        })
    }
    /// Declared current phase; fractions are local to this named phase.
    #[must_use]
    pub const fn phase(self) -> CommandProgressPhase {
        self.phase
    }
    /// Exact completed units of the declared phase.
    #[must_use]
    pub const fn completed(self) -> usize {
        self.completed
    }
    /// Explicit positive total units of the declared phase.
    #[must_use]
    pub const fn total(self) -> NonZeroUsize {
        self.total
    }
    /// Original zero-based member position and payload-derived identity when applicable.
    #[must_use]
    pub const fn member(self) -> Option<(usize, CommandId)> {
        self.member
    }
}
/// Invalid declared fraction; no clipping, saturation or invented denominator occurs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandProgressError {
    /// Exact supplied completed count.
    pub completed: usize,
    /// Exact supplied positive total.
    pub total: NonZeroUsize,
}
impl fmt::Display for CommandProgressError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "progress {} exceeds total {}",
            self.completed, self.total
        )
    }
}
impl std::error::Error for CommandProgressError {}

/// Host decision at a cancellable pre-effect progress checkpoint.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandProgressDecision {
    /// Continue within the installed operation policy.
    Continue,
    /// Refuse further work before model publication.
    Cancel,
}
/// Host-owned progress/cancellation observer; it cannot return a candidate or replace authority.
/// Runtime must emit actual phases before effects and honor Cancel atomically. Type construction
/// alone establishes neither runtime wiring nor cancellation behavior.
pub trait CommandProgressObserver {
    /// Observe readonly phase data and return a pre-effect continuation decision.
    fn observe(&self, progress: &CommandProgress) -> CommandProgressDecision;
}
impl CommandProgressObserver for () {
    fn observe(&self, _progress: &CommandProgress) -> CommandProgressDecision {
        CommandProgressDecision::Continue
    }
}
